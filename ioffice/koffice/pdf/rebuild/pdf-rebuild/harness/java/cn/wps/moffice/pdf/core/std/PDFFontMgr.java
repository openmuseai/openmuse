package cn.wps.moffice.pdf.core.std;

public final class PDFFontMgr {
    private PDFFontMgr() {}
    public static native boolean native_loadPrivateFont(String filePath);
    public static native boolean native_loadPrivateFonts(String directoryPath);
}
