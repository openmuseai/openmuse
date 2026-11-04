// TestPdf - rebuilt-layer test/render helpers (namespaced cn.wps.rebuild).
package cn.wps.rebuild;

public class TestPdf {
    // doc/page lifecycle (pdfium)
    public static native long nativeNewDoc();
    public static native void nativeCloseDoc(long doc);
    public static native long nativeAddPage(long doc, float w, float h);
    public static native long nativeGetPage(long doc, int index);
    public static native void nativeClosePage(long page);
    public static native boolean nativeSaveDoc(long doc, String path);
    public static native long nativeOpenPdf(String path);
    public static native float nativeGetPageWidth(long page);
    public static native float nativeGetPageHeight(long page);

    // content generation
    public static native boolean nativeDrawText(long doc, long page, float x, float y,
                                                String text, float size,
                                                int r, int g, int b);
    public static native boolean nativeDrawRect(long page, float x, float y,
                                                float w, float h, int r, int g, int b);

    // rendering
    public static native boolean nativeRenderPage(long page, int w, int h, int[] outArgb);

    // annotations (freetext feature)
    public static native int nativeAnnotCount(long page);
    public static native int nativeAnnotType(long page, int index);
    public static native boolean nativeSetContents(long page, String text);
}