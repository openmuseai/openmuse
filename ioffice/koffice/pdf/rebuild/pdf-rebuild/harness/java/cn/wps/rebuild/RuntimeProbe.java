package cn.wps.rebuild;

public final class RuntimeProbe {
    private RuntimeProbe() {}

    public static native int nativeLastErrorCode();
    public static native String nativeLastErrorName();
    public static native int nativeValidatePageOwner(long page, long document);
    public static native int nativeLiveHandleCount();
    public static native long[] nativeRecognitionContextDiagnostics(long document);
    public static native long[] nativeRecognitionProcessorDiagnostics(long document);
    public static native int[] nativeRecognitionPageDiagnostics(long document, int pageIndex);
    public static native int[] nativeRecognitionDomainDiagnostics(long document, int pageIndex);
    public static native int nativeStagePageMutation(long page, int editStatus,
                                                      boolean focus, boolean hover);
    public static native long[] nativeEditDiagnostics(long page);
    public static native long[] nativeTextObjectIds(long page);
    public static native String[] nativeTextObjectDiagnostics(long page);
    public static native long[] nativePageObjectHandles(long page, int type);
    public static native long nativeTakePendingEditUndo(long page);
    public static native boolean nativeGetEditUndoDamage(long command, float[] bounds);
    public static native int nativeTextSelectionDirection(long page);
    public static native int[] nativeResolveBidiRuns(String text, int paragraphDirection);
    public static native int[] nativeShapeReplacement(long document, String originalName,
                                                       String text, int direction);
}
