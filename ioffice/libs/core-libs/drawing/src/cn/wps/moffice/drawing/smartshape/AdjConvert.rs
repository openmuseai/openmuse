#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::Mutex;

use super::GeoAdjust::GeoAdjust;
use super::AdjVal::AdjVal;

// Java: import java.lang.reflect.InvocationTargetException
// Java: import java.lang.reflect.Method
// Java: import java.util.Map
// Java: import java.util.TreeMap

// Java field: Map<Integer, String> funcMap
static funcMap: OnceLock<Mutex<HashMap<i32, &'static str>>> = OnceLock::new();

fn get_func_map() -> &'static Mutex<HashMap<i32, &'static str>> {
    funcMap.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(96, "smileyFace");
        map.insert(65, "foldedCorner");
        map.insert(187, "seal");
        map.insert(58, "seal");
        map.insert(59, "seal");
        map.insert(92, "seal");
        map.insert(60, "seal");
        map.insert(61, "symmetrical");
        map.insert(62, "symmetrical");
        map.insert(63, "symmetrical");
        map.insert(106, "symmetrical");
        map.insert(64, "symmetricalX");
        map.insert(188, "symmetricalX");
        map.insert(156, "symmetricalX");
        map.insert(157, "symmetricalX");
        map.insert(158, "symmetricalX");
        map.insert(159, "symmetricalX");
        map.insert(144, "angle");
        map.insert(145, "angle");
        map.insert(146, "angle");
        map.insert(147, "angle");
        map.insert(148, "angle");
        map.insert(149, "angle");
        map.insert(150, "angle");
        map.insert(151, "angle");
        map.insert(15, "chevron");
        map.insert(55, "chevron");
        map.insert(85, "bracket");
        map.insert(86, "bracket");
        map.insert(87, "brace");
        map.insert(88, "brace");
        map.insert(7, "parallelogram");
        map.insert(9, "parallelogram");
        map.insert(13, "arrow");
        map.insert(68, "upArrow");
        map.insert(67, "downArrow");
        map.insert(66, "leftArrow");
        map.insert(69, "leftArrow");
        map.insert(70, "upDownArrow");
        map.insert(76, "quadArrow");
        map.insert(182, "quadArrow");
        map.insert(93, "stripedRightArrow");
        map.insert(94, "notchedRightArrow");
        map.insert(54, "ribbon2");
        map.insert(108, "ribbon2");
        map.insert(53, "ribbon");
        map.insert(107, "ribbon");
        map.insert(77, "leftArrowCallout");
        map.insert(78, "rightArrowCallout");
        map.insert(81, "leftRightArrowCallout");
        map.insert(79, "upArrowCallout");
        map.insert(80, "downArrowCallout");
        map.insert(82, "quadArrowCallout");
        map.insert(83, "quadArrowCallout");
        map.insert(102, "curvedRightArrow");
        map.insert(103, "curvedLeftArrow");
        map.insert(104, "curvedUpArrow");
        map.insert(105, "curvedDownArrow");
        map.insert(90, "bentUpArrow");
        map.insert(89, "leftUpArrow");
        map.insert(91, "bentArrow");
        map.insert(95, "blockArc");
        map.insert(99, "circularArrow");
        map.insert(19, "arc");
        map.insert(22, "can");
        map.insert(100, "nothing");
        map.insert(0, "nothing");
        Mutex::new(map)
    })
}

// Java: public class AdjConvert
pub struct AdjConvert {
}

// Java: public class AdjConvert
impl AdjConvert {
    // Java: AdjConvert()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static GeoAdjust convertAdjVML2GSpace(GeoAdjust geoAdjust, int shapeType, float whRatio)
    pub fn convertAdjVML2GSpace(geoAdjust: Option<GeoAdjust>, shapeType: i32, whRatio: f32) -> Option<GeoAdjust> {
        // Java: if (geoAdjust == null)
        if geoAdjust.is_none() {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: geoAdjust = adjustValue(geoAdjust, shapeType);
            let geoAdjust = Self::adjustValue(geoAdjust.unwrap(), shapeType);
            // Java: return convert(geoAdjust, shapeType, whRatio, "convert2GSpace_");
            return Some(Self::convert(&geoAdjust, shapeType, whRatio, "convert2GSpace_"));
        }
    }

    // Java: static GeoAdjust convertAdjGSpace2VML(GeoAdjust geoAdjust, int shapeType, float whRatio)
    pub fn convertAdjGSpace2VML(geoAdjust: Option<GeoAdjust>, shapeType: i32, whRatio: f32) -> Option<GeoAdjust> {
        // Java: if (geoAdjust == null)
        if geoAdjust.is_none() {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: GeoAdjust reAdjust = convert(geoAdjust, shapeType, whRatio, "convert2Vml_");
            let reAdjust = Self::convert(geoAdjust.as_ref().unwrap(), shapeType, whRatio, "convert2Vml_");
            // Java: return adjustValue(reAdjust, shapeType);
            return Some(Self::adjustValue(reAdjust, shapeType));
        }
    }

    // Java: static GeoAdjust convert(GeoAdjust geoAdjust, int shapeType, float whRatio, String strTo)
    pub fn convert(geoAdjust: &GeoAdjust, shapeType: i32, whRatio: f32, strTo: &str) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = null;
        let mut reAdjust: Option<GeoAdjust> = None;
        // Java: String strFun = strTo + getFunctionName(shapeType);
        let strFun = format!("{}{}", strTo, Self::getFunctionName(shapeType));
        // Java: Class<?> c = AdjConvert.class;
        #[allow(unused_variables)]
        let c = "AdjConvert";
        // Java: Method m = null;
        let mut m: Option<fn(&GeoAdjust, f32) -> GeoAdjust> = None;
        // Java: Class<?>[] cargs = new Class[]{GeoAdjust.class, Float.TYPE};
        #[allow(unused_variables)]
        let cargs = ("GeoAdjust", "float");
        // Java: try
        let method_result: Result<fn(&GeoAdjust, f32) -> GeoAdjust, String> = Self::lookup_method(&strFun);
        // Java: m = c.getMethod(strFun, cargs);
        if let Ok(method) = method_result {
            m = Some(method);
        }
        // Java: catch (SecurityException e)
        if let Err(ref e) = method_result {
            // Java: e.printStackTrace();
            eprintln!("{}", e);
        }
        // Java: catch (NoSuchMethodException e)
        if let Err(ref e) = method_result {
            // Java: e.printStackTrace();
            eprintln!("{}", e);
        }
        // Java: if (m == null)
        if m.is_none() {
            // Java: return geoAdjust;
            return geoAdjust.clone().unwrap();
        }
        // Java: else
        else {
            // Java: Object[] args = new Object[]{geoAdjust, whRatio};
            #[allow(unused_variables)]
            let args = (geoAdjust, whRatio);
            let func = m.unwrap();
            // Java: try
            let invoke_result: Result<GeoAdjust, String> = Ok(func(geoAdjust, whRatio));
            // Java: reAdjust = (GeoAdjust)m.invoke(AdjConvert.class, args);
            if let Ok(ref result) = invoke_result {
                reAdjust = Some(result.clone().unwrap());
            }
            // Java: catch (IllegalArgumentException e)
            if let Err(ref e) = invoke_result {
                // Java: e.printStackTrace();
                eprintln!("{}", e);
            }
            // Java: catch (IllegalAccessException e)
            if let Err(ref e) = invoke_result {
                // Java: e.printStackTrace();
                eprintln!("{}", e);
            }
            // Java: catch (InvocationTargetException e)
            if let Err(ref e) = invoke_result {
                // Java: e.printStackTrace();
                eprintln!("{}", e);
            }
        }
        // Java: return reAdjust;
        reAdjust.unwrap_or_else(|| geoAdjust.clone().unwrap())
    }

    fn lookup_method(func_name: &str) -> Result<fn(&GeoAdjust, f32) -> GeoAdjust, String> {
        match func_name {
            "convert2GSpace_normal" => Ok(Self::convert2GSpace_normal),
            "convert2Vml_normal" => Ok(Self::convert2Vml_normal),
            "convert2GSpace_smileyFace" => Ok(Self::convert2GSpace_smileyFace),
            "convert2Vml_smileyFace" => Ok(Self::convert2Vml_smileyFace),
            "convert2GSpace_foldedCorner" => Ok(Self::convert2GSpace_foldedCorner),
            "convert2Vml_foldedCorner" => Ok(Self::convert2Vml_foldedCorner),
            "convert2GSpace_seal" => Ok(Self::convert2GSpace_seal),
            "convert2Vml_seal" => Ok(Self::convert2Vml_seal),
            "convert2GSpace_symmetrical" => Ok(Self::convert2GSpace_symmetrical),
            "convert2Vml_symmetrical" => Ok(Self::convert2Vml_symmetrical),
            "convert2GSpace_symmetricalX" => Ok(Self::convert2GSpace_symmetricalX),
            "convert2Vml_symmetricalX" => Ok(Self::convert2Vml_symmetricalX),
            "convert2GSpace_angle" => Ok(Self::convert2GSpace_angle),
            "convert2Vml_angle" => Ok(Self::convert2Vml_angle),
            "convert2GSpace_chevron" => Ok(Self::convert2GSpace_chevron),
            "convert2Vml_chevron" => Ok(Self::convert2Vml_chevron),
            "convert2GSpace_bracket" => Ok(Self::convert2GSpace_bracket),
            "convert2Vml_bracket" => Ok(Self::convert2Vml_bracket),
            "convert2GSpace_brace" => Ok(Self::convert2GSpace_brace),
            "convert2Vml_brace" => Ok(Self::convert2Vml_brace),
            "convert2GSpace_parallelogram" => Ok(Self::convert2GSpace_parallelogram),
            "convert2Vml_parallelogram" => Ok(Self::convert2Vml_parallelogram),
            "convert2GSpace_arrow" => Ok(Self::convert2GSpace_arrow),
            "convert2Vml_arrow" => Ok(Self::convert2Vml_arrow),
            "convert2GSpace_leftArrow" => Ok(Self::convert2GSpace_leftArrow),
            "convert2Vml_leftArrow" => Ok(Self::convert2Vml_leftArrow),
            "convert2GSpace_upArrow" => Ok(Self::convert2GSpace_upArrow),
            "convert2Vml_upArrow" => Ok(Self::convert2Vml_upArrow),
            "convert2GSpace_downArrow" => Ok(Self::convert2GSpace_downArrow),
            "convert2Vml_downArrow" => Ok(Self::convert2Vml_downArrow),
            "convert2GSpace_upDownArrow" => Ok(Self::convert2GSpace_upDownArrow),
            "convert2Vml_upDownArrow" => Ok(Self::convert2Vml_upDownArrow),
            "convert2GSpace_quadArrow" => Ok(Self::convert2GSpace_quadArrow),
            "convert2Vml_quadArrow" => Ok(Self::convert2Vml_quadArrow),
            "convert2GSpace_stripedRightArrow" => Ok(Self::convert2GSpace_stripedRightArrow),
            "convert2Vml_stripedRightArrow" => Ok(Self::convert2Vml_stripedRightArrow),
            "convert2GSpace_notchedRightArrow" => Ok(Self::convert2GSpace_notchedRightArrow),
            "convert2Vml_notchedRightArrow" => Ok(Self::convert2Vml_notchedRightArrow),
            "convert2GSpace_ribbon2" => Ok(Self::convert2GSpace_ribbon2),
            "convert2Vml_ribbon2" => Ok(Self::convert2Vml_ribbon2),
            "convert2GSpace_ribbon" => Ok(Self::convert2GSpace_ribbon),
            "convert2Vml_ribbon" => Ok(Self::convert2Vml_ribbon),
            "convert2GSpace_leftArrowCallout" => Ok(Self::convert2GSpace_leftArrowCallout),
            "convert2Vml_leftArrowCallout" => Ok(Self::convert2Vml_leftArrowCallout),
            "convert2GSpace_rightArrowCallout" => Ok(Self::convert2GSpace_rightArrowCallout),
            "convert2Vml_rightArrowCallout" => Ok(Self::convert2Vml_rightArrowCallout),
            "convert2GSpace_leftRightArrowCallout" => Ok(Self::convert2GSpace_leftRightArrowCallout),
            "convert2Vml_leftRightArrowCallout" => Ok(Self::convert2Vml_leftRightArrowCallout),
            "convert2GSpace_upArrowCallout" => Ok(Self::convert2GSpace_upArrowCallout),
            "convert2Vml_upArrowCallout" => Ok(Self::convert2Vml_upArrowCallout),
            "convert2GSpace_downArrowCallout" => Ok(Self::convert2GSpace_downArrowCallout),
            "convert2Vml_downArrowCallout" => Ok(Self::convert2Vml_downArrowCallout),
            "convert2GSpace_quadArrowCallout" => Ok(Self::convert2GSpace_quadArrowCallout),
            "convert2Vml_quadArrowCallout" => Ok(Self::convert2Vml_quadArrowCallout),
            "convert2GSpace_curvedRightArrow" => Ok(Self::convert2GSpace_curvedRightArrow),
            "convert2Vml_curvedRightArrow" => Ok(Self::convert2Vml_curvedRightArrow),
            "convert2GSpace_curvedLeftArrow" => Ok(Self::convert2GSpace_curvedLeftArrow),
            "convert2Vml_curvedLeftArrow" => Ok(Self::convert2Vml_curvedLeftArrow),
            "convert2GSpace_curvedUpArrow" => Ok(Self::convert2GSpace_curvedUpArrow),
            "convert2Vml_curvedUpArrow" => Ok(Self::convert2Vml_curvedUpArrow),
            "convert2GSpace_curvedDownArrow" => Ok(Self::convert2GSpace_curvedDownArrow),
            "convert2Vml_curvedDownArrow" => Ok(Self::convert2Vml_curvedDownArrow),
            "convert2GSpace_bentUpArrow" => Ok(Self::convert2GSpace_bentUpArrow),
            "convert2Vml_bentUpArrow" => Ok(Self::convert2Vml_bentUpArrow),
            "convert2GSpace_leftUpArrow" => Ok(Self::convert2GSpace_leftUpArrow),
            "convert2Vml_leftUpArrow" => Ok(Self::convert2Vml_leftUpArrow),
            "convert2GSpace_bentArrow" => Ok(Self::convert2GSpace_bentArrow),
            "convert2Vml_bentArrow" => Ok(Self::convert2Vml_bentArrow),
            "convert2GSpace_blockArc" => Ok(Self::convert2GSpace_blockArc),
            "convert2Vml_blockArc" => Ok(Self::convert2Vml_blockArc),
            "convert2GSpace_circularArrow" => Ok(Self::convert2GSpace_circularArrow),
            "convert2Vml_circularArrow" => Ok(Self::convert2Vml_circularArrow),
            "convert2GSpace_arc" => Ok(Self::convert2GSpace_arc),
            "convert2Vml_arc" => Ok(Self::convert2Vml_arc),
            "convert2GSpace_nothing" => Ok(Self::convert2GSpace_nothing),
            "convert2Vml_nothing" => Ok(Self::convert2Vml_nothing),
            "convert2GSpace_can" => Ok(Self::convert2GSpace_can),
            "convert2Vml_can" => Ok(Self::convert2Vml_can),
            _ => Err(format!("NoSuchMethodException: {}", func_name)),
        }
    }

    // Java: static void changeAdjust(GeoAdjust geoAdjust, int adj1, int adj2)
    fn changeAdjust(geoAdjust: &mut GeoAdjust, adj1: i32, adj2: i32) {
        // Java: boolean bHasAdj1 = geoAdjust.hasAdjust(adj1);
        let bHasAdj1 = geoAdjust.hasAdjust(adj1);
        // Java: boolean bHasAdj2 = geoAdjust.hasAdjust(adj2);
        let bHasAdj2 = geoAdjust.hasAdjust(adj2);
        // Java: if (bHasAdj1 && bHasAdj2)
        if bHasAdj1 && bHasAdj2 {
            // Java: int adjust = geoAdjust.getAdjust(adj1);
            let adjust = geoAdjust.getAdjust(adj1);
            // Java: geoAdjust.setAdjust(adj1, geoAdjust.getAdjust(adj2));
            geoAdjust.setAdjust(adj1, geoAdjust.getAdjust(adj2));
            // Java: geoAdjust.setAdjust(adj2, adjust);
            geoAdjust.setAdjust(adj2, adjust);
        // Java: else if (bHasAdj1)
        } else if bHasAdj1 {
            // Java: geoAdjust.setAdjust(adj2, geoAdjust.getAdjust(adj1));
            geoAdjust.setAdjust(adj2, geoAdjust.getAdjust(adj1));
            // Java: geoAdjust.deleteAdjust(adj1);
            geoAdjust.deleteAdjust(adj1);
        // Java: else if (bHasAdj2)
        } else if bHasAdj2 {
            // Java: geoAdjust.setAdjust(adj1, geoAdjust.getAdjust(adj2));
            geoAdjust.setAdjust(adj1, geoAdjust.getAdjust(adj2));
            // Java: geoAdjust.deleteAdjust(adj2);
            geoAdjust.deleteAdjust(adj2);
        }
    }

    // Java: static GeoAdjust adjustValue(GeoAdjust geoAdjust, int shapeType)
    fn adjustValue(mut geoAdjust: GeoAdjust, shapeType: i32) -> GeoAdjust {
        // Java: if (GeoAdjust.isAdjustValue12(shapeType))
        if GeoAdjust::isAdjustValue12(shapeType) {
            // Java: changeAdjust(geoAdjust, 0, 1);
            Self::changeAdjust(&mut geoAdjust, 0, 1);
        // Java: else if (GeoAdjust.isAdjustValue14(shapeType))
        } else if GeoAdjust::isAdjustValue14(shapeType) {
            // Java: changeAdjust(geoAdjust, 0, 3);
            Self::changeAdjust(&mut geoAdjust, 0, 3);
        // Java: else if (GeoAdjust.isTailedAdjust(shapeType))
        } else if GeoAdjust::isTailedAdjust(shapeType) {
            // Java: geoAdjust.reverseAdjust(shapeType);
            geoAdjust.reverseAdjust(shapeType);
        }
        // Java: return geoAdjust;
        geoAdjust
    }

    // Java: static String getFunctionName(int shapeType)
    fn getFunctionName(shapeType: i32) -> &'static str {
        // Java: if (funcMap == null)
        if funcMap.get().is_none() {
            // Java: funcMap = new TreeMap();
            let mut map = HashMap::new();
            // Java: funcMap.put(96, "smileyFace");
            map.insert(96, "smileyFace");
            // Java: funcMap.put(65, "foldedCorner");
            map.insert(65, "foldedCorner");
            // Java: funcMap.put(187, "seal");
            map.insert(187, "seal");
            // Java: funcMap.put(58, "seal");
            map.insert(58, "seal");
            // Java: funcMap.put(59, "seal");
            map.insert(59, "seal");
            // Java: funcMap.put(92, "seal");
            map.insert(92, "seal");
            // Java: funcMap.put(60, "seal");
            map.insert(60, "seal");
            // Java: funcMap.put(61, "symmetrical");
            map.insert(61, "symmetrical");
            // Java: funcMap.put(62, "symmetrical");
            map.insert(62, "symmetrical");
            // Java: funcMap.put(63, "symmetrical");
            map.insert(63, "symmetrical");
            // Java: funcMap.put(106, "symmetrical");
            map.insert(106, "symmetrical");
            // Java: funcMap.put(64, "symmetricalX");
            map.insert(64, "symmetricalX");
            // Java: funcMap.put(188, "symmetricalX");
            map.insert(188, "symmetricalX");
            // Java: funcMap.put(156, "symmetricalX");
            map.insert(156, "symmetricalX");
            // Java: funcMap.put(157, "symmetricalX");
            map.insert(157, "symmetricalX");
            // Java: funcMap.put(158, "symmetricalX");
            map.insert(158, "symmetricalX");
            // Java: funcMap.put(159, "symmetricalX");
            map.insert(159, "symmetricalX");
            // Java: funcMap.put(144, "angle");
            map.insert(144, "angle");
            // Java: funcMap.put(145, "angle");
            map.insert(145, "angle");
            // Java: funcMap.put(146, "angle");
            map.insert(146, "angle");
            // Java: funcMap.put(147, "angle");
            map.insert(147, "angle");
            // Java: funcMap.put(148, "angle");
            map.insert(148, "angle");
            // Java: funcMap.put(149, "angle");
            map.insert(149, "angle");
            // Java: funcMap.put(150, "angle");
            map.insert(150, "angle");
            // Java: funcMap.put(151, "angle");
            map.insert(151, "angle");
            // Java: funcMap.put(15, "chevron");
            map.insert(15, "chevron");
            // Java: funcMap.put(55, "chevron");
            map.insert(55, "chevron");
            // Java: funcMap.put(85, "bracket");
            map.insert(85, "bracket");
            // Java: funcMap.put(86, "bracket");
            map.insert(86, "bracket");
            // Java: funcMap.put(87, "brace");
            map.insert(87, "brace");
            // Java: funcMap.put(88, "brace");
            map.insert(88, "brace");
            // Java: funcMap.put(7, "parallelogram");
            map.insert(7, "parallelogram");
            // Java: funcMap.put(9, "parallelogram");
            map.insert(9, "parallelogram");
            // Java: funcMap.put(13, "arrow");
            map.insert(13, "arrow");
            // Java: funcMap.put(68, "upArrow");
            map.insert(68, "upArrow");
            // Java: funcMap.put(67, "downArrow");
            map.insert(67, "downArrow");
            // Java: funcMap.put(66, "leftArrow");
            map.insert(66, "leftArrow");
            // Java: funcMap.put(69, "leftArrow");
            map.insert(69, "leftArrow");
            // Java: funcMap.put(70, "upDownArrow");
            map.insert(70, "upDownArrow");
            // Java: funcMap.put(76, "quadArrow");
            map.insert(76, "quadArrow");
            // Java: funcMap.put(182, "quadArrow");
            map.insert(182, "quadArrow");
            // Java: funcMap.put(93, "stripedRightArrow");
            map.insert(93, "stripedRightArrow");
            // Java: funcMap.put(94, "notchedRightArrow");
            map.insert(94, "notchedRightArrow");
            // Java: funcMap.put(54, "ribbon2");
            map.insert(54, "ribbon2");
            // Java: funcMap.put(108, "ribbon2");
            map.insert(108, "ribbon2");
            // Java: funcMap.put(53, "ribbon");
            map.insert(53, "ribbon");
            // Java: funcMap.put(107, "ribbon");
            map.insert(107, "ribbon");
            // Java: funcMap.put(77, "leftArrowCallout");
            map.insert(77, "leftArrowCallout");
            // Java: funcMap.put(78, "rightArrowCallout");
            map.insert(78, "rightArrowCallout");
            // Java: funcMap.put(81, "leftRightArrowCallout");
            map.insert(81, "leftRightArrowCallout");
            // Java: funcMap.put(79, "upArrowCallout");
            map.insert(79, "upArrowCallout");
            // Java: funcMap.put(80, "downArrowCallout");
            map.insert(80, "downArrowCallout");
            // Java: funcMap.put(82, "quadArrowCallout");
            map.insert(82, "quadArrowCallout");
            // Java: funcMap.put(83, "quadArrowCallout");
            map.insert(83, "quadArrowCallout");
            // Java: funcMap.put(102, "curvedRightArrow");
            map.insert(102, "curvedRightArrow");
            // Java: funcMap.put(103, "curvedLeftArrow");
            map.insert(103, "curvedLeftArrow");
            // Java: funcMap.put(104, "curvedUpArrow");
            map.insert(104, "curvedUpArrow");
            // Java: funcMap.put(105, "curvedDownArrow");
            map.insert(105, "curvedDownArrow");
            // Java: funcMap.put(90, "bentUpArrow");
            map.insert(90, "bentUpArrow");
            // Java: funcMap.put(89, "leftUpArrow");
            map.insert(89, "leftUpArrow");
            // Java: funcMap.put(91, "bentArrow");
            map.insert(91, "bentArrow");
            // Java: funcMap.put(95, "blockArc");
            map.insert(95, "blockArc");
            // Java: funcMap.put(99, "circularArrow");
            map.insert(99, "circularArrow");
            // Java: funcMap.put(19, "arc");
            map.insert(19, "arc");
            // Java: funcMap.put(22, "can");
            map.insert(22, "can");
            // Java: funcMap.put(100, "nothing");
            map.insert(100, "nothing");
            // Java: funcMap.put(0, "nothing");
            map.insert(0, "nothing");
            let _ = funcMap.set(Mutex::new(map));
        }
        let map = funcMap.get().unwrap().lock().unwrap();
        // Java: String strName = (String)funcMap.get(shapeType);
        let strName = map.get(&shapeType).copied();
        // Java: if (strName == null)
        if strName.is_none() {
            // Java: strName = "normal";
            return "normal";
        }
        // Java: return strName;
        strName.unwrap()
    }

    // Java: static void convert2GSpace(GeoAdjust src, GeoAdjust dst, int n)
    fn convert2GSpace(src: &GeoAdjust, dst: &mut GeoAdjust, n: i32) {
        // Java: if (src.hasAdjust(n))
        if src.hasAdjust(n) {
            // Java: dst.setAdjust(n, AdjVal.adj2GSpace(src.getAdjust(n)));
            dst.setAdjust(n, AdjVal::adj2GSpace(src.getAdjust(n)));
        }
    }

    // Java: static void convert2GSpace(GeoAdjust src, GeoAdjust dst, int n, int val)
    fn convert2GSpace_with_default(src: &GeoAdjust, dst: &mut GeoAdjust, n: i32, val: i32) {
        // Java: val = src.hasAdjust(n) ? src.getAdjust(n) : val;
        let val = if src.hasAdjust(n) { src.getAdjust(n) } else { val };
        // Java: dst.setAdjust(n, AdjVal.adj2GSpace(val));
        dst.setAdjust(n, AdjVal::adj2GSpace(val));
    }

    // Java: static void convert2GSpaceSub(GeoAdjust src, GeoAdjust dst, int n, int max)
    fn convert2GSpaceSub(src: &GeoAdjust, dst: &mut GeoAdjust, n: i32, max: i32) {
        // Java: if (src.hasAdjust(n))
        if src.hasAdjust(n) {
            // Java: dst.setAdjust(n, AdjVal.adj2GSpace(max - src.getAdjust(n)));
            dst.setAdjust(n, AdjVal::adj2GSpace(max - src.getAdjust(n)));
        }
    }

    // Java: static void convert2GSpaceSub(GeoAdjust src, GeoAdjust dst, int n, int max, int val)
    fn convert2GSpaceSub_with_default(src: &GeoAdjust, dst: &mut GeoAdjust, n: i32, max: i32, val: i32) {
        // Java: val = src.hasAdjust(n) ? src.getAdjust(n) : val;
        let val = if src.hasAdjust(n) { src.getAdjust(n) } else { val };
        // Java: dst.setAdjust(n, AdjVal.adj2GSpace(max - val));
        dst.setAdjust(n, AdjVal::adj2GSpace(max - val));
    }

    // Java: static void convert2Vml(GeoAdjust src, GeoAdjust dst, int n)
    fn convert2Vml(src: &GeoAdjust, dst: &mut GeoAdjust, n: i32) {
        // Java: if (src.hasAdjust(n))
        if src.hasAdjust(n) {
            // Java: dst.setAdjust(n, AdjVal.adj2DgVml(src.getAdjust(n)));
            dst.setAdjust(n, AdjVal::adj2DgVml(src.getAdjust(n)));
        }
    }

    // Java: static void convert2Vml(GeoAdjust src, GeoAdjust dst, int n, int val)
    fn convert2Vml_with_default(src: &GeoAdjust, dst: &mut GeoAdjust, n: i32, val: i32) {
        // Java: val = src.hasAdjust(n) ? src.getAdjust(n) : val;
        let val = if src.hasAdjust(n) { src.getAdjust(n) } else { val };
        // Java: dst.setAdjust(n, AdjVal.adj2DgVml(val));
        dst.setAdjust(n, AdjVal::adj2DgVml(val));
    }

    // Java: static void convert2VmlSub(GeoAdjust src, GeoAdjust dst, int n, int max)
    fn convert2VmlSub(src: &GeoAdjust, dst: &mut GeoAdjust, n: i32, max: i32) {
        // Java: if (src.hasAdjust(n))
        if src.hasAdjust(n) {
            // Java: dst.setAdjust(n, max - AdjVal.adj2DgVml(src.getAdjust(n)));
            dst.setAdjust(n, max - AdjVal::adj2DgVml(src.getAdjust(n)));
        }
    }

    // Java: static void convert2VmlSub(GeoAdjust src, GeoAdjust dst, int n, int max, int val)
    fn convert2VmlSub_with_default(src: &GeoAdjust, dst: &mut GeoAdjust, n: i32, max: i32, val: i32) {
        // Java: val = src.hasAdjust(n) ? src.getAdjust(n) : val;
        let val = if src.hasAdjust(n) { src.getAdjust(n) } else { val };
        // Java: dst.setAdjust(n, max - AdjVal.adj2DgVml(val));
        dst.setAdjust(n, max - AdjVal::adj2DgVml(val));
    }

    // Java: static GeoAdjust convert2Vml_normal(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_normal(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: for(int i = 0; i <= 9; ++i)
        for i in 0..=9 {
            // Java: if (geoAdjust.hasAdjust(i))
            if geoAdjust.hasAdjust(i) {
                // Java: reAdjust.setAdjust(i, AdjVal.adj2DgVml(geoAdjust.getAdjust(i)));
                reAdjust.setAdjust(i, AdjVal::adj2DgVml(geoAdjust.getAdjust(i)));
            }
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_normal(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_normal(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: for(int i = 0; i <= 9; ++i)
        for i in 0..=9 {
            // Java: if (geoAdjust.hasAdjust(i))
            if geoAdjust.hasAdjust(i) {
                // Java: reAdjust.setAdjust(i, AdjVal.adj2GSpace(geoAdjust.getAdjust(i)));
                reAdjust.setAdjust(i, AdjVal::adj2GSpace(geoAdjust.getAdjust(i)));
            }
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_smileyFace(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_smileyFace(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = -4653 + (adj - 15510) * 9306 / 2010;
            let adjRe = -4653 + (adj - 15510) * 9306 / 2010;
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_smileyFace(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_smileyFace(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = 15510 + (adj - -4653) * 2010 / 9306;
            let adjRe = 15510 + (adj + 4653) * 2010 / 9306;
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_foldedCorner(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_foldedCorner(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: convert2GSpaceSub(geoAdjust, reAdjust, 0, 21600);
        Self::convert2GSpaceSub(geoAdjust, &mut reAdjust, 0, 21600);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_foldedCorner(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_foldedCorner(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: convert2VmlSub(geoAdjust, reAdjust, 0, 21600);
        Self::convert2VmlSub(geoAdjust, &mut reAdjust, 0, 21600);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_seal(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_seal(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: convert2GSpaceSub(geoAdjust, reAdjust, 0, 10800);
        Self::convert2GSpaceSub(geoAdjust, &mut reAdjust, 0, 10800);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_seal(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_seal(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: convert2VmlSub(geoAdjust, reAdjust, 0, 10800);
        Self::convert2VmlSub(geoAdjust, &mut reAdjust, 0, 10800);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_symmetrical(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_symmetrical(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: for(int i = 0; i < 2; ++i)
        for i in 0..2 {
            // Java: if (geoAdjust.hasAdjust(i))
            if geoAdjust.hasAdjust(i) {
                // Java: int adj = geoAdjust.getAdjust(i);
                let adj = geoAdjust.getAdjust(i);
                // Java: int adjRe = AdjVal.adj2GSpace(adj - 10800);
                let adjRe = AdjVal::adj2GSpace(adj - 10800);
                // Java: reAdjust.setAdjust(i, adjRe);
                reAdjust.setAdjust(i, adjRe);
            }
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_symmetrical(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_symmetrical(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: for(int i = 0; i < 2; ++i)
        for i in 0..2 {
            // Java: if (geoAdjust.hasAdjust(i))
            if geoAdjust.hasAdjust(i) {
                // Java: int adj = geoAdjust.getAdjust(i);
                let adj = geoAdjust.getAdjust(i);
                // Java: int adjRe = 10800 + AdjVal.adj2DgVml(adj);
                let adjRe = 10800 + AdjVal::adj2DgVml(adj);
                // Java: reAdjust.setAdjust(i, adjRe);
                reAdjust.setAdjust(i, adjRe);
            }
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_symmetricalX(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_symmetricalX(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: convert2GSpace(geoAdjust, reAdjust, 0);
        Self::convert2GSpace(geoAdjust, &mut reAdjust, 0);
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = AdjVal.adj2GSpace(adj - 10800);
            let adjRe = AdjVal::adj2GSpace(adj - 10800);
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_symmetricalX(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_symmetricalX(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: convert2Vml(geoAdjust, reAdjust, 0);
        Self::convert2Vml(geoAdjust, &mut reAdjust, 0);
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = AdjVal.adj2DgVml(adj) + 10800;
            let adjRe = AdjVal::adj2DgVml(adj) + 10800;
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_angle(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_angle(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = AdjVal.angle2GSpace(adj);
            let adjRe = AdjVal::angle2GSpace(adj);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_angle(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_angle(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = AdjVal.angle2DgVml(adj);
            let adjRe = AdjVal::angle2DgVml(adj);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_chevron(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_chevron(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 16200;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 16200 };
        // Java: float heightRt = Math.min(1.0F / whRatio, 1.0F);
        let heightRt = (1.0f32 / whRatio).min(1.0);
        // Java: int adjRe = AdjVal.dgVml2Ratio(21600 - adj, heightRt, 21600);
        let adjRe = AdjVal::dgVml2Ratio(21600 - adj, heightRt, 21600);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_chevron(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_chevron(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : '썐';
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 50000 };
        // Java: float heightRt = Math.min(1.0F / whRatio, 1.0F);
        let heightRt = (1.0f32 / whRatio).min(1.0);
        // Java: int adjRe = 21600 - AdjVal.ratio2DgVml(adj, heightRt, 21600);
        let adjRe = 21600 - AdjVal::ratio2DgVml(adj, heightRt, 21600);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_bracket(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_bracket(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: float widthRt = Math.min(whRatio, 1.0F);
            let widthRt = whRatio.min(1.0);
            // Java: int adjRe = AdjVal.dgVml2Ratio(adj, widthRt, 10800);
            let adjRe = AdjVal::dgVml2Ratio(adj, widthRt, 10800);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_bracket(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_bracket(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: float widthRt = Math.min(whRatio, 1.0F);
            let widthRt = whRatio.min(1.0);
            // Java: int adjRe = AdjVal.ratio2DgVml(adj, widthRt, 10800);
            let adjRe = AdjVal::ratio2DgVml(adj, widthRt, 10800);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_brace(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_brace(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: float widthRt = Math.min(whRatio, 1.0F);
            let widthRt = whRatio.min(1.0);
            // Java: int adjRe = AdjVal.dgVml2Ratio(adj, widthRt, 21600);
            let adjRe = AdjVal::dgVml2Ratio(adj, widthRt, 21600);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = AdjVal.adj2GSpace(adj);
            let mut adjRe = AdjVal::adj2GSpace(adj);
            // Java: adjRe = Math.max(adjRe, 1);
            adjRe = adjRe.max(1);
            // Java: adjRe = Math.min(adjRe, 99999);
            adjRe = adjRe.min(99999);
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_brace(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_brace(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: float widthRt = Math.min(whRatio, 1.0F);
            let widthRt = whRatio.min(1.0);
            // Java: int adjRe = AdjVal.ratio2DgVml(adj, widthRt, 21600);
            let adjRe = AdjVal::ratio2DgVml(adj, widthRt, 21600);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: convert2Vml(geoAdjust, reAdjust, 1);
        Self::convert2Vml(geoAdjust, &mut reAdjust, 1);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_parallelogram(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_parallelogram(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
            let heighRt = (1.0f32 / whRatio).min(1.0);
            // Java: int adjRe = AdjVal.dgVml2Ratio(adj, heighRt, 21600);
            let adjRe = AdjVal::dgVml2Ratio(adj, heighRt, 21600);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_parallelogram(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_parallelogram(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
            let heighRt = (1.0f32 / whRatio).min(1.0);
            // Java: int adjRe = AdjVal.ratio2DgVml(adj, heighRt, 21600);
            let adjRe = AdjVal::ratio2DgVml(adj, heighRt, 21600);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_arrow(GeoAdjust geoAdjust, float whRatio, int adj1, int adj2, boolean bReverseRt, boolean bReverseMax)
    fn convert2GSpace_arrow_full(geoAdjust: &GeoAdjust, whRatio: f32, adj1: i32, adj2: i32, bReverseRt: bool, bReverseMax: bool) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : adj1;
        let mut adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { adj1 };
        // Java: int adjRe = 100000 - 2 * AdjVal.adj2GSpace(adj);
        let mut adjRe = 100000 - 2 * AdjVal::adj2GSpace(adj);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : adj2;
        adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { adj2 };
        // Java: float rt = bReverseRt ? Math.min(1.0F / whRatio, 1.0F) : Math.min(whRatio, 1.0F);
        let rt = if bReverseRt { (1.0f32 / whRatio).min(1.0) } else { whRatio.min(1.0) };
        // Java: if (bReverseMax)
        if bReverseMax {
            // Java: adj = 21600 - adj;
            adj = 21600 - adj;
        }
        // Java: adjRe = AdjVal.dgVml2Ratio(adj, rt, 21600);
        adjRe = AdjVal::dgVml2Ratio(adj, rt, 21600);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_arrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_arrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2GSpace_arrow(geoAdjust, whRatio, 5400, 16200, true, true);
        Self::convert2GSpace_arrow_full(geoAdjust, whRatio, 5400, 16200, true, true)
    }

    // Java: static GeoAdjust convert2GSpace_leftArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_leftArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2GSpace_arrow(geoAdjust, whRatio, 5400, 5400, true, false);
        Self::convert2GSpace_arrow_full(geoAdjust, whRatio, 5400, 5400, true, false)
    }

    // Java: static GeoAdjust convert2GSpace_upArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_upArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2GSpace_arrow(geoAdjust, whRatio, 5400, 5400, false, false);
        Self::convert2GSpace_arrow_full(geoAdjust, whRatio, 5400, 5400, false, false)
    }

    // Java: static GeoAdjust convert2GSpace_downArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_downArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2GSpace_arrow(geoAdjust, whRatio, 5400, 16200, false, true);
        Self::convert2GSpace_arrow_full(geoAdjust, whRatio, 5400, 16200, false, true)
    }

    // Java: static GeoAdjust convert2GSpace_upDownArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_upDownArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2GSpace_arrow(geoAdjust, whRatio, 5400, 4100, false, false);
        Self::convert2GSpace_arrow_full(geoAdjust, whRatio, 5400, 4100, false, false)
    }

    // Java: static GeoAdjust convert2Vml_arrow(GeoAdjust geoAdjust, float whRatio, boolean bReverseRt, boolean bReverseMax)
    fn convert2Vml_arrow_full(geoAdjust: &GeoAdjust, whRatio: f32, bReverseRt: bool, bReverseMax: bool) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : '썐';
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 50000 };
        // Java: int adjRe = AdjVal.adj2DgVml((100000 - adj) / 2);
        let mut adjRe = AdjVal::adj2DgVml((100000 - adj) / 2);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : '썐';
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 50000 };
        // Java: float rt = bReverseRt ? Math.min(1.0F / whRatio, 1.0F) : Math.min(whRatio, 1.0F);
        let rt = if bReverseRt { (1.0f32 / whRatio).min(1.0) } else { whRatio.min(1.0) };
        // Java: adjRe = AdjVal.ratio2DgVml(adj, rt, 21600);
        adjRe = AdjVal::ratio2DgVml(adj, rt, 21600);
        // Java: if (bReverseMax)
        if bReverseMax {
            // Java: adjRe = 21600 - adjRe;
            adjRe = 21600 - adjRe;
        }
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_arrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_arrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2Vml_arrow(geoAdjust, whRatio, true, true);
        Self::convert2Vml_arrow_full(geoAdjust, whRatio, true, true)
    }

    // Java: static GeoAdjust convert2Vml_leftArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_leftArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2Vml_arrow(geoAdjust, whRatio, true, false);
        Self::convert2Vml_arrow_full(geoAdjust, whRatio, true, false)
    }

    // Java: static GeoAdjust convert2Vml_upArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_upArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2Vml_arrow(geoAdjust, whRatio, false, false);
        Self::convert2Vml_arrow_full(geoAdjust, whRatio, false, false)
    }

    // Java: static GeoAdjust convert2Vml_downArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_downArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2Vml_arrow(geoAdjust, whRatio, false, true);
        Self::convert2Vml_arrow_full(geoAdjust, whRatio, false, true)
    }

    // Java: static GeoAdjust convert2Vml_upDownArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_upDownArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2Vml_arrow(geoAdjust, whRatio, false, false);
        Self::convert2Vml_arrow_full(geoAdjust, whRatio, false, false)
    }

    // Java: static GeoAdjust convert2GSpace_quadArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_quadArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = AdjVal.dgVml2Ratio(10800 - adj, heighRt, 10800);
            let adjRe = AdjVal::dgVml2Ratio(10800 - adj, heighRt, 10800);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = AdjVal.dgVml2Ratio(10800 - adj, heighRt, 10800) / 2;
            let adjRe = AdjVal::dgVml2Ratio(10800 - adj, heighRt, 10800) / 2;
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(2))
        if geoAdjust.hasAdjust(2) {
            // Java: int adj = geoAdjust.getAdjust(2);
            let adj = geoAdjust.getAdjust(2);
            // Java: int adjRe = AdjVal.dgVml2Ratio(adj, widthRt, 21600);
            let adjRe = AdjVal::dgVml2Ratio(adj, widthRt, 21600);
            // Java: reAdjust.setAdjust(2, adjRe);
            reAdjust.setAdjust(2, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_quadArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_quadArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = 10800 - AdjVal.ratio2DgVml(adj, heighRt, 10800);
            let adjRe = 10800 - AdjVal::ratio2DgVml(adj, heighRt, 10800);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = 10800 - AdjVal.ratio2DgVml(adj * 2, heighRt, 10800);
            let adjRe = 10800 - AdjVal::ratio2DgVml(adj * 2, heighRt, 10800);
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(2))
        if geoAdjust.hasAdjust(2) {
            // Java: int adj = geoAdjust.getAdjust(2);
            let adj = geoAdjust.getAdjust(2);
            // Java: int adjRe = AdjVal.ratio2DgVml(adj, widthRt, 21600);
            let adjRe = AdjVal::ratio2DgVml(adj, widthRt, 21600);
            // Java: reAdjust.setAdjust(2, adjRe);
            reAdjust.setAdjust(2, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_stripedRightArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_stripedRightArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 5400;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 5400 };
        // Java: int adjRe = 100000 - 2 * AdjVal.adj2GSpace(adj);
        let adjRe = 100000 - 2 * AdjVal::adj2GSpace(adj);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 16200;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 16200 };
        // Java: adjRe = AdjVal.dgVml2Ratio(21600 - adj, heighRt, 18225);
        let adjRe = AdjVal::dgVml2Ratio(21600 - adj, heighRt, 18225);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_stripedRightArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_stripedRightArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : '썐';
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 50000 };
        // Java: int adjRe = AdjVal.adj2DgVml((100000 - adj) / 2);
        let adjRe = AdjVal::adj2DgVml((100000 - adj) / 2);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(1) : '썐';
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 50000 };
        // Java: adjRe = 21600 - AdjVal.ratio2DgVml(adj, heighRt, 18225);
        let adjRe = 21600 - AdjVal::ratio2DgVml(adj, heighRt, 18225);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_notchedRightArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_notchedRightArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 5400;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 5400 };
        // Java: int adjRe = 100000 - 2 * AdjVal.adj2GSpace(adj);
        let adjRe = 100000 - 2 * AdjVal::adj2GSpace(adj);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 16200;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 16200 };
        // Java: adjRe = AdjVal.dgVml2Ratio(21600 - adj, heighRt, 21600);
        let adjRe = AdjVal::dgVml2Ratio(21600 - adj, heighRt, 21600);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_notchedRightArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_notchedRightArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : '썐';
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 50000 };
        // Java: int adjRe = AdjVal.adj2DgVml((100000 - adj) / 2);
        let adjRe = AdjVal::adj2DgVml((100000 - adj) / 2);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : '썐';
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 50000 };
        // Java: adjRe = 21600 - AdjVal.ratio2DgVml(adj, heighRt, 21600);
        let adjRe = 21600 - AdjVal::ratio2DgVml(adj, heighRt, 21600);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_ribbon2(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_ribbon2(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = 100000 - AdjVal.adj2GSpace(adj);
            let adjRe = 100000 - AdjVal::adj2GSpace(adj);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = 100000 - 2 * AdjVal.adj2GSpace(adj);
            let adjRe = 100000 - 2 * AdjVal::adj2GSpace(adj);
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: convert2GSpace(geoAdjust, reAdjust, 2);
        Self::convert2GSpace(geoAdjust, &mut reAdjust, 2);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_ribbon2(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_ribbon2(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = AdjVal.adj2DgVml(100000 - adj);
            let adjRe = AdjVal::adj2DgVml(100000 - adj);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = AdjVal.adj2DgVml((100000 - adj) / 2);
            let adjRe = AdjVal::adj2DgVml((100000 - adj) / 2);
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: convert2Vml(geoAdjust, reAdjust, 2);
        Self::convert2Vml(geoAdjust, &mut reAdjust, 2);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_ribbon(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_ribbon(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: convert2GSpace(geoAdjust, reAdjust, 0);
        Self::convert2GSpace(geoAdjust, &mut reAdjust, 0);
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = 100000 - 2 * AdjVal.adj2GSpace(adj);
            let adjRe = 100000 - 2 * AdjVal::adj2GSpace(adj);
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(2))
        if geoAdjust.hasAdjust(2) {
            // Java: int adj = geoAdjust.getAdjust(2);
            let adj = geoAdjust.getAdjust(2);
            // Java: int adjRe = 100000 - AdjVal.adj2GSpace(adj);
            let adjRe = 100000 - AdjVal::adj2GSpace(adj);
            // Java: reAdjust.setAdjust(2, adjRe);
            reAdjust.setAdjust(2, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_ribbon(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_ribbon(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: convert2Vml(geoAdjust, reAdjust, 0);
        Self::convert2Vml(geoAdjust, &mut reAdjust, 0);
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = AdjVal.adj2DgVml((100000 - adj) / 2);
            let adjRe = AdjVal::adj2DgVml((100000 - adj) / 2);
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(2))
        if geoAdjust.hasAdjust(2) {
            // Java: int adj = geoAdjust.getAdjust(2);
            let adj = geoAdjust.getAdjust(2);
            // Java: int adjRe = AdjVal.adj2DgVml(100000 - adj);
            let adjRe = AdjVal::adj2DgVml(100000 - adj);
            // Java: reAdjust.setAdjust(2, adjRe);
            reAdjust.setAdjust(2, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_leftArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_leftArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 8100;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 8100 };
        // Java: int adjRe = AdjVal.dgVml2Ratio(10800 - adj, widthRt, 10800);
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, widthRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 5400;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 5400 };
        // Java: adjRe = AdjVal.dgVml2Ratio(10800 - adj, widthRt, 10800) / 2;
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, widthRt, 10800) / 2;
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 3600;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 3600 };
        // Java: adjRe = AdjVal.dgVml2Ratio(adj, heighRt, 21600);
        let adjRe = AdjVal::dgVml2Ratio(adj, heighRt, 21600);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: convert2GSpaceSub(geoAdjust, reAdjust, 3, 21600, 7200);
        Self::convert2GSpaceSub_with_default(geoAdjust, &mut reAdjust, 3, 21600, 7200);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_leftArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_leftArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 25000;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 25000 };
        // Java: int adjRe = 10800 - AdjVal.ratio2DgVml(adj, widthRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj, widthRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 25000;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 25000 };
        // Java: adjRe = 10800 - AdjVal.ratio2DgVml(adj * 2, widthRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj * 2, widthRt, 10800);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 25000;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 25000 };
        // Java: adjRe = AdjVal.ratio2DgVml(adj, heighRt, 21600);
        let adjRe = AdjVal::ratio2DgVml(adj, heighRt, 21600);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: convert2VmlSub(geoAdjust, reAdjust, 3, 21600, 64977);
        Self::convert2VmlSub_with_default(geoAdjust, &mut reAdjust, 3, 21600, 64977);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_rightArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_rightArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 8100;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 8100 };
        // Java: int adjRe = AdjVal.dgVml2Ratio(10800 - adj, widthRt, 10800);
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, widthRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 5400;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 5400 };
        // Java: adjRe = AdjVal.dgVml2Ratio(10800 - adj, widthRt, 10800) / 2;
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, widthRt, 10800) / 2;
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 18000;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 18000 };
        // Java: adjRe = AdjVal.dgVml2Ratio(21600 - adj, heighRt, 21600);
        let adjRe = AdjVal::dgVml2Ratio(21600 - adj, heighRt, 21600);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: convert2GSpace(geoAdjust, reAdjust, 3, 14400);
        Self::convert2GSpace_with_default(geoAdjust, &mut reAdjust, 3, 14400);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_rightArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_rightArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 25000;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 25000 };
        // Java: int adjRe = 10800 - AdjVal.ratio2DgVml(adj, widthRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj, widthRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 25000;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 25000 };
        // Java: adjRe = 10800 - AdjVal.ratio2DgVml(adj * 2, widthRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj * 2, widthRt, 10800);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 25000;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 25000 };
        // Java: adjRe = 21600 - AdjVal.ratio2DgVml(adj, heighRt, 21600);
        let adjRe = 21600 - AdjVal::ratio2DgVml(adj, heighRt, 21600);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: convert2Vml(geoAdjust, reAdjust, 3, 64977);
        Self::convert2Vml_with_default(geoAdjust, &mut reAdjust, 3, 64977);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_leftRightArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_leftRightArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 8100;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 8100 };
        // Java: int adjRe = AdjVal.dgVml2Ratio(10800 - adj, widthRt, 10800);
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, widthRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 5400;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 5400 };
        // Java: adjRe = AdjVal.dgVml2Ratio(10800 - adj, widthRt, 10800) / 2;
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, widthRt, 10800) / 2;
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 2700;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 2700 };
        // Java: adjRe = AdjVal.dgVml2Ratio(adj, heighRt, 10800) / 2;
        let adjRe = AdjVal::dgVml2Ratio(adj, heighRt, 10800) / 2;
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: if (geoAdjust.hasAdjust(3))
        if geoAdjust.hasAdjust(3) {
            // Java: adj = geoAdjust.getAdjust(3);
            let adj = geoAdjust.getAdjust(3);
            // Java: adjRe = AdjVal.adj2GSpace(10800 - adj) * 2;
            let adjRe = AdjVal::adj2GSpace(10800 - adj) * 2;
            // Java: reAdjust.setAdjust(3, adjRe);
            reAdjust.setAdjust(3, adjRe);
        // Java: else
        } else {
            // Java: reAdjust.setAdjust(3, 50000);
            reAdjust.setAdjust(3, 50000);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_leftRightArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_leftRightArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 25000;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 25000 };
        // Java: int adjRe = 10800 - AdjVal.ratio2DgVml(adj, widthRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj, widthRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 25000;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 25000 };
        // Java: adjRe = 10800 - AdjVal.ratio2DgVml(adj * 2, widthRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj * 2, widthRt, 10800);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 25000;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 25000 };
        // Java: adjRe = AdjVal.ratio2DgVml(adj * 2, heighRt, 10800);
        let adjRe = AdjVal::ratio2DgVml(adj * 2, heighRt, 10800);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: adj = geoAdjust.hasAdjust(3) ? geoAdjust.getAdjust(3) : '믻';
        let adj = if geoAdjust.hasAdjust(3) { geoAdjust.getAdjust(3) } else { 50000 };
        // Java: adjRe = 10800 - AdjVal.adj2DgVml(adj / 2);
        let adjRe = 10800 - AdjVal::adj2DgVml(adj / 2);
        // Java: reAdjust.setAdjust(3, adjRe);
        reAdjust.setAdjust(3, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_upArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_upArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 8100;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 8100 };
        // Java: int adjRe = AdjVal.dgVml2Ratio(10800 - adj, heighRt, 10800);
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, heighRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 5400;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 5400 };
        // Java: adjRe = AdjVal.dgVml2Ratio(10800 - adj, heighRt, 10800) / 2;
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, heighRt, 10800) / 2;
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 3600;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 3600 };
        // Java: adjRe = AdjVal.dgVml2Ratio(adj, widthRt, 21600);
        let adjRe = AdjVal::dgVml2Ratio(adj, widthRt, 21600);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: convert2GSpaceSub(geoAdjust, reAdjust, 3, 21600, 7200);
        Self::convert2GSpaceSub_with_default(geoAdjust, &mut reAdjust, 3, 21600, 7200);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_upArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_upArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 25000;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 25000 };
        // Java: int adjRe = 10800 - AdjVal.ratio2DgVml(adj, heighRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj, heighRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 25000;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 25000 };
        // Java: adjRe = 10800 - AdjVal.ratio2DgVml(adj * 2, heighRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj * 2, heighRt, 10800);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 25000;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 25000 };
        // Java: adjRe = AdjVal.ratio2DgVml(adj, widthRt, 21600);
        let adjRe = AdjVal::ratio2DgVml(adj, widthRt, 21600);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: adj = geoAdjust.hasAdjust(3) ? geoAdjust.getAdjust(3) : '\ufdd1';
        let adj = if geoAdjust.hasAdjust(3) { geoAdjust.getAdjust(3) } else { 64977 };
        // Java: adjRe = 21600 - AdjVal.adj2DgVml(adj);
        let adjRe = 21600 - AdjVal::adj2DgVml(adj);
        // Java: reAdjust.setAdjust(3, adjRe);
        reAdjust.setAdjust(3, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_downArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_downArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 8100;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 8100 };
        // Java: int adjRe = AdjVal.dgVml2Ratio(10800 - adj, heighRt, 10800);
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, heighRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 5400;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 5400 };
        // Java: adjRe = AdjVal.dgVml2Ratio(10800 - adj, heighRt, 10800) / 2;
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, heighRt, 10800) / 2;
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 18000;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 18000 };
        // Java: adjRe = AdjVal.dgVml2Ratio(21600 - adj, widthRt, 21600);
        let adjRe = AdjVal::dgVml2Ratio(21600 - adj, widthRt, 21600);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: convert2GSpace(geoAdjust, reAdjust, 3, 14400);
        Self::convert2GSpace_with_default(geoAdjust, &mut reAdjust, 3, 14400);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_downArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_downArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 25000;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 25000 };
        // Java: int adjRe = 10800 - AdjVal.ratio2DgVml(adj, heighRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj, heighRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 25000;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 25000 };
        // Java: adjRe = 10800 - AdjVal.ratio2DgVml(adj * 2, heighRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj * 2, heighRt, 10800);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 25000;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 25000 };
        // Java: adjRe = 21600 - AdjVal.ratio2DgVml(adj, widthRt, 21600);
        let adjRe = 21600 - AdjVal::ratio2DgVml(adj, widthRt, 21600);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: convert2Vml(geoAdjust, reAdjust, 3, 64977);
        Self::convert2Vml_with_default(geoAdjust, &mut reAdjust, 3, 64977);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_quadArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_quadArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 9450;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 9450 };
        // Java: int adjRe = AdjVal.dgVml2Ratio(10800 - adj, heighRt, 10800);
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, heighRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 8100;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 8100 };
        // Java: adjRe = AdjVal.dgVml2Ratio(10800 - adj, heighRt, 10800) / 2;
        let adjRe = AdjVal::dgVml2Ratio(10800 - adj, heighRt, 10800) / 2;
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 2700;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 2700 };
        // Java: adjRe = AdjVal.dgVml2Ratio(adj, widthRt, 10800) / 2;
        let adjRe = AdjVal::dgVml2Ratio(adj, widthRt, 10800) / 2;
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: adj = geoAdjust.hasAdjust(3) ? geoAdjust.getAdjust(3) : 5400;
        let adj = if geoAdjust.hasAdjust(3) { geoAdjust.getAdjust(3) } else { 5400 };
        // Java: adjRe = AdjVal.adj2GSpace(10800 - adj) * 2;
        let adjRe = AdjVal::adj2GSpace(10800 - adj) * 2;
        // Java: reAdjust.setAdjust(3, adjRe);
        reAdjust.setAdjust(3, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_quadArrowCallout(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_quadArrowCallout(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 18515;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 18515 };
        // Java: int adjRe = 10800 - AdjVal.ratio2DgVml(adj, heighRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj, heighRt, 10800);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 18515;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 18515 };
        // Java: adjRe = 10800 - AdjVal.ratio2DgVml(adj * 2, heighRt, 10800);
        let adjRe = 10800 - AdjVal::ratio2DgVml(adj * 2, heighRt, 10800);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 18515;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 18515 };
        // Java: adjRe = AdjVal.ratio2DgVml(adj * 2, widthRt, 10800);
        let adjRe = AdjVal::ratio2DgVml(adj * 2, widthRt, 10800);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: adj = geoAdjust.hasAdjust(3) ? geoAdjust.getAdjust(3) : '믻';
        let adj = if geoAdjust.hasAdjust(3) { geoAdjust.getAdjust(3) } else { 50000 };
        // Java: adjRe = 10800 - AdjVal.adj2DgVml(adj / 2);
        let adjRe = 10800 - AdjVal::adj2DgVml(adj / 2);
        // Java: reAdjust.setAdjust(3, adjRe);
        reAdjust.setAdjust(3, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_curvedArrow(GeoAdjust geoAdjust, float whRatio, int adj1, int adj2, int adj3, boolean bReverseRt, boolean bReverseMax)
    fn convert2GSpace_curvedArrow_full(geoAdjust: &GeoAdjust, whRatio: f32, adj1: i32, adj2: i32, adj3: i32, bReverseRt: bool, bReverseMax: bool) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let mut heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let mut widthRt = whRatio.min(1.0);
        // Java: if (bReverseRt)
        if bReverseRt {
            // Java: float rt = heighRt;
            let rt = heighRt;
            // Java: heighRt = widthRt;
            heighRt = widthRt;
            // Java: widthRt = rt;
            widthRt = rt;
        }
        // Java: int adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : adj2;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { adj2 };
        // Java: int adjRe = AdjVal.dgVml2Ratio(21600 - adj, widthRt, 21600);
        let adjRe = AdjVal::dgVml2Ratio(21600 - adj, widthRt, 21600);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: int center = adjRe / 2;
        let center = adjRe / 2;
        // Java: adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : adj1;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { adj1 };
        // Java: adjRe = AdjVal.dgVml2Ratio(21600 - adj, widthRt, 21600);
        let adjRe = AdjVal::dgVml2Ratio(21600 - adj, widthRt, 21600);
        // Java: assert adjRe <= center;
        debug_assert!(adjRe <= center);
        // Java: adjRe = (center - adjRe) * 2;
        let adjRe = (center - adjRe) * 2;
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : adj3;
        let mut adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { adj3 };
        // Java: if (bReverseMax)
        if bReverseMax {
            // Java: adj = 21600 - adj;
            adj = 21600 - adj;
        }
        // Java: adjRe = AdjVal.dgVml2Ratio(adj, heighRt, 21600);
        let adjRe = AdjVal::dgVml2Ratio(adj, heighRt, 21600);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_curvedRightArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_curvedRightArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2GSpace_curvedArrow(geoAdjust, whRatio, 19000, 12340, 14380, false, true);
        Self::convert2GSpace_curvedArrow_full(geoAdjust, whRatio, 19000, 12340, 14380, false, true)
    }

    // Java: static GeoAdjust convert2GSpace_curvedLeftArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_curvedLeftArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2GSpace_curvedArrow(geoAdjust, whRatio, 19000, 12340, 7220, false, false);
        Self::convert2GSpace_curvedArrow_full(geoAdjust, whRatio, 19000, 12340, 7220, false, false)
    }

    // Java: static GeoAdjust convert2GSpace_curvedUpArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_curvedUpArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2GSpace_curvedArrow(geoAdjust, whRatio, 19000, 12340, 7220, true, false);
        Self::convert2GSpace_curvedArrow_full(geoAdjust, whRatio, 19000, 12340, 7220, true, false)
    }

    // Java: static GeoAdjust convert2GSpace_curvedDownArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_curvedDownArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2GSpace_curvedArrow(geoAdjust, whRatio, 19000, 12340, 14380, true, true);
        Self::convert2GSpace_curvedArrow_full(geoAdjust, whRatio, 19000, 12340, 14380, true, true)
    }

    // Java: static GeoAdjust convert2Vml_curvedArrow(GeoAdjust geoAdjust, float whRatio, boolean bReverseRt, boolean bReverseMax)
    fn convert2Vml_curvedArrow_full(geoAdjust: &GeoAdjust, whRatio: f32, bReverseRt: bool, bReverseMax: bool) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let mut heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let mut widthRt = whRatio.min(1.0);
        // Java: if (bReverseRt)
        if bReverseRt {
            // Java: float rt = heighRt;
            let rt = heighRt;
            // Java: heighRt = widthRt;
            heighRt = widthRt;
            // Java: widthRt = rt;
            widthRt = rt;
        }
        // Java: int adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : '썐';
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 50000 };
        // Java: int adjRe = 21600 - AdjVal.ratio2DgVml(adj, widthRt, 21600);
        let adjRe = 21600 - AdjVal::ratio2DgVml(adj, widthRt, 21600);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: int center = adj / 2;
        let center = adj / 2;
        // Java: adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 25000;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 25000 };
        // Java: adj = center - adj / 2;
        let adj_val = center - adj / 2;
        // Java: adjRe = 21600 - AdjVal.ratio2DgVml(adj, widthRt, 21600);
        let adjRe = 21600 - AdjVal::ratio2DgVml(adj_val, widthRt, 21600);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 25000;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 25000 };
        // Java: adjRe = AdjVal.ratio2DgVml(adj, heighRt, 21600);
        let mut adjRe = AdjVal::ratio2DgVml(adj, heighRt, 21600);
        // Java: if (bReverseMax)
        if bReverseMax {
            // Java: adjRe = 21600 - adjRe;
            adjRe = 21600 - adjRe;
        }
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_curvedRightArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_curvedRightArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2Vml_curvedArrow(geoAdjust, whRatio, false, true);
        Self::convert2Vml_curvedArrow_full(geoAdjust, whRatio, false, true)
    }

    // Java: static GeoAdjust convert2Vml_curvedLeftArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_curvedLeftArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2Vml_curvedArrow(geoAdjust, whRatio, false, false);
        Self::convert2Vml_curvedArrow_full(geoAdjust, whRatio, false, false)
    }

    // Java: static GeoAdjust convert2Vml_curvedUpArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_curvedUpArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2Vml_curvedArrow(geoAdjust, whRatio, true, false);
        Self::convert2Vml_curvedArrow_full(geoAdjust, whRatio, true, false)
    }

    // Java: static GeoAdjust convert2Vml_curvedDownArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_curvedDownArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return convert2Vml_curvedArrow(geoAdjust, whRatio, true, true);
        Self::convert2Vml_curvedArrow_full(geoAdjust, whRatio, true, true)
    }

    // Java: static GeoAdjust convert2GSpace_bentUpArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_bentUpArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 9200;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 9200 };
        // Java: int adjRe = AdjVal.dgVml2Ratio(21600 - adj, heighRt, 21600) / 2;
        let adjRe = AdjVal::dgVml2Ratio(21600 - adj, heighRt, 21600) / 2;
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: int xRangeStart = 10800 + adj / 2;
        let xRangeStart = 10800 + adj / 2;
        // Java: adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 18500;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 18500 };
        // Java: adjRe = AdjVal.dgVml2Ratio(adj - xRangeStart, widthRt, 4646) / 2;
        let mut adjRe = AdjVal::dgVml2Ratio(adj - xRangeStart, widthRt, 4646) / 2;
        // Java: adjRe = Math.min(adjRe, 50000);
        adjRe = adjRe.min(50000);
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 6170;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 6170 };
        // Java: adjRe = AdjVal.dgVml2Ratio(adj, widthRt, 10800) / 2;
        let adjRe = AdjVal::dgVml2Ratio(adj, widthRt, 10800) / 2;
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_bentUpArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_bentUpArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 25000;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 25000 };
        // Java: int adjRe = 21600 - AdjVal.ratio2DgVml(adj * 2, heighRt, 21600);
        let adjRe = 21600 - AdjVal::ratio2DgVml(adj * 2, heighRt, 21600);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: int xRangeStart = 10800 + adjRe / 2;
        let xRangeStart = 10800 + adjRe / 2;
        // Java: adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 25000;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 25000 };
        // Java: adjRe = AdjVal.ratio2DgVml(adj * 2, widthRt, 4646) + xRangeStart;
        let adjRe = AdjVal::ratio2DgVml(adj * 2, widthRt, 4646) + xRangeStart;
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 25000;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 25000 };
        // Java: adjRe = AdjVal.ratio2DgVml(2 * adj, widthRt, 10800);
        let adjRe = AdjVal::ratio2DgVml(2 * adj, widthRt, 10800);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_leftUpArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_leftUpArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 9200;
        let adj = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 9200 };
        // Java: int adjRe = AdjVal.dgVml2Ratio(21600 - adj, heighRt, 21600) / 2;
        let adjRe = AdjVal::dgVml2Ratio(21600 - adj, heighRt, 21600) / 2;
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: int xRangeStart = 10800 + adj / 2;
        let xRangeStart = 10800 + adj / 2;
        // Java: adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 18500;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 18500 };
        // Java: adjRe = AdjVal.dgVml2Ratio(adj - xRangeStart, widthRt, 21600 - xRangeStart);
        let adjRe = AdjVal::dgVml2Ratio(adj - xRangeStart, widthRt, 21600 - xRangeStart);
        // Java: adjRe = (int)((double)1.0F * (double)adjRe / (double)50000.0F * (double)reAdjust.getAdjust(1));
        let adjRe = (1.0f64 * adjRe as f64 / 50000.0f64 * reAdjust.getAdjust(1) as f64) as i32;
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 6170;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 6170 };
        // Java: adjRe = AdjVal.dgVml2Ratio(adj, widthRt, 10800) / 2;
        let adjRe = AdjVal::dgVml2Ratio(adj, widthRt, 10800) / 2;
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_leftUpArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_leftUpArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: int adj2 = geoAdjust.hasAdjust(1) ? geoAdjust.getAdjust(1) : 25000;
        let adj2 = if geoAdjust.hasAdjust(1) { geoAdjust.getAdjust(1) } else { 25000 };
        // Java: int adjRe = 21600 - AdjVal.ratio2DgVml(adj2 * 2, heighRt, 21600);
        let adjRe = 21600 - AdjVal::ratio2DgVml(adj2 * 2, heighRt, 21600);
        // Java: reAdjust.setAdjust(1, adjRe);
        reAdjust.setAdjust(1, adjRe);
        // Java: int xRangeStart = 10800 + adjRe / 2;
        let xRangeStart = 10800 + adjRe / 2;
        // Java: int adj = geoAdjust.hasAdjust(0) ? geoAdjust.getAdjust(0) : 25000;
        let adj = if geoAdjust.hasAdjust(0) { geoAdjust.getAdjust(0) } else { 25000 };
        // Java: adjRe = (int)((double)1.0F * (double)adj / (double)adj2 * (double)50000.0F);
        let adjRe = (1.0f64 * adj as f64 / adj2 as f64 * 50000.0f64) as i32;
        // Java: adjRe = AdjVal.ratio2DgVml(adjRe, widthRt, 21600 - xRangeStart) + xRangeStart;
        let adjRe = AdjVal::ratio2DgVml(adjRe, widthRt, 21600 - xRangeStart) + xRangeStart;
        // Java: reAdjust.setAdjust(0, adjRe);
        reAdjust.setAdjust(0, adjRe);
        // Java: adj = geoAdjust.hasAdjust(2) ? geoAdjust.getAdjust(2) : 25000;
        let adj = if geoAdjust.hasAdjust(2) { geoAdjust.getAdjust(2) } else { 25000 };
        // Java: adjRe = AdjVal.ratio2DgVml(2 * adj, widthRt, 10800);
        let adjRe = AdjVal::ratio2DgVml(2 * adj, widthRt, 10800);
        // Java: reAdjust.setAdjust(2, adjRe);
        reAdjust.setAdjust(2, adjRe);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_bentArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_bentArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = AdjVal.dgVml2Ratio(21600 - adj, widthRt, 9173) / 2;
            let adjRe = AdjVal::dgVml2Ratio(21600 - adj, widthRt, 9173) / 2;
            // Java: reAdjust.setAdjust(2, adjRe);
            reAdjust.setAdjust(2, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = AdjVal.dgVml2Ratio(6079 - adj, heighRt, 6079) / 2;
            let adjRe = AdjVal::dgVml2Ratio(6079 - adj, heighRt, 6079) / 2;
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: reAdjust.setAdjust(1, 25000);
        reAdjust.setAdjust(1, 25000);
        // Java: reAdjust.setAdjust(3, 50000);
        reAdjust.setAdjust(3, 50000);
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_bentArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_bentArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float heighRt = Math.min(1.0F / whRatio, 1.0F);
        let heighRt = (1.0f32 / whRatio).min(1.0);
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = 6079 - AdjVal.ratio2DgVml(adj * 2, heighRt, 6079);
            let adjRe = 6079 - AdjVal::ratio2DgVml(adj * 2, heighRt, 6079);
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: if (geoAdjust.hasAdjust(2))
        if geoAdjust.hasAdjust(2) {
            // Java: int adj = geoAdjust.getAdjust(2);
            let adj = geoAdjust.getAdjust(2);
            // Java: int adjRe = 21600 - AdjVal.ratio2DgVml(adj * 2, widthRt, 9173);
            let adjRe = 21600 - AdjVal::ratio2DgVml(adj * 2, widthRt, 9173);
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_blockArc(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_blockArc(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: float adjRe1 = 1.0F * (float)adj / 65536.0F;
            let mut adjRe1 = 1.0f32 * adj as f32 / 65536.0;
            // Java: if (adjRe1 < 0.0F)
            if adjRe1 < 0.0 {
                // Java: adjRe1 += 360.0F;
                adjRe1 += 360.0;
            }
            // Java: reAdjust.setAdjust(0, (int)(adjRe1 * 60000.0F));
            reAdjust.setAdjust(0, (adjRe1 * 60000.0) as i32);
            // Java: float adjRe2 = 180.0F - adjRe1;
            let mut adjRe2 = 180.0f32 - adjRe1;
            // Java: if (adjRe2 < 0.0F)
            if adjRe2 < 0.0 {
                // Java: adjRe2 += 360.0F;
                adjRe2 += 360.0;
            }
            // Java: reAdjust.setAdjust(1, (int)(adjRe2 * 60000.0F));
            reAdjust.setAdjust(1, (adjRe2 * 60000.0) as i32);
        // Java: else
        } else {
            // Java: reAdjust.setAdjust(0, 10800000);
            reAdjust.setAdjust(0, 10800000);
        }
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: int adjRe = AdjVal.dgVml2Ratio(10800 - adj, widthRt, 10800) / 2;
            let adjRe = AdjVal::dgVml2Ratio(10800 - adj, widthRt, 10800) / 2;
            // Java: reAdjust.setAdjust(2, adjRe);
            reAdjust.setAdjust(2, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_blockArc(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_blockArc(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: float adjRe1 = 1.0F * (float)adj / 60000.0F;
            let mut adjRe1 = 1.0f32 * adj as f32 / 60000.0;
            // Java: if (adjRe1 > 180.0F)
            if adjRe1 > 180.0 {
                // Java: adjRe1 -= 360.0F;
                adjRe1 -= 360.0;
            }
            // Java: reAdjust.setAdjust(0, (int)(adjRe1 * 65536.0F));
            reAdjust.setAdjust(0, (adjRe1 * 65536.0) as i32);
        }
        // Java: if (geoAdjust.hasAdjust(2))
        if geoAdjust.hasAdjust(2) {
            // Java: int adj = geoAdjust.getAdjust(2);
            let adj = geoAdjust.getAdjust(2);
            // Java: int adjRe = 10800 - AdjVal.ratio2DgVml(2 * adj, widthRt, 10800);
            let adjRe = 10800 - AdjVal::ratio2DgVml(2 * adj, widthRt, 10800);
            // Java: reAdjust.setAdjust(1, adjRe);
            reAdjust.setAdjust(1, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_circularArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_circularArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: reAdjust.setAdjust(1, 1800000);
        reAdjust.setAdjust(1, 1800000);
        // Java: if (geoAdjust.hasAdjust(3))
        if geoAdjust.hasAdjust(3) {
            // Java: int adj = geoAdjust.getAdjust(3);
            let adj = geoAdjust.getAdjust(3);
            // Java: float adjRe = 1.0F * (float)adj / 65536.0F;
            let mut adjRe = 1.0f32 * adj as f32 / 65536.0;
            // Java: if (adjRe < 0.0F)
            if adjRe < 0.0 {
                // Java: adjRe += 360.0F;
                adjRe += 360.0;
            }
            // Java: reAdjust.setAdjust(3, (int)(adjRe * 60000.0F));
            reAdjust.setAdjust(3, (adjRe * 60000.0) as i32);
        }
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: float adjRe = 1.0F * (float)adj / 65536.0F;
            let mut adjRe = 1.0f32 * adj as f32 / 65536.0;
            // Java: if (adjRe < 0.0F)
            if adjRe < 0.0 {
                // Java: adjRe += 360.0F;
                adjRe += 360.0;
            }
            // Java: reAdjust.setAdjust(2, (int)(adjRe * 60000.0F));
            reAdjust.setAdjust(2, (adjRe * 60000.0) as i32);
        }
        // Java: if (geoAdjust.hasAdjust(2))
        if geoAdjust.hasAdjust(2) {
            // Java: int adj = geoAdjust.getAdjust(2);
            let adj = geoAdjust.getAdjust(2);
            // Java: int adjRe = AdjVal.dgVml2Ratio(10800 - adj, widthRt, 10800) / 2;
            let adjRe = AdjVal::dgVml2Ratio(10800 - adj, widthRt, 10800) / 2;
            // Java: reAdjust.setAdjust(0, adjRe);
            reAdjust.setAdjust(0, adjRe);
            // Java: reAdjust.setAdjust(4, adjRe / 2 + 12500);
            reAdjust.setAdjust(4, adjRe / 2 + 12500);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_circularArrow(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_circularArrow(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: float widthRt = Math.min(whRatio, 1.0F);
        let widthRt = whRatio.min(1.0);
        // Java: if (geoAdjust.hasAdjust(3))
        if geoAdjust.hasAdjust(3) {
            // Java: int adj = geoAdjust.getAdjust(3);
            let adj = geoAdjust.getAdjust(3);
            // Java: float adjRe = 1.0F * (float)adj / 60000.0F;
            let mut adjRe = 1.0f32 * adj as f32 / 60000.0;
            // Java: if (adjRe >= 180.0F)
            if adjRe >= 180.0 {
                // Java: adjRe -= 360.0F;
                adjRe -= 360.0;
            }
            // Java: reAdjust.setAdjust(3, (int)(adjRe * 65536.0F));
            reAdjust.setAdjust(3, (adjRe * 65536.0) as i32);
        }
        // Java: if (geoAdjust.hasAdjust(2))
        if geoAdjust.hasAdjust(2) {
            // Java: int adj = geoAdjust.getAdjust(2);
            let adj = geoAdjust.getAdjust(2);
            // Java: float adjRe = 1.0F * (float)adj / 60000.0F;
            let mut adjRe = 1.0f32 * adj as f32 / 60000.0;
            // Java: if (adjRe >= 180.0F)
            if adjRe >= 180.0 {
                // Java: adjRe -= 360.0F;
                adjRe -= 360.0;
            }
            // Java: reAdjust.setAdjust(1, (int)(adjRe * 65536.0F));
            reAdjust.setAdjust(1, (adjRe * 65536.0) as i32);
        }
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: int adjRe = 10800 - AdjVal.ratio2DgVml(2 * adj, widthRt, 10800);
            let adjRe = 10800 - AdjVal::ratio2DgVml(2 * adj, widthRt, 10800);
            // Java: reAdjust.setAdjust(2, adjRe);
            reAdjust.setAdjust(2, adjRe);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_arc(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_arc(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: float adjRe = 1.0F * (float)adj / 65536.0F;
            let mut adjRe = 1.0f32 * adj as f32 / 65536.0;
            // Java: if (adjRe < 0.0F)
            if adjRe < 0.0 {
                // Java: adjRe += 360.0F;
                adjRe += 360.0;
            }
            // Java: reAdjust.setAdjust(0, (int)(adjRe * 60000.0F));
            reAdjust.setAdjust(0, (adjRe * 60000.0) as i32);
        }
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: float adjRe = 1.0F * (float)adj / 65536.0F;
            let mut adjRe = 1.0f32 * adj as f32 / 65536.0;
            // Java: if (adjRe < 0.0F)
            if adjRe < 0.0 {
                // Java: adjRe += 360.0F;
                adjRe += 360.0;
            }
            // Java: reAdjust.setAdjust(1, (int)(adjRe * 60000.0F));
            reAdjust.setAdjust(1, (adjRe * 60000.0) as i32);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2Vml_arc(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_arc(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: GeoAdjust reAdjust = new GeoAdjust();
        let mut reAdjust = GeoAdjust::new();
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: int adj = geoAdjust.getAdjust(0);
            let adj = geoAdjust.getAdjust(0);
            // Java: float adjRe = 1.0F * (float)adj / 60000.0F;
            let mut adjRe = 1.0f32 * adj as f32 / 60000.0;
            // Java: if (adjRe > 180.0F)
            if adjRe > 180.0 {
                // Java: adjRe -= 360.0F;
                adjRe -= 360.0;
            }
            // Java: reAdjust.setAdjust(0, (int)(adjRe * 65536.0F));
            reAdjust.setAdjust(0, (adjRe * 65536.0) as i32);
        }
        // Java: if (geoAdjust.hasAdjust(1))
        if geoAdjust.hasAdjust(1) {
            // Java: int adj = geoAdjust.getAdjust(1);
            let adj = geoAdjust.getAdjust(1);
            // Java: float adjRe = 1.0F * (float)adj / 60000.0F;
            let mut adjRe = 1.0f32 * adj as f32 / 60000.0;
            // Java: if (adjRe > 180.0F)
            if adjRe > 180.0 {
                // Java: adjRe -= 360.0F;
                adjRe -= 360.0;
            }
            // Java: reAdjust.setAdjust(1, (int)(adjRe * 65536.0F));
            reAdjust.setAdjust(1, (adjRe * 65536.0) as i32);
        }
        // Java: return reAdjust;
        reAdjust
    }

    // Java: static GeoAdjust convert2GSpace_nothing(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_nothing(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return geoAdjust;
        geoAdjust.clone().unwrap()
    }

    // Java: static GeoAdjust convert2Vml_nothing(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_nothing(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: return geoAdjust;
        geoAdjust.clone().unwrap()
    }

    // Java: static GeoAdjust convert2Vml_can(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2Vml_can(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: GeoAdjust reAdjust = new GeoAdjust();
            let mut reAdjust = GeoAdjust::new();
            // Java: reAdjust.setAdjust(0, (int)((float)AdjVal.adj2DgVml(geoAdjust.getAdjust(0)) * whRatio));
            reAdjust.setAdjust(0, (AdjVal::adj2DgVml(geoAdjust.getAdjust(0)) as f32 * whRatio) as i32);
            // Java: return reAdjust;
            reAdjust
        // Java: else
        } else {
            // Java: return geoAdjust;
            geoAdjust.clone().unwrap()
        }
    }

    // Java: static GeoAdjust convert2GSpace_can(GeoAdjust geoAdjust, float whRatio)
    pub fn convert2GSpace_can(geoAdjust: &GeoAdjust, whRatio: f32) -> GeoAdjust {
        // Java: if (geoAdjust.hasAdjust(0))
        if geoAdjust.hasAdjust(0) {
            // Java: GeoAdjust reAdjust = new GeoAdjust();
            let mut reAdjust = GeoAdjust::new();
            // Java: reAdjust.setAdjust(0, (int)((float)AdjVal.adj2GSpace(geoAdjust.getAdjust(0)) / whRatio));
            reAdjust.setAdjust(0, (AdjVal::adj2GSpace(geoAdjust.getAdjust(0)) as f32 / whRatio) as i32);
            // Java: return reAdjust;
            reAdjust
        // Java: else
        } else {
            // Java: return geoAdjust;
            geoAdjust.clone().unwrap()
        }
    }
}
