#!/usr/bin/env python3
"""Generate the deterministic WT6c2a logical-RTL geometry fixture."""

from pathlib import Path

from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.pdfgen import canvas


ROOT = Path(__file__).resolve().parents[1]
FONT = ROOT / "harness" / "assets" / "fonts" / "NotoSansHebrew-WT6-subset.ttf"
OUTPUT = ROOT / "harness" / "assets" / "rtl-edit-corpus.pdf"


def main() -> None:
    pdfmetrics.registerFont(TTFont("NotoSansHebrew-WT6", str(FONT)))
    c = canvas.Canvas(str(OUTPUT), pagesize=(612, 792), pageCompression=0, invariant=1)
    c.setTitle("WT6c2a logical RTL geometry")

    c.setFont("Helvetica", 14)
    c.drawString(72, 730, "LTR control: edit geometry")

    # PDF strings keep logical UTF-16 order. A negative character spacing
    # makes successive Hebrew glyph origins move from right to left, producing
    # the geometry contract exercised by the original native editor without
    # relying on a renderer-side string reversal.
    text = c.beginText(500, 680)
    text.setFont("NotoSansHebrew-WT6", 20)
    text.setCharSpace(-28)
    text.textOut("שלום")
    c.drawText(text)

    # A second object includes a neutral separator and ASCII digits. The base
    # direction must still be taken from the first strong Hebrew character.
    mixed = c.beginText(500, 630)
    mixed.setFont("NotoSansHebrew-WT6", 20)
    mixed.setCharSpace(-28)
    mixed.textOut("עולם-12")
    c.drawText(mixed)

    c.showPage()
    c.save()
    print(OUTPUT)


if __name__ == "__main__":
    main()
