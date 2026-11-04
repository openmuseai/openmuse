#!/usr/bin/env python3
"""Generate the deterministic WT6a CJK fallback/reflow fixture."""

from pathlib import Path

from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.pdfgen import canvas


ROOT = Path(__file__).resolve().parents[1]
FONT = ROOT / "harness" / "assets" / "fonts" / "NotoSansSC-WT6-subset.ttf"
OUTPUT = ROOT / "harness" / "assets" / "complex-text-corpus.pdf"


def main() -> None:
    pdfmetrics.registerFont(TTFont("NotoSansSC-WT6", str(FONT)))
    c = canvas.Canvas(str(OUTPUT), pagesize=(612, 792), pageCompression=0, invariant=1)
    c.setTitle("WT6a CJK font fallback")
    c.setFont("Helvetica", 14)
    c.drawString(72, 700, "English prefix and suffix")
    c.setFont("NotoSansSC-WT6", 14)
    # Keep every glyph used by the edit oracle in the embedded subset. PDF
    # fonts expose an encoding fixed at creation time; WT6a verifies fallback
    # to a compatible font already present in the document.
    c.drawString(72, 678, "中文段落用于字体回退新增内容自动换行")
    c.setFont("Helvetica", 14)
    c.drawString(72, 656, "Second English line")
    c.showPage()
    c.save()
    print(OUTPUT)


if __name__ == "__main__":
    main()
