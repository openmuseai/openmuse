import android.graphics.PointF;
import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public class GlyphCaretTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("glyph corpus PDF required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open corpus");
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0, "load page");
        PDFPage api = new PDFPage(page.value());
        document.native_setEditStatus(doc.value(), 1);
        check(api.native_SectionRecognize(page.value()), "recognize domains");

        check(api.native_OnLButtonDown(page.value(), 80, 704, 0), "focus proportional text");
        check("Wim il1 MW".equals(api.native_GetText(page.value())), "read proportional text");
        check(api.native_GetCaret(page.value()) == 0, "click establishes nearest caret");
        check(api.native_GetRangeIndexAtPos(page.value(), 84, 700) == 1,
              "wide W uses glyph advance");
        check(api.native_GetRangeIndexAtPos(page.value(), 94, 700) == 2,
              "narrow i uses glyph advance");
        check(api.native_SetCaret(page.value(), 1), "set caret");
        PointF[] first = api.native_GetCaretPoints(page.value());
        check(first != null && first.length == 2 && Math.abs(first[0].x - 88.992f) < 0.02f,
              "caret uses PDFium character origin");
        RectF glyph = new RectF();
        api.native_GetEditSelectionRect(page.value(), 1, glyph);
        check(Math.abs(glyph.width()) > 16 && Math.abs(glyph.height()) > 12,
              "character geometry is non-empty");
        float[] quad = api.native_GetEditSelectionQuadRect(page.value(), 1);
        check(quad != null && quad.length == 8 && quad[2] > quad[0], "character quad returned");
        check(!api.native_SetCaret(page.value(), 11) &&
              RuntimeProbe.nativeLastErrorCode() == -1006, "out-of-range caret rejected");
        check(api.native_SetTextSelectRange(page.value(), 1, 3), "set non-collapsed range");
        check(api.native_GetCaretPoints(page.value()) == null, "selection has no caret points");

        check(api.native_OnLButtonDown(page.value(), 309, 404, 0), "focus rotated text");
        check("Rotate caret".equals(api.native_GetText(page.value())), "read rotated text");
        check(api.native_GetRangeIndexAtPos(page.value(), 335, 416.31f) == 4,
              "rotated baseline hit maps to glyph boundary");
        check(api.native_SetCaret(page.value(), 2), "set rotated caret");
        PointF[] rotated = api.native_GetCaretPoints(page.value());
        check(rotated != null && Math.abs(rotated[0].x - rotated[1].x) > 1 &&
              Math.abs(rotated[0].y - rotated[1].y) > 1,
              "rotated caret follows character matrix");

        check(api.native_unloadPage2(page.value()) == 0, "close page");
        check(document.native_closePDF(doc.value()) == 0, "close document");
        System.out.println("GLYPH_CARET PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
