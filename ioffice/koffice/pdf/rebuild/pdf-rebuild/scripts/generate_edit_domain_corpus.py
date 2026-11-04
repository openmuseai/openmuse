#!/usr/bin/env python3
"""Generate deterministic CE1.2c PDF editor-domain fixtures."""

from pathlib import Path

from pypdf import PdfReader, PdfWriter
from pypdf.generic import RectangleObject
from reportlab.pdfgen import canvas


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "harness" / "assets" / "edit-domain-corpus"
PAGE = (612, 792)


def base_canvas(name: str) -> canvas.Canvas:
    OUT.mkdir(parents=True, exist_ok=True)
    value = canvas.Canvas(str(OUT / name), pagesize=PAGE, pageCompression=0, invariant=1)
    value.setTitle(f"CE1.2c {name}")
    return value


def line(c: canvas.Canvas, text: str, x: float, y: float, size: float = 12) -> None:
    c.setFont("Helvetica", size)
    c.drawString(x, y, text)


def multicolumn() -> None:
    c = base_canvas("multicolumn.pdf")
    line(c, "LEFT COLUMN ALPHA", 54, 700)
    line(c, "left column beta", 54, 682)
    line(c, "left column gamma", 54, 664)
    line(c, "RIGHT COLUMN ONE", 330, 700)
    line(c, "right column two", 330, 682)
    line(c, "right column three", 330, 664)
    line(c, "FULL WIDTH FOOTER", 210, 100, 10)
    c.showPage()
    c.save()


def rotated() -> None:
    c = base_canvas("rotated.pdf")
    line(c, "HORIZONTAL ALPHA", 80, 700)
    line(c, "horizontal beta", 80, 682)
    c.saveState()
    c.translate(470, 180)
    c.rotate(90)
    line(c, "ROTATED NINETY ONE", 0, 0)
    line(c, "rotated ninety two", 0, -18)
    c.restoreState()
    c.saveState()
    c.translate(260, 400)
    c.rotate(25)
    line(c, "SLANTED TWENTY FIVE", 0, 0)
    c.restoreState()
    c.showPage()
    c.save()


def cropbox() -> None:
    uncropped = OUT / "cropbox-source.pdf"
    c = canvas.Canvas(str(uncropped), pagesize=PAGE, pageCompression=0, invariant=1)
    c.setTitle("CE1.2c cropbox")
    line(c, "VISIBLE CROP ALPHA", 72, 650)
    line(c, "visible crop beta", 72, 632)
    line(c, "OUTSIDE LEFT", 2, 400)
    line(c, "OUTSIDE TOP", 200, 760)
    c.showPage()
    c.save()
    reader = PdfReader(uncropped)
    writer = PdfWriter()
    writer.clone_document_from_reader(reader)
    writer.pages[0].cropbox = RectangleObject((36, 72, 576, 720))
    with (OUT / "cropbox.pdf").open("wb") as stream:
        writer.write(stream)
    uncropped.unlink()


def form_xobject() -> None:
    c = base_canvas("form-xobject.pdf")
    c.beginForm("EDITABLE_FORM", 0, 0, 240, 100)
    line(c, "FORM TEXT ALPHA", 0, 70)
    line(c, "form text beta", 0, 52)
    c.endForm()
    c.saveState()
    c.translate(90, 520)
    c.scale(1.2, 1.2)
    c.doForm("EDITABLE_FORM")
    c.restoreState()
    line(c, "TOP LEVEL TEXT", 90, 420)
    c.showPage()
    c.save()


if __name__ == "__main__":
    multicolumn()
    rotated()
    cropbox()
    form_xobject()
    for item in sorted(OUT.glob("*.pdf")):
        reader = PdfReader(item)
        if len(reader.pages) != 1:
            raise RuntimeError(f"unexpected page count: {item}")
        print(item)
