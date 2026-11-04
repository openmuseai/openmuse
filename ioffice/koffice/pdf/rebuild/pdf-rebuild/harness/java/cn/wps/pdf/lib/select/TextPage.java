package cn.wps.pdf.lib.select;

/** Minimal exact-FQCN JNI contract used by host and device probes. */
public final class TextPage {
    private TextPage() {}

    public static native int nCharCount(long textPage);
    public static native float nFontSize(long textPage, int index);
    public static native int nColor(long textPage, int index);
    public static native String nSysFontName(long textPage, int index);
}
