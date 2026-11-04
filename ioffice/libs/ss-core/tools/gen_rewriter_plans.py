#!/usr/bin/env python3
"""
Generate xxx_rewriter_plan.md files for all Phase 1 leaf directories.
Scans Java source directories, counts files/lines, determines types,
and creates plan markdown files in the corresponding Rust directories.
"""

import os
import re
import sys
from pathlib import Path

WORKSPACE = Path(r"d:\agentic\src\wordOffice")

# Phase 1 batch definitions: (batch_id, java_pkg_label, java_dir_relative_to_src, rust_dir_relative_to_ss_core_src, plan_filename)
# java_src_base tells which Java repo to look in
PHASE1_BATCHES = [
    # --- moss.util.* (core/src) ---
    ("1.1", "moss.util", "cn/wps/moss/util", "util", "util_rewriter_plan.md", "core"),
    ("1.2", "moss.util.binarytree", "cn/wps/moss/util/binarytree", "util/binarytree", "binarytree_rewriter_plan.md", "core"),
    ("1.3", "moss.util.binarytree.node", "cn/wps/moss/util/binarytree/node", "util/binarytree/node", "node_rewriter_plan.md", "core"),
    ("1.4", "moss.util.cache", "cn/wps/moss/util/cache", "util/cache", "cache_rewriter_plan.md", "core"),
    ("1.5", "moss.util.checksum", "cn/wps/moss/util/checksum", "util/checksum", "checksum_rewriter_plan.md", "core"),
    ("1.6", "moss.util.deqlist", "cn/wps/moss/util/deqlist", "util/deqlist", "deqlist_rewriter_plan.md", "core"),
    ("1.7", "moss.util.objpool", "cn/wps/moss/util/objpool", "util/objpool", "objpool_rewriter_plan.md", "core"),
    ("1.8", "moss.util.primitive", "cn/wps/moss/util/primitive", "util/primitive", "primitive_rewriter_plan.md", "core"),
    ("1.9", "moss.util.rangelist", "cn/wps/moss/util/rangelist", "util/rangelist", "rangelist_rewriter_plan.md", "core"),
    # --- moss root + framework + engine (core/src) ---
    ("1.10", "moss", "cn/wps/moss", "moss", "moss_rewriter_plan.md", "core"),
    ("1.11", "moss.framework", "cn/wps/moss/framework", "framework", "framework_rewriter_plan.md", "core"),
    ("1.12", "moss.engine.grid", "cn/wps/moss/engine/grid", "engine/grid", "grid_rewriter_plan.md", "core"),
    ("1.13", "moss.engine.grid.griditerator", "cn/wps/moss/engine/grid/griditerator", "engine/grid/griditerator", "griditerator_rewriter_plan.md", "core"),
    ("1.14", "moss.engine.eval", "cn/wps/moss/engine/eval", "engine/eval", "eval_rewriter_plan.md", "core"),
    ("1.15", "moss.engine.eval.sup", "cn/wps/moss/engine/eval/sup", "engine/eval/sup", "sup_rewriter_plan.md", "core"),
    ("1.16", "moss.engine.linktb", "cn/wps/moss/engine/linktb", "engine/linktb", "linktb_rewriter_plan.md", "core"),
    ("1.17", "moss.engine.areaevent", "cn/wps/moss/engine/areaevent", "engine/areaevent", "areaevent_rewriter_plan.md", "core"),
    # --- moss.core.* + moss.app core (core/src) ---
    ("1.18", "moss.core.book", "cn/wps/moss/core/book", "core/book", "book_rewriter_plan.md", "core"),
    ("1.19", "moss.core.sheet", "cn/wps/moss/core/sheet", "core/sheet", "sheet_rewriter_plan.md", "core"),
    ("1.20", "moss.core.table", "cn/wps/moss/core/table", "core/table", "table_rewriter_plan.md", "core"),
    ("1.21", "moss.core.theme", "cn/wps/moss/core/theme", "core/theme", "theme_rewriter_plan.md", "core"),
    ("1.22", "moss.core.drawing", "cn/wps/moss/core/drawing", "core/drawing", "drawing_rewriter_plan.md", "core"),
    ("1.23", "moss.core.drawing.tools", "cn/wps/moss/core/drawing/tools", "core/drawing/tools", "tools_rewriter_plan.md", "core"),
    ("1.24", "moss.app", "cn/wps/moss/app", "app", "app_rewriter_plan.md", "core"),
    ("1.25", "moss.app.event", "cn/wps/moss/app/event", "app/event", "event_rewriter_plan.md", "core"),
    ("1.26", "moss.app.except", "cn/wps/moss/app/except", "app/except", "except_rewriter_plan.md", "core"),
    ("1.27", "moss.app.xf", "cn/wps/moss/app/xf", "app/xf", "xf_rewriter_plan.md", "core"),
    # --- moffice.drawing.data.* (core/src) ---
    ("1.28", "moffice.drawing.data.basic", "cn/wps/moffice/drawing/data/basic", "drawing/data/basic", "basic_rewriter_plan.md", "core"),
    ("1.29", "moffice.drawing.data.blip", "cn/wps/moffice/drawing/data/blip", "drawing/data/blip", "blip_rewriter_plan.md", "core"),
    ("1.30", "moffice.drawing.data.color", "cn/wps/moffice/drawing/data/color", "drawing/data/color", "color_rewriter_plan.md", "core"),
    ("1.31", "moffice.drawing.data.effect", "cn/wps/moffice/drawing/data/effect", "drawing/data/effect", "effect_rewriter_plan.md", "core"),
    ("1.32", "moffice.drawing.data.fill", "cn/wps/moffice/drawing/data/fill", "drawing/data/fill", "fill_rewriter_plan.md", "core"),
    ("1.33", "moffice.drawing.data.font", "cn/wps/moffice/drawing/data/font", "drawing/data/font", "font_rewriter_plan.md", "core"),
    ("1.34", "moffice.drawing.data.geometry", "cn/wps/moffice/drawing/data/geometry", "drawing/data/geometry", "geometry_rewriter_plan.md", "core"),
    ("1.35", "moffice.drawing.data.hyperlink", "cn/wps/moffice/drawing/data/hyperlink", "drawing/data/hyperlink", "hyperlink_rewriter_plan.md", "core"),
    ("1.36", "moffice.drawing.data.media", "cn/wps/moffice/drawing/data/media", "drawing/data/media", "media_rewriter_plan.md", "core"),
    ("1.37", "moffice.drawing.data.outline", "cn/wps/moffice/drawing/data/outline", "drawing/data/outline", "outline_rewriter_plan.md", "core"),
    ("1.38", "moffice.drawing.data.shape", "cn/wps/moffice/drawing/data/shape", "drawing/data/shape", "shape_rewriter_plan.md", "core"),
    ("1.39", "moffice.drawing.data.style", "cn/wps/moffice/drawing/data/style", "drawing/data/style", "style_rewriter_plan.md", "core"),
    ("1.40", "moffice.drawing.data.table", "cn/wps/moffice/drawing/data/table", "drawing/data/table", "drawing_table_rewriter_plan.md", "core"),
    ("1.41", "moffice.drawing.data.text", "cn/wps/moffice/drawing/data/text", "drawing/data/text", "drawing_data_text_rewriter_plan.md", "core"),
    ("1.42", "moffice.drawing.data.threeD", "cn/wps/moffice/drawing/data/threeD", "drawing/data/three_d", "three_d_rewriter_plan.md", "core"),
    ("1.43", "moffice.drawing", "cn/wps/moffice/drawing", "drawing", "drawing_root_rewriter_plan.md", "core"),
    # --- moffice.kso.* (core/src) ---
    ("1.44", "moffice.kso.kproto", "cn/wps/moffice/kso/kproto", "kso/kproto", "kproto_rewriter_plan.md", "core"),
    ("1.45", "moffice.kso.kproto.internal", "cn/wps/moffice/kso/kproto/internal", "kso/kproto/internal", "internal_rewriter_plan.md", "core"),
    ("1.46", "moffice.kso.kproto.memory", "cn/wps/moffice/kso/kproto/memory", "kso/kproto/memory", "memory_rewriter_plan.md", "core"),
    ("1.47", "moffice.kso.kproto.util", "cn/wps/moffice/kso/kproto/util", "kso/kproto/util", "kproto_util_rewriter_plan.md", "core"),
    ("1.48", "moffice.kso", "cn/wps/moffice/kso", "kso", "kso_rewriter_plan.md", "core"),
    ("1.49", "moffice.kso.unit", "cn/wps/moffice/kso/unit", "kso/unit", "unit_rewriter_plan.md", "core"),
    ("1.50", "moffice.pool", "cn/wps/moffice/pool", "pool", "pool_rewriter_plan.md", "core"),
    ("1.51", "moffice.util", "cn/wps/moffice/util", "moffice_util", "moffice_util_rewriter_plan.md", "core"),
    ("1.52", "poi.ss", "org/apache/poi/ss", "poi/ss", "ss_rewriter_plan.md", "core"),
    ("1.53", "poi.ss.util", "org/apache/poi/ss/util", "poi/ss/util", "poi_util_rewriter_plan.md", "core"),
    ("1.54", "poi.ss.usermodel", "org/apache/poi/ss/usermodel", "poi/ss/usermodel", "usermodel_rewriter_plan.md", "core"),
    # --- chart.data.* (et-core-chart/src) ---
    ("1.55", "chart", "cn/wps/chart", "chart", "chart_root_rewriter_plan.md", "chart"),
    ("1.56", "chart.data", "cn/wps/chart/data", "chart/data", "chart_data_rewriter_plan.md", "chart"),
    ("1.57", "chart.data.axis", "cn/wps/chart/data/axis", "chart/data/axis", "axis_rewriter_plan.md", "chart"),
    ("1.58", "chart.data.base", "cn/wps/chart/data/base", "chart/data/base", "base_rewriter_plan.md", "chart"),
    ("1.59", "chart.data.chart", "cn/wps/chart/data/chart", "chart/data/chart", "chart_chart_rewriter_plan.md", "chart"),
    ("1.60", "chart.data.chartspace", "cn/wps/chart/data/chartspace", "chart/data/chartspace", "chartspace_rewriter_plan.md", "chart"),
    ("1.61", "chart.data.d3", "cn/wps/chart/data/d3", "chart/data/d3", "d3_rewriter_plan.md", "chart"),
    ("1.62", "chart.data.dispunits", "cn/wps/chart/data/dispunits", "chart/data/dispunits", "dispunits_rewriter_plan.md", "chart"),
    ("1.63", "chart.data.label", "cn/wps/chart/data/label", "chart/data/label", "label_rewriter_plan.md", "chart"),
    ("1.64", "chart.data.layout", "cn/wps/chart/data/layout", "chart/data/layout", "layout_rewriter_plan.md", "chart"),
    ("1.65", "chart.data.legend", "cn/wps/chart/data/legend", "chart/data/legend", "legend_rewriter_plan.md", "chart"),
    ("1.66", "chart.data.pivot", "cn/wps/chart/data/pivot", "chart/data/pivot", "pivot_rewriter_plan.md", "chart"),
    ("1.67", "chart.data.print", "cn/wps/chart/data/print", "chart/data/print", "print_rewriter_plan.md", "chart"),
    ("1.68", "chart.data.series", "cn/wps/chart/data/series", "chart/data/series", "series_rewriter_plan.md", "chart"),
    ("1.69", "chart.data.shape", "cn/wps/chart/data/shape", "chart/data/shape", "chart_shape_rewriter_plan.md", "chart"),
    ("1.70", "chart.data.source", "cn/wps/chart/data/source", "chart/data/source", "source_rewriter_plan.md", "chart"),
    ("1.71", "chart.data.str", "cn/wps/chart/data/str", "chart/data/str", "str_rewriter_plan.md", "chart"),
    ("1.72", "chart.data.trend", "cn/wps/chart/data/trend", "chart/data/trend", "trend_rewriter_plan.md", "chart"),
    ("1.73", "chart.msenum", "cn/wps/chart/msenum", "chart/msenum", "msenum_rewriter_plan.md", "chart"),
    ("1.74", "chart.common", "cn/wps/chart/common", "chart/common", "common_rewriter_plan.md", "chart"),
    ("1.75", "chart.store", "cn/wps/chart/store", "chart/store", "store_rewriter_plan.md", "chart"),
    ("1.76", "chart.objpool", "cn/wps/chart/objpool", "chart/objpool", "objpool_rewriter_plan.md", "chart"),
    ("1.77", "chart.style", "cn/wps/chart/style", "chart/style", "chart_style_rewriter_plan.md", "chart"),
    ("1.78", "chart.format", "cn/wps/chart/format", "chart/format", "format_rewriter_plan.md", "chart"),
    ("1.79", "chart.description", "cn/wps/chart/description", "chart/description", "description_rewriter_plan.md", "chart"),
    ("1.80", "chart.event", "cn/wps/chart/event", "chart/event", "chart_event_rewriter_plan.md", "chart"),
    # --- ss.formula.* (et-core-formula/src) ---
    ("1.81", "ss.constant", "cn/wps/et/ss/constant", "formula/constant", "constant_rewriter_plan.md", "formula"),
    ("1.82", "ss.interfaces", "cn/wps/et/ss/interfaces", "formula/interfaces", "interfaces_rewriter_plan.md", "formula"),
    ("1.83", "ss.formula", "cn/wps/et/ss/formula", "formula", "formula_rewriter_plan.md", "formula"),
    ("1.84", "ss.formula.constant", "cn/wps/et/ss/formula/constant", "formula/formula_constant", "formula_constant_rewriter_plan.md", "formula"),
    ("1.85", "ss.formula.ptg", "cn/wps/et/ss/formula/ptg", "formula/ptg", "ptg_rewriter_plan.md", "formula"),
    ("1.86", "ss.formula.ptg.external", "cn/wps/et/ss/formula/ptg/external", "formula/ptg/external", "external_rewriter_plan.md", "formula"),
    ("1.87", "ss.formula.parser", "cn/wps/et/ss/formula/parser", "formula/parser", "parser_rewriter_plan.md", "formula"),
    ("1.88", "ss.formula.parser.rav", "cn/wps/et/ss/formula/parser/rav", "formula/parser/rav", "rav_rewriter_plan.md", "formula"),
    ("1.89", "ss.formula.eval", "cn/wps/et/ss/formula/eval", "formula/eval", "eval_rewriter_plan.md", "formula"),
    ("1.90", "ss.formula.eval.advance", "cn/wps/et/ss/formula/eval/advance", "formula/eval/advance", "advance_rewriter_plan.md", "formula"),
    ("1.91", "ss.formula.evalcache", "cn/wps/et/ss/formula/evalcache", "formula/evalcache", "evalcache_rewriter_plan.md", "formula"),
    ("1.92", "ss.formula.evaluator", "cn/wps/et/ss/formula/evaluator", "formula/evaluator", "evaluator_rewriter_plan.md", "formula"),
    ("1.93", "ss.formula.function", "cn/wps/et/ss/formula/function", "formula/function", "function_rewriter_plan.md", "formula"),
    ("1.94", "ss.formula.functions", "cn/wps/et/ss/formula/functions", "formula/functions", "functions_rewriter_plan.md", "formula"),
    ("1.95", "ss.formula.functions.base", "cn/wps/et/ss/formula/functions/base", "formula/functions/base", "functions_base_rewriter_plan.md", "formula"),
    ("1.96", "ss.formula.functions.builtin", "cn/wps/et/ss/formula/functions/builtin", "formula/functions/builtin", "builtin_rewriter_plan.md", "formula"),
    ("1.97", "ss.formula.functions.calendar", "cn/wps/et/ss/formula/functions/calendar", "formula/functions/calendar", "calendar_rewriter_plan.md", "formula"),
    ("1.98", "ss.formula.functions.db", "cn/wps/et/ss/formula/functions/db", "formula/functions/db", "db_rewriter_plan.md", "formula"),
    ("1.99", "ss.formula.functions.financial", "cn/wps/et/ss/formula/functions/financial", "formula/functions/financial", "financial_rewriter_plan.md", "formula"),
    ("1.100", "ss.formula.functions.matrix", "cn/wps/et/ss/formula/functions/matrix", "formula/functions/matrix", "matrix_rewriter_plan.md", "formula"),
    ("1.101", "ss.formula.functions.statistics", "cn/wps/et/ss/formula/functions/statistics", "formula/functions/statistics", "statistics_rewriter_plan.md", "formula"),
    ("1.102", "ss.formula.functions.util", "cn/wps/et/ss/formula/functions/util", "formula/functions/util", "functions_util_rewriter_plan.md", "formula"),
    ("1.103a", "ss.formula.functions.util.db", "cn/wps/et/ss/formula/functions/util/db", "formula/functions/util/db", "util_db_rewriter_plan.md", "formula"),
    ("1.103b", "ss.formula.functions.util.financial", "cn/wps/et/ss/formula/functions/util/financial", "formula/functions/util/financial", "util_financial_rewriter_plan.md", "formula"),
    ("1.103c", "ss.formula.functions.util.iteration", "cn/wps/et/ss/formula/functions/util/iteration", "formula/functions/util/iteration", "util_iteration_rewriter_plan.md", "formula"),
    ("1.103d", "ss.formula.functions.util.matcher", "cn/wps/et/ss/formula/functions/util/matcher", "formula/functions/util/matcher", "util_matcher_rewriter_plan.md", "formula"),
    ("1.103e", "ss.formula.functions.util.numbercollector", "cn/wps/et/ss/formula/functions/util/numbercollector", "formula/functions/util/numbercollector", "util_numbercollector_rewriter_plan.md", "formula"),
    ("1.103f", "ss.formula.functions.util.statistics", "cn/wps/et/ss/formula/functions/util/statistics", "formula/functions/util/statistics", "util_statistics_rewriter_plan.md", "formula"),
    ("1.103g", "ss.formula.functions.util.statistics.toms708", "cn/wps/et/ss/formula/functions/util/statistics/toms708", "formula/functions/util/statistics/toms708", "toms708_rewriter_plan.md", "formula"),
    ("1.104", "ss.formula.atp", "cn/wps/et/ss/formula/atp", "formula/atp", "atp_rewriter_plan.md", "formula"),
    ("1.105", "ss.formula.util", "cn/wps/et/ss/formula/util", "formula/util", "formula_util_rewriter_plan.md", "formula"),
    ("1.106", "ss.calcchain", "cn/wps/et/ss/calcchain", "formula/calcchain", "calcchain_rewriter_plan.md", "formula"),
    ("1.107a", "ss.calcchain.listener", "cn/wps/et/ss/calcchain/listener", "formula/calcchain/listener", "listener_rewriter_plan.md", "formula"),
    ("1.107b", "ss.calcchain.marktype", "cn/wps/et/ss/calcchain/marktype", "formula/calcchain/marktype", "marktype_rewriter_plan.md", "formula"),
    ("1.107c", "ss.calcchain.threadpool", "cn/wps/et/ss/calcchain/threadpool", "formula/calcchain/threadpool", "threadpool_rewriter_plan.md", "formula"),
    ("1.108", "ss.dependent", "cn/wps/et/ss/dependent", "formula/dependent", "dependent_rewriter_plan.md", "formula"),
    ("1.109", "ss.dependent.refer", "cn/wps/et/ss/dependent/refer", "formula/dependent/refer", "refer_rewriter_plan.md", "formula"),
    ("1.110", "ss.dependent.refer.base", "cn/wps/et/ss/dependent/refer/base", "formula/dependent/refer/base", "refer_base_rewriter_plan.md", "formula"),
    ("1.111", "ss.multlang", "cn/wps/et/ss/multlang", "formula/multlang", "multlang_rewriter_plan.md", "formula"),
    ("1.112", "ss.multlang.res", "cn/wps/et/ss/multlang/res", "formula/multlang/res", "res_rewriter_plan.md", "formula"),
    ("1.113", "ss.util", "cn/wps/et/ss/util", "formula/ss_util", "ss_util_rewriter_plan.md", "formula"),
    ("1.114", "ss.util.indexmap", "cn/wps/et/ss/util/indexmap", "formula/ss_util/indexmap", "indexmap_rewriter_plan.md", "formula"),
    ("1.115", "ss.util.level", "cn/wps/et/ss/util/level", "formula/ss_util/level", "level_rewriter_plan.md", "formula"),
    ("1.116", "ss.util.primeval.objmap", "cn/wps/et/ss/util/primeval/objmap", "formula/ss_util/primeval/objmap", "objmap_rewriter_plan.md", "formula"),
]

JAVA_BASES = {
    "core": WORKSPACE / "koffice" / "et" / "core" / "src",
    "io": WORKSPACE / "koffice" / "et" / "et-core-io" / "src",
    "chart": WORKSPACE / "koffice" / "et" / "et-core-chart" / "src",
    "formula": WORKSPACE / "koffice" / "et" / "et-core-formula" / "src",
}

RUST_BASE = WORKSPACE / "libs" / "ss-core" / "src"


def pascal_to_snake(name: str) -> str:
    s1 = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s2 = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s1)
    return s2.lower()


def detect_java_type(filepath: Path) -> str:
    try:
        content = filepath.read_text(encoding='utf-8', errors='replace')
    except Exception:
        return "class"
    if re.search(r'\binterface\s+\w+', content):
        return "interface"
    if re.search(r'\benum\s+\w+', content):
        return "enum"
    if re.search(r'\babstract\s+class\s+', content):
        return "abstract class"
    return "class"


def count_lines(filepath: Path) -> int:
    try:
        return len(filepath.read_text(encoding='utf-8', errors='replace').splitlines())
    except Exception:
        return 0


def get_java_files_only_in_dir(java_dir: Path) -> list:
    """Get only .java files directly in this directory (not subdirs)."""
    if not java_dir.exists():
        return []
    return sorted([f for f in java_dir.iterdir() if f.is_file() and f.suffix == '.java'])


def generate_plan(batch_id, java_pkg, java_dir, rust_dir, plan_filename, java_files):
    total_lines = 0
    rows = []
    for i, jf in enumerate(java_files, 1):
        lines = count_lines(jf)
        total_lines += lines
        jtype = detect_java_type(jf)
        rust_name = pascal_to_snake(jf.stem) + ".rs"
        rows.append((i, jf.name, lines, jtype, rust_name))

    content = f"# {java_pkg} 目录重写计划\n\n"
    content += f"## Java 源码目录\n\n"
    content += f"`{java_dir}` ({len(java_files)} files, ~{total_lines} lines)\n\n"
    content += f"## Rust 目标目录\n\n"
    content += f"`{rust_dir}`\n\n"
    content += f"## 进度概览\n\n"
    content += f"- 已重写: 0/{len(java_files)}\n"
    content += f"- 待重写: {len(java_files)}/{len(java_files)}\n"
    content += f"- 不重写: 0/{len(java_files)}\n\n"
    content += f"## 文件清单与进度\n\n"
    content += "| # | Java 文件 | Java 行数 | 类型 | Rust 文件 | 状态 | 验证 | 备注 |\n"
    content += "|---|-----------|--------:|------|-----------|:----:|:----:|------|\n"
    for (i, jname, lines, jtype, rname) in rows:
        content += f"| {i} | `{jname}` | {lines} | {jtype} | `{rname}` | ⬜ 待重写 | | |\n"

    content += f"\n## 验证命令\n\n"
    content += f"```bash\n"
    content += f"python libs/word-core/tools/verify_rewrite.py \\\n"
    content += f"    \"{java_dir}\" \\\n"
    content += f"    \"{rust_dir}\"\n"
    content += f"```\n\n"
    content += f"## 验证结果\n\n"
    content += f"尚未验证\n"
    return content


def main():
    created = []
    skipped = []

    for batch in PHASE1_BATCHES:
        batch_id, java_pkg, java_rel, rust_rel, plan_name, repo = batch
        java_base = JAVA_BASES[repo]
        java_dir = java_base / java_rel.replace("/", os.sep)
        rust_dir = RUST_BASE / rust_rel.replace("/", os.sep)

        java_files = get_java_files_only_in_dir(java_dir)
        if not java_files:
            print(f"[SKIP] {batch_id} {java_pkg}: no .java files in {java_dir}")
            skipped.append(batch_id)
            continue

        os.makedirs(rust_dir, exist_ok=True)

        plan_path = rust_dir / plan_name
        if plan_path.exists():
            print(f"[EXISTS] {batch_id} {java_pkg}: {plan_path}")
            created.append((batch_id, java_pkg, str(rust_dir), plan_name, len(java_files)))
            continue

        content = generate_plan(batch_id, java_pkg, java_dir, rust_dir, plan_name, java_files)
        plan_path.write_text(content, encoding='utf-8')
        print(f"[CREATED] {batch_id} {java_pkg}: {plan_path} ({len(java_files)} files)")
        created.append((batch_id, java_pkg, str(rust_dir), plan_name, len(java_files)))

    print(f"\n=== Summary ===")
    print(f"Created/Existing: {len(created)}")
    print(f"Skipped (no java files): {len(skipped)}")

    # Output table for RUST_REWRITE_PLAN.md section 6
    print(f"\n=== Section 6 table entries ===")
    for (bid, pkg, rdir, pname, nfiles) in created:
        rdir_short = rdir.replace(str(RUST_BASE), "src").replace("\\", "/")
        print(f"| `{pkg}` | `{rdir_short}/` | `{pname}` | ⬜ 待重写 ({nfiles} files) |")


if __name__ == "__main__":
    main()
