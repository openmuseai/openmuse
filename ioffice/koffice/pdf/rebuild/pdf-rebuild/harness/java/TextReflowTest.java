import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;

public final class TextReflowTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static String normalize(String value) {
        return value == null ? "" : value.replace('\n', ' ').replaceAll(" +", " ");
    }

    public static void main(String[] args) {
        if (args.length != 2) throw new IllegalArgumentException("input and output PDF required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle page = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open");
        check(document.native_getPage(doc.value(), 0, page) == 0, "page");
        document.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize");
        check(api.native_OnLButtonDown(page.value(), 80, 700, 0), "focus uniform paragraph");
        String before = api.native_GetEditAllText(page.value());
        String first = api.native_GetText(page.value());
        String overflow = " alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu";
        check(before != null && first != null &&
              api.native_SetTextSelectRange(page.value(), first.length(), first.length()) &&
              api.native_OnChars(page.value(), overflow, false, first.length() + overflow.length()),
                "overflow edit");
        String after = api.native_GetEditAllText(page.value());
        check(after != null && after.split("\\n", -1).length > before.split("\\n", -1).length,
                "new layout lines created");
        check(normalize(after).contains("alpha beta gamma delta epsilon") &&
              normalize(after).contains("lambda mu"), "reflow preserves logical words");
        check(document.native_syncEditPageObj(doc.value(), page.value(), true), "sync");
        check(api.native_unloadPage2(page.value()) == 0, "unload before save");
        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        check(session != 0 && saver.start(session, args[1]) && saver.run(session, 500) == 100,
                "save");
        saver.destroy(session);
        check(document.native_closePDF(doc.value()) == 0, "close");

        NativeHandle reopened = new NativeHandle();
        NativeHandle reopenedPage = new NativeHandle();
        check(PDFDocument.native_openPDF(args[1], reopened) == 0, "reopen");
        check(document.native_getPage(reopened.value(), 0, reopenedPage) == 0, "reopen page");
        document.native_setEditStatus(reopened.value(), 1);
        PDFPage verify = new PDFPage(reopenedPage.value());
        check(verify.native_SectionRecognize(reopenedPage.value()) &&
              verify.native_OnLButtonDown(reopenedPage.value(), 80, 700, 0), "refocus reflow");
        String saved = verify.native_GetEditAllText(reopenedPage.value());
        check(saved != null && normalize(saved).contains("lambda mu") &&
              saved.split("\\n", -1).length > before.split("\\n", -1).length,
                "created text objects survive reopen");
        verify.native_unloadPage2(reopenedPage.value());
        document.native_closePDF(reopened.value());
        System.out.println("TEXT_REFLOW PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
