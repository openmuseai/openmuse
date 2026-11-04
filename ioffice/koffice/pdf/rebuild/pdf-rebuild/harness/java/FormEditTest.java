import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public class FormEditTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static PDFPage load(PDFDocument document, long doc, int index, NativeHandle result) {
        check(document.native_getPage(doc, index, result) == 0, "load page " + index);
        PDFPage page = new PDFPage(result.value());
        page.native_SectionRecognize(result.value());
        return page;
    }

    static String focus(PDFPage page, long handle, float x, float y) {
        check(page.native_OnLButtonDown(handle, x, y, 0), "focus at " + x + "," + y);
        String value = page.native_GetText(handle);
        return value == null ? "" : value.trim();
    }

    public static void main(String[] args) {
        if (args.length != 2) throw new IllegalArgumentException("input and output PDFs required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open corpus");
        document.native_setEditStatus(doc.value(), 1);
        check(document.native_separateSharedDataInEditPage(doc.value(), 0), "separate shared page");

        NativeHandle page0Handle = new NativeHandle();
        PDFPage page0 = load(document, doc.value(), 0, page0Handle);
        String original = focus(page0, page0Handle.value(), 90, 674);
        check("SHARED FORM TEXT".equals(original), "first Form text");
        check(page0.native_SetTextSelectRange(page0Handle.value(), 0, original.length()) &&
                page0.native_OnChars(page0Handle.value(), "EDITED FORM", false, 11),
                "replace nested text");
        long replace = RuntimeProbe.nativeTakePendingEditUndo(page0Handle.value());
        check(replace != 0, "replace command");
        check("EDITED FORM".equals(focus(page0, page0Handle.value(), 90, 674)),
                "first instance changed");
        check("SHARED FORM TEXT".equals(focus(page0, page0Handle.value(), 318, 674)),
                "same-page shared instance isolated");

        NativeHandle page1Handle = new NativeHandle();
        PDFPage page1 = load(document, doc.value(), 1, page1Handle);
        check("SHARED FORM TEXT".equals(focus(page1, page1Handle.value(), 90, 674)),
                "cross-page shared instance isolated");
        page1.native_unloadPage2(page1Handle.value());

        PDFFormFill.native_undoEdit(replace);
        check("SHARED FORM TEXT".equals(focus(page0, page0Handle.value(), 90, 674)),
                "undo nested replace");
        PDFFormFill.native_redoEdit(replace);
        check("EDITED FORM".equals(focus(page0, page0Handle.value(), 90, 674)),
                "redo nested replace");

        int editedLength = page0.native_GetTextLength(page0Handle.value());
        check(page0.native_SetTextSelectRange(page0Handle.value(), 0, editedLength) &&
                page0.native_OnKeyDown(page0Handle.value(), 112, 0), "delete nested text");
        long deletion = RuntimeProbe.nativeTakePendingEditUndo(page0Handle.value());
        check(deletion != 0 && page0.native_GetFocusType(page0Handle.value()) == 0,
                "nested delete command");
        PDFFormFill.native_undoEdit(deletion);
        check("EDITED FORM".equals(focus(page0, page0Handle.value(), 90, 674)),
                "undo nested delete");
        PDFFormFill.native_redoEdit(deletion);
        check(page0.native_GetFocusType(page0Handle.value()) == 0, "redo nested delete");
        PDFFormFill.native_undoEdit(deletion);

        check(document.native_syncEditPageObj(doc.value(), page0Handle.value(), true),
                "sync Form page before save");
        page0.native_unloadPage2(page0Handle.value());
        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        check(session != 0 && saver.start(session, args[1]) && saver.run(session, 500) == 100,
                "save edited Form");
        saver.destroy(session);
        PDFFormFill.native_deleteTransEdit(deletion);
        PDFFormFill.native_deleteTransEdit(replace);
        document.native_closePDF(doc.value());

        NativeHandle reopened = new NativeHandle();
        check(PDFDocument.native_openPDF(args[1], reopened) == 0, "reopen saved Form");
        document.native_setEditStatus(reopened.value(), 1);
        NativeHandle verify0Handle = new NativeHandle();
        PDFPage verify0 = load(document, reopened.value(), 0, verify0Handle);
        String savedFirst = focus(verify0, verify0Handle.value(), 90, 674);
        if (!"EDITED FORM".equals(savedFirst)) System.out.println("saved first actual=" + savedFirst);
        check("EDITED FORM".equals(savedFirst), "saved first instance");
        check("SHARED FORM TEXT".equals(focus(verify0, verify0Handle.value(), 318, 674)),
                "saved same-page isolation");
        verify0.native_unloadPage2(verify0Handle.value());
        NativeHandle verify1Handle = new NativeHandle();
        PDFPage verify1 = load(document, reopened.value(), 1, verify1Handle);
        check("SHARED FORM TEXT".equals(focus(verify1, verify1Handle.value(), 90, 674)),
                "saved cross-page isolation");
        verify1.native_unloadPage2(verify1Handle.value());
        document.native_closePDF(reopened.value());

        System.out.println("FORM_EDIT PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
