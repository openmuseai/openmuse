// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.core.runtime.IClassLoaderManager
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.util.ArtMode
// Java: import java.lang.reflect.Constructor
// Java: import java.lang.reflect.InvocationTargetException

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;
// Java field: String WRITER_TEXT_EXTRACTOR_CLASS
pub(crate) const WRITER_TEXT_EXTRACTOR_CLASS: &str = "cn.wps.moffice.text_extractor.WriterTextExtractor";
// Java field: String TXT_TEXT_EXTRACTOR_CLASS
pub(crate) const TXT_TEXT_EXTRACTOR_CLASS: &str = "cn.wps.moffice.text_extractor.TxtTextExtractor";

// Java: public class ReflectionHelper
pub struct ReflectionHelper {
}

// Java: public class ReflectionHelper
impl ReflectionHelper {
    // Java: ReflectionHelper()
    pub fn new() -> Self {
      // Java: super();
      ReflectionHelper {}
    }

    // Java: static ITextExtractor getWriterTextExtractor(Object owner, String path, String password, int length, ITEListener listener)
    pub fn getWriterTextExtractor(_owner: &dyn std::any::Any, _path: &str, _password: &str, _length: i32, _listener: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: return getExtractor(owner, "cn.wps.moffice.text_extractor.WriterTextExtractor", path, password, length, listener);
      todo!()
    }

    // Java: static ITextExtractor getTxtTextExtractor(Object owner, String path, String password, int length, ITEListener listener)
    pub fn getTxtTextExtractor(_owner: &dyn std::any::Any, _path: &str, _password: &str, _length: i32, _listener: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: return getExtractor(owner, "cn.wps.moffice.text_extractor.TxtTextExtractor", path, password, length, listener);
      todo!()
    }

    // Java: static ITextExtractor getExtractor(Object owner, String className, String path, String password, int length, ITEListener listener)
    pub fn getExtractor(_owner: &dyn std::any::Any, _className: &str, _path: &str, _password: &str, _length: i32, _listener: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      // Java: cl = IClassLoaderManager.getInstance().getWrClassLoader();
      // Java: else
      // Java: cl = owner.getClass().getClassLoader();
      // Java: ITextExtractor textExtractor = null;
      // Java: try
      // Java: Class<?> clazz = cl.loadClass(className);
      // Java: Class<?>[] argTypes = new Class[]{String.class, String.class, Integer.TYPE, ITEListener.class};
      // Java: Constructor<?> constructor = clazz.getConstructor(argTypes);
      // Java: textExtractor = (ITextExtractor)constructor.newInstance(path, password, length, listener);
      // Java: catch (ClassNotFoundException e)
      // Java: IOLog.e(TAG, "ClassNotFoundException", e);
      // Java: catch (InstantiationException e)
      // Java: IOLog.e(TAG, "InstantiationException", e);
      // Java: catch (IllegalAccessException e)
      // Java: IOLog.e(TAG, "IllegalAccessException", e);
      // Java: catch (NoSuchMethodException e)
      // Java: IOLog.e(TAG, "NoSuchMethodException", e);
      // Java: catch (IllegalArgumentException e)
      // Java: IOLog.e(TAG, "IllegalArgumentException", e);
      // Java: catch (InvocationTargetException e)
      // Java: IOLog.e(TAG, "InvocationTargetException", e);
      // Java: IOAssert.assertNotNull("textExtractor should not be null.", textExtractor);
      // Java: return textExtractor;
      todo!()
    }

}
