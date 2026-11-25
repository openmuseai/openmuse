#!/usr/bin/env python3
"""生成 post3 录屏的封面标题卡（横版 1720x1080 + 竖版 1080x1920）。

沿用 post2 系列的视觉语言（白底 / 蓝色强调 / STHeiti 真实字体度量自动折行），
文字内容按录屏实际内容草拟，可用 --title / --sub / --desc / --chip 覆盖。

输出：post3/build/covers/cover-01*.png、cover-02*.png
"""
from __future__ import annotations

import argparse
import pathlib

from PIL import Image, ImageDraw, ImageFont

BG = (255, 255, 255)
INK = (23, 28, 37)
MUTED = (108, 117, 130)
ACCENT = (37, 99, 235)
RULE = (228, 232, 238)
CHIP = (239, 246, 255)

FONT_BOLD = "/System/Library/Fonts/STHeiti Medium.ttc"
FONT_LIGHT = "/System/Library/Fonts/STHeiti Light.ttc"

ROOT = pathlib.Path(__file__).resolve().parent
OUT = ROOT / "build" / "covers"

# 单套字号，横竖版共用（两块画布都是「满屏」设计）
S_KICKER, S_TITLE, S_SUB, S_DESC, S_CHIP, S_FOOT = 36, 88, 52, 38, 32, 30

CLIPS = {
    "01": dict(
        kicker="OpenMuse × DSH · 工作台实录 ①",
        title="让 DSH 读一遍 post2 素材",
        sub="截图 OCR → 素材复盘 → 落到 build/ 目录",
        desc="工作台里的一次真实操作：把 post2 的 4 张截图交给 DSH，用 Vision 做中英 OCR，"
             "回读 build/post2-douyin 的产物，再判断下一步做什么。",
        chip="录屏无解说 · 41 秒",
    ),
    "02": dict(
        kicker="OpenMuse × DSH · 工作台实录 ②",
        title="一条抖音流水线的完整走查",
        sub="交付形态 · 10 页幻灯片 · 扫码登录 · 发布留痕",
        desc="跟着 README 走一遍 post2 抖音流水线：为什么是竖版视频而不是图文帖、"
             "幻灯片怎么生成、扫码登录怎么过、发布结果留在哪里。",
        chip="原声解说 · 59 秒",
    ),
}

FOOTER = "github.com/openmuseai/openmuse"


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
    lines = wrap(draw, text, fnt, max_w)
    for i, ln in enumerate(lines):
        draw.text((x, y + i * line_h), ln, font=fnt, fill=fill)
    return y + len(lines) * line_h


def build(w: int, h: int, c: dict[str, str]) -> Image.Image:
    img = Image.new("RGB", (w, h), BG)
    d = ImageDraw.Draw(img)
    margin = round(w * 0.072)
    max_w = w - 2 * margin

    # 顶部强调条 + 左侧竖线，保持系列识别度
    d.rectangle([0, 0, w, 14], fill=ACCENT)

    land = w > h
    y = round(h * (0.150 if land else 0.170))
    y = draw_block(d, c["kicker"], font(FONT_BOLD, S_KICKER), margin, y, max_w, ACCENT, 52)
    d.line([(margin, y + 22), (margin + 300, y + 22)], fill=ACCENT, width=6)

    y = round(h * (0.245 if land else 0.255))
    y = draw_block(d, c["title"], font(FONT_BOLD, S_TITLE), margin, y, max_w, INK, 116)
    y = draw_block(d, c["sub"], font(FONT_BOLD, S_SUB), margin, y + 26, max_w, ACCENT, 74)

    yd = round(h * (0.640 if land else 0.620))
    d.line([(margin, yd - 46), (w - margin, yd - 46)], fill=RULE, width=3)
    draw_block(d, c["desc"], font(FONT_LIGHT, S_DESC), margin, yd, max_w, MUTED, 56)

    # 底部 chip + 出处
    ch = round(h * (0.800 if land else 0.830))
    d.rounded_rectangle([margin, ch, w - margin, ch + 76], radius=16, fill=CHIP)
    draw_block(d, c["chip"], font(FONT_LIGHT, S_CHIP), margin + 30, ch + 20, max_w - 60, ACCENT, 46)

    fy = round(h * (0.915 if land else 0.925))
    d.text((margin, fy), FOOTER, font=font(FONT_LIGHT, S_FOOT), fill=MUTED)
    return img


def save(img: Image.Image, name: str) -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    img.save(OUT / name)
    print(f"  ✓ {name}  {img.size[0]}x{img.size[1]}")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--clip", default="all", choices=["all", "01", "02"])
    ap.add_argument("--title")
    ap.add_argument("--sub")
    ap.add_argument("--desc")
    ap.add_argument("--chip")
    args = ap.parse_args()

    ids = list(CLIPS) if args.clip == "all" else [args.clip]
    for cid in ids:
        c = dict(CLIPS[cid])
        for k in ("title", "sub", "desc", "chip"):
            if getattr(args, k):
                c[k] = getattr(args, k)
        save(build(1720, 1080, c), f"cover-{cid}.png")
        save(build(1080, 1920, c), f"cover-{cid}-vertical.png")


if __name__ == "__main__":
    main()
