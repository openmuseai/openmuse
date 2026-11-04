import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public final class LongDocumentEditScanTest {
    static int pass, fail;
    static void check(boolean value, String name) {
        if (value) pass++; else { fail++; System.out.println("FAIL: " + name); }
    }
    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("long PDF required");
        System.loadLibrary("kspfreetext");
        PDFDocument api = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open long document");
        int pages = api.native_getPageCount(doc.value());
        check(pages >= 50, "long corpus has at least 50 pages (" + pages + ")");
        api.native_setEditStatus(doc.value(), 1);
        int recognized = 0, domains = 0;
        long started = System.nanoTime();
        for (int index = 0; index < pages; ++index) {
            NativeHandle page = new NativeHandle();
            if (api.native_getPage(doc.value(), index, page) != 0) continue;
            PDFPage pageApi = new PDFPage(page.value());
            if (pageApi.native_SectionRecognize(page.value())) {
                recognized++;
                domains += pageApi.native_getTextEditorCount(page.value());
            }
            pageApi.native_unloadPage2(page.value());
        }
        long elapsedMs = (System.nanoTime() - started) / 1_000_000L;
        check(recognized == pages, "every page recognized (" + recognized + "/" + pages + ")");
        check(domains > pages, "semantic domains found across document (" + domains + ")");
        check(elapsedMs < 60_000, "full edit scan under 60s (" + elapsedMs + "ms)");
        check(api.native_closePDF(doc.value()) == 0, "close long document");
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "page scan leaks no handles");
        System.out.println("LONG_DOCUMENT_EDIT_SCAN PASS=" + pass + " FAIL=" + fail
                + " PAGES=" + pages + " DOMAINS=" + domains + " ELAPSED_MS=" + elapsedMs);
        if (fail != 0) System.exit(1);
    }
}
