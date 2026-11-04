"""Exercise the unmodified GenOffice desktop PPTX pipeline against a deck.

This is a reference test, not the OpenMuse plugin implementation. It requires
an installed GenOffice CLI because upstream rendering uses its Electron export
pipeline. All edited files and PNGs are written to an explicit output folder.
"""

from __future__ import annotations

import argparse
import json
import struct
import subprocess
from pathlib import Path


DEFAULT_CLI = Path("/Applications/GenOffice.app/Contents/Resources/cli/genoffice")


def command(cli: Path, *args: str) -> dict:
    result = subprocess.run(
        [str(cli), *args, "--json"],
        check=True,
        capture_output=True,
        text=True,
    )
    payload = json.loads(result.stdout)
    if payload.get("status") != "ok":
        raise RuntimeError(f"GenOffice failed: {payload}")
    return payload


def png_size(path: Path) -> tuple[int, int]:
    with path.open("rb") as image:
        header = image.read(24)
    if header[:8] != b"\x89PNG\r\n\x1a\n" or header[12:16] != b"IHDR":
        raise ValueError(f"Invalid PNG: {path}")
    return struct.unpack(">II", header[16:24])


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("pptx", type=Path, help="Source PPTX (never modified)")
    parser.add_argument("--out", required=True, type=Path, help="Evidence directory")
    parser.add_argument("--cli", type=Path, default=DEFAULT_CLI)
    args = parser.parse_args()
    source = args.pptx.resolve(strict=True)
    cli = args.cli.resolve(strict=True)
    output = args.out.resolve()
    output.mkdir(parents=True, exist_ok=True)

    original = command(cli, "slides", "read", str(source), "--slide", "0")
    slide_count = original["detail"]["slides"]
    first_page = original["detail"]["pages"][0]
    target = next(
        element for element in first_page["elements"] if element["type"] == "text"
    )

    render_dir = output / "original-render"
    render_dir.mkdir(exist_ok=True)
    for previous in render_dir.glob("*.png"):
        previous.unlink()
    command(cli, "slides", "render", str(source), "--out", str(render_dir))
    images = sorted(render_dir.glob("*.png"))
    if len(images) != slide_count:
        raise AssertionError(f"Expected {slide_count} PNGs, got {len(images)}")
    dimensions = png_size(images[0])
    if min(dimensions) <= 0:
        raise AssertionError(f"Empty first slide image: {dimensions}")

    edited = output / "edited.pptx"
    ops = output / "set-text.json"
    replacement = "OpenMuse upstream pipeline verification"
    ops.write_text(
        json.dumps(
            [{
                "op": "setText",
                "target": {"slide": 0, "el": target["id"]},
                "paragraphs": [{"runs": [{"text": replacement}]}],
            }],
            ensure_ascii=False,
        ),
        encoding="utf-8",
    )
    if edited.exists():
        edited.unlink()
    command(cli, "slides", "apply", str(source), "--ops", str(ops), "--out", str(edited))
    reopened = command(cli, "slides", "read", str(edited), "--slide", "0")
    changed = next(
        element
        for element in reopened["detail"]["pages"][0]["elements"]
        if element["id"] == target["id"]
    )
    if changed["text"] != replacement or reopened["detail"]["slides"] != slide_count:
        raise AssertionError("Edited PPTX did not reopen with the expected text/deck")

    evidence = {
        "source": str(source),
        "genoffice_cli": str(cli),
        "slide_count": slide_count,
        "rendered_png_count": len(images),
        "first_png": str(images[0]),
        "first_png_size": dimensions,
        "edited_pptx": str(edited),
        "edited_element": target["id"],
        "reopened_text": changed["text"],
    }
    (output / "result.json").write_text(
        json.dumps(evidence, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )
    print(json.dumps(evidence, ensure_ascii=False))


if __name__ == "__main__":
    main()
