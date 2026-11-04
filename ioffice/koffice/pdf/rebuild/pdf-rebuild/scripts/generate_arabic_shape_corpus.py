#!/usr/bin/env python3
"""Generate the pinned WT6c2b2 Arabic shaping/font corpus."""

import argparse
import hashlib
import subprocess
import tempfile
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "harness" / "assets"
FONT_DIR = ASSETS / "fonts"
FONT = FONT_DIR / "NotoSansArabic-WT6-subset.ttf"
LICENSE = FONT_DIR / "NotoSansArabic-OFL.txt"
OUTPUT = ASSETS / "arabic-shape-corpus.pdf"
SOURCE_URL = "https://raw.githubusercontent.com/google/fonts/main/ofl/notosansarabic/NotoSansArabic%5Bwdth%2Cwght%5D.ttf"
SOURCE_SHA256 = "63111b5b2e074dd48cc67692e0a2726d86ee94c1c37fe8598257b7b4e87e869e"
CORPUS = "سلاملا مُدَرِّسABCofficeafine12Editable PDF text oneSecond line for replacement "


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def obtain_source(explicit: Path | None, temp: Path) -> Path:
    source = explicit if explicit else temp / "NotoSansArabic-source.ttf"
    if not source.is_file():
        urllib.request.urlretrieve(SOURCE_URL, source)
    actual = sha256(source)
    if actual != SOURCE_SHA256:
        raise SystemExit(f"source font SHA-256 mismatch: {actual}")
    return source


def build_static_subset(source: Path) -> None:
    from fontTools import subset
    from fontTools.ttLib import TTFont
    from fontTools.varLib.instancer import instantiateVariableFont

    variable = TTFont(str(source))
    static = instantiateVariableFont(variable, {"wght": 400, "wdth": 100})
    options = subset.Options()
    options.layout_features = ["*"]
    options.name_IDs = [0, 1, 2, 3, 4, 5, 6]
    options.name_legacy = True
    options.notdef_glyph = True
    options.recommended_glyphs = True
    subsetter = subset.Subsetter(options=options)
    subsetter.populate(text=CORPUS)
    subsetter.subset(static)
    static.save(str(FONT))


def chrome_binary(explicit: str | None) -> str:
    candidates = [explicit, "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                  "google-chrome", "chromium"]
    for value in candidates:
        if not value:
            continue
        if "/" not in value or Path(value).is_file():
            return value
    raise SystemExit("Chrome/Chromium is required; pass --chrome")


def build_pdf(chrome: str, temp: Path) -> None:
    html = temp / "arabic-shape-corpus.html"
    raw = temp / "chrome.pdf"
    html.write_text(f'''<!doctype html><meta charset="utf-8"><style>
@page {{ size: 8.5in 11in; margin: 0; }}
@font-face {{ font-family: NotoArabic; src: url('{FONT.as_uri()}'); }}
html,body {{ margin:0; width:8.5in; height:11in; font-family:Arial,sans-serif; }}
h1 {{ position:absolute; left:40pt; top:28pt; font-size:13pt; font-weight:400; }}
.label {{ position:absolute; left:40pt; font-size:9pt; }}
.value {{ position:absolute; right:72pt; width:260pt; text-align:right; direction:rtl; unicode-bidi:isolate; font:24pt NotoArabic; }}
#l1,#v1 {{ top:88pt }} #l2,#v2 {{ top:148pt }} #l3,#v3 {{ top:208pt }} #l4,#v4 {{ top:268pt }}
#l5 {{ top:328pt }} #v5 {{ position:absolute; left:180pt; top:320pt; font:24pt NotoArabic; }}
</style><h1>WT6c2b2 Arabic shaping and glyph-cluster oracle</h1>
<div id=l1 class=label>ARABIC</div><div id=v1 class=value>سلام</div>
<div id=l2 class=label>LAM_ALEF</div><div id=v2 class=value>لا</div>
<div id=l3 class=label>MARKS</div><div id=v3 class=value>مُدَرِّس</div>
<div id=l4 class=label>MIXED</div><div id=v4 class=value>سلام-12-ABC</div>
<div id=l5 class=label>LATIN_LIGATURE</div><div id=v5>office affine</div>''', encoding="utf-8")
    subprocess.run([chrome, "--headless", "--disable-gpu", "--no-pdf-header-footer",
                    f"--print-to-pdf={raw}", html.as_uri()], check=True,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    from pypdf import PdfReader, PdfWriter
    reader = PdfReader(raw)
    writer = PdfWriter()
    for page in reader.pages:
        writer.add_page(page)
    writer.add_metadata({"/Title": "WT6c2b2 Arabic shaping and glyph clusters",
                         "/Producer": "pdf-rebuild deterministic corpus generator",
                         "/CreationDate": "D:20000101000000Z",
                         "/ModDate": "D:20000101000000Z"})
    with OUTPUT.open("wb") as stream:
        writer.write(stream)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source-font", type=Path)
    parser.add_argument("--chrome")
    parser.add_argument("--subset-only", action="store_true")
    parser.add_argument("--pdf-only", action="store_true")
    args = parser.parse_args()
    FONT_DIR.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="wt6c2b2-") as directory:
        temp = Path(directory)
        if not args.pdf_only:
            build_static_subset(obtain_source(args.source_font, temp))
        if not args.subset_only:
            if not FONT.is_file():
                raise SystemExit(f"missing {FONT}; generate the subset first")
            build_pdf(chrome_binary(args.chrome), temp)
    print(f"font={FONT} sha256={sha256(FONT)}")
    print(f"pdf={OUTPUT} sha256={sha256(OUTPUT)}")
    print(f"license={LICENSE}")


if __name__ == "__main__":
    main()
