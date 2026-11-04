// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map
use std::any::{Any, TypeId};
use std::sync::{Mutex, OnceLock};

// Java field: int sDefCapacity
static S_DEF_CAPACITY: OnceLock<Mutex<i32>> = OnceLock::new();
fn get_def_capacity() -> i32 { *S_DEF_CAPACITY.get_or_init(|| Mutex::new(16)).lock().unwrap() }

// Java field: Map<Class, ConcurrentArrayQueue> sPoolMap
static S_POOL_MAP: OnceLock<Mutex<HashMap<TypeId, Vec<Box<dyn Any + Send>>>>> = OnceLock::new();
fn get_pool_map() -> &'static Mutex<HashMap<TypeId, Vec<Box<dyn Any + Send>>>> {
    S_POOL_MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

// Java: public class AnyObjPool
pub struct AnyObjPool {
}

// Java: public class AnyObjPool
impl AnyObjPool {
    // Java: AnyObjPool()
    pub fn new() -> Self {
      // Java: super();
      AnyObjPool {}
    }

    // Java: static public static <T> T allocateOrNew(Class<T> tClass)
    pub fn allocateOrNew<T: Any + Send + Default>() -> T {
      // Java: ConcurrentArrayQueue pool;
      // Java: synchronized(tClass)
      {
         let mut pool_map = get_pool_map().lock().unwrap();
         // Java: pool = (ConcurrentArrayQueue)sPoolMap.get(tClass);
         let pool = pool_map.get_mut(&TypeId::of::<T>());
         // Java: T val = null;
         // Java: if (pool != null)
         if let Some(queue) = pool
         {
            // Java: val = (T)pool.tryPop();
            if let Some(boxed) = queue.pop() {
                if let Ok(val) = boxed.downcast::<T>() {
                    return *val;
                }
            }
         }
      }
      // Java: if (val == null)
      {
         // Java: val = (T)tryAllocate(tClass);
         // Java: return val;
         Self::tryAllocate::<T>()
      }
    }

    // Java: static public static <T> boolean tryReuse(T val)
    pub fn tryReuse<T: Any + Send>(val: T) -> bool {
      // Java: if (val != null)
      {
         // Java: Class cls = val.getClass();
         let type_id = TypeId::of::<T>();
         // Java: ConcurrentArrayQueue pool;
         // Java: synchronized(cls)
         {
            let mut pool_map = get_pool_map().lock().unwrap();
            // Java: pool = (ConcurrentArrayQueue)sPoolMap.get(cls);
            // Java: if (pool == null)
            let pool = pool_map.entry(type_id).or_insert_with(|| {
               // Java: pool = new ConcurrentArrayQueue(sDefCapacity);
               // Java: sPoolMap.put(cls, pool);
               Vec::with_capacity(get_def_capacity() as usize)
            });
            // Java: return pool.tryPush(val);
            pool.push(Box::new(val));
            return true;
         }
      }
      // Java: else
      {
         // Java: return false;
         false
      }
    }

    // Java: static void clear()
    pub fn clear() {
      // Java: sPoolMap.clear();
      get_pool_map().lock().unwrap().clear();
    }

    // Java: static <T> T tryAllocate(Class<T> tClass)
    pub fn tryAllocate<T: Default>() -> T {
      // Java: try
      {
         // Java: T val = (T)tClass.newInstance();
         let val = T::default();
         // Java: return val;
         return val;
      }
      // Java: catch (InstantiationException e)
      {
         // Java: throw new RuntimeException("InstantiationException", e);
         panic!("InstantiationException");
      }
      // Java: catch (IllegalAccessException e)
      {
         // Java: throw new RuntimeException("IllegalAccessException", e);
         panic!("IllegalAccessException");
      }
    }

}
