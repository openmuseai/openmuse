#!/usr/bin/env python3
"""Generate the deterministic CE2.2 glyph hit/caret oracle PDF."""

from pathlib import Path

from reportlab.pdfgen import canvas


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "harness" / "assets" / "glyph-caret-corpus.pdf"


def main() -> None:
    pdf = canvas.Canvas(str(OUTPUT), pagesize=(612, 792), pageCompression=0, invariant=1)
    pdf.setTitle("CE2.2 glyph caret corpus")
    pdf.setFont("Helvetica", 18)
    pdf.drawString(72, 700, "Wim il1 MW")
    pdf.saveState()
    pdf.translate(300, 400)
    pdf.rotate(25)
    pdf.setFont("Helvetica", 18)
    pdf.drawString(0, 0, "Rotate caret")
    pdf.restoreState()
    pdf.showPage()
    pdf.save()
    print(OUTPUT)


if __name__ == "__main__":
    main()
