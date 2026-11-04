import android.graphics.PointF;
import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.edit.PDFPageEditor;
import cn.wps.moffice.pdf.core.edit.PDFTextEditor;
import cn.wps.moffice.pdf.core.select.PDFTextPage;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public final class RtlEditGeometryTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("RTL corpus required");
        System.loadLibrary("kspfreetext");
        int[] bidi = RuntimeProbe.nativeResolveBidiRuns("עולם-12", 1);
        check(bidi != null && bidi.length == 6,
              "PDFium ICU resolves mixed RTL paragraph into two runs");
        if (bidi != null && bidi.length == 6) {
            check(bidi[0] == 5 && bidi[1] == 2 && bidi[2] == 0 &&
                  bidi[3] == 0 && bidi[4] == 5 && bidi[5] == 1,
                  "UBA visual runs retain logical UTF-16 offsets and direction");
        }
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle pageHandle = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open RTL corpus");
        check(document.native_getPage(doc.value(), 0, pageHandle) == 0, "load page");
        PDFPage page = new PDFPage(pageHandle.value());

        PDFTextPage textApi = new PDFTextPage();
        long textPage = textApi.native_new(pageHandle.value());
        String pageText = textApi.native_getPageText(textPage);
        int rtlStart = pageText == null ? -1 : pageText.indexOf("םולש");
        check(rtlStart >= 0, "PDFTextPage preserves original visual RTL contract");
        if (rtlStart >= 0) {
            check(textApi.native_countRects(textPage, rtlStart, rtlStart + 3) > 0,
                  "RTL text has selection rectangle");
            RectF rect = new RectF();
            check(textApi.native_getRect(textPage, 0, rect) == 1,
                  "PDFTextPage rectangle reports RTL");
        }

        document.native_setEditStatus(doc.value(), 1);
        check(page.native_SectionRecognize(pageHandle.value()), "recognize RTL edit domain");
        check(page.native_OnLButtonDown(pageHandle.value(), 500, 680, 0),
              "focus RTL text object");
        check("שלום".equals(page.native_GetText(pageHandle.value())),
              "focused text remains in logical order");
        check(page.native_SetCaret(pageHandle.value(), 0), "set RTL leading caret");
        PointF[] leading = page.native_GetCaretPoints(pageHandle.value());
        check(page.native_SetCaret(pageHandle.value(), 1), "set next logical caret");
        PointF[] next = page.native_GetCaretPoints(pageHandle.value());
        check(leading != null && next != null && next[0].x < leading[0].x,
              "logical caret advance moves visually left");
        check(page.native_GetRangeIndexAtPos(pageHandle.value(), 499, 680) == 0,
              "right edge maps to logical start");
        check(page.native_GetRangeIndexAtPos(pageHandle.value(), 485, 680) == 1,
              "leftward point maps to next logical boundary");

        check(page.native_SetTextSelectRange(pageHandle.value(), 0, 4),
              "select logical RTL range");
        check(RuntimeProbe.nativeTextSelectionDirection(pageHandle.value()) == 1,
              "runtime selection direction is RTL");
        PDFPageEditor pageEditor = new PDFPageEditor();
        Object[] extended = pageEditor.native_selectionRectsEx(pageHandle.value());
        check(extended != null && extended.length == 2 &&
              extended[0] instanceof RectF[] && extended[1] instanceof int[],
              "selectionRectsEx preserves original object-array ABI");
        if (extended != null && extended.length == 2 && extended[0] instanceof RectF[] &&
            extended[1] instanceof int[]) {
            RectF[] rects = (RectF[]) extended[0];
            int[] directions = (int[]) extended[1];
            check(rects.length == 1 && directions.length == 1 && directions[0] == 1,
                  "edit selection rectangle carries RTL direction");
            check(rects.length == 1 && rects[0].left < 450 && rects[0].right > 499,
                  "RTL edit selection covers decreasing visual advances");
        }
        PDFTextEditor textEditor = new PDFTextEditor();
        check(textEditor.native_getSelDirection(pageHandle.value()) == 1,
              "text-editor direction ABI returns RTL for page bridge");

        check(page.native_OnLButtonDown(pageHandle.value(), 500, 630, 0),
              "focus mixed Hebrew-number object");
        check("עולם-12".equals(page.native_GetText(pageHandle.value())),
              "mixed object exposes logical editor text");
        String domainText = page.native_GetEditAllText(pageHandle.value());
        int mixedStart = domainText == null ? -1 : domainText.indexOf("עולם-12");
        check(mixedStart >= 0, "mixed object has stable domain logical offset");
        check(mixedStart >= 0 && page.native_SetTextSelectRange(
                  pageHandle.value(), mixedStart, mixedStart + 7),
              "select mixed bidi object");
        Object[] mixed = pageEditor.native_selectionRectsEx(pageHandle.value());
        if (mixed != null && mixed.length == 2 && mixed[1] instanceof int[]) {
            int[] directions = (int[]) mixed[1];
            boolean ltr = false, rtl = false;
            for (int direction : directions) {
                ltr |= direction == 0;
                rtl |= direction == 1;
            }
            check(ltr && rtl, "selectionRectsEx splits mixed visual runs by direction");
        } else {
            check(false, "selectionRectsEx returns mixed visual runs");
        }
        check(mixedStart >= 0 && page.native_SetTextSelectRange(
                  pageHandle.value(), mixedStart + 5, mixedStart + 7) &&
              RuntimeProbe.nativeTextSelectionDirection(pageHandle.value()) == 0,
              "numeric logical run reports LTR selection direction");

        check(page.native_unloadPage2(pageHandle.value()) == 0, "close page");
        check(document.native_closePDF(doc.value()) == 0, "close document");
        System.out.println("RTL_EDIT_GEOMETRY PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
