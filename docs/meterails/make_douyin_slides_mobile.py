#!/usr/bin/env python3
"""手机版 DSH 上线 · 20 秒抖音竖版幻灯片（1080x1920）。

在 make_douyin_slides_v2.py 的基础上重做封面（主题：手机版 DSH 上线）并把页数压到
7 页 / 20 秒；最后一页（含 GitHub 地址）停留 5 秒，明显长于其它页。

输出：build/post2-douyin-mobile/slides/*.png
"""
from __future__ import annotations

import pathlib
import sys

from PIL import Image, ImageDraw

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import make_douyin_slides_v2 as base  # noqa: E402  复用字体/折行/画布等低层工具

W, H = base.W, base.H
BG, INK, MUTED, ACCENT, WARN = base.BG, base.INK, base.MUTED, base.ACCENT, base.WARN
RULE, CHIP = base.RULE, base.CHIP
MARGIN = base.MARGIN
font = base.font
wrap = base.wrap
draw_block = base.draw_block
new_canvas = base.new_canvas
header = base.header
footer = base.footer
FIGDIR = base.FIGS

base.OUT = pathlib.Path(__file__).resolve().parent / "build" / "post2-douyin-mobile" / "slides"
save = base.save


def slide_cover_mobile() -> Image.Image:
    img, d = new_canvas()
    d.rectangle([0, 0, W, 14], fill=ACCENT)
    d.text((MARGIN, 250), "OpenMuse · 抖音图文", font=font(base.FONT_BOLD, 40), fill=ACCENT)
    d.line([(MARGIN, 316), (MARGIN + 420, 316)], fill=ACCENT, width=6)

    y = draw_block(d, "手机版 DSH 上线", font(base.FONT_BOLD, 104), MARGIN, 420, W - 2 * MARGIN, INK, 140)
    draw_block(d, "DSH 缺的 6 样东西，谁来补？", font(base.FONT_BOLD, 58), MARGIN, y + 30,
               W - 2 * MARGIN, ACCENT, 82)

    d.rounded_rectangle([MARGIN, y + 210, W - MARGIN, y + 430], radius=20, fill=CHIP)
    draw_block(d, "配对桌面 DSH 会话", font(base.FONT_BOLD, 42), MARGIN + 34, y + 250,
               W - 2 * MARGIN - 68, INK, 56)
    draw_block(d, "同账号实时接管 · 手机接着看", font(base.FONT_LIGHT, 36), MARGIN + 34, y + 320,
               W - 2 * MARGIN - 68, ACCENT, 52)

    draw_block(d, "账号、存储、多设备、窗口——DSH 亲口说了不管。这篇把 6 个缺口和两条 Codex 接缝一次讲清。",
               font(base.FONT_LIGHT, 38), MARGIN, y + 500, W - 2 * MARGIN, MUTED, 56)
    footer(d, "github.com/openmuseai/openmuse")
    return img


def slide_mobile_gap() -> Image.Image:
    img, d = new_canvas()
    y = header(d, 1, "手机版补的是哪个洞", "DSH 缺的第 1、3 条：身份与设备")
    cards = [
        ("洞 1", "账号与身份", "DSH 只有 dsh-anonymous-user-id；手机版走账号体系，谁拥有什么说得清", ACCENT),
        ("洞 3", "多设备与移动端", "DSH 没有移动客户端；手机版配对桌面会话，人走开也不断线", ACCENT),
        ("证据", "提交记录", "feat(mobile): pair with desktop DSH sessions / same-account realtime desktop handoff", MUTED),
    ]
    for tag, name, desc, color in cards:
        d.rounded_rectangle([MARGIN, y, W - MARGIN, y + 300], radius=18, outline=RULE, width=3)
        d.text((MARGIN + 30, y + 26), tag, font=font(base.FONT_BOLD, 32), fill=color)
        draw_block(d, name, font(base.FONT_BOLD, 44), MARGIN + 30, y + 76, W - 2 * MARGIN - 60, INK, 58)
        draw_block(d, desc, font(base.FONT_LIGHT, 32), MARGIN + 30, y + 152, W - 2 * MARGIN - 60, MUTED, 44)
        y += 330
    footer(d, "演示环境：OpenMuse 桌面应用 + 手机端同账号")
    return img


def slide_gaps() -> Image.Image:
    img, d = new_canvas()
    y = header(d, 2, "DSH 到底缺什么", "6 条，每条都能在随附闭包里对上")
    rows = [
        ("1", "账号与身份", "只有匿名 ID，没有账号体系"),
        ("2", "存储与同步", "只有本地 storage / spill / attachment"),
        ("3", "多设备与移动端", "没有移动客户端，人一走开就断线"),
        ("4", "桌面宿主表面", "窗口、编辑器、Office 预览都不做"),
        ("5", "开箱门槛", "凭据、权限、沙箱、桥接要自己配"),
        ("6", "云端执行世界", "控制面之外，substrate 没落地"),
    ]
    for num, name, desc in rows:
        d.rounded_rectangle([MARGIN, y, W - MARGIN, y + 176], radius=18, outline=RULE, width=3)
        d.text((MARGIN + 30, y + 26), num, font=font(base.FONT_BOLD, 52), fill=ACCENT)
        d.text((MARGIN + 110, y + 30), name, font=font(base.FONT_BOLD, 44), fill=INK)
        draw_block(d, desc, font(base.FONT_LIGHT, 32), MARGIN + 110, y + 96, W - 2 * MARGIN - 150, MUTED, 44)
        y += 198
    footer(d, "这不是 bug，是分工：账号 / 存储 / 设备 / 窗口本来就不归它管")
    return img


def slide_figure(idx: int, heading: str, sub: str, fname: str, caption: str, foot: str) -> Image.Image:
    img, d = new_canvas()
    y = header(d, idx, heading, sub)
    top = y + 10
    bottom = H - 380
    avail_w, avail_h = W - 2 * MARGIN, bottom - top
    fig = Image.open(FIGDIR / fname).convert("RGB")
    scale = min(avail_w / fig.width, avail_h / fig.height)
    fig = fig.resize((int(fig.width * scale), int(fig.height * scale)), Image.LANCZOS)
    img.paste(fig, ((W - fig.width) // 2, top + (avail_h - fig.height) // 2))
    draw_block(d, caption, font(base.FONT_LIGHT, 33), MARGIN, H - 320, W - 2 * MARGIN, MUTED, 46)
    footer(d, foot)
    return img


def slide_seams() -> Image.Image:
    img, d = new_canvas()
    y = header(d, 5, "那“装进 Codex”呢", "能补的与补不了的")
    cards = [
        ("接缝一", "dsh-hooks-codex", "你写过的 Codex hooks.json 原样生效：SessionStart / UserPromptSubmit / PreToolUse / PostToolUse / Stop", ACCENT),
        ("接缝二", "dsh-subagent-codex", "把 Codex 变成可编排的一次性子级，与进程内 / ACP / SDK 子级并排", ACCENT),
        ("补不了", "账号 · 存储 · 多设备 · 窗口", "Codex 是好脑子，但也没长身体", WARN),
    ]
    for tag, name, desc, color in cards:
        d.rounded_rectangle([MARGIN, y, W - MARGIN, y + 300], radius=18, outline=RULE, width=3)
        d.text((MARGIN + 30, y + 26), tag, font=font(base.FONT_BOLD, 32), fill=color)
        draw_block(d, name, font(base.FONT_BOLD, 44), MARGIN + 30, y + 76, W - 2 * MARGIN - 60, INK, 58)
        draw_block(d, desc, font(base.FONT_LIGHT, 32), MARGIN + 30, y + 152, W - 2 * MARGIN - 60, MUTED, 44)
        y += 330
    footer(d, "选宿主 = 选你要补哪几块")
    return img


def slide_outro() -> Image.Image:
    img, d = new_canvas()
    y = header(d, 6, "上手三步", "不用改内核，也不用重写钩子")
    steps = [
        "git clone https://github.com/openmuseai/openmuse.git",
        "挂 dsh-hooks-codex，configPath 指向 ./.codex/hooks.json",
        "要账号 / 存储 / 多设备，看宿主那一侧的实现",
    ]
    for i, text in enumerate(steps, 1):
        d.rounded_rectangle([MARGIN, y, MARGIN + 62, y + 62], radius=14, fill=CHIP)
        d.text((MARGIN + 20, y + 8), str(i), font=font(base.FONT_BOLD, 38), fill=ACCENT)
        end = draw_block(d, text, font(base.FONT_LIGHT, 38), MARGIN + 92, y + 6, W - 2 * MARGIN - 92, INK, 54)
        y = max(end, y + 62) + 46
    d.rounded_rectangle([MARGIN, y + 40, W - MARGIN, y + 340], radius=20, fill=ACCENT)
    d.text((MARGIN + 40, y + 90), "手机版 DSH 已上线，仓库在这里", font=font(base.FONT_BOLD, 38), fill=(255, 255, 255))
    d.text((MARGIN + 40, y + 176), "github.com/openmuseai/", font=font(base.FONT_BOLD, 54), fill=(255, 255, 255))
    d.text((MARGIN + 40, y + 248), "openmuse", font=font(base.FONT_BOLD, 54), fill=(255, 255, 255))
    footer(d, "基于公开仓库与 DSH 随附闭包（278 个包）的只读代码取证整理")
    return img


def main() -> int:
    print("渲染 20s 竖版幻灯片（手机版 DSH 上线）…")
    save(slide_cover_mobile(), "00-cover-mobile.png")
    save(slide_mobile_gap(), "01-mobile-gap.png")
    save(slide_gaps(), "02-gaps.png")
    save(slide_figure(3, "宿主做到了什么", "本地工作区 · 插件按需激活",
                      "fig1-openmuse-workspace.png",
                      "文档保存在本机；编辑器、Viewer、助手按需由插件激活",
                      "OpenMuse 桌面应用实机窗口"), "03-host-workspace.png")
    save(slide_figure(4, "手机版 / 账号的证据", "提交记录里能对上",
                      "fig4-github-repo.png",
                      "pair with desktop DSH sessions · same-account realtime desktop handoff",
                      "github.com/openmuseai/openmuse"), "04-host-repo.png")
    save(slide_seams(), "05-seams.png")
    save(slide_outro(), "06-outro-github.png")
    print("完成：", base.OUT)
    return 0


if __name__ == "__main__":
    sys.exit(main())
