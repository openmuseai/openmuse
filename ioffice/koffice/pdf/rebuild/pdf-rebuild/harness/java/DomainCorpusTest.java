import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public class DomainCorpusTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static boolean close(float a, float b) { return Math.abs(a - b) < 0.001f; }

    static float[] multiply(float[] p, float[] q) {
        return new float[] {
            p[0] * q[0] + p[1] * q[3], p[0] * q[1] + p[1] * q[4],
            p[0] * q[2] + p[1] * q[5] + p[2],
            p[3] * q[0] + p[4] * q[3], p[3] * q[1] + p[4] * q[4],
            p[3] * q[2] + p[4] * q[5] + p[5], 0, 0, 1};
    }

    static void verify(String path, String name, int expectedDomains,
                       int expectedObjects, float width, float height,
                       float tx, float ty, int rotatedDomains) {
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(path, doc) == 0, name + " open");
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0, name + " page");
        PDFPage api = new PDFPage(page.value());
        RectF size = api.getPageSize();
        check(close(size.width(), width) && close(size.height(), height), name + " page size");
        float[] display = new float[9];
        check(api.native_getDisplayMatirx(page.value(), display,
                  new RectF(0, 0, width, height), 0) == 0,
              name + " display matrix call");
        check(close(display[0], 1) && close(display[1], 0) && close(display[2], tx) &&
              close(display[3], 0) && close(display[4], -1) && close(display[5], ty) &&
              close(display[8], 1), name + " display matrix values");
        document.native_setEditStatus(doc.value(), 1);
        check(api.native_SectionRecognize(page.value()), name + " recognize");
        check(api.native_getTextEditorCount(page.value()) == expectedDomains,
              name + " domain topology");
        long[] ids = RuntimeProbe.nativeTextObjectIds(page.value());
        check(ids != null && ids.length == expectedObjects, name + " object count");
        int[] structure = RuntimeProbe.nativeRecognitionPageDiagnostics(doc.value(), 0);
        check(structure != null && structure.length == 10,
              name + " structural diagnostics");
        if (structure != null && structure.length == 10) {
            check(structure[0] == expectedDomains, name + " projected domain count");
            check(structure[2] == 1, name + " one LrSect root");
            check(structure[3] + structure[6] == expectedDomains,
                  name + " typed paragraph/formula editors");
            check(structure[4] >= expectedDomains, name + " LrLine children");
            check(structure[5] == expectedObjects, name + " one LrSpan per text object");
            check(structure[1] == 1 + expectedDomains + structure[4] + structure[5],
                  name + " complete structural tree");
        }
        int nonIdentity = 0;
        for (int i = 0; i < expectedDomains; i++) {
            RectF rect = new RectF();
            float[] forward = new float[9];
            float[] reverse = new float[9];
            check(api.native_getTextEditorRect(page.value(), i, rect, forward, false),
                  name + " forward geometry " + i);
            check(Math.abs(rect.right - rect.left) > 0.1f &&
                  Math.abs(rect.top - rect.bottom) > 0.1f,
                  name + " non-empty geometry " + i);
            check(api.native_getTextEditorRect(page.value(), i, new RectF(), reverse, true),
                  name + " reverse geometry " + i);
            float[] identity = multiply(forward, reverse);
            check(close(identity[0], 1) && close(identity[4], 1) &&
                  close(identity[2], 0) && close(identity[5], 0),
                  name + " invertible matrix " + i);
            if (Math.abs(forward[1]) > 0.01f || Math.abs(forward[3]) > 0.01f)
                nonIdentity++;
        }
        check(nonIdentity == rotatedDomains, name + " orientation domains");
        check(api.native_unloadPage2(page.value()) == 0, name + " close page");
        check(document.native_closePDF(doc.value()) == 0, name + " close document");
    }

    public static void main(String[] args) {
        if (args.length != 4) throw new IllegalArgumentException("four corpus PDFs required");
        System.loadLibrary("kspfreetext");
        verify(args[0], "multicolumn", 3, 7, 612, 792, 0, 792, 0);
        verify(args[1], "rotated", 3, 5, 612, 792, 0, 792, 2);
        verify(args[2], "cropbox", 3, 4, 540, 648, -36, 720, 0);
        verify(args[3], "form", 1, 3, 612, 792, 0, 792, 0);
        System.out.println("DOMAIN_CORPUS PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
