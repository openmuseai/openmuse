use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::shape::sppr::geometry::AdjustValueListHanlder::AdjustValueListHanlder;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// rust: ★2B-③-2 —— 由影子 `layout_shape::{GeomObj,GeomGuideObj}` 改为**真身**。
//   `GeomGuideObj` 仅作为"采集期"的临时载体（子处理器逐个填 `name`/`fmla`），
//   在 `endElement` 时桥接进真身 `PresetGeometry` 的 `avLst` KProto 子树。
// rust: ★2B-③-2 —— 处理器持有 `Geometry2D`（Java: `shapePr.geom()`），内部经 `prstGeom()` 到达子对象。
use crate::cn::wps::drawing::data::geometry::Geometry2D::Geometry2D;
use crate::cn::wps::show::util::layout_shape::GeomGuideObj;

// java public class PresetGeometryHandler extends DefaultElementHandler
pub struct PresetGeometryHandler {
    // java private PresetGeometry presetGeometry
    presetGeometry: Rc<RefCell<Geometry2D>>,
    // java private List<GeomGuide> adjustableValues
    adjustableValues: Option<Rc<RefCell<Vec<Rc<RefCell<GeomGuideObj>>>>>>,
}

impl PresetGeometryHandler {
    // java public PresetGeometryHandler(PresetGeometry presetGeometry)
    pub fn PresetGeometryHandler(presetGeometry: Rc<RefCell<Geometry2D>>) -> Self {
        Self {
            presetGeometry,
            adjustableValues: None,
        }
    }
}

impl ElementHandler for PresetGeometryHandler {
    fn kind(&self) -> &'static str {
        "PresetGeometryHandler"
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy.get_by_name(1114293) {
            if let Some(presetShapeType) = DGMap::presetShapeTypeMap().get(temp.to_string().as_str()) {
                self.presetGeometry.borrow_mut().prstGeom().putPrst(*presetShapeType);
            }
        }
    }

    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114422:
            1114422 => {
                let list = Rc::new(RefCell::new(Vec::new()));
                self.adjustableValues = Some(list.clone());
                Some(Box::new(AdjustValueListHanlder::AdjustValueListHanlder(list)))
            }
            _ => None,
        }
    }

    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        if let Some(adjustableValues) = self.adjustableValues.take() {
            let owned: Vec<GeomGuideObj> = adjustableValues
                .borrow()
                .iter()
                .map(|g| g.borrow().clone())
                .collect();
            // rust: ★2B-③-2 —— 桥接进真身 `avLst`（Java: `this.presetGeometry.avLst()`）。
            //   真身 `avLst` 是 KProto 子对象（`Class<T>` 派发在 Rust 侧不可表达），
            //   故按 `AdjustValueList::write(&[&dyn KoAr])` 写入真 `GeomGuide` 列表。
            //   owner: ppt.drawing.dmlr.handler.shape.sppr.geometry.PresetGeometryHandler
            //   wave: M-PPT-SHAPE-DISPATCH
            let mut guides: Vec<crate::cn::wps::drawing::data::geometry::GeomGuide::GeomGuide> =
                Vec::with_capacity(owned.len());
            for g in &owned {
                let mut real = crate::cn::wps::drawing::data::geometry::GeomGuide::GeomGuide::create();
                if let Some(n) = &g.name {
                    real.putName(n.clone());
                }
                if let Some(f) = &g.fmla {
                    real.putFmla(f.clone());
                }
                guides.push(real);
            }
            let refs: Vec<&dyn crate::cn::wps::kso::kproto::KoAr::KoAr> =
                guides.iter().map(|g| g as &dyn crate::cn::wps::kso::kproto::KoAr::KoAr).collect();
            let mut pg = self.presetGeometry.borrow_mut();
            let mut pg = pg.prstGeom();
            let mut av = crate::cn::wps::drawing::data::geometry::PresetGeometry::AdjustValueList::create();
            av.write(&refs);
            // Java: this.map.put(2, value.lastBin(this.zone)) —— 经真身 `writeAvLst`。
            pg.writeAvLst(Some(&av as &dyn crate::cn::wps::kso::kproto::KoAr::KoAr));
        }
    }
}
