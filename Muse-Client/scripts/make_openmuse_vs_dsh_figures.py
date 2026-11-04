#!/usr/bin/env python3
"""Generate the OpenMuse vs DSH comparison figures (Pillow only).

Outputs PNGs into docs/assets/openmuse-vs-dsh/:

    fig1-positioning.png        OpenMuse / DSH 定位与身份对比
    fig2-capability-matrix.png  18 项能力维度成熟度矩阵
    fig3-collaboration.png      宿主 / 智能体协作边界与执行世界
    fig4-domain-distribution.png  随附闭包按能力域的构成对比

Why Pillow and not matplotlib: matplotlib is not present in the bundled
Python runtime, Pillow is.

Every text string is measured with the real font before it is placed, and the
layout helpers assert that nothing overflows its box. Run with the bundled
Python (matplotlib is absent, Pillow is present):

    python3 scripts/make_openmuse_vs_dsh_figures.py
"""

from __future__ import annotations

import os
import sys

from PIL import Image, ImageDraw, ImageFont

# ---------------------------------------------------------------- canvas

SCALE = 2                     # render at 2x then downsample for crisp text
W = 1600                      # logical width
FONT_PATH = "/System/Library/Fonts/Hiragino Sans GB.ttc"
FONT_BODY = 0                 # W3
FONT_BOLD = 2                 # W6

OUT_DIR = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "docs", "assets", "openmuse-vs-dsh",
)

# palette
BG = (255, 255, 255)
INK = (22, 24, 29)
MUTED = (91, 98, 112)
FAINT = (140, 147, 160)
LINE = (228, 231, 238)
PANEL = (247, 248, 251)
OM = (31, 111, 235)           # OpenMuse accent
DSH = (107, 78, 230)          # DSH accent
OK = (31, 111, 235)
WARN = (214, 130, 20)

MATURITY_COLORS = {
    0: ((236, 239, 244), INK),
    1: ((246, 217, 168), INK),
    2: ((168, 199, 240), INK),
    3: ((47, 111, 235), (255, 255, 255)),
}
MATURITY_LABELS = {
    0: "无 / 不在职责内",
    1: "仅设计稿或由他方提供",
    2: "部分实现 / 未过发布门禁",
    3: "已实现 / 成熟",
}

_fonts: dict[tuple[int, int], ImageFont.FreeTypeFont] = {}
_CANVAS_H: int = 0


def font(size: int, bold: bool = False) -> ImageFont.FreeTypeFont:
    key = (size, 1 if bold else 0)
    if key not in _fonts:
        _fonts[key] = ImageFont.truetype(
            FONT_PATH, size * SCALE, index=FONT_BOLD if bold else FONT_BODY
        )
    return _fonts[key]


def tw(draw: ImageDraw.ImageDraw, text: str, size: int, bold: bool = False) -> float:
    """Measured text width in logical pixels."""
    return draw.textlength(text, font=font(size, bold)) / SCALE


def th(size: int) -> int:
    return int(round(size * 1.42))


def wrap(draw, text, size, max_w, bold=False):
    """Greedy wrap that also breaks CJK runs, measured with the real font."""
    lines, cur = [], ""
    for ch in text:
        if ch == "\n":
            lines.append(cur)
            cur = ""
            continue
        trial = cur + ch
        if tw(draw, trial, size, bold) > max_w and cur:
            lines.append(cur)
            cur = ch
        else:
            cur = trial
    if cur:
        lines.append(cur)
    return lines or [""]


def assert_fits(draw, text, size, max_w, bold=False, where=""):
    w = tw(draw, text, size, bold)
    if w > max_w + 0.5:
        raise AssertionError(
            f"text overflows ({where}): width {w:.1f} > {max_w:.1f} :: {text[:60]!r}"
        )


def new_canvas(height: int) -> tuple[Image.Image, ImageDraw.ImageDraw]:
    global _CANVAS_H
    _CANVAS_H = height
    img = Image.new("RGB", (W * SCALE, height * SCALE), BG)
    return img, ImageDraw.Draw(img)


def save(img: Image.Image, height: int, name: str) -> str:
    out = img.resize((W, height), Image.LANCZOS)
    path = os.path.join(OUT_DIR, name)
    os.makedirs(OUT_DIR, exist_ok=True)
    out.save(path, "PNG", optimize=True)
    return path


def card(d, box, fill=PANEL, outline=LINE, radius=14, width=2):
    x0, y0, x1, y1 = [v * SCALE for v in box]
    rrect(d, [x0, y0, x1, y1], radius=radius * SCALE, fill=fill,
          outline=outline, width=int(width * SCALE))


def rrect(d, box, radius=8, fill=None, outline=None, width=1):
    """rounded_rectangle with every geometric argument coerced to int.

    Pillow's rounded_rectangle (unlike rectangle / ellipse / text) rejects
    floats, and this layout works in logical pixels multiplied by SCALE.
    """
    x0, y0, x1, y1 = (int(round(v)) for v in box)
    if x1 < x0:
        x0, x1 = x1, x0
    if y1 < y0:
        y0, y1 = y1, y0
    d.rounded_rectangle(
        [x0, y0, x1, y1],
        radius=max(0, int(round(radius))),
        fill=fill,
        outline=outline,
        width=max(1, int(round(width))),
    )


def text(d, xy, s, size, color=INK, bold=False):
    """Draw text, refusing to draw anything that leaves the canvas.

    Pillow silently clips out-of-canvas text, which would be invisible to a
    review that cannot look at the rendered image, so guard it here instead.
    """
    x, y = xy[0] * SCALE, xy[1] * SCALE
    if x < 0 or y < 0:
        raise AssertionError(f"text at negative coordinate ({xy}): {s!r}")
    right = xy[0] + tw(d, s, size, bold)
    bottom = xy[1] + th(size)
    if right > W - 2:
        raise AssertionError(
            f"text clipped at right edge (x={xy[0]:.0f}, right={right:.0f} > {W - 2}): {s!r}"
        )
    if _CANVAS_H and bottom > _CANVAS_H - 2:
        raise AssertionError(
            f"text clipped at bottom edge (bottom={bottom:.0f} > {_CANVAS_H - 2}): {s!r}"
        )
    d.text((x, y), s, font=font(size, bold), fill=color)


def title_block(d, y, main, sub):
    assert_fits(d, main, 34, W - 80, True, "fig title")
    text(d, (40, y), main, 34, INK, True)
    if sub:
        for i, ln in enumerate(wrap(d, sub, 19, W - 80)):
            text(d, (40, y + 46 + i * 27), ln, 19, MUTED)
        return y + 46 + len(wrap(d, sub, 19, W - 80)) * 27 + 18
    return y + 60


def footer(d, y, note):
    for i, ln in enumerate(wrap(d, note, 16, W - 80)):
        text(d, (40, y + i * 23), ln, 16, FAINT)
    return y + len(wrap(d, note, 16, W - 80)) * 23


# ------------------------------------------------------- fig 1 positioning

POSITIONING = [
    (
        "OpenMuse",
        "本地优先的工程工作台「宿主」",
        OM,
        [
            ("产品形态",
             "Flutter 桌面应用（macOS / Windows）＋ Android / iOS 移动端；"
             "Rust 契约与宿主运行时 crate 21 个、Dart package 22 个、首发插件 10 个。"),
            ("核心身份",
             "Workspace 与资源身份权威、权限与授权真源、插件宿主、窗口与三区 Workbench 拥有者、"
             "平台 Broker（Command / Event / Service / Context / Capability）。"),
            ("运行载体",
             "Host 进程 + 独立插件进程。Helix 经 PTY、Viewer 经 WKWebView、"
             "DSH 经 127.0.0.1:0 sidecar 接入；高频绘制与输入不进 Broker。"),
            ("明确不拥有",
             "Helix 的 PTY 与终端实现、格式渲染器与格式内部模型、"
             "DSH 的 CLI 参数与模型凭据、Agent 对话 UI、插件逐帧绘制与 IME。"),
            ("首发状态",
             "macOS 工程包与 zip 已产出（38.8 MB，ad-hoc 签名，未公证）；"
             "Windows 安装包未通过门禁，不得宣称已发布。"),
        ],
    ),
    (
        "DSH（DeepSeek Harness）",
        "可插件化的 AI 编码智能体「内核」",
        DSH,
        [
            ("产品形态",
             "Electron 桌面壳 dsh-desktop 0.11.0，内含 @deepseek-ai/dsh 0.2.0-rc.2；"
             "另有 dsh CLI、Web GUI、headless 与 ACP / SDK 嵌入形态。"),
            ("核心身份",
             "Agent Loop（调用模型→执行工具→重复）、工具执行与沙箱策略、"
             "会话持久化与上下文压缩、模型接入与凭据、子代理与工作流编排。"),
            ("运行载体",
             "Cordis 插件树：随附闭包 278 个 dsh-* 包，含 62 个客户端 UI 包、"
             "24 个会话包、21 个工具包；profile 用 patch 层叠加 bundles 装配。"),
            ("明确不拥有",
             "跨插件资源身份与授权、Workspace 权威、宿主窗口与布局、"
             "文件访问的最终裁决；这些由宿主通过 seam 提供。"),
            ("版本注意",
             "本机 DSH Desktop 为 0.2.0-rc.2，而 OpenMuse 产品 pin 的是 0.1.7-rc.1；"
             "两者 seam 集合与合同需显式升级验收。"),
        ],
    ),
]


def fig1():
    d_probe = ImageDraw.Draw(Image.new("RGB", (10, 10)))
    margin, gap = 40, 32
    panel_w = (W - 2 * margin - gap) // 2
    inner = panel_w - 56

    # pre-layout to know the height
    def layout(rows):
        h = 96  # header
        for label, body in rows:
            h += th(20) + 6
            h += len(wrap(d_probe, body, 18, inner)) * 26 + 20
        return h

    panel_h = max(layout(r) for _, _, _, r in POSITIONING)
    height = 150 + panel_h + 40 + 96
    img, d = new_canvas(height)
    y = title_block(
        d, 34,
        "图 1 · 定位对比：一个是宿主，一个是内核",
        "两者不是同类竞品。OpenMuse 拥有资源、权限与窗口；DSH 拥有智能体循环与工具执行。"
        "证据：Muse-Client/README.md、docs/PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md、"
        "/tmp 解包闭包 package.json。",
    )

    for i, (name, tagline, accent, rows) in enumerate(POSITIONING):
        x0 = margin + i * (panel_w + gap)
        x1 = x0 + panel_w
        card(d, (x0, y, x1, y + panel_h), fill=(250, 251, 253))
        # header band
        rrect(d, 
            [x0 * SCALE, y * SCALE, x1 * SCALE, (y + 84) * SCALE],
            radius=14 * SCALE, fill=accent,
        )
        d.rectangle(
            [x0 * SCALE, (y + 60) * SCALE, x1 * SCALE, (y + 84) * SCALE], fill=accent
        )
        assert_fits(d, name, 27, inner, True, "fig1 name")
        text(d, (x0 + 28, y + 16), name, 27, (255, 255, 255), True)
        assert_fits(d, tagline, 17, inner, False, "fig1 tagline")
        text(d, (x0 + 28, y + 52), tagline, 17, (235, 238, 250))

        cy = y + 104
        for label, body in rows:
            assert_fits(d, label, 20, inner, True, f"fig1 label {label}")
            text(d, (x0 + 28, cy), label, 20, accent, True)
            cy += th(20) + 6
            for ln in wrap(d, body, 18, inner):
                text(d, (x0 + 28, cy), ln, 18, INK)
                cy += 26
            cy += 20
        if cy - 20 > y + panel_h + 1:
            raise AssertionError(f"fig1 panel overflow: {cy - 20} > {y + panel_h}")

    footer(
        d, y + panel_h + 26,
        "注：本图与后续三图的所有数字均来自本仓库与 DSH 随附闭包的实际文件，"
        "非产品文案推断。DSH 版本号取自 app.asar 内 package.json。",
    )
    return save(img, height, "fig1-positioning.png")


# -------------------------------------------------- fig 2 capability matrix

# (维度, OpenMuse 成熟度, DSH 成熟度, 证据要点)
MATRIX = [
    ("宿主窗口与工作台布局", 3, 3, "三区 Workbench / Electron 桌面 + Web GUI"),
    ("插件模型与热插拔", 2, 3, "编译期装配 vs Cordis 插件树 + HMR"),
    ("资源身份与 Workspace 权威", 3, 2, "ResourceRef/lease/revision vs ctx.workspace"),
    ("权限、审批与审计", 2, 3, "Broker 权限交集 vs approval + policy"),
    ("执行沙箱与隔离", 2, 3, "本地 runtime 契约 vs landlock / ACL"),
    ("文本编辑与终端 PTY", 2, 3, "Helix 插件 macOS PoC vs bash / pwsh"),
    ("文件预览与 Office 引擎", 2, 3, "只读引擎 vs 捆绑 LibreOffice 创作"),
    ("智能体循环与工具调用", 1, 3, "不实现，消费 DSH vs agent-loop"),
    ("子代理与工作流编排", 1, 3, "由 DSH 提供 vs subagent / workflow"),
    ("会话持久化与上下文压缩", 1, 3, "由 DSH 提供 vs jsonl + sqlite"),
    ("模型接入与凭据管理", 2, 3, "Credential 设计 vs dsh-llm 家族"),
    ("Web 搜索与抓取", 0, 3, "不在职责内 vs web_search / fetch"),
    ("MCP 与外部协议接入", 1, 3, "由 DSH 提供 vs mcp-client / ACP"),
    ("插件市场与分发门禁", 2, 3, "CLI registry + 签名门禁 vs dshmarket"),
    ("存储、同步与可移植性", 3, 1, "S3 ABI + TCK + BYOS vs 本地 storage"),
    ("账号、设备配对与多设备", 2, 1, "GoTrue + 配对 relay vs 匿名 id"),
    ("云端执行世界", 2, 2, "控制面已接受 vs seam 只随附 local"),
    ("遥测与可观测性", 2, 3, "审计脱敏 vs otel + token-meter"),
]


def fig2():
    row_h = 52
    head_h = 118
    name_w = 470
    note_w = 400
    col_w = 268
    x_name = 40
    x_note = x_name + name_w
    x_om = x_note + note_w
    x_dsh = x_om + col_w
    table_w = name_w + note_w + 2 * col_w
    if x_dsh + col_w > W - 40:
        raise AssertionError("fig2 table wider than the canvas")
    height = head_h + len(MATRIX) * row_h + 56 + 150

    img, d = new_canvas(height)
    title_block(
        d, 30,
        "图 2 · 能力成熟度矩阵：18 个维度上的分工与落差",
        "0=无 / 不在职责内，1=仅设计稿，或该能力完全由对侧提供，2=部分实现或未过发布门禁，"
        "3=已实现。逐条判定依据见报告第 4 节。",
    )

    y = 118
    text(d, (x_name, y), "能力维度", 20, INK, True)
    text(d, (x_note, y), "证据要点", 20, INK, True)
    for x, nm, accent in ((x_om, "OpenMuse", OM), (x_dsh, "DSH", DSH)):
        w = tw(d, nm, 22, True)
        text(d, (x + (col_w - w) / 2, y - 2), nm, 22, accent, True)
    y += 34

    for i, (name, om, dsh, note) in enumerate(MATRIX):
        ry = y + i * row_h
        if i % 2 == 0:
            d.rectangle(
                [x_name * SCALE, ry * SCALE, (x_name + table_w) * SCALE,
                 (ry + row_h - 4) * SCALE], fill=(250, 251, 253)
            )
        assert_fits(d, name, 17, name_w - 20, False, f"fig2 label {name}")
        text(d, (x_name + 8, ry + 16), name, 17, INK)

        note_lines = wrap(d, note, 14, note_w - 24)
        if len(note_lines) > 2:
            raise AssertionError(f"fig2 note too long for 2 lines: {note!r}")
        for k, ln in enumerate(note_lines):
            assert_fits(d, ln, 14, note_w - 24, False, f"fig2 note {name}")
            text(d, (x_note + 8, ry + 9 + k * 19), ln, 14, MUTED)

        for x, val in ((x_om, om), (x_dsh, dsh)):
            bx0, bx1 = x + 10, x + col_w - 10
            by0, by1 = ry + 6, ry + row_h - 10
            fill, fg = MATURITY_COLORS[val]
            rrect(d, [bx0 * SCALE, by0 * SCALE, bx1 * SCALE, by1 * SCALE],
                  radius=8 * SCALE, fill=fill)
            text(d, (bx0 + 16, by0 + (by1 - by0 - th(20)) / 2 + 1), str(val), 20, fg, True)
            cap = {0: "—", 1: "设计/对侧", 2: "部分", 3: "已实现"}[val]
            assert_fits(d, cap, 15, bx1 - bx0 - 50, False, f"fig2 cap {name}")
            text(d, (bx0 + 42, by0 + (by1 - by0 - th(15)) / 2 + 2), cap, 15, fg)

    # legend
    ly = y + len(MATRIX) * row_h + 26
    text(d, (x_name, ly), "成熟度图例", 18, INK, True)
    lx = x_name
    for lvl in (0, 1, 2, 3):
        fill, fg = MATURITY_COLORS[lvl]
        lab = MATURITY_LABELS[lvl]
        w = 44 + tw(d, lab, 16) + 26
        rrect(d, [lx * SCALE, (ly + 30) * SCALE, (lx + w) * SCALE, (ly + 62) * SCALE],
              radius=8 * SCALE, fill=fill)
        text(d, (lx + 14, ly + 36), str(lvl), 18, fg, True)
        text(d, (lx + 34, ly + 38), lab, 16, fg)
        lx += w + 14
        if lx > W - 40:
            raise AssertionError("fig2 legend overflows the canvas width")

    footer(
        d, ly + 76,
        "读法：OpenMuse 领先的三项（资源权威、存储与同步、账号多设备）正是宿主的职责；"
        "DSH 领先的项集中在智能体内核。注意本表衡量的是「成熟度」，不是产品价值："
        "OpenMuse 自评为 PoC 阶段（macOS 未签名公证、Windows 未发布），而 DSH 是已发布产品，"
        "因此矩阵天然偏向 DSH。真正重叠的只有执行沙箱与云端执行世界，且是互补关系。",
    )
    return save(img, height, "fig2-capability-matrix.png")


# ----------------------------------------------------- fig 3 collaboration


def fig3():
    height = 1000
    img, d = new_canvas(height)
    y = title_block(
        d, 30,
        "图 3 · 协作边界：宿主给资源，内核给循环",
        "OpenMuse 不实现 Agent，DSH 不实现资源权威。两侧通过插件清单、"
        "Cordis provider seam 与窄工具调用对接。",
    )

    top = 132
    box_h = 360
    left = (40, top, 700, top + box_h)
    right = (900, top, 1560, top + box_h)

    for box, accent, heading, sub, items in (
        (left, OM, "OpenMuse Host", "资源与权限的真源", [
            "Plugin Catalog / Contribution Registry",
            "Resource Authority：resourceRef / revision / lease",
            "Platform Broker：Command / Event / Service / Context",
            "View Manager：attach / bounds / focus / z-order",
            "Credential Service：不可导出的凭据句柄",
            "Workspace：多 Mount 抽象资源树",
        ]),
        (right, DSH, "DSH sidecar", "智能体循环的真源", [
            "Agent Loop：调用模型 → 执行工具 → 重复",
            "ctx.tools：bash / fs / web / subagent / workflow …",
            "Session：jsonl 持久化 + compaction + 遥测",
            "Cordis loader：profile 与 patch 层装配",
            "llm / credentials：模型路由与凭据",
            "skills / goal / plan-mode：长期任务状态",
        ]),
    ):
        x0, y0, x1, y1 = box
        card(d, box, fill=(250, 251, 253))
        rrect(d, 
            [x0 * SCALE, y0 * SCALE, x1 * SCALE, (y0 + 74) * SCALE],
            radius=14 * SCALE, fill=accent,
        )
        d.rectangle([x0 * SCALE, (y0 + 50) * SCALE, x1 * SCALE, (y0 + 74) * SCALE], fill=accent)
        text(d, (x0 + 24, y0 + 14), heading, 25, (255, 255, 255), True)
        text(d, (x0 + 24, y0 + 46), sub, 16, (236, 239, 250))
        iy = y0 + 96
        for it in items:
            d.ellipse(
                [(x0 + 26) * SCALE, (iy + 9) * SCALE, (x0 + 34) * SCALE, (iy + 17) * SCALE],
                fill=accent,
            )
            assert_fits(d, it, 17, x1 - x0 - 60, False, "fig3 item")
            text(d, (x0 + 46, iy), it, 17, INK)
            iy += 40

    # seam
    sx = 760
    d.rectangle([sx * SCALE, top * SCALE, (sx + 80) * SCALE, (top + box_h) * SCALE], fill=(243, 244, 249))
    for i, ln in enumerate(wrap(d, "插件边界", 17, 76, True)):
        text(d, (sx + 12, top + 150 + i * 24), ln, 17, INK, True)
    for i, ln in enumerate(wrap(d, "Cordis provider 替换点", 15, 76)):
        text(d, (sx + 8, top + 210 + i * 22), ln, 15, MUTED)

    # arrows / exchanges
    ay = top + box_h + 46
    text(d, (40, ay - 34), "跨边界的两条消息路径", 20, INK, True)

    flows = [
        (OM, "OpenMuse → DSH", "Workspace binding（Mount ↔ DSH Workspace）、"
                              "ContextProjection、credential handle、capability 租约"),
        (DSH, "DSH → OpenMuse", "workspace.openResource(ResourceRef, anchor) → "
                               "Broker 权限裁决 → Surface Orchestrator → receipt"),
    ]
    fy = ay
    for accent, head, body in flows:
        card(d, (40, fy, 1560, fy + 78), fill=(250, 251, 253))
        rrect(d, 
            [40 * SCALE, fy * SCALE, 50 * SCALE, (fy + 78) * SCALE], radius=5 * SCALE, fill=accent
        )
        text(d, (70, fy + 12), head, 19, accent, True)
        for i, ln in enumerate(wrap(d, body, 17, 1440)):
            text(d, (70, fy + 40 + i * 24), ln, 17, INK)
        fy += 92

    # execution world band
    ey = fy + 6
    card(d, (40, ey, 1560, ey + 132), fill=(243, 244, 249))
    text(d, (70, ey + 14), "执行世界（替换同一组 provider row，不改 Agent Loop）", 19, INK, True)
    worlds = [
        ("Local sandbox", "Host Sandbox Service + 本地 runtime"),
        ("Cloud runtime", "tenant/task 独占 container 或 microVM"),
        ("Paired Desktop", "经配对 Desktop 复用其本地能力"),
    ]
    wx = 70
    for nm, desc in worlds:
        wbox = 470
        card(d, (wx, ey + 50, wx + wbox, ey + 112), fill=(255, 255, 255), radius=10)
        text(d, (wx + 18, ey + 60), nm, 18, DSH, True)
        assert_fits(d, desc, 15, wbox - 36, False, "fig3 world")
        text(d, (wx + 18, ey + 84), desc, 15, MUTED)
        wx += wbox + 20
    if wx - 20 > 1540:
        raise AssertionError("fig3 execution-world band overflows")

    footer(
        d, ey + 148,
        "被替换的 seam：ctx.fs / ctx.subprocess / ctx.shell / ctx.sandbox / ctx.jobs。"
        "模型看到的 bash、文件工具与 jobs 工具保持不变，因此 Local、Cloud、Paired Desktop "
        "的差异下沉到同一执行世界，而不是分叉成多套业务插件。",
    )
    return save(img, height, "fig3-collaboration.png")


# ------------------------------------------------ fig 4 domain distribution

# 手工归类，两侧使用同一套能力域。计数来源：
#   DSH   = /tmp/dsh-asar/node_modules/@deepseek-ai 下 278 个 dsh-* 包
#   OpenMuse = crates/(21) + packages/(22) + plugins/(10) = 53 个单元
DOMAINS = [
    ("UI 与客户端", 62, 3),
    ("会话与上下文", 28, 0),
    ("工具与命令", 24, 0),
    ("智能体与编排", 17, 4),
    ("宿主、契约与 API", 22, 9),
    ("执行 seam 与沙箱", 20, 3),
    ("模型接入与凭据", 9, 1),
    ("存储与同步", 6, 7),
    ("搜索、网络与集成", 12, 0),
    ("插件分发与市场", 5, 2),
    ("技能与长期任务", 8, 0),
    ("文件预览与 Office", 2, 5),
    ("编辑器与终端", 0, 4),
    ("账号与多设备", 1, 5),
    ("云端执行", 0, 2),
    ("移动端与语音", 0, 5),
    ("遥测与审计", 6, 1),
    ("其他与实验", 34, 2),
]


def fig4():
    row_h = 40
    head_h = 118
    height = head_h + len(DOMAINS) * row_h + 40 + 150

    img, d = new_canvas(height)
    y = title_block(
        d, 30,
        "图 4 · 随附闭包的构成对比：内核很宽，宿主很薄",
        "同一套能力域下的单元数量。DSH 是 278 个包构成的插件树；"
        "OpenMuse 是 21 个 Rust crate + 22 个 Dart package + 10 个首发插件，共 53 个单元。",
    )

    y = 118
    left_x, right_x = 40, 850
    half = 710
    label_w = 250
    bar_max = half - label_w - 90
    dsh_max = max(v[1] for v in DOMAINS)
    om_max = max(v[2] for v in DOMAINS)

    text(d, (left_x, y), "DSH：278 个包", 21, DSH, True)
    text(d, (right_x, y), "OpenMuse：53 个单元", 21, OM, True)
    y += 32

    for i, (name, dn, on) in enumerate(DOMAINS):
        ry = y + i * row_h
        if i % 2 == 0:
            for x in (left_x, right_x):
                d.rectangle(
                    [x * SCALE, ry * SCALE, (x + half) * SCALE, (ry + row_h - 3) * SCALE],
                    fill=(250, 251, 253),
                )
        for x, val, vmax, accent in ((left_x, dn, dsh_max, DSH), (right_x, on, om_max, OM)):
            assert_fits(d, name, 16, label_w - 16, False, f"fig4 label {name}")
            text(d, (x + 8, ry + 9), name, 16, INK)
            bx = x + label_w
            bw = 0 if val == 0 else max(18, bar_max * val / vmax)
            if val:
                rrect(d, 
                    [bx * SCALE, (ry + 8) * SCALE, (bx + bw) * SCALE, (ry + row_h - 12) * SCALE],
                    radius=6 * SCALE, fill=accent,
                )
                text(d, (bx + bw + 10, ry + 9), str(val), 16, accent, True)
            else:
                text(d, (bx + 2, ry + 9), "—", 16, FAINT)

    footer(
        d, y + len(DOMAINS) * row_h + 18,
        "归类为手工判定（同一单元只计一次），可在 scripts/make_openmuse_vs_dsh_figures.py 中审阅。"
        "结论：DSH 的体量集中在客户端 UI、会话、工具与 agent 编排；"
        "OpenMuse 的体量集中在宿主契约、存储同步、账号多设备与 Office 预览——两侧几乎不重叠。",
    )
    return save(img, height, "fig4-domain-distribution.png")


def main():
    os.makedirs(OUT_DIR, exist_ok=True)
    paths = [fig1(), fig2(), fig3(), fig4()]
    for p in paths:
        size = os.path.getsize(p)
        with Image.open(p) as im:
            print(f"{os.path.relpath(p)}  {im.width}x{im.height}  {size/1024:.1f} KiB")
    print("OK")


if __name__ == "__main__":
    sys.exit(main())
