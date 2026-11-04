import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public class EditLifecycleTest {
    static final int TEXT = 1;
    static final int IMAGE = 6;
    static final int MIXED = 7;
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("fixture path required");
        System.loadLibrary("kspfreetext");
        PDFDocument api = new PDFDocument();
        NativeHandle doc1 = new NativeHandle();
        NativeHandle doc2 = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc1) == 0, "open first document");
        check(PDFDocument.native_openPDF(args[0], doc2) == 0, "open second document");
        NativeHandle page = new NativeHandle();
        check(api.native_getPage(doc1.value(), 0, page) == 0, "load edit page");
        PDFPage pageApi = new PDFPage(page.value());

        check(api.nGetEditStatus(doc1.value()) == 0, "default edit status NONE");
        check(api.nGetInsertStatus(doc1.value()) == 0, "default insert status NONE");
        api.native_setEditStatus(doc1.value(), MIXED);
        api.native_setInsertStatus(doc1.value(), 4);
        check(api.nGetEditStatus(doc1.value()) == MIXED, "round-trip MIXED status");
        check(api.nGetInsertStatus(doc1.value()) == 4, "round-trip TEXT insert status");

        check(RuntimeProbe.nativeStagePageMutation(page.value(), TEXT, true, true) == 0,
              "stage text mutation");
        check(pageApi.native_IsModified(page.value()), "page reports modified");
        check(pageApi.native_CheckFuncModified(page.value(), TEXT), "text bit reports modified");
        check(!pageApi.native_CheckFuncModified(page.value(), IMAGE), "image bit remains clean");
        long[] staged = RuntimeProbe.nativeEditDiagnostics(page.value());
        check(staged != null && staged.length == 7 && staged[1] > 0 && staged[5] == 1 && staged[6] == 1,
              "mutation epoch focus and hover recorded");

        check(api.native_EditBackup(doc1.value(), page.value(), MIXED), "backup edit page");
        check(RuntimeProbe.nativeEditDiagnostics(page.value())[4] == 1, "backup marker recorded");
        check(api.native_ClearEditFocus(doc1.value(), page.value()), "clear focus");
        check(RuntimeProbe.nativeEditDiagnostics(page.value())[5] == 0, "focus cleared only");
        check(api.native_ClearHoverHandle(doc1.value(), page.value()), "clear hover");
        check(RuntimeProbe.nativeEditDiagnostics(page.value())[6] == 0, "hover cleared only");

        check(!api.native_syncEditPageObj(doc2.value(), page.value(), false),
              "cross-document sync rejected");
        check(RuntimeProbe.nativeLastErrorCode() == -1004, "owner mismatch error retained");
        check(api.native_syncEditPageObj(doc1.value(), page.value(), false), "sync staged edit page");
        long[] synced = RuntimeProbe.nativeEditDiagnostics(page.value());
        check(!pageApi.native_IsModified(page.value()) && synced[0] == 0,
              "sync clears dirty functions");
        check(synced[2] == staged[1] && synced[3] == 1,
              "sync commits epoch and invalidates text cache");

        check(RuntimeProbe.nativeStagePageMutation(page.value(), IMAGE, true, false) == 0,
              "stage second mutation");
        check(api.native_ClearEditPageData(doc1.value(), page.value()), "clear staged page data");
        long[] cleared = RuntimeProbe.nativeEditDiagnostics(page.value());
        check(cleared[0] == 0 && cleared[1] == cleared[2] && cleared[5] == 0,
              "clear restores committed metadata boundary");
        check(api.native_separateSharedDataInEditPage(doc1.value(), 0),
              "valid page shared-data preflight");
        check(!api.native_separateSharedDataInEditPage(doc1.value(), 99),
              "invalid page shared-data preflight rejected");
        check(RuntimeProbe.nativeLastErrorCode() == -1006, "page range error retained");

        pageApi.native_regenerateContent(page.value());
        check(RuntimeProbe.nativeLastErrorCode() == 0, "regenerate content succeeds");
        api.native_setEditStatus(doc1.value(), 99);
        check(RuntimeProbe.nativeLastErrorCode() == -1001, "invalid edit status classified");
        check(api.nGetEditStatus(doc1.value()) == MIXED, "invalid edit status does not mutate");
        check(RuntimeProbe.nativeLastErrorCode() == 0,
              "successful getter replaces prior invalid-argument error");
        api.native_setEditStatus(page.value(), TEXT);
        check(RuntimeProbe.nativeLastErrorCode() == -1003, "page cannot act as document");

        check(pageApi.native_unloadPage2(page.value()) == 0, "close page");
        check(!api.native_ClearEditFocus(doc1.value(), page.value()), "stale page rejected");
        check(RuntimeProbe.nativeLastErrorCode() == -1005, "stale page error retained");
        check(api.native_closePDF(doc1.value()) == 0, "close first document");
        check(api.native_closePDF(doc2.value()) == 0, "close second document");
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "no leaked handles");

        System.out.println("EDIT_LIFECYCLE PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
