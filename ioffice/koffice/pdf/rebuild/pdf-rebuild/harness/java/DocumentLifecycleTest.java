import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.annot.PDFAnnotationEditor;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public class DocumentLifecycleTest {
    static int pass = 0;
    static int fail = 0;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static NativeHandle open(String path) {
        NativeHandle out = new NativeHandle();
        check(PDFDocument.native_openPDF(path, out) == 0, "open document");
        check(out.value() != 0, "NativeHandle.mValue writeback");
        return out;
    }

    public static void main(String[] args) {
        if (args.length != 2) throw new IllegalArgumentException("plain and password fixture paths required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();

        check(PDFDocument.native_openPDF(null, new NativeHandle()) == -1001,
              "null path rejected");
        check("INVALID_ARGUMENT".equals(RuntimeProbe.nativeLastErrorName()),
              "invalid argument error name");

        NativeHandle doc1 = open(args[0]);
        check(document.native_isValid(doc1.value()), "document valid");
        check(document.native_getPageCount(doc1.value()) == 5, "page count=5");

        NativeHandle page1 = new NativeHandle();
        check(document.native_getPage(doc1.value(), 0, page1) == 0, "get first page");
        check(page1.value() != 0, "page NativeHandle.mValue writeback");
        PDFPage page = new PDFPage(page1.value());
        check(page.native_isValid(page1.value()), "page valid");
        check(page.native_getPageIndex(page1.value()) == 0, "page index is native 0-based");
        RectF size = page.getPageSize();
        check(size.width() > 0 && size.height() > 0, "page size positive");
        RectF sizeByIndex = new RectF();
        check(document.native_getPageSizeFromPageIndex(doc1.value(), 0, sizeByIndex) == 0,
              "page size by document index");
        check(Math.abs(size.width() - sizeByIndex.width()) < 0.01f &&
              Math.abs(size.height() - sizeByIndex.height()) < 0.01f,
              "page size APIs agree");

        check(document.native_GetSectionRecognizeAlgGen() == 1,
              "section recognition generation defaults to one");
        check(document.native_SetSectionRecognizeAlgGen(3) &&
              document.native_GetSectionRecognizeAlgGen() == 3,
              "section recognition generation is module state");
        check(document.native_SetSectionRecognizeAlgGen(1),
              "restore section recognition generation");
        long[] contextBefore = RuntimeProbe.nativeRecognitionContextDiagnostics(doc1.value());
        check(contextBefore != null && contextBefore.length == 8 &&
              contextBefore[0] == 0 && contextBefore[1] == 0 && contextBefore[2] == -1,
              "new document has empty recognition context");
        document.native_setEditStatus(doc1.value(), 1);
        check(page.native_SectionRecognize(page1.value()),
              "recognize page through production PDFPage JNI");
        long[] contextAfter = RuntimeProbe.nativeRecognitionContextDiagnostics(doc1.value());
        check(contextAfter != null && contextAfter[0] == 1 && contextAfter[1] == 1 &&
              contextAfter[2] == 0 && contextAfter[4] == 1 &&
              contextAfter[5] == 1 && contextAfter[6] == contextAfter[3] &&
              contextAfter[7] == 0,
              "document context records recognition after page wrapper call");
        check(page.native_SectionRecognize(page1.value()),
              "repeat recognition replaces cached page records");
        long[] contextReplaced = RuntimeProbe.nativeRecognitionContextDiagnostics(doc1.value());
        check(contextReplaced != null && contextReplaced[0] == 2 &&
              contextReplaced[1] == 1 && contextReplaced[5] == 1 &&
              contextReplaced[6] == contextReplaced[3] && contextReplaced[7] == 1,
              "page-keyed recognition cache replaces instead of appending");

        check(document.native_getPage(doc1.value(), 5, new NativeHandle()) == -1006,
              "out-of-range page rejected");
        check(document.native_getPageCount(page1.value()) == -1003,
              "wrong handle type rejected");
        check(document.native_reopenInPassword(doc1.value(), "unused") == -1001,
              "reopen only accepts password-pending document");

        NativeHandle doc2 = open(args[0]);
        long[] secondContext = RuntimeProbe.nativeRecognitionContextDiagnostics(doc2.value());
        check(secondContext != null && secondContext[0] == 0 && secondContext[1] == 0,
              "recognition context is isolated per document");
        check(RuntimeProbe.nativeValidatePageOwner(page1.value(), doc2.value()) == -1004,
              "cross-document page rejected");
        check(new PDFAnnotationEditor().native_addFreeText(doc1.value(), page1.value(), 20, 20),
              "WPS FreeText accepts registry handles");
        check(RuntimeProbe.nativeLiveHandleCount() == 3, "live handle count");

        check(page.native_unloadPage2(page1.value()) == 0, "close page");
        check(!page.native_isValid(page1.value()), "closed page invalid");
        check(page.native_unloadPage2(page1.value()) == -1005, "double page close is stale");
        check(document.native_closePDF(doc1.value()) == 0, "close first document");
        check(!document.native_isValid(doc1.value()), "closed document invalid");
        check(RuntimeProbe.nativeRecognitionContextDiagnostics(doc1.value()) == null,
              "closed document rejects recognition diagnostics");
        check(RuntimeProbe.nativeLastErrorCode() == -1005,
              "recognition diagnostics preserve stale generation error");
        check(document.native_closePDF(doc2.value()) == 0, "close second document");

        NativeHandle doc3 = open(args[0]);
        NativeHandle page3 = new NativeHandle();
        check(document.native_getPage(doc3.value(), 0, page3) == 0, "get page for cascade close");
        check(document.native_closePDF(doc3.value()) == 0, "cascade close document");
        check(!page.native_isValid(page3.value()), "document close invalidates child page");
        check(RuntimeProbe.nativeLastErrorCode() == -1005, "stale generation error retained");
        check("STALE_HANDLE".equals(RuntimeProbe.nativeLastErrorName()),
              "stale generation error name");

        NativeHandle missing = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0] + ".missing", missing) == -1007,
              "missing file maps to IO error");
        check(missing.value() == 0, "failed open leaves output handle zero");

        NativeHandle locked = new NativeHandle();
        check(PDFDocument.native_openPDF(args[1], locked) == -3,
              "password document reports legacy Password=-3");
        check(locked.value() != 0, "password document preserves reopen handle");
        check(document.native_reopenInPassword(locked.value(), "wrong") == -3,
              "wrong password rejected");
        check(document.native_reopenInPassword(locked.value(), "user123") == 0,
              "correct password reopens document");
        check(document.native_getPageCount(locked.value()) == 5,
              "reopened password document page count");
        check(document.native_closePDF(locked.value()) == 0,
              "close password document");
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "registry empty after lifecycle");

        System.out.println("DOCUMENT_LIFECYCLE PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
