#![allow(non_snake_case, unused_variables, unused_mut)]

use std::collections::HashMap;

use super::PathData::PathData;
use super::AdjustHandler::AdjustHandler;
use super::CDL::CDL;
use super::CFL::CFL;
use super::CCL::CCL;
use crate::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath;

// Java: public class PresetData extends PathData
pub struct PresetData {
    pub base: PathData,
    // Java field: int _avcCount
    pub(crate) _avcCount: i32,
    // Java field: int[] _originCxnList
    pub(crate) _originCxnList: Option<Vec<i32>>,
    // Java field: float[] _cxnList
    pub(crate) _cxnList: Option<Vec<f32>>,
    // Java field: boolean _needCalculateAdjustData
    pub(crate) _needCalculateAdjustData: bool,
    // Java field: int[] _avcLst
    pub(crate) _avcLst: Option<Vec<i32>>,
    // Java field: Vector<AdjustHandler> _ahList
    pub(crate) _ahList: Option<Vec<AdjustHandler>>,
    // Java field: int[][] _pathList
    pub(crate) _pathList: Option<Vec<Vec<i32>>>,
}

// Java: public class PresetData extends PathData
impl PresetData {
    // Java: PresetData(int shapeType)
    pub fn new_with_type(mut shapeType: i32) -> Self {
        // Java: super();
        let mut pd = PresetData {
            base: PathData::new(),
            // Java: int _avcCount = 0
            _avcCount: 0,
            _originCxnList: None,
            _cxnList: None,
            // Java: boolean _needCalculateAdjustData = true
            _needCalculateAdjustData: true,
            _avcLst: None,
            _ahList: None,
            _pathList: None,
        };
        // Java: if (shapeType == 100)
        if shapeType == 100 {
            // Java: shapeType = 99;
            shapeType = 99;
        }
        // Java: this.setType(shapeType);
        pd.base.setType(shapeType);
        // Java: PreDefData data = CDL.getDefData(shapeType);
        let data = CDL::getDefData(shapeType);
        // Java: this.parsePreDefData(data);
        pd.parsePreDefData(&data);
        pd
    }

    // Java: PresetData()
    pub fn new() -> Self {
        // Java: super();
        PresetData {
            base: PathData::new(),
            _avcCount: 0,
            _originCxnList: None,
            _cxnList: None,
            _needCalculateAdjustData: true,
            _avcLst: None,
            _ahList: None,
            _pathList: None,
        }
    }

    // Java: void destroy()
    pub fn destroy(&mut self) {
        // Java: super.destroy();
        self.base.destroy();
        // Java: this._rect = null;
        self.base._rect = None;
        // Java: this._varList = null;
        self.base._varList = Vec::new();
        // Java: this._originCxnList = null;
        self._originCxnList = None;
        // Java: this._cxnList = null;
        self._cxnList = None;
        // Java: this._ahList = null;
        self._ahList = None;
        // Java: this._pathList = null;
        self._pathList = None;
    }

    // Java: void reset(float w, float h)
    pub fn reset(&mut self, w: f32, h: f32) {
        // Java: if (w != this._rect.width() || h != this._rect.height())
        if let Some(ref mut rect) = self.base._rect {
            if w != rect.width() || h != rect.height() {
                // Java: this._rect.right = this._rect.left + w;
                rect.right = rect.left + w;
                // Java: this._rect.bottom = this._rect.top + h;
                rect.bottom = rect.top + h;
                // Java: this._needCalculateAdjustData = true;
                self._needCalculateAdjustData = true;
            }
        }
    }

    // Java: float[] getCxnLst()
    pub fn getCxnLst(&mut self) -> Option<&Vec<f32>> {
        // Java: Map<Integer, Float> gdValueBuf = new HashMap();
        let mut gdValueBuf: HashMap<i32, f32> = HashMap::new();
        // Java: int cxnNum = this._originCxnList.length;
        let cxnNum = self._originCxnList.as_ref().map(|v| v.len()).unwrap_or(0);
        // Java: this._cxnList = new float[cxnNum];
        let mut cxn_list = vec![0.0_f32; cxnNum];
        // Java: for(int i = 0; i < cxnNum; ++i)
        for i in 0..cxnNum {
            // Java: this._cxnList[i] = this.getValue(this._originCxnList[i], gdValueBuf, this._varList);
            let origin_val = self._originCxnList.as_ref().unwrap()[i];
            cxn_list[i] = self.getValue(origin_val, &mut gdValueBuf, &self.base._varList.clone());
        }
        self._cxnList = Some(cxn_list);
        // Java: return this._cxnList;
        self._cxnList.as_ref()
    }

    // Java: int[][] getPathList()
    pub fn getPathList(&self) -> Option<&Vec<Vec<i32>>> {
        // Java: return this._pathList;
        self._pathList.as_ref()
    }

    // Java: Vector<AdjustHandler> getAhLst()
    pub fn getAhLst(&mut self) -> Option<&Vec<AdjustHandler>> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: Map<Integer, Float> gdValueBuf = new HashMap();
        // Java: AdjustHandler ah = null;
        // Java: AdjustData ad = null;
        // Java: int i = 0;
        // Java: for(int ahListSize = this._ahList.size(); i < ahListSize; ++i)
        // Java: ah = (AdjustHandler)this._ahList.elementAt(i);
        // Java: ah.x = this.getValue((int)ah.gdX, gdValueBuf, this._varList);
        // Java: ah.y = this.getValue((int)ah.gdY, gdValueBuf, this._varList);
        // Java: if (ah.type != 3 && ah.type != 6)
        // Java: ad = ((SingleAdjustHandler)ah).adjustData;
        // Java: ad.value = (float)this._varList[ad.index];
        // Java: else
        // Java: ad = ((CoupleAdjustHandler)ah).firstAdjustData;
        // Java: ad.value = (float)this._varList[ad.index];
        // Java: ad = ((CoupleAdjustHandler)ah).secondAdjustData;
        // Java: ad.value = (float)this._varList[ad.index];
        // Java: if (this._needCalculateAdjustData)
        // Java: int[] varList = this.getSetVarList((int[])null);
        // Java: i = 0;
        // Java: for(int ahListSize = this._ahList.size(); i < ahListSize; ++i)
        // Java: ah = (AdjustHandler)this._ahList.elementAt(i);
        // Java: if (ah.type == 1)
        // Java: this._calculateCartesianAdjustData((int)ah.gdX, ((SingleAdjustHandler)ah).adjustData, varList);
        // Java: else if (ah.type == 2)
        // Java: this._calculateCartesianAdjustData((int)ah.gdY, ((SingleAdjustHandler)ah).adjustData, varList);
        // Java: else if (ah.type == 3)
        // Java: this._calculateCartesianAdjustData((int)ah.gdX, ((CoupleAdjustHandler)ah).firstAdjustData, varList);
        // Java: this._calculateCartesianAdjustData((int)ah.gdY, ((CoupleAdjustHandler)ah).secondAdjustData, varList);
        // Java: else if (ah.type == 5)
        // Java: this._calculatePolarRadiusAdjustData((int)ah.gdX, (int)ah.gdY, ((SingleAdjustHandler)ah).adjustData, varList);
        // Java: else if (ah.type == 4)
        // Java: this._calculatePolarAngleAdjustData(((SingleAdjustHandler)ah).adjustData, varList);
        // Java: else if (ah.type == 6)
        // Java: this._calculatePolarAngleAdjustData(((CoupleAdjustHandler)ah).firstAdjustData, varList);
        // Java: this._calculatePolarRadiusAdjustData((int)ah.gdX, (int)ah.gdY, ((CoupleAdjustHandler)ah).secondAdjustData, varList);
        // Java: this.getSetVarList(varList);
        // Java: this._needCalculateAdjustData = false;
        // Java: return this._ahList;
        */
    }

    // Java: int[] getAdjusts()
    pub fn getAdjusts(&self) -> Option<&Vec<i32>> {
        // Java: return this._avcLst;
        self._avcLst.as_ref()
    }

    // Java: synchronized int[] getSetVarList(int[] varList)
    pub fn getSetVarList(&mut self, var_list: Option<Vec<i32>>) -> Vec<i32> {
        // Java: if (varList == null)
        if var_list.is_none() {
            // Java: varList = new int[this._varList.length];
            let mut varList = vec![0i32; self.base._varList.len()];
            // Java: System.arraycopy(this._varList, 0, varList, 0, this._varList.length);
            varList.copy_from_slice(&self.base._varList);
            // Java: return varList;
            return varList;
        }
        // Java: else
        {
            // Java: System.arraycopy(varList, 0, this._varList, 0, varList.length);
            let vl = var_list.unwrap();
            let len = vl.len().min(self.base._varList.len());
            self.base._varList[..len].copy_from_slice(&vl[..len]);
            // Java: return this._varList;
            self.base._varList.clone()
        }
    }

    // Java: GraphicsPath[] calcShapePath()
    pub fn calcShapePath(&mut self) -> Option<Vec<GraphicsPath>> {
        // Java: this._needCalculateAdjustData = true;
        self._needCalculateAdjustData = true;
        // Java: Map<Integer, Float> gdValueBuf = new HashMap();
        let mut gdValueBuf: HashMap<i32, f32> = HashMap::new();
        // Java: int[] varList = this.getSetVarList((int[])null);
        let var_list = self.getSetVarList(None);
        // Java: return CCL.generatePath(this, gdValueBuf, varList);
        CCL::generatePath(self, &mut gdValueBuf, &var_list)
    }

    // Java: float getValue(int pcArg, Map<Integer, Float> gdValueBuf, int[] varList)
    pub fn getValue(&self, pcArg: i32, gdValueBuf: &mut HashMap<i32, f32>, varList: &[i32]) -> f32 {
        // Java: if (CFL.isFormulaIndex(pcArg))
        if CFL::isFormulaIndex(pcArg) {
            // Java: return CFL.getSymbolicValue(pcArg, this._rect);
            let rect = self.base._rect.as_ref().expect("PresetData._rect");
            return CFL::getSymbolicValue(pcArg, rect);
        // Java: else
        } else {
            // Java: int avcGdIndex = CFL.getAvcGdIndex(pcArg);
            let avcGdIndex = CFL::getAvcGdIndex(pcArg);
            // Java: int avcGdValue = varList[avcGdIndex];
            let avcGdValue = varList[avcGdIndex as usize];
            // Java: if (avcGdIndex < this._avcCount)
            if avcGdIndex < self._avcCount {
                // Java: return (float)avcGdValue;
                avcGdValue as f32
            // Java: else
            } else {
                // Java: Float valueBuffer = (Float)gdValueBuf.get(avcGdIndex);
                // Java: if (valueBuffer != null && !valueBuffer.isNaN())
                if let Some(&valueBuffer) = gdValueBuf.get(&avcGdIndex) {
                    if !valueBuffer.is_nan() {
                        // Java: return valueBuffer;
                        return valueBuffer;
                    }
                }
                // Java: else
                // Java: int formulaIndex = CFL.getFormulaIndex(avcGdValue);
                let formulaIndex = CFL::getFormulaIndex(avcGdValue);
                // Java: int argCnt = CFL.getArgCnt(avcGdValue);
                let argCnt = CFL::getArgCnt(avcGdValue);
                // Java: float[] args = new float[argCnt];
                let mut args = vec![0.0_f32; argCnt as usize];
                // Java: for(int i = 0; i < argCnt; ++i)
                for i in 0..argCnt {
                    // Java: args[i] = this.getValue(varList[avcGdIndex + i + 1], gdValueBuf, varList);
                    args[i as usize] = self.getValue(varList[(avcGdIndex + i + 1) as usize], gdValueBuf, varList);
                }
                // Java: Integer key = avcGdIndex;
                let key = avcGdIndex;
                // Java: Float value = CFL.getFormulasValue(formulaIndex, args);
                let value = CFL::getFormulasValue(formulaIndex, &args);
                // Java: gdValueBuf.put(key, value);
                gdValueBuf.insert(key, value);
                // Java: return (Float)gdValueBuf.get(key);
                *gdValueBuf.get(&key).unwrap()
            }
        }
    }

    // Java: void parsePreDefData(PreDefData preDefData)
    pub fn parsePreDefData(&mut self, preDefData: &super::PreDefData::PreDefData) {
        // Java: int i = 0;
        let _i = 0;
        // Java: int[] avcLst = preDefData.avcLst;
        let avcLst = &preDefData.avcLst;
        // Java: int[] gdLst = preDefData.gdLst;
        let gdLst = &preDefData.gdLst;
        // Java: if (null != gdLst && null != avcLst)
        {
            // Java: this._avcCount = avcLst.length;
            self._avcCount = avcLst.len() as i32;
            // Java: int gdCount = gdLst.length;
            let gdCount = gdLst.len();
            // Java: this._varList = new int[this._avcCount + gdCount];
            let mut var_list = vec![0i32; self._avcCount as usize + gdCount];
            // Java: this._avcLst = new int[this._avcCount];
            self._avcLst = Some(vec![0i32; self._avcCount as usize]);
            // Java: System.arraycopy(avcLst, 0, this._varList, 0, this._avcCount);
            var_list[..avcLst.len()].copy_from_slice(avcLst);
            // Java: System.arraycopy(avcLst, 0, this._avcLst, 0, this._avcCount);
            if let Some(ref mut avc) = self._avcLst {
                avc.copy_from_slice(avcLst);
            }
            // Java: System.arraycopy(gdLst, 0, this._varList, this._avcCount, gdCount);
            var_list[self._avcCount as usize..].copy_from_slice(gdLst);
            self.base._varList = var_list;
        }
        // Java: int[] defCxnLst = preDefData.cxnLst;
        let defCxnLst = &preDefData.cxnLst;
        // Java: if (defCxnLst != null)
        {
            // Java: int cxnCount = defCxnLst.length;
            let cxnCount = defCxnLst.len();
            // Java: this._originCxnList = new int[cxnCount];
            // Java: System.arraycopy(defCxnLst, 0, this._originCxnList, 0, cxnCount);
            let mut origin = vec![0i32; cxnCount];
            origin.copy_from_slice(defCxnLst);
            self._originCxnList = Some(origin);
        }
        // Java: int[] defAhLst = preDefData.ahLst;
        let defAhLst = &preDefData.ahLst;
        // Java: if (defAhLst != null)
        // Java: this._ahList = new Vector();
        // Java: int ahCount = defAhLst.length;
        let ahCount = defAhLst.len();
        let _ = ahCount;
        /* DEFERRED_NON_FUNCTION
        // Java: for(int var12 = 0; var12 < ahCount; ++var12)
        // Java: int type = preDefData.ahLst[var12];
        // Java: switch (type)
        // Java: case 1:
        // Java: case 2:
        // Java: case 4:
        // Java: case 5:
        // Java: SingleAdjustHandler singleAh = new SingleAdjustHandler(type);
        // Java: this._loadAdjustData(preDefData, var12, singleAh.adjustData);
        // Java: singleAh.gdX = (float)preDefData.ahLst[var12 + 4];
        // Java: singleAh.gdY = (float)preDefData.ahLst[var12 + 5];
        // Java: var12 += 5;
        // Java: this._ahList.add(singleAh);
        // Java: break;
        // Java: case 3:
        // Java: case 6:
        // Java: CoupleAdjustHandler coupleAd = new CoupleAdjustHandler(type);
        // Java: this._loadAdjustData(preDefData, var12, coupleAd.firstAdjustData);
        // Java: this._loadAdjustData(preDefData, var12 + 3, coupleAd.secondAdjustData);
        // Java: coupleAd.gdX = (float)preDefData.ahLst[var12 + 7];
        // Java: coupleAd.gdY = (float)preDefData.ahLst[var12 + 8];
        // Java: var12 += 8;
        // Java: this._ahList.add(coupleAd);
        */
        // Java: int[] defRect = preDefData.rect;
        let defRect = &preDefData.rect;
        // Java: if (defRect != null)
        {
            // Java: int rectCount = defRect.length;
            let rectCount = defRect.len();
            // Java: this._textBoxRectList = new int[rectCount];
            // Java: System.arraycopy(defRect, 0, this._textBoxRectList, 0, rectCount);
            self.base._textBoxRectList = vec![0i32; rectCount];
            self.base._textBoxRectList.copy_from_slice(defRect);
        }
        // Java: int[][] pathLst = preDefData.pathLst;
        let pathLst = &preDefData.pathLst;
        // Java: int pathCount = pathLst.length;
        let pathCount = pathLst.len();
        // Java: this._pathList = new int[pathCount][];
        let mut path_list: Vec<Vec<i32>> = Vec::with_capacity(pathCount);
        // Java: for(int var13 = 0; var13 < pathCount; ++var13)
        for var13 in 0..pathCount {
            // Java: int elemCount = pathLst[var13].length;
            let elemCount = pathLst[var13].len();
            // Java: int[] path = new int[elemCount];
            let mut path = vec![0i32; elemCount];
            // Java: System.arraycopy(pathLst[var13], 0, path, 0, elemCount);
            path.copy_from_slice(&pathLst[var13]);
            // Java: this._pathList[var13] = path;
            path_list.push(path);
        }
        self._pathList = Some(path_list);
    }

    // Java: void _loadAdjustData(PreDefData preDefData, int i, AdjustData adjustData)
    fn _loadAdjustData(&mut self, preDefData: &super::PreDefData::PreDefData, i: usize, adjustData: &mut AdjustData) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: adjustData.index = preDefData.ahLst[i + 1] - 48;
        // Java: adjustData.originValue = (float)this._avcLst[adjustData.index];
        // Java: adjustData.value = (float)this._varList[adjustData.index];
        // Java: adjustData.minValueIndex = preDefData.ahLst[i + 2];
        // Java: adjustData.maxValueIndex = preDefData.ahLst[i + 3];
        */
    }

    // Java: void _calculateCartesianAdjustData(int gdIndex, AdjustData ad, int[] varList)
    fn _calculateCartesianAdjustData(&self, gdIndex: i32, ad: &mut AdjustData, varList: &mut Vec<i32>) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: Map<Integer, Float> gdValueBuf = new HashMap();
        // Java: ad.minValue = this.getValue(ad.minValueIndex, gdValueBuf, varList);
        // Java: ad.maxValue = this.getValue(ad.maxValueIndex, gdValueBuf, varList);
        // Java: int index = ad.index;
        // Java: int temp = varList[index];
        // Java: gdValueBuf.clear();
        // Java: ad.value = (float)varList[index];
        // Java: varList[index] = (int)ad.minValue;
        // Java: ad.coordinateMinValue = this.getValue(gdIndex, gdValueBuf, varList);
        // Java: gdValueBuf.clear();
        // Java: varList[index] = (int)ad.maxValue;
        // Java: ad.coordinateMaxValue = this.getValue(gdIndex, gdValueBuf, varList);
        // Java: gdValueBuf.clear();
        // Java: varList[index] = (int)ad.originValue;
        // Java: ad.coordinateOriginValue = this.getValue(gdIndex, gdValueBuf, varList);
        // Java: varList[index] = temp;
        */
    }

    // Java: void _calculatePolarRadiusAdjustData(int gdX, int gdY, AdjustData ad, int[] varList)
    fn _calculatePolarRadiusAdjustData(&self, gdX: i32, gdY: i32, ad: &mut AdjustData, varList: &mut Vec<i32>) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: Map<Integer, Float> gdValueBuf = new HashMap();
        // Java: ad.minValue = this.getValue(ad.minValueIndex, gdValueBuf, varList);
        // Java: ad.maxValue = this.getValue(ad.maxValueIndex, gdValueBuf, varList);
        // Java: int index = ad.index;
        // Java: int temp = varList[index];
        // Java: float centerX = this._rect.width() / 2.0F;
        // Java: float centerY = this._rect.height() / 2.0F;
        // Java: gdValueBuf.clear();
        // Java: varList[index] = (int)ad.minValue;
        // Java: float dx = this.getValue(gdX, gdValueBuf, varList) - centerX;
        // Java: float dy = this.getValue(gdY, gdValueBuf, varList) - centerY;
        // Java: ad.coordinateMinValue = (float)Math.sqrt((double)(dx * dx + dy * dy));
        // Java: gdValueBuf.clear();
        // Java: varList[index] = (int)ad.maxValue;
        // Java: dx = this.getValue(gdX, gdValueBuf, varList) - centerX;
        // Java: dy = this.getValue(gdY, gdValueBuf, varList) - centerY;
        // Java: ad.coordinateMaxValue = (float)Math.sqrt((double)(dx * dx + dy * dy));
        // Java: gdValueBuf.clear();
        // Java: varList[index] = (int)ad.originValue;
        // Java: dx = this.getValue(gdX, gdValueBuf, varList) - centerX;
        // Java: dy = this.getValue(gdY, gdValueBuf, varList) - centerY;
        // Java: ad.coordinateOriginValue = (float)Math.sqrt((double)(dx * dx + dy * dy));
        // Java: varList[index] = temp;
        */
    }

    // Java: void _calculatePolarAngleAdjustData(AdjustData ad, int[] varList)
    fn _calculatePolarAngleAdjustData(&self, ad: &mut AdjustData, varList: &mut Vec<i32>) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: Map<Integer, Float> gdValueBuf = new HashMap();
        // Java: ad.minValue = this.getValue(ad.minValueIndex, gdValueBuf, varList);
        // Java: ad.maxValue = this.getValue(ad.maxValueIndex, gdValueBuf, varList);
        */
    }
}

pub struct AdjustData {
    pub index: usize,
    pub originValue: f32,
    pub value: f32,
    pub minValueIndex: i32,
    pub maxValueIndex: i32,
    pub minValue: f32,
    pub maxValue: f32,
    pub coordinateMinValue: f32,
    pub coordinateMaxValue: f32,
    pub coordinateOriginValue: f32,
}
