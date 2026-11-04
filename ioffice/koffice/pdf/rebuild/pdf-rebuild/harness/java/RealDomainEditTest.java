import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public final class RealDomainEditTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    public static void main(String[] args) {
        if (args.length != 2) throw new IllegalArgumentException("input and output PDF required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle page = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open paper");
        check(document.native_getPage(doc.value(), 1, page) == 0, "load page 2");
        document.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize");
        check(api.native_OnLButtonDown(page.value(), 90, 690, 0), "focus chapter 1");
        String before = api.native_GetEditAllText(page.value());
        String heading = "1. Introduction ";
        String replacement = "1. Overview ";
        check(before != null && before.startsWith(heading), "precondition heading");
        check(api.native_SetTextSelectRange(page.value(), 0, heading.length()) &&
              api.native_OnChars(page.value(), replacement, false, replacement.length()),
                "replace selection spanning paint objects");
        String after = api.native_GetEditAllText(page.value());
        check(after != null && after.startsWith(replacement) &&
              after.substring(replacement.length()).equals(before.substring(heading.length())),
                "cross-run delta preserves unselected domain text");
        long command = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(command != 0, "one atomic undo handle");
        check(RuntimeProbe.nativeTakePendingEditUndo(page.value()) == 0,
                "no per-object undo leakage");
        PDFFormFill.native_undoEdit(command);
        check(before.equals(api.native_GetEditAllText(page.value())), "atomic undo");
        PDFFormFill.native_redoEdit(command);
        check(after.equals(api.native_GetEditAllText(page.value())), "atomic redo");
        check(document.native_syncEditPageObj(doc.value(), page.value(), true), "sync page");
        PDFFormFill.native_deleteTransEdit(command);
        check(api.native_unloadPage2(page.value()) == 0, "close page before save");
        PDFNativeSaver saver = new PDFNativeSaver();
        long save = saver.newSession(doc.value(), false);
        check(save != 0 && saver.start(save, args[1]) && saver.run(save, 500) == 100,
                "save cross-run edit");
        saver.destroy(save);
        check(document.native_closePDF(doc.value()) == 0, "close document");

        NativeHandle reopened = new NativeHandle();
        NativeHandle reopenedPage = new NativeHandle();
        check(PDFDocument.native_openPDF(args[1], reopened) == 0, "reopen saved output");
        check(document.native_getPage(reopened.value(), 1, reopenedPage) == 0,
                "reload saved page");
        document.native_setEditStatus(reopened.value(), 1);
        PDFPage reopenedApi = new PDFPage(reopenedPage.value());
        check(reopenedApi.native_SectionRecognize(reopenedPage.value()) &&
              reopenedApi.native_OnLButtonDown(reopenedPage.value(), 90, 690, 0),
                "refocus saved editor");
        String saved = reopenedApi.native_GetEditAllText(reopenedPage.value());
        check(saved != null && saved.startsWith(replacement), "saved text survives reopen");
        reopenedApi.native_unloadPage2(reopenedPage.value());
        document.native_closePDF(reopened.value());
        System.out.println("REAL_DOMAIN_EDIT PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
