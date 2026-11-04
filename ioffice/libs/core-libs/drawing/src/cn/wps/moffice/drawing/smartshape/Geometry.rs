#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.PropBase
use super::super::PropBase::PropBase;

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class Geometry extends PropBase
pub struct Geometry {
    pub base: PropBase,
}

// Java: public class Geometry extends PropBase
impl Geometry {
    // Java: Geometry()
    pub fn new() -> Self {
      // Java: super();
      Geometry { base: PropBase::new() }
    }

    // Java: Geometry clone()
    pub fn clone(&self) -> Geometry {
      // Java: Geometry geometry = (Geometry)super.clone();
      let mut geometry = Geometry { base: Clone::clone(&self.base) };
      // Java: GeoAdjust geoAdjust = this.getGeoAdjust();
      let geo_adjust = self.getGeoAdjust();
      // Java: if (geoAdjust != null)
      if geo_adjust.is_some() {
         // Java: geometry.setGeoAdjust(geoAdjust.clone());
         geometry.setGeoAdjust(geo_adjust);
      }
      // Java: return geometry;
      geometry
    }

    // Java: void setGeoTextRect(RectF rectF)
    pub fn setGeoTextRect(&mut self, rectF: Element) {
      // Java: this.mProperty.setValue(88, rectF);
      self.base.mProperty.as_mut().unwrap().setValue(88, rectF);
    }

    // Java: RectF getGeoTextRect()
    pub fn getGeoTextRect(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(88);
      // Java: return object == null ? null : (RectF)object;
      self.base.mProperty.as_ref().unwrap().base.getValue(88)
    }

    // Java: void setGeoCordSize(RectF rectF)
    pub fn setGeoCordSize(&mut self, rectF: Element) {
      // Java: this.mProperty.setValue(63, rectF);
      self.base.mProperty.as_mut().unwrap().setValue(63, rectF);
    }

    // Java: RectF getGeoCordSize()
    pub fn getGeoCordSize(&self) -> Arc<RectF> {
      // Java: Object object = this.mProperty.getValue(63);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(63);
      // Java: RectF rt = object == null ? null : (RectF)object;
      let rt = object.and_then(|v| v.downcast_ref::<RectF>().cloned()).map(|r| Arc::new(r));
      // Java: return rt == null ? new RectF(0.0F, 0.0F, 21600.0F, 21600.0F) : rt;
      rt.unwrap_or_else(|| Arc::new(RectF::new_with(0.0_f32, 0.0_f32, 21600.0_f32, 21600.0_f32)))
    }

    // Java: void setGeoAdjust(GeoAdjust adjust)
    pub fn setGeoAdjust(&mut self, adjust: Element) {
      // Java: this.mProperty.setValue(65, adjust);
      self.base.mProperty.as_mut().unwrap().setValue(65, adjust);
    }

    // Java: GeoAdjust getGeoAdjust()
    pub fn getGeoAdjust(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(65);
      // Java: return object == null ? null : (GeoAdjust)object;
      self.base.mProperty.as_ref().unwrap().base.getValue(65)
    }

    // Java: void setVertices(int[] vertices)
    pub fn setVertices(&mut self, vertices: Element) {
      // Java: this.mProperty.setValue(67, vertices);
      self.base.mProperty.as_mut().unwrap().setValue(67, vertices);
    }

    // Java: int[] getVertices()
    pub fn getVertices(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(67);
      // Java: return object == null ? null : (int[])object;
      self.base.mProperty.as_ref().unwrap().base.getValue(67)
    }

    // Java: void setVerticesCbElem(int cbElem)
    pub fn setVerticesCbElem(&mut self, cbElem: i32) {
      // Java: this.mProperty.setInt(79, cbElem);
      self.base.mProperty.as_mut().unwrap().setInt(79, cbElem);
    }

    // Java: int getVerticesCbElem()
    pub fn getVerticesCbElem(&self) -> i32 {
      // Java: return this.mProperty.getInt(79, 8);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(79, 8)
    }

    // Java: void setSegment(Segment[] segments)
    pub fn setSegment(&mut self, segments: Element) {
      // Java: this.mProperty.setValue(68, segments);
      self.base.mProperty.as_mut().unwrap().setValue(68, segments);
    }

    // Java: Segment[] getSegment()
    pub fn getSegment(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(68);
      // Java: return object == null ? null : (Segment[])object;
      self.base.mProperty.as_ref().unwrap().base.getValue(68)
    }

    // Java: void setShapePath(ShapePath path)
    pub fn setShapePath(&mut self, path: Element) {
      // Java: this.mProperty.setValue(78, path);
      self.base.mProperty.as_mut().unwrap().setValue(78, path);
    }

    // Java: ShapePath getShapePath()
    pub fn getShapePath(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(78);
      // Java: return object == null ? null : (ShapePath)object;
      self.base.mProperty.as_ref().unwrap().base.getValue(78)
    }

    // Java: void setConnectionSites(int[] v)
    pub fn setConnectionSites(&mut self, v: Element) {
      // Java: this.mProperty.setValue(71, v);
      self.base.mProperty.as_mut().unwrap().setValue(71, v);
    }

    // Java: int[] getConnectionSites()
    pub fn getConnectionSites(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(71);
      // Java: return object == null ? null : (int[])object;
      self.base.mProperty.as_ref().unwrap().base.getValue(71)
    }

    // Java: void setConnectionSites4DOCX(int[] v)
    pub fn setConnectionSites4DOCX(&mut self, v: Element) {
      // Java: this.mProperty.setValue(87, v);
      self.base.mProperty.as_mut().unwrap().setValue(87, v);
    }

    // Java: int[] getConnectionSites4DOCX()
    pub fn getConnectionSites4DOCX(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(87);
      // Java: return object == null ? null : (int[])object;
      self.base.mProperty.as_ref().unwrap().base.getValue(87)
    }

    // Java: void setConnectionSitesDir(float[] a)
    pub fn setConnectionSitesDir(&mut self, a: Element) {
      // Java: this.mProperty.setValue(72, a);
      self.base.mProperty.as_mut().unwrap().setValue(72, a);
    }

    // Java: float[] getConnectionSitesDir()
    pub fn getConnectionSitesDir(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(72);
      // Java: return object == null ? null : (float[])object;
      self.base.mProperty.as_ref().unwrap().base.getValue(72)
    }

    // Java: void setLimoX(int v)
    pub fn setLimoX(&mut self, v: i32) {
      // Java: this.mProperty.setInt(73, v);
      self.base.mProperty.as_mut().unwrap().setInt(73, v);
    }

    // Java: int getLimoX()
    pub fn getLimoX(&self) -> i32 {
      // Java: return this.mProperty.getInt(73, Integer.MIN_VALUE);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(73, i32::MIN)
    }

    // Java: Integer getLimoXObj()
    pub fn getLimoXObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(73);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(73);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setLimoY(int v)
    pub fn setLimoY(&mut self, v: i32) {
      // Java: this.mProperty.setInt(74, v);
      self.base.mProperty.as_mut().unwrap().setInt(74, v);
    }

    // Java: int getLimoY()
    pub fn getLimoY(&self) -> i32 {
      // Java: return this.mProperty.getInt(74, Integer.MIN_VALUE);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(74, i32::MIN)
    }

    // Java: Integer getLimoYObj()
    pub fn getLimoYObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(74);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(74);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setInscribe(int[] v)
    pub fn setInscribe(&mut self, v: Element) {
      // Java: this.mProperty.setValue(75, v);
      self.base.mProperty.as_mut().unwrap().setValue(75, v);
    }

    // Java: int[] getInscribe()
    pub fn getInscribe(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(75);
      // Java: return object == null ? null : (int[])object;
      self.base.mProperty.as_ref().unwrap().base.getValue(75)
    }

    // Java: void setConnectionSitesType(int v)
    pub fn setConnectionSitesType(&mut self, v: i32) {
      // Java: this.mProperty.setInt(70, v);
      self.base.mProperty.as_mut().unwrap().setInt(70, v);
    }

    // Java: int getConnectionSitesType()
    pub fn getConnectionSitesType(&self) -> i32 {
      // Java: return this.mProperty.getInt(70, 1);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(70, 1)
    }

    // Java: Integer getConnectionSitesTypeObj()
    pub fn getConnectionSitesTypeObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(70);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(70);
      // Java: return object == null ? null : (Integer)object;
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setAdjustHandles(ADJH[] a)
    pub fn setAdjustHandles(&mut self, a: Element) {
      // Java: this.mProperty.setValue(76, a);
      self.base.mProperty.as_mut().unwrap().setValue(76, a);
    }

    // Java: ADJH[] getAdjustHandles()
    pub fn getAdjustHandles(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(76);
      // Java: return object == null ? null : (ADJH[])object;
      self.base.mProperty.as_ref().unwrap().base.getValue(76)
    }

    // Java: void setFormulas(GeoFormula[] formulas)
    pub fn setFormulas(&mut self, formulas: Element) {
      // Java: this.mProperty.setValue(77, formulas);
      self.base.mProperty.as_mut().unwrap().setValue(77, formulas);
    }

    // Java: GeoFormula[] getFormulas()
    pub fn getFormulas(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(77);
      // Java: return object == null ? null : (GeoFormula[])object;
      self.base.mProperty.as_ref().unwrap().base.getValue(77)
    }

    // Java: void setShadowOK(boolean bOK)
    pub fn setShadowOK(&mut self, bOK: bool) {
      // Java: this.mProperty.setBoolean(80, bOK);
      self.base.mProperty.as_mut().unwrap().setBoolean(80, bOK);
    }

    // Java: boolean getShadowOK()
    pub fn getShadowOK(&self) -> bool {
      // Java: return this.mProperty.getBoolean(80, true);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(80, true)
    }

    // Java: void set3DOK(boolean bOK)
    pub fn set3DOK(&mut self, bOK: bool) {
      // Java: this.mProperty.setBoolean(81, bOK);
      self.base.mProperty.as_mut().unwrap().setBoolean(81, bOK);
    }

    // Java: boolean get3DOK()
    pub fn get3DOK(&self) -> bool {
      // Java: return this.mProperty.getBoolean(81, true);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(81, true)
    }

    // Java: void setLineOK(boolean bOK)
    pub fn setLineOK(&mut self, bOK: bool) {
      // Java: this.mProperty.setBoolean(82, bOK);
      self.base.mProperty.as_mut().unwrap().setBoolean(82, bOK);
    }

    // Java: boolean getLineOK()
    pub fn getLineOK(&self) -> bool {
      // Java: return this.mProperty.getBoolean(82, true);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(82, true)
    }

    // Java: Boolean getLineOKObj()
    pub fn getLineOKObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(82);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(82);
      // Java: return object == null ? null : (Boolean)object;
      object.and_then(|v| v.downcast_ref::<bool>().copied())
    }

    // Java: void setGtextOK(boolean bOK)
    pub fn setGtextOK(&mut self, bOK: bool) {
      // Java: this.mProperty.setBoolean(83, bOK);
      self.base.mProperty.as_mut().unwrap().setBoolean(83, bOK);
    }

    // Java: boolean getGtextOK()
    pub fn getGtextOK(&self) -> bool {
      // Java: return this.mProperty.getBoolean(83, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(83, false)
    }

    // Java: void setFillShadeShapeOK(boolean bOK)
    pub fn setFillShadeShapeOK(&mut self, bOK: bool) {
      // Java: this.mProperty.setBoolean(84, bOK);
      self.base.mProperty.as_mut().unwrap().setBoolean(84, bOK);
    }

    // Java: boolean getFillShadeShapeOK()
    pub fn getFillShadeShapeOK(&self) -> bool {
      // Java: return this.mProperty.getBoolean(84, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(84, false)
    }

    // Java: void setFillOK(boolean bOK)
    pub fn setFillOK(&mut self, bOK: bool) {
      // Java: this.mProperty.setBoolean(85, bOK);
      self.base.mProperty.as_mut().unwrap().setBoolean(85, bOK);
    }

    // Java: boolean getFillOK()
    pub fn getFillOK(&self) -> bool {
      // Java: return this.mProperty.getBoolean(85, true);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(85, true)
    }

    // Java: Boolean getFillOKObj()
    pub fn getFillOKObj(&self) -> Option<bool> {
      // Java: Object object = this.mProperty.getValue(85);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(85);
      // Java: return object == null ? null : (Boolean)object;
      object.and_then(|v| v.downcast_ref::<bool>().copied())
    }

    // Java: void setColumnLineOK(boolean bOK)
    pub fn setColumnLineOK(&mut self, bOK: bool) {
      // Java: this.mProperty.setBoolean(86, bOK);
      self.base.mProperty.as_mut().unwrap().setBoolean(86, bOK);
    }

    // Java: boolean getColumnLineOK()
    pub fn getColumnLineOK(&self) -> bool {
      // Java: return this.mProperty.getBoolean(86, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(86, false)
    }

}
