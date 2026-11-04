// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.core.runtime.IClassLoaderManager
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.util.ArtMode
// Java: import java.lang.reflect.Constructor
// Java: import java.lang.reflect.InvocationTargetException

// Java field: String TAG
pub(crate) const TAG: &str = "MhtReflectionHelper";
// Java field: String MHT_READER_CLASS
pub(crate) const MHT_READER_CLASS: &str = "cn.wps.moffice.writer.io.mht2html.Mht2Html";

// Java: public class MhtReflectionHelper
pub struct MhtReflectionHelper {
}

// Java: public class MhtReflectionHelper
impl MhtReflectionHelper {
    // Java: MhtReflectionHelper()
    pub fn new() -> Self {
        // Java: super();
        MhtReflectionHelper {}
    }

    // Java: static IMhtReader getMhtReader(Object owner, String srcPath, RandomAccessFile raf, boolean isExcelMht)
    pub fn getMhtReader(_owner: &dyn std::any::Any, _srcPath: &str, _raf: &dyn std::any::Any, _isExcelMht: bool) -> Option<Box<dyn std::any::Any>> {
        // Java: ClassLoader cl = null;
        // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
        {
            // Java: cl = IClassLoaderManager.getInstance().getWriterCoreExtensionClassLoader();
        }
        // Java: else
        {
            // Java: cl = owner.getClass().getClassLoader();
        }
        // Java: IMhtReader reader = null;
        // Java: try
        {
            // Java: Class<?> clazz = cl.loadClass("cn.wps.moffice.writer.io.mht2html.Mht2Html");
            // Java: Class<?>[] argTypes = new Class[]{String.class, RandomAccessFile.class, Boolean.TYPE};
            // Java: Constructor<?> constructor = clazz.getConstructor(argTypes);
            // Java: reader = (IMhtReader)constructor.newInstance(srcPath, raf, isExcelMht);
        }
        // Java: catch (ClassNotFoundException e)
        {
            // Java: IOLog.e(TAG, "ClassNotFoundException", e);
        }
        // Java: catch (InstantiationException e)
        {
            // Java: IOLog.e(TAG, "InstantiationException", e);
        }
        // Java: catch (IllegalAccessException e)
        {
            // Java: IOLog.e(TAG, "IllegalAccessException", e);
        }
        // Java: catch (NoSuchMethodException e)
        {
            // Java: IOLog.e(TAG, "NoSuchMethodException", e);
        }
        // Java: catch (IllegalArgumentException e)
        {
            // Java: IOLog.e(TAG, "IllegalArgumentException", e);
        }
        // Java: catch (InvocationTargetException e)
        {
            // Java: IOLog.e(TAG, "InvocationTargetException", e);
        }
        // Java: IOAssert.assertNotNull("reader should not be null.", reader);
        // Java: return reader;
        todo!()
    }

}
