#!/usr/bin/env python3
"""Write a small, deterministic two-page PDF with URI and GoTo links."""

import pathlib
import sys


def main() -> None:
    output = pathlib.Path(sys.argv[1])
    page1 = b"BT /F1 14 Tf 50 210 Td (Open web link) Tj 0 -40 Td (Go to page 2) Tj ET\n"
    page2 = b"BT /F1 14 Tf 50 210 Td (Second page) Tj ET\n"
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R 6 0 R] /Count 2 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 300 300] "
        b"/Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R "
        b"/Annots [8 0 R 9 0 R] >>",
        b"<< /Length " + str(len(page1)).encode() + b" >>\nstream\n" + page1 + b"endstream",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 300 300] "
        b"/Resources << /Font << /F1 5 0 R >> >> /Contents 7 0 R >>",
        b"<< /Length " + str(len(page2)).encode() + b" >>\nstream\n" + page2 + b"endstream",
        b"<< /Type /Annot /Subtype /Link /Rect [50 200 220 230] /Border [0 0 0] "
        b"/A << /S /URI /URI (https://example.com/pdf) >> >>",
        b"<< /Type /Annot /Subtype /Link /Rect [50 160 220 190] /Border [0 0 0] "
        b"/Dest [6 0 R /Fit] >>",
    ]
    data = bytearray(b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n")
    offsets = [0]
    for number, body in enumerate(objects, 1):
        offsets.append(len(data))
        data.extend(f"{number} 0 obj\n".encode())
        data.extend(body)
        data.extend(b"\nendobj\n")
    xref = len(data)
    data.extend(f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode())
    for offset in offsets[1:]:
        data.extend(f"{offset:010d} 00000 n \n".encode())
    data.extend(
        f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
    )
    output.write_bytes(data)


if __name__ == "__main__":
    main()
