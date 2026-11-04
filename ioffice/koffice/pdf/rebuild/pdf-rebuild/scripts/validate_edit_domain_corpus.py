#!/usr/bin/env python3
"""Validate CE1.2c fixtures before sending them to either native library."""

import json
from pathlib import Path

from pypdf import PdfReader


ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "harness" / "assets" / "edit-domain-corpus"


def box(value):
    return [float(item) for item in value]


def main() -> None:
    result = {}
    for path in sorted(CORPUS.glob("*.pdf")):
        reader = PdfReader(path)
        if len(reader.pages) != 1:
            raise AssertionError(f"{path.name}: page count")
        page = reader.pages[0]
        text = page.extract_text() or ""
        result[path.name] = {
            "media_box": box(page.mediabox),
            "crop_box": box(page.cropbox),
            "text_lines": text.splitlines(),
            "form_xobjects": 0,
        }
        resources = page.get("/Resources", {})
        for reference in resources.get("/XObject", {}).values():
            if reference.get_object().get("/Subtype") == "/Form":
                result[path.name]["form_xobjects"] += 1
    if result["cropbox.pdf"]["crop_box"] != [36.0, 72.0, 576.0, 720.0]:
        raise AssertionError("cropbox.pdf: CropBox")
    if result["cropbox.pdf"]["media_box"] != [0.0, 0.0, 612.0, 792.0]:
        raise AssertionError("cropbox.pdf: MediaBox")
    if result["form-xobject.pdf"]["form_xobjects"] != 1:
        raise AssertionError("form-xobject.pdf: Form XObject")
    if len(result["multicolumn.pdf"]["text_lines"]) != 7:
        raise AssertionError("multicolumn.pdf: text lines")
    if len(result["rotated.pdf"]["text_lines"]) != 5:
        raise AssertionError("rotated.pdf: text lines")
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
