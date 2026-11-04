// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.core.runtime.IClassLoaderManager
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.io.IDocument
// Java: import cn.wps.moffice.util.ArtMode
// Java: import java.lang.reflect.Constructor
// Java: import java.lang.reflect.InvocationTargetException

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;
// Java field: String WRITER_SLIM_TOOL_MANAGER_CLASS
pub(crate) const WRITER_SLIM_TOOL_MANAGER_CLASS: &str = "cn.wps.moffice.writer.slim.WriterSlimToolMgr";
// Java field: String ET_SLIM_TOOL_MANAGER_CLASS
pub(crate) const ET_SLIM_TOOL_MANAGER_CLASS: &str = "cn.wps.moss.app.slim.ETSlimToolMgr";
// Java field: String WPP_SLIM_TOOL_MANAGER_CLASS
pub(crate) const WPP_SLIM_TOOL_MANAGER_CLASS: &str = "cn.wps.show.app.slim.WPPSlimToolMgr";
// Java field: String PDF_SLIM_TOOL_MANAGER_CLASS
pub(crate) const PDF_SLIM_TOOL_MANAGER_CLASS: &str = "cn.wps.moffice.pdf.core.slim.PDFSlimToolMgr";
// Java field: String SLIM_MEDIA_TOOL_CLASS
pub(crate) const SLIM_MEDIA_TOOL_CLASS: &str = "cn.wps.moffice.slim.SlimMediaTool";

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

    // Java: static ISlimToolManager getWriterSlimToolMgr(Object owner, IDocument slimSrc, ISlimListener listener)
    pub fn getWriterSlimToolMgr(_owner: &dyn std::any::Any, _slimSrc: &dyn std::any::Any, _listener: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      // Java: cl = IClassLoaderManager.getInstance().getWrClassLoader();
      // Java: else
      // Java: cl = owner.getClass().getClassLoader();
      // Java: ISlimToolManager slimToolMgr = null;
      // Java: try
      // Java: Class<?> clazz = cl.loadClass("cn.wps.moffice.writer.slim.WriterSlimToolMgr");
      // Java: Class<?>[] argTypes = new Class[]{IDocument.class, ISlimListener.class};
      // Java: Constructor<?> constructor = clazz.getConstructor(argTypes);
      // Java: slimToolMgr = (ISlimToolManager)constructor.newInstance(slimSrc, listener);
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
      // Java: IOAssert.assertNotNull("slimToolMgr should not be null.", slimToolMgr);
      // Java: return slimToolMgr;
      todo!()
    }

    // Java: static ISlimToolManager getETSlimToolMgr(Object owner, IDocument slimSrc, ISlimListener listener)
    pub fn getETSlimToolMgr(_owner: &dyn std::any::Any, _slimSrc: &dyn std::any::Any, _listener: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      // Java: cl = IClassLoaderManager.getInstance().getWrClassLoader();
      // Java: else
      // Java: cl = owner.getClass().getClassLoader();
      // Java: ISlimToolManager slimToolMgr = null;
      // Java: try
      // Java: Class<?> clazz = cl.loadClass("cn.wps.moss.app.slim.ETSlimToolMgr");
      // Java: Class<?>[] argTypes = new Class[]{IDocument.class, ISlimListener.class};
      // Java: Constructor<?> constructor = clazz.getConstructor(argTypes);
      // Java: slimToolMgr = (ISlimToolManager)constructor.newInstance(slimSrc, listener);
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
      // Java: IOAssert.assertNotNull("slimToolMgr should not be null.", slimToolMgr);
      // Java: return slimToolMgr;
      todo!()
    }

    // Java: static ISlimToolManager getWPPSlimToolMgr(Object owner, IDocument slimSrc, ISlimListener listener)
    pub fn getWPPSlimToolMgr(_owner: &dyn std::any::Any, _slimSrc: &dyn std::any::Any, _listener: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      // Java: cl = IClassLoaderManager.getInstance().getWrClassLoader();
      // Java: else
      // Java: cl = owner.getClass().getClassLoader();
      // Java: ISlimToolManager slimToolMgr = null;
      // Java: try
      // Java: Class<?> clazz = cl.loadClass("cn.wps.show.app.slim.WPPSlimToolMgr");
      // Java: Class<?>[] argTypes = new Class[]{IDocument.class, ISlimListener.class};
      // Java: Constructor<?> constructor = clazz.getConstructor(argTypes);
      // Java: slimToolMgr = (ISlimToolManager)constructor.newInstance(slimSrc, listener);
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
      // Java: IOAssert.assertNotNull("slimToolMgr should not be null.", slimToolMgr);
      // Java: return slimToolMgr;
      todo!()
    }

    // Java: static ISlimToolManager getPDFSlimToolMgr(Object owner, IDocument slimSrc, ISlimListener listener)
    pub fn getPDFSlimToolMgr(_owner: &dyn std::any::Any, _slimSrc: &dyn std::any::Any, _listener: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      // Java: cl = IClassLoaderManager.getInstance().getPdfClassLoader();
      // Java: else
      // Java: cl = owner.getClass().getClassLoader();
      // Java: ISlimToolManager slimToolMgr = null;
      // Java: try
      // Java: Class<?> clazz = cl.loadClass("cn.wps.moffice.pdf.core.slim.PDFSlimToolMgr");
      // Java: Class<?>[] argTypes = new Class[]{IDocument.class, ISlimListener.class};
      // Java: Constructor<?> constructor = clazz.getConstructor(argTypes);
      // Java: slimToolMgr = (ISlimToolManager)constructor.newInstance(slimSrc, listener);
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
      // Java: IOAssert.assertNotNull("slimToolMgr should not be null.", slimToolMgr);
      // Java: return slimToolMgr;
      todo!()
    }

    // Java: static ISlimMediaTool getSlimMediaTool(Object owner)
    pub fn getSlimMediaTool(_owner: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: ClassLoader cl = null;
      // Java: if (Platform.getMultiDexConfig() && !ArtMode.IS_VM_MULTIDEX_CAPABLE)
      // Java: cl = IClassLoaderManager.getInstance().getWrClassLoader();
      // Java: else
      // Java: cl = owner.getClass().getClassLoader();
      // Java: ISlimMediaTool slimMeidaTool = null;
      // Java: try
      // Java: Class<?> clazz = cl.loadClass("cn.wps.moffice.slim.SlimMediaTool");
      // Java: Class<?>[] argTypes = new Class[0];
      // Java: Constructor<?> constructor = clazz.getConstructor(argTypes);
      // Java: slimMeidaTool = (ISlimMediaTool)constructor.newInstance();
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
      // Java: IOAssert.assertNotNull("slimMeidaTool should not be null.", slimMeidaTool);
      // Java: return slimMeidaTool;
      todo!()
    }

}
