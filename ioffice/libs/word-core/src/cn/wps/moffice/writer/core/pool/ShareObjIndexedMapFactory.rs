// Java: import cn.wps.collections.IShareObjIndexdMap
use util::collections::IShareObjIndexdMap::*;
// Java: import cn.wps.moffice.writer.core.pool.ShareObjIndexedMapImpl
use crate::cn::wps::moffice::writer::core::pool::ShareObjIndexedMapImpl::*;
// Java: import cn.wps.moffice.writer.core.pool.INormalObjIndexdMap
use crate::cn::wps::moffice::writer::core::pool::INormalObjIndexdMap::*;
// Java: import cn.wps.moffice.writer.core.pool.NormalObjIndexdMapImpl
use crate::cn::wps::moffice::writer::core::pool::NormalObjIndexdMapImpl::*;

// java public class ShareObjIndexedMapFactory
#[derive(Default)]
pub struct ShareObjIndexedMapFactory {
}

// java public class ShareObjIndexedMapFactory
impl ShareObjIndexedMapFactory {
    // java public static IShareObjIndexdMap<IShareObjIndexdMap.IShareObjIndexed> newMixClassInstance()
    pub fn newMixClassInstance() -> ShareObjIndexedMapImpl {
      // java return newInstance();
      return Self::newInstance();
    }

    // java public static IShareObjIndexdMap<T> newInstance()
    pub fn newInstance() -> ShareObjIndexedMapImpl {
      // java return new ShareObjIndexedMapImpl<T>();
      return ShareObjIndexedMapImpl::new();
    }

    // java public static INormalObjIndexdMap newNormalObjIndexdMap()
    pub fn newNormalObjIndexdMap() -> Box<dyn INormalObjIndexdMap> {
      // java return new NormalObjIndexdMapImpl();
      return NormalObjIndexdMapImpl::new();
    }

}
