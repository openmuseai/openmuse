#!/usr/bin/env python3
"""生成 post4 成片封面：主题「DSH 多端运行的全家桶」。

沿用 post2 / post3 系列的视觉语言：白底 + 蓝色强调（#2563EB）+ STHeiti 真实字体度量折行。
两种方向：

  panels  三端实录版：标题下方并排放三条**成片里真实抽出的画面**（Desktop / Web / Mobile），
          等高不等宽（和成片里三端同高、两侧模糊填充的处理一致），下方各带端名标签。
  text    纯文字版：完全沿用 post3 封面的构图（kicker / 标题 / 副标题 / 正文 / chip / 出处）。

画幅：横版 1920x1080（与成片同规格）；竖版 1080x1920（抖音）。
文案为草稿，可用 --title / --sub / --desc / --chip / --kicker 覆盖后重出。

输出：post4/build/covers/
用法：
  python3 make_post4_covers.py                 # 两种方向、两种画幅
  python3 make_post4_covers.py --style panels  # 只出三端实录版
  python3 make_post4_covers.py --preview       # 同时打印字符预览与几何/对比度自检
"""
from __future__ import annotations

import argparse
import pathlib
import subprocess
import sys

from PIL import Image, ImageDraw, ImageFilter, ImageFont

# ---------- 系列视觉语言（与 post2 / post3 一致） ----------
BG = (255, 255, 255)
INK = (23, 28, 37)
MUTED = (108, 117, 130)
ACCENT = (37, 99, 235)
RULE = (228, 232, 238)
CHIP = (239, 246, 255)
BORDER = (178, 188, 201)
PANEL_BG = (246, 248, 251)

FONT_BOLD = "/System/Library/Fonts/STHeiti Medium.ttc"
FONT_LIGHT = "/System/Library/Fonts/STHeiti Light.ttc"

ROOT = pathlib.Path(__file__).resolve().parent
VIDEO = ROOT / "build" / "openmuse-post4-multiplatform-1920x1080.mp4"
OUT = ROOT / "build" / "covers"
FRAMES = OUT / "frames"
FFMPEG = "/opt/homebrew/bin/ffmpeg"

FOOTER = "github.com/openmuseai/openmuse"

# 成片里每段的真实内容区：时间点 + 内容裁切（与 build_post4.sh 的画布处理一致）
SEGMENTS = [
    ("desktop", "桌面 Desktop", 5.0, 1720, 100),
    ("web", "浏览器 Web", 25.0, 1832, 44),
    ("mobile", "手机 Mobile", 37.0, 1664, 128),
]

COPY = dict(
    kicker="OpenMuse × DSH · 多端实录",
    title="DSH 多端运行的全家桶",
    # 竖版标题按语义断开，避免「全家桶」被折行拆开
    title_vertical="DSH\n多端运行的全家桶",
    sub="一套 DSH，桌面 / 浏览器 / 手机三端跑通",
    # 竖版宽度只有 924，按语义断行，避免行尾只剩一个「通」字
    sub_vertical="一套 DSH\n桌面 / 浏览器 / 手机三端跑通",
    desc="同一份 DSH 工作流分别在 Desktop、Web、Mobile 上运行，三条录屏拼成 42 秒成片，"
         "含交叉溶解转场与背景音乐。",
    chip="桌面 → 浏览器 → 手机 · 42 秒成片",
)

S_KICKER, S_TITLE, S_SUB, S_DESC, S_CHIP, S_FOOT, S_LABEL = 36, 96, 52, 34, 32, 30, 34

FOOT_PORTRAIT = 1740          # 竖版出处基线；正文与 chip 由它反推


def font(path: str, size: int) -> ImageFont.FreeTypeFont:
    return ImageFont.truetype(path, size)


def wrap(draw: ImageDraw.ImageDraw, text: str, fnt, max_w: int) -> list[str]:
    """按真实字体度量折行；遇换行符强制断行。"""
    lines: list[str] = []
    for para in text.split("\n"):
        cur = ""
        for ch in para:
            trial = cur + ch
            if draw.textlength(trial, font=fnt) <= max_w or not cur:
                cur = trial
            else:
                lines.append(cur)
                cur = ch
        lines.append(cur)
    return lines


def draw_block(draw, text, fnt, x, y, max_w, fill, line_h) -> int:
    """返回块底部 y；同时把每行包围盒记到 draw._boxes 供自检。"""
    lines = wrap(draw, text, fnt, max_w)
    for i, ln in enumerate(lines):
        yy = y + i * line_h
        draw.text((x, yy), ln, font=fnt, fill=fill)
        if hasattr(draw, "_boxes"):
            w = draw.textlength(ln, font=fnt)
            asc, desc = fnt.getmetrics()
            draw._boxes.append((x, yy, x + w, yy + asc + desc, ln))
    return y + len(lines) * line_h


# ---------- 三端缩略图 ----------
def extract_frames() -> dict[str, Image.Image]:
    """从成片里抽出三端的真实内容区（不缩放，交给调用方按目标高度缩放）。"""
    if not VIDEO.exists():
        sys.exit(f"缺少成片：{VIDEO}")
    FRAMES.mkdir(parents=True, exist_ok=True)
    imgs: dict[str, Image.Image] = {}
    for key, _label, t, w, x in SEGMENTS:
        png = FRAMES / f"{key}.png"
        if not png.exists():
            subprocess.run(
                [FFMPEG, "-hide_banner", "-loglevel", "error", "-y", "-ss", str(t),
                 "-i", str(VIDEO), "-frames:v", "1",
                 "-vf", f"crop={w}:1080:{x}:0", str(png)],
                check=True,
            )
        imgs[key] = Image.open(png).convert("RGB")
    return imgs


def rounded(img: Image.Image, radius: int) -> Image.Image:
    mask = Image.new("L", img.size, 0)
    ImageDraw.Draw(mask).rounded_rectangle([0, 0, img.size[0] - 1, img.size[1] - 1],
                                           radius=radius, fill=255)
    out = img.convert("RGBA")
    out.putalpha(mask)
    return out


def paste_panel(img: Image.Image, thumb: Image.Image, x: int, y: int, radius: int = 14,
                shadow: int = 6) -> None:
    """把缩略图作为卡片贴上：柔和投影 + 圆角 + 描边。

    成片里的 Web 端是很亮的白色界面，白底上只靠圆角不足以成卡片，所以补投影和略深的描边。
    """
    pw, ph = thumb.size
    mask = Image.new("L", img.size, 0)
    ImageDraw.Draw(mask).rounded_rectangle(
        [x, y + shadow // 2, x + pw - 1, y + ph - 1 + shadow // 2], radius=radius, fill=110)
    mask = mask.filter(ImageFilter.GaussianBlur(shadow))
    img.paste(Image.new("RGB", img.size, (200, 208, 219)), (0, 0), mask)

    card_mask = Image.new("L", (pw, ph), 0)
    ImageDraw.Draw(card_mask).rounded_rectangle([0, 0, pw - 1, ph - 1], radius=radius, fill=255)
    img.paste(thumb.convert("RGB"), (x, y), card_mask)
    ImageDraw.Draw(img).rounded_rectangle([x, y, x + pw - 1, y + ph - 1],
                                          radius=radius, outline=BORDER, width=2)


def panel_row(d: ImageDraw.ImageDraw, img: Image.Image, frames, y: int, height: int,
              gap: int, label_size: int, max_w: int) -> int:
    """并排画三条等高面板 + 各自端名标签，返回整体底部 y。

    文案或画幅变化导致整行超出可用宽度时自动缩小面板高度，避免溢出。
    """
    while height > 60:
        panels = [(key, round(frames[key].size[0] * height / frames[key].size[1]), height)
                  for key, _l, _t, _w, _x in SEGMENTS]
        total = sum(p[1] for p in panels) + gap * (len(panels) - 1)
        if total <= max_w:
            break
        height -= 4
    else:
        sys.exit("三端面板放不下：请减小 gap 或换更窄的抽帧")
    panels = [(key, round(frames[key].size[0] * height / frames[key].size[1]), height)
              for key, _l, _t, _w, _x in SEGMENTS]
    total = sum(p[1] for p in panels) + gap * (len(panels) - 1)
    x = (img.size[0] - total) // 2
    label_y = y + height + 22
    for key, pw, ph in panels:
        paste_panel(img, frames[key].resize((pw, ph), Image.LANCZOS), x, y)
        label = dict((k, l) for k, l, *_ in SEGMENTS)[key]
        fnt = font(FONT_BOLD, label_size)
        while d.textlength(label, font=fnt) > pw - 12 and label_size > 16:
            label_size -= 2
            fnt = font(FONT_BOLD, label_size)
        tw = d.textlength(label, font=fnt)
        d.text((x + (pw - tw) / 2, label_y), label, font=fnt, fill=INK)
        if hasattr(d, "_boxes"):
            asc, desc = fnt.getmetrics()
            d._boxes.append((x + (pw - tw) / 2, label_y, x + (pw + tw) / 2,
                             label_y + asc + desc, label))
        x += pw + gap
    return label_y + 46


# ---------- 两个方向 ----------
def build_panels(w: int, h: int, c: dict[str, str], frames) -> Image.Image:
    img = Image.new("RGB", (w, h), BG)
    d = ImageDraw.Draw(img)
    d._boxes = []
    margin = round(w * 0.072)
    max_w = w - 2 * margin
    land = w > h

    d.rectangle([0, 0, w, 14], fill=ACCENT)

    if land:
        y = draw_block(d, c["kicker"], font(FONT_BOLD, S_KICKER), margin, 120, max_w, ACCENT, 52)
        d.line([(margin, y + 22), (margin + 300, y + 22)], fill=ACCENT, width=6)
        y = draw_block(d, c["title"], font(FONT_BOLD, S_TITLE), margin, 240, max_w, INK, 116)
        y = draw_block(d, c["sub"], font(FONT_BOLD, S_SUB), margin, y + 26, max_w, ACCENT, 74)
        labels_bottom = panel_row(d, img, frames, 512, 300, 26, S_LABEL, max_w)
        ch = 912
        if labels_bottom + 20 > ch:
            sys.exit("横版版式放不下：三端标签与 chip 冲突")
        d.rounded_rectangle([margin, ch, w - margin, ch + 74], radius=16, fill=CHIP)
        draw_block(d, c["chip"], font(FONT_LIGHT, S_CHIP), margin + 30, ch + 20, max_w - 60, ACCENT, 46)
        d.text((margin, 1016), FOOTER, font=font(FONT_LIGHT, S_FOOT), fill=MUTED)
    else:
        y = draw_block(d, c["kicker"], font(FONT_BOLD, 40), margin, 380, max_w, ACCENT, 56)
        d.line([(margin, y + 24), (margin + 260, y + 24)], fill=ACCENT, width=6)
        y = draw_block(d, c["title_vertical"], font(FONT_BOLD, S_TITLE), margin, 500, max_w, INK, 116)
        y = draw_block(d, c["sub_vertical"], font(FONT_BOLD, S_SUB), margin, y + 40, max_w, ACCENT, 74)
        labels_bottom = panel_row(d, img, frames, 1040, 180, 20, 28, max_w)
        # 正文与 chip 由「出处」位置反推，文案变长时自动上移而不是压住出处
        f_desc = font(FONT_LIGHT, S_DESC)
        desc_h = len(wrap(d, c["desc"], f_desc, max_w)) * 48
        ch = FOOT_PORTRAIT - 24 - 80          # chip 底与出处留 24px
        yd = ch - 60 - desc_h                 # 正文底与 chip 顶留 60px
        if yd < labels_bottom + 20:
            sys.exit("竖版版式放不下：正文/chip 与三端标签冲突，请缩短正文或上移面板")
        draw_block(d, c["desc"], f_desc, margin, yd, max_w, MUTED, 48)
        d.rounded_rectangle([margin, ch, w - margin, ch + 80], radius=16, fill=CHIP)
        draw_block(d, c["chip"], font(FONT_LIGHT, S_CHIP), margin + 30, ch + 22, max_w - 60, ACCENT, 46)
        d.text((margin, FOOT_PORTRAIT), FOOTER, font=font(FONT_LIGHT, S_FOOT), fill=MUTED)
    img._boxes = d._boxes
    return img


def build_text(w: int, h: int, c: dict[str, str]) -> Image.Image:
    """纯文字版：沿用 post3 封面的构图。"""
    img = Image.new("RGB", (w, h), BG)
    d = ImageDraw.Draw(img)
    d._boxes = []
    margin = round(w * 0.072)
    max_w = w - 2 * margin
    land = w > h

    d.rectangle([0, 0, w, 14], fill=ACCENT)
    y = round(h * (0.150 if land else 0.170))
    y = draw_block(d, c["kicker"], font(FONT_BOLD, S_KICKER), margin, y, max_w, ACCENT, 52)
    d.line([(margin, y + 22), (margin + 300, y + 22)], fill=ACCENT, width=6)

    y = round(h * (0.245 if land else 0.255))
    y = draw_block(d, c["title"] if land else c["title_vertical"],
                   font(FONT_BOLD, S_TITLE), margin, y, max_w, INK, 116)
    y = draw_block(d, c["sub"] if land else c["sub_vertical"],
                   font(FONT_BOLD, S_SUB), margin, y + 26, max_w, ACCENT, 74)

    yd = round(h * (0.560 if land else 0.500))
    d.line([(margin, yd - 46), (w - margin, yd - 46)], fill=RULE, width=3)
    draw_block(d, c["desc"], font(FONT_LIGHT, S_DESC), margin, yd, max_w, MUTED, 56)

    ch = round(h * (0.800 if land else 0.830))
    d.rounded_rectangle([margin, ch, w - margin, ch + 76], radius=16, fill=CHIP)
    draw_block(d, c["chip"], font(FONT_LIGHT, S_CHIP), margin + 30, ch + 20, max_w - 60, ACCENT, 46)

    d.text((margin, round(h * (0.915 if land else 0.925))), FOOTER,
           font=font(FONT_LIGHT, S_FOOT), fill=MUTED)
    img._boxes = d._boxes
    return img


# ---------- 自检 ----------
def luminance(rgb) -> float:
    def ch(v):
        v /= 255
        return v / 12.92 if v <= 0.04045 else ((v + 0.055) / 1.055) ** 2.4
    r, g, b = (ch(v) for v in rgb)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def contrast(fg, bg) -> float:
    a, b = sorted((luminance(fg), luminance(bg)), reverse=True)
    return (a + 0.05) / (b + 0.05)


def check(img: Image.Image, name: str, boxes, margin: int) -> None:
    w, h = img.size
    print(f"  [{name}] {w}x{h}")
    print(f"    文本对比度：正文/标签 INK {contrast(INK, BG):.2f}:1  "
          f"强调 ACCENT {contrast(ACCENT, BG):.2f}:1  "
          f"次要 MUTED {contrast(MUTED, BG):.2f}:1  chip内 ACCENT/CHIP {contrast(ACCENT, CHIP):.2f}:1")
    bad = []
    for x0, y0, x1, y1, txt in boxes:
        if x0 < margin - 1 or x1 > w - margin + 1 or y0 < 0 or y1 > h:
            bad.append((txt, (round(x0), round(y0), round(x1), round(y1))))
    print(f"    文本越界：{'无' if not bad else bad}")
    # 文本块之间不得重叠（并排标签同高不算重叠）
    overlaps = []
    for i in range(len(boxes)):
        for j in range(i + 1, len(boxes)):
            a, b = boxes[i], boxes[j]
            if a[0] < b[2] and b[0] < a[2] and a[1] < b[3] and b[1] < a[3]:
                overlaps.append((a[4], b[4]))
    print(f"    文本重叠：{'无' if not overlaps else overlaps}")


def preview(img: Image.Image, cols: int = 100) -> None:
    """字符预览：把封面降采样成明暗网格，用于核对版式分布（非像素级审看）。"""
    rows = max(1, round(cols * img.size[1] / img.size[0] / 2.0))
    small = img.convert("L").resize((cols, rows), Image.BOX)
    ramp = "@%#*+=-:. "
    px = small.load()
    print(f"    ┌{'─' * cols}┐")
    for y in range(rows):
        line = "".join(ramp[min(len(ramp) - 1, px[x, y] * len(ramp) // 256)] for x in range(cols))
        print(f"    │{line}│")
    print(f"    └{'─' * cols}┘")


def save(img: Image.Image, name: str) -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    img.save(OUT / name)
    print(f"  ✓ {name}  {img.size[0]}x{img.size[1]}")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--style", default="all", choices=["all", "panels", "text"])
    ap.add_argument("--kicker")
    ap.add_argument("--title")
    ap.add_argument("--sub")
    ap.add_argument("--desc")
    ap.add_argument("--chip")
    ap.add_argument("--preview", action="store_true")
    ap.add_argument("--refresh-frames", action="store_true", help="重新从成片抽帧")
    args = ap.parse_args()

    c = dict(COPY)
    for k in ("kicker", "title", "sub", "desc", "chip"):
        if getattr(args, k):
            c[k] = getattr(args, k)
            if k == "sub":
                c["sub_vertical"] = args.sub
            if k == "title":
                # 竖版标题按「DSH」后的语义断行，避免拆开固定搭配
                c["title_vertical"] = args.title.replace("DSH ", "DSH\n", 1)
    if args.refresh_frames:
        for f in FRAMES.glob("*.png"):
            f.unlink()

    frames = extract_frames() if args.style in ("all", "panels") else {}

    jobs = []
    if args.style in ("all", "panels"):
        jobs.append(("cover-panels.png", build_panels(1920, 1080, c, frames), round(1920 * 0.072)))
        jobs.append(("cover-panels-vertical.png", build_panels(1080, 1920, c, frames), round(1080 * 0.072)))
    if args.style in ("all", "text"):
        jobs.append(("cover-text.png", build_text(1920, 1080, c), round(1920 * 0.072)))
        jobs.append(("cover-text-vertical.png", build_text(1080, 1920, c), round(1080 * 0.072)))

    for name, img, margin in jobs:
        save(img, name)
        if args.preview:
            check(img, name, img._boxes if hasattr(img, "_boxes") else [], margin)
    if args.preview and jobs:
        preview(jobs[0][1])


if __name__ == "__main__":
    main()
