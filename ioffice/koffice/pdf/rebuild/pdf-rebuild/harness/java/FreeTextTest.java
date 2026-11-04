// FreeTextTest - drives the rebuilt FreeText feature via JNI and verifies
// behavior against the decompilation-derived spec (golden rules):
//   * color in:  0x00RRGGBB / 0xAARRGGBB  -> stored 0xFFRRGGBB (alpha forced)
//   * rect ops in page space
//   * PDF dict result: /Subtype /FreeText, /Rect, /C, /DA with size+color
import android.graphics.RectF;
import cn.wps.rebuild.FreeTextStub;
import cn.wps.rebuild.TestPdf;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class FreeTextTest {

    static int pass = 0, fail = 0;

    static void check(boolean cond, String what) {
        if (cond) { pass++; } else { fail++; System.out.println("FAIL: " + what); }
    }

    static void checkEq(long a, long b, String what) {
        check(a == b, what + " (got " + a + ", want " + b + ")");
    }

    static void checkNear(float a, float b, float eps, String what) {
        check(Math.abs(a - b) <= eps, what + " (got " + a + ", want " + b + ")");
    }

    public static void main(String[] args) throws Exception {
        System.loadLibrary("kspfreetext");
        long doc = TestPdf.nativeNewDoc();
        check(doc != 0, "newDoc");
        long page = TestPdf.nativeAddPage(doc, 612, 792); // US Letter points
        check(page != 0, "addPage");

        // ---- 1. addFreeText: create annotation ----
        boolean ok = FreeTextStub.native_addFreeText(doc, page, 100, 100);
        check(ok, "addFreeText returns true");
        check(TestPdf.nativeAnnotCount(page) == 1, "one annotation on page");
        check(TestPdf.nativeAnnotType(page, 0) == 3, "subtype == FreeText (3)");

        // ---- 2. color: set 0x00FF0000 (red, opaque) -> get 0xFFFF0000 ----
        FreeTextStub.native_setFreeTextColor(page, 0x00FF0000);
        int c = FreeTextStub.native_getFreeTextColor(page);
        checkEq(c & 0xFFFFFFFFL, 0xFFFF0000L, "color red round-trip 0xFFRRGGBB");

        // color with alpha already set stays: 0x80FF0000 -> 0xFFFF0000
        FreeTextStub.native_setFreeTextColor(page, 0x8000FF00); // green with 50% alpha
        c = FreeTextStub.native_getFreeTextColor(page);
        checkEq(c & 0xFFFFFFFFL, 0xFF00FF00L, "color green alpha forced to 0xFF");

        // ---- 3. font size: set 24 -> get 24 ----
        FreeTextStub.native_setSelectFontSize(page, 24.0f);
        float fs = FreeTextStub.native_getSelectFontSize(page);
        checkNear(fs, 24.0f, 0.01f, "font size round-trip 24");

        // ---- 4. rect: page-space round-trip ----
        RectF r = new RectF(10, 20, 110, 40);
        FreeTextStub.native_setFreeTextRect(page, r);
        RectF r2 = new RectF();
        FreeTextStub.native_getFreeTextRect(page, 0, r2);
        checkNear(r2.left, 10, 0.01f, "rect.left");
        checkNear(r2.top, 20, 0.01f, "rect.top");
        checkNear(r2.right, 110, 0.01f, "rect.right");
        checkNear(r2.bottom, 40, 0.01f, "rect.bottom");

        // final state: red for the dict byte check, plus text content
        FreeTextStub.native_setFreeTextColor(page, 0x00FF0000);
        check(TestPdf.nativeSetContents(page, "Hello"), "setContents");

        // ---- 5. save + raw dict verification ----
        String out = "/tmp/freetext_spike.pdf";
        check(TestPdf.nativeSaveDoc(doc, out), "save pdf");
        TestPdf.nativeClosePage(page);
        TestPdf.nativeCloseDoc(doc);

        // raw byte-level dict checks (external verifiability)
        byte[] pdf = Files.readAllBytes(Paths.get(out));
        String s = new String(pdf, "ISO-8859-1");
        check(s.contains("/Subtype/FreeText") || s.contains("/Subtype /FreeText"),
              "dict has /Subtype FreeText");
        check(s.matches("(?s).*/C\\s*\\[[^]]*1\\s+0\\s+0[^]]*\\].*"),
              "dict /C contains red (1 0 0)");
        check(s.matches("(?s).*/DA\\s*\\(\\s*/Helv\\s+24\\s+Tf[^)]*\\).*"),
              "dict /DA has /Helv 24 Tf");
        check(s.matches("(?s).*/Rect\\s*\\[\\s*10\\s+40\\s+110\\s+20\\s*\\].*"),
              "dict /Rect [llx lly urx ury] = [10 40 110 20]");
        check(s.contains("/Contents(Hello)"), "dict /Contents(Hello)");
        System.out.println("PDF size: " + pdf.length + " bytes");

        // ---- 6. reopen and verify via PDFium again ----
        // (optional; raw checks above already cover the dict)
        System.out.println("\n======== RESULT: PASS=" + pass + " FAIL=" + fail + " ========");
        if (fail > 0) System.exit(1);
    }
}
