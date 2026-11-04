package cn.wps.moffice.pdf.core.io;

/** Host-harness ABI stub matching the SDK's package, class and overloads. */
public final class PDFNativeSaver {
    public long newSession(long document, boolean immutable) {
        return native_new(document, immutable);
    }
    public boolean start(long session, String path) { return native_start(session, path); }
    public int run(long session) { return native_continue(session); }
    public int run(long session, int milliseconds) {
        return native_continue(session, milliseconds);
    }
    public void reset(long session) { native_ResetDocument(session); }
    public void destroy(long session) { native_delete(session); }
    public int closeParser(long document) { return native_closeParser(document); }
    public long openOptimize(long document, String path) {
        return native_openOptimize(document, path);
    }
    public int continueOptimize(long document, long optimize, int milliseconds) {
        return native_continueOptimize(document, optimize, milliseconds);
    }
    public void closeOptimize(long document, long optimize) {
        native_closeOptimize(document, optimize);
    }

    private native long native_new(long document, boolean immutable);
    private native boolean native_start(long creator, String path);
    private native int native_continue(long creator, int milliseconds);
    private native int native_continue(long creator);
    private native void native_delete(long creator);
    private native void native_ResetDocument(long creator);
    private native int native_closeParser(long document);
    private native long native_openOptimize(long document, String path);
    private native int native_continueOptimize(long document, long optimize, int milliseconds);
    private native void native_closeOptimize(long document, long optimize);
}
