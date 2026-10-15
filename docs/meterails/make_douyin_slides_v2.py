#!/usr/bin/env python3
"""生成抖音图文（幻灯片）竖版帧 1080x1920。

内容取自 OPENMUSE-DSH-PAINPOINTS-知乎图文.md 的「痛点 + DSH 缺口」主线，
文字用真实字体度量并自动折行；截图按宽度等比缩放居中。
输出：build/post2-douyin/slides/*.png
"""
from __future__ import annotations

import pathlib
import sys

from PIL import Image, ImageDraw, ImageFont

W, H = 1080, 1920
BG = (255, 255, 255)
INK = (23, 28, 37)
MUTED = (108, 117, 130)
ACCENT = (37, 99, 235)
WARN = (220, 38, 38)
RULE = (228, 232, 238)
CHIP = (239, 246, 255)

FONT_BOLD = "/System/Library/Fonts/STHeiti Medium.ttc"
FONT_LIGHT = "/System/Library/Fonts/STHeiti Light.ttc"

ROOT = pathlib.Path(__file__).resolve().parent
FIGS = ROOT / "assets" / "dsh-in-codex"
OUT = ROOT / "build" / "post2-douyin" / "slides"

MARGIN = 90


def font(path: str, size: int) -> ImageFont.FreeTypeFont:
    return ImageFont.truetype(path, size)


def wrap(draw: ImageDraw.ImageDraw, text: str, fnt, max_w: int) -> list[str]:
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


def draw_block(draw, text, fnt, x, y, max_w, fill, line_h, max_lines=None) -> int:
    lines = wrap(draw, text, fnt, max_w)
    if max_lines is not None and len(lines) > max_lines:
        lines = lines[:max_lines]
        lines[-1] = lines[-1][:-1] + "…"
    for i, ln in enumerate(lines):
        draw.text((x, y + i * line_h), ln, font=fnt, fill=fill)
    return y + len(lines) * line_h


def new_canvas() -> tuple[Image.Image, ImageDraw.ImageDraw]:
    img = Image.new("RGB", (W, H), BG)
    return img, ImageDraw.Draw(img)


def save(img: Image.Image, name: str) -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    img.save(OUT / name)
    print(f"  ✓ {name}  {img.size[0]}x{img.size[1]}")


def header(d: ImageDraw.ImageDraw, idx: int, heading: str, sub: str = "") -> int:
    d.text((MARGIN, 130), f"{idx:02d}", font=font(FONT_BOLD, 46), fill=ACCENT)
    d.line([(MARGIN + 84, 156), (MARGIN + 84 + 90, 156)], fill=ACCENT, width=6)
    y = draw_block(d, heading, font(FONT_BOLD, 66), MARGIN, 220, W - 2 * MARGIN, INK, 88)
    if sub:
        y = draw_block(d, sub, font(FONT_LIGHT, 38), MARGIN, y + 8, W - 2 * MARGIN, ACCENT, 56)
    d.line([(MARGIN, y + 40), (W - MARGIN, y + 40)], fill=RULE, width=3)
    return y + 90


def footer(d: ImageDraw.ImageDraw, text: str, color=MUTED) -> None:
    draw_block(d, text, font(FONT_LIGHT, 33), MARGIN, H - 150, W - 2 * MARGIN, color, 48)


def slide_cover() -> Image.Image:
    img, d = new_canvas()
    d.rectangle([0, 0, W, 14], fill=ACCENT)
    d.text((MARGIN, 300), "OpenMuse × DSH · 抖音图文", font=font(FONT_BOLD, 40), fill=ACCENT)
    d.line([(MARGIN, 366), (MARGIN + 420, 366)], fill=ACCENT, width=6)
    y = draw_block(d, "把 DSH 装进 Codex 中", font(FONT_BOLD, 96), MARGIN, 470, W - 2 * MARGIN, INK, 128)
    draw_block(
        d,
        "DSH 缺的 6 样东西，谁来补？",
        font(FONT_BOLD, 62),
        MARGIN,
        y + 40,
        W - 2 * MARGIN,
        ACCENT,
        86,
    )
    draw_block(
        d,
        "账号、存储、多设备、窗口——DSH 亲口说了不管。这篇把 6 个缺口、补法和两条 Codex 接缝一次讲清。",
        font(FONT_LIGHT, 40),
        MARGIN,
        y + 260,
        W - 2 * MARGIN,
        MUTED,
        60,
    )
    d.rounded_rectangle([MARGIN, H - 330, W - MARGIN, H - 230], radius=16, fill=CHIP)
    draw_block(
        d,
        "只读代码取证 · 278 个随附包逐项核对",
        font(FONT_LIGHT, 34),
        MARGIN + 34,
        H - 306,
        W - 2 * MARGIN - 68,
        ACCENT,
        48,
    )
    footer(d, "github.com/openmuseai/openmuse")
    return img


def slide_pains() -> Image.Image:
    img, d = new_canvas()
    y = header(d, 1, "先对号入座", "这几个坑，你踩过几个？")
    items = [
        "换台电脑，会话没了，工作区还得重新配一遍",
        "手机上想看眼进度，发现那边只有一个命令行",
        "装完 DSH，模型凭据、权限、沙箱全得自己拼",
        "想把文件甩给同事看，对方先说“你 clone 一下”",
        "手一抖升级了 DSH，插件全炸，版本对不上",
    ]
    for i, text in enumerate(items, 1):
        d.rounded_rectangle([MARGIN, y, MARGIN + 62, y + 62], radius=14, fill=CHIP)
        d.text((MARGIN + 20, y + 8), str(i), font=font(FONT_BOLD, 38), fill=ACCENT)
        end = draw_block(d, text, font(FONT_LIGHT, 42), MARGIN + 92, y + 6, W - 2 * MARGIN - 92, INK, 58)
        y = max(end, y + 62) + 44
    footer(d, "中两条以上，问题真不在你——这是 DSH 主动划出去的边界", WARN)
    return img


def slide_gaps() -> Image.Image:
    img, d = new_canvas()
    y = header(d, 2, "DSH 到底缺什么", "6 条，每条都能在随附闭包里对上")
    rows = [
        ("1", "账号与身份", "只有 dsh-anonymous-user-id，没有账号体系"),
        ("2", "存储与同步", "只有本地 storage / spill / attachment"),
        ("3", "多设备与移动端", "没有移动客户端，人一走开就断线"),
        ("4", "桌面宿主表面", "窗口、编辑器、Office 预览都不做"),
        ("5", "开箱门槛", "凭据、权限、沙箱、桥接要自己配"),
        ("6", "云端执行世界", "控制面之外，substrate 没落地"),
    ]
    for num, name, desc in rows:
        d.rounded_rectangle([MARGIN, y, W - MARGIN, y + 176], radius=18, outline=RULE, width=3)
        d.text((MARGIN + 30, y + 26), num, font=font(FONT_BOLD, 52), fill=ACCENT)
        d.text((MARGIN + 110, y + 30), name, font=font(FONT_BOLD, 44), fill=INK)
        draw_block(d, desc, font(FONT_LIGHT, 32), MARGIN + 110, y + 96, W - 2 * MARGIN - 150, MUTED, 44)
        y += 198
    footer(d, "这不是 bug，是分工：账号 / 存储 / 设备 / 窗口本来就不归它管")
    return img


def slide_principle() -> Image.Image:
    img, d = new_canvas()
    y = header(d, 3, "补坑第一原则", "宿主只扩展执行面")
    d.text((MARGIN, y + 20), "别动内核", font=font(FONT_BOLD, 96), fill=ACCENT)
    y += 180
    for text in (
        "profile patch 只重写 provider row",
        "合同里 6 行 provider，没有一行是 agent-loop",
        "sidecar 拉起 dsh web --host 127.0.0.1 --port 0",
        "权限只要 4 项：context.read / process.sidecar 等",
    ):
        d.ellipse([MARGIN + 6, y + 18, MARGIN + 24, y + 36], fill=ACCENT)
        end = draw_block(d, text, font(FONT_LIGHT, 40), MARGIN + 56, y, W - 2 * MARGIN - 56, INK, 56)
        y = max(end, y + 56) + 34
    d.rounded_rectangle([MARGIN, y + 30, W - MARGIN, y + 300], radius=18, fill=CHIP)
    draw_block(
        d,
        "内核不动，才有后面“换宿主”的资格。改了循环，换一次宿主就得重写一遍。",
        font(FONT_LIGHT, 38),
        MARGIN + 34,
        y + 70,
        W - 2 * MARGIN - 68,
        ACCENT,
        56,
    )
    footer(d, "证据：plugins/dsh-agent · contracts/dsh/0.1.7-rc.1.contract.json")
    return img


def slide_figure(idx: int, heading: str, sub: str, fname: str, caption: str, foot: str) -> Image.Image:
    img, d = new_canvas()
    y = header(d, idx, heading, sub)
    top = y + 10
    bottom = H - 380
    avail_w, avail_h = W - 2 * MARGIN, bottom - top
    fig = Image.open(FIGS / fname).convert("RGB")
    scale = min(avail_w / fig.width, avail_h / fig.height)
    fig = fig.resize((int(fig.width * scale), int(fig.height * scale)), Image.LANCZOS)
    img.paste(fig, ((W - fig.width) // 2, top + (avail_h - fig.height) // 2))
    draw_block(d, caption, font(FONT_LIGHT, 33), MARGIN, H - 320, W - 2 * MARGIN, MUTED, 46)
    footer(d, foot)
    return img


def slide_seams() -> Image.Image:
    img, d = new_canvas()
    y = header(d, 8, "那“装进 Codex”呢", "能补的与补不了的，先把期待值对齐")
    cards = [
        ("接缝一", "dsh-hooks-codex", "你写过的 Codex hooks.json 原样生效，5 个 hook 点：SessionStart / UserPromptSubmit / PreToolUse / PostToolUse / Stop", ACCENT),
        ("接缝二", "dsh-subagent-codex", "走官方 app-server 协议，把 Codex 变成可编排的一次性子级，与进程内 / ACP / SDK 子级并排", ACCENT),
        ("补不了", "账号 · 存储 · 多设备 · 窗口", "Codex 是好脑子，但也没长身体：这四样它同样不会给你", WARN),
    ]
    for tag, name, desc, color in cards:
        d.rounded_rectangle([MARGIN, y, W - MARGIN, y + 300], radius=18, outline=RULE, width=3)
        d.text((MARGIN + 30, y + 26), tag, font=font(FONT_BOLD, 32), fill=color)
        draw_block(d, name, font(FONT_BOLD, 44), MARGIN + 30, y + 76, W - 2 * MARGIN - 60, INK, 58)
        draw_block(d, desc, font(FONT_LIGHT, 32), MARGIN + 30, y + 152, W - 2 * MARGIN - 60, MUTED, 44)
        y += 330
    footer(d, "选宿主 = 选你要补哪几块：要钩子与编排接 Codex，要身体找 OpenMuse")
    return img


def slide_outro() -> Image.Image:
    img, d = new_canvas()
    y = header(d, 9, "上手三步", "不用改内核，也不用重写钩子")
    steps = [
        "git clone https://github.com/openmuseai/openmuse.git",
        "挂 dsh-hooks-codex，configPath 指向 ./.codex/hooks.json",
        "要账号 / 存储 / 多设备，去看宿主那一侧的实现",
    ]
    for i, text in enumerate(steps, 1):
        d.rounded_rectangle([MARGIN, y, MARGIN + 62, y + 62], radius=14, fill=CHIP)
        d.text((MARGIN + 20, y + 8), str(i), font=font(FONT_BOLD, 38), fill=ACCENT)
        end = draw_block(d, text, font(FONT_LIGHT, 38), MARGIN + 92, y + 6, W - 2 * MARGIN - 92, INK, 54)
        y = max(end, y + 62) + 46
    d.rounded_rectangle([MARGIN, y + 40, W - MARGIN, y + 320], radius=20, fill=ACCENT)
    d.text((MARGIN + 40, y + 90), "仓库在这里，自己看", font=font(FONT_BOLD, 40), fill=(255, 255, 255))
    d.text((MARGIN + 40, y + 170), "github.com/openmuseai/", font=font(FONT_BOLD, 52), fill=(255, 255, 255))
    d.text((MARGIN + 40, y + 236), "openmuse", font=font(FONT_BOLD, 52), fill=(255, 255, 255))
    footer(d, "本文基于公开仓库与 DSH 随附闭包（278 个包）的只读代码取证整理")
    return img


def main() -> int:
    print("渲染竖版幻灯片…")
    save(slide_cover(), "00-cover.png")
    save(slide_pains(), "01-pains.png")
    save(slide_gaps(), "02-gaps.png")
    save(slide_principle(), "03-principle.png")
    save(slide_figure(4, "宿主做到了什么 · 1/4", "本地工作区 · 插件按需激活",
                      "fig1-openmuse-workspace.png",
                      "文档保存在本机；编辑器、Viewer、助手按需由插件激活",
                      "OpenMuse 桌面应用实机窗口"), "04-host-workspace.png")
    save(slide_figure(5, "宿主做到了什么 · 2/4", "默认只读，写入要显式申请",
                      "fig2-document-preview.png",
                      "“文件预览 · 只读”与 Edit 分开：补的是信心，不是功能",
                      "OpenMuse 桌面应用实机窗口"), "05-host-preview.png")
    save(slide_figure(6, "宿主做到了什么 · 3/4", "对话 / 轨迹 + Token 计量",
                      "fig3-conversation-trajectory.png",
                      "它想了什么、做了什么、花了多少，一眼能看见",
                      "OpenMuse 桌面应用实机窗口"), "06-host-trajectory.png")
    save(slide_figure(7, "宿主做到了什么 · 4/4", "补洞的证据在提交记录里",
                      "fig4-github-repo.png",
                      "同账号接管、手机配对、沙箱租约、清单 v2、接缝 TCK",
                      "github.com/openmuseai/openmuse"), "07-host-repo.png")
    save(slide_seams(), "08-seams.png")
    save(slide_outro(), "09-outro.png")
    print("完成：", OUT)
    return 0


if __name__ == "__main__":
    sys.exit(main())
