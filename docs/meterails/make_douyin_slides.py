#!/usr/bin/env python3
"""生成抖音图文视频的幻灯片帧。

把 OPENMUSE-VS-DSH-知乎图文.md 的素材 + 4 张图表，渲染成 1920x1080 的幻灯片 PNG，
供 ffmpeg 合成视频。所有文本用真实字体测量，超宽自动折行。
"""
from __future__ import annotations

import pathlib
import sys

from PIL import Image, ImageDraw, ImageFont

W, H = 1920, 1080
BG = (255, 255, 255)
INK = (23, 28, 37)
MUTED = (108, 117, 130)
ACCENT = (37, 99, 235)
RULE = (228, 232, 238)

FONT_BOLD = "/System/Library/Fonts/STHeiti Medium.ttc"
FONT_LIGHT = "/System/Library/Fonts/STHeiti Light.ttc"

ROOT = pathlib.Path(__file__).resolve().parent
FIGS = ROOT / "assets" / "openmuse-vs-dsh"
OUT = ROOT / "build" / "slides"


def font(path: str, size: int) -> ImageFont.FreeTypeFont:
    return ImageFont.truetype(path, size)


def wrap(draw: ImageDraw.ImageDraw, text: str, fnt, max_w: int) -> list[str]:
    """按字符宽度折行；中文无空格，逐字累积测量。"""
    lines: list[str] = []
    cur = ""
    for ch in text:
        trial = cur + ch
        if draw.textlength(trial, font=fnt) <= max_w or not cur:
            cur = trial
        else:
            lines.append(cur)
            cur = ch
    if cur:
        lines.append(cur)
    return lines


def draw_block(draw, text, fnt, x, y, max_w, fill, line_h, max_lines=None):
    lines = wrap(draw, text, fnt, max_w)
    if max_lines is not None:
        lines = lines[:max_lines]
    for i, ln in enumerate(lines):
        draw.text((x, y + i * line_h), ln, font=fnt, fill=fill)
    return y + len(lines) * line_h


def canvas() -> tuple[Image.Image, ImageDraw.ImageDraw]:
    img = Image.new("RGB", (W, H), BG)
    return img, ImageDraw.Draw(img)


def save(img: Image.Image, name: str) -> pathlib.Path:
    OUT.mkdir(parents=True, exist_ok=True)
    p = OUT / name
    img.save(p)
    print(f"  ✓ {p.name}  {img.size[0]}x{img.size[1]}")
    return p


def slide_title() -> Image.Image:
    img, d = canvas()
    f_label = font(FONT_BOLD, 40)
    f_title = font(FONT_BOLD, 108)
    f_sub = font(FONT_LIGHT, 46)

    d.text((140, 190), "OpenMuse × DSH · 工程盘点", font=f_label, fill=ACCENT)
    d.line([(140, 256), (140 + 620, 256)], fill=ACCENT, width=6)

    y = draw_block(d, "一个是宿主，一个是内核", f_title, 140, 330, W - 280, INK, 140)
    draw_block(
        d,
        "用 53 个代码单元和 278 个插件包，做一次不采信产品文案的能力盘点",
        f_sub,
        140,
        y + 46,
        W - 320,
        MUTED,
        70,
    )

    d.text((140, H - 130), "只读代码取证 · 每条结论附文件路径", font=font(FONT_LIGHT, 36), fill=MUTED)
    return img


def slide_figure(idx: int, heading: str, sub: str, fname: str, caption: str) -> Image.Image:
    img, d = canvas()
    f_head = font(FONT_BOLD, 62)
    f_sub = font(FONT_LIGHT, 38)
    f_cap = font(FONT_LIGHT, 34)

    d.text((120, 70), f"{idx:02d} / {heading}", font=f_head, fill=INK)
    d.text((120, 150), sub, font=f_sub, fill=ACCENT)
    d.line([(120, 212), (W - 120, 212)], fill=RULE, width=3)

    top, bottom = 250, H - 130
    avail_h = bottom - top
    avail_w = W - 240

    fig = Image.open(FIGS / fname).convert("RGB")
    scale = min(avail_w / fig.width, avail_h / fig.height)
    fig = fig.resize((int(fig.width * scale), int(fig.height * scale)), Image.LANCZOS)

    fx = (W - fig.width) // 2
    fy = top + (avail_h - fig.height) // 2
    img.paste(fig, (fx, fy))

    d.text((120, H - 96), caption, font=f_cap, fill=MUTED)
    return img


def slide_conclusion() -> Image.Image:
    img, d = canvas()
    d.text((140, 130), "一句话分工", font=font(FONT_BOLD, 40), fill=ACCENT)
    d.line([(140, 196), (140 + 300, 196)], fill=ACCENT, width=6)

    f_big = font(FONT_BOLD, 76)
    f_body = font(FONT_LIGHT, 40)

    y = 300
    for line, col in (
        ("OpenMuse 给资源、权限与窗口", INK),
        ("DSH 给循环、工具与会话", ACCENT),
    ):
        d.text((140, y), line, font=f_big, fill=col)
        y += 130

    d.line([(140, y + 40), (W - 140, y + 40)], fill=RULE, width=3)
    draw_block(
        d,
        "宿主不该替内核思考，内核也不该替宿主决定数据放哪。",
        f_body,
        140,
        y + 90,
        W - 320,
        MUTED,
        62,
    )
    d.text((140, H - 120), "真正的风险不是能力差距，是版本漂移。", font=font(FONT_LIGHT, 36), fill=MUTED)
    return img


def main() -> int:
    print("渲染幻灯片…")
    save(slide_title(), "00-title.png")
    save(
        slide_figure(
            1,
            "定位对比",
            "一个是宿主，一个是内核",
            "fig1-positioning.png",
            "图 1 · 两者核心身份、运行载体与规模几乎不重叠",
        ),
        "01-positioning.png",
    )
    save(
        slide_figure(
            2,
            "能力成熟度矩阵",
            "18 个维度的 0–3 分逐条判定",
            "fig2-capability-matrix.png",
            "图 2 · 衡量成熟度，不衡量产品价值",
        ),
        "02-matrix.png",
    )
    save(
        slide_figure(
            3,
            "协作边界",
            "宿主给资源，内核给循环",
            "fig3-collaboration.png",
            "图 3 · 接缝是 Cordis provider seam，不是 API 适配层",
        ),
        "03-collaboration.png",
    )
    save(
        slide_figure(
            4,
            "闭包构成对比",
            "278 个 dsh-* 包 vs 53 个单元",
            "fig4-domain-distribution.png",
            "图 4 · 空白格暴露的是边界选择，不是能力缺失",
        ),
        "04-domains.png",
    )
    save(slide_conclusion(), "05-conclusion.png")
    print("完成。")
    return 0


if __name__ == "__main__":
    sys.exit(main())
