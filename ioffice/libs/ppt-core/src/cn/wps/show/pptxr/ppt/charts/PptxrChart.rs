// Rust stub for PptxrChart (auto-generated)
// Original: cn/wps/show/pptxr/ppt/charts/PptxrChart.java
use crate::*;

pub struct PptxrChart {
    // TODO: fields
}

impl PptxrChart {
    pub fn new() -> Self { todo!() }
    pub fn open(&self) { todo!() }
    pub fn openSingleChart(&self, packagePart: OpcrPackagePart, rId: String) { todo!() }
    pub fn isGVml(&self) -> bool { todo!() }
    pub fn importHostApp(&self, chartThemeOverride: OpcrRelationship) { todo!() }
    pub fn importHostAppTheme(&self, chartThemeOverride: OpcrRelationship) { todo!() }
    pub fn readChartThemeOverride(&self, chartThemeOverride: OpcrRelationship) -> KmoTheme { todo!() }
    pub fn getRels(&self, relation: OpcRelation, rId: String) -> OpcrRelationship { todo!() }
    pub fn importRelations(&self, relses: OpcrRelationships) -> OpcrRelationship { todo!() }
    pub fn isChartImage(&self, relationType: String) -> bool { todo!() }
    pub fn importImage(&self, rels: OpcrRelationship) { todo!() }
    pub fn isStyleChart(&self, relationType: String) -> bool { todo!() }
    pub fn importChartStyle(&self, packagePart: OpcrPackagePart) { todo!() }
    pub fn isColorChart(&self, relationType: String) -> bool { todo!() }
    pub fn importColorStyle(&self, packagePart: OpcrPackagePart) { todo!() }
    pub fn isChartSrc(relType: String) -> bool { todo!() }
    pub fn importChartSrc(&self, rels: OpcrRelationship, packagePart: OpcrPackagePart) { todo!() }
    pub fn isThemeOverride(&self, relType: String) -> bool { todo!() }
    pub fn isTheme(&self, relType: String) -> bool { todo!() }
}
