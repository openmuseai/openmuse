#!/usr/bin/env python3
"""Generate the deterministic CE4.2 shared Form XObject editing corpus."""

from pathlib import Path

from pypdf import PdfReader
import reportlab
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.pdfgen import canvas


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "harness" / "assets" / "form-edit-corpus.pdf"
PAGE = (612, 792)


def place(c: canvas.Canvas, x: float, y: float) -> None:
    c.saveState()
    c.translate(x, y)
    c.doForm("SHARED_EDIT_FORM")
    c.restoreState()


def main() -> None:
    vera = Path(reportlab.__file__).resolve().parent / "fonts" / "Vera.ttf"
    pdfmetrics.registerFont(TTFont("CorpusVera", str(vera)))
    c = canvas.Canvas(str(OUT), pagesize=PAGE, pageCompression=0, invariant=1)
    c.setTitle("CE4.2 shared Form XObject editing corpus")
    c.beginForm("SHARED_EDIT_FORM", 0, 0, 220, 54)
    c.setFont("CorpusVera", 14)
    c.drawString(10, 20, "SHARED FORM TEXT")
    c.endForm()

    c.setFont("Helvetica", 10)
    c.drawString(72, 742, "PAGE ONE: two references to the same Form XObject")
    place(c, 72, 650)
    place(c, 300, 650)
    c.showPage()

    c.setFont("Helvetica", 10)
    c.drawString(72, 742, "PAGE TWO: a third reference to the same Form XObject")
    place(c, 72, 650)
    c.showPage()
    c.save()

    reader = PdfReader(OUT)
    if len(reader.pages) != 2:
        raise RuntimeError("form corpus must have exactly two pages")
    refs = []
    for page in reader.pages:
        xobjects = page["/Resources"]["/XObject"]
        forms = [ref for ref in xobjects.values() if ref.get_object().get("/Subtype") == "/Form"]
        if len(forms) != 1:
            raise RuntimeError("each page must reference one Form XObject")
        refs.append(forms[0].idnum)
    if refs[0] != refs[1]:
        raise RuntimeError(f"Form must be shared across pages, got object ids {refs}")
    print(f"{OUT}: pages=2 shared_form_object={refs[0]}")


if __name__ == "__main__":
    main()
