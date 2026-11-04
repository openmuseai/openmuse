#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import android.util.SparseArray
// Java: import cn.wps.drawing.data.geometry.GeomGuide
use crate::cn::wps::drawing::data::geometry::GeomGuide::GeomGuide;
// Java: import cn.wps.moffice.drawing.smartshape.GeoAdjust
use drawing::cn::wps::moffice::drawing::smartshape::GeoAdjust::GeoAdjust;
// Java: import cn.wps.moffice.drawing.smartshape.GeoFormula
use drawing::cn::wps::moffice::drawing::smartshape::GeoFormula::GeoFormula;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.regex.Pattern

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

struct IntPattern;
impl IntPattern {
    fn is_match(&self, s: &str) -> bool {
        if s.is_empty() { return false; }
        let bytes = s.as_bytes();
        let start = if bytes[0] == b'-' { 1 } else { 0 };
        if start >= bytes.len() { return false; }
        bytes[start..].iter().all(|b| b.is_ascii_digit())
    }
}
static INT_PATTERN_RE: IntPattern = IntPattern;

// Java field: char SPLITER
pub(crate) const SPLITER: char = ' ';
// Java field: float DML_ANGLE_BASE
pub(crate) const DML_ANGLE_BASE: f32 = 60000.0;

// Java field: Pattern INT_PATTERN
// Java field: HashMap<String, Integer> mNameMap
// Java field: HashMap<String, Integer> mBuildInGuides

fn get_mNameMap() -> &'static HashMap<String, i32> {
    static MAP: OnceLock<HashMap<String, i32>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("+-".to_string(), 0);
        m.insert("*/".to_string(), 1);
        m.insert("+/".to_string(), 2);
        m.insert("abs".to_string(), 3);
        m.insert("min".to_string(), 4);
        m.insert("max".to_string(), 5);
        m.insert("?:".to_string(), 6);
        m.insert("mod".to_string(), 7);
        m.insert("sqrt".to_string(), 13);
        m.insert("cat2".to_string(), 11);
        m.insert("sat2".to_string(), 12);
        m.insert("pin".to_string(), 17);
        m.insert("val".to_string(), 18);
        m.insert("at2".to_string(), 19);
        m.insert("sinatan".to_string(), 20);
        m.insert("cosatan".to_string(), 21);
        m.insert("tanatan".to_string(), 22);
        m
    })
}

fn get_mBuildInGuides() -> &'static HashMap<String, i32> {
    static MAP: OnceLock<HashMap<String, i32>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("w".to_string(), 0);
        m.insert("h".to_string(), 1);
        m.insert("wd2".to_string(), 2);
        m.insert("hd2".to_string(), 3);
        m.insert("cd2".to_string(), 15);
        m.insert("cd4".to_string(), 16);
        m.insert("cd8".to_string(), 17);
        m.insert("3cd4".to_string(), 18);
        m.insert("3cd8".to_string(), 19);
        m.insert("cd3".to_string(), 20);
        m.insert("cd6".to_string(), 21);
        m.insert("hd4".to_string(), 22);
        m.insert("hd5".to_string(), 23);
        m.insert("hd6".to_string(), 24);
        m.insert("hd8".to_string(), 25);
        m.insert("t".to_string(), 26);
        m.insert("l".to_string(), 26);
        m.insert("b".to_string(), 1);
        m.insert("r".to_string(), 0);
        m.insert("ls".to_string(), 27);
        m.insert("ss".to_string(), 28);
        m.insert("ssd2".to_string(), 29);
        m.insert("ssd4".to_string(), 30);
        m.insert("ssd6".to_string(), 31);
        m.insert("ssd8".to_string(), 32);
        m.insert("wd4".to_string(), 33);
        m.insert("wd5".to_string(), 34);
        m.insert("wd6".to_string(), 35);
        m.insert("wd8".to_string(), 36);
        m.insert("wd10".to_string(), 37);
        m.insert("hd3".to_string(), 38);
        m.insert("wd3".to_string(), 39);
        m.insert("ssd16".to_string(), 40);
        m.insert("ssd32".to_string(), 41);
        m.insert("hd7".to_string(), 42);
        m.insert("wd32".to_string(), 43);
        m.insert("7cd8".to_string(), 44);
        m.insert("hd10".to_string(), 45);
        m.insert("wd12".to_string(), 46);
        m
    })
}

// Java: public class FormulaParser
pub struct FormulaParser {
    // Java field: SparseArray<Double> mFmlResult
    mFmlResult: HashMap<i32, f64>,
    // Java field: HashMap<String, Integer> mAdjIndex
    mAdjIndex: HashMap<String, i32>,
    // Java field: HashMap<String, Integer> mFmlIndex
    mFmlIndex: HashMap<String, i32>,
    // Java field: List<String> mFormulaItems
    mFormulaItems: Vec<String>,
    // Java field: StringBuffer mFormulaBuffer
    mFormulaBuffer: String,
    // Java field: List<GeoFormula> mGeoFormulas
    mGeoFormulas: Vec<GeoFormula>,
    // Java field: boolean mInitBuildIn
    mInitBuildIn: bool,
}

// Java: public class FormulaParseResult
pub struct FormulaParseResult {
    // Java field: GeoAdjust mGeoAdjust
    pub(crate) mGeoAdjust: Option<GeoAdjust>,
    // Java field: GeoFormula[] mGeoFormula
    pub(crate) mGeoFormula: Option<Vec<GeoFormula>>,
}

// Java: public class FormulaParser
impl FormulaParser {
    pub fn new() -> Self {
        FormulaParser {
            mFmlResult: HashMap::new(),
            mAdjIndex: HashMap::new(),
            mFmlIndex: HashMap::new(),
            mFormulaItems: Vec::new(),
            mFormulaBuffer: String::new(),
            mGeoFormulas: Vec::new(),
            mInitBuildIn: false,
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mFmlResult.clear();
      self.mFmlResult.clear();
      // Java: this.mFormulaItems.clear();
      self.mFormulaItems.clear();
      // Java: this.mFormulaBuffer.setLength(0);
      self.mFormulaBuffer.clear();
      // Java: this.mGeoFormulas.clear();
      self.mGeoFormulas.clear();
      // Java: this.mAdjIndex.clear();
      self.mAdjIndex.clear();
      // Java: this.mFmlIndex.clear();
      self.mFmlIndex.clear();
    }

    // Java: double getValueByName(String name)
    pub fn getValueByName(&self, name: &str) -> f64 {
      // Java: Double v = (Double)this.mFmlResult.get(name.hashCode());
      let hash = Self::hash_code(name);
      let v = self.mFmlResult.get(&hash);
      // Java: return v != null ? v : (double)0.0F;
      v.copied().unwrap_or(0.0)
    }

    fn hash_code(s: &str) -> i32 {
        let mut h: i32 = 0;
        for c in s.chars() {
            h = h.wrapping_mul(31).wrapping_add(c as i32);
        }
        h
    }

    // Java: static float getBuildInVal(String name, float w, float h)
    pub fn getBuildInVal(name: &str, w: f32, h: f32) -> f32 {
      // Java: float value = 0.0F;
      let mut value = 0.0f32;
      // Java: Integer buildInGuide = (Integer)mBuildInGuides.get(name);
      let buildInGuide = get_mBuildInGuides().get(name);
      // Java: if (buildInGuide != null)
      if let Some(&guide) = buildInGuide
      {
         // Java: value = getGuildInValue(buildInGuide, w, h);
         value = Self::getGuildInValue(guide, w, h);
      }
      // Java: return value;
      value
    }

    // Java: FormulaParseResult parse(List<GeomGuide> adjustValues, float w, float h, boolean bFormula, boolean needReturnVal)
    pub fn parse(&mut self, adjustValues: &[GeomGuide], w: f32, h: f32, bFormula: bool, needReturnVal: bool) -> Option<FormulaParseResult> {
      // Java: this.mGeoFormulas.clear();
      self.mGeoFormulas.clear();
      // Java: this.mAdjIndex.clear();
      self.mAdjIndex.clear();
      // Java: if (!this.mInitBuildIn)
      if !self.mInitBuildIn
      {
         // Java: this.mInitBuildIn = true;
         self.mInitBuildIn = true;
         // Java: for(Map.Entry<String, Integer> entry : mBuildInGuides.entrySet())
         for (key, value) in get_mBuildInGuides().iter()
         {
            // Java: double val = (double)getGuildInValue((Integer)entry.getValue(), w, h);
            let val = Self::getGuildInValue(*value, w, h) as f64;
            // Java: this.mFmlResult.put(((String)entry.getKey()).hashCode(), val);
            self.mFmlResult.insert(Self::hash_code(key), val);
         }
      }
      // Java: GeoAdjust geoAdjust = null;
      let mut geoAdjust: Option<GeoAdjust> = None;
      // Java: if (!bFormula && needReturnVal)
      if !bFormula && needReturnVal
      {
         // Java: geoAdjust = new GeoAdjust();
         geoAdjust = Some(GeoAdjust::new());
      }
      // Java: int valIndex = 0;
      let mut valIndex: i32 = 0;
      // Java: int i = 0;
      let mut i = 0;
      // Java: for(int size = adjustValues.size(); i < size; ++i)
      let size = adjustValues.len();
      while i < size
      {
         // Java: GeomGuide guide = (GeomGuide)adjustValues.get(i);
         let guide = &adjustValues[i];
         // Java: String[] items = this.parseFmla(guide.fmla());
         let items = self.parseFmla(&guide.get_fmla());
         // Java: if (items.length != 0)
         if !items.is_empty()
         {
            // Java: double val = (double)0.0F;
            let mut val: f64 = 0.0;
            // Java: double x = (double)0.0F;
            let mut x: f64 = 0.0;
            // Java: double y = (double)0.0F;
            let mut y: f64 = 0.0;
            // Java: double z = (double)0.0F;
            let mut z: f64 = 0.0;
            // Java: int formualName = this.getFormualName(items[0]);
            let formualName = Self::getFormualName_static(&items[0]);
            // Java: switch (formualName)
            match formualName
            {
               // Java: case 0:
               0 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: z = this.parseItem(items[3], w, h);
                  z = self.parseItem(&items.get(3).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x + y - z;
                  val = x + y - z;
                  // Java: break;
               }
               // Java: case 1:
               1 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: z = this.parseItem(items[3], w, h);
                  z = self.parseItem(&items.get(3).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x * y / z;
                  val = if z != 0.0 { x * y / z } else { 0.0 };
                  // Java: break;
               }
               // Java: case 2:
               2 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: z = this.parseItem(items[3], w, h);
                  z = self.parseItem(&items.get(3).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = (x + y) / z;
                  val = if z != 0.0 { (x + y) / z } else { 0.0 };
                  // Java: break;
               }
               // Java: case 3:
               3 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = Math.abs(x);
                  val = x.abs();
                  // Java: break;
               }
               // Java: case 4:
               4 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x > y ? y : x;
                  val = if x > y { y } else { x };
                  // Java: break;
               }
               // Java: case 5:
               5 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x > y ? x : y;
                  val = if x > y { x } else { y };
                  // Java: break;
               }
               // Java: case 6:
               6 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: z = this.parseItem(items[3], w, h);
                  z = self.parseItem(&items.get(3).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x > (double)0.0F ? y : z;
                  val = if x > 0.0 { y } else { z };
                  // Java: break;
               }
               // Java: case 7:
               7 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: z = this.parseItem(items[3], w, h);
                  z = self.parseItem(&items.get(3).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = Math.sqrt(x * x + y * y + z * z);
                  val = (x * x + y * y + z * z).sqrt();
                  // Java: break;
               }
               // Java: case 8:
               // Java: case 9:
               // Java: case 10:
               // Java: case 14:
               // Java: case 15:
               // Java: case 16:
               // Java: default:
               8 | 9 | 10 | 14 | 15 | 16 | _ if formualName == 8 || formualName == 9 || formualName == 10 || formualName == 14 || formualName == 15 || formualName == 16 || (formualName < 0) => {
                  // Java: continue;
                  i += 1;
                  continue;
               }
               // Java: case 11:
               11 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: z = this.parseItem(items[3], w, h);
                  z = self.parseItem(&items.get(3).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x * Math.cos(Math.atan2(z, y));
                  val = x * (z.atan2(y)).cos();
                  // Java: break;
               }
               // Java: case 12:
               12 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: z = this.parseItem(items[3], w, h);
                  z = self.parseItem(&items.get(3).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x * Math.sin(Math.atan2(z, y));
                  val = x * (z.atan2(y)).sin();
                  // Java: break;
               }
               // Java: case 13:
               13 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = Math.sqrt(x);
                  val = x.sqrt();
                  // Java: break;
               }
               // Java: case 17:
               17 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: z = this.parseItem(items[3], w, h);
                  z = self.parseItem(&items.get(3).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = y < x ? x : (y > z ? z : y);
                  val = if y < x { x } else if y > z { z } else { y };
                  // Java: break;
               }
               // Java: case 18:
               18 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x;
                  val = x;
                  // Java: break;
               }
               // Java: case 19:
               19 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = Math.toDegrees(Math.atan2(y, x)) * (double)60000.0F;
                  val = y.atan2(x).to_degrees() * 60000.0;
                  // Java: break;
               }
               // Java: case 20:
               20 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x * Math.sin(Math.toRadians(y / (double)60000.0F));
                  val = x * (y / 60000.0).to_radians().sin();
                  // Java: break;
               }
               // Java: case 21:
               21 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x * Math.cos(Math.toRadians(y / (double)60000.0F));
                  val = x * (y / 60000.0).to_radians().cos();
                  // Java: break;
               }
               // Java: case 22:
               22 => {
                  // Java: x = this.parseItem(items[1], w, h);
                  x = self.parseItem(&items.get(1).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: y = this.parseItem(items[2], w, h);
                  y = self.parseItem(&items.get(2).map(|s| s.as_str()).unwrap_or(""), w, h);
                  // Java: val = x * Math.tan(Math.toRadians(y / (double)60000.0F));
                  val = x * (y / 60000.0).to_radians().tan();
               }
               _ => {
                  i += 1;
                  continue;
               }
            }
            // Java: this.mFmlResult.put(guide.name().hashCode(), val);
            self.mFmlResult.insert(Self::hash_code(&guide.get_name()), val);
            // Java: if (needReturnVal)
            if needReturnVal
            {
               // Java: if (bFormula)
               if bFormula
               {
                  // Java: int[] args = new int[]{(int)x, (int)y, (int)z};
                  let args = vec![x as i32, y as i32, z as i32];
                  // Java: GeoFormula formula = new GeoFormula();
                  let mut formula = GeoFormula::new();
                  // Java: formula.name = formualName;
                  formula.name = formualName;
                  // Java: formula.arguments = args;
                  formula.arguments = Some(args);
                  // Java: this.mGeoFormulas.add(valIndex, formula);
                  if (valIndex as usize) <= self.mGeoFormulas.len() {
                      self.mGeoFormulas.insert(valIndex as usize, formula);
                  } else {
                      self.mGeoFormulas.push(formula);
                  }
               }
               // Java: else
               else
               {
                  // Java: geoAdjust.setAdjust(valIndex, (int)val);
                  if let Some(ref mut adj) = geoAdjust {
                      adj.setAdjust(valIndex, val as i32);
                  }
                  // Java: this.mAdjIndex.put(guide.name(), valIndex);
                  self.mAdjIndex.insert(guide.get_name(), valIndex);
               }
            }
            // Java: ++valIndex;
            valIndex += 1;
         }
         i += 1;
      }
      // Java: FormulaParseResult result = null;
      let mut result: Option<FormulaParseResult> = None;
      // Java: if (needReturnVal)
      if needReturnVal
      {
         // Java: result = new FormulaParseResult();
         let mut res = FormulaParseResult { mGeoAdjust: None, mGeoFormula: None };
         // Java: if (bFormula)
         if bFormula
         {
            // Java: GeoFormula[] formulas = new GeoFormula[this.mGeoFormulas.size()];
            // Java: this.mGeoFormulas.toArray(formulas);
            let formulas: Vec<GeoFormula> = self.mGeoFormulas.drain(..).collect();
            // Java: result.mGeoFormula = formulas;
            res.mGeoFormula = Some(formulas);
            // Java: this.mGeoFormulas.clear();
            self.mGeoFormulas.clear();
         }
         // Java: else
         else
         {
            // Java: result.mGeoAdjust = geoAdjust;
            res.mGeoAdjust = geoAdjust;
         }
         result = Some(res);
      }
      // Java: return result;
      result
    }

    // Java: FormulaParseResult parseAdjust(List<GeomGuide> adjustValues, float w, float h)
    pub fn parseAdjust(&mut self, adjustValues: &[GeomGuide], w: f32, h: f32) -> Option<FormulaParseResult> {
      // Java: return this.parse(adjustValues, w, h, false, true);
      self.parse(adjustValues, w, h, false, true)
    }

    // Java: FormulaParseResult parseFormula(List<GeomGuide> gd, float w, float h)
    pub fn parseFormula(&mut self, gd: &[GeomGuide], w: f32, h: f32) -> Option<FormulaParseResult> {
      // Java: return this.parse(gd, w, h, true, true);
      self.parse(gd, w, h, true, true)
    }

    // Java: void parseAdjustNonReturn(List<GeomGuide> adjustValues, float w, float h)
    pub fn parseAdjustNonReturn(&mut self, adjustValues: &[GeomGuide], w: f32, h: f32) {
      // Java: this.parse(adjustValues, w, h, false, true);
      self.parse(adjustValues, w, h, false, true);
    }

    // Java: void parseFormulaNonReturn(List<GeomGuide> gd, float w, float h)
    pub fn parseFormulaNonReturn(&mut self, gd: &[GeomGuide], w: f32, h: f32) {
      // Java: this.parse(gd, w, h, true, true);
      self.parse(gd, w, h, true, true);
    }

    // Java: double parseItem(String item, float w, float h)
    pub fn parseItem(&self, item: &str, w: f32, h: f32) -> f64 {
      // Java: double value = (double)0.0F;
      let mut value: f64 = 0.0;
      // Java: try
      match item.parse::<i32>()
      {
         // Java: value = (double)Integer.parseInt(item);
         Ok(v) => { value = v as f64; }
         // Java: catch (NumberFormatException var8)
         Err(_) =>
         {
            // Java: Double v = (Double)this.mFmlResult.get(item.hashCode());
            let hash = Self::hash_code(item);
            let v = self.mFmlResult.get(&hash);
            // Java: value = v == null ? (double)0.0F : v;
            value = v.copied().unwrap_or(0.0);
         }
      }
      // Java: return value;
      value
    }

    // Java: static float getGuildInValue(int bulidInType, float w, float h)
    pub fn getGuildInValue(bulidInType: i32, w: f32, h: f32) -> f32 {
      // Java: switch (bulidInType)
      match bulidInType
      {
         // Java: case 0:
         0 => {
            // Java: return w;
            return w;
         }
         // Java: case 1:
         1 => {
            // Java: return h;
            return h;
         }
         // Java: case 2:
         2 => {
            // Java: return w * 0.5F;
            return w * 0.5;
         }
         // Java: case 3:
         3 => {
            // Java: return h * 0.5F;
            return h * 0.5;
         }
         // Java: case 4:
         // Java: case 5:
         // Java: case 6:
         // Java: case 7:
         // Java: case 8:
         // Java: case 9:
         // Java: case 10:
         // Java: case 11:
         // Java: case 12:
         // Java: case 13:
         // Java: case 14:
         // Java: default:
         4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 => {
            // Java: return 0.0F;
            return 0.0;
         }
         // Java: case 15:
         15 => {
            // Java: return 1.62E7F;
            return 1.62e7;
         }
         // Java: case 16:
         16 => {
            // Java: return 8100000.0F;
            return 8100000.0;
         }
         // Java: case 17:
         17 => {
            // Java: return 1350000.0F;
            return 1350000.0;
         }
         // Java: case 18:
         18 => {
            // Java: return 1.89E7F;
            return 1.89e7;
         }
         // Java: case 19:
         19 => {
            // Java: return 1.08E7F;
            return 1.08e7;
         }
         // Java: case 20:
         20 => {
            // Java: return 5400000.0F;
            return 5400000.0;
         }
         // Java: case 21:
         21 => {
            // Java: return 2700000.0F;
            return 2700000.0;
         }
         // Java: case 22:
         22 => {
            // Java: return h / 4.0F;
            return h / 4.0;
         }
         // Java: case 23:
         23 => {
            // Java: return h / 5.0F;
            return h / 5.0;
         }
         // Java: case 24:
         24 => {
            // Java: return h / 6.0F;
            return h / 6.0;
         }
         // Java: case 25:
         25 => {
            // Java: return h / 8.0F;
            return h / 8.0;
         }
         // Java: case 26:
         26 => {
            // Java: return 0.0F;
            return 0.0;
         }
         // Java: case 27:
         27 => {
            // Java: return Math.max(w, h);
            return w.max(h);
         }
         // Java: case 28:
         28 => {
            // Java: return Math.min(w, h);
            return w.min(h);
         }
         // Java: case 29:
         29 => {
            // Java: return Math.min(w, h) * 0.5F;
            return w.min(h) * 0.5;
         }
         // Java: case 30:
         30 => {
            // Java: return Math.min(w, h) / 4.0F;
            return w.min(h) / 4.0;
         }
         // Java: case 31:
         31 => {
            // Java: return Math.min(w, h) / 6.0F;
            return w.min(h) / 6.0;
         }
         // Java: case 32:
         32 => {
            // Java: return Math.min(w, h) / 8.0F;
            return w.min(h) / 8.0;
         }
         // Java: case 33:
         33 => {
            // Java: return w / 4.0F;
            return w / 4.0;
         }
         // Java: case 34:
         34 => {
            // Java: return w / 5.0F;
            return w / 5.0;
         }
         // Java: case 35:
         35 => {
            // Java: return w / 6.0F;
            return w / 6.0;
         }
         // Java: case 36:
         36 => {
            // Java: return w / 8.0F;
            return w / 8.0;
         }
         // Java: case 37:
         37 => {
            // Java: return w / 10.0F;
            return w / 10.0;
         }
         // Java: case 38:
         38 => {
            // Java: return h / 3.0F;
            return h / 3.0;
         }
         // Java: case 39:
         39 => {
            // Java: return w / 3.0F;
            return w / 3.0;
         }
         // Java: case 40:
         40 => {
            // Java: return Math.min(w, h) / 16.0F;
            return w.min(h) / 16.0;
         }
         // Java: case 41:
         41 => {
            // Java: return Math.min(w, h) / 32.0F;
            return w.min(h) / 32.0;
         }
         // Java: case 42:
         42 => {
            // Java: return h / 7.0F;
            return h / 7.0;
         }
         // Java: case 43:
         43 => {
            // Java: return w / 32.0F;
            return w / 32.0;
         }
         // Java: case 44:
         44 => {
            // Java: return 7200000.0F;
            return 7200000.0;
         }
         // Java: case 45:
         45 => {
            // Java: return h / 10.0F;
            return h / 10.0;
         }
         // Java: case 46:
         46 => {
            // Java: return w / 12.0F;
            return w / 12.0;
         }
         _ => {
            return 0.0;
         }
      }
    }

    // Java: String[] parseFmla(String fmla)
    pub fn parseFmla(&mut self, fmla: &str) -> Vec<String> {
      // Java: this.mFormulaItems.clear();
      self.mFormulaItems.clear();
      // Java: this.mFormulaBuffer.setLength(0);
      self.mFormulaBuffer.clear();
      // Java: int listIndex = 0;
      let mut listIndex = 0;
      // Java: int i = 0;
      let mut i = 0;
      // Java: for(int size = fmla.length(); i < size; ++i)
      let chars: Vec<char> = fmla.chars().collect();
      let size = chars.len();
      while i < size
      {
         // Java: char c = fmla.charAt(i);
         let c = chars[i];
         // Java: if (c != ' ')
         if c != ' '
         {
            // Java: this.mFormulaBuffer.append(c);
            self.mFormulaBuffer.push(c);
         }
         // Java: else
         else
         {
            // Java: this.mFormulaItems.add(listIndex, this.mFormulaBuffer.toString());
            self.mFormulaItems.push(self.mFormulaBuffer.clone());
            // Java: this.mFormulaBuffer.setLength(0);
            self.mFormulaBuffer.clear();
            // Java: ++listIndex;
            listIndex += 1;
         }
         i += 1;
      }
      // Java: this.mFormulaItems.add(listIndex, this.mFormulaBuffer.toString());
      self.mFormulaItems.push(self.mFormulaBuffer.clone());
      // Java: String[] fmlas = new String[this.mFormulaItems.size()];
      // Java: this.mFormulaItems.toArray(fmlas);
      let fmlas = self.mFormulaItems.clone();
      // Java: return fmlas;
      fmlas
    }

    // Java: int getFormualName(String name)
    pub fn getFormualName(&self, name: &str) -> i32 {
      // Java: Integer val = (Integer)mNameMap.get(name);
      let val = get_mNameMap().get(name);
      // Java: return val != null ? val : -1;
      val.copied().unwrap_or(-1)
    }

    fn getFormualName_static(name: &str) -> i32 {
      let val = get_mNameMap().get(name);
      val.copied().unwrap_or(-1)
    }

    // Java: boolean isNumeric(String str)
    pub fn isNumeric(str_val: &str) -> bool {
      // Java: return INT_PATTERN.matcher(str).matches();
      INT_PATTERN_RE.is_match(str_val)
    }

}

// Java: public class FormulaParseResult
impl FormulaParseResult {
}
