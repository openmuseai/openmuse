// Rust stub for KmoOleChart (auto-generated)
// Original: cn/wps/drawing/app/chart/KmoOleChart.java
use crate::*;

pub struct KmoOleChart {
    // TODO: fields
}

impl KmoOleChart {
    pub fn new() -> Self { todo!() }
    pub fn create(&self, r#type: i32, styleID: i32, colorId: i32, themePath: String, rids: Integer>) { todo!() }
    pub fn hostApp(&self) -> DgHostApp { todo!() }
    pub fn r#type(&self) -> i32 { todo!() }
    pub fn copy(&self) -> Box<dyn GraphicObject> { todo!() }
    pub fn getChartXmlPath(&self) -> String { todo!() }
    pub fn setChartXmlPath(&self, chartXmlPath: String) { todo!() }
    pub fn getWorkbookPath(&self) -> String { todo!() }
    pub fn setWorkbookPath(&self, workbookPath: String) { todo!() }
    pub fn getColorStylePath(&self) -> String { todo!() }
    pub fn setColorStylePath(&self, colorStylePath: String) { todo!() }
    pub fn getChartStylePath(&self) -> String { todo!() }
    pub fn setChartStylePath(&self, chartStylePath: String) { todo!() }
    pub fn getExternalDataRID(&self) -> String { todo!() }
    pub fn setExternalDataRID(&self, externalDataRID: String) { todo!() }
    pub fn addMeidaItem(&self, rID: String, mediaID: i32) { todo!() }
    pub fn getChartMediaRidMap(&self) -> Map { todo!() }
    pub fn getThemePath(&self) -> String { todo!() }
    pub fn setThemePath(&self, themePath: String) { todo!() }
    pub fn setGVml(&self, isGVml: bool) { todo!() }
    pub fn isThemeForChart(&self) -> bool { todo!() }
    pub fn setThemeForChart(&self, themeForChart: bool) { todo!() }
    pub fn isThemeForChartOverride(&self) -> bool { todo!() }
    pub fn setThemeForChartOverride(&self, themeForChartOverride: bool) { todo!() }
    pub fn getChart(&self) -> Box<dyn IChart> { todo!() }
    pub fn saveChart(&self, isGVml: bool) -> ChartOOXmlData { todo!() }
    pub fn getrId(&self) -> String { todo!() }
    pub fn setrId(&self, rId: String) { todo!() }
    pub fn setChart(&self, chart: Box<dyn IChart>) { todo!() }
    pub fn openChart(&self) { todo!() }
    pub fn resetChartData(&self) { todo!() }
    pub fn swapRowCol(&self) { todo!() }
    pub fn editDataSource(&self) { todo!() }
    pub fn changeStyle(&self, styleId: i32) { todo!() }
    pub fn changeColorStyle(&self, colorId: i32) { todo!() }
    pub fn changeChart(&self, r#type: i32, styleId: i32, colorId: i32) { todo!() }
    pub fn didFinishSwapping(&self, isUndo: bool) { todo!() }
    pub fn getLoader(&self) -> Box<dyn std::any::Any + Send + Sync> { todo!() }
    pub fn createChart(&self, r#type: i32, styleId: i32, colorId: i32) -> Box<dyn IChart> { todo!() }
    pub fn updateChartFile(&self, chart: Box<dyn IChart>) { todo!() }
    pub fn clear(&self) { todo!() }
    pub fn chartType(&self) -> Integer { todo!() }
    pub fn styleId(&self) -> i32 { todo!() }
    pub fn colorStyleId(&self) -> i32 { todo!() }
    pub fn setExternalPath(&self, externalPath: String) { todo!() }
    pub fn getExternalPath(&self) -> String { todo!() }
    pub fn setExernalType(&self, exernalType: String) { todo!() }
    pub fn getExternalType(&self) -> String { todo!() }
    pub fn isExternalChart(&self) -> bool { todo!() }
    pub fn relationships(&self) -> ChartExtRelation { todo!() }
}
