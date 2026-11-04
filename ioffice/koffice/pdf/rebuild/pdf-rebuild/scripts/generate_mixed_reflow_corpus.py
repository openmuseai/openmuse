#!/usr/bin/env python3
"""Generate the deterministic WT5c mixed-style paragraph fixture."""

from pathlib import Path

from reportlab.pdfbase.pdfmetrics import stringWidth
from reportlab.pdfgen import canvas


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "harness" / "assets" / "mixed-reflow-corpus.pdf"


def fragment(c: canvas.Canvas, text: str, font: str, size: float,
             x: float, y: float) -> float:
    c.setFont(font, size)
    c.setFillColorRGB(0, 0, 0)
    c.drawString(x, y, text)
    return x + stringWidth(text, font, size)


def main() -> None:
    c = canvas.Canvas(str(OUTPUT), pagesize=(612, 792), pageCompression=0, invariant=1)
    c.setTitle("WT5c mixed style reflow")
    x = fragment(c, "Regular alpha ", "Helvetica", 12, 72, 700)
    x = fragment(c, "BOLD BETA", "Helvetica-Bold", 12, x, 700)
    fragment(c, " regular gamma", "Helvetica", 12, x, 700)
    x = fragment(c, "Second delta ", "Helvetica", 12, 72, 682)
    x = fragment(c, "italic epsilon", "Helvetica-Oblique", 12, x, 682)
    fragment(c, " regular zeta", "Helvetica", 12, x, 682)
    c.showPage()
    c.save()
    print(OUTPUT)


if __name__ == "__main__":
    main()
