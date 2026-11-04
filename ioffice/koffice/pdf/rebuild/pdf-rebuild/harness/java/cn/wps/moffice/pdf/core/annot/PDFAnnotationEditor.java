// Rebuilt addFreeText native surface (exact WPS name).
package cn.wps.moffice.pdf.core.annot;

public class PDFAnnotationEditor {
    public native boolean native_addFreeText(long handle, long pageHandle,
                                             float left, float top);
}