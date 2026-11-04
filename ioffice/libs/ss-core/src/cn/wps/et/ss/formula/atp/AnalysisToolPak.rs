// java package cn.wps.et.ss.formula.atp;

// java createFunctionsMap 登记的键。isAtpFunction / isXLFN 只查这张表。
const FUNCTIONS: &[&str] = &[
    "_xlfn.DISPIMG",
    "ACCRINT",
    "ACCRINTM",
    "AMORDEGRC",
    "AMORLINC",
    "_xlfn.AVERAGEIF",
    "_xlfn.AVERAGEIFS",
    "_xlfn.BAHTTEXT",
    "BESSELI",
    "BESSELJ",
    "BESSELK",
    "BESSELY",
    "BIN2DEC",
    "BIN2HEX",
    "BIN2OCT",
    "COMPLEX",
    "CONVERT",
    "_xlfn.COUNTIFS",
    "COUPDAYBS",
    "COUPDAYS",
    "COUPDAYSNC",
    "COUPNCD",
    "COUPNUM",
    "COUPPCD",
    "_xlfn.CUBEKPIMEMBER",
    "_xlfn.CUBEMEMBER",
    "_xlfn.CUBEMEMBERPROPERTY",
    "_xlfn.CUBERANKEDMEMBER",
    "_xlfn.CUBESET",
    "_xlfn.CUBESETCOUNT",
    "_xlfn.CUBEVALUE",
    "CUMIPMT",
    "CUMPRINC",
    "DEC2BIN",
    "DEC2HEX",
    "DEC2OCT",
    "DELTA",
    "DISC",
    "DOLLARDE",
    "DOLLARFR",
    "DURATION",
    "EDATE",
    "EFFECT",
    "EOMONTH",
    "ERF",
    "ERFC",
    "FACTDOUBLE",
    "FVSCHEDULE",
    "GCD",
    "GESTEP",
    "HEX2BIN",
    "HEX2DEC",
    "HEX2OCT",
    "_xlfn.IFERROR",
    "IFERROR",
    "_xlfn.IFNA",
    "IFNA",
    "IMABS",
    "IMAGINARY",
    "IMARGUMENT",
    "IMCONJUGATE",
    "IMCOS",
    "IMDIV",
    "IMEXP",
    "IMLN",
    "IMLOG10",
    "IMLOG2",
    "IMPOWER",
    "IMPRODUCT",
    "IMREAL",
    "IMSIN",
    "IMSQRT",
    "IMSUB",
    "IMSUM",
    "INTRATE",
    "ISEVEN",
    "ISODD",
    "JIS",
    "LCM",
    "MDURATION",
    "MROUND",
    "MULTINOMIAL",
    "NETWORKDAYS",
    "NOMINAL",
    "OCT2BIN",
    "OCT2DEC",
    "OCT2HEX",
    "ODDFPRICE",
    "ODDFYIELD",
    "ODDLPRICE",
    "ODDLYIELD",
    "PRICE",
    "PRICEDISC",
    "PRICEMAT",
    "QUOTIENT",
    "RANDBETWEEN",
    "RECEIVED",
    "_xlfn.RTD",
    "SERIESSUM",
    "SQRTPI",
    "_xlfn.SUMIFS",
    "_xlfn.MAXIFS",
    "TBILLEQ",
    "TBILLPRICE",
    "TBILLYIELD",
    "WEEKNUM",
    "WORKDAY",
    "XIRR",
    "XNPV",
    "YEARFRAC",
    "YIELD",
    "YIELDDISC",
    "YIELDMAT",
    "_xlfn.IFS",
    "_xlfn.RANK.AVG",
    "_xlfn.TEXTJOIN",
    "XLOOKUP",
    "_xlfn.XLOOKUP",
];

// java public final class AnalysisToolPak
pub struct AnalysisToolPak;

impl AnalysisToolPak {
    // java public boolean isAtpFunction(String name)
    pub fn isAtpFunction(name: &str) -> bool {
        // java name = name.toUpperCase(); strip _XLFN.; key or "_xlfn." + name
        let name = normalize(name);
        let xlfn = format!("_xlfn.{name}");
        FUNCTIONS.iter().any(|key| *key == name || *key == xlfn)
    }

    // java public boolean isXLFN(String name)
    pub fn isXLFN(name: &str) -> bool {
        let name = normalize(name);
        let xlfn = format!("_xlfn.{name}");
        FUNCTIONS.iter().any(|key| *key == xlfn)
    }
}

fn normalize(name: &str) -> String {
    let mut name = name.to_ascii_uppercase();
    if let Some(rest) = name.strip_prefix("_XLFN.") {
        name = rest.to_string();
    }
    name
}
// java public IFunction findFunction(String name) {
// java name = name.toUpperCase();
// java if (name.startsWith("_XLFN.")) {
// java name = name.substring(6);
// java }
// java
// java String xlfnName = "_xlfn." + name;
// java IFunction ret = (IFunction)this._functionsByName.get(name);
// java if (ret == null) {
// java ret = (IFunction)this._functionsByName.get(xlfnName);
// java }
// java
// java return ret;
// java }
impl super::super::udf::IUDFFinder::IUDFFinder for AnalysisToolPak{fn findFunction(&self,name:&str)->super::super::evaluator::EvalResult<Option<std::sync::Arc<dyn super::super::functions::base::IFunction::IFunction>>>{let name=normalize(name);if name=="EDATE"{return Ok(Some(std::sync::Arc::new(super::EDate::EDate)));}if Self::isAtpFunction(&name){return Err(super::super::evaluator::EvalFailure::Unported("AnalysisToolPak function"));}Ok(None)}}
