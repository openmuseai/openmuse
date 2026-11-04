import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;

public class EditPointerInteractionTest {
    static int pass;
    static int fail;
    static void check(boolean value, String name) {
        if (value) pass++; else { fail++; System.out.println("FAIL: " + name); }
    }

    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("glyph corpus PDF required");
        System.loadLibrary("kspfreetext");
        PDFDocument docApi = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle page = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open");
        check(docApi.native_getPage(doc.value(), 0, page) == 0, "load page");
        docApi.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize");

        check(api.native_GetHoverType(page.value(), 80, 704) == 1, "text hover type");
        check(api.native_GetHoverType(page.value(), 20, 20) == 0, "blank hover type");
        check(api.native_OnLButtonDown(page.value(), 80, 704, 0), "pointer down");
        check(api.native_OnMouseMove(page.value(), 145, 704, 0), "pointer drag");
        int[] range = api.native_GetTextSelectRange(page.value());
        check(range != null && range[0] == 0 && range[1] > 0, "drag grows range from anchor");
        check(api.native_OnLButtonUp(page.value(), 145, 704), "pointer up");
        range = api.native_GetTextSelectRange(page.value());
        check(range != null && range[0] == 0 && range[1] > 0, "up retains range");

        check(api.native_OnLButtonDoubleClick(page.value(), 100, 704), "double click");
        range = api.native_GetTextSelectRange(page.value());
        check(range != null && range[0] == 0 && range[1] == 3,
              "double click selects word: " + (range == null ? "null" : range[0] + "," + range[1]));
        check(api.native_OnLButtonTripleClick(page.value(), 100, 704), "triple click");
        range = api.native_GetTextSelectRange(page.value());
        check(range != null && range[0] == 0 && range[1] == "Wim il1 MW".length(),
              "triple click selects text object");

        check(api.native_unloadPage2(page.value()) == 0, "close page");
        check(docApi.native_closePDF(doc.value()) == 0, "close doc");
        System.out.println("EDIT_POINTER PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
