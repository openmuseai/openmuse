// FreeText.cpp - implementation on PDFium public annotation API.
#include "FreeText.h"
#include <fpdf_annot.h>
#include <fpdf_doc.h>
#include <fpdf_edit.h>
#include <fpdfview.h>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <string>

namespace ksp {

// ---------------------------------------------------------------------------
// internal helpers
// ---------------------------------------------------------------------------
using WideString = std::basic_string<FPDF_WCHAR>;

static WideString toWide(const std::string& s) {  // ASCII -> UTF-16LE-ish
    WideString w;
    w.reserve(s.size());
    for (unsigned char c : s) w.push_back((FPDF_WCHAR)c);
    return w;
}

static FPDF_ANNOTATION firstFreeText(Page page_in) {
    FPDF_PAGE page = (FPDF_PAGE)page_in;
    if (!page) return nullptr;
    int n = FPDFPage_GetAnnotCount(page);
    for (int i = 0; i < n; ++i) {
        FPDF_ANNOTATION a = FPDFPage_GetAnnot(page, i);
        if (!a) continue;
        if (FPDFAnnot_GetSubtype(a) == FPDF_ANNOT_FREETEXT) return a;
        FPDFPage_CloseAnnot(a);
    }
    return nullptr;
}

static std::string buildDA(float size, int r, int g, int b) {
    char buf[128];
    std::snprintf(buf, sizeof(buf), "/Helv %g Tf %d %d %d rg", (double)size, r, g, b);
    return buf;
}

// parse "<digits> Tf" from a DA string
static bool parseDASize(const std::string& da, float* size) {
    std::string::size_type pos = 0;
    while ((pos = da.find("Tf", pos)) != std::string::npos) {
        std::string::size_type end = pos;
        // skip whitespace before "Tf"
        std::string::size_type numEnd = end;
        while (numEnd > 0 && (da[numEnd - 1] == ' ' || da[numEnd - 1] == '\t')) --numEnd;
        std::string::size_type start = numEnd;
        while (start > 0) {
            char c = da[start - 1];
            if (!(c == '.' || c == '-' || c == '+' || (c >= '0' && c <= '9'))) break;
            --start;
        }
        std::string tok = da.substr(start, numEnd - start);
        char* stop = nullptr;
        double v = std::strtod(tok.c_str(), &stop);
        if (stop != tok.c_str() && v > 0 && v < 100000) {
            *size = (float)v;
            return true;
        }
        pos = end + 2;
    }
    return false;
}

static void splitARGB(uint32_t argb, int& r, int& g, int& b) {
    r = (argb >> 16) & 0xFF;
    g = (argb >> 8) & 0xFF;
    b = argb & 0xFF;
}


// ---- appearance stream generation (draw the FreeText visibly in any viewer) ----
static std::string escPdfString(const std::string& s) {
    std::string o;
    for (char c : s) {
        if (c == '(' || c == ')' || c == '\\') o += '\\';
        o += c;
    }
    return o;
}

static bool ftBuildAndSetAP(FPDF_ANNOTATION a) {
    FS_RECTF rc{0, 0, 0, 0};
    if (!FPDFAnnot_GetRect(a, &rc)) return false;
    float w = rc.right - rc.left, h = rc.bottom - rc.top;
    if (w < 1.0f || h < 1.0f) return false;

    unsigned long n = FPDFAnnot_GetStringValue(a, "Contents", nullptr, 0);
    std::string text;
    if (n > 0) {
        WideString ww(n, 0);
        FPDFAnnot_GetStringValue(a, "Contents", &ww[0], n);
        for (FPDF_WCHAR u : ww) {
            unsigned short us = (unsigned short)u;
            if (us >= 32 && us < 127) text += (char)us;
        }
    }
    unsigned r = 0, g = 0, b = 0;
    float size = 12.0f;
    unsigned long dl = FPDFAnnot_GetStringValue(a, "DA", nullptr, 0);
    if (dl > 0) {
        WideString dw(dl, 0);
        FPDFAnnot_GetStringValue(a, "DA", &dw[0], dl);
        std::string da;
        for (FPDF_WCHAR u : dw) {
            unsigned short us = (unsigned short)u;
            if (us > 0 && us < 128) da += (char)us;
        }
        int r2 = 0, g2 = 0, b2 = 0;
        float sz2 = 0;
        if (std::sscanf(da.c_str(), "/Helv %f Tf %d %d %d rg", &sz2, &r2, &g2, &b2) == 4) {
            size = sz2; r = (unsigned)r2; g = (unsigned)g2; b = (unsigned)b2;
        } else {
            parseDASize(da, &size);
        }
    }
    char buf[4096];
    int len = std::snprintf(buf, sizeof(buf),
        "q 0.4 0.4 0.4 RG 0.8 w 0.5 0.5 %g %g re S Q "
        "BT /Helv %g Tf %d %d %d rg 2 %g Td (%s) Tj ET",
        (double)w - 1.0, (double)h - 1.0, (double)size, r, g, b,
        (double)(h * 0.75f), escPdfString(text).c_str());
    if (len <= 0) return false;
    std::string ap(buf, (size_t)len);
    return FPDFAnnot_SetAP(a, FPDF_ANNOT_APPEARANCEMODE_NORMAL, toWide(ap).c_str());
}

// refresh AP after any visual property change
static void ftUpdAP(FPDF_ANNOTATION a) { ftBuildAndSetAP(a); }

// ---------------------------------------------------------------------------
// feature ops
// ---------------------------------------------------------------------------
bool ftAddFreeText(Doc doc, Page page, float left, float top) {
    // NOTE: default box size is a documented approximation (WPS's exact default
    // rect depends on UI flow; calibrate on a real-device diff test).
    const float W = 60.0f, H = 20.0f;
    FPDF_ANNOTATION a = FPDFPage_CreateAnnot((FPDF_PAGE)page, FPDF_ANNOT_FREETEXT);
    if (!a) return false;
    FS_RECTF rc{left, top, left + W, top + H};
    bool ok = FPDFAnnot_SetRect(a, &rc);
    WideString da = toWide(buildDA(18.0f, 0, 0, 0));
    ok = FPDFAnnot_SetStringValue(a, "DA", da.c_str()) && ok;
    ok = FPDFAnnot_SetStringValue(a, "DS", da.c_str()) && ok;
    ftBuildAndSetAP(a);
    FPDFPage_CloseAnnot(a);
    return ok;
}

bool ftSetColor(Page page, uint32_t argb) {
    argb |= 0xff000000u;  // force alpha (original: param | 0xff000000)
    int r, g, b;
    splitARGB(argb, r, g, b);
    FPDF_ANNOTATION a = firstFreeText(page);
    if (!a) return false;
    bool ok = FPDFAnnot_SetColor(a, FPDFANNOT_COLORTYPE_Color,
                                 (unsigned)r, (unsigned)g, (unsigned)b, 255);
    // preserve the currently configured font size when rewriting /DA
    float size = 18.0f;
    {
        unsigned long len = FPDFAnnot_GetStringValue(a, "DA", nullptr, 0);
        if (len > 0) {
            WideString w(len, 0);
            FPDFAnnot_GetStringValue(a, "DA", &w[0], len);
            std::string da0;
            for (FPDF_WCHAR u : w) {
                unsigned short us = (unsigned short)u;
                if (us > 0 && us < 128) da0 += (char)us;
            }
            float s = 0;
            if (parseDASize(da0, &s)) size = s;
        }
    }
    std::string da = buildDA(size, r, g, b);
    WideString wda = toWide(da);
    ok = FPDFAnnot_SetStringValue(a, "DA", wda.c_str()) && ok;
    ok = FPDFAnnot_SetStringValue(a, "DS", wda.c_str()) && ok;
    ftBuildAndSetAP(a);
    FPDFPage_CloseAnnot(a);
    return ok;
}

uint32_t ftGetColor(Page page) {
    FPDF_ANNOTATION a = firstFreeText(page);
    if (!a) return 0;
    unsigned r = 0, g = 0, b = 0, al = 0;
    if (!FPDFAnnot_GetColor(a, FPDFANNOT_COLORTYPE_Color, &r, &g, &b, &al)) {
        // fallback: parse color from /DA "<r> <g> <b> rg" (we write both)
        unsigned long len = FPDFAnnot_GetStringValue(a, "DA", nullptr, 0);
        if (len > 0) {
            WideString w(len, 0);
            FPDFAnnot_GetStringValue(a, "DA", &w[0], len);
            std::string da;
            for (FPDF_WCHAR u : w) {
                unsigned short us = (unsigned short)u;
                if (us == ' ' || us == '.' || (us >= '0' && us <= '9')) da += (char)us;
            }
            float v[3] = {0, 0, 0};
            int n = std::sscanf(da.c_str(), "%f %f %f", &v[0], &v[1], &v[2]);
            if (n == 3) {
                r = (unsigned)(v[0] * 255);
                g = (unsigned)(v[1] * 255);
                b = (unsigned)(v[2] * 255);
            }
        }
    }
    FPDFPage_CloseAnnot(a);
    return 0xff000000u | ((r & 0xFF) << 16) | ((g & 0xFF) << 8) | (b & 0xFF);
}

bool ftSetFontSize(Page page, float size) {
    FPDF_ANNOTATION a = firstFreeText(page);
    if (!a) return false;
    unsigned r = 0, g = 0, b = 0, al = 0;
    if (!FPDFAnnot_GetColor(a, FPDFANNOT_COLORTYPE_Color, &r, &g, &b, &al)) {
        r = g = b = 0;
    }
    WideString da = toWide(buildDA(size, (int)r, (int)g, (int)b));
    bool ok = FPDFAnnot_SetStringValue(a, "DA", da.c_str());
    ok = FPDFAnnot_SetStringValue(a, "DS", da.c_str()) && ok;
    ftBuildAndSetAP(a);
    FPDFPage_CloseAnnot(a);
    return ok;
}

float ftGetFontSize(Page page) {
    FPDF_ANNOTATION a = firstFreeText(page);
    if (!a) return 0.0f;
    float size = 0.0f;
    unsigned long len = FPDFAnnot_GetStringValue(a, "DA", nullptr, 0);
    if (len > 0) {
        WideString wda(len, 0);
        FPDFAnnot_GetStringValue(a, "DA", &wda[0], len);
        std::string da;
        for (FPDF_WCHAR u : wda) {
            if (u > 0 && u < 128) da += (char)u;
        }
        std::fprintf(stderr, "[ftGetFontSize] DA len=%lu ascii='%s'\n", len, da.c_str());
        parseDASize(da, &size);
    }
    FPDFPage_CloseAnnot(a);
    return size;
}

bool ftSetRect(Page page, const Rect& r) {
    FPDF_ANNOTATION a = firstFreeText(page);
    if (!a) return false;
    FS_RECTF rc{r.left, r.top, r.right, r.bottom};
    bool ok = FPDFAnnot_SetRect(a, &rc);
    FPDFPage_CloseAnnot(a);
    return ok;
}

bool ftGetRect(Page page, Rect& r) {
    FPDF_ANNOTATION a = firstFreeText(page);
    if (!a) return false;
    FS_RECTF rc{0, 0, 0, 0};
    bool ok = FPDFAnnot_GetRect(a, &rc);
    r.left = rc.left; r.top = rc.top; r.right = rc.right; r.bottom = rc.bottom;
    FPDFPage_CloseAnnot(a);
    return ok;
}

bool ftSetText(Page page, const WideString& s) {
    FPDF_ANNOTATION a = firstFreeText(page);
    if (!a) return false;
    bool ok = FPDFAnnot_SetStringValue(a, "Contents", s.c_str());
    ftBuildAndSetAP(a);
    FPDFPage_CloseAnnot(a);
    return ok;
}

int ftAnnotCount(Page page) {
    if (!page) return 0;
    return FPDFPage_GetAnnotCount((FPDF_PAGE)page);
}

int ftAnnotType(Page page, int index) {
    FPDF_ANNOTATION a = FPDFPage_GetAnnot((FPDF_PAGE)page, index);
    if (!a) return -1;
    int t = (int)FPDFAnnot_GetSubtype(a);
    FPDFPage_CloseAnnot(a);
    return t;
}

bool ftAnnotColor(Page page, int index, float& r, float& g, float& b) {
    FPDF_ANNOTATION a = FPDFPage_GetAnnot((FPDF_PAGE)page, index);
    if (!a) return false;
    unsigned rr = 0, gg = 0, bb = 0, aa = 0;
    bool ok = FPDFAnnot_GetColor(a, FPDFANNOT_COLORTYPE_Color, &rr, &gg, &bb, &aa);
    if (ok) { r = (float)rr; g = (float)gg; b = (float)bb; }
    FPDFPage_CloseAnnot(a);
    return ok;
}

}  // namespace ksp