#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::{self, KoBin as KoBinTrait, Decode, Encode, Factory, Null};
// Java: import cn.wps.kso.kproto.KoClosable
use crate::cn::wps::kso::kproto::KoClosable::KoClosable;
// Java: import cn.wps.kso.kproto.KoZone
use crate::cn::wps::kso::kproto::KoZone::KoZone;
// Java: import cn.wps.kso.kproto.internal.KpBinBox
use crate::cn::wps::kso::kproto::internal::KpBinBox::KpBinBox;
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import java.util.ArrayList
// Java: import java.util.Collection

// Java: public class CustomGeometry implements KoAr, KoClosable
pub struct CustomGeometry {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: GeomRect rect
    pub(crate) rect: Option<Box<dyn KoAr>>,
    // Java field: ConnectionSiteList cxnList
    pub(crate) cxnList: Option<Box<dyn KoAr>>,
    // Java field: AdjustHandleList ahList
    pub(crate) ahList: Option<Box<dyn KoAr>>,
    // Java field: GeomGuideList gdLst
    pub(crate) gdLst: Option<Box<dyn KoAr>>,
    // Java field: AdjustValueList avLst
    pub(crate) avLst: Option<Box<dyn KoAr>>,
    // Java field: PathList pathLst
    pub(crate) pathLst: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: CustomGeometry parent
    pub(crate) parent: Option<Box<CustomGeometry>>,
}

// Java: public class GeomRect
pub struct GeomRect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjCoordinate l
    pub(crate) l: Option<Box<dyn KoAr>>,
    // Java field: AdjCoordinate t
    pub(crate) t: Option<Box<dyn KoAr>>,
    // Java field: AdjCoordinate r
    pub(crate) r: Option<Box<dyn KoAr>>,
    // Java field: AdjCoordinate b
    pub(crate) b: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: GeomRect parent
    pub(crate) parent: Option<Box<GeomRect>>,
}

// Java: public class ConnectionSite
pub struct ConnectionSite {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjPoint2D pos
    pub(crate) pos: Option<Box<dyn KoAr>>,
    // Java field: AdjAngle ang
    pub(crate) ang: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ConnectionSite parent
    pub(crate) parent: Option<Box<ConnectionSite>>,
}

// Java: public class ConnectionSiteList
pub struct ConnectionSiteList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class AdjustHandle
pub struct AdjustHandle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: XY ahXY
    pub(crate) ahXY: Option<Box<dyn KoAr>>,
    // Java field: Polar ahPolar
    pub(crate) ahPolar: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: AdjustHandle parent
    pub(crate) parent: Option<Box<AdjustHandle>>,
}

// Java: public class XY
pub struct XY {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjCoordinate maxX
    pub(crate) maxX: Option<Box<dyn KoAr>>,
    // Java field: AdjCoordinate maxY
    pub(crate) maxY: Option<Box<dyn KoAr>>,
    // Java field: AdjCoordinate minX
    pub(crate) minX: Option<Box<dyn KoAr>>,
    // Java field: AdjCoordinate minY
    pub(crate) minY: Option<Box<dyn KoAr>>,
    // Java field: AdjPoint2D position
    pub(crate) position: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: XY parent
    pub(crate) parent: Option<Box<XY>>,
}

// Java: public class Polar
pub struct Polar {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjAngle maxAng
    pub(crate) maxAng: Option<Box<dyn KoAr>>,
    // Java field: AdjCoordinate maxR
    pub(crate) maxR: Option<Box<dyn KoAr>>,
    // Java field: AdjAngle minAng
    pub(crate) minAng: Option<Box<dyn KoAr>>,
    // Java field: AdjCoordinate minR
    pub(crate) minR: Option<Box<dyn KoAr>>,
    // Java field: AdjPoint2D position
    pub(crate) position: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Polar parent
    pub(crate) parent: Option<Box<Polar>>,
}

// Java: public class AdjustHandleList
pub struct AdjustHandleList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class GeomGuideList
pub struct GeomGuideList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class AdjustValueList
pub struct AdjustValueList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class Path
pub struct Path {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: PathSegmentList pathSegmentLst
    pub(crate) pathSegmentLst: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Path parent
    pub(crate) parent: Option<Box<Path>>,
}

// Java: public class ArcTo
pub struct ArcTo {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjCoordinate heightRadius
    pub(crate) heightRadius: Option<Box<dyn KoAr>>,
    // Java field: AdjCoordinate widthRadius
    pub(crate) widthRadius: Option<Box<dyn KoAr>>,
    // Java field: AdjAngle startAngle
    pub(crate) startAngle: Option<Box<dyn KoAr>>,
    // Java field: AdjAngle swingAngle
    pub(crate) swingAngle: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ArcTo parent
    pub(crate) parent: Option<Box<ArcTo>>,
}

// Java: public class CubicBezierCurveTo
pub struct CubicBezierCurveTo {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjPoint2D first
    pub(crate) first: Option<Box<dyn KoAr>>,
    // Java field: AdjPoint2D second
    pub(crate) second: Option<Box<dyn KoAr>>,
    // Java field: AdjPoint2D third
    pub(crate) third: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: CubicBezierCurveTo parent
    pub(crate) parent: Option<Box<CubicBezierCurveTo>>,
}

// Java: public class QuadraticBezierCurveTo
pub struct QuadraticBezierCurveTo {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjPoint2D first
    pub(crate) first: Option<Box<dyn KoAr>>,
    // Java field: AdjPoint2D second
    pub(crate) second: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: QuadraticBezierCurveTo parent
    pub(crate) parent: Option<Box<QuadraticBezierCurveTo>>,
}

// Java: public class PathSegment
pub struct PathSegment {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjPoint2D moveTo
    pub(crate) moveTo: Option<Box<dyn KoAr>>,
    // Java field: AdjPoint2D lnTo
    pub(crate) lnTo: Option<Box<dyn KoAr>>,
    // Java field: ArcTo arcTo
    pub(crate) arcTo: Option<Box<dyn KoAr>>,
    // Java field: QuadraticBezierCurveTo quadBezTo
    pub(crate) quadBezTo: Option<Box<dyn KoAr>>,
    // Java field: CubicBezierCurveTo cubicBezTo
    pub(crate) cubicBezTo: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: PathSegment parent
    pub(crate) parent: Option<Box<PathSegment>>,
}

// Java: public class PathSegmentList
pub struct PathSegmentList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class PathList
pub struct PathList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class CustomGeometry implements KoAr, KoClosable
impl CustomGeometry {
    // Java: static protected static <Value1 extends CustomGeometry> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::CustomGeometry_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static CustomGeometry create()
    pub fn create() -> Self {
      // Java: return initialize(CustomGeometry.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static CustomGeometry open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(CustomGeometry.class, bin);
      Self::initialize(bin)
    }

    // Java: CustomGeometry()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), rect: None, cxnList: None, ahList: None, gdLst: None, avLst: None, pathLst: None, parent: None,
      }
    }

    // Java: CustomGeometry(KoBin bin)
    pub fn CustomGeometry_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), rect: None, cxnList: None, ahList: None, gdLst: None, avLst: None, pathLst: None, parent: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.map.load(this.box.get());
      let bin_ref = self.bin_box.get();
      self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(CustomGeometry value)
    pub fn load_value(&mut self, value: Option<&CustomGeometry>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin_with_zone(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.rect != null && this.rect.dirty())
        if self.rect.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(1, this.rect.saveBin(zone));
         let saved = self.rect.as_mut().unwrap().saveBin(zone);
         self.map.put(1, saved);
      }
      // Java: if (this.cxnList != null && this.cxnList.dirty())
        if self.cxnList.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(2, this.cxnList.saveBin(zone));
         let saved = self.cxnList.as_mut().unwrap().saveBin(zone);
         self.map.put(2, saved);
      }
      // Java: if (this.ahList != null && this.ahList.dirty())
        if self.ahList.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(3, this.ahList.saveBin(zone));
         let saved = self.ahList.as_mut().unwrap().saveBin(zone);
         self.map.put(3, saved);
      }
      // Java: if (this.gdLst != null && this.gdLst.dirty())
        if self.gdLst.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(4, this.gdLst.saveBin(zone));
         let saved = self.gdLst.as_mut().unwrap().saveBin(zone);
         self.map.put(4, saved);
      }
      // Java: if (this.avLst != null && this.avLst.dirty())
        if self.avLst.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(5, this.avLst.saveBin(zone));
         let saved = self.avLst.as_mut().unwrap().saveBin(zone);
         self.map.put(5, saved);
      }
      // Java: if (this.pathLst != null && this.pathLst.dirty())
        if self.pathLst.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(6, this.pathLst.saveBin(zone));
         let saved = self.pathLst.as_mut().unwrap().saveBin(zone);
         self.map.put(6, saved);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
         let saved = self.map.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(CustomGeometry o)
    pub fn equalsLax(&self, o: &CustomGeometry) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &CustomGeometry) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &CustomGeometry, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof CustomGeometry)
        if true
      {
          let dst = o;
         // Java: CustomGeometry dst = (CustomGeometry)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasRect() || !dst.hasRect()) && (!this.hasRect() || dst.hasRect()))
            else if (self.hasRect() || !dst.hasRect()) && (!self.hasRect() || dst.hasRect())
         {
            // Java: if (this.hasRect() && dst.hasRect())
                if self.hasRect() && dst.hasRect()
            {
               // Java: if (isStrict && !this.rect().equals(dst.rect()))
                    if todo!("KoAr equals/equalsLax")

               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.rect().equalsLax(dst.rect()))
                    if todo!("KoAr equals/equalsLax")

               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasCxnList() || !dst.hasCxnList()) && (!this.hasCxnList() || dst.hasCxnList()))
                if (self.hasCxnList() || !dst.hasCxnList()) && (!self.hasCxnList() || dst.hasCxnList())

            {
               // Java: if (this.hasCxnList() && dst.hasCxnList())
                    if self.hasCxnList() && dst.hasCxnList()
               {
                  // Java: if (isStrict && !this.cxnList().equals(dst.cxnList()))
                        if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.cxnList().equalsLax(dst.cxnList()))
                        if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasAhList() || !dst.hasAhList()) && (!this.hasAhList() || dst.hasAhList()))
                    if (self.hasAhList() || !dst.hasAhList()) && (!self.hasAhList() || dst.hasAhList())

               {
                  // Java: if (this.hasAhList() && dst.hasAhList())
                        if self.hasAhList() && dst.hasAhList()
                  {
                     // Java: if (isStrict && !this.ahList().equals(dst.ahList()))
                            if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.ahList().equalsLax(dst.ahList()))
                            if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasGdLst() || !dst.hasGdLst()) && (!this.hasGdLst() || dst.hasGdLst()))
                        if (self.hasGdLst() || !dst.hasGdLst()) && (!self.hasGdLst() || dst.hasGdLst())

                  {
                     // Java: if (this.hasGdLst() && dst.hasGdLst())
                            if self.hasGdLst() && dst.hasGdLst()
                     {
                        // Java: if (isStrict && !this.gdLst().equals(dst.gdLst()))
                                if todo!("KoAr equals/equalsLax")

                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.gdLst().equalsLax(dst.gdLst()))
                                if todo!("KoAr equals/equalsLax")

                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasAvLst() || !dst.hasAvLst()) && (!this.hasAvLst() || dst.hasAvLst()))
                            if (self.hasAvLst() || !dst.hasAvLst()) && (!self.hasAvLst() || dst.hasAvLst())

                     {
                        // Java: if (this.hasAvLst() && dst.hasAvLst())
                                if self.hasAvLst() && dst.hasAvLst()
                        {
                           // Java: if (isStrict && !this.avLst().equals(dst.avLst()))
                                    if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.avLst().equalsLax(dst.avLst()))
                                    if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: if ((this.hasPathLst() || !dst.hasPathLst()) && (!this.hasPathLst() || dst.hasPathLst()))
                                if (self.hasPathLst() || !dst.hasPathLst()) && (!self.hasPathLst() || dst.hasPathLst())

                        {
                           // Java: if (this.hasPathLst() && dst.hasPathLst())
                                    if self.hasPathLst() && dst.hasPathLst()
                           {
                              // Java: if (isStrict && !this.pathLst().equals(dst.pathLst()))
                                        if todo!("KoAr equals/equalsLax")

                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: if (!isStrict && !this.pathLst().equalsLax(dst.pathLst()))
                                        if todo!("KoAr equals/equalsLax")

                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: return true;
                           return true;
                        }
                        // Java: else
                                else
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: else
                            else
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                        else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
                    else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
                else
            {
               // Java: return false;
               return false;
            }
         }
         // Java: else
            else
         {
            // Java: return false;
            return false;
         }
      }
      // Java: else
        else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasRect())
        if self.hasRect()
      {
         // Java: ret += this.rect().hashCode();
         ret = ret.wrapping_add(todo!("rect hashCode"));
      }
      // Java: if (this.hasCxnList())
        if self.hasCxnList()
      {
         // Java: ret += this.cxnList().hashCode();
         ret = ret.wrapping_add(todo!("cxnList hashCode"));
      }
      // Java: if (this.hasAhList())
        if self.hasAhList()
      {
         // Java: ret += this.ahList().hashCode();
         ret = ret.wrapping_add(todo!("ahList hashCode"));
      }
      // Java: if (this.hasGdLst())
        if self.hasGdLst()
      {
         // Java: ret += this.gdLst().hashCode();
         ret = ret.wrapping_add(todo!("gdLst hashCode"));
      }
      // Java: if (this.hasAvLst())
        if self.hasAvLst()
      {
         // Java: ret += this.avLst().hashCode();
         ret = ret.wrapping_add(todo!("avLst hashCode"));
      }
      // Java: if (this.hasPathLst())
        if self.hasPathLst()
      {
         // Java: ret += this.pathLst().hashCode();
         ret = ret.wrapping_add(todo!("pathLst hashCode"));
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.rect != null && this.rect.dirty())
        if self.rect.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.cxnList != null && this.cxnList.dirty())
        else if self.cxnList.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.ahList != null && this.ahList.dirty())
        else if self.ahList.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.gdLst != null && this.gdLst.dirty())
        else if self.gdLst.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.avLst != null && this.avLst.dirty())
        else if self.avLst.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.pathLst != null && this.pathLst.dirty() ? true : this.map.dirty();
         return self.pathLst.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
      }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone;
      self.saveBin(zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get();
      self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanParent();
      self.cleanParent();
      // Java: this.map.load(KoBin.Null.value1());
      let null_bin = Null::value1();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.clear();
      // Java: this.zone.leave();
      self.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
      // Java: this.rect = null;
      self.rect = None;
      // Java: this.cxnList = null;
      self.cxnList = None;
      // Java: this.ahList = null;
      self.ahList = None;
      // Java: this.gdLst = null;
      self.gdLst = None;
      // Java: this.avLst = null;
      self.avLst = None;
      // Java: this.pathLst = null;
      self.pathLst = None;
    }

    // Java: void follow(CustomGeometry parent)
    pub fn follow(&mut self, parent: Option<Box<CustomGeometry>>) {
      // Java: if (parent != null)
        if let Some(mut p) = parent
      {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(p);
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
        if self.parent.is_some()
      {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: CustomGeometry parent()
    pub fn get_parent(&self) -> Option<&CustomGeometry> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone;
      self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: GeomRect rect()
    pub fn get_rect(&self) -> &dyn KoAr {
      // Java: if (this.rect == null)
        if self.rect.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.rect = (GeomRect)KoBin.Decode.toAny(GeomRect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasRect())
            else if self.parent != null && self.parent.hasRect()
         {
            // Java: this.rect = this.parent.rect();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.rect = (GeomRect)KoBin.Decode.toAny(GeomRect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.rect;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasRect()
    pub fn hasRect(&self) -> bool {
      // Java: return this.map.has(1) || this.rect != null && this.rect.dirty() || this.parent != null && this.parent.hasRect();
      self.map.has(1) || self.rect.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasRect())
    }

    // Java: void deleteRect()
    pub fn deleteRect(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.rect != null)
        if self.rect.is_some()
      {
         // Java: this.rect = null;
         self.rect = None;
      }
    }

    // Java: void writeRect(GeomRect value)
    pub fn writeRect(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(1, bin);
            // Java: if (this.rect != null)
                if self.rect.is_some()
            {
               // Java: this.rect = null;
               self.rect = None;
            }
         }
      }
    }

    // Java: void forceWriteRect(GeomRect value)
    pub fn forceWriteRect(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.rect != null)
            if self.rect.is_some()
         {
            // Java: this.rect = null;
            self.rect = None;
         }
      }
    }

    // Java: ConnectionSiteList cxnList()
    pub fn get_cxnList(&self) -> &dyn KoAr {
      // Java: if (this.cxnList == null)
        if self.cxnList.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.cxnList = (ConnectionSiteList)KoBin.Decode.toAny(ConnectionSiteList.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasCxnList())
            else if self.parent != null && self.parent.hasCxnList()
         {
            // Java: this.cxnList = this.parent.cxnList();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.cxnList = (ConnectionSiteList)KoBin.Decode.toAny(ConnectionSiteList.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.cxnList;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasCxnList()
    pub fn hasCxnList(&self) -> bool {
      // Java: return this.map.has(2) || this.cxnList != null && this.cxnList.dirty() || this.parent != null && this.parent.hasCxnList();
      self.map.has(2) || self.cxnList.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCxnList())
    }

    // Java: void deleteCxnList()
    pub fn deleteCxnList(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.cxnList != null)
        if self.cxnList.is_some()
      {
         // Java: this.cxnList = null;
         self.cxnList = None;
      }
    }

    // Java: void writeCxnList(ConnectionSiteList value)
    pub fn writeCxnList(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(2, bin);
            // Java: if (this.cxnList != null)
                if self.cxnList.is_some()
            {
               // Java: this.cxnList = null;
               self.cxnList = None;
            }
         }
      }
    }

    // Java: void forceWriteCxnList(ConnectionSiteList value)
    pub fn forceWriteCxnList(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.cxnList != null)
            if self.cxnList.is_some()
         {
            // Java: this.cxnList = null;
            self.cxnList = None;
         }
      }
    }

    // Java: AdjustHandleList ahList()
    pub fn get_ahList(&self) -> &dyn KoAr {
      // Java: if (this.ahList == null)
        if self.ahList.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: this.ahList = (AdjustHandleList)KoBin.Decode.toAny(AdjustHandleList.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasAhList())
            else if self.parent != null && self.parent.hasAhList()
         {
            // Java: this.ahList = this.parent.ahList();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.ahList = (AdjustHandleList)KoBin.Decode.toAny(AdjustHandleList.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.ahList;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasAhList()
    pub fn hasAhList(&self) -> bool {
      // Java: return this.map.has(3) || this.ahList != null && this.ahList.dirty() || this.parent != null && this.parent.hasAhList();
      self.map.has(3) || self.ahList.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAhList())
    }

    // Java: void deleteAhList()
    pub fn deleteAhList(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.ahList != null)
        if self.ahList.is_some()
      {
         // Java: this.ahList = null;
         self.ahList = None;
      }
    }

    // Java: void writeAhList(AdjustHandleList value)
    pub fn writeAhList(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(3, bin);
            self.map.put(3, bin);
            // Java: if (this.ahList != null)
                if self.ahList.is_some()
            {
               // Java: this.ahList = null;
               self.ahList = None;
            }
         }
      }
    }

    // Java: void forceWriteAhList(AdjustHandleList value)
    pub fn forceWriteAhList(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.ahList != null)
            if self.ahList.is_some()
         {
            // Java: this.ahList = null;
            self.ahList = None;
         }
      }
    }

    // Java: GeomGuideList gdLst()
    pub fn get_gdLst(&self) -> &dyn KoAr {
      // Java: if (this.gdLst == null)
        if self.gdLst.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: this.gdLst = (GeomGuideList)KoBin.Decode.toAny(GeomGuideList.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasGdLst())
            else if self.parent != null && self.parent.hasGdLst()
         {
            // Java: this.gdLst = this.parent.gdLst();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.gdLst = (GeomGuideList)KoBin.Decode.toAny(GeomGuideList.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.gdLst;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasGdLst()
    pub fn hasGdLst(&self) -> bool {
      // Java: return this.map.has(4) || this.gdLst != null && this.gdLst.dirty() || this.parent != null && this.parent.hasGdLst();
      self.map.has(4) || self.gdLst.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasGdLst())
    }

    // Java: void deleteGdLst()
    pub fn deleteGdLst(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.gdLst != null)
        if self.gdLst.is_some()
      {
         // Java: this.gdLst = null;
         self.gdLst = None;
      }
    }

    // Java: void writeGdLst(GeomGuideList value)
    pub fn writeGdLst(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(4, bin);
            self.map.put(4, bin);
            // Java: if (this.gdLst != null)
                if self.gdLst.is_some()
            {
               // Java: this.gdLst = null;
               self.gdLst = None;
            }
         }
      }
    }

    // Java: void forceWriteGdLst(GeomGuideList value)
    pub fn forceWriteGdLst(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.gdLst != null)
            if self.gdLst.is_some()
         {
            // Java: this.gdLst = null;
            self.gdLst = None;
         }
      }
    }

    // Java: AdjustValueList avLst()
    pub fn get_avLst(&self) -> &dyn KoAr {
      // Java: if (this.avLst == null)
        if self.avLst.is_none()
      {
         // Java: if (this.map.has(5))
            if self.map.has(5)
         {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: this.avLst = (AdjustValueList)KoBin.Decode.toAny(AdjustValueList.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasAvLst())
            else if self.parent != null && self.parent.hasAvLst()
         {
            // Java: this.avLst = this.parent.avLst();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.avLst = (AdjustValueList)KoBin.Decode.toAny(AdjustValueList.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.avLst;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasAvLst()
    pub fn hasAvLst(&self) -> bool {
      // Java: return this.map.has(5) || this.avLst != null && this.avLst.dirty() || this.parent != null && this.parent.hasAvLst();
      self.map.has(5) || self.avLst.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAvLst())
    }

    // Java: void deleteAvLst()
    pub fn deleteAvLst(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
      // Java: if (this.avLst != null)
        if self.avLst.is_some()
      {
         // Java: this.avLst = null;
         self.avLst = None;
      }
    }

    // Java: void writeAvLst(AdjustValueList value)
    pub fn writeAvLst(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(5, bin);
            self.map.put(5, bin);
            // Java: if (this.avLst != null)
                if self.avLst.is_some()
            {
               // Java: this.avLst = null;
               self.avLst = None;
            }
         }
      }
    }

    // Java: void forceWriteAvLst(AdjustValueList value)
    pub fn forceWriteAvLst(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(5, v.lastBin(self.zone));
         // Java: if (this.avLst != null)
            if self.avLst.is_some()
         {
            // Java: this.avLst = null;
            self.avLst = None;
         }
      }
    }

    // Java: PathList pathLst()
    pub fn get_pathLst(&self) -> &dyn KoAr {
      // Java: if (this.pathLst == null)
        if self.pathLst.is_none()
      {
         // Java: if (this.map.has(6))
            if self.map.has(6)
         {
            // Java: KoBin bin = this.map.byKey(6);
            let bin = self.map.byKey(6);
            // Java: this.pathLst = (PathList)KoBin.Decode.toAny(PathList.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasPathLst())
            else if self.parent != null && self.parent.hasPathLst()
         {
            // Java: this.pathLst = this.parent.pathLst();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.pathLst = (PathList)KoBin.Decode.toAny(PathList.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.pathLst;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasPathLst()
    pub fn hasPathLst(&self) -> bool {
      // Java: return this.map.has(6) || this.pathLst != null && this.pathLst.dirty() || this.parent != null && this.parent.hasPathLst();
      self.map.has(6) || self.pathLst.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPathLst())
    }

    // Java: void deletePathLst()
    pub fn deletePathLst(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
      // Java: if (this.pathLst != null)
        if self.pathLst.is_some()
      {
         // Java: this.pathLst = null;
         self.pathLst = None;
      }
    }

    // Java: void writePathLst(PathList value)
    pub fn writePathLst(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(6, bin);
            self.map.put(6, bin);
            // Java: if (this.pathLst != null)
                if self.pathLst.is_some()
            {
               // Java: this.pathLst = null;
               self.pathLst = None;
            }
         }
      }
    }

    // Java: void forceWritePathLst(PathList value)
    pub fn forceWritePathLst(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(6, value.lastBin(this.zone));
         self.map.put(6, v.lastBin(self.zone));
         // Java: if (this.pathLst != null)
            if self.pathLst.is_some()
         {
            // Java: this.pathLst = null;
            self.pathLst = None;
         }
      }
    }

}

// Java: public class GeomRect
impl GeomRect {
    // Java: static protected static <Value1 extends GeomRect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::GeomRect_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static GeomRect create()
    pub fn create() -> Self {
         // Java: return initialize(GeomRect.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static GeomRect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(GeomRect.class, bin);
         Self::initialize(bin)
    }

    // Java: GeomRect()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.map = new KpMap();
         map: KpMap::new(),
         zone: KoZone::defaultZone(), l: None, t: None, r: None, b: None, parent: None,
         }
    }

    // Java: GeomRect(KoBin bin)
    pub fn GeomRect_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.map = new KpMap(bin);
         map: KpMap::KpMap_from(bin),
         zone: KoZone::defaultZone(), l: None, t: None, r: None, b: None, parent: None,
         }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
         // Java: this.cleanCache();
         self.cleanCache();
         // Java: this.cleanFollow();
         self.cleanFollow();
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.map.load(this.box.get());
         let bin_ref = self.bin_box.get();
         self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(GeomRect value)
    pub fn load_value(&mut self, value: Option<&GeomRect>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin_with_zone(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: if (this.l != null && this.l.dirty())
        if self.l.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: this.map.put(1, this.l.saveBin(zone));
            let saved = self.l.as_mut().unwrap().saveBin(zone);
            self.map.put(1, saved);
         }
         // Java: if (this.t != null && this.t.dirty())
        if self.t.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: this.map.put(2, this.t.saveBin(zone));
            let saved = self.t.as_mut().unwrap().saveBin(zone);
            self.map.put(2, saved);
         }
         // Java: if (this.r != null && this.r.dirty())
        if self.r.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: this.map.put(3, this.r.saveBin(zone));
            let saved = self.r.as_mut().unwrap().saveBin(zone);
            self.map.put(3, saved);
         }
         // Java: if (this.b != null && this.b.dirty())
        if self.b.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: this.map.put(4, this.b.saveBin(zone));
            let saved = self.b.as_mut().unwrap().saveBin(zone);
            self.map.put(4, saved);
         }
         // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
         if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(GeomRect o)
    pub fn equalsLax(&self, o: &GeomRect) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &GeomRect) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &GeomRect, isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof GeomRect)
        if true
         {
          let dst = o;
            // Java: GeomRect dst = (GeomRect)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasL() || !dst.hasL()) && (!this.hasL() || dst.hasL()))
            else if (self.hasL() || !dst.hasL()) && (!self.hasL() || dst.hasL())
            {
               // Java: if (this.hasL() && dst.hasL())
                if self.hasL() && dst.hasL()
               {
                  // Java: if (isStrict && !this.l().equals(dst.l()))
                    if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.l().equalsLax(dst.l()))
                    if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasT() || !dst.hasT()) && (!this.hasT() || dst.hasT()))
                if (self.hasT() || !dst.hasT()) && (!self.hasT() || dst.hasT())

               {
                  // Java: if (this.hasT() && dst.hasT())
                    if self.hasT() && dst.hasT()
                  {
                     // Java: if (isStrict && !this.t().equals(dst.t()))
                        if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.t().equalsLax(dst.t()))
                        if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasR() || !dst.hasR()) && (!this.hasR() || dst.hasR()))
                    if (self.hasR() || !dst.hasR()) && (!self.hasR() || dst.hasR())

                  {
                     // Java: if (this.hasR() && dst.hasR())
                        if self.hasR() && dst.hasR()
                     {
                        // Java: if (isStrict && !this.r().equals(dst.r()))
                            if todo!("KoAr equals/equalsLax")

                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.r().equalsLax(dst.r()))
                            if todo!("KoAr equals/equalsLax")

                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasB() || !dst.hasB()) && (!this.hasB() || dst.hasB()))
                        if (self.hasB() || !dst.hasB()) && (!self.hasB() || dst.hasB())

                     {
                        // Java: if (this.hasB() && dst.hasB())
                            if self.hasB() && dst.hasB()
                        {
                           // Java: if (isStrict && !this.b().equals(dst.b()))
                                if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.b().equalsLax(dst.b()))
                                if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: return true;
                        return true;
                     }
                     // Java: else
                        else
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                    else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
                else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
            else
            {
               // Java: return false;
               return false;
            }
         }
         // Java: else
        else
         {
            // Java: return false;
            return false;
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: int ret = 0;
         let mut ret: i32 = 0;
         // Java: if (this.hasL())
        if self.hasL()
         {
            // Java: ret += this.l().hashCode();
            ret = ret.wrapping_add(todo!("l hashCode"));
         }
         // Java: if (this.hasT())
        if self.hasT()
         {
            // Java: ret += this.t().hashCode();
            ret = ret.wrapping_add(todo!("t hashCode"));
         }
         // Java: if (this.hasR())
        if self.hasR()
         {
            // Java: ret += this.r().hashCode();
            ret = ret.wrapping_add(todo!("r hashCode"));
         }
         // Java: if (this.hasB())
        if self.hasB()
         {
            // Java: ret += this.b().hashCode();
            ret = ret.wrapping_add(todo!("b hashCode"));
         }
         // Java: return ret;
         ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: if (this.l != null && this.l.dirty())
        if self.l.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: return true;
            return true;
         }
         // Java: else if (this.t != null && this.t.dirty())
        else if self.t.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: return true;
            return true;
         }
         // Java: else if (this.r != null && this.r.dirty())
        else if self.r.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: return true;
            return true;
         }
         // Java: else
        else
         {
            // Java: return this.b != null && this.b.dirty() ? true : this.map.dirty();
            return self.b.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
         }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.cleanFollow();
         self.cleanFollow();
         // Java: this.cleanCache();
         self.cleanCache();
         // Java: this.cleanParent();
         self.cleanParent();
         // Java: this.map.load(KoBin.Null.value1());
         let null_bin = Null::value1();
         self.map.load(null_bin.as_ref());
         // Java: this.box.clear();
         self.bin_box.clear();
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
         // Java: this.l = null;
         self.l = None;
         // Java: this.t = null;
         self.t = None;
         // Java: this.r = null;
         self.r = None;
         // Java: this.b = null;
         self.b = None;
    }

    // Java: void follow(GeomRect parent)
    pub fn follow(&mut self, parent: Option<Box<GeomRect>>) {
         // Java: if (parent != null)
        if let Some(mut p) = parent
         {
            // Java: parent.discard();
            p.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
            self.parent = Some(p);
         }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
         // Java: if (this.parent != null)
        if self.parent.is_some()
         {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(ref mut p) = self.parent { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
         }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
         // Java: return this.parent != null;
         self.parent.is_some()
    }

    // Java: GeomRect parent()
    pub fn get_parent(&self) -> Option<&GeomRect> {
         // Java: return this.parent;
         self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
         // Java: this.discard(this.zone);
         let zone = self.zone;
         self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
         // Java: this.parent = null;
         self.parent = None;
    }

    // Java: AdjCoordinate l()
    pub fn get_l(&self) -> &dyn KoAr {
         // Java: if (this.l == null)
        if self.l.is_none()
         {
            // Java: if (this.map.has(1))
            if self.map.has(1)
            {
               // Java: KoBin bin = this.map.byKey(1);
               let bin = self.map.byKey(1);
               // Java: this.l = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
            // Java: else if (this.parent != null && this.parent.hasL())
            else if self.parent != null && self.parent.hasL()
            {
               // Java: this.l = this.parent.l();
               todo!("lazy init requires &mut self")
            }
            // Java: else
            else
            {
               // Java: KoBin bin = KoBin.Null.value1();
               let _bin = Null::value1();
               // Java: this.l = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
         }
         // Java: return this.l;
         todo!("lazy init requires &mut self")
    }

    // Java: boolean hasL()
    pub fn hasL(&self) -> bool {
         // Java: return this.map.has(1) || this.l != null && this.l.dirty() || this.parent != null && this.parent.hasL();
         self.map.has(1) || self.l.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasL())
    }

    // Java: void deleteL()
    pub fn deleteL(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(1);
         // Java: if (this.l != null)
        if self.l.is_some()
         {
            // Java: this.l = null;
            self.l = None;
         }
    }

    // Java: void writeL(AdjCoordinate value)
    pub fn writeL(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = v.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(1, bin);
               self.map.put(1, bin);
               // Java: if (this.l != null)
                if self.l.is_some()
               {
                  // Java: this.l = null;
                  self.l = None;
               }
            }
         }
    }

    // Java: void forceWriteL(AdjCoordinate value)
    pub fn forceWriteL(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.map.put(1, value.lastBin(this.zone));
            self.map.put(1, v.lastBin(self.zone));
            // Java: if (this.l != null)
            if self.l.is_some()
            {
               // Java: this.l = null;
               self.l = None;
            }
         }
    }

    // Java: AdjCoordinate t()
    pub fn get_t(&self) -> &dyn KoAr {
         // Java: if (this.t == null)
        if self.t.is_none()
         {
            // Java: if (this.map.has(2))
            if self.map.has(2)
            {
               // Java: KoBin bin = this.map.byKey(2);
               let bin = self.map.byKey(2);
               // Java: this.t = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
            // Java: else if (this.parent != null && this.parent.hasT())
            else if self.parent != null && self.parent.hasT()
            {
               // Java: this.t = this.parent.t();
               todo!("lazy init requires &mut self")
            }
            // Java: else
            else
            {
               // Java: KoBin bin = KoBin.Null.value1();
               let _bin = Null::value1();
               // Java: this.t = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
         }
         // Java: return this.t;
         todo!("lazy init requires &mut self")
    }

    // Java: boolean hasT()
    pub fn hasT(&self) -> bool {
         // Java: return this.map.has(2) || this.t != null && this.t.dirty() || this.parent != null && this.parent.hasT();
         self.map.has(2) || self.t.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasT())
    }

    // Java: void deleteT()
    pub fn deleteT(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(2);
         // Java: if (this.t != null)
        if self.t.is_some()
         {
            // Java: this.t = null;
            self.t = None;
         }
    }

    // Java: void writeT(AdjCoordinate value)
    pub fn writeT(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = v.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(2, bin);
               self.map.put(2, bin);
               // Java: if (this.t != null)
                if self.t.is_some()
               {
                  // Java: this.t = null;
                  self.t = None;
               }
            }
         }
    }

    // Java: void forceWriteT(AdjCoordinate value)
    pub fn forceWriteT(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.map.put(2, value.lastBin(this.zone));
            self.map.put(2, v.lastBin(self.zone));
            // Java: if (this.t != null)
            if self.t.is_some()
            {
               // Java: this.t = null;
               self.t = None;
            }
         }
    }

    // Java: AdjCoordinate r()
    pub fn get_r(&self) -> &dyn KoAr {
         // Java: if (this.r == null)
        if self.r.is_none()
         {
            // Java: if (this.map.has(3))
            if self.map.has(3)
            {
               // Java: KoBin bin = this.map.byKey(3);
               let bin = self.map.byKey(3);
               // Java: this.r = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
            // Java: else if (this.parent != null && this.parent.hasR())
            else if self.parent != null && self.parent.hasR()
            {
               // Java: this.r = this.parent.r();
               todo!("lazy init requires &mut self")
            }
            // Java: else
            else
            {
               // Java: KoBin bin = KoBin.Null.value1();
               let _bin = Null::value1();
               // Java: this.r = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
         }
         // Java: return this.r;
         todo!("lazy init requires &mut self")
    }

    // Java: boolean hasR()
    pub fn hasR(&self) -> bool {
         // Java: return this.map.has(3) || this.r != null && this.r.dirty() || this.parent != null && this.parent.hasR();
         self.map.has(3) || self.r.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasR())
    }

    // Java: void deleteR()
    pub fn deleteR(&mut self) {
         // Java: this.map.put(3, (KoBin)null);
         self.map.clear_key(3);
         // Java: if (this.r != null)
        if self.r.is_some()
         {
            // Java: this.r = null;
            self.r = None;
         }
    }

    // Java: void writeR(AdjCoordinate value)
    pub fn writeR(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = v.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(3, bin);
               self.map.put(3, bin);
               // Java: if (this.r != null)
                if self.r.is_some()
               {
                  // Java: this.r = null;
                  self.r = None;
               }
            }
         }
    }

    // Java: void forceWriteR(AdjCoordinate value)
    pub fn forceWriteR(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.map.put(3, value.lastBin(this.zone));
            self.map.put(3, v.lastBin(self.zone));
            // Java: if (this.r != null)
            if self.r.is_some()
            {
               // Java: this.r = null;
               self.r = None;
            }
         }
    }

    // Java: AdjCoordinate b()
    pub fn get_b(&self) -> &dyn KoAr {
         // Java: if (this.b == null)
        if self.b.is_none()
         {
            // Java: if (this.map.has(4))
            if self.map.has(4)
            {
               // Java: KoBin bin = this.map.byKey(4);
               let bin = self.map.byKey(4);
               // Java: this.b = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
            // Java: else if (this.parent != null && this.parent.hasB())
            else if self.parent != null && self.parent.hasB()
            {
               // Java: this.b = this.parent.b();
               todo!("lazy init requires &mut self")
            }
            // Java: else
            else
            {
               // Java: KoBin bin = KoBin.Null.value1();
               let _bin = Null::value1();
               // Java: this.b = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
         }
         // Java: return this.b;
         todo!("lazy init requires &mut self")
    }

    // Java: boolean hasB()
    pub fn hasB(&self) -> bool {
         // Java: return this.map.has(4) || this.b != null && this.b.dirty() || this.parent != null && this.parent.hasB();
         self.map.has(4) || self.b.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasB())
    }

    // Java: void deleteB()
    pub fn deleteB(&mut self) {
         // Java: this.map.put(4, (KoBin)null);
         self.map.clear_key(4);
         // Java: if (this.b != null)
        if self.b.is_some()
         {
            // Java: this.b = null;
            self.b = None;
         }
    }

    // Java: void writeB(AdjCoordinate value)
    pub fn writeB(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = v.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(4, bin);
               self.map.put(4, bin);
               // Java: if (this.b != null)
                if self.b.is_some()
               {
                  // Java: this.b = null;
                  self.b = None;
               }
            }
         }
    }

    // Java: void forceWriteB(AdjCoordinate value)
    pub fn forceWriteB(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.map.put(4, value.lastBin(this.zone));
            self.map.put(4, v.lastBin(self.zone));
            // Java: if (this.b != null)
            if self.b.is_some()
            {
               // Java: this.b = null;
               self.b = None;
            }
         }
    }

}

// Java: public class ConnectionSite
impl ConnectionSite {
    // Java: static protected static <Value1 extends ConnectionSite> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::ConnectionSite_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static ConnectionSite create()
    pub fn create() -> Self {
         // Java: return initialize(ConnectionSite.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static ConnectionSite open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(ConnectionSite.class, bin);
         Self::initialize(bin)
    }

    // Java: ConnectionSite()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.map = new KpMap();
         map: KpMap::new(),
         zone: KoZone::defaultZone(), pos: None, ang: None, parent: None,
         }
    }

    // Java: ConnectionSite(KoBin bin)
    pub fn ConnectionSite_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.map = new KpMap(bin);
         map: KpMap::KpMap_from(bin),
         zone: KoZone::defaultZone(), pos: None, ang: None, parent: None,
         }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
         // Java: this.cleanCache();
         self.cleanCache();
         // Java: this.cleanFollow();
         self.cleanFollow();
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.map.load(this.box.get());
         let bin_ref = self.bin_box.get();
         self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(ConnectionSite value)
    pub fn load_value(&mut self, value: Option<&ConnectionSite>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin_with_zone(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: if (this.pos != null && this.pos.dirty())
        if self.pos.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: this.map.put(1, this.pos.saveBin(zone));
            let saved = self.pos.as_mut().unwrap().saveBin(zone);
            self.map.put(1, saved);
         }
         // Java: if (this.ang != null && this.ang.dirty())
        if self.ang.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: this.map.put(2, this.ang.saveBin(zone));
            let saved = self.ang.as_mut().unwrap().saveBin(zone);
            self.map.put(2, saved);
         }
         // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
         if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(ConnectionSite o)
    pub fn equalsLax(&self, o: &ConnectionSite) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ConnectionSite) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ConnectionSite, isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof ConnectionSite)
        if true
         {
          let dst = o;
            // Java: ConnectionSite dst = (ConnectionSite)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasPos() || !dst.hasPos()) && (!this.hasPos() || dst.hasPos()))
            else if (self.hasPos() || !dst.hasPos()) && (!self.hasPos() || dst.hasPos())
            {
               // Java: if (this.hasPos() && dst.hasPos())
                if self.hasPos() && dst.hasPos()
               {
                  // Java: if (isStrict && !this.pos().equals(dst.pos()))
                    if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.pos().equalsLax(dst.pos()))
                    if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasAng() || !dst.hasAng()) && (!this.hasAng() || dst.hasAng()))
                if (self.hasAng() || !dst.hasAng()) && (!self.hasAng() || dst.hasAng())

               {
                  // Java: if (this.hasAng() && dst.hasAng())
                    if self.hasAng() && dst.hasAng()
                  {
                     // Java: if (isStrict && !this.ang().equals(dst.ang()))
                        if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.ang().equalsLax(dst.ang()))
                        if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: return true;
                  return true;
               }
               // Java: else
                else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
            else
            {
               // Java: return false;
               return false;
            }
         }
         // Java: else
        else
         {
            // Java: return false;
            return false;
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: int ret = 0;
         let mut ret: i32 = 0;
         // Java: if (this.hasPos())
        if self.hasPos()
         {
            // Java: ret += this.pos().hashCode();
            ret = ret.wrapping_add(todo!("pos hashCode"));
         }
         // Java: if (this.hasAng())
        if self.hasAng()
         {
            // Java: ret += this.ang().hashCode();
            ret = ret.wrapping_add(todo!("ang hashCode"));
         }
         // Java: return ret;
         ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: if (this.pos != null && this.pos.dirty())
        if self.pos.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: return true;
            return true;
         }
         // Java: else
        else
         {
            // Java: return this.ang != null && this.ang.dirty() ? true : this.map.dirty();
            return self.ang.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
         }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.cleanFollow();
         self.cleanFollow();
         // Java: this.cleanCache();
         self.cleanCache();
         // Java: this.cleanParent();
         self.cleanParent();
         // Java: this.map.load(KoBin.Null.value1());
         let null_bin = Null::value1();
         self.map.load(null_bin.as_ref());
         // Java: this.box.clear();
         self.bin_box.clear();
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
         // Java: this.pos = null;
         self.pos = None;
         // Java: this.ang = null;
         self.ang = None;
    }

    // Java: void follow(ConnectionSite parent)
    pub fn follow(&mut self, parent: Option<Box<ConnectionSite>>) {
         // Java: if (parent != null)
        if let Some(mut p) = parent
         {
            // Java: parent.discard();
            p.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
            self.parent = Some(p);
         }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
         // Java: if (this.parent != null)
        if self.parent.is_some()
         {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(ref mut p) = self.parent { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
         }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
         // Java: return this.parent != null;
         self.parent.is_some()
    }

    // Java: ConnectionSite parent()
    pub fn get_parent(&self) -> Option<&ConnectionSite> {
         // Java: return this.parent;
         self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
         // Java: this.discard(this.zone);
         let zone = self.zone;
         self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
         // Java: this.parent = null;
         self.parent = None;
    }

    // Java: AdjPoint2D pos()
    pub fn get_pos(&self) -> &dyn KoAr {
         // Java: if (this.pos == null)
        if self.pos.is_none()
         {
            // Java: if (this.map.has(1))
            if self.map.has(1)
            {
               // Java: KoBin bin = this.map.byKey(1);
               let bin = self.map.byKey(1);
               // Java: this.pos = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
            // Java: else if (this.parent != null && this.parent.hasPos())
            else if self.parent != null && self.parent.hasPos()
            {
               // Java: this.pos = this.parent.pos();
               todo!("lazy init requires &mut self")
            }
            // Java: else
            else
            {
               // Java: KoBin bin = KoBin.Null.value1();
               let _bin = Null::value1();
               // Java: this.pos = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
         }
         // Java: return this.pos;
         todo!("lazy init requires &mut self")
    }

    // Java: boolean hasPos()
    pub fn hasPos(&self) -> bool {
         // Java: return this.map.has(1) || this.pos != null && this.pos.dirty() || this.parent != null && this.parent.hasPos();
         self.map.has(1) || self.pos.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPos())
    }

    // Java: void deletePos()
    pub fn deletePos(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(1);
         // Java: if (this.pos != null)
        if self.pos.is_some()
         {
            // Java: this.pos = null;
            self.pos = None;
         }
    }

    // Java: void writePos(AdjPoint2D value)
    pub fn writePos(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = v.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(1, bin);
               self.map.put(1, bin);
               // Java: if (this.pos != null)
                if self.pos.is_some()
               {
                  // Java: this.pos = null;
                  self.pos = None;
               }
            }
         }
    }

    // Java: void forceWritePos(AdjPoint2D value)
    pub fn forceWritePos(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.map.put(1, value.lastBin(this.zone));
            self.map.put(1, v.lastBin(self.zone));
            // Java: if (this.pos != null)
            if self.pos.is_some()
            {
               // Java: this.pos = null;
               self.pos = None;
            }
         }
    }

    // Java: AdjAngle ang()
    pub fn get_ang(&self) -> &dyn KoAr {
         // Java: if (this.ang == null)
        if self.ang.is_none()
         {
            // Java: if (this.map.has(2))
            if self.map.has(2)
            {
               // Java: KoBin bin = this.map.byKey(2);
               let bin = self.map.byKey(2);
               // Java: this.ang = (AdjAngle)KoBin.Decode.toAny(AdjAngle.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
            // Java: else if (this.parent != null && this.parent.hasAng())
            else if self.parent != null && self.parent.hasAng()
            {
               // Java: this.ang = this.parent.ang();
               todo!("lazy init requires &mut self")
            }
            // Java: else
            else
            {
               // Java: KoBin bin = KoBin.Null.value1();
               let _bin = Null::value1();
               // Java: this.ang = (AdjAngle)KoBin.Decode.toAny(AdjAngle.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
         }
         // Java: return this.ang;
         todo!("lazy init requires &mut self")
    }

    // Java: boolean hasAng()
    pub fn hasAng(&self) -> bool {
         // Java: return this.map.has(2) || this.ang != null && this.ang.dirty() || this.parent != null && this.parent.hasAng();
         self.map.has(2) || self.ang.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAng())
    }

    // Java: void deleteAng()
    pub fn deleteAng(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(2);
         // Java: if (this.ang != null)
        if self.ang.is_some()
         {
            // Java: this.ang = null;
            self.ang = None;
         }
    }

    // Java: void writeAng(AdjAngle value)
    pub fn writeAng(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = v.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(2, bin);
               self.map.put(2, bin);
               // Java: if (this.ang != null)
                if self.ang.is_some()
               {
                  // Java: this.ang = null;
                  self.ang = None;
               }
            }
         }
    }

    // Java: void forceWriteAng(AdjAngle value)
    pub fn forceWriteAng(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.map.put(2, value.lastBin(this.zone));
            self.map.put(2, v.lastBin(self.zone));
            // Java: if (this.ang != null)
            if self.ang.is_some()
            {
               // Java: this.ang = null;
               self.ang = None;
            }
         }
    }

}

// Java: public class ConnectionSiteList
impl ConnectionSiteList {
    // Java: ConnectionSiteList()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.list = new KpList(this.zone);
         list: KpList::new(KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: ConnectionSiteList(KoBin bin)
    pub fn ConnectionSiteList_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.list = new KpList(bin, this.zone);
         list: KpList::KpList_from(bin, KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get();
         self.list.load(bin_ref.as_ref(), _zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(ConnectionSiteList value)
    pub fn load_value(&mut self, value: Option<&ConnectionSiteList>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin_with_zone(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() {
            let saved = self.list.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(ConnectionSiteList o)
    pub fn equalsLax(&self, o: &ConnectionSiteList) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ConnectionSiteList) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ConnectionSiteList, isStrict: bool) -> bool {
         // Java: if (null == o)
        if true
         {
          let dst = o;
            // Java: return false;
            return false;
         }
         // Java: else
        else
         {
            // Java: ConnectionSiteList dst = (ConnectionSiteList)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            else
            {
               // Java: ArrayList<ConnectionSite> dstList = new ArrayList();
               todo!("list toList")
               // Java: dst.read(dstList);
               todo!("dst.read(dstList);")
               // Java: ArrayList<ConnectionSite> srcList = new ArrayList();
               todo!("list toList")
               // Java: this.read(srcList);
               todo!("this.read(srcList);")
               // Java: if (dstList.size() != srcList.size())
                todo!("if (dstList.size() != srcList.size())")
               {
                  // Java: return false;
                  return false;
               }
               // Java: else
                else
               {
                  // Java: int size = dstList.size();
                  todo!("int size = dstList.size();")
                  // Java: for(int i = 0; i < size; ++i)
                    todo!("for(int i = 0; i < size; ++i)")
                  {
                     // Java: ConnectionSite dstClrTrans = (ConnectionSite)dstList.get(i);
                     todo!("ConnectionSite dstClrTrans = (ConnectionSite)dstList.get(i);")
                     // Java: ConnectionSite srcClrTrans = (ConnectionSite)srcList.get(i);
                     todo!("ConnectionSite srcClrTrans = (ConnectionSite)srcList.get(i);")
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                        if isStrict && !dstClrTrans.equals(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                        if !isStrict && !dstClrTrans.equalsLax(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: return true;
                  return true;
               }
            }
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         todo!("list hashCode")
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::value1();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends ConnectionSiteList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::ConnectionSiteList_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static ConnectionSiteList create()
    pub fn create() -> Self {
         // Java: return initialize(ConnectionSiteList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static ConnectionSiteList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(ConnectionSiteList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<ConnectionSite> valueLst)
    pub fn read(&self, Collection<ConnectionSite> valueLst) {
         // Java: valueLst.clear();
         todo!("valueLst.clear();")
         // Java: int size = this.list.size();
         todo!("int size = this.list.size();")
         // Java: for(int i = 0; i < size; ++i)
        todo!("for(int i = 0; i < size; ++i)")
         {
            // Java: valueLst.add((ConnectionSite)KoBin.Decode.toAny(ConnectionSite.class, this.list.get(i), this.zone));
            todo!("lazy init requires &mut self")
         }
    }

    // Java: void write(Iterable<ConnectionSite> valueLst)
    pub fn write(&self, Iterable<ConnectionSite> valueLst) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(ConnectionSite i : valueLst)
        todo!("for(ConnectionSite i : valueLst)")
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            todo!("KoBin bin = i.lastBin(this.zone);")
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.list.add(bin);
               todo!("list add")
            }
         }
    }

    // Java: void forceWrite(Iterable<ConnectionSite> valueLst)
    pub fn forceWrite(&self, Iterable<ConnectionSite> valueLst) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(ConnectionSite i : valueLst)
        todo!("for(ConnectionSite i : valueLst)")
         {
            // Java: this.list.add(i.lastBin(this.zone));
            todo!("list add")
         }
    }

}

// Java: public class AdjustHandle
impl AdjustHandle {
    // Java: static protected static <Value1 extends AdjustHandle> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::AdjustHandle_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static AdjustHandle create()
    pub fn create() -> Self {
         // Java: return initialize(AdjustHandle.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static AdjustHandle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(AdjustHandle.class, bin);
         Self::initialize(bin)
    }

    // Java: AdjustHandle()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.map = new KpMap();
         map: KpMap::new(),
         zone: KoZone::defaultZone(), ahXY: None, ahPolar: None, parent: None,
         }
    }

    // Java: AdjustHandle(KoBin bin)
    pub fn AdjustHandle_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.map = new KpMap(bin);
         map: KpMap::KpMap_from(bin),
         zone: KoZone::defaultZone(), ahXY: None, ahPolar: None, parent: None,
         }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
         // Java: this.cleanCache();
         self.cleanCache();
         // Java: this.cleanFollow();
         self.cleanFollow();
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.map.load(this.box.get());
         let bin_ref = self.bin_box.get();
         self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(AdjustHandle value)
    pub fn load_value(&mut self, value: Option<&AdjustHandle>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin_with_zone(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: if (this.ahXY != null && this.ahXY.dirty())
        if self.ahXY.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: this.map.put(1, this.ahXY.saveBin(zone));
            let saved = self.ahXY.as_mut().unwrap().saveBin(zone);
            self.map.put(1, saved);
         }
         // Java: if (this.ahPolar != null && this.ahPolar.dirty())
        if self.ahPolar.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: this.map.put(2, this.ahPolar.saveBin(zone));
            let saved = self.ahPolar.as_mut().unwrap().saveBin(zone);
            self.map.put(2, saved);
         }
         // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
         if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(AdjustHandle o)
    pub fn equalsLax(&self, o: &AdjustHandle) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AdjustHandle) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &AdjustHandle, isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof AdjustHandle)
        if true
         {
          let dst = o;
            // Java: AdjustHandle dst = (AdjustHandle)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasAhXY() || !dst.hasAhXY()) && (!this.hasAhXY() || dst.hasAhXY()))
            else if (self.hasAhXY() || !dst.hasAhXY()) && (!self.hasAhXY() || dst.hasAhXY())
            {
               // Java: if (this.hasAhXY() && dst.hasAhXY())
                if self.hasAhXY() && dst.hasAhXY()
               {
                  // Java: if (isStrict && !this.ahXY().equals(dst.ahXY()))
                    if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.ahXY().equalsLax(dst.ahXY()))
                    if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasAhPolar() || !dst.hasAhPolar()) && (!this.hasAhPolar() || dst.hasAhPolar()))
                if (self.hasAhPolar() || !dst.hasAhPolar()) && (!self.hasAhPolar() || dst.hasAhPolar())

               {
                  // Java: if (this.hasAhPolar() && dst.hasAhPolar())
                    if self.hasAhPolar() && dst.hasAhPolar()
                  {
                     // Java: if (isStrict && !this.ahPolar().equals(dst.ahPolar()))
                        if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.ahPolar().equalsLax(dst.ahPolar()))
                        if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: return true;
                  return true;
               }
               // Java: else
                else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
            else
            {
               // Java: return false;
               return false;
            }
         }
         // Java: else
        else
         {
            // Java: return false;
            return false;
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: int ret = 0;
         let mut ret: i32 = 0;
         // Java: if (this.hasAhXY())
        if self.hasAhXY()
         {
            // Java: ret += this.ahXY().hashCode();
            ret = ret.wrapping_add(todo!("ahXY hashCode"));
         }
         // Java: if (this.hasAhPolar())
        if self.hasAhPolar()
         {
            // Java: ret += this.ahPolar().hashCode();
            ret = ret.wrapping_add(todo!("ahPolar hashCode"));
         }
         // Java: return ret;
         ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: if (this.ahXY != null && this.ahXY.dirty())
        if self.ahXY.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: return true;
            return true;
         }
         // Java: else
        else
         {
            // Java: return this.ahPolar != null && this.ahPolar.dirty() ? true : this.map.dirty();
            return self.ahPolar.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
         }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.cleanFollow();
         self.cleanFollow();
         // Java: this.cleanCache();
         self.cleanCache();
         // Java: this.cleanParent();
         self.cleanParent();
         // Java: this.map.load(KoBin.Null.value1());
         let null_bin = Null::value1();
         self.map.load(null_bin.as_ref());
         // Java: this.box.clear();
         self.bin_box.clear();
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
         // Java: this.ahXY = null;
         self.ahXY = None;
         // Java: this.ahPolar = null;
         self.ahPolar = None;
    }

    // Java: void follow(AdjustHandle parent)
    pub fn follow(&mut self, parent: Option<Box<AdjustHandle>>) {
         // Java: if (parent != null)
        if let Some(mut p) = parent
         {
            // Java: parent.discard();
            p.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
            self.parent = Some(p);
         }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
         // Java: if (this.parent != null)
        if self.parent.is_some()
         {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(ref mut p) = self.parent { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
         }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
         // Java: return this.parent != null;
         self.parent.is_some()
    }

    // Java: AdjustHandle parent()
    pub fn get_parent(&self) -> Option<&AdjustHandle> {
         // Java: return this.parent;
         self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
         // Java: this.discard(this.zone);
         let zone = self.zone;
         self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
         // Java: this.parent = null;
         self.parent = None;
    }

    // Java: XY ahXY()
    pub fn get_ahXY(&self) -> &dyn KoAr {
         // Java: if (this.ahXY == null)
        if self.ahXY.is_none()
         {
            // Java: if (this.map.has(1))
            if self.map.has(1)
            {
               // Java: KoBin bin = this.map.byKey(1);
               let bin = self.map.byKey(1);
               // Java: this.ahXY = (XY)KoBin.Decode.toAny(XY.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
            // Java: else if (this.parent != null && this.parent.hasAhXY())
            else if self.parent != null && self.parent.hasAhXY()
            {
               // Java: this.ahXY = this.parent.ahXY();
               todo!("lazy init requires &mut self")
            }
            // Java: else
            else
            {
               // Java: KoBin bin = KoBin.Null.value1();
               let _bin = Null::value1();
               // Java: this.ahXY = (XY)KoBin.Decode.toAny(XY.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
         }
         // Java: return this.ahXY;
         todo!("lazy init requires &mut self")
    }

    // Java: boolean hasAhXY()
    pub fn hasAhXY(&self) -> bool {
         // Java: return this.map.has(1) || this.ahXY != null && this.ahXY.dirty() || this.parent != null && this.parent.hasAhXY();
         self.map.has(1) || self.ahXY.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAhXY())
    }

    // Java: void deleteAhXY()
    pub fn deleteAhXY(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(1);
         // Java: if (this.ahXY != null)
        if self.ahXY.is_some()
         {
            // Java: this.ahXY = null;
            self.ahXY = None;
         }
    }

    // Java: void writeAhXY(XY value)
    pub fn writeAhXY(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = v.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(1, bin);
               self.map.put(1, bin);
               // Java: if (this.ahXY != null)
                if self.ahXY.is_some()
               {
                  // Java: this.ahXY = null;
                  self.ahXY = None;
               }
            }
         }
    }

    // Java: void forceWriteAhXY(XY value)
    pub fn forceWriteAhXY(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.map.put(1, value.lastBin(this.zone));
            self.map.put(1, v.lastBin(self.zone));
            // Java: if (this.ahXY != null)
            if self.ahXY.is_some()
            {
               // Java: this.ahXY = null;
               self.ahXY = None;
            }
         }
    }

    // Java: Polar ahPolar()
    pub fn get_ahPolar(&self) -> &dyn KoAr {
         // Java: if (this.ahPolar == null)
        if self.ahPolar.is_none()
         {
            // Java: if (this.map.has(2))
            if self.map.has(2)
            {
               // Java: KoBin bin = this.map.byKey(2);
               let bin = self.map.byKey(2);
               // Java: this.ahPolar = (Polar)KoBin.Decode.toAny(Polar.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
            // Java: else if (this.parent != null && this.parent.hasAhPolar())
            else if self.parent != null && self.parent.hasAhPolar()
            {
               // Java: this.ahPolar = this.parent.ahPolar();
               todo!("lazy init requires &mut self")
            }
            // Java: else
            else
            {
               // Java: KoBin bin = KoBin.Null.value1();
               let _bin = Null::value1();
               // Java: this.ahPolar = (Polar)KoBin.Decode.toAny(Polar.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
         }
         // Java: return this.ahPolar;
         todo!("lazy init requires &mut self")
    }

    // Java: boolean hasAhPolar()
    pub fn hasAhPolar(&self) -> bool {
         // Java: return this.map.has(2) || this.ahPolar != null && this.ahPolar.dirty() || this.parent != null && this.parent.hasAhPolar();
         self.map.has(2) || self.ahPolar.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAhPolar())
    }

    // Java: void deleteAhPolar()
    pub fn deleteAhPolar(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(2);
         // Java: if (this.ahPolar != null)
        if self.ahPolar.is_some()
         {
            // Java: this.ahPolar = null;
            self.ahPolar = None;
         }
    }

    // Java: void writeAhPolar(Polar value)
    pub fn writeAhPolar(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = v.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(2, bin);
               self.map.put(2, bin);
               // Java: if (this.ahPolar != null)
                if self.ahPolar.is_some()
               {
                  // Java: this.ahPolar = null;
                  self.ahPolar = None;
               }
            }
         }
    }

    // Java: void forceWriteAhPolar(Polar value)
    pub fn forceWriteAhPolar(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.map.put(2, value.lastBin(this.zone));
            self.map.put(2, v.lastBin(self.zone));
            // Java: if (this.ahPolar != null)
            if self.ahPolar.is_some()
            {
               // Java: this.ahPolar = null;
               self.ahPolar = None;
            }
         }
    }

}

// Java: public class XY
impl XY {
    // Java: static protected static <Value1 extends XY> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
            // Java: KoZone zone = KoZone.enter();
            let zone = KoZone::enter();
            // Java: Value1 inst = (Value1)(zone.allocate(clazz));
            let mut inst = Self::XY_from(bin);
            // Java: inst.zone = zone;
            inst.zone = zone;
            // Java: inst.load(bin);
            inst.load_bin(bin);
            // Java: return inst;
            inst
    }

    // Java: static XY create()
    pub fn create() -> Self {
            // Java: return initialize(XY.class, KoBin.Null.empty());
            let empty = Null::empty();
            Self::initialize(empty.as_ref())
    }

    // Java: static XY open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
            // Java: return initialize(XY.class, bin);
            Self::initialize(bin)
    }

    // Java: XY()
    pub fn new() -> Self {
      Self {
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpMap();
            map: KpMap::new(),
            zone: KoZone::defaultZone(), maxX: None, maxY: None, minX: None, minY: None, position: None, parent: None,
            }
    }

    // Java: XY(KoBin bin)
    pub fn XY_from(bin: &dyn KoBinTrait) -> Self {
      Self {
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpMap(bin);
            map: KpMap::KpMap_from(bin),
            zone: KoZone::defaultZone(), maxX: None, maxY: None, minX: None, minY: None, position: None, parent: None,
            }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.box.set(bin);
            self.bin_box.set(Some(bin));
            // Java: this.map.load(this.box.get());
            let bin_ref = self.bin_box.get();
            self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
            // Java: this.loadBin(bin, this.zone);
            let zone = self.zone;
            self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(XY value)
    pub fn load_value(&mut self, value: Option<&XY>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.loadBin(value.lastBin(this.zone), this.zone);
               let bin = v.lastBin(self.zone);
               self.loadBin_with_zone(bin.as_ref(), self.zone);
            }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: if (this.maxX != null && this.maxX.dirty())
        if self.maxX.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(3, this.maxX.saveBin(zone));
               let saved = self.maxX.as_mut().unwrap().saveBin(zone);
               self.map.put(3, saved);
            }
            // Java: if (this.maxY != null && this.maxY.dirty())
        if self.maxY.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(4, this.maxY.saveBin(zone));
               let saved = self.maxY.as_mut().unwrap().saveBin(zone);
               self.map.put(4, saved);
            }
            // Java: if (this.minX != null && this.minX.dirty())
        if self.minX.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(5, this.minX.saveBin(zone));
               let saved = self.minX.as_mut().unwrap().saveBin(zone);
               self.map.put(5, saved);
            }
            // Java: if (this.minY != null && this.minY.dirty())
        if self.minY.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(6, this.minY.saveBin(zone));
               let saved = self.minY.as_mut().unwrap().saveBin(zone);
               self.map.put(6, saved);
            }
            // Java: if (this.position != null && this.position.dirty())
        if self.position.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(7, this.position.saveBin(zone));
               let saved = self.position.as_mut().unwrap().saveBin(zone);
               self.map.put(7, saved);
            }
            // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
            if self.map.dirty() {
               let saved = self.map.save(zone);
               self.bin_box.set(Some(saved.as_ref()));
               self.bin_box.get()
            } else {
               self.bin_box.get()
            }
    }

    // Java: boolean equalsLax(XY o)
    pub fn equalsLax(&self, o: &XY) -> bool {
            // Java: return this.equals(o, false);
            self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &XY) -> bool {
            // Java: return this.equals(o, true);
            self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &XY, isStrict: bool) -> bool {
            // Java: if (null != o && o instanceof XY)
        if true
            {
          let dst = o;
               // Java: XY dst = (XY)o;
               // Java: if (this == dst)
            if std::ptr::eq(self, dst)
               {
                  // Java: return true;
                  return true;
               }
               // Java: else if ((this.hasGdRefX() || !dst.hasGdRefX()) && (!this.hasGdRefX() || dst.hasGdRefX()))
            else if (self.hasGdRefX() || !dst.hasGdRefX()) && (!self.hasGdRefX() || dst.hasGdRefX())
               {
                  // Java: if (this.hasGdRefX() && dst.hasGdRefX() && !this.gdRefX().equals(dst.gdRefX()))
                if self.hasGdRefX() && dst.hasGdRefX() && !self.gdRefX().equals(dst.gdRefX())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasGdRefY() || !dst.hasGdRefY()) && (!this.hasGdRefY() || dst.hasGdRefY()))
                else if (self.hasGdRefY() || !dst.hasGdRefY()) && (!self.hasGdRefY() || dst.hasGdRefY())
                  {
                     // Java: if (this.hasGdRefY() && dst.hasGdRefY() && !this.gdRefY().equals(dst.gdRefY()))
                    if self.hasGdRefY() && dst.hasGdRefY() && !self.gdRefY().equals(dst.gdRefY())
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasMaxX() || !dst.hasMaxX()) && (!this.hasMaxX() || dst.hasMaxX()))
                    else if (self.hasMaxX() || !dst.hasMaxX()) && (!self.hasMaxX() || dst.hasMaxX())
                     {
                        // Java: if (this.hasMaxX() && dst.hasMaxX())
                        if self.hasMaxX() && dst.hasMaxX()
                        {
                           // Java: if (isStrict && !this.maxX().equals(dst.maxX()))
                            if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.maxX().equalsLax(dst.maxX()))
                            if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: if ((this.hasMaxY() || !dst.hasMaxY()) && (!this.hasMaxY() || dst.hasMaxY()))
                        if (self.hasMaxY() || !dst.hasMaxY()) && (!self.hasMaxY() || dst.hasMaxY())

                        {
                           // Java: if (this.hasMaxY() && dst.hasMaxY())
                            if self.hasMaxY() && dst.hasMaxY()
                           {
                              // Java: if (isStrict && !this.maxY().equals(dst.maxY()))
                                if todo!("KoAr equals/equalsLax")

                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: if (!isStrict && !this.maxY().equalsLax(dst.maxY()))
                                if todo!("KoAr equals/equalsLax")

                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: if ((this.hasMinX() || !dst.hasMinX()) && (!this.hasMinX() || dst.hasMinX()))
                            if (self.hasMinX() || !dst.hasMinX()) && (!self.hasMinX() || dst.hasMinX())

                           {
                              // Java: if (this.hasMinX() && dst.hasMinX())
                                if self.hasMinX() && dst.hasMinX()
                              {
                                 // Java: if (isStrict && !this.minX().equals(dst.minX()))
                                    if todo!("KoAr equals/equalsLax")

                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: if (!isStrict && !this.minX().equalsLax(dst.minX()))
                                    if todo!("KoAr equals/equalsLax")

                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: if ((this.hasMinY() || !dst.hasMinY()) && (!this.hasMinY() || dst.hasMinY()))
                                if (self.hasMinY() || !dst.hasMinY()) && (!self.hasMinY() || dst.hasMinY())

                              {
                                 // Java: if (this.hasMinY() && dst.hasMinY())
                                    if self.hasMinY() && dst.hasMinY()
                                 {
                                    // Java: if (isStrict && !this.minY().equals(dst.minY()))
                                        if todo!("KoAr equals/equalsLax")

                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: if (!isStrict && !this.minY().equalsLax(dst.minY()))
                                        if todo!("KoAr equals/equalsLax")

                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                 }
                                 // Java: if ((this.hasPosition() || !dst.hasPosition()) && (!this.hasPosition() || dst.hasPosition()))
                                    if (self.hasPosition() || !dst.hasPosition()) && (!self.hasPosition() || dst.hasPosition())

                                 {
                                    // Java: if (this.hasPosition() && dst.hasPosition())
                                        if self.hasPosition() && dst.hasPosition()
                                    {
                                       // Java: if (isStrict && !this.position().equals(dst.position()))
                                            if todo!("KoAr equals/equalsLax")

                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: if (!isStrict && !this.position().equalsLax(dst.position()))
                                            if todo!("KoAr equals/equalsLax")

                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                    }
                                    // Java: return true;
                                    return true;
                                 }
                                 // Java: else
                                    else
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: else
                                else
                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: else
                            else
                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: else
                        else
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: else
                    else
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
            else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
        else
            {
               // Java: return false;
               return false;
            }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
            // Java: int ret = 0;
            let mut ret: i32 = 0;
            // Java: if (this.hasGdRefX())
        if self.hasGdRefX()
            {
               // Java: ret += String.valueOf(this.gdRefX()).hashCode();
               ret = ret.wrapping_add(self.gdRefX().len() as i32);
            }
            // Java: if (this.hasGdRefY())
        if self.hasGdRefY()
            {
               // Java: ret += String.valueOf(this.gdRefY()).hashCode();
               ret = ret.wrapping_add(self.gdRefY().len() as i32);
            }
            // Java: if (this.hasMaxX())
        if self.hasMaxX()
            {
               // Java: ret += this.maxX().hashCode();
               ret = ret.wrapping_add(todo!("maxX hashCode"));
            }
            // Java: if (this.hasMaxY())
        if self.hasMaxY()
            {
               // Java: ret += this.maxY().hashCode();
               ret = ret.wrapping_add(todo!("maxY hashCode"));
            }
            // Java: if (this.hasMinX())
        if self.hasMinX()
            {
               // Java: ret += this.minX().hashCode();
               ret = ret.wrapping_add(todo!("minX hashCode"));
            }
            // Java: if (this.hasMinY())
        if self.hasMinY()
            {
               // Java: ret += this.minY().hashCode();
               ret = ret.wrapping_add(todo!("minY hashCode"));
            }
            // Java: if (this.hasPosition())
        if self.hasPosition()
            {
               // Java: ret += this.position().hashCode();
               ret = ret.wrapping_add(todo!("position hashCode"));
            }
            // Java: return ret;
            ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
            // Java: if (this.maxX != null && this.maxX.dirty())
        if self.maxX.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.maxY != null && this.maxY.dirty())
        else if self.maxY.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.minX != null && this.minX.dirty())
        else if self.minX.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.minY != null && this.minY.dirty())
        else if self.minY.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else
        else
            {
               // Java: return this.position != null && this.position.dirty() ? true : this.map.dirty();
               return self.position.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
            }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
            // Java: this.saveBin(this.zone);
            let zone = self.zone;
            self.saveBin(zone);
            // Java: return this.box.get();
            self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: return KoBin.Factory.dup(this.box.get(), zone);
            Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
            // Java: this.loadBin(this.box.get(), zone);
            let bin = self.bin_box.get();
            self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanParent();
            self.cleanParent();
            // Java: this.map.load(KoBin.Null.value1());
            let null_bin = Null::value1();
            self.map.load(null_bin.as_ref());
            // Java: this.box.clear();
            self.bin_box.clear();
            // Java: this.zone.leave();
            self.zone.leave();
            // Java: this.zone = KoZone.defaultZone();
            self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
            // Java: this.maxX = null;
            self.maxX = None;
            // Java: this.maxY = null;
            self.maxY = None;
            // Java: this.minX = null;
            self.minX = None;
            // Java: this.minY = null;
            self.minY = None;
            // Java: this.position = null;
            self.position = None;
    }

    // Java: void follow(XY parent)
    pub fn follow(&mut self, parent: Option<Box<XY>>) {
            // Java: if (parent != null)
        if let Some(mut p) = parent
            {
               // Java: parent.discard();
               p.discard();
               // Java: this.discard();
               self.discard();
               // Java: this.parent = parent;
               self.parent = Some(p);
            }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
            // Java: if (this.parent != null)
        if self.parent.is_some()
            {
               // Java: this.discard();
               self.discard();
               // Java: this.parent.discard();
               if let Some(ref mut p) = self.parent { p.discard(); }
               // Java: this.cleanParent();
               self.cleanParent();
            }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
            // Java: return this.parent != null;
            self.parent.is_some()
    }

    // Java: XY parent()
    pub fn get_parent(&self) -> Option<&XY> {
            // Java: return this.parent;
            self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
            // Java: this.discard(this.zone);
            let zone = self.zone;
            self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
            // Java: this.parent = null;
            self.parent = None;
    }

    // Java: String gdRefX()
    pub fn gdRefX(&self) -> String {
            // Java: if (!this.map.has(1))
        if !self.map.has(1)
            {
               // Java: return this.parent == null ? "" : this.parent.gdRefX();
               return self.parent.as_ref().map_or(String::new(), |p| p.gdRefX());
            }
            // Java: else
        else
            {
               // Java: KoBin bin = this.map.byKey(1);
               let bin = self.map.byKey(1);
               // Java: return KoBin.Decode.toText(bin);
               return Decode::toText(bin.as_ref());
            }
    }

    // Java: boolean hasGdRefX()
    pub fn hasGdRefX(&self) -> bool {
            // Java: return this.map.has(1) || this.parent != null && this.parent.hasGdRefX();
            self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasGdRefX())
    }

    // Java: void deleteGdRefX()
    pub fn deleteGdRefX(&mut self) {
            // Java: this.map.put(1, (KoBin)null);
            self.map.clear_key(1);
    }

    // Java: void putGdRefX(String value)
    pub fn putGdRefX(&mut self, value: String) {
            // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
            self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: String gdRefY()
    pub fn gdRefY(&self) -> String {
            // Java: if (!this.map.has(2))
        if !self.map.has(2)
            {
               // Java: return this.parent == null ? "" : this.parent.gdRefY();
               return self.parent.as_ref().map_or(String::new(), |p| p.gdRefY());
            }
            // Java: else
        else
            {
               // Java: KoBin bin = this.map.byKey(2);
               let bin = self.map.byKey(2);
               // Java: return KoBin.Decode.toText(bin);
               return Decode::toText(bin.as_ref());
            }
    }

    // Java: boolean hasGdRefY()
    pub fn hasGdRefY(&self) -> bool {
            // Java: return this.map.has(2) || this.parent != null && this.parent.hasGdRefY();
            self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasGdRefY())
    }

    // Java: void deleteGdRefY()
    pub fn deleteGdRefY(&mut self) {
            // Java: this.map.put(2, (KoBin)null);
            self.map.clear_key(2);
    }

    // Java: void putGdRefY(String value)
    pub fn putGdRefY(&mut self, value: String) {
            // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
            self.map.put(2, Encode::from_int(value, self.zone));
    }

    // Java: AdjCoordinate maxX()
    pub fn get_maxX(&self) -> &dyn KoAr {
            // Java: if (this.maxX == null)
        if self.maxX.is_none()
            {
               // Java: if (this.map.has(3))
            if self.map.has(3)
               {
                  // Java: KoBin bin = this.map.byKey(3);
                  let bin = self.map.byKey(3);
                  // Java: this.maxX = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasMaxX())
            else if self.parent != null && self.parent.hasMaxX()
               {
                  // Java: this.maxX = this.parent.maxX();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.maxX = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.maxX;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasMaxX()
    pub fn hasMaxX(&self) -> bool {
            // Java: return this.map.has(3) || this.maxX != null && this.maxX.dirty() || this.parent != null && this.parent.hasMaxX();
            self.map.has(3) || self.maxX.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMaxX())
    }

    // Java: void deleteMaxX()
    pub fn deleteMaxX(&mut self) {
            // Java: this.map.put(3, (KoBin)null);
            self.map.clear_key(3);
            // Java: if (this.maxX != null)
        if self.maxX.is_some()
            {
               // Java: this.maxX = null;
               self.maxX = None;
            }
    }

    // Java: void writeMaxX(AdjCoordinate value)
    pub fn writeMaxX(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(3, bin);
                  self.map.put(3, bin);
                  // Java: if (this.maxX != null)
                if self.maxX.is_some()
                  {
                     // Java: this.maxX = null;
                     self.maxX = None;
                  }
               }
            }
    }

    // Java: void forceWriteMaxX(AdjCoordinate value)
    pub fn forceWriteMaxX(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(3, value.lastBin(this.zone));
               self.map.put(3, v.lastBin(self.zone));
               // Java: if (this.maxX != null)
            if self.maxX.is_some()
               {
                  // Java: this.maxX = null;
                  self.maxX = None;
               }
            }
    }

    // Java: AdjCoordinate maxY()
    pub fn get_maxY(&self) -> &dyn KoAr {
            // Java: if (this.maxY == null)
        if self.maxY.is_none()
            {
               // Java: if (this.map.has(4))
            if self.map.has(4)
               {
                  // Java: KoBin bin = this.map.byKey(4);
                  let bin = self.map.byKey(4);
                  // Java: this.maxY = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasMaxY())
            else if self.parent != null && self.parent.hasMaxY()
               {
                  // Java: this.maxY = this.parent.maxY();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.maxY = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.maxY;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasMaxY()
    pub fn hasMaxY(&self) -> bool {
            // Java: return this.map.has(4) || this.maxY != null && this.maxY.dirty() || this.parent != null && this.parent.hasMaxY();
            self.map.has(4) || self.maxY.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMaxY())
    }

    // Java: void deleteMaxY()
    pub fn deleteMaxY(&mut self) {
            // Java: this.map.put(4, (KoBin)null);
            self.map.clear_key(4);
            // Java: if (this.maxY != null)
        if self.maxY.is_some()
            {
               // Java: this.maxY = null;
               self.maxY = None;
            }
    }

    // Java: void writeMaxY(AdjCoordinate value)
    pub fn writeMaxY(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(4, bin);
                  self.map.put(4, bin);
                  // Java: if (this.maxY != null)
                if self.maxY.is_some()
                  {
                     // Java: this.maxY = null;
                     self.maxY = None;
                  }
               }
            }
    }

    // Java: void forceWriteMaxY(AdjCoordinate value)
    pub fn forceWriteMaxY(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(4, value.lastBin(this.zone));
               self.map.put(4, v.lastBin(self.zone));
               // Java: if (this.maxY != null)
            if self.maxY.is_some()
               {
                  // Java: this.maxY = null;
                  self.maxY = None;
               }
            }
    }

    // Java: AdjCoordinate minX()
    pub fn get_minX(&self) -> &dyn KoAr {
            // Java: if (this.minX == null)
        if self.minX.is_none()
            {
               // Java: if (this.map.has(5))
            if self.map.has(5)
               {
                  // Java: KoBin bin = this.map.byKey(5);
                  let bin = self.map.byKey(5);
                  // Java: this.minX = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasMinX())
            else if self.parent != null && self.parent.hasMinX()
               {
                  // Java: this.minX = this.parent.minX();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.minX = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.minX;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasMinX()
    pub fn hasMinX(&self) -> bool {
            // Java: return this.map.has(5) || this.minX != null && this.minX.dirty() || this.parent != null && this.parent.hasMinX();
            self.map.has(5) || self.minX.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMinX())
    }

    // Java: void deleteMinX()
    pub fn deleteMinX(&mut self) {
            // Java: this.map.put(5, (KoBin)null);
            self.map.clear_key(5);
            // Java: if (this.minX != null)
        if self.minX.is_some()
            {
               // Java: this.minX = null;
               self.minX = None;
            }
    }

    // Java: void writeMinX(AdjCoordinate value)
    pub fn writeMinX(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(5, bin);
                  self.map.put(5, bin);
                  // Java: if (this.minX != null)
                if self.minX.is_some()
                  {
                     // Java: this.minX = null;
                     self.minX = None;
                  }
               }
            }
    }

    // Java: void forceWriteMinX(AdjCoordinate value)
    pub fn forceWriteMinX(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(5, value.lastBin(this.zone));
               self.map.put(5, v.lastBin(self.zone));
               // Java: if (this.minX != null)
            if self.minX.is_some()
               {
                  // Java: this.minX = null;
                  self.minX = None;
               }
            }
    }

    // Java: AdjCoordinate minY()
    pub fn get_minY(&self) -> &dyn KoAr {
            // Java: if (this.minY == null)
        if self.minY.is_none()
            {
               // Java: if (this.map.has(6))
            if self.map.has(6)
               {
                  // Java: KoBin bin = this.map.byKey(6);
                  let bin = self.map.byKey(6);
                  // Java: this.minY = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasMinY())
            else if self.parent != null && self.parent.hasMinY()
               {
                  // Java: this.minY = this.parent.minY();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.minY = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.minY;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasMinY()
    pub fn hasMinY(&self) -> bool {
            // Java: return this.map.has(6) || this.minY != null && this.minY.dirty() || this.parent != null && this.parent.hasMinY();
            self.map.has(6) || self.minY.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMinY())
    }

    // Java: void deleteMinY()
    pub fn deleteMinY(&mut self) {
            // Java: this.map.put(6, (KoBin)null);
            self.map.clear_key(6);
            // Java: if (this.minY != null)
        if self.minY.is_some()
            {
               // Java: this.minY = null;
               self.minY = None;
            }
    }

    // Java: void writeMinY(AdjCoordinate value)
    pub fn writeMinY(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(6, bin);
                  self.map.put(6, bin);
                  // Java: if (this.minY != null)
                if self.minY.is_some()
                  {
                     // Java: this.minY = null;
                     self.minY = None;
                  }
               }
            }
    }

    // Java: void forceWriteMinY(AdjCoordinate value)
    pub fn forceWriteMinY(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(6, value.lastBin(this.zone));
               self.map.put(6, v.lastBin(self.zone));
               // Java: if (this.minY != null)
            if self.minY.is_some()
               {
                  // Java: this.minY = null;
                  self.minY = None;
               }
            }
    }

    // Java: AdjPoint2D position()
    pub fn get_position(&self) -> &dyn KoAr {
            // Java: if (this.position == null)
        if self.position.is_none()
            {
               // Java: if (this.map.has(7))
            if self.map.has(7)
               {
                  // Java: KoBin bin = this.map.byKey(7);
                  let bin = self.map.byKey(7);
                  // Java: this.position = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasPosition())
            else if self.parent != null && self.parent.hasPosition()
               {
                  // Java: this.position = this.parent.position();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.position = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.position;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasPosition()
    pub fn hasPosition(&self) -> bool {
            // Java: return this.map.has(7) || this.position != null && this.position.dirty() || this.parent != null && this.parent.hasPosition();
            self.map.has(7) || self.position.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPosition())
    }

    // Java: void deletePosition()
    pub fn deletePosition(&mut self) {
            // Java: this.map.put(7, (KoBin)null);
            self.map.clear_key(7);
            // Java: if (this.position != null)
        if self.position.is_some()
            {
               // Java: this.position = null;
               self.position = None;
            }
    }

    // Java: void writePosition(AdjPoint2D value)
    pub fn writePosition(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(7, bin);
                  self.map.put(7, bin);
                  // Java: if (this.position != null)
                if self.position.is_some()
                  {
                     // Java: this.position = null;
                     self.position = None;
                  }
               }
            }
    }

    // Java: void forceWritePosition(AdjPoint2D value)
    pub fn forceWritePosition(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(7, value.lastBin(this.zone));
               self.map.put(7, v.lastBin(self.zone));
               // Java: if (this.position != null)
            if self.position.is_some()
               {
                  // Java: this.position = null;
                  self.position = None;
               }
            }
    }

}

// Java: public class Polar
impl Polar {
    // Java: static protected static <Value1 extends Polar> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
            // Java: KoZone zone = KoZone.enter();
            let zone = KoZone::enter();
            // Java: Value1 inst = (Value1)(zone.allocate(clazz));
            let mut inst = Self::Polar_from(bin);
            // Java: inst.zone = zone;
            inst.zone = zone;
            // Java: inst.load(bin);
            inst.load_bin(bin);
            // Java: return inst;
            inst
    }

    // Java: static Polar create()
    pub fn create() -> Self {
            // Java: return initialize(Polar.class, KoBin.Null.empty());
            let empty = Null::empty();
            Self::initialize(empty.as_ref())
    }

    // Java: static Polar open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
            // Java: return initialize(Polar.class, bin);
            Self::initialize(bin)
    }

    // Java: Polar()
    pub fn new() -> Self {
      Self {
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpMap();
            map: KpMap::new(),
            zone: KoZone::defaultZone(), maxAng: None, maxR: None, minAng: None, minR: None, position: None, parent: None,
            }
    }

    // Java: Polar(KoBin bin)
    pub fn Polar_from(bin: &dyn KoBinTrait) -> Self {
      Self {
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpMap(bin);
            map: KpMap::KpMap_from(bin),
            zone: KoZone::defaultZone(), maxAng: None, maxR: None, minAng: None, minR: None, position: None, parent: None,
            }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.box.set(bin);
            self.bin_box.set(Some(bin));
            // Java: this.map.load(this.box.get());
            let bin_ref = self.bin_box.get();
            self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
            // Java: this.loadBin(bin, this.zone);
            let zone = self.zone;
            self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(Polar value)
    pub fn load_value(&mut self, value: Option<&Polar>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.loadBin(value.lastBin(this.zone), this.zone);
               let bin = v.lastBin(self.zone);
               self.loadBin_with_zone(bin.as_ref(), self.zone);
            }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: if (this.maxAng != null && this.maxAng.dirty())
        if self.maxAng.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(3, this.maxAng.saveBin(zone));
               let saved = self.maxAng.as_mut().unwrap().saveBin(zone);
               self.map.put(3, saved);
            }
            // Java: if (this.maxR != null && this.maxR.dirty())
        if self.maxR.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(4, this.maxR.saveBin(zone));
               let saved = self.maxR.as_mut().unwrap().saveBin(zone);
               self.map.put(4, saved);
            }
            // Java: if (this.minAng != null && this.minAng.dirty())
        if self.minAng.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(5, this.minAng.saveBin(zone));
               let saved = self.minAng.as_mut().unwrap().saveBin(zone);
               self.map.put(5, saved);
            }
            // Java: if (this.minR != null && this.minR.dirty())
        if self.minR.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(6, this.minR.saveBin(zone));
               let saved = self.minR.as_mut().unwrap().saveBin(zone);
               self.map.put(6, saved);
            }
            // Java: if (this.position != null && this.position.dirty())
        if self.position.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(7, this.position.saveBin(zone));
               let saved = self.position.as_mut().unwrap().saveBin(zone);
               self.map.put(7, saved);
            }
            // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
            if self.map.dirty() {
               let saved = self.map.save(zone);
               self.bin_box.set(Some(saved.as_ref()));
               self.bin_box.get()
            } else {
               self.bin_box.get()
            }
    }

    // Java: boolean equalsLax(Polar o)
    pub fn equalsLax(&self, o: &Polar) -> bool {
            // Java: return this.equals(o, false);
            self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Polar) -> bool {
            // Java: return this.equals(o, true);
            self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Polar, isStrict: bool) -> bool {
            // Java: if (null != o && o instanceof Polar)
        if true
            {
          let dst = o;
               // Java: Polar dst = (Polar)o;
               // Java: if (this == dst)
            if std::ptr::eq(self, dst)
               {
                  // Java: return true;
                  return true;
               }
               // Java: else if ((this.hasGdRefAng() || !dst.hasGdRefAng()) && (!this.hasGdRefAng() || dst.hasGdRefAng()))
            else if (self.hasGdRefAng() || !dst.hasGdRefAng()) && (!self.hasGdRefAng() || dst.hasGdRefAng())
               {
                  // Java: if (this.hasGdRefAng() && dst.hasGdRefAng() && !this.gdRefAng().equals(dst.gdRefAng()))
                if self.hasGdRefAng() && dst.hasGdRefAng() && !self.gdRefAng().equals(dst.gdRefAng())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasGdRefR() || !dst.hasGdRefR()) && (!this.hasGdRefR() || dst.hasGdRefR()))
                else if (self.hasGdRefR() || !dst.hasGdRefR()) && (!self.hasGdRefR() || dst.hasGdRefR())
                  {
                     // Java: if (this.hasGdRefR() && dst.hasGdRefR() && !this.gdRefR().equals(dst.gdRefR()))
                    if self.hasGdRefR() && dst.hasGdRefR() && !self.gdRefR().equals(dst.gdRefR())
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasMaxAng() || !dst.hasMaxAng()) && (!this.hasMaxAng() || dst.hasMaxAng()))
                    else if (self.hasMaxAng() || !dst.hasMaxAng()) && (!self.hasMaxAng() || dst.hasMaxAng())
                     {
                        // Java: if (this.hasMaxAng() && dst.hasMaxAng())
                        if self.hasMaxAng() && dst.hasMaxAng()
                        {
                           // Java: if (isStrict && !this.maxAng().equals(dst.maxAng()))
                            if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.maxAng().equalsLax(dst.maxAng()))
                            if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: if ((this.hasMaxR() || !dst.hasMaxR()) && (!this.hasMaxR() || dst.hasMaxR()))
                        if (self.hasMaxR() || !dst.hasMaxR()) && (!self.hasMaxR() || dst.hasMaxR())

                        {
                           // Java: if (this.hasMaxR() && dst.hasMaxR())
                            if self.hasMaxR() && dst.hasMaxR()
                           {
                              // Java: if (isStrict && !this.maxR().equals(dst.maxR()))
                                if todo!("KoAr equals/equalsLax")

                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: if (!isStrict && !this.maxR().equalsLax(dst.maxR()))
                                if todo!("KoAr equals/equalsLax")

                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: if ((this.hasMinAng() || !dst.hasMinAng()) && (!this.hasMinAng() || dst.hasMinAng()))
                            if (self.hasMinAng() || !dst.hasMinAng()) && (!self.hasMinAng() || dst.hasMinAng())

                           {
                              // Java: if (this.hasMinAng() && dst.hasMinAng())
                                if self.hasMinAng() && dst.hasMinAng()
                              {
                                 // Java: if (isStrict && !this.minAng().equals(dst.minAng()))
                                    if todo!("KoAr equals/equalsLax")

                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: if (!isStrict && !this.minAng().equalsLax(dst.minAng()))
                                    if todo!("KoAr equals/equalsLax")

                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: if ((this.hasMinR() || !dst.hasMinR()) && (!this.hasMinR() || dst.hasMinR()))
                                if (self.hasMinR() || !dst.hasMinR()) && (!self.hasMinR() || dst.hasMinR())

                              {
                                 // Java: if (this.hasMinR() && dst.hasMinR())
                                    if self.hasMinR() && dst.hasMinR()
                                 {
                                    // Java: if (isStrict && !this.minR().equals(dst.minR()))
                                        if todo!("KoAr equals/equalsLax")

                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: if (!isStrict && !this.minR().equalsLax(dst.minR()))
                                        if todo!("KoAr equals/equalsLax")

                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                 }
                                 // Java: if ((this.hasPosition() || !dst.hasPosition()) && (!this.hasPosition() || dst.hasPosition()))
                                    if (self.hasPosition() || !dst.hasPosition()) && (!self.hasPosition() || dst.hasPosition())

                                 {
                                    // Java: if (this.hasPosition() && dst.hasPosition())
                                        if self.hasPosition() && dst.hasPosition()
                                    {
                                       // Java: if (isStrict && !this.position().equals(dst.position()))
                                            if todo!("KoAr equals/equalsLax")

                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: if (!isStrict && !this.position().equalsLax(dst.position()))
                                            if todo!("KoAr equals/equalsLax")

                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                    }
                                    // Java: return true;
                                    return true;
                                 }
                                 // Java: else
                                    else
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: else
                                else
                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: else
                            else
                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: else
                        else
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: else
                    else
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
            else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
        else
            {
               // Java: return false;
               return false;
            }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
            // Java: int ret = 0;
            let mut ret: i32 = 0;
            // Java: if (this.hasGdRefAng())
        if self.hasGdRefAng()
            {
               // Java: ret += String.valueOf(this.gdRefAng()).hashCode();
               ret = ret.wrapping_add(self.gdRefAng().len() as i32);
            }
            // Java: if (this.hasGdRefR())
        if self.hasGdRefR()
            {
               // Java: ret += String.valueOf(this.gdRefR()).hashCode();
               ret = ret.wrapping_add(self.gdRefR().len() as i32);
            }
            // Java: if (this.hasMaxAng())
        if self.hasMaxAng()
            {
               // Java: ret += this.maxAng().hashCode();
               ret = ret.wrapping_add(todo!("maxAng hashCode"));
            }
            // Java: if (this.hasMaxR())
        if self.hasMaxR()
            {
               // Java: ret += this.maxR().hashCode();
               ret = ret.wrapping_add(todo!("maxR hashCode"));
            }
            // Java: if (this.hasMinAng())
        if self.hasMinAng()
            {
               // Java: ret += this.minAng().hashCode();
               ret = ret.wrapping_add(todo!("minAng hashCode"));
            }
            // Java: if (this.hasMinR())
        if self.hasMinR()
            {
               // Java: ret += this.minR().hashCode();
               ret = ret.wrapping_add(todo!("minR hashCode"));
            }
            // Java: if (this.hasPosition())
        if self.hasPosition()
            {
               // Java: ret += this.position().hashCode();
               ret = ret.wrapping_add(todo!("position hashCode"));
            }
            // Java: return ret;
            ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
            // Java: if (this.maxAng != null && this.maxAng.dirty())
        if self.maxAng.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.maxR != null && this.maxR.dirty())
        else if self.maxR.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.minAng != null && this.minAng.dirty())
        else if self.minAng.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.minR != null && this.minR.dirty())
        else if self.minR.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else
        else
            {
               // Java: return this.position != null && this.position.dirty() ? true : this.map.dirty();
               return self.position.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
            }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
            // Java: this.saveBin(this.zone);
            let zone = self.zone;
            self.saveBin(zone);
            // Java: return this.box.get();
            self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: return KoBin.Factory.dup(this.box.get(), zone);
            Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
            // Java: this.loadBin(this.box.get(), zone);
            let bin = self.bin_box.get();
            self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanParent();
            self.cleanParent();
            // Java: this.map.load(KoBin.Null.value1());
            let null_bin = Null::value1();
            self.map.load(null_bin.as_ref());
            // Java: this.box.clear();
            self.bin_box.clear();
            // Java: this.zone.leave();
            self.zone.leave();
            // Java: this.zone = KoZone.defaultZone();
            self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
            // Java: this.maxAng = null;
            self.maxAng = None;
            // Java: this.maxR = null;
            self.maxR = None;
            // Java: this.minAng = null;
            self.minAng = None;
            // Java: this.minR = null;
            self.minR = None;
            // Java: this.position = null;
            self.position = None;
    }

    // Java: void follow(Polar parent)
    pub fn follow(&mut self, parent: Option<Box<Polar>>) {
            // Java: if (parent != null)
        if let Some(mut p) = parent
            {
               // Java: parent.discard();
               p.discard();
               // Java: this.discard();
               self.discard();
               // Java: this.parent = parent;
               self.parent = Some(p);
            }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
            // Java: if (this.parent != null)
        if self.parent.is_some()
            {
               // Java: this.discard();
               self.discard();
               // Java: this.parent.discard();
               if let Some(ref mut p) = self.parent { p.discard(); }
               // Java: this.cleanParent();
               self.cleanParent();
            }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
            // Java: return this.parent != null;
            self.parent.is_some()
    }

    // Java: Polar parent()
    pub fn get_parent(&self) -> Option<&Polar> {
            // Java: return this.parent;
            self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
            // Java: this.discard(this.zone);
            let zone = self.zone;
            self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
            // Java: this.parent = null;
            self.parent = None;
    }

    // Java: String gdRefAng()
    pub fn gdRefAng(&self) -> String {
            // Java: if (!this.map.has(1))
        if !self.map.has(1)
            {
               // Java: return this.parent == null ? "" : this.parent.gdRefAng();
               return self.parent.as_ref().map_or(String::new(), |p| p.gdRefAng());
            }
            // Java: else
        else
            {
               // Java: KoBin bin = this.map.byKey(1);
               let bin = self.map.byKey(1);
               // Java: return KoBin.Decode.toText(bin);
               return Decode::toText(bin.as_ref());
            }
    }

    // Java: boolean hasGdRefAng()
    pub fn hasGdRefAng(&self) -> bool {
            // Java: return this.map.has(1) || this.parent != null && this.parent.hasGdRefAng();
            self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasGdRefAng())
    }

    // Java: void deleteGdRefAng()
    pub fn deleteGdRefAng(&mut self) {
            // Java: this.map.put(1, (KoBin)null);
            self.map.clear_key(1);
    }

    // Java: void putGdRefAng(String value)
    pub fn putGdRefAng(&mut self, value: String) {
            // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
            self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: String gdRefR()
    pub fn gdRefR(&self) -> String {
            // Java: if (!this.map.has(2))
        if !self.map.has(2)
            {
               // Java: return this.parent == null ? "" : this.parent.gdRefR();
               return self.parent.as_ref().map_or(String::new(), |p| p.gdRefR());
            }
            // Java: else
        else
            {
               // Java: KoBin bin = this.map.byKey(2);
               let bin = self.map.byKey(2);
               // Java: return KoBin.Decode.toText(bin);
               return Decode::toText(bin.as_ref());
            }
    }

    // Java: boolean hasGdRefR()
    pub fn hasGdRefR(&self) -> bool {
            // Java: return this.map.has(2) || this.parent != null && this.parent.hasGdRefR();
            self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasGdRefR())
    }

    // Java: void deleteGdRefR()
    pub fn deleteGdRefR(&mut self) {
            // Java: this.map.put(2, (KoBin)null);
            self.map.clear_key(2);
    }

    // Java: void putGdRefR(String value)
    pub fn putGdRefR(&mut self, value: String) {
            // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
            self.map.put(2, Encode::from_int(value, self.zone));
    }

    // Java: AdjAngle maxAng()
    pub fn get_maxAng(&self) -> &dyn KoAr {
            // Java: if (this.maxAng == null)
        if self.maxAng.is_none()
            {
               // Java: if (this.map.has(3))
            if self.map.has(3)
               {
                  // Java: KoBin bin = this.map.byKey(3);
                  let bin = self.map.byKey(3);
                  // Java: this.maxAng = (AdjAngle)KoBin.Decode.toAny(AdjAngle.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasMaxAng())
            else if self.parent != null && self.parent.hasMaxAng()
               {
                  // Java: this.maxAng = this.parent.maxAng();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.maxAng = (AdjAngle)KoBin.Decode.toAny(AdjAngle.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.maxAng;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasMaxAng()
    pub fn hasMaxAng(&self) -> bool {
            // Java: return this.map.has(3) || this.maxAng != null && this.maxAng.dirty() || this.parent != null && this.parent.hasMaxAng();
            self.map.has(3) || self.maxAng.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMaxAng())
    }

    // Java: void deleteMaxAng()
    pub fn deleteMaxAng(&mut self) {
            // Java: this.map.put(3, (KoBin)null);
            self.map.clear_key(3);
            // Java: if (this.maxAng != null)
        if self.maxAng.is_some()
            {
               // Java: this.maxAng = null;
               self.maxAng = None;
            }
    }

    // Java: void writeMaxAng(AdjAngle value)
    pub fn writeMaxAng(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(3, bin);
                  self.map.put(3, bin);
                  // Java: if (this.maxAng != null)
                if self.maxAng.is_some()
                  {
                     // Java: this.maxAng = null;
                     self.maxAng = None;
                  }
               }
            }
    }

    // Java: void forceWriteMaxAng(AdjAngle value)
    pub fn forceWriteMaxAng(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(3, value.lastBin(this.zone));
               self.map.put(3, v.lastBin(self.zone));
               // Java: if (this.maxAng != null)
            if self.maxAng.is_some()
               {
                  // Java: this.maxAng = null;
                  self.maxAng = None;
               }
            }
    }

    // Java: AdjCoordinate maxR()
    pub fn get_maxR(&self) -> &dyn KoAr {
            // Java: if (this.maxR == null)
        if self.maxR.is_none()
            {
               // Java: if (this.map.has(4))
            if self.map.has(4)
               {
                  // Java: KoBin bin = this.map.byKey(4);
                  let bin = self.map.byKey(4);
                  // Java: this.maxR = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasMaxR())
            else if self.parent != null && self.parent.hasMaxR()
               {
                  // Java: this.maxR = this.parent.maxR();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.maxR = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.maxR;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasMaxR()
    pub fn hasMaxR(&self) -> bool {
            // Java: return this.map.has(4) || this.maxR != null && this.maxR.dirty() || this.parent != null && this.parent.hasMaxR();
            self.map.has(4) || self.maxR.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMaxR())
    }

    // Java: void deleteMaxR()
    pub fn deleteMaxR(&mut self) {
            // Java: this.map.put(4, (KoBin)null);
            self.map.clear_key(4);
            // Java: if (this.maxR != null)
        if self.maxR.is_some()
            {
               // Java: this.maxR = null;
               self.maxR = None;
            }
    }

    // Java: void writeMaxR(AdjCoordinate value)
    pub fn writeMaxR(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(4, bin);
                  self.map.put(4, bin);
                  // Java: if (this.maxR != null)
                if self.maxR.is_some()
                  {
                     // Java: this.maxR = null;
                     self.maxR = None;
                  }
               }
            }
    }

    // Java: void forceWriteMaxR(AdjCoordinate value)
    pub fn forceWriteMaxR(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(4, value.lastBin(this.zone));
               self.map.put(4, v.lastBin(self.zone));
               // Java: if (this.maxR != null)
            if self.maxR.is_some()
               {
                  // Java: this.maxR = null;
                  self.maxR = None;
               }
            }
    }

    // Java: AdjAngle minAng()
    pub fn get_minAng(&self) -> &dyn KoAr {
            // Java: if (this.minAng == null)
        if self.minAng.is_none()
            {
               // Java: if (this.map.has(5))
            if self.map.has(5)
               {
                  // Java: KoBin bin = this.map.byKey(5);
                  let bin = self.map.byKey(5);
                  // Java: this.minAng = (AdjAngle)KoBin.Decode.toAny(AdjAngle.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasMinAng())
            else if self.parent != null && self.parent.hasMinAng()
               {
                  // Java: this.minAng = this.parent.minAng();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.minAng = (AdjAngle)KoBin.Decode.toAny(AdjAngle.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.minAng;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasMinAng()
    pub fn hasMinAng(&self) -> bool {
            // Java: return this.map.has(5) || this.minAng != null && this.minAng.dirty() || this.parent != null && this.parent.hasMinAng();
            self.map.has(5) || self.minAng.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMinAng())
    }

    // Java: void deleteMinAng()
    pub fn deleteMinAng(&mut self) {
            // Java: this.map.put(5, (KoBin)null);
            self.map.clear_key(5);
            // Java: if (this.minAng != null)
        if self.minAng.is_some()
            {
               // Java: this.minAng = null;
               self.minAng = None;
            }
    }

    // Java: void writeMinAng(AdjAngle value)
    pub fn writeMinAng(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(5, bin);
                  self.map.put(5, bin);
                  // Java: if (this.minAng != null)
                if self.minAng.is_some()
                  {
                     // Java: this.minAng = null;
                     self.minAng = None;
                  }
               }
            }
    }

    // Java: void forceWriteMinAng(AdjAngle value)
    pub fn forceWriteMinAng(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(5, value.lastBin(this.zone));
               self.map.put(5, v.lastBin(self.zone));
               // Java: if (this.minAng != null)
            if self.minAng.is_some()
               {
                  // Java: this.minAng = null;
                  self.minAng = None;
               }
            }
    }

    // Java: AdjCoordinate minR()
    pub fn get_minR(&self) -> &dyn KoAr {
            // Java: if (this.minR == null)
        if self.minR.is_none()
            {
               // Java: if (this.map.has(6))
            if self.map.has(6)
               {
                  // Java: KoBin bin = this.map.byKey(6);
                  let bin = self.map.byKey(6);
                  // Java: this.minR = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasMinR())
            else if self.parent != null && self.parent.hasMinR()
               {
                  // Java: this.minR = this.parent.minR();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.minR = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.minR;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasMinR()
    pub fn hasMinR(&self) -> bool {
            // Java: return this.map.has(6) || this.minR != null && this.minR.dirty() || this.parent != null && this.parent.hasMinR();
            self.map.has(6) || self.minR.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMinR())
    }

    // Java: void deleteMinR()
    pub fn deleteMinR(&mut self) {
            // Java: this.map.put(6, (KoBin)null);
            self.map.clear_key(6);
            // Java: if (this.minR != null)
        if self.minR.is_some()
            {
               // Java: this.minR = null;
               self.minR = None;
            }
    }

    // Java: void writeMinR(AdjCoordinate value)
    pub fn writeMinR(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(6, bin);
                  self.map.put(6, bin);
                  // Java: if (this.minR != null)
                if self.minR.is_some()
                  {
                     // Java: this.minR = null;
                     self.minR = None;
                  }
               }
            }
    }

    // Java: void forceWriteMinR(AdjCoordinate value)
    pub fn forceWriteMinR(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(6, value.lastBin(this.zone));
               self.map.put(6, v.lastBin(self.zone));
               // Java: if (this.minR != null)
            if self.minR.is_some()
               {
                  // Java: this.minR = null;
                  self.minR = None;
               }
            }
    }

    // Java: AdjPoint2D position()
    pub fn get_position(&self) -> &dyn KoAr {
            // Java: if (this.position == null)
        if self.position.is_none()
            {
               // Java: if (this.map.has(7))
            if self.map.has(7)
               {
                  // Java: KoBin bin = this.map.byKey(7);
                  let bin = self.map.byKey(7);
                  // Java: this.position = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasPosition())
            else if self.parent != null && self.parent.hasPosition()
               {
                  // Java: this.position = this.parent.position();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.position = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.position;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasPosition()
    pub fn hasPosition(&self) -> bool {
            // Java: return this.map.has(7) || this.position != null && this.position.dirty() || this.parent != null && this.parent.hasPosition();
            self.map.has(7) || self.position.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPosition())
    }

    // Java: void deletePosition()
    pub fn deletePosition(&mut self) {
            // Java: this.map.put(7, (KoBin)null);
            self.map.clear_key(7);
            // Java: if (this.position != null)
        if self.position.is_some()
            {
               // Java: this.position = null;
               self.position = None;
            }
    }

    // Java: void writePosition(AdjPoint2D value)
    pub fn writePosition(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(7, bin);
                  self.map.put(7, bin);
                  // Java: if (this.position != null)
                if self.position.is_some()
                  {
                     // Java: this.position = null;
                     self.position = None;
                  }
               }
            }
    }

    // Java: void forceWritePosition(AdjPoint2D value)
    pub fn forceWritePosition(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(7, value.lastBin(this.zone));
               self.map.put(7, v.lastBin(self.zone));
               // Java: if (this.position != null)
            if self.position.is_some()
               {
                  // Java: this.position = null;
                  self.position = None;
               }
            }
    }

}

// Java: public class AdjustHandleList
impl AdjustHandleList {
    // Java: AdjustHandleList()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.list = new KpList(this.zone);
         list: KpList::new(KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: AdjustHandleList(KoBin bin)
    pub fn AdjustHandleList_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.list = new KpList(bin, this.zone);
         list: KpList::KpList_from(bin, KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get();
         self.list.load(bin_ref.as_ref(), _zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(AdjustHandleList value)
    pub fn load_value(&mut self, value: Option<&AdjustHandleList>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin_with_zone(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() {
            let saved = self.list.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(AdjustHandleList o)
    pub fn equalsLax(&self, o: &AdjustHandleList) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AdjustHandleList) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &AdjustHandleList, isStrict: bool) -> bool {
         // Java: if (null == o)
        if true
         {
          let dst = o;
            // Java: return false;
            return false;
         }
         // Java: else
        else
         {
            // Java: AdjustHandleList dst = (AdjustHandleList)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            else
            {
               // Java: ArrayList<AdjustHandle> dstList = new ArrayList();
               todo!("list toList")
               // Java: dst.read(dstList);
               todo!("dst.read(dstList);")
               // Java: ArrayList<AdjustHandle> srcList = new ArrayList();
               todo!("list toList")
               // Java: this.read(srcList);
               todo!("this.read(srcList);")
               // Java: if (dstList.size() != srcList.size())
                todo!("if (dstList.size() != srcList.size())")
               {
                  // Java: return false;
                  return false;
               }
               // Java: else
                else
               {
                  // Java: int size = dstList.size();
                  todo!("int size = dstList.size();")
                  // Java: for(int i = 0; i < size; ++i)
                    todo!("for(int i = 0; i < size; ++i)")
                  {
                     // Java: AdjustHandle dstClrTrans = (AdjustHandle)dstList.get(i);
                     todo!("AdjustHandle dstClrTrans = (AdjustHandle)dstList.get(i);")
                     // Java: AdjustHandle srcClrTrans = (AdjustHandle)srcList.get(i);
                     todo!("AdjustHandle srcClrTrans = (AdjustHandle)srcList.get(i);")
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                        if isStrict && !dstClrTrans.equals(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                        if !isStrict && !dstClrTrans.equalsLax(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: return true;
                  return true;
               }
            }
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         todo!("list hashCode")
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::value1();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends AdjustHandleList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::AdjustHandleList_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static AdjustHandleList create()
    pub fn create() -> Self {
         // Java: return initialize(AdjustHandleList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static AdjustHandleList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(AdjustHandleList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<AdjustHandle> valueLst)
    pub fn read(&self, Collection<AdjustHandle> valueLst) {
         // Java: valueLst.clear();
         todo!("valueLst.clear();")
         // Java: int size = this.list.size();
         todo!("int size = this.list.size();")
         // Java: for(int i = 0; i < size; ++i)
        todo!("for(int i = 0; i < size; ++i)")
         {
            // Java: valueLst.add((AdjustHandle)KoBin.Decode.toAny(AdjustHandle.class, this.list.get(i), this.zone));
            todo!("lazy init requires &mut self")
         }
    }

    // Java: void write(Iterable<AdjustHandle> valueLst)
    pub fn write(&self, Iterable<AdjustHandle> valueLst) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(AdjustHandle i : valueLst)
        todo!("for(AdjustHandle i : valueLst)")
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            todo!("KoBin bin = i.lastBin(this.zone);")
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.list.add(bin);
               todo!("list add")
            }
         }
    }

    // Java: void forceWrite(Iterable<AdjustHandle> valueLst)
    pub fn forceWrite(&self, Iterable<AdjustHandle> valueLst) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(AdjustHandle i : valueLst)
        todo!("for(AdjustHandle i : valueLst)")
         {
            // Java: this.list.add(i.lastBin(this.zone));
            todo!("list add")
         }
    }

}

// Java: public class GeomGuideList
impl GeomGuideList {
    // Java: GeomGuideList()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.list = new KpList(this.zone);
         list: KpList::new(KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: GeomGuideList(KoBin bin)
    pub fn GeomGuideList_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.list = new KpList(bin, this.zone);
         list: KpList::KpList_from(bin, KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get();
         self.list.load(bin_ref.as_ref(), _zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(GeomGuideList value)
    pub fn load_value(&mut self, value: Option<&GeomGuideList>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin_with_zone(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() {
            let saved = self.list.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(GeomGuideList o)
    pub fn equalsLax(&self, o: &GeomGuideList) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &GeomGuideList) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &GeomGuideList, isStrict: bool) -> bool {
         // Java: if (null == o)
        if true
         {
          let dst = o;
            // Java: return false;
            return false;
         }
         // Java: else
        else
         {
            // Java: GeomGuideList dst = (GeomGuideList)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            else
            {
               // Java: ArrayList<GeomGuide> dstList = new ArrayList();
               todo!("list toList")
               // Java: dst.read(dstList);
               todo!("dst.read(dstList);")
               // Java: ArrayList<GeomGuide> srcList = new ArrayList();
               todo!("list toList")
               // Java: this.read(srcList);
               todo!("this.read(srcList);")
               // Java: if (dstList.size() != srcList.size())
                todo!("if (dstList.size() != srcList.size())")
               {
                  // Java: return false;
                  return false;
               }
               // Java: else
                else
               {
                  // Java: int size = dstList.size();
                  todo!("int size = dstList.size();")
                  // Java: for(int i = 0; i < size; ++i)
                    todo!("for(int i = 0; i < size; ++i)")
                  {
                     // Java: GeomGuide dstClrTrans = (GeomGuide)dstList.get(i);
                     todo!("GeomGuide dstClrTrans = (GeomGuide)dstList.get(i);")
                     // Java: GeomGuide srcClrTrans = (GeomGuide)srcList.get(i);
                     todo!("GeomGuide srcClrTrans = (GeomGuide)srcList.get(i);")
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                        if isStrict && !dstClrTrans.equals(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                        if !isStrict && !dstClrTrans.equalsLax(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: return true;
                  return true;
               }
            }
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         todo!("list hashCode")
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::value1();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends GeomGuideList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::GeomGuideList_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static GeomGuideList create()
    pub fn create() -> Self {
         // Java: return initialize(GeomGuideList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static GeomGuideList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(GeomGuideList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<GeomGuide> valueLst)
    pub fn read(&self, Collection<GeomGuide> valueLst) {
         // Java: valueLst.clear();
         todo!("valueLst.clear();")
         // Java: int size = this.list.size();
         todo!("int size = this.list.size();")
         // Java: for(int i = 0; i < size; ++i)
        todo!("for(int i = 0; i < size; ++i)")
         {
            // Java: valueLst.add((GeomGuide)KoBin.Decode.toAny(GeomGuide.class, this.list.get(i), this.zone));
            todo!("lazy init requires &mut self")
         }
    }

    // Java: void write(Iterable<GeomGuide> valueLst)
    pub fn write(&self, Iterable<GeomGuide> valueLst) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(GeomGuide i : valueLst)
        todo!("for(GeomGuide i : valueLst)")
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            todo!("KoBin bin = i.lastBin(this.zone);")
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.list.add(bin);
               todo!("list add")
            }
         }
    }

    // Java: void forceWrite(Iterable<GeomGuide> valueLst)
    pub fn forceWrite(&self, Iterable<GeomGuide> valueLst) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(GeomGuide i : valueLst)
        todo!("for(GeomGuide i : valueLst)")
         {
            // Java: this.list.add(i.lastBin(this.zone));
            todo!("list add")
         }
    }

}

// Java: public class AdjustValueList
impl AdjustValueList {
    // Java: AdjustValueList()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.list = new KpList(this.zone);
         list: KpList::new(KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: AdjustValueList(KoBin bin)
    pub fn AdjustValueList_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.list = new KpList(bin, this.zone);
         list: KpList::KpList_from(bin, KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get();
         self.list.load(bin_ref.as_ref(), _zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(AdjustValueList value)
    pub fn load_value(&mut self, value: Option<&AdjustValueList>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin_with_zone(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() {
            let saved = self.list.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(AdjustValueList o)
    pub fn equalsLax(&self, o: &AdjustValueList) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AdjustValueList) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &AdjustValueList, isStrict: bool) -> bool {
         // Java: if (null == o)
        if true
         {
          let dst = o;
            // Java: return false;
            return false;
         }
         // Java: else
        else
         {
            // Java: AdjustValueList dst = (AdjustValueList)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            else
            {
               // Java: ArrayList<GeomGuide> dstList = new ArrayList();
               todo!("list toList")
               // Java: dst.read(dstList);
               todo!("dst.read(dstList);")
               // Java: ArrayList<GeomGuide> srcList = new ArrayList();
               todo!("list toList")
               // Java: this.read(srcList);
               todo!("this.read(srcList);")
               // Java: if (dstList.size() != srcList.size())
                todo!("if (dstList.size() != srcList.size())")
               {
                  // Java: return false;
                  return false;
               }
               // Java: else
                else
               {
                  // Java: int size = dstList.size();
                  todo!("int size = dstList.size();")
                  // Java: for(int i = 0; i < size; ++i)
                    todo!("for(int i = 0; i < size; ++i)")
                  {
                     // Java: GeomGuide dstClrTrans = (GeomGuide)dstList.get(i);
                     todo!("GeomGuide dstClrTrans = (GeomGuide)dstList.get(i);")
                     // Java: GeomGuide srcClrTrans = (GeomGuide)srcList.get(i);
                     todo!("GeomGuide srcClrTrans = (GeomGuide)srcList.get(i);")
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                        if isStrict && !dstClrTrans.equals(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                        if !isStrict && !dstClrTrans.equalsLax(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: return true;
                  return true;
               }
            }
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         todo!("list hashCode")
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::value1();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends AdjustValueList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::AdjustValueList_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static AdjustValueList create()
    pub fn create() -> Self {
         // Java: return initialize(AdjustValueList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static AdjustValueList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(AdjustValueList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<GeomGuide> valueLst)
    pub fn read(&self, Collection<GeomGuide> valueLst) {
         // Java: valueLst.clear();
         todo!("valueLst.clear();")
         // Java: int size = this.list.size();
         todo!("int size = this.list.size();")
         // Java: for(int i = 0; i < size; ++i)
        todo!("for(int i = 0; i < size; ++i)")
         {
            // Java: valueLst.add((GeomGuide)KoBin.Decode.toAny(GeomGuide.class, this.list.get(i), this.zone));
            todo!("lazy init requires &mut self")
         }
    }

    // Java: void write(Iterable<GeomGuide> valueLst)
    pub fn write(&self, Iterable<GeomGuide> valueLst) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(GeomGuide i : valueLst)
        todo!("for(GeomGuide i : valueLst)")
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            todo!("KoBin bin = i.lastBin(this.zone);")
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.list.add(bin);
               todo!("list add")
            }
         }
    }

    // Java: void forceWrite(Iterable<GeomGuide> valueLst)
    pub fn forceWrite(&self, Iterable<GeomGuide> valueLst) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(GeomGuide i : valueLst)
        todo!("for(GeomGuide i : valueLst)")
         {
            // Java: this.list.add(i.lastBin(this.zone));
            todo!("list add")
         }
    }

}

// Java: public class Path
impl Path {
    // Java: static protected static <Value1 extends Path> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::Path_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static Path create()
    pub fn create() -> Self {
         // Java: return initialize(Path.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static Path open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(Path.class, bin);
         Self::initialize(bin)
    }

    // Java: Path()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.map = new KpMap();
         map: KpMap::new(),
         zone: KoZone::defaultZone(), pathSegmentLst: None, parent: None,
         }
    }

    // Java: Path(KoBin bin)
    pub fn Path_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.map = new KpMap(bin);
         map: KpMap::KpMap_from(bin),
         zone: KoZone::defaultZone(), pathSegmentLst: None, parent: None,
         }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
         // Java: this.cleanCache();
         self.cleanCache();
         // Java: this.cleanFollow();
         self.cleanFollow();
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.map.load(this.box.get());
         let bin_ref = self.bin_box.get();
         self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(Path value)
    pub fn load_value(&mut self, value: Option<&Path>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin_with_zone(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: if (this.pathSegmentLst != null && this.pathSegmentLst.dirty())
        if self.pathSegmentLst.as_ref().map_or(false, |x| x.dirty())
         {
            // Java: this.map.put(6, this.pathSegmentLst.saveBin(zone));
            let saved = self.pathSegmentLst.as_mut().unwrap().saveBin(zone);
            self.map.put(6, saved);
         }
         // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
         if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(Path o)
    pub fn equalsLax(&self, o: &Path) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Path) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Path, isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof Path)
        if true
         {
          let dst = o;
            // Java: Path dst = (Path)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasExtrusionOk() || !dst.hasExtrusionOk()) && (!this.hasExtrusionOk() || dst.hasExtrusionOk()))
            else if (self.hasExtrusionOk() || !dst.hasExtrusionOk()) && (!self.hasExtrusionOk() || dst.hasExtrusionOk())
            {
               // Java: if (this.hasExtrusionOk() && dst.hasExtrusionOk() && this.isExtrusionOk() != dst.isExtrusionOk())
                if self.hasExtrusionOk() && dst.hasExtrusionOk() && self.isExtrusionOk() != dst.isExtrusionOk()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasFill() || !dst.hasFill()) && (!this.hasFill() || dst.hasFill()))
                else if (self.hasFill() || !dst.hasFill()) && (!self.hasFill() || dst.hasFill())
               {
                  // Java: if (this.hasFill() && dst.hasFill() && this.fill() != dst.fill())
                    if self.hasFill() && dst.hasFill() && self.fill() != dst.fill()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasH() || !dst.hasH()) && (!this.hasH() || dst.hasH()))
                    else if (self.hasH() || !dst.hasH()) && (!self.hasH() || dst.hasH())
                  {
                     // Java: if (this.hasH() && dst.hasH() && this.h() != dst.h())
                        if self.hasH() && dst.hasH() && self.h() != dst.h()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasStroke() || !dst.hasStroke()) && (!this.hasStroke() || dst.hasStroke()))
                        else if (self.hasStroke() || !dst.hasStroke()) && (!self.hasStroke() || dst.hasStroke())
                     {
                        // Java: if (this.hasStroke() && dst.hasStroke() && this.isStroke() != dst.isStroke())
                            if self.hasStroke() && dst.hasStroke() && self.isStroke() != dst.isStroke()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasW() || !dst.hasW()) && (!this.hasW() || dst.hasW()))
                            else if (self.hasW() || !dst.hasW()) && (!self.hasW() || dst.hasW())
                        {
                           // Java: if (this.hasW() && dst.hasW() && this.w() != dst.w())
                                if self.hasW() && dst.hasW() && self.w() != dst.w()
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasPathSegmentLst() || !dst.hasPathSegmentLst()) && (!this.hasPathSegmentLst() || dst.hasPathSegmentLst()))
                                else if (self.hasPathSegmentLst() || !dst.hasPathSegmentLst()) && (!self.hasPathSegmentLst() || dst.hasPathSegmentLst())
                           {
                              // Java: if (this.hasPathSegmentLst() && dst.hasPathSegmentLst())
                                    if self.hasPathSegmentLst() && dst.hasPathSegmentLst()
                              {
                                 // Java: if (isStrict && !this.pathSegmentLst().equals(dst.pathSegmentLst()))
                                        if todo!("KoAr equals/equalsLax")

                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: if (!isStrict && !this.pathSegmentLst().equalsLax(dst.pathSegmentLst()))
                                        if todo!("KoAr equals/equalsLax")

                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: return true;
                              return true;
                           }
                           // Java: else
                                else
                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: else
                            else
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: else
                        else
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                    else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
                else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
            else
            {
               // Java: return false;
               return false;
            }
         }
         // Java: else
        else
         {
            // Java: return false;
            return false;
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: int ret = 0;
         let mut ret: i32 = 0;
         // Java: if (this.hasExtrusionOk())
        if self.hasExtrusionOk()
         {
            // Java: ret += Boolean.valueOf(this.isExtrusionOk()).hashCode();
            ret = ret.wrapping_add(if self.isExtrusionOk() { 1231 } else { 1237 });
         }
         // Java: if (this.hasFill())
        if self.hasFill()
         {
            // Java: ret += Integer.valueOf(this.fill()).hashCode();
            ret = ret.wrapping_add(self.fill());
         }
         // Java: if (this.hasH())
        if self.hasH()
         {
            // Java: ret += Integer.valueOf(this.h()).hashCode();
            ret = ret.wrapping_add(self.h());
         }
         // Java: if (this.hasStroke())
        if self.hasStroke()
         {
            // Java: ret += Boolean.valueOf(this.isStroke()).hashCode();
            ret = ret.wrapping_add(if self.isStroke() { 1231 } else { 1237 });
         }
         // Java: if (this.hasW())
        if self.hasW()
         {
            // Java: ret += Integer.valueOf(this.w()).hashCode();
            ret = ret.wrapping_add(self.w());
         }
         // Java: if (this.hasPathSegmentLst())
        if self.hasPathSegmentLst()
         {
            // Java: ret += this.pathSegmentLst().hashCode();
            ret = ret.wrapping_add(todo!("pathSegmentLst hashCode"));
         }
         // Java: return ret;
         ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.pathSegmentLst != null && this.pathSegmentLst.dirty() ? true : this.map.dirty();
         return self.pathSegmentLst.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.cleanFollow();
         self.cleanFollow();
         // Java: this.cleanCache();
         self.cleanCache();
         // Java: this.cleanParent();
         self.cleanParent();
         // Java: this.map.load(KoBin.Null.value1());
         let null_bin = Null::value1();
         self.map.load(null_bin.as_ref());
         // Java: this.box.clear();
         self.bin_box.clear();
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
         // Java: this.pathSegmentLst = null;
         self.pathSegmentLst = None;
    }

    // Java: void follow(Path parent)
    pub fn follow(&mut self, parent: Option<Box<Path>>) {
         // Java: if (parent != null)
        if let Some(mut p) = parent
         {
            // Java: parent.discard();
            p.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
            self.parent = Some(p);
         }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
         // Java: if (this.parent != null)
        if self.parent.is_some()
         {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(ref mut p) = self.parent { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
         }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
         // Java: return this.parent != null;
         self.parent.is_some()
    }

    // Java: Path parent()
    pub fn get_parent(&self) -> Option<&Path> {
         // Java: return this.parent;
         self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
         // Java: this.discard(this.zone);
         let zone = self.zone;
         self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
         // Java: this.parent = null;
         self.parent = None;
    }

    // Java: boolean isExtrusionOk()
    pub fn isExtrusionOk(&self) -> bool {
         // Java: if (!this.map.has(1))
        if !self.map.has(1)
         {
            // Java: return this.parent == null ? true : this.parent.isExtrusionOk();
            return self.parent.as_ref().map_or(true, |p| p.isExtrusionOk());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toBool(bin);
            return Decode::toBool(bin.as_ref());
         }
    }

    // Java: boolean hasExtrusionOk()
    pub fn hasExtrusionOk(&self) -> bool {
         // Java: return this.map.has(1) || this.parent != null && this.parent.hasExtrusionOk();
         self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasExtrusionOk())
    }

    // Java: void deleteExtrusionOk()
    pub fn deleteExtrusionOk(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(1);
    }

    // Java: void putExtrusionOk(boolean value)
    pub fn putExtrusionOk(&mut self, value: bool) {
         // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
         self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: int fill()
    pub fn fill(&self) -> i32 {
         // Java: if (!this.map.has(2))
        if !self.map.has(2)
         {
            // Java: return this.parent == null ? 5 : this.parent.fill();
            return self.parent.as_ref().map_or(5, |p| p.fill());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toInt(bin);
            return Decode::toInt(bin.as_ref());
         }
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
         // Java: return this.map.has(2) || this.parent != null && this.parent.hasFill();
         self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasFill())
    }

    // Java: void deleteFill()
    pub fn deleteFill(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(2);
    }

    // Java: void putFill(int value)
    pub fn putFill(&mut self, value: i32) {
         // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
         self.map.put(2, Encode::from_int(value, self.zone));
    }

    // Java: int h()
    pub fn h(&self) -> i32 {
         // Java: if (!this.map.has(3))
        if !self.map.has(3)
         {
            // Java: return this.parent == null ? 0 : this.parent.h();
            return self.parent.as_ref().map_or(0, |p| p.h());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: return KoBin.Decode.toInt(bin);
            return Decode::toInt(bin.as_ref());
         }
    }

    // Java: boolean hasH()
    pub fn hasH(&self) -> bool {
         // Java: return this.map.has(3) || this.parent != null && this.parent.hasH();
         self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasH())
    }

    // Java: void deleteH()
    pub fn deleteH(&mut self) {
         // Java: this.map.put(3, (KoBin)null);
         self.map.clear_key(3);
    }

    // Java: void putH(int value)
    pub fn putH(&mut self, value: i32) {
         // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
         self.map.put(3, Encode::from_int(value, self.zone));
    }

    // Java: boolean isStroke()
    pub fn isStroke(&self) -> bool {
         // Java: if (!this.map.has(4))
        if !self.map.has(4)
         {
            // Java: return this.parent == null ? true : this.parent.isStroke();
            return self.parent.as_ref().map_or(true, |p| p.isStroke());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: return KoBin.Decode.toBool(bin);
            return Decode::toBool(bin.as_ref());
         }
    }

    // Java: boolean hasStroke()
    pub fn hasStroke(&self) -> bool {
         // Java: return this.map.has(4) || this.parent != null && this.parent.hasStroke();
         self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasStroke())
    }

    // Java: void deleteStroke()
    pub fn deleteStroke(&mut self) {
         // Java: this.map.put(4, (KoBin)null);
         self.map.clear_key(4);
    }

    // Java: void putStroke(boolean value)
    pub fn putStroke(&mut self, value: bool) {
         // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
         self.map.put(4, Encode::from_int(value, self.zone));
    }

    // Java: int w()
    pub fn w(&self) -> i32 {
         // Java: if (!this.map.has(5))
        if !self.map.has(5)
         {
            // Java: return this.parent == null ? 0 : this.parent.w();
            return self.parent.as_ref().map_or(0, |p| p.w());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: return KoBin.Decode.toInt(bin);
            return Decode::toInt(bin.as_ref());
         }
    }

    // Java: boolean hasW()
    pub fn hasW(&self) -> bool {
         // Java: return this.map.has(5) || this.parent != null && this.parent.hasW();
         self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasW())
    }

    // Java: void deleteW()
    pub fn deleteW(&mut self) {
         // Java: this.map.put(5, (KoBin)null);
         self.map.clear_key(5);
    }

    // Java: void putW(int value)
    pub fn putW(&mut self, value: i32) {
         // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
         self.map.put(5, Encode::from_int(value, self.zone));
    }

    // Java: PathSegmentList pathSegmentLst()
    pub fn get_pathSegmentLst(&self) -> &dyn KoAr {
         // Java: if (this.pathSegmentLst == null)
        if self.pathSegmentLst.is_none()
         {
            // Java: if (this.map.has(6))
            if self.map.has(6)
            {
               // Java: KoBin bin = this.map.byKey(6);
               let bin = self.map.byKey(6);
               // Java: this.pathSegmentLst = (PathSegmentList)KoBin.Decode.toAny(PathSegmentList.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
            // Java: else if (this.parent != null && this.parent.hasPathSegmentLst())
            else if self.parent != null && self.parent.hasPathSegmentLst()
            {
               // Java: this.pathSegmentLst = this.parent.pathSegmentLst();
               todo!("lazy init requires &mut self")
            }
            // Java: else
            else
            {
               // Java: KoBin bin = KoBin.Null.value1();
               let _bin = Null::value1();
               // Java: this.pathSegmentLst = (PathSegmentList)KoBin.Decode.toAny(PathSegmentList.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
         }
         // Java: return this.pathSegmentLst;
         todo!("lazy init requires &mut self")
    }

    // Java: boolean hasPathSegmentLst()
    pub fn hasPathSegmentLst(&self) -> bool {
         // Java: return this.map.has(6) || this.pathSegmentLst != null && this.pathSegmentLst.dirty() || this.parent != null && this.parent.hasPathSegmentLst();
         self.map.has(6) || self.pathSegmentLst.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPathSegmentLst())
    }

    // Java: void deletePathSegmentLst()
    pub fn deletePathSegmentLst(&mut self) {
         // Java: this.map.put(6, (KoBin)null);
         self.map.clear_key(6);
         // Java: if (this.pathSegmentLst != null)
        if self.pathSegmentLst.is_some()
         {
            // Java: this.pathSegmentLst = null;
            self.pathSegmentLst = None;
         }
    }

    // Java: void writePathSegmentLst(PathSegmentList value)
    pub fn writePathSegmentLst(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = v.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(6, bin);
               self.map.put(6, bin);
               // Java: if (this.pathSegmentLst != null)
                if self.pathSegmentLst.is_some()
               {
                  // Java: this.pathSegmentLst = null;
                  self.pathSegmentLst = None;
               }
            }
         }
    }

    // Java: void forceWritePathSegmentLst(PathSegmentList value)
    pub fn forceWritePathSegmentLst(&mut self, value: Option<&dyn KoAr>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.map.put(6, value.lastBin(this.zone));
            self.map.put(6, v.lastBin(self.zone));
            // Java: if (this.pathSegmentLst != null)
            if self.pathSegmentLst.is_some()
            {
               // Java: this.pathSegmentLst = null;
               self.pathSegmentLst = None;
            }
         }
    }

}

// Java: public class ArcTo
impl ArcTo {
    // Java: static protected static <Value1 extends ArcTo> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
            // Java: KoZone zone = KoZone.enter();
            let zone = KoZone::enter();
            // Java: Value1 inst = (Value1)(zone.allocate(clazz));
            let mut inst = Self::ArcTo_from(bin);
            // Java: inst.zone = zone;
            inst.zone = zone;
            // Java: inst.load(bin);
            inst.load_bin(bin);
            // Java: return inst;
            inst
    }

    // Java: static ArcTo create()
    pub fn create() -> Self {
            // Java: return initialize(ArcTo.class, KoBin.Null.empty());
            let empty = Null::empty();
            Self::initialize(empty.as_ref())
    }

    // Java: static ArcTo open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
            // Java: return initialize(ArcTo.class, bin);
            Self::initialize(bin)
    }

    // Java: ArcTo()
    pub fn new() -> Self {
      Self {
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpMap();
            map: KpMap::new(),
            zone: KoZone::defaultZone(), heightRadius: None, widthRadius: None, startAngle: None, swingAngle: None, parent: None,
            }
    }

    // Java: ArcTo(KoBin bin)
    pub fn ArcTo_from(bin: &dyn KoBinTrait) -> Self {
      Self {
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpMap(bin);
            map: KpMap::KpMap_from(bin),
            zone: KoZone::defaultZone(), heightRadius: None, widthRadius: None, startAngle: None, swingAngle: None, parent: None,
            }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.box.set(bin);
            self.bin_box.set(Some(bin));
            // Java: this.map.load(this.box.get());
            let bin_ref = self.bin_box.get();
            self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
            // Java: this.loadBin(bin, this.zone);
            let zone = self.zone;
            self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(ArcTo value)
    pub fn load_value(&mut self, value: Option<&ArcTo>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.loadBin(value.lastBin(this.zone), this.zone);
               let bin = v.lastBin(self.zone);
               self.loadBin_with_zone(bin.as_ref(), self.zone);
            }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: if (this.heightRadius != null && this.heightRadius.dirty())
        if self.heightRadius.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(1, this.heightRadius.saveBin(zone));
               let saved = self.heightRadius.as_mut().unwrap().saveBin(zone);
               self.map.put(1, saved);
            }
            // Java: if (this.widthRadius != null && this.widthRadius.dirty())
        if self.widthRadius.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(2, this.widthRadius.saveBin(zone));
               let saved = self.widthRadius.as_mut().unwrap().saveBin(zone);
               self.map.put(2, saved);
            }
            // Java: if (this.startAngle != null && this.startAngle.dirty())
        if self.startAngle.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(3, this.startAngle.saveBin(zone));
               let saved = self.startAngle.as_mut().unwrap().saveBin(zone);
               self.map.put(3, saved);
            }
            // Java: if (this.swingAngle != null && this.swingAngle.dirty())
        if self.swingAngle.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(4, this.swingAngle.saveBin(zone));
               let saved = self.swingAngle.as_mut().unwrap().saveBin(zone);
               self.map.put(4, saved);
            }
            // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
            if self.map.dirty() {
               let saved = self.map.save(zone);
               self.bin_box.set(Some(saved.as_ref()));
               self.bin_box.get()
            } else {
               self.bin_box.get()
            }
    }

    // Java: boolean equalsLax(ArcTo o)
    pub fn equalsLax(&self, o: &ArcTo) -> bool {
            // Java: return this.equals(o, false);
            self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ArcTo) -> bool {
            // Java: return this.equals(o, true);
            self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ArcTo, isStrict: bool) -> bool {
            // Java: if (null != o && o instanceof ArcTo)
        if true
            {
          let dst = o;
               // Java: ArcTo dst = (ArcTo)o;
               // Java: if (this == dst)
            if std::ptr::eq(self, dst)
               {
                  // Java: return true;
                  return true;
               }
               // Java: else if ((this.hasHeightRadius() || !dst.hasHeightRadius()) && (!this.hasHeightRadius() || dst.hasHeightRadius()))
            else if (self.hasHeightRadius() || !dst.hasHeightRadius()) && (!self.hasHeightRadius() || dst.hasHeightRadius())
               {
                  // Java: if (this.hasHeightRadius() && dst.hasHeightRadius())
                if self.hasHeightRadius() && dst.hasHeightRadius()
                  {
                     // Java: if (isStrict && !this.heightRadius().equals(dst.heightRadius()))
                    if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.heightRadius().equalsLax(dst.heightRadius()))
                    if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasWidthRadius() || !dst.hasWidthRadius()) && (!this.hasWidthRadius() || dst.hasWidthRadius()))
                if (self.hasWidthRadius() || !dst.hasWidthRadius()) && (!self.hasWidthRadius() || dst.hasWidthRadius())

                  {
                     // Java: if (this.hasWidthRadius() && dst.hasWidthRadius())
                    if self.hasWidthRadius() && dst.hasWidthRadius()
                     {
                        // Java: if (isStrict && !this.widthRadius().equals(dst.widthRadius()))
                        if todo!("KoAr equals/equalsLax")

                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.widthRadius().equalsLax(dst.widthRadius()))
                        if todo!("KoAr equals/equalsLax")

                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasStartAngle() || !dst.hasStartAngle()) && (!this.hasStartAngle() || dst.hasStartAngle()))
                    if (self.hasStartAngle() || !dst.hasStartAngle()) && (!self.hasStartAngle() || dst.hasStartAngle())

                     {
                        // Java: if (this.hasStartAngle() && dst.hasStartAngle())
                        if self.hasStartAngle() && dst.hasStartAngle()
                        {
                           // Java: if (isStrict && !this.startAngle().equals(dst.startAngle()))
                            if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.startAngle().equalsLax(dst.startAngle()))
                            if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: if ((this.hasSwingAngle() || !dst.hasSwingAngle()) && (!this.hasSwingAngle() || dst.hasSwingAngle()))
                        if (self.hasSwingAngle() || !dst.hasSwingAngle()) && (!self.hasSwingAngle() || dst.hasSwingAngle())

                        {
                           // Java: if (this.hasSwingAngle() && dst.hasSwingAngle())
                            if self.hasSwingAngle() && dst.hasSwingAngle()
                           {
                              // Java: if (isStrict && !this.swingAngle().equals(dst.swingAngle()))
                                if todo!("KoAr equals/equalsLax")

                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: if (!isStrict && !this.swingAngle().equalsLax(dst.swingAngle()))
                                if todo!("KoAr equals/equalsLax")

                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: return true;
                           return true;
                        }
                        // Java: else
                        else
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: else
                    else
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
            else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
        else
            {
               // Java: return false;
               return false;
            }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
            // Java: int ret = 0;
            let mut ret: i32 = 0;
            // Java: if (this.hasHeightRadius())
        if self.hasHeightRadius()
            {
               // Java: ret += this.heightRadius().hashCode();
               ret = ret.wrapping_add(todo!("heightRadius hashCode"));
            }
            // Java: if (this.hasWidthRadius())
        if self.hasWidthRadius()
            {
               // Java: ret += this.widthRadius().hashCode();
               ret = ret.wrapping_add(todo!("widthRadius hashCode"));
            }
            // Java: if (this.hasStartAngle())
        if self.hasStartAngle()
            {
               // Java: ret += this.startAngle().hashCode();
               ret = ret.wrapping_add(todo!("startAngle hashCode"));
            }
            // Java: if (this.hasSwingAngle())
        if self.hasSwingAngle()
            {
               // Java: ret += this.swingAngle().hashCode();
               ret = ret.wrapping_add(todo!("swingAngle hashCode"));
            }
            // Java: return ret;
            ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
            // Java: if (this.heightRadius != null && this.heightRadius.dirty())
        if self.heightRadius.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.widthRadius != null && this.widthRadius.dirty())
        else if self.widthRadius.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.startAngle != null && this.startAngle.dirty())
        else if self.startAngle.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else
        else
            {
               // Java: return this.swingAngle != null && this.swingAngle.dirty() ? true : this.map.dirty();
               return self.swingAngle.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
            }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
            // Java: this.saveBin(this.zone);
            let zone = self.zone;
            self.saveBin(zone);
            // Java: return this.box.get();
            self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: return KoBin.Factory.dup(this.box.get(), zone);
            Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
            // Java: this.loadBin(this.box.get(), zone);
            let bin = self.bin_box.get();
            self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanParent();
            self.cleanParent();
            // Java: this.map.load(KoBin.Null.value1());
            let null_bin = Null::value1();
            self.map.load(null_bin.as_ref());
            // Java: this.box.clear();
            self.bin_box.clear();
            // Java: this.zone.leave();
            self.zone.leave();
            // Java: this.zone = KoZone.defaultZone();
            self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
            // Java: this.heightRadius = null;
            self.heightRadius = None;
            // Java: this.widthRadius = null;
            self.widthRadius = None;
            // Java: this.startAngle = null;
            self.startAngle = None;
            // Java: this.swingAngle = null;
            self.swingAngle = None;
    }

    // Java: void follow(ArcTo parent)
    pub fn follow(&mut self, parent: Option<Box<ArcTo>>) {
            // Java: if (parent != null)
        if let Some(mut p) = parent
            {
               // Java: parent.discard();
               p.discard();
               // Java: this.discard();
               self.discard();
               // Java: this.parent = parent;
               self.parent = Some(p);
            }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
            // Java: if (this.parent != null)
        if self.parent.is_some()
            {
               // Java: this.discard();
               self.discard();
               // Java: this.parent.discard();
               if let Some(ref mut p) = self.parent { p.discard(); }
               // Java: this.cleanParent();
               self.cleanParent();
            }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
            // Java: return this.parent != null;
            self.parent.is_some()
    }

    // Java: ArcTo parent()
    pub fn get_parent(&self) -> Option<&ArcTo> {
            // Java: return this.parent;
            self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
            // Java: this.discard(this.zone);
            let zone = self.zone;
            self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
            // Java: this.parent = null;
            self.parent = None;
    }

    // Java: AdjCoordinate heightRadius()
    pub fn get_heightRadius(&self) -> &dyn KoAr {
            // Java: if (this.heightRadius == null)
        if self.heightRadius.is_none()
            {
               // Java: if (this.map.has(1))
            if self.map.has(1)
               {
                  // Java: KoBin bin = this.map.byKey(1);
                  let bin = self.map.byKey(1);
                  // Java: this.heightRadius = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasHeightRadius())
            else if self.parent != null && self.parent.hasHeightRadius()
               {
                  // Java: this.heightRadius = this.parent.heightRadius();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.heightRadius = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.heightRadius;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasHeightRadius()
    pub fn hasHeightRadius(&self) -> bool {
            // Java: return this.map.has(1) || this.heightRadius != null && this.heightRadius.dirty() || this.parent != null && this.parent.hasHeightRadius();
            self.map.has(1) || self.heightRadius.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasHeightRadius())
    }

    // Java: void deleteHeightRadius()
    pub fn deleteHeightRadius(&mut self) {
            // Java: this.map.put(1, (KoBin)null);
            self.map.clear_key(1);
            // Java: if (this.heightRadius != null)
        if self.heightRadius.is_some()
            {
               // Java: this.heightRadius = null;
               self.heightRadius = None;
            }
    }

    // Java: void writeHeightRadius(AdjCoordinate value)
    pub fn writeHeightRadius(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(1, bin);
                  self.map.put(1, bin);
                  // Java: if (this.heightRadius != null)
                if self.heightRadius.is_some()
                  {
                     // Java: this.heightRadius = null;
                     self.heightRadius = None;
                  }
               }
            }
    }

    // Java: void forceWriteHeightRadius(AdjCoordinate value)
    pub fn forceWriteHeightRadius(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(1, value.lastBin(this.zone));
               self.map.put(1, v.lastBin(self.zone));
               // Java: if (this.heightRadius != null)
            if self.heightRadius.is_some()
               {
                  // Java: this.heightRadius = null;
                  self.heightRadius = None;
               }
            }
    }

    // Java: AdjCoordinate widthRadius()
    pub fn get_widthRadius(&self) -> &dyn KoAr {
            // Java: if (this.widthRadius == null)
        if self.widthRadius.is_none()
            {
               // Java: if (this.map.has(2))
            if self.map.has(2)
               {
                  // Java: KoBin bin = this.map.byKey(2);
                  let bin = self.map.byKey(2);
                  // Java: this.widthRadius = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasWidthRadius())
            else if self.parent != null && self.parent.hasWidthRadius()
               {
                  // Java: this.widthRadius = this.parent.widthRadius();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.widthRadius = (AdjCoordinate)KoBin.Decode.toAny(AdjCoordinate.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.widthRadius;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasWidthRadius()
    pub fn hasWidthRadius(&self) -> bool {
            // Java: return this.map.has(2) || this.widthRadius != null && this.widthRadius.dirty() || this.parent != null && this.parent.hasWidthRadius();
            self.map.has(2) || self.widthRadius.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasWidthRadius())
    }

    // Java: void deleteWidthRadius()
    pub fn deleteWidthRadius(&mut self) {
            // Java: this.map.put(2, (KoBin)null);
            self.map.clear_key(2);
            // Java: if (this.widthRadius != null)
        if self.widthRadius.is_some()
            {
               // Java: this.widthRadius = null;
               self.widthRadius = None;
            }
    }

    // Java: void writeWidthRadius(AdjCoordinate value)
    pub fn writeWidthRadius(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(2, bin);
                  self.map.put(2, bin);
                  // Java: if (this.widthRadius != null)
                if self.widthRadius.is_some()
                  {
                     // Java: this.widthRadius = null;
                     self.widthRadius = None;
                  }
               }
            }
    }

    // Java: void forceWriteWidthRadius(AdjCoordinate value)
    pub fn forceWriteWidthRadius(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(2, value.lastBin(this.zone));
               self.map.put(2, v.lastBin(self.zone));
               // Java: if (this.widthRadius != null)
            if self.widthRadius.is_some()
               {
                  // Java: this.widthRadius = null;
                  self.widthRadius = None;
               }
            }
    }

    // Java: AdjAngle startAngle()
    pub fn get_startAngle(&self) -> &dyn KoAr {
            // Java: if (this.startAngle == null)
        if self.startAngle.is_none()
            {
               // Java: if (this.map.has(3))
            if self.map.has(3)
               {
                  // Java: KoBin bin = this.map.byKey(3);
                  let bin = self.map.byKey(3);
                  // Java: this.startAngle = (AdjAngle)KoBin.Decode.toAny(AdjAngle.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasStartAngle())
            else if self.parent != null && self.parent.hasStartAngle()
               {
                  // Java: this.startAngle = this.parent.startAngle();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.startAngle = (AdjAngle)KoBin.Decode.toAny(AdjAngle.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.startAngle;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasStartAngle()
    pub fn hasStartAngle(&self) -> bool {
            // Java: return this.map.has(3) || this.startAngle != null && this.startAngle.dirty() || this.parent != null && this.parent.hasStartAngle();
            self.map.has(3) || self.startAngle.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasStartAngle())
    }

    // Java: void deleteStartAngle()
    pub fn deleteStartAngle(&mut self) {
            // Java: this.map.put(3, (KoBin)null);
            self.map.clear_key(3);
            // Java: if (this.startAngle != null)
        if self.startAngle.is_some()
            {
               // Java: this.startAngle = null;
               self.startAngle = None;
            }
    }

    // Java: void writeStartAngle(AdjAngle value)
    pub fn writeStartAngle(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(3, bin);
                  self.map.put(3, bin);
                  // Java: if (this.startAngle != null)
                if self.startAngle.is_some()
                  {
                     // Java: this.startAngle = null;
                     self.startAngle = None;
                  }
               }
            }
    }

    // Java: void forceWriteStartAngle(AdjAngle value)
    pub fn forceWriteStartAngle(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(3, value.lastBin(this.zone));
               self.map.put(3, v.lastBin(self.zone));
               // Java: if (this.startAngle != null)
            if self.startAngle.is_some()
               {
                  // Java: this.startAngle = null;
                  self.startAngle = None;
               }
            }
    }

    // Java: AdjAngle swingAngle()
    pub fn get_swingAngle(&self) -> &dyn KoAr {
            // Java: if (this.swingAngle == null)
        if self.swingAngle.is_none()
            {
               // Java: if (this.map.has(4))
            if self.map.has(4)
               {
                  // Java: KoBin bin = this.map.byKey(4);
                  let bin = self.map.byKey(4);
                  // Java: this.swingAngle = (AdjAngle)KoBin.Decode.toAny(AdjAngle.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasSwingAngle())
            else if self.parent != null && self.parent.hasSwingAngle()
               {
                  // Java: this.swingAngle = this.parent.swingAngle();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.swingAngle = (AdjAngle)KoBin.Decode.toAny(AdjAngle.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.swingAngle;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasSwingAngle()
    pub fn hasSwingAngle(&self) -> bool {
            // Java: return this.map.has(4) || this.swingAngle != null && this.swingAngle.dirty() || this.parent != null && this.parent.hasSwingAngle();
            self.map.has(4) || self.swingAngle.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSwingAngle())
    }

    // Java: void deleteSwingAngle()
    pub fn deleteSwingAngle(&mut self) {
            // Java: this.map.put(4, (KoBin)null);
            self.map.clear_key(4);
            // Java: if (this.swingAngle != null)
        if self.swingAngle.is_some()
            {
               // Java: this.swingAngle = null;
               self.swingAngle = None;
            }
    }

    // Java: void writeSwingAngle(AdjAngle value)
    pub fn writeSwingAngle(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(4, bin);
                  self.map.put(4, bin);
                  // Java: if (this.swingAngle != null)
                if self.swingAngle.is_some()
                  {
                     // Java: this.swingAngle = null;
                     self.swingAngle = None;
                  }
               }
            }
    }

    // Java: void forceWriteSwingAngle(AdjAngle value)
    pub fn forceWriteSwingAngle(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(4, value.lastBin(this.zone));
               self.map.put(4, v.lastBin(self.zone));
               // Java: if (this.swingAngle != null)
            if self.swingAngle.is_some()
               {
                  // Java: this.swingAngle = null;
                  self.swingAngle = None;
               }
            }
    }

}

// Java: public class CubicBezierCurveTo
impl CubicBezierCurveTo {
    // Java: static protected static <Value1 extends CubicBezierCurveTo> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
            // Java: KoZone zone = KoZone.enter();
            let zone = KoZone::enter();
            // Java: Value1 inst = (Value1)(zone.allocate(clazz));
            let mut inst = Self::CubicBezierCurveTo_from(bin);
            // Java: inst.zone = zone;
            inst.zone = zone;
            // Java: inst.load(bin);
            inst.load_bin(bin);
            // Java: return inst;
            inst
    }

    // Java: static CubicBezierCurveTo create()
    pub fn create() -> Self {
            // Java: return initialize(CubicBezierCurveTo.class, KoBin.Null.empty());
            let empty = Null::empty();
            Self::initialize(empty.as_ref())
    }

    // Java: static CubicBezierCurveTo open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
            // Java: return initialize(CubicBezierCurveTo.class, bin);
            Self::initialize(bin)
    }

    // Java: CubicBezierCurveTo()
    pub fn new() -> Self {
      Self {
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpMap();
            map: KpMap::new(),
            zone: KoZone::defaultZone(), first: None, second: None, third: None, parent: None,
            }
    }

    // Java: CubicBezierCurveTo(KoBin bin)
    pub fn CubicBezierCurveTo_from(bin: &dyn KoBinTrait) -> Self {
      Self {
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpMap(bin);
            map: KpMap::KpMap_from(bin),
            zone: KoZone::defaultZone(), first: None, second: None, third: None, parent: None,
            }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.box.set(bin);
            self.bin_box.set(Some(bin));
            // Java: this.map.load(this.box.get());
            let bin_ref = self.bin_box.get();
            self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
            // Java: this.loadBin(bin, this.zone);
            let zone = self.zone;
            self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(CubicBezierCurveTo value)
    pub fn load_value(&mut self, value: Option<&CubicBezierCurveTo>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.loadBin(value.lastBin(this.zone), this.zone);
               let bin = v.lastBin(self.zone);
               self.loadBin_with_zone(bin.as_ref(), self.zone);
            }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: if (this.first != null && this.first.dirty())
        if self.first.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(1, this.first.saveBin(zone));
               let saved = self.first.as_mut().unwrap().saveBin(zone);
               self.map.put(1, saved);
            }
            // Java: if (this.second != null && this.second.dirty())
        if self.second.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(2, this.second.saveBin(zone));
               let saved = self.second.as_mut().unwrap().saveBin(zone);
               self.map.put(2, saved);
            }
            // Java: if (this.third != null && this.third.dirty())
        if self.third.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(3, this.third.saveBin(zone));
               let saved = self.third.as_mut().unwrap().saveBin(zone);
               self.map.put(3, saved);
            }
            // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
            if self.map.dirty() {
               let saved = self.map.save(zone);
               self.bin_box.set(Some(saved.as_ref()));
               self.bin_box.get()
            } else {
               self.bin_box.get()
            }
    }

    // Java: boolean equalsLax(CubicBezierCurveTo o)
    pub fn equalsLax(&self, o: &CubicBezierCurveTo) -> bool {
            // Java: return this.equals(o, false);
            self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &CubicBezierCurveTo) -> bool {
            // Java: return this.equals(o, true);
            self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &CubicBezierCurveTo, isStrict: bool) -> bool {
            // Java: if (null != o && o instanceof CubicBezierCurveTo)
        if true
            {
          let dst = o;
               // Java: CubicBezierCurveTo dst = (CubicBezierCurveTo)o;
               // Java: if (this == dst)
            if std::ptr::eq(self, dst)
               {
                  // Java: return true;
                  return true;
               }
               // Java: else if ((this.hasFirst() || !dst.hasFirst()) && (!this.hasFirst() || dst.hasFirst()))
            else if (self.hasFirst() || !dst.hasFirst()) && (!self.hasFirst() || dst.hasFirst())
               {
                  // Java: if (this.hasFirst() && dst.hasFirst())
                if self.hasFirst() && dst.hasFirst()
                  {
                     // Java: if (isStrict && !this.first().equals(dst.first()))
                    if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.first().equalsLax(dst.first()))
                    if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasSecond() || !dst.hasSecond()) && (!this.hasSecond() || dst.hasSecond()))
                if (self.hasSecond() || !dst.hasSecond()) && (!self.hasSecond() || dst.hasSecond())

                  {
                     // Java: if (this.hasSecond() && dst.hasSecond())
                    if self.hasSecond() && dst.hasSecond()
                     {
                        // Java: if (isStrict && !this.second().equals(dst.second()))
                        if todo!("KoAr equals/equalsLax")

                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.second().equalsLax(dst.second()))
                        if todo!("KoAr equals/equalsLax")

                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasThird() || !dst.hasThird()) && (!this.hasThird() || dst.hasThird()))
                    if (self.hasThird() || !dst.hasThird()) && (!self.hasThird() || dst.hasThird())

                     {
                        // Java: if (this.hasThird() && dst.hasThird())
                        if self.hasThird() && dst.hasThird()
                        {
                           // Java: if (isStrict && !this.third().equals(dst.third()))
                            if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.third().equalsLax(dst.third()))
                            if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: return true;
                        return true;
                     }
                     // Java: else
                    else
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
            else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
        else
            {
               // Java: return false;
               return false;
            }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
            // Java: int ret = 0;
            let mut ret: i32 = 0;
            // Java: if (this.hasFirst())
        if self.hasFirst()
            {
               // Java: ret += this.first().hashCode();
               ret = ret.wrapping_add(todo!("first hashCode"));
            }
            // Java: if (this.hasSecond())
        if self.hasSecond()
            {
               // Java: ret += this.second().hashCode();
               ret = ret.wrapping_add(todo!("second hashCode"));
            }
            // Java: if (this.hasThird())
        if self.hasThird()
            {
               // Java: ret += this.third().hashCode();
               ret = ret.wrapping_add(todo!("third hashCode"));
            }
            // Java: return ret;
            ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
            // Java: if (this.first != null && this.first.dirty())
        if self.first.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.second != null && this.second.dirty())
        else if self.second.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else
        else
            {
               // Java: return this.third != null && this.third.dirty() ? true : this.map.dirty();
               return self.third.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
            }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
            // Java: this.saveBin(this.zone);
            let zone = self.zone;
            self.saveBin(zone);
            // Java: return this.box.get();
            self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: return KoBin.Factory.dup(this.box.get(), zone);
            Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
            // Java: this.loadBin(this.box.get(), zone);
            let bin = self.bin_box.get();
            self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanParent();
            self.cleanParent();
            // Java: this.map.load(KoBin.Null.value1());
            let null_bin = Null::value1();
            self.map.load(null_bin.as_ref());
            // Java: this.box.clear();
            self.bin_box.clear();
            // Java: this.zone.leave();
            self.zone.leave();
            // Java: this.zone = KoZone.defaultZone();
            self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
            // Java: this.first = null;
            self.first = None;
            // Java: this.second = null;
            self.second = None;
            // Java: this.third = null;
            self.third = None;
    }

    // Java: void follow(CubicBezierCurveTo parent)
    pub fn follow(&mut self, parent: Option<Box<CubicBezierCurveTo>>) {
            // Java: if (parent != null)
        if let Some(mut p) = parent
            {
               // Java: parent.discard();
               p.discard();
               // Java: this.discard();
               self.discard();
               // Java: this.parent = parent;
               self.parent = Some(p);
            }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
            // Java: if (this.parent != null)
        if self.parent.is_some()
            {
               // Java: this.discard();
               self.discard();
               // Java: this.parent.discard();
               if let Some(ref mut p) = self.parent { p.discard(); }
               // Java: this.cleanParent();
               self.cleanParent();
            }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
            // Java: return this.parent != null;
            self.parent.is_some()
    }

    // Java: CubicBezierCurveTo parent()
    pub fn get_parent(&self) -> Option<&CubicBezierCurveTo> {
            // Java: return this.parent;
            self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
            // Java: this.discard(this.zone);
            let zone = self.zone;
            self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
            // Java: this.parent = null;
            self.parent = None;
    }

    // Java: AdjPoint2D first()
    pub fn get_first(&self) -> &dyn KoAr {
            // Java: if (this.first == null)
        if self.first.is_none()
            {
               // Java: if (this.map.has(1))
            if self.map.has(1)
               {
                  // Java: KoBin bin = this.map.byKey(1);
                  let bin = self.map.byKey(1);
                  // Java: this.first = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasFirst())
            else if self.parent != null && self.parent.hasFirst()
               {
                  // Java: this.first = this.parent.first();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.first = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.first;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasFirst()
    pub fn hasFirst(&self) -> bool {
            // Java: return this.map.has(1) || this.first != null && this.first.dirty() || this.parent != null && this.parent.hasFirst();
            self.map.has(1) || self.first.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFirst())
    }

    // Java: void deleteFirst()
    pub fn deleteFirst(&mut self) {
            // Java: this.map.put(1, (KoBin)null);
            self.map.clear_key(1);
            // Java: if (this.first != null)
        if self.first.is_some()
            {
               // Java: this.first = null;
               self.first = None;
            }
    }

    // Java: void writeFirst(AdjPoint2D value)
    pub fn writeFirst(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(1, bin);
                  self.map.put(1, bin);
                  // Java: if (this.first != null)
                if self.first.is_some()
                  {
                     // Java: this.first = null;
                     self.first = None;
                  }
               }
            }
    }

    // Java: void forceWriteFirst(AdjPoint2D value)
    pub fn forceWriteFirst(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(1, value.lastBin(this.zone));
               self.map.put(1, v.lastBin(self.zone));
               // Java: if (this.first != null)
            if self.first.is_some()
               {
                  // Java: this.first = null;
                  self.first = None;
               }
            }
    }

    // Java: AdjPoint2D second()
    pub fn get_second(&self) -> &dyn KoAr {
            // Java: if (this.second == null)
        if self.second.is_none()
            {
               // Java: if (this.map.has(2))
            if self.map.has(2)
               {
                  // Java: KoBin bin = this.map.byKey(2);
                  let bin = self.map.byKey(2);
                  // Java: this.second = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasSecond())
            else if self.parent != null && self.parent.hasSecond()
               {
                  // Java: this.second = this.parent.second();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.second = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.second;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasSecond()
    pub fn hasSecond(&self) -> bool {
            // Java: return this.map.has(2) || this.second != null && this.second.dirty() || this.parent != null && this.parent.hasSecond();
            self.map.has(2) || self.second.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSecond())
    }

    // Java: void deleteSecond()
    pub fn deleteSecond(&mut self) {
            // Java: this.map.put(2, (KoBin)null);
            self.map.clear_key(2);
            // Java: if (this.second != null)
        if self.second.is_some()
            {
               // Java: this.second = null;
               self.second = None;
            }
    }

    // Java: void writeSecond(AdjPoint2D value)
    pub fn writeSecond(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(2, bin);
                  self.map.put(2, bin);
                  // Java: if (this.second != null)
                if self.second.is_some()
                  {
                     // Java: this.second = null;
                     self.second = None;
                  }
               }
            }
    }

    // Java: void forceWriteSecond(AdjPoint2D value)
    pub fn forceWriteSecond(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(2, value.lastBin(this.zone));
               self.map.put(2, v.lastBin(self.zone));
               // Java: if (this.second != null)
            if self.second.is_some()
               {
                  // Java: this.second = null;
                  self.second = None;
               }
            }
    }

    // Java: AdjPoint2D third()
    pub fn get_third(&self) -> &dyn KoAr {
            // Java: if (this.third == null)
        if self.third.is_none()
            {
               // Java: if (this.map.has(3))
            if self.map.has(3)
               {
                  // Java: KoBin bin = this.map.byKey(3);
                  let bin = self.map.byKey(3);
                  // Java: this.third = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasThird())
            else if self.parent != null && self.parent.hasThird()
               {
                  // Java: this.third = this.parent.third();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.third = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.third;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasThird()
    pub fn hasThird(&self) -> bool {
            // Java: return this.map.has(3) || this.third != null && this.third.dirty() || this.parent != null && this.parent.hasThird();
            self.map.has(3) || self.third.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasThird())
    }

    // Java: void deleteThird()
    pub fn deleteThird(&mut self) {
            // Java: this.map.put(3, (KoBin)null);
            self.map.clear_key(3);
            // Java: if (this.third != null)
        if self.third.is_some()
            {
               // Java: this.third = null;
               self.third = None;
            }
    }

    // Java: void writeThird(AdjPoint2D value)
    pub fn writeThird(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(3, bin);
                  self.map.put(3, bin);
                  // Java: if (this.third != null)
                if self.third.is_some()
                  {
                     // Java: this.third = null;
                     self.third = None;
                  }
               }
            }
    }

    // Java: void forceWriteThird(AdjPoint2D value)
    pub fn forceWriteThird(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(3, value.lastBin(this.zone));
               self.map.put(3, v.lastBin(self.zone));
               // Java: if (this.third != null)
            if self.third.is_some()
               {
                  // Java: this.third = null;
                  self.third = None;
               }
            }
    }

}

// Java: public class QuadraticBezierCurveTo
impl QuadraticBezierCurveTo {
    // Java: static protected static <Value1 extends QuadraticBezierCurveTo> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
            // Java: KoZone zone = KoZone.enter();
            let zone = KoZone::enter();
            // Java: Value1 inst = (Value1)(zone.allocate(clazz));
            let mut inst = Self::QuadraticBezierCurveTo_from(bin);
            // Java: inst.zone = zone;
            inst.zone = zone;
            // Java: inst.load(bin);
            inst.load_bin(bin);
            // Java: return inst;
            inst
    }

    // Java: static QuadraticBezierCurveTo create()
    pub fn create() -> Self {
            // Java: return initialize(QuadraticBezierCurveTo.class, KoBin.Null.empty());
            let empty = Null::empty();
            Self::initialize(empty.as_ref())
    }

    // Java: static QuadraticBezierCurveTo open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
            // Java: return initialize(QuadraticBezierCurveTo.class, bin);
            Self::initialize(bin)
    }

    // Java: QuadraticBezierCurveTo()
    pub fn new() -> Self {
      Self {
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpMap();
            map: KpMap::new(),
            zone: KoZone::defaultZone(), first: None, second: None, parent: None,
            }
    }

    // Java: QuadraticBezierCurveTo(KoBin bin)
    pub fn QuadraticBezierCurveTo_from(bin: &dyn KoBinTrait) -> Self {
      Self {
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpMap(bin);
            map: KpMap::KpMap_from(bin),
            zone: KoZone::defaultZone(), first: None, second: None, parent: None,
            }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.box.set(bin);
            self.bin_box.set(Some(bin));
            // Java: this.map.load(this.box.get());
            let bin_ref = self.bin_box.get();
            self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
            // Java: this.loadBin(bin, this.zone);
            let zone = self.zone;
            self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(QuadraticBezierCurveTo value)
    pub fn load_value(&mut self, value: Option<&QuadraticBezierCurveTo>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.loadBin(value.lastBin(this.zone), this.zone);
               let bin = v.lastBin(self.zone);
               self.loadBin_with_zone(bin.as_ref(), self.zone);
            }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: if (this.first != null && this.first.dirty())
        if self.first.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(1, this.first.saveBin(zone));
               let saved = self.first.as_mut().unwrap().saveBin(zone);
               self.map.put(1, saved);
            }
            // Java: if (this.second != null && this.second.dirty())
        if self.second.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(2, this.second.saveBin(zone));
               let saved = self.second.as_mut().unwrap().saveBin(zone);
               self.map.put(2, saved);
            }
            // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
            if self.map.dirty() {
               let saved = self.map.save(zone);
               self.bin_box.set(Some(saved.as_ref()));
               self.bin_box.get()
            } else {
               self.bin_box.get()
            }
    }

    // Java: boolean equalsLax(QuadraticBezierCurveTo o)
    pub fn equalsLax(&self, o: &QuadraticBezierCurveTo) -> bool {
            // Java: return this.equals(o, false);
            self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &QuadraticBezierCurveTo) -> bool {
            // Java: return this.equals(o, true);
            self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &QuadraticBezierCurveTo, isStrict: bool) -> bool {
            // Java: if (null != o && o instanceof QuadraticBezierCurveTo)
        if true
            {
          let dst = o;
               // Java: QuadraticBezierCurveTo dst = (QuadraticBezierCurveTo)o;
               // Java: if (this == dst)
            if std::ptr::eq(self, dst)
               {
                  // Java: return true;
                  return true;
               }
               // Java: else if ((this.hasFirst() || !dst.hasFirst()) && (!this.hasFirst() || dst.hasFirst()))
            else if (self.hasFirst() || !dst.hasFirst()) && (!self.hasFirst() || dst.hasFirst())
               {
                  // Java: if (this.hasFirst() && dst.hasFirst())
                if self.hasFirst() && dst.hasFirst()
                  {
                     // Java: if (isStrict && !this.first().equals(dst.first()))
                    if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.first().equalsLax(dst.first()))
                    if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasSecond() || !dst.hasSecond()) && (!this.hasSecond() || dst.hasSecond()))
                if (self.hasSecond() || !dst.hasSecond()) && (!self.hasSecond() || dst.hasSecond())

                  {
                     // Java: if (this.hasSecond() && dst.hasSecond())
                    if self.hasSecond() && dst.hasSecond()
                     {
                        // Java: if (isStrict && !this.second().equals(dst.second()))
                        if todo!("KoAr equals/equalsLax")

                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.second().equalsLax(dst.second()))
                        if todo!("KoAr equals/equalsLax")

                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: return true;
                     return true;
                  }
                  // Java: else
                else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
            else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
        else
            {
               // Java: return false;
               return false;
            }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
            // Java: int ret = 0;
            let mut ret: i32 = 0;
            // Java: if (this.hasFirst())
        if self.hasFirst()
            {
               // Java: ret += this.first().hashCode();
               ret = ret.wrapping_add(todo!("first hashCode"));
            }
            // Java: if (this.hasSecond())
        if self.hasSecond()
            {
               // Java: ret += this.second().hashCode();
               ret = ret.wrapping_add(todo!("second hashCode"));
            }
            // Java: return ret;
            ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
            // Java: if (this.first != null && this.first.dirty())
        if self.first.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else
        else
            {
               // Java: return this.second != null && this.second.dirty() ? true : this.map.dirty();
               return self.second.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
            }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
            // Java: this.saveBin(this.zone);
            let zone = self.zone;
            self.saveBin(zone);
            // Java: return this.box.get();
            self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: return KoBin.Factory.dup(this.box.get(), zone);
            Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
            // Java: this.loadBin(this.box.get(), zone);
            let bin = self.bin_box.get();
            self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanParent();
            self.cleanParent();
            // Java: this.map.load(KoBin.Null.value1());
            let null_bin = Null::value1();
            self.map.load(null_bin.as_ref());
            // Java: this.box.clear();
            self.bin_box.clear();
            // Java: this.zone.leave();
            self.zone.leave();
            // Java: this.zone = KoZone.defaultZone();
            self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
            // Java: this.first = null;
            self.first = None;
            // Java: this.second = null;
            self.second = None;
    }

    // Java: void follow(QuadraticBezierCurveTo parent)
    pub fn follow(&mut self, parent: Option<Box<QuadraticBezierCurveTo>>) {
            // Java: if (parent != null)
        if let Some(mut p) = parent
            {
               // Java: parent.discard();
               p.discard();
               // Java: this.discard();
               self.discard();
               // Java: this.parent = parent;
               self.parent = Some(p);
            }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
            // Java: if (this.parent != null)
        if self.parent.is_some()
            {
               // Java: this.discard();
               self.discard();
               // Java: this.parent.discard();
               if let Some(ref mut p) = self.parent { p.discard(); }
               // Java: this.cleanParent();
               self.cleanParent();
            }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
            // Java: return this.parent != null;
            self.parent.is_some()
    }

    // Java: QuadraticBezierCurveTo parent()
    pub fn get_parent(&self) -> Option<&QuadraticBezierCurveTo> {
            // Java: return this.parent;
            self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
            // Java: this.discard(this.zone);
            let zone = self.zone;
            self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
            // Java: this.parent = null;
            self.parent = None;
    }

    // Java: AdjPoint2D first()
    pub fn get_first(&self) -> &dyn KoAr {
            // Java: if (this.first == null)
        if self.first.is_none()
            {
               // Java: if (this.map.has(1))
            if self.map.has(1)
               {
                  // Java: KoBin bin = this.map.byKey(1);
                  let bin = self.map.byKey(1);
                  // Java: this.first = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasFirst())
            else if self.parent != null && self.parent.hasFirst()
               {
                  // Java: this.first = this.parent.first();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.first = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.first;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasFirst()
    pub fn hasFirst(&self) -> bool {
            // Java: return this.map.has(1) || this.first != null && this.first.dirty() || this.parent != null && this.parent.hasFirst();
            self.map.has(1) || self.first.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFirst())
    }

    // Java: void deleteFirst()
    pub fn deleteFirst(&mut self) {
            // Java: this.map.put(1, (KoBin)null);
            self.map.clear_key(1);
            // Java: if (this.first != null)
        if self.first.is_some()
            {
               // Java: this.first = null;
               self.first = None;
            }
    }

    // Java: void writeFirst(AdjPoint2D value)
    pub fn writeFirst(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(1, bin);
                  self.map.put(1, bin);
                  // Java: if (this.first != null)
                if self.first.is_some()
                  {
                     // Java: this.first = null;
                     self.first = None;
                  }
               }
            }
    }

    // Java: void forceWriteFirst(AdjPoint2D value)
    pub fn forceWriteFirst(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(1, value.lastBin(this.zone));
               self.map.put(1, v.lastBin(self.zone));
               // Java: if (this.first != null)
            if self.first.is_some()
               {
                  // Java: this.first = null;
                  self.first = None;
               }
            }
    }

    // Java: AdjPoint2D second()
    pub fn get_second(&self) -> &dyn KoAr {
            // Java: if (this.second == null)
        if self.second.is_none()
            {
               // Java: if (this.map.has(2))
            if self.map.has(2)
               {
                  // Java: KoBin bin = this.map.byKey(2);
                  let bin = self.map.byKey(2);
                  // Java: this.second = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasSecond())
            else if self.parent != null && self.parent.hasSecond()
               {
                  // Java: this.second = this.parent.second();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.second = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.second;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasSecond()
    pub fn hasSecond(&self) -> bool {
            // Java: return this.map.has(2) || this.second != null && this.second.dirty() || this.parent != null && this.parent.hasSecond();
            self.map.has(2) || self.second.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSecond())
    }

    // Java: void deleteSecond()
    pub fn deleteSecond(&mut self) {
            // Java: this.map.put(2, (KoBin)null);
            self.map.clear_key(2);
            // Java: if (this.second != null)
        if self.second.is_some()
            {
               // Java: this.second = null;
               self.second = None;
            }
    }

    // Java: void writeSecond(AdjPoint2D value)
    pub fn writeSecond(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(2, bin);
                  self.map.put(2, bin);
                  // Java: if (this.second != null)
                if self.second.is_some()
                  {
                     // Java: this.second = null;
                     self.second = None;
                  }
               }
            }
    }

    // Java: void forceWriteSecond(AdjPoint2D value)
    pub fn forceWriteSecond(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(2, value.lastBin(this.zone));
               self.map.put(2, v.lastBin(self.zone));
               // Java: if (this.second != null)
            if self.second.is_some()
               {
                  // Java: this.second = null;
                  self.second = None;
               }
            }
    }

}

// Java: public class PathSegment
impl PathSegment {
    // Java: static protected static <Value1 extends PathSegment> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
            // Java: KoZone zone = KoZone.enter();
            let zone = KoZone::enter();
            // Java: Value1 inst = (Value1)(zone.allocate(clazz));
            let mut inst = Self::PathSegment_from(bin);
            // Java: inst.zone = zone;
            inst.zone = zone;
            // Java: inst.load(bin);
            inst.load_bin(bin);
            // Java: return inst;
            inst
    }

    // Java: static PathSegment create()
    pub fn create() -> Self {
            // Java: return initialize(PathSegment.class, KoBin.Null.empty());
            let empty = Null::empty();
            Self::initialize(empty.as_ref())
    }

    // Java: static PathSegment open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
            // Java: return initialize(PathSegment.class, bin);
            Self::initialize(bin)
    }

    // Java: PathSegment()
    pub fn new() -> Self {
      Self {
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpMap();
            map: KpMap::new(),
            zone: KoZone::defaultZone(), moveTo: None, lnTo: None, arcTo: None, quadBezTo: None, cubicBezTo: None, parent: None,
            }
    }

    // Java: PathSegment(KoBin bin)
    pub fn PathSegment_from(bin: &dyn KoBinTrait) -> Self {
      Self {
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpMap(bin);
            map: KpMap::KpMap_from(bin),
            zone: KoZone::defaultZone(), moveTo: None, lnTo: None, arcTo: None, quadBezTo: None, cubicBezTo: None, parent: None,
            }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.box.set(bin);
            self.bin_box.set(Some(bin));
            // Java: this.map.load(this.box.get());
            let bin_ref = self.bin_box.get();
            self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
            // Java: this.loadBin(bin, this.zone);
            let zone = self.zone;
            self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(PathSegment value)
    pub fn load_value(&mut self, value: Option<&PathSegment>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.loadBin(value.lastBin(this.zone), this.zone);
               let bin = v.lastBin(self.zone);
               self.loadBin_with_zone(bin.as_ref(), self.zone);
            }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: if (this.moveTo != null && this.moveTo.dirty())
        if self.moveTo.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(3, this.moveTo.saveBin(zone));
               let saved = self.moveTo.as_mut().unwrap().saveBin(zone);
               self.map.put(3, saved);
            }
            // Java: if (this.lnTo != null && this.lnTo.dirty())
        if self.lnTo.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(4, this.lnTo.saveBin(zone));
               let saved = self.lnTo.as_mut().unwrap().saveBin(zone);
               self.map.put(4, saved);
            }
            // Java: if (this.arcTo != null && this.arcTo.dirty())
        if self.arcTo.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(5, this.arcTo.saveBin(zone));
               let saved = self.arcTo.as_mut().unwrap().saveBin(zone);
               self.map.put(5, saved);
            }
            // Java: if (this.quadBezTo != null && this.quadBezTo.dirty())
        if self.quadBezTo.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(6, this.quadBezTo.saveBin(zone));
               let saved = self.quadBezTo.as_mut().unwrap().saveBin(zone);
               self.map.put(6, saved);
            }
            // Java: if (this.cubicBezTo != null && this.cubicBezTo.dirty())
        if self.cubicBezTo.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: this.map.put(7, this.cubicBezTo.saveBin(zone));
               let saved = self.cubicBezTo.as_mut().unwrap().saveBin(zone);
               self.map.put(7, saved);
            }
            // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
            if self.map.dirty() {
               let saved = self.map.save(zone);
               self.bin_box.set(Some(saved.as_ref()));
               self.bin_box.get()
            } else {
               self.bin_box.get()
            }
    }

    // Java: boolean equalsLax(PathSegment o)
    pub fn equalsLax(&self, o: &PathSegment) -> bool {
            // Java: return this.equals(o, false);
            self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &PathSegment) -> bool {
            // Java: return this.equals(o, true);
            self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &PathSegment, isStrict: bool) -> bool {
            // Java: if (null != o && o instanceof PathSegment)
        if true
            {
          let dst = o;
               // Java: PathSegment dst = (PathSegment)o;
               // Java: if (this == dst)
            if std::ptr::eq(self, dst)
               {
                  // Java: return true;
                  return true;
               }
               // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
            else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
               {
                  // Java: if (this.hasType() && dst.hasType() && this.type() != dst.type())
                if self.hasType() && dst.hasType() && self.get_type() != dst.get_type()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasClose() || !dst.hasClose()) && (!this.hasClose() || dst.hasClose()))
                else if (self.hasClose() || !dst.hasClose()) && (!self.hasClose() || dst.hasClose())
                  {
                     // Java: if (this.hasClose() && dst.hasClose() && this.isClose() != dst.isClose())
                    if self.hasClose() && dst.hasClose() && self.isClose() != dst.isClose()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasMoveTo() || !dst.hasMoveTo()) && (!this.hasMoveTo() || dst.hasMoveTo()))
                    else if (self.hasMoveTo() || !dst.hasMoveTo()) && (!self.hasMoveTo() || dst.hasMoveTo())
                     {
                        // Java: if (this.hasMoveTo() && dst.hasMoveTo())
                        if self.hasMoveTo() && dst.hasMoveTo()
                        {
                           // Java: if (isStrict && !this.moveTo().equals(dst.moveTo()))
                            if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.moveTo().equalsLax(dst.moveTo()))
                            if todo!("KoAr equals/equalsLax")

                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: if ((this.hasLnTo() || !dst.hasLnTo()) && (!this.hasLnTo() || dst.hasLnTo()))
                        if (self.hasLnTo() || !dst.hasLnTo()) && (!self.hasLnTo() || dst.hasLnTo())

                        {
                           // Java: if (this.hasLnTo() && dst.hasLnTo())
                            if self.hasLnTo() && dst.hasLnTo()
                           {
                              // Java: if (isStrict && !this.lnTo().equals(dst.lnTo()))
                                if todo!("KoAr equals/equalsLax")

                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: if (!isStrict && !this.lnTo().equalsLax(dst.lnTo()))
                                if todo!("KoAr equals/equalsLax")

                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: if ((this.hasArcTo() || !dst.hasArcTo()) && (!this.hasArcTo() || dst.hasArcTo()))
                            if (self.hasArcTo() || !dst.hasArcTo()) && (!self.hasArcTo() || dst.hasArcTo())

                           {
                              // Java: if (this.hasArcTo() && dst.hasArcTo())
                                if self.hasArcTo() && dst.hasArcTo()
                              {
                                 // Java: if (isStrict && !this.arcTo().equals(dst.arcTo()))
                                    if todo!("KoAr equals/equalsLax")

                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: if (!isStrict && !this.arcTo().equalsLax(dst.arcTo()))
                                    if todo!("KoAr equals/equalsLax")

                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: if ((this.hasQuadBezTo() || !dst.hasQuadBezTo()) && (!this.hasQuadBezTo() || dst.hasQuadBezTo()))
                                if (self.hasQuadBezTo() || !dst.hasQuadBezTo()) && (!self.hasQuadBezTo() || dst.hasQuadBezTo())

                              {
                                 // Java: if (this.hasQuadBezTo() && dst.hasQuadBezTo())
                                    if self.hasQuadBezTo() && dst.hasQuadBezTo()
                                 {
                                    // Java: if (isStrict && !this.quadBezTo().equals(dst.quadBezTo()))
                                        if todo!("KoAr equals/equalsLax")

                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: if (!isStrict && !this.quadBezTo().equalsLax(dst.quadBezTo()))
                                        if todo!("KoAr equals/equalsLax")

                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                 }
                                 // Java: if ((this.hasCubicBezTo() || !dst.hasCubicBezTo()) && (!this.hasCubicBezTo() || dst.hasCubicBezTo()))
                                    if (self.hasCubicBezTo() || !dst.hasCubicBezTo()) && (!self.hasCubicBezTo() || dst.hasCubicBezTo())

                                 {
                                    // Java: if (this.hasCubicBezTo() && dst.hasCubicBezTo())
                                        if self.hasCubicBezTo() && dst.hasCubicBezTo()
                                    {
                                       // Java: if (isStrict && !this.cubicBezTo().equals(dst.cubicBezTo()))
                                            if todo!("KoAr equals/equalsLax")

                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: if (!isStrict && !this.cubicBezTo().equalsLax(dst.cubicBezTo()))
                                            if todo!("KoAr equals/equalsLax")

                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                    }
                                    // Java: return true;
                                    return true;
                                 }
                                 // Java: else
                                    else
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: else
                                else
                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: else
                            else
                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: else
                        else
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: else
                    else
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
            else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
        else
            {
               // Java: return false;
               return false;
            }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
            // Java: int ret = 0;
            let mut ret: i32 = 0;
            // Java: if (this.hasType())
        if self.hasType()
            {
               // Java: ret += Integer.valueOf(this.type()).hashCode();
               ret = ret.wrapping_add(self.get_type());
            }
            // Java: if (this.hasClose())
        if self.hasClose()
            {
               // Java: ret += Boolean.valueOf(this.isClose()).hashCode();
               ret = ret.wrapping_add(if self.isClose() { 1231 } else { 1237 });
            }
            // Java: if (this.hasMoveTo())
        if self.hasMoveTo()
            {
               // Java: ret += this.moveTo().hashCode();
               ret = ret.wrapping_add(todo!("moveTo hashCode"));
            }
            // Java: if (this.hasLnTo())
        if self.hasLnTo()
            {
               // Java: ret += this.lnTo().hashCode();
               ret = ret.wrapping_add(todo!("lnTo hashCode"));
            }
            // Java: if (this.hasArcTo())
        if self.hasArcTo()
            {
               // Java: ret += this.arcTo().hashCode();
               ret = ret.wrapping_add(todo!("arcTo hashCode"));
            }
            // Java: if (this.hasQuadBezTo())
        if self.hasQuadBezTo()
            {
               // Java: ret += this.quadBezTo().hashCode();
               ret = ret.wrapping_add(todo!("quadBezTo hashCode"));
            }
            // Java: if (this.hasCubicBezTo())
        if self.hasCubicBezTo()
            {
               // Java: ret += this.cubicBezTo().hashCode();
               ret = ret.wrapping_add(todo!("cubicBezTo hashCode"));
            }
            // Java: return ret;
            ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
            // Java: if (this.moveTo != null && this.moveTo.dirty())
        if self.moveTo.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.lnTo != null && this.lnTo.dirty())
        else if self.lnTo.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.arcTo != null && this.arcTo.dirty())
        else if self.arcTo.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else if (this.quadBezTo != null && this.quadBezTo.dirty())
        else if self.quadBezTo.as_ref().map_or(false, |x| x.dirty())
            {
               // Java: return true;
               return true;
            }
            // Java: else
        else
            {
               // Java: return this.cubicBezTo != null && this.cubicBezTo.dirty() ? true : this.map.dirty();
               return self.cubicBezTo.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
            }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
            // Java: this.saveBin(this.zone);
            let zone = self.zone;
            self.saveBin(zone);
            // Java: return this.box.get();
            self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: return KoBin.Factory.dup(this.box.get(), zone);
            Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
            // Java: this.loadBin(this.box.get(), zone);
            let bin = self.bin_box.get();
            self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
            // Java: this.cleanFollow();
            self.cleanFollow();
            // Java: this.cleanCache();
            self.cleanCache();
            // Java: this.cleanParent();
            self.cleanParent();
            // Java: this.map.load(KoBin.Null.value1());
            let null_bin = Null::value1();
            self.map.load(null_bin.as_ref());
            // Java: this.box.clear();
            self.bin_box.clear();
            // Java: this.zone.leave();
            self.zone.leave();
            // Java: this.zone = KoZone.defaultZone();
            self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
            // Java: this.moveTo = null;
            self.moveTo = None;
            // Java: this.lnTo = null;
            self.lnTo = None;
            // Java: this.arcTo = null;
            self.arcTo = None;
            // Java: this.quadBezTo = null;
            self.quadBezTo = None;
            // Java: this.cubicBezTo = null;
            self.cubicBezTo = None;
    }

    // Java: void follow(PathSegment parent)
    pub fn follow(&mut self, parent: Option<Box<PathSegment>>) {
            // Java: if (parent != null)
        if let Some(mut p) = parent
            {
               // Java: parent.discard();
               p.discard();
               // Java: this.discard();
               self.discard();
               // Java: this.parent = parent;
               self.parent = Some(p);
            }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
            // Java: if (this.parent != null)
        if self.parent.is_some()
            {
               // Java: this.discard();
               self.discard();
               // Java: this.parent.discard();
               if let Some(ref mut p) = self.parent { p.discard(); }
               // Java: this.cleanParent();
               self.cleanParent();
            }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
            // Java: return this.parent != null;
            self.parent.is_some()
    }

    // Java: PathSegment parent()
    pub fn get_parent(&self) -> Option<&PathSegment> {
            // Java: return this.parent;
            self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
            // Java: this.discard(this.zone);
            let zone = self.zone;
            self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
            // Java: this.parent = null;
            self.parent = None;
    }

    // Java: int type()
    pub fn get_type(&self) -> i32 {
            // Java: if (!this.map.has(1))
        if !self.map.has(1)
            {
               // Java: return this.parent == null ? 1 : this.parent.type();
               return self.parent.as_ref().map_or(1, |p| p.get_type());
            }
            // Java: else
        else
            {
               // Java: KoBin bin = this.map.byKey(1);
               let bin = self.map.byKey(1);
               // Java: return KoBin.Decode.toInt(bin);
               return Decode::toInt(bin.as_ref());
            }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
            // Java: return this.map.has(1) || this.parent != null && this.parent.hasType();
            self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
            // Java: this.map.put(1, (KoBin)null);
            self.map.clear_key(1);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
            // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
            self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: boolean isClose()
    pub fn isClose(&self) -> bool {
            // Java: if (!this.map.has(2))
        if !self.map.has(2)
            {
               // Java: return this.parent == null ? false : this.parent.isClose();
               return self.parent.as_ref().map_or(false, |p| p.isClose());
            }
            // Java: else
        else
            {
               // Java: KoBin bin = this.map.byKey(2);
               let bin = self.map.byKey(2);
               // Java: return KoBin.Decode.toBool(bin);
               return Decode::toBool(bin.as_ref());
            }
    }

    // Java: boolean hasClose()
    pub fn hasClose(&self) -> bool {
            // Java: return this.map.has(2) || this.parent != null && this.parent.hasClose();
            self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasClose())
    }

    // Java: void deleteClose()
    pub fn deleteClose(&mut self) {
            // Java: this.map.put(2, (KoBin)null);
            self.map.clear_key(2);
    }

    // Java: void putClose(boolean value)
    pub fn putClose(&mut self, value: bool) {
            // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
            self.map.put(2, Encode::from_int(value, self.zone));
    }

    // Java: AdjPoint2D moveTo()
    pub fn get_moveTo(&self) -> &dyn KoAr {
            // Java: if (this.moveTo == null)
        if self.moveTo.is_none()
            {
               // Java: if (this.map.has(3))
            if self.map.has(3)
               {
                  // Java: KoBin bin = this.map.byKey(3);
                  let bin = self.map.byKey(3);
                  // Java: this.moveTo = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasMoveTo())
            else if self.parent != null && self.parent.hasMoveTo()
               {
                  // Java: this.moveTo = this.parent.moveTo();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.moveTo = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.moveTo;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasMoveTo()
    pub fn hasMoveTo(&self) -> bool {
            // Java: return this.map.has(3) || this.moveTo != null && this.moveTo.dirty() || this.parent != null && this.parent.hasMoveTo();
            self.map.has(3) || self.moveTo.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasMoveTo())
    }

    // Java: void deleteMoveTo()
    pub fn deleteMoveTo(&mut self) {
            // Java: this.map.put(3, (KoBin)null);
            self.map.clear_key(3);
            // Java: if (this.moveTo != null)
        if self.moveTo.is_some()
            {
               // Java: this.moveTo = null;
               self.moveTo = None;
            }
    }

    // Java: void writeMoveTo(AdjPoint2D value)
    pub fn writeMoveTo(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(3, bin);
                  self.map.put(3, bin);
                  // Java: if (this.moveTo != null)
                if self.moveTo.is_some()
                  {
                     // Java: this.moveTo = null;
                     self.moveTo = None;
                  }
               }
            }
    }

    // Java: void forceWriteMoveTo(AdjPoint2D value)
    pub fn forceWriteMoveTo(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(3, value.lastBin(this.zone));
               self.map.put(3, v.lastBin(self.zone));
               // Java: if (this.moveTo != null)
            if self.moveTo.is_some()
               {
                  // Java: this.moveTo = null;
                  self.moveTo = None;
               }
            }
    }

    // Java: AdjPoint2D lnTo()
    pub fn get_lnTo(&self) -> &dyn KoAr {
            // Java: if (this.lnTo == null)
        if self.lnTo.is_none()
            {
               // Java: if (this.map.has(4))
            if self.map.has(4)
               {
                  // Java: KoBin bin = this.map.byKey(4);
                  let bin = self.map.byKey(4);
                  // Java: this.lnTo = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasLnTo())
            else if self.parent != null && self.parent.hasLnTo()
               {
                  // Java: this.lnTo = this.parent.lnTo();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.lnTo = (AdjPoint2D)KoBin.Decode.toAny(AdjPoint2D.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.lnTo;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasLnTo()
    pub fn hasLnTo(&self) -> bool {
            // Java: return this.map.has(4) || this.lnTo != null && this.lnTo.dirty() || this.parent != null && this.parent.hasLnTo();
            self.map.has(4) || self.lnTo.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLnTo())
    }

    // Java: void deleteLnTo()
    pub fn deleteLnTo(&mut self) {
            // Java: this.map.put(4, (KoBin)null);
            self.map.clear_key(4);
            // Java: if (this.lnTo != null)
        if self.lnTo.is_some()
            {
               // Java: this.lnTo = null;
               self.lnTo = None;
            }
    }

    // Java: void writeLnTo(AdjPoint2D value)
    pub fn writeLnTo(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(4, bin);
                  self.map.put(4, bin);
                  // Java: if (this.lnTo != null)
                if self.lnTo.is_some()
                  {
                     // Java: this.lnTo = null;
                     self.lnTo = None;
                  }
               }
            }
    }

    // Java: void forceWriteLnTo(AdjPoint2D value)
    pub fn forceWriteLnTo(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(4, value.lastBin(this.zone));
               self.map.put(4, v.lastBin(self.zone));
               // Java: if (this.lnTo != null)
            if self.lnTo.is_some()
               {
                  // Java: this.lnTo = null;
                  self.lnTo = None;
               }
            }
    }

    // Java: ArcTo arcTo()
    pub fn get_arcTo(&self) -> &dyn KoAr {
            // Java: if (this.arcTo == null)
        if self.arcTo.is_none()
            {
               // Java: if (this.map.has(5))
            if self.map.has(5)
               {
                  // Java: KoBin bin = this.map.byKey(5);
                  let bin = self.map.byKey(5);
                  // Java: this.arcTo = (ArcTo)KoBin.Decode.toAny(ArcTo.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasArcTo())
            else if self.parent != null && self.parent.hasArcTo()
               {
                  // Java: this.arcTo = this.parent.arcTo();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.arcTo = (ArcTo)KoBin.Decode.toAny(ArcTo.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.arcTo;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasArcTo()
    pub fn hasArcTo(&self) -> bool {
            // Java: return this.map.has(5) || this.arcTo != null && this.arcTo.dirty() || this.parent != null && this.parent.hasArcTo();
            self.map.has(5) || self.arcTo.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasArcTo())
    }

    // Java: void deleteArcTo()
    pub fn deleteArcTo(&mut self) {
            // Java: this.map.put(5, (KoBin)null);
            self.map.clear_key(5);
            // Java: if (this.arcTo != null)
        if self.arcTo.is_some()
            {
               // Java: this.arcTo = null;
               self.arcTo = None;
            }
    }

    // Java: void writeArcTo(ArcTo value)
    pub fn writeArcTo(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(5, bin);
                  self.map.put(5, bin);
                  // Java: if (this.arcTo != null)
                if self.arcTo.is_some()
                  {
                     // Java: this.arcTo = null;
                     self.arcTo = None;
                  }
               }
            }
    }

    // Java: void forceWriteArcTo(ArcTo value)
    pub fn forceWriteArcTo(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(5, value.lastBin(this.zone));
               self.map.put(5, v.lastBin(self.zone));
               // Java: if (this.arcTo != null)
            if self.arcTo.is_some()
               {
                  // Java: this.arcTo = null;
                  self.arcTo = None;
               }
            }
    }

    // Java: QuadraticBezierCurveTo quadBezTo()
    pub fn get_quadBezTo(&self) -> &dyn KoAr {
            // Java: if (this.quadBezTo == null)
        if self.quadBezTo.is_none()
            {
               // Java: if (this.map.has(6))
            if self.map.has(6)
               {
                  // Java: KoBin bin = this.map.byKey(6);
                  let bin = self.map.byKey(6);
                  // Java: this.quadBezTo = (QuadraticBezierCurveTo)KoBin.Decode.toAny(QuadraticBezierCurveTo.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasQuadBezTo())
            else if self.parent != null && self.parent.hasQuadBezTo()
               {
                  // Java: this.quadBezTo = this.parent.quadBezTo();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.quadBezTo = (QuadraticBezierCurveTo)KoBin.Decode.toAny(QuadraticBezierCurveTo.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.quadBezTo;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasQuadBezTo()
    pub fn hasQuadBezTo(&self) -> bool {
            // Java: return this.map.has(6) || this.quadBezTo != null && this.quadBezTo.dirty() || this.parent != null && this.parent.hasQuadBezTo();
            self.map.has(6) || self.quadBezTo.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasQuadBezTo())
    }

    // Java: void deleteQuadBezTo()
    pub fn deleteQuadBezTo(&mut self) {
            // Java: this.map.put(6, (KoBin)null);
            self.map.clear_key(6);
            // Java: if (this.quadBezTo != null)
        if self.quadBezTo.is_some()
            {
               // Java: this.quadBezTo = null;
               self.quadBezTo = None;
            }
    }

    // Java: void writeQuadBezTo(QuadraticBezierCurveTo value)
    pub fn writeQuadBezTo(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(6, bin);
                  self.map.put(6, bin);
                  // Java: if (this.quadBezTo != null)
                if self.quadBezTo.is_some()
                  {
                     // Java: this.quadBezTo = null;
                     self.quadBezTo = None;
                  }
               }
            }
    }

    // Java: void forceWriteQuadBezTo(QuadraticBezierCurveTo value)
    pub fn forceWriteQuadBezTo(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(6, value.lastBin(this.zone));
               self.map.put(6, v.lastBin(self.zone));
               // Java: if (this.quadBezTo != null)
            if self.quadBezTo.is_some()
               {
                  // Java: this.quadBezTo = null;
                  self.quadBezTo = None;
               }
            }
    }

    // Java: CubicBezierCurveTo cubicBezTo()
    pub fn get_cubicBezTo(&self) -> &dyn KoAr {
            // Java: if (this.cubicBezTo == null)
        if self.cubicBezTo.is_none()
            {
               // Java: if (this.map.has(7))
            if self.map.has(7)
               {
                  // Java: KoBin bin = this.map.byKey(7);
                  let bin = self.map.byKey(7);
                  // Java: this.cubicBezTo = (CubicBezierCurveTo)KoBin.Decode.toAny(CubicBezierCurveTo.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
               // Java: else if (this.parent != null && this.parent.hasCubicBezTo())
            else if self.parent != null && self.parent.hasCubicBezTo()
               {
                  // Java: this.cubicBezTo = this.parent.cubicBezTo();
                  todo!("lazy init requires &mut self")
               }
               // Java: else
            else
               {
                  // Java: KoBin bin = KoBin.Null.value1();
                  let _bin = Null::value1();
                  // Java: this.cubicBezTo = (CubicBezierCurveTo)KoBin.Decode.toAny(CubicBezierCurveTo.class, bin, this.zone);
                  todo!("lazy init requires &mut self")
               }
            }
            // Java: return this.cubicBezTo;
            todo!("lazy init requires &mut self")
    }

    // Java: boolean hasCubicBezTo()
    pub fn hasCubicBezTo(&self) -> bool {
            // Java: return this.map.has(7) || this.cubicBezTo != null && this.cubicBezTo.dirty() || this.parent != null && this.parent.hasCubicBezTo();
            self.map.has(7) || self.cubicBezTo.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCubicBezTo())
    }

    // Java: void deleteCubicBezTo()
    pub fn deleteCubicBezTo(&mut self) {
            // Java: this.map.put(7, (KoBin)null);
            self.map.clear_key(7);
            // Java: if (this.cubicBezTo != null)
        if self.cubicBezTo.is_some()
            {
               // Java: this.cubicBezTo = null;
               self.cubicBezTo = None;
            }
    }

    // Java: void writeCubicBezTo(CubicBezierCurveTo value)
    pub fn writeCubicBezTo(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: KoBin bin = value.lastBin(this.zone);
               let bin = v.lastBin(self.zone);
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.map.put(7, bin);
                  self.map.put(7, bin);
                  // Java: if (this.cubicBezTo != null)
                if self.cubicBezTo.is_some()
                  {
                     // Java: this.cubicBezTo = null;
                     self.cubicBezTo = None;
                  }
               }
            }
    }

    // Java: void forceWriteCubicBezTo(CubicBezierCurveTo value)
    pub fn forceWriteCubicBezTo(&mut self, value: Option<&dyn KoAr>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.map.put(7, value.lastBin(this.zone));
               self.map.put(7, v.lastBin(self.zone));
               // Java: if (this.cubicBezTo != null)
            if self.cubicBezTo.is_some()
               {
                  // Java: this.cubicBezTo = null;
                  self.cubicBezTo = None;
               }
            }
    }

}

// Java: public class PathSegmentList
impl PathSegmentList {
    // Java: PathSegmentList()
    pub fn new() -> Self {
      Self {
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.list = new KpList(this.zone);
            list: KpList::new(KoZone::defaultZone()),
            zone: KoZone::defaultZone(),
            }
    }

    // Java: PathSegmentList(KoBin bin)
    pub fn PathSegmentList_from(bin: &dyn KoBinTrait) -> Self {
      Self {
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.list = new KpList(bin, this.zone);
            list: KpList::KpList_from(bin, KoZone::defaultZone()),
            zone: KoZone::defaultZone(),
            }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
            // Java: this.box.set(bin);
            self.bin_box.set(Some(bin));
            // Java: this.list.load(this.box.get(), zone);
            let bin_ref = self.bin_box.get();
            self.list.load(bin_ref.as_ref(), _zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
            // Java: this.loadBin(bin, this.zone);
            let zone = self.zone;
            self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(PathSegmentList value)
    pub fn load_value(&mut self, value: Option<&PathSegmentList>) {
            // Java: if (value != null)
        if let Some(v) = value
            {
               // Java: this.loadBin(value.lastBin(this.zone), this.zone);
               let bin = v.lastBin(self.zone);
               self.loadBin_with_zone(bin.as_ref(), self.zone);
            }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
            if self.list.dirty() {
               let saved = self.list.save(zone);
               self.bin_box.set(Some(saved.as_ref()));
               self.bin_box.get()
            } else {
               self.bin_box.get()
            }
    }

    // Java: boolean equalsLax(PathSegmentList o)
    pub fn equalsLax(&self, o: &PathSegmentList) -> bool {
            // Java: return this.equals(o, false);
            self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &PathSegmentList) -> bool {
            // Java: return this.equals(o, true);
            self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &PathSegmentList, isStrict: bool) -> bool {
            // Java: if (null == o)
        if true
            {
          let dst = o;
               // Java: return false;
               return false;
            }
            // Java: else
        else
            {
               // Java: PathSegmentList dst = (PathSegmentList)o;
               // Java: if (this == dst)
            if std::ptr::eq(self, dst)
               {
                  // Java: return true;
                  return true;
               }
               // Java: else
            else
               {
                  // Java: ArrayList<PathSegment> dstList = new ArrayList();
                  todo!("list toList")
                  // Java: dst.read(dstList);
                  todo!("dst.read(dstList);")
                  // Java: ArrayList<PathSegment> srcList = new ArrayList();
                  todo!("list toList")
                  // Java: this.read(srcList);
                  todo!("this.read(srcList);")
                  // Java: if (dstList.size() != srcList.size())
                todo!("if (dstList.size() != srcList.size())")
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else
                else
                  {
                     // Java: int size = dstList.size();
                     todo!("int size = dstList.size();")
                     // Java: for(int i = 0; i < size; ++i)
                    todo!("for(int i = 0; i < size; ++i)")
                     {
                        // Java: PathSegment dstClrTrans = (PathSegment)dstList.get(i);
                        todo!("PathSegment dstClrTrans = (PathSegment)dstList.get(i);")
                        // Java: PathSegment srcClrTrans = (PathSegment)srcList.get(i);
                        todo!("PathSegment srcClrTrans = (PathSegment)srcList.get(i);")
                        // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                        if isStrict && !dstClrTrans.equals(srcClrTrans)
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                        if !isStrict && !dstClrTrans.equalsLax(srcClrTrans)
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: return true;
                     return true;
                  }
               }
            }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
            // Java: return this.list.hashCode();
            todo!("list hashCode")
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
            // Java: return this.list.dirty();
            self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
            // Java: this.saveBin(this.zone);
            let zone = self.zone;
            self.saveBin(zone);
            // Java: return this.box.get();
            self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
            // Java: this.loadBin(this.box.get(), zone);
            let bin = self.bin_box.get();
            self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
            // Java: return KoBin.Factory.dup(this.box.get(), zone);
            Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
            // Java: this.list.load(KoBin.Null.value1(), this.zone);
            let null_bin = Null::value1();
            self.list.load(null_bin.as_ref(), self.zone);
            // Java: this.zone.leave();
            self.zone.leave();
            // Java: this.zone = KoZone.defaultZone();
            self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends PathSegmentList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
            // Java: KoZone zone = KoZone.enter();
            let zone = KoZone::enter();
            // Java: Value1 inst = (Value1)(zone.allocate(clazz));
            let mut inst = Self::PathSegmentList_from(bin);
            // Java: inst.zone = zone;
            inst.zone = zone;
            // Java: inst.load(bin);
            inst.load_bin(bin);
            // Java: return inst;
            inst
    }

    // Java: static PathSegmentList create()
    pub fn create() -> Self {
            // Java: return initialize(PathSegmentList.class, KoBin.Null.empty());
            let empty = Null::empty();
            Self::initialize(empty.as_ref())
    }

    // Java: static PathSegmentList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
            // Java: return initialize(PathSegmentList.class, bin);
            Self::initialize(bin)
    }

    // Java: void read(Collection<PathSegment> valueLst)
    pub fn read(&self, Collection<PathSegment> valueLst) {
            // Java: valueLst.clear();
            todo!("valueLst.clear();")
            // Java: int size = this.list.size();
            todo!("int size = this.list.size();")
            // Java: for(int i = 0; i < size; ++i)
        todo!("for(int i = 0; i < size; ++i)")
            {
               // Java: valueLst.add((PathSegment)KoBin.Decode.toAny(PathSegment.class, this.list.get(i), this.zone));
               todo!("lazy init requires &mut self")
            }
    }

    // Java: void write(Iterable<PathSegment> valueLst)
    pub fn write(&self, Iterable<PathSegment> valueLst) {
            // Java: this.list.clear();
            self.list.clear();
            // Java: for(PathSegment i : valueLst)
        todo!("for(PathSegment i : valueLst)")
            {
               // Java: KoBin bin = i.lastBin(this.zone);
               todo!("KoBin bin = i.lastBin(this.zone);")
               // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
               {
                  // Java: this.list.add(bin);
                  todo!("list add")
               }
            }
    }

    // Java: void forceWrite(Iterable<PathSegment> valueLst)
    pub fn forceWrite(&self, Iterable<PathSegment> valueLst) {
            // Java: this.list.clear();
            self.list.clear();
            // Java: for(PathSegment i : valueLst)
        todo!("for(PathSegment i : valueLst)")
            {
               // Java: this.list.add(i.lastBin(this.zone));
               todo!("list add")
            }
    }

}

// Java: public class PathList
impl PathList {
    // Java: PathList()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.list = new KpList(this.zone);
         list: KpList::new(KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: PathList(KoBin bin)
    pub fn PathList_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.list = new KpList(bin, this.zone);
         list: KpList::KpList_from(bin, KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get();
         self.list.load(bin_ref.as_ref(), _zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(PathList value)
    pub fn load_value(&mut self, value: Option<&PathList>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin_with_zone(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() {
            let saved = self.list.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(PathList o)
    pub fn equalsLax(&self, o: &PathList) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &PathList) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &PathList, isStrict: bool) -> bool {
         // Java: if (null == o)
        if true
         {
          let dst = o;
            // Java: return false;
            return false;
         }
         // Java: else
        else
         {
            // Java: PathList dst = (PathList)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            else
            {
               // Java: ArrayList<Path> dstList = new ArrayList();
               todo!("list toList")
               // Java: dst.read(dstList);
               todo!("dst.read(dstList);")
               // Java: ArrayList<Path> srcList = new ArrayList();
               todo!("list toList")
               // Java: this.read(srcList);
               todo!("this.read(srcList);")
               // Java: if (dstList.size() != srcList.size())
                todo!("if (dstList.size() != srcList.size())")
               {
                  // Java: return false;
                  return false;
               }
               // Java: else
                else
               {
                  // Java: int size = dstList.size();
                  todo!("int size = dstList.size();")
                  // Java: for(int i = 0; i < size; ++i)
                    todo!("for(int i = 0; i < size; ++i)")
                  {
                     // Java: Path dstClrTrans = (Path)dstList.get(i);
                     todo!("Path dstClrTrans = (Path)dstList.get(i);")
                     // Java: Path srcClrTrans = (Path)srcList.get(i);
                     todo!("Path srcClrTrans = (Path)srcList.get(i);")
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                        if isStrict && !dstClrTrans.equals(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                        if !isStrict && !dstClrTrans.equalsLax(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: return true;
                  return true;
               }
            }
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         todo!("list hashCode")
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::value1();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends PathList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::PathList_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static PathList create()
    pub fn create() -> Self {
         // Java: return initialize(PathList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static PathList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(PathList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<Path> valueLst)
    pub fn read(&self, Collection<Path> valueLst) {
         // Java: valueLst.clear();
         todo!("valueLst.clear();")
         // Java: int size = this.list.size();
         todo!("int size = this.list.size();")
         // Java: for(int i = 0; i < size; ++i)
        todo!("for(int i = 0; i < size; ++i)")
         {
            // Java: valueLst.add((Path)KoBin.Decode.toAny(Path.class, this.list.get(i), this.zone));
            todo!("lazy init requires &mut self")
         }
    }

    // Java: void write(Iterable<Path> valueLst)
    pub fn write(&self, Iterable<Path> valueLst) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(Path i : valueLst)
        todo!("for(Path i : valueLst)")
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            todo!("KoBin bin = i.lastBin(this.zone);")
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.list.add(bin);
               todo!("list add")
            }
         }
    }

    // Java: void forceWrite(Iterable<Path> valueLst)
    pub fn forceWrite(&self, Iterable<Path> valueLst) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(Path i : valueLst)
        todo!("for(Path i : valueLst)")
         {
            // Java: this.list.add(i.lastBin(this.zone));
            todo!("list add")
         }
    }

}

impl KoAr for CustomGeometry {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { CustomGeometry::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { CustomGeometry::lastBin(self, var1) }
}

impl KoClosable for CustomGeometry {
    fn close(&mut self) { CustomGeometry::close(self); }
}
