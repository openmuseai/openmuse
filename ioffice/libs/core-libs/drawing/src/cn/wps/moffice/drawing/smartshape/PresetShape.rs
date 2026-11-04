#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_imports)]

// Java: import cn.wps.graphics.PointF
use core_libs_util::graphics::PointF::PointF;
// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.Shape
use crate::cn::wps::moffice::drawing::Shape::Shape;
// Java: import java.util.Vector

use super::DataCal::{DataCal, PresetData as DataCalPresetData};
use super::PresetData::PresetData;
use super::SingleAdjustHandler::SingleAdjustHandler;
use super::AdjustHandler::AdjustHandler;
use super::PathData::PathData;

type Vector<T> = Vec<T>;

fn clone_path_data(pd: &PathData) -> PathData {
    let mut base = PathData::new();
    base._varList = pd._varList.clone();
    base._rect = pd._rect.clone();
    base._textBoxRectList = pd._textBoxRectList.clone();
    base._isAdjusted = pd._isAdjusted;
    base.setType(pd.getType());
    base
}

fn new_pointf_array(count: i32) -> Vec<Option<PointF>> {
    let mut pts = Vec::new();
    for _ in 0..count {
        pts.push(None);
    }
    pts
}

fn convert_datacal_preset(pd: &DataCalPresetData) -> PresetData {
    PresetData {
        base: clone_path_data(&pd.base),
        _avcCount: 0,
        _originCxnList: None,
        _cxnList: None,
        _needCalculateAdjustData: true,
        _avcLst: None,
        _ahList: None,
        _pathList: None,
    }
}

// Java: public class PresetShape
pub struct PresetShape {
    // Java field: Shape mShape
    pub(crate) mShape: Option<Shape>,
    // Java field: DataCal mDataCal
    pub(crate) mDataCal: Option<DataCal>,
    // Java field: PresetData mPresetData
    pub(crate) mPresetData: Option<PresetData>,
    // Java field: Vector<AdjustHandler> mAhLst
    pub(crate) mAhLst: Option<Vector<Option<AdjustHandler>>>,
    // Java field: int[] mNotSupportAdjustShaps
    mNotSupportAdjustShaps: Vec<i32>,
}

// Java: public class PresetShape
impl PresetShape {
    const NOT_SUPPORT_ADJUST_SHAPES: &'static [i32] =
        &[189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200];

    // Java: PresetShape(Shape shape, DataCal dataCal)
    pub fn new(shape: Shape, dataCal: DataCal) -> Self {
        // Java: super();
        ();
        let mut instance = PresetShape {
            mShape: None,
            mDataCal: None,
            mPresetData: None,
            mAhLst: None,
            mNotSupportAdjustShaps: vec![
                189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200,
            ],
        };
        instance.init(shape, dataCal);
        instance
    }

    fn init(&mut self, shape: Shape, dataCal: DataCal) {
        // Java: this.mShape = shape;
        self.mShape = Some(shape);
        // Java: this.mDataCal = dataCal;
        self.mDataCal = Some(dataCal);
        // Java: this.mPresetData = dataCal.getPresetData();
        self.mPresetData = self
            .mDataCal
            .as_ref()
            .and_then(|dc| dc.getPresetData().map(convert_datacal_preset));
    }

    // Java: int getAdjustCount()
    pub fn getAdjustCount(&mut self) -> i32 {
        // Java: if (this.mAhLst == null)
        if self.mAhLst.is_none() {
            // Java: this.mAhLst = this.mPresetData.getAhLst();
            self.mAhLst = self
                .mPresetData
                .as_mut()
                .and_then(|p| p.getAhLst())
                .map(|lst| lst.iter().map(|ah| Some(ah.clone())).collect());
        }
        // Java: return this.mAhLst != null ? this.mAhLst.size() : 0;
        self.mAhLst.as_ref().map_or(0, |v| v.len() as i32)
    }

    // Java: void todoError()
    pub fn todoError(&mut self) {
        // Java: if (this.mAhLst != null)
        if self.mAhLst.is_some() {
            // Java: int shapeType = this.mShape.getShapeType();
            let shapeType = self.mShape.as_ref().map_or(0, |s| s.getShapeType());
            // Java: if (shapeType == 92 || shapeType == 60)
            if shapeType == 92 || shapeType == 60 {
                // Java: AdjustHandler ah = (AdjustHandler)this.mAhLst.get(0);
                let ah = self.mAhLst.as_mut().and_then(|v| v.get_mut(0).and_then(|a| a.as_mut()));
                // Java: if (ah != null)
                if ah.is_some() {
                    // Java: SingleAdjustHandler sah = (SingleAdjustHandler)ah;
                    let ah = ah.unwrap();
                    let mut sah = SingleAdjustHandler::new(ah.r#type);
                    sah.base.x = ah.x;
                    sah.base.y = ah.y;
                    sah.base.gdX = ah.gdX;
                    sah.base.gdY = ah.gdY;
                    // Java: sah.adjustData.minValue = 50000.0F;
                    sah.adjustData.minValue = 50000.0_f32;
                    // Java: sah.adjustData.maxValue = 0.0F;
                    sah.adjustData.maxValue = 0.0_f32;
                    // Java: sah.adjustData.coordinateMinValue = 0.0F;
                    sah.adjustData.coordinateMinValue = 0.0_f32;
                    *ah = sah.base;
                }
            }
        }
    }

    // Java: AdjustHandler getAdjustHandler(int i)
    pub fn getAdjustHandler(&mut self, i: i32) -> Option<AdjustHandler> {
        // Java: int count = this.getAdjustCount();
        let count = self.getAdjustCount();
        // Java: return i >= count ? null : (AdjustHandler)this.mAhLst.elementAt(i);
        if i >= count {
            None
        } else {
            self.mAhLst
                .as_ref()
                .and_then(|v| v.get(i as usize))
                .and_then(|a| a.clone())
        }
    }

    // Java: PointF getAdjustPos(int i)
    pub fn getAdjustPos(&mut self, i: i32) -> Option<PointF> {
        // Java: int count = this.getAdjustCount();
        let count = self.getAdjustCount();
        // Java: if (i >= count)
        if i >= count {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: AdjustHandler ah = (AdjustHandler)this.mAhLst.elementAt(i);
            let ah = self
                .mAhLst
                .as_ref()
                .and_then(|v| v.get(i as usize))
                .and_then(|a| a.as_ref());
            // Java: return ah == null ? null : new PointF(ah.x, ah.y);
            let ah = ah;
            if ah.is_none() {
                None
            } else {
                ah.map(|ah| PointF::new_xy(ah.x, ah.y))
            }
        }
    }

    // Java: PointF[] getAdjustPoints()
    pub fn getAdjustPoints(&mut self) -> Option<Vec<Option<PointF>>> {
        // Java: int count = this.getAdjustCount();
        let count = self.getAdjustCount();
        // Java: if (count < 1)
        if count < 1 {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: PointF[] pts = new PointF[count];
            let mut pts = new_pointf_array(count);
            // Java: for(int i = 0; i < count; ++i)
            for i in 0..count {
                // Java: AdjustHandler ah = (AdjustHandler)this.mAhLst.elementAt(i);
                let ah = self
                    .mAhLst
                    .as_ref()
                    .and_then(|v| v.get(i as usize))
                    .and_then(|a| a.as_ref());
                // Java: pts[i] = ah == null ? null : new PointF(ah.x, ah.y);
                pts[i as usize] = ah.map(|ah| PointF::new_xy(ah.x, ah.y));
            }
            // Java: return pts;
            Some(pts)
        }
    }

    // Java: RectF getRect()
    pub fn getRect(&self) -> RectF {
        // Java: return new RectF(this.mPresetData._rect);
        self.mPresetData
            .as_ref()
            .and_then(|p| p.base._rect.as_ref())
            .map(|r| RectF::new_from(r))
            .unwrap_or_else(RectF::new)
    }

    // Java: static boolean isNotSupportAdjust(Shape shape)
    pub fn isNotSupportAdjust(shape: &mut Shape) -> bool {
        // Java: int shapeType = shape.getShapeType();
        let shapeType = shape.getShapeType();
        // Java: for(int i = 0; i < mNotSupportAdjustShaps.length; ++i)
        for i in 0..PresetShape::NOT_SUPPORT_ADJUST_SHAPES.len() {
            // Java: if (mNotSupportAdjustShaps[i] == shapeType)
            if PresetShape::NOT_SUPPORT_ADJUST_SHAPES[i] == shapeType {
                // Java: return true;
                return true;
            }
        }
        // Java: return (shapeType == 100 || shapeType == 99) && shape.getPicture() != null;
        (shapeType == 100 || shapeType == 99) && shape.getPicture().is_some()
    }
}
