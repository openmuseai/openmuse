// NativeHandle - same FQCN/fields as WPS's, for JNI out-param binding.
package cn.wps.moffice.pdf.core;

public class NativeHandle {
    protected long mValue = 0;

    public long value() {
        return mValue;
    }
}