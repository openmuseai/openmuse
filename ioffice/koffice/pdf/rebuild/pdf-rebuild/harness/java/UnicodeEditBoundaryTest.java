import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public final class UnicodeEditBoundaryTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("input required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle page = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open");
        check(document.native_getPage(doc.value(), 0, page) == 0, "page");
        document.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize");
        check(api.native_OnLButtonDown(page.value(), 100, 700, 0), "focus");
        String original = api.native_GetEditAllText(page.value());
        int caret = original.indexOf("Regular") + "Regular".length();
        check(caret >= 0 && api.native_SetTextSelectRange(page.value(), caret, caret),
                "set emoji insertion caret");
        check(!api.native_OnChars(page.value(), "\ud83d\ude00", false, caret + 2),
                "reject emoji string like original editor");
        check(!api.native_OnChar(page.value(), 0x1f600, 0),
                "reject supplementary OnChar");
        check(original.equals(api.native_GetEditAllText(page.value())),
                "emoji rejection preserves text");

        int secondEnd = original.indexOf("Second") + "Second".length();
        check(secondEnd > caret && api.native_SetTextSelectRange(
                page.value(), secondEnd, secondEnd), "set domain-global caret");
        check(api.native_OnKeyDown(page.value(), 67, 0), "domain-global backspace");
        check(api.native_GetEditAllText(page.value()).contains("Secon delta"),
                "backspace edits later run, not focused atom offset");
        long undo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(undo != 0, "backspace undo command");
        if (undo != 0) PDFFormFill.native_undoEdit(undo);
        check(original.equals(api.native_GetEditAllText(page.value())), "backspace undo");
        PDFFormFill.native_deleteTransEdit(undo);
        api.native_unloadPage2(page.value());
        document.native_closePDF(doc.value());
        System.out.println("UNICODE_EDIT_BOUNDARY PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
