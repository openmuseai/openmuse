import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.select.PDFTextPage;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.rebuild.RuntimeProbe;
import java.io.File;

public final class SaveLifecycleTest {
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
        if (args.length != 2) throw new IllegalArgumentException("fixture and output required");
        System.loadLibrary("kspfreetext");
        String source = args[0], output = args[1];
        new File(output).delete();
        PDFDocument document = new PDFDocument();
        PDFNativeSaver saver = new PDFNativeSaver();
        NativeHandle doc = open(source);
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0, "load child page");
        long text = new PDFTextPage().native_new(page.value());
        check(text != 0, "load child TextPage");

        check(saver.newSession(page.value(), false) == 0, "reject Page as Document");
        check(RuntimeProbe.nativeLastErrorCode() == -1003, "save wrong-type error");
        long session = saver.newSession(doc.value(), false);
        check(session != 0, "create SaveSession");
        check(RuntimeProbe.nativeLiveHandleCount() == 4, "save session is registered child");
        check(!saver.start(session, null), "null save path rejected");
        check(RuntimeProbe.nativeLastErrorCode() == -1001, "null path error");
        check(saver.start(session, output), "start output writer");
        check(!saver.start(session, output), "cannot start one session twice");
        check(saver.run(session, 200) == 100, "timed continue reaches FINISHED");
        check(saver.run(session) == 100, "finished continue remains 100");
        check(new File(output).length() > 0, "saved file is non-empty");
        saver.destroy(session);
        check(saver.run(session) == -1, "destroyed SaveSession is stale");
        check(RuntimeProbe.nativeLastErrorCode() == -1005, "stale SaveSession error");

        NativeHandle roundTrip = open(output);
        check(document.native_getPageCount(roundTrip.value()) ==
                document.native_getPageCount(doc.value()), "rebuilt reopen page count");
        check(document.native_closePDF(roundTrip.value()) == 0, "close round-trip doc");

        String cancelled = output + ".cancelled";
        new File(cancelled).delete();
        long cancelSession = saver.newSession(doc.value(), false);
        check(cancelSession != 0 && saver.start(cancelSession, cancelled), "start cancellable save");
        saver.reset(cancelSession);
        saver.destroy(cancelSession);
        check(!new File(cancelled).exists(), "reset removes incomplete output");

        check(saver.closeParser(doc.value()) == 0, "close parser for reopen");
        check(new PDFTextPage().native_getPageText(text).isEmpty(),
              "close parser invalidates TextPage subtree");
        check(RuntimeProbe.nativeLastErrorCode() == -1005, "closed child is stale");
        check(document.native_reopen(doc.value(), output) == 0, "reopen same document token");
        check(document.native_getPageCount(doc.value()) > 0, "reopened document usable");

        long cascade = saver.newSession(doc.value(), false);
        check(cascade != 0, "create session for cascade close");
        check(document.native_closePDF(doc.value()) == 0, "close document and save child");
        check(saver.run(cascade) == -1, "document close invalidates SaveSession");
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "save lifecycle leaves no handles");

        System.out.println("SAVE_LIFECYCLE PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
