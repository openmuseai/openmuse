import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public final class CrossPageEditTransactionTest {
    static final String ORIGINAL = "Editable PDF text one";
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static NativeHandle load(PDFDocument document, long doc, int index) {
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc, index, page) == 0, "load page " + index);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize page " + index);
        return page;
    }

    static String firstText(NativeHandle page) {
        PDFPage api = new PDFPage(page.value());
        check(api.native_OnLButtonDown(page.value(), 80, 700, 0), "focus first text");
        return api.native_GetText(page.value());
    }

    static long append(PDFDocument document, long doc, int index, String suffix) {
        NativeHandle page = load(document, doc, index);
        PDFPage api = new PDFPage(page.value());
        String before = firstText(page);
        check(api.native_SetTextSelectRange(page.value(), before.length(), before.length())
                && api.native_OnChars(page.value(), suffix, false,
                                      before.length() + suffix.length()),
                "append on page " + index);
        long command = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(command != 0, "take command page " + index);
        float[] damage = new float[4];
        check(RuntimeProbe.nativeGetEditUndoDamage(command, damage),
                "command damage page " + index);
        check(damage[0] >= 0 && damage[1] >= 0 && damage[2] > damage[0]
                        && damage[3] > damage[1],
                "damage is normalized page geometry " + index);
        check((damage[2] - damage[0]) < 600 && (damage[3] - damage[1]) < 200,
                "damage is editor-local rather than whole page " + index);
        check(api.native_unloadPage2(page.value()) == 0, "recycle edited page " + index);
        return command;
    }

    static void expect(PDFDocument document, long doc, int index, String expected) {
        NativeHandle page = load(document, doc, index);
        check(expected.equals(firstText(page)), "page " + index + " text=" + expected);
        check(new PDFPage(page.value()).native_unloadPage2(page.value()) == 0,
                "recycle checked page " + index);
    }

    public static void main(String[] args) {
        if (args.length != 2)
            throw new IllegalArgumentException("two-page PDF and output required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open");
        document.native_setEditStatus(doc.value(), 1);

        long page0 = append(document, doc.value(), 0, "A");
        PDFFormFill.native_undoEdit(page0);
        check(RuntimeProbe.nativeLastErrorCode() == 0,
                "page-0 command survives page recycle");
        expect(document, doc.value(), 0, ORIGINAL);
        PDFFormFill.native_redoEdit(page0);
        expect(document, doc.value(), 0, ORIGINAL + "A");

        long page1 = append(document, doc.value(), 1, "B");
        PDFFormFill.native_undoEdit(page1);
        expect(document, doc.value(), 1, ORIGINAL);
        expect(document, doc.value(), 0, ORIGINAL + "A");

        PDFFormFill.native_undoEdit(page0);
        expect(document, doc.value(), 0, ORIGINAL);
        PDFFormFill.native_redoEdit(page0);
        PDFFormFill.native_redoEdit(page1);
        expect(document, doc.value(), 0, ORIGINAL + "A");
        expect(document, doc.value(), 1, ORIGINAL + "B");

        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        check(session != 0 && saver.start(session, args[1]), "start cross-page save");
        check(saver.run(session, 200) == 100, "finish cross-page save");
        saver.destroy(session);

        check(document.native_closePDF(doc.value()) == 0, "close document");
        PDFFormFill.native_undoEdit(page0);
        check(RuntimeProbe.nativeLastErrorCode() == -1005,
                "document close invalidates retained page-0 command");
        PDFFormFill.native_redoEdit(page1);
        check(RuntimeProbe.nativeLastErrorCode() == -1005,
                "document close invalidates retained page-1 command");
        NativeHandle reopened = new NativeHandle();
        check(PDFDocument.native_openPDF(args[1], reopened) == 0, "reopen saved document");
        expect(document, reopened.value(), 0, ORIGINAL + "A");
        expect(document, reopened.value(), 1, ORIGINAL + "B");
        check(document.native_closePDF(reopened.value()) == 0, "close saved document");
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "no retained page/command handles");
        System.out.println("CROSS_PAGE_EDIT_TRANSACTION PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
