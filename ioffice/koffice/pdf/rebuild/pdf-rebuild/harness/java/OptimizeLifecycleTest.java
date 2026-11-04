import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.select.PDFTextPage;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.rebuild.RuntimeProbe;
import java.io.File;

public final class OptimizeLifecycleTest {
    static int pass, fail;
    static void check(boolean value, String name) {
        if (value) pass++; else { fail++; System.err.println("FAIL: " + name); }
    }
    static NativeHandle open(String path) {
        NativeHandle out = new NativeHandle();
        check(PDFDocument.native_openPDF(path, out) == 0, "open source");
        return out;
    }
    public static void main(String[] args) {
        if (args.length != 3) throw new IllegalArgumentException("fixture and two outputs required");
        System.loadLibrary("kspfreetext");
        String source = args[0], outputA = args[1], outputB = args[2];
        new File(outputA).delete();
        new File(outputB).delete();
        PDFDocument api = new PDFDocument();
        PDFNativeSaver saver = new PDFNativeSaver();
        NativeHandle docA = open(source);
        NativeHandle docB = open(source);
        NativeHandle page = new NativeHandle();
        check(api.native_getPage(docA.value(), 0, page) == 0, "load Page for wrong-type test");

        check(api.native_openOptimize(page.value(), outputA, 0) == 0,
                "reject Page as optimizer Document");
        check(RuntimeProbe.nativeLastErrorCode() == -1003, "optimize wrong-type error");
        check(api.native_openOptimize(docA.value(), null, 0) == 0,
                "reject null optimize path");
        check(RuntimeProbe.nativeLastErrorCode() == -1001, "optimize null-path error");
        check(api.native_openOptimize(docA.value(), "", 0) == 0,
                "reject empty optimize path");
        check(RuntimeProbe.nativeLastErrorCode() == -1001, "optimize empty-path error");
        check(api.native_openOptimize(docA.value(), new File(source).getParent(), 0) == 0,
                "reject directory as optimize output");
        check(RuntimeProbe.nativeLastErrorCode() == -1007, "optimize output IO error");

        long optimize = api.native_openOptimize(docA.value(), outputA, 0);
        check(optimize != 0, "PDFDocument opens OptimizeSession");
        check(api.native_continueOptimize(docB.value(), optimize, 200) == -1,
                "reject cross-document continue");
        check(RuntimeProbe.nativeLastErrorCode() == -1004, "optimize owner mismatch error");
        api.native_closeOptimize(docB.value(), optimize);
        check(RuntimeProbe.nativeLastErrorCode() == -1004,
                "wrong owner cannot close OptimizeSession");
        check(api.native_continueOptimize(docA.value(), optimize, -1) == -1,
                "reject negative optimize time slice");
        check(RuntimeProbe.nativeLastErrorCode() == -1001, "negative slice error");
        check(api.native_continueOptimize(docA.value(), optimize, 200) == 100,
                "PDFDocument optimize reaches FINISHED");
        check(api.native_continueOptimize(docA.value(), optimize, 200) == 100,
                "finished optimize remains 100");
        check(new File(outputA).length() > 0, "optimized output is non-empty");
        api.native_closeOptimize(docA.value(), optimize);
        check(api.native_continueOptimize(docA.value(), optimize, 200) == -1,
                "closed OptimizeSession is stale");
        check(RuntimeProbe.nativeLastErrorCode() == -1005, "closed optimize stale error");

        NativeHandle roundTrip = open(outputA);
        check(api.native_getPageCount(roundTrip.value()) == api.native_getPageCount(docA.value()),
                "optimized output page count");
        NativeHandle optimizedPage = new NativeHandle();
        check(api.native_getPage(roundTrip.value(), 0, optimizedPage) == 0,
                "load optimized output page");
        long optimizedTextPage = new PDFTextPage().native_new(optimizedPage.value());
        check(optimizedTextPage != 0, "load optimized output TextPage");
        String optimizedText = new PDFTextPage().native_getPageText(optimizedTextPage);
        check(optimizedText != null && optimizedText.contains("3-Jaws indexing spacers"),
                "optimized output preserves text");
        check(api.native_closePDF(roundTrip.value()) == 0,
                "close optimized round trip with children");

        long wrapped = saver.openOptimize(docA.value(), outputB);
        check(wrapped != 0, "PDFNativeSaver opens OptimizeSession");
        check(saver.continueOptimize(docA.value(), wrapped, 200) == 100,
                "PDFNativeSaver optimize reaches FINISHED");
        saver.closeOptimize(docA.value(), wrapped);
        NativeHandle wrappedRoundTrip = open(outputB);
        check(api.native_getPageCount(wrappedRoundTrip.value()) ==
                api.native_getPageCount(docA.value()), "wrapped output page count");
        check(api.native_closePDF(wrappedRoundTrip.value()) == 0, "close wrapped round trip");

        String cancelled = outputB + ".cancelled";
        new File(cancelled).delete();
        long cancel = api.native_openOptimize(docA.value(), cancelled, 0);
        check(cancel != 0, "open cancellable OptimizeSession");
        api.native_closeOptimize(docA.value(), cancel);
        check(api.native_continueOptimize(docA.value(), cancel, 200) == -1,
                "cancelled OptimizeSession is stale");
        check(new File(cancelled).delete() || !new File(cancelled).exists(),
                "Java layer can remove cancelled temporary output");

        String cascaded = outputB + ".cascade";
        new File(cascaded).delete();
        long cascade = api.native_openOptimize(docA.value(), cascaded, 0);
        check(cascade != 0, "open optimizer before parser close");
        check(saver.closeParser(docA.value()) == 0, "close parser with optimizer child");
        check(api.native_continueOptimize(docA.value(), cascade, 200) == -1,
                "parser close invalidates OptimizeSession");
        check(RuntimeProbe.nativeLastErrorCode() == -1005, "parser-close optimize stale error");
        check(api.native_reopen(docA.value(), source) == 0, "reopen Document after optimizer cascade");
        new File(cascaded).delete();

        long documentCascade = saver.openOptimize(docA.value(), cascaded);
        check(documentCascade != 0, "open optimizer before Document close");
        check(api.native_closePDF(docA.value()) == 0, "Document close cascades optimizer");
        check(saver.continueOptimize(docA.value(), documentCascade, 200) == -1,
                "closed Document rejects optimizer continue");
        new File(cascaded).delete();
        check(api.native_closePDF(docB.value()) == 0, "close second Document");
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "optimize lifecycle leaves no handles");

        System.out.println("OPTIMIZE_LIFECYCLE PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
