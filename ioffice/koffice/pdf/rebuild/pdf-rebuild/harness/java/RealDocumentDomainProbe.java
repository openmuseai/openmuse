import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public final class RealDocumentDomainProbe {
    public static void main(String[] args) {
        if (args.length < 1) throw new IllegalArgumentException("PDF path required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        if (PDFDocument.native_openPDF(args[0], doc) != 0)
            throw new IllegalStateException("open failed: " + args[0]);
        document.native_setEditStatus(doc.value(), 1);
        int[] pages = args.length == 1 ? new int[] {0, 1} : new int[] {Integer.parseInt(args[1])};
        for (int pageIndex : pages) {
            NativeHandle page = new NativeHandle();
            if (document.native_getPage(doc.value(), pageIndex, page) != 0)
                throw new IllegalStateException("load page failed: " + pageIndex);
            PDFPage api = new PDFPage(page.value());
            api.native_SectionRecognize(page.value());
            int count = api.native_getTextEditorCount(page.value());
            System.out.println("PAGE index=" + pageIndex + " domains=" + count);
            for (int i = 0; i < count; i++) {
                RectF rect = new RectF();
                float[] matrix = new float[9];
                api.native_getTextEditorRect(page.value(), i, rect, matrix, false);
                System.out.println("DOMAIN index=" + i + " rect=" + rect
                        + " matrix=" + matrix[0] + "," + matrix[1] + "," + matrix[2]
                        + "," + matrix[3] + "," + matrix[4] + "," + matrix[5]);
            }
            String[] objects = RuntimeProbe.nativeTextObjectDiagnostics(page.value());
            if (objects != null) for (String object : objects) System.out.println("OBJECT " + object);
            api.native_unloadPage2(page.value());
        }
        document.native_closePDF(doc.value());
    }
}
