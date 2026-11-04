// FreeText.h - KSP FreeText annotation layer rebuilt on PDFium public API.
//
// Reconstructed behavior (evidence from libkwopdf.so decompilation):
//   * setFreeTextColor stores color | 0xff000000   (alpha forced, from
//     PDFAnnotationEditor_native_setTextColor -> f_328584: param|0xff000000)
//   * rect ops work in page space (Java maps device<->page around the natives)
//   * addFreeText creates a FreeText annotation on the page
// PDF textbook mapping: FreeText annot = /Subtype /FreeText, /Rect, /C color,
// /DA default appearance "/Helv <size> Tf <r> <g> <b> rg", /Contents text.
#pragma once
#include <cstdint>
#include <string>
#include <fpdfview.h>  // FPDF_WCHAR

namespace ksp {

using Doc = void*;  // FPDF_DOCUMENT
using Page = void*; // FPDF_PAGE
struct fpdf_annotation_t__;
using Annot = fpdf_annotation_t__*;

struct Rect { float left, top, right, bottom; };

// --- feature ops (page-based resolution: first FreeText annot on the page) ---
bool    ftAddFreeText(Doc doc, Page page, float left, float top);
bool    ftSetColor(Page page, uint32_t argb);       // alpha forced to 0xFF
uint32_t ftGetColor(Page page);
bool    ftSetFontSize(Page page, float size);        // writes /DA
float   ftGetFontSize(Page page);                    // parses /DA "Tf"
bool    ftSetRect(Page page, const Rect& r);         // page space
bool    ftGetRect(Page page, Rect& r);
bool    ftSetText(Page page, const std::basic_string<FPDF_WCHAR>& s); // /Contents

// test/verification helpers
int     ftAnnotCount(Page page);
int     ftAnnotType(Page page, int index);
bool    ftAnnotColor(Page page, int index, float& r, float& g, float& b);

}  // namespace ksp