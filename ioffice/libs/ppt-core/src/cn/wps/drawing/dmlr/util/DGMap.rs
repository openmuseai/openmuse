use std::collections::HashMap;
use std::sync::OnceLock;

// java public class DGMap
pub struct DGMap {}

impl DGMap {
    // java public static final Map blackWhiteModeMap
    pub fn blackWhiteModeMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                ("auto", 1),
                ("black", 8),
                ("blackGray", 6),
                ("blackWhite", 7),
                ("clr", 0),
                ("gray", 2),
                ("grayWhite", 5),
                ("hidden", 10),
                ("invGray", 4),
                ("ltGray", 3),
                ("white", 9),
            ])
        })
    }

    // java public static final Map tileFlipMap
    pub fn tileFlipMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([("none", 0), ("x", 1), ("xy", 3), ("y", 2)])
        })
    }

    // java public static final Map blipCompressionMap
    pub fn blipCompressionMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                ("email", 0),
                ("hqprint", 1),
                ("none", 2),
                ("print", 3),
                ("screen", 4),
            ])
        })
    }

    // java public static final Map schemeColorMap
    pub fn schemeColorMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                ("accent1", 4),
                ("accent2", 5),
                ("accent3", 6),
                ("accent4", 7),
                ("accent5", 8),
                ("accent6", 9),
                ("bg1", 12),
                ("bg2", 13),
                ("dk1", 0),
                ("dk2", 2),
                ("folHlink", 11),
                ("hlink", 10),
                ("lt1", 1),
                ("lt2", 3),
                ("phClr", 14),
                ("tx1", 15),
                ("tx2", 16),
            ])
        })
    }

    // java public static final Map presetPatternMap
    pub fn presetPatternMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("cross", 22), ("horz", 12), ("vert", 13)]))
    }

    // java public static final Map lineCapMap
    pub fn lineCapMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("flat", 2), ("sq", 1), ("rnd", 0)]))
    }

    // java public static final Map penAlignmentMap
    pub fn penAlignmentMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("ctr", 0), ("in", 1)]))
    }

    // java public static final Map compoundLineTypeMap
    pub fn compoundLineTypeMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                ("dbl", 1),
                ("sng", 0),
                ("thickThin", 2),
                ("thinThick", 3),
                ("tri", 4),
            ])
        })
    }

    // java public static final Map presetLineDashValueMap
    pub fn presetLineDashValueMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                ("solid", 0),
                ("sysDash", 1),
                ("sysDot", 2),
                ("sysDashDot", 3),
                ("sysDashDotDot", 4),
                ("dot", 5),
                ("dash", 6),
                ("lgDash", 7),
                ("dashDot", 8),
                ("lgDashDot", 9),
                ("lgDashDotDot", 10),
            ])
        })
    }

    // java public static final Map lineEndTypeMap
    pub fn lineEndTypeMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                ("none", 0),
                ("triangle", 1),
                ("stealth", 2),
                ("diamond", 3),
                ("oval", 4),
                ("arrow", 5),
            ])
        })
    }

    // java public static final Map lineEndWidthMap
    pub fn lineEndWidthMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("lg", 2), ("med", 1), ("sm", 0)]))
    }

    // java public static final Map lineEndLengthMap
    pub fn lineEndLengthMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("lg", 2), ("med", 1), ("sm", 0)]))
    }

    // java public static final Map presetShapeTypeMap
    pub fn presetShapeTypeMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                ("rect", 1),
                ("roundRect", 2),
                ("ellipse", 3),
                ("line", 20),
                ("pentagon", 56),
            ])
        })
    }

    // java public static final Map fillModeMap
    pub fn fillModeMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                ("darken", 0),
                ("darkenLess", 1),
                ("lighten", 2),
                ("lightenLess", 3),
                ("none", 4),
                ("norm", 5),
            ])
        })
    }

    // java public static final Map textCapMap
    pub fn textCapMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("none", 1), ("small", 2), ("all", 0)]))
    }

    // java public static final Map textStrikeMap
    pub fn textStrikeMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([("noStrike", 0), ("sngStrike", 1), ("dblStrike", 2)])
        })
    }

    // java public static final Map textUnderlineMap
    pub fn textUnderlineMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                ("none", 12),
                ("words", 17),
                ("sng", 13),
                ("dbl", 4),
                ("heavy", 11),
                ("dotted", 9),
                ("dottedHeavy", 10),
                ("dash", 0),
                ("dashHeavy", 1),
                ("dashLong", 2),
                ("dashLongHeavy", 3),
                ("dotDash", 5),
                ("dotDashHeavy", 6),
                ("dotDotDash", 7),
                ("dotDotDashHeavy", 8),
                ("wavy", 14),
                ("wavyHeavy", 16),
                ("wavyDbl", 15),
            ])
        })
    }

    // java public static final Map textAnchoringTypeMap
    pub fn textAnchoringTypeMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([("b", 2), ("ctr", 1), ("dist", 4), ("just", 3), ("t", 0)])
        })
    }

    // java public static final Map textHorzOverflowTypeMap
    pub fn textHorzOverflowTypeMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("clip", 1), ("overflow", 0)]))
    }

    // java public static final Map textVerticalTypeMap
    pub fn textVerticalTypeMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                ("eaVert", 4),
                ("horz", 0),
                ("mongolianVert", 5),
                ("vert", 1),
                ("vert270", 2),
                ("wordArtVert", 3),
                ("wordArtVertRtl", 6),
            ])
        })
    }

    // java public static final Map textVerOverflowMap
    pub fn textVerOverflowMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("clip", 2), ("ellipsis", 1), ("overflow", 0)]))
    }

    // java public static final Map textWrapMap
    pub fn textWrapMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("none", 0), ("square", 1)]))
    }

    // java public static final Map orientationMap
    pub fn orientationMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("horz", 0), ("vert", 1)]))
    }

    // java public static final Map fontCollectionIndexMap
    //
    // rust: ★M-PPT-KMOSLIDE W6c-3 —— 原先**缺失**（`DGMap.java:692-698` 有，Rust 无），
    //   使 `StyleMatrixReferenceHandler.startElement` 的 `fontCollectionIndexMap.get(...)`
    //   无法落地。按 Java 逐项补出（取值与 `DGMap.java` 逐字一致）。
    //   owner: ppt.drawing.dmlr.util    wave: M-PPT-KMOSLIDE-W6c-3
    pub fn fontCollectionIndexMap() -> &'static HashMap<&'static str, i32> {
        static MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
        MAP.get_or_init(|| HashMap::from([("none", 2), ("minor", 1), ("major", 0)]))
    }
}
