// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.core.runtime.IClassLoaderManager
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.online.security.IOnlineSecurityTool
// Java: import cn.wps.moffice.util.ArtMode
// Java: import java.lang.reflect.Constructor
// Java: import java.lang.reflect.InvocationTargetException
// Java: import java.lang.reflect.Method

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: String FILEPARSER_CLASS
pub(crate) const FILEPARSER_CLASS: &str = "cn.wps.io.file.parser.FileParser";
// Java field: String TEXTDOCUMENT_CLASS
pub(crate) const TEXTDOCUMENT_CLASS: &str = "cn.wps.moffice.writer.core.TextDocument";
// Java field: String KMOBOOK_CLASS
pub(crate) const KMOBOOK_CLASS: &str = "cn.wps.moss.app.KmoBookApi";
// Java field: String PRESENTATION_CLASS
pub(crate) const PRESENTATION_CLASS: &str = "cn.wps.show.app.KmoPresentation";
// Java field: String PDFDOCUMENT_CLASS
pub(crate) const PDFDOCUMENT_CLASS: &str = "cn.wps.moffice.pdf.core.std.PDFDocument";
// Java field: String ONLINE_SECURITY_IMPL
pub(crate) const ONLINE_SECURITY_IMPL: &str = "cn.wps.moffice.online.security.OnlineSecurityTool";

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

    // Java: static IDocument getTextDocument(Object owner)
    pub fn getTextDocument(_owner: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      {
         // Java: cl = IClassLoaderManager.getInstance().getWrClassLoader();
      }
      // Java: else
      {
         // Java: cl = owner.getClass().getClassLoader();
      }
      // Java: IDocument doc = null;
      // Java: try
      {
         // Java: Class<?> clazz = cl.loadClass("cn.wps.moffice.writer.core.TextDocument");
         // Java: Class<?>[] argTypes = new Class[0];
         // Java: Constructor<?> constructor = clazz.getConstructor(argTypes);
         // Java: doc = (IDocument)constructor.newInstance();
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
      // Java: IOAssert.assertNotNull("htmlReader should not be null.", doc);
      // Java: return doc;
      todo!()
    }

    // Java: static IFileParser getFileParser(Object owner, String path)
    pub fn getFileParser(_owner: &dyn std::any::Any, _path: &str) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      {
         // Java: cl = IClassLoaderManager.getInstance().getWrClassLoader();
      }
      // Java: else
      {
         // Java: cl = owner.getClass().getClassLoader();
      }
      // Java: IFileParser parser = null;
      // Java: try
      {
         // Java: Class<?> clazz = cl.loadClass("cn.wps.io.file.parser.FileParser");
         // Java: Class<?>[] argTypes = new Class[]{File.class};
         // Java: Constructor<?> constructor = clazz.getConstructor(argTypes);
         // Java: parser = (IFileParser)constructor.newInstance(new File(path));
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
      // Java: IOAssert.assertNotNull("htmlReader should not be null.", parser);
      // Java: return parser;
      todo!()
    }

    // Java: static IDocument getKmoBook(Object owner)
    pub fn getKmoBook(_owner: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      {
         // Java: cl = IClassLoaderManager.getInstance().getSsClassLoader();
      }
      // Java: else
      {
         // Java: cl = owner.getClass().getClassLoader();
      }
      // Java: IDocument doc = null;
      // Java: try
      {
         // Java: Class<?> clazz = cl.loadClass("cn.wps.moss.app.KmoBookApi");
         // Java: Constructor<?> constructor = clazz.getConstructor();
         // Java: doc = (IDocument)constructor.newInstance();
         // Java: Method method = clazz.getMethod("setAutoCalcOn", Boolean.TYPE);
         // Java: method.invoke(doc, false);
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
      // Java: IOAssert.assertNotNull("htmlReader should not be null.", doc);
      // Java: return doc;
      todo!()
    }

    // Java: static IDocument getKmoPresentation(Object owner)
    pub fn getKmoPresentation(_owner: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      {
         // Java: cl = IClassLoaderManager.getInstance().getPptClassLoader();
      }
      // Java: else
      {
         // Java: cl = owner.getClass().getClassLoader();
      }
      // Java: IDocument doc = null;
      // Java: try
      {
         // Java: Class<?> clazz = cl.loadClass("cn.wps.show.app.KmoPresentation");
         // Java: Constructor<?> constructor = clazz.getConstructor();
         // Java: doc = (IDocument)constructor.newInstance();
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
      // Java: IOAssert.assertNotNull("htmlReader should not be null.", doc);
      // Java: return doc;
      todo!()
    }

    // Java: static IDocument getPDFDocument(Object owner)
    pub fn getPDFDocument(_owner: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      {
         // Java: cl = IClassLoaderManager.getInstance().getPdfClassLoader();
      }
      // Java: else
      {
         // Java: cl = owner.getClass().getClassLoader();
      }
      // Java: IDocument doc = null;
      // Java: try
      {
         // Java: Class<?> clazz = cl.loadClass("cn.wps.moffice.pdf.core.std.PDFDocument");
         // Java: Constructor<?> constructor = clazz.getConstructor();
         // Java: doc = (IDocument)constructor.newInstance();
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
      // Java: IOAssert.assertNotNull("htmlReader should not be null.", doc);
      // Java: return doc;
      todo!()
    }

    // Java: static IOnlineSecurityTool getSecurityTool(Object owner)
    pub fn getSecurityTool(_owner: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      {
         // Java: cl = IClassLoaderManager.getInstance().getSecureDocClassLoader();
      }
      // Java: else
      {
         // Java: cl = owner.getClass().getClassLoader();
      }
      // Java: IOnlineSecurityTool securityTool = null;
      // Java: try
      {
         // Java: Class<?> clazz = cl.loadClass("cn.wps.moffice.online.security.OnlineSecurityTool");
         // Java: Constructor<?> constructor = clazz.getConstructor();
         // Java: securityTool = (IOnlineSecurityTool)constructor.newInstance();
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
      // Java: IOAssert.assertNotNull("securityTool should not be null.", securityTool);
      // Java: return securityTool;
      todo!()
    }

}
