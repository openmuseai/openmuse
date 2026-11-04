// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.base.log.Log
use util::base::log::Log::Log;
// Java: import cn.wps.enml.io.IENMLDocumentImporter
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
// Java: import java.lang.reflect.Constructor
// Java: import java.lang.reflect.InvocationTargetException
// Java: import java.util.HashMap
use std::collections::HashMap;

// Java field: String TAG
pub(crate) const TAG: &str = "HandlerPool";

// Java: public class HandlerPool
pub struct HandlerPool {
    // Java field: HashMap<String, XmlDefaultHandler> mMap
    mMap: HashMap<String, i32>,
}

// Java: public class HandlerPool
impl HandlerPool {
    // Java: HandlerPool()
    pub fn new() -> Self {
      // Java: super();
      HandlerPool {
          mMap: HashMap::new(),
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mMap.clear();
      self.mMap.clear();
    }

    // Java: public <T> T getHandler(String elementName, Class<T> clazz, HandlerPool pool, IENMLDocumentImporter importer)
    pub fn getHandler(&mut self, _element_name: &str, _pool: &HandlerPool, _importer: &dyn IENMLDocumentImporter) -> i32 {
      // Java: Assert.assertNotNull("elementName should not be null.", elementName);
      // Java: Assert.assertNotNull("clazz should not be null.", clazz);
      // Java: Assert.assertNotNull("pool should not be null.", pool);
      // Java: Assert.assertNotNull("importer should not be null.", importer);
      // Java: T handler = (T)this.mMap.get(elementName);
      // Java: if (null == handler)
      {
         // Java: Constructor<T> ctor = getConstructor(clazz);
         // Java: Assert.assertNotNull("ctor should not be null.", ctor);
         // Java: handler = (T)newHandlerInstance(ctor, importer, pool);
         // Java: Assert.assertNotNull("handler should not be null.", handler);
         // Java: Assert.assertTrue("handler should be instance of XmlDefaultHandler.", handler instanceof XmlDefaultHandler);
         // Java: this.mMap.put(elementName, (XmlDefaultHandler)handler);
      }
      // Java: return handler;
      todo!()
    }

    // Java: static private static <T> Constructor<T> getConstructor(Class<T> clazz)
    pub fn getConstructor() -> i32 {
      // Java: Assert.assertNotNull("clazz should not be null.", clazz);
      // Java: Constructor<T> ctor = null;
      // Java: try
      {
         // Java: ctor = clazz.getConstructor(IENMLDocumentImporter.class, HandlerPool.class);
      }
      // Java: catch (SecurityException e)
      {
         // Java: Log.e(TAG, "SecurityException", e);
      }
      // Java: catch (NoSuchMethodException e)
      {
         // Java: Log.d(TAG, "NoSuchMethodException", e);
      }
      // Java: return ctor;
      todo!()
    }

    // Java: static private static <T> T newHandlerInstance(Constructor<T> ctor, IENMLDocumentImporter importer, HandlerPool pool)
    pub fn newHandlerInstance(_importer: &dyn IENMLDocumentImporter, _pool: &HandlerPool) -> i32 {
      // Java: Assert.assertNotNull("ctor should not be null.", ctor);
      // Java: Assert.assertNotNull("importer should not be null.", importer);
      // Java: Assert.assertNotNull("pool should not be null.", pool);
      // Java: T instance = null;
      // Java: try
      {
         // Java: instance = (T)ctor.newInstance(importer, pool);
      }
      // Java: catch (IllegalArgumentException e)
      {
         // Java: Log.e(TAG, "IllegalArgumentException", e);
      }
      // Java: catch (InstantiationException e)
      {
         // Java: Log.e(TAG, "InstantiationException", e);
      }
      // Java: catch (IllegalAccessException e)
      {
         // Java: Log.e(TAG, "IllegalAccessException", e);
      }
      // Java: catch (InvocationTargetException e)
      {
         // Java: Log.e(TAG, "InvocationTargetException", e);
      }
      // Java: return instance;
      todo!()
    }

}
