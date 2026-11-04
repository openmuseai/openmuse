// PDFDocument stub - minimal native surface to drive the ORIGINAL libkwopdf.so
// document/page lifecycle (same FQCN + method names + field layout).
package cn.wps.moffice.pdf.core.std;

import cn.wps.moffice.pdf.core.NativeHandle;

public class PDFDocument {
    private long mNativePdfDoc;

    // Original SO probe only: module-global setting, not a document handle.
    public native boolean native_SetSectionRecognizeAlgGen(int generation);
    public native int native_GetSectionRecognizeAlgGen();

    public static native int native_openPDF(String filePath, NativeHandle nativePdfDoc);
    public native boolean native_isValid(long nativePdfDoc);
    public native int native_getPageCount(long nativePdfDoc);
    public native int native_getPage(long nativePdfDoc, int index, NativeHandle nativePdfPage);
    public native int native_getPageSizeFromPageIndex(long nativePdfDoc, int index,
                                                       android.graphics.RectF pageSize);
    public native int native_reopenInPassword(long nativePdfDoc, String password);
    public native int native_reopen(long nativePdfDoc, String filePath);
    public native long native_openOptimize(long nativePdfDoc, String filePath, int flag);
    public native int native_continueOptimize(long nativePdfDoc, long nativeOptimize,
                                               int milliseconds);
    public native void native_closeOptimize(long nativePdfDoc, long nativeOptimize);
    public native void native_setInsertStatus(long nativePdfDoc, int status);
    public native void native_setEditStatus(long nativePdfDoc, int status);
    public native int nGetInsertStatus(long nativePdfDoc);
    public native int nGetEditStatus(long nativePdfDoc);
    public native boolean native_syncEditPageObj(long docHandle, long pageHandle,
                                                  boolean isSave);
    public native boolean native_ClearEditPageData(long docHandle, long pageHandle);
    public native boolean native_ClearEditFocus(long docHandle, long pageHandle);
    public native boolean native_ClearHoverHandle(long docHandle, long pageHandle);
    public native boolean native_separateSharedDataInEditPage(long docHandle, int pageIndex);
    public native boolean native_EditBackup(long docHandle, long pageHandle, int editStatus);
    public native long native_getFormfill(long nativePdfDoc);
    public native boolean native_setPasswords(long nativePdfDoc, String filePath,
                                               String ownerPassword, String userPassword,
                                               int permissions, boolean isUserPassword);
    public native int native_closePDF(long nativePdfDoc);
}
