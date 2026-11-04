use std::collections::HashMap;

// Java field: HashMap<String, MsoCellSpecial> sMap
// Java field: String Placeholder
// Java field: String TemplateColumn
// Java field: String TemplateRow

// Java: public enum MsoCellSpecial
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MsoCellSpecial {
    Placeholder,
    TemplateColumn,
    TemplateRow,
}

// Java: public enum MsoCellSpecial
impl MsoCellSpecial {
    // Java: MsoCellSpecial(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", MsoCellSpecial.NAME.sMap);
        // Java: MsoCellSpecial.NAME.sMap.put(value, this);
        match self {
            Self::Placeholder => "placeholder",
            Self::TemplateColumn => "template-column",
            Self::TemplateRow => "template-row",
        }
    }

    // Java: static MsoCellSpecial fromValue(String v)
    pub fn from_value(v: &str) -> Option<MsoCellSpecial> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", MsoCellSpecial.NAME.sMap);
        // Java: return (MsoCellSpecial)MsoCellSpecial.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String Placeholder
    pub const Placeholder: &str = "placeholder";
    // Java field: String TemplateColumn
    pub const TemplateColumn: &str = "template-column";
    // Java field: String TemplateRow
    pub const TemplateRow: &str = "template-row";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[MsoCellSpecial] = &[MsoCellSpecial::Placeholder, MsoCellSpecial::TemplateColumn, MsoCellSpecial::TemplateRow];
