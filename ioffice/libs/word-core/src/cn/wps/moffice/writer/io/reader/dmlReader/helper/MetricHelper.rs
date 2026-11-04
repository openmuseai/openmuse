#![allow(non_snake_case)]
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use util::base::io::{MetricUtility::MetricUtility, TypeConversion::TypeConversion};

// Java: public class MetricHelper
pub struct MetricHelper;
impl MetricHelper {
    // Java: public static Float percent2Float(Attributes attrs, String name)
    pub fn percent2Float(attrs: &dyn SaxAttributes, name: &str) -> Option<f32> {
        // Java: ConvertHelper.getString4EmptyNamespace(attrs, name)
        let val = if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri("", name) }?;
        if val.ends_with('%') {
            let fVal = TypeConversion::string2Float(&val)?;
            Some(MetricUtility::Percent2Float(fVal))
        } else {
            let lVal = TypeConversion::string2Long(&val)?;
            Some(MetricUtility::STPercentageDecimal2Float(lVal))
        }
    }
}
