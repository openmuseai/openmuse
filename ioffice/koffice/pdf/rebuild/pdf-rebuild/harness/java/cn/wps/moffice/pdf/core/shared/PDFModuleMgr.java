// PDFModuleMgr stub - fields must match the ORIGINAL class layout exactly,
// because the original native_initialize() writes mNativePdfModule via JNI.
package cn.wps.moffice.pdf.core.shared;

public class PDFModuleMgr {
    private long mNativePdfModule;

    public int initialize() {
        return native_initialize(0);
    }

    public native int native_initialize(int overseaVersion);
}
