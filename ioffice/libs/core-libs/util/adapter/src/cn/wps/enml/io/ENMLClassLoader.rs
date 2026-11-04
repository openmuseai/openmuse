// Java: import cn.wps.core.runtime.IClassLoaderManager
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.util.ArtMode
// Java: import cn.wps.moffice.util.KSLog
// Java: import java.lang.reflect.Constructor
// Java: import java.lang.reflect.InvocationTargetException
// Java: import java.lang.reflect.Method

// Java field: String TAG
pub(crate) const TAG: &str = "ENMLClassLoader";
// Java field: Object RET_EXCEPTION_RAISED
static RET_EXCEPTION_RAISED: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: String CLASS_NAME_ENMLDOCUMENT
pub(crate) const CLASS_NAME_ENMLDOCUMENT: &str = "cn.wps.enml.io.ENMLDocument";
// Java field: Class<?>[] CONSTRUCTOR_ARGS_ENMLDOCUMENT
static CONSTRUCTOR_ARGS_ENMLDOCUMENT: std::sync::OnceLock<Vec<Box<dyn std::any::Any + Send + Sync>>> = std::sync::OnceLock::new();
// Java field: String METHOD_NAME_PARSE
pub(crate) const METHOD_NAME_PARSE: &str = "parse";
// Java field: Class<?>[] METHOD_ARGS_PARSE
static METHOD_ARGS_PARSE: std::sync::OnceLock<Vec<Box<dyn std::any::Any + Send + Sync>>> = std::sync::OnceLock::new();
// Java field: String METHOD_NAME_SET_DOCUMENT_IMPORTER
pub(crate) const METHOD_NAME_SET_DOCUMENT_IMPORTER: &str = "setDocumentImporter";
// Java field: Class<?>[] METHOD_ARGS_SET_DOCUMENT_IMPORTER
static METHOD_ARGS_SET_DOCUMENT_IMPORTER: std::sync::OnceLock<Vec<Box<dyn std::any::Any + Send + Sync>>> = std::sync::OnceLock::new();
// Java field: String CLASS_NAME_ENMLDIGESTREADER
pub(crate) const CLASS_NAME_ENMLDIGESTREADER: &str = "cn.wps.enml.io.ENMLDigestReader";
// Java field: Class<?>[] CONSTRUCTOR_ARGS_ENMLDIGESTREADER
static CONSTRUCTOR_ARGS_ENMLDIGESTREADER: std::sync::OnceLock<Vec<Box<dyn std::any::Any + Send + Sync>>> = std::sync::OnceLock::new();
// Java field: String METHOD_NAME_READ
pub(crate) const METHOD_NAME_READ: &str = "read";
// Java field: Class<?>[] METHOD_ARGS_READ
static METHOD_ARGS_READ: std::sync::OnceLock<Vec<Box<dyn std::any::Any + Send + Sync>>> = std::sync::OnceLock::new();

// Java: public class ENMLClassLoader
pub struct ENMLClassLoader {
}

// Java: public class ENMLClassLoader
impl ENMLClassLoader {
    // Java: ENMLClassLoader()
    pub fn new() -> Self {
      // Java: super();
      ENMLClassLoader {}
    }

    // Java: static ClassLoader doGetClassLoader()
    pub fn doGetClassLoader() -> Option<Box<dyn std::any::Any>> {
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      {
         // Java: IClassLoaderManager classLoaderManager = IClassLoaderManager.getInstance();
         // Java: return null == classLoaderManager ? null : classLoaderManager.getWriterAuxLibsClassLoader();
      }
      // Java: else
      {
         // Java: return ENMLClassLoader.class.getClassLoader();
      }
      todo!()
    }

    // Java: static private static Class<?> doLoadClass(String className)
    pub fn doLoadClass(_className: &str) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader loader = doGetClassLoader();
      // Java: if (null == loader)
      {
         // Java: return null;
      }
      // Java: else
      {
         // Java: Class<?> ret = null;
         // Java: try
         {
            // Java: ret = loader.loadClass(className);
         }
         // Java: catch (ClassNotFoundException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: return ret;
      }
      todo!()
    }

    // Java: static Object doNewInstance(Class<?> clz, Class<?>[] argTypes, Object... args)
    pub fn doNewInstance(_clz: Option<&dyn std::any::Any>, _argTypes: &[Box<dyn std::any::Any>], _args: &[Box<dyn std::any::Any>]) -> Option<Box<dyn std::any::Any>> {
      // Java: if (null == clz)
      {
         // Java: return null;
      }
      // Java: else
      {
         // Java: Object ret = RET_EXCEPTION_RAISED;
         // Java: try
         {
            // Java: Constructor<?> constructor = clz.getConstructor(argTypes);
            // Java: ret = constructor.newInstance(args);
         }
         // Java: catch (NoSuchMethodException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: catch (SecurityException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: catch (InstantiationException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: catch (IllegalAccessException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: catch (IllegalArgumentException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: catch (InvocationTargetException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: return ret;
      }
      todo!()
    }

    // Java: static Object doInvokeMethod(Class<?> clz, String methodName, Class<?>[] argTypes, Object obj, Object... args)
    pub fn doInvokeMethod(_clz: Option<&dyn std::any::Any>, _methodName: &str, _argTypes: &[Box<dyn std::any::Any>], _obj: &dyn std::any::Any, _args: &[Box<dyn std::any::Any>]) -> Option<Box<dyn std::any::Any>> {
      // Java: if (null == clz)
      {
         // Java: return null;
      }
      // Java: else
      {
         // Java: Object ret = RET_EXCEPTION_RAISED;
         // Java: try
         {
            // Java: Method method = clz.getDeclaredMethod(methodName, argTypes);
            // Java: method.setAccessible(true);
            // Java: ret = method.invoke(obj, args);
         }
         // Java: catch (NoSuchMethodException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: catch (SecurityException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: catch (IllegalAccessException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: catch (IllegalArgumentException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: catch (InvocationTargetException e)
         {
            // Java: KSLog.d(TAG, "", e);
         }
         // Java: return ret;
      }
      todo!()
    }

    // Java: static boolean parseENMLDocument(File f, IENMLDocumentImporter importer)
    pub fn parseENMLDocument(_f: &dyn std::any::Any, _importer: &dyn std::any::Any) -> bool {
      // Java: Class<?> clz = doLoadClass("cn.wps.enml.io.ENMLDocument");
      // Java: if (null == clz)
      {
         // Java: return false;
      }
      // Java: else
      {
         // Java: Object obj = doNewInstance(clz, CONSTRUCTOR_ARGS_ENMLDOCUMENT, f);
         // Java: if (null == obj)
         {
            // Java: return false;
         }
         // Java: else
         {
            // Java: Object ret1 = doInvokeMethod(clz, "setDocumentImporter", METHOD_ARGS_SET_DOCUMENT_IMPORTER, obj, importer);
            // Java: if (RET_EXCEPTION_RAISED == ret1)
            {
               // Java: return false;
            }
            // Java: else
            {
               // Java: Object ret2 = doInvokeMethod(clz, "parse", METHOD_ARGS_PARSE, obj);
               // Java: return RET_EXCEPTION_RAISED != ret2;
            }
         }
      }
      todo!()
    }

    // Java: static boolean readSummary(File f, StringBuffer outBuffer, int length)
    pub fn readSummary(_f: &dyn std::any::Any, _outBuffer: &mut String, _length: i32) -> bool {
      // Java: Class<?> clz = doLoadClass("cn.wps.enml.io.ENMLDigestReader");
      // Java: if (null == clz)
      {
         // Java: return false;
      }
      // Java: else
      {
         // Java: Object obj = doNewInstance(clz, CONSTRUCTOR_ARGS_ENMLDIGESTREADER, f.getAbsolutePath());
         // Java: if (null == obj)
         {
            // Java: return false;
         }
         // Java: else
         {
            // Java: Boolean ret = (Boolean)doInvokeMethod(clz, "read", METHOD_ARGS_READ, obj, outBuffer, length);
            // Java: if (RET_EXCEPTION_RAISED == ret)
            {
               // Java: return false;
            }
            // Java: else
            {
               // Java: return null != ret && ret;
            }
         }
      }
      todo!()
    }

}
