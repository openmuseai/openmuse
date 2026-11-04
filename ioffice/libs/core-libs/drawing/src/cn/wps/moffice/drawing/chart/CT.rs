#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]

use std::sync::OnceLock;

// Java field: int WRITER
pub(crate) const WRITER: i32 = 1;
// Java field: int SPREADSHEET
pub(crate) const SPREADSHEET: i32 = 2;
// Java field: int PRESENTATION
pub(crate) const PRESENTATION: i32 = 3;
// Java field: int xlUnspecified
pub(crate) const xlUnspecified: i32 = -1;
// Java field: int xl3DArea
pub(crate) const xl3DArea: i32 = 1;
// Java field: int xl3DAreaStacked
pub(crate) const xl3DAreaStacked: i32 = 2;
// Java field: int xl3DAreaStacked100
pub(crate) const xl3DAreaStacked100: i32 = 3;
// Java field: int xl3DBarClustered
pub(crate) const xl3DBarClustered: i32 = 4;
// Java field: int xl3DBarStacked
pub(crate) const xl3DBarStacked: i32 = 5;
// Java field: int xl3DBarStacked100
pub(crate) const xl3DBarStacked100: i32 = 6;
// Java field: int xl3DColumn
pub(crate) const xl3DColumn: i32 = 7;
// Java field: int xl3DColumnClustered
pub(crate) const xl3DColumnClustered: i32 = 8;
// Java field: int xl3DColumnStacked
pub(crate) const xl3DColumnStacked: i32 = 9;
// Java field: int xl3DColumnStacked100
pub(crate) const xl3DColumnStacked100: i32 = 10;
// Java field: int xl3DLine
pub(crate) const xl3DLine: i32 = 11;
// Java field: int xl3DPie
pub(crate) const xl3DPie: i32 = 12;
// Java field: int xl3DPieExploded
pub(crate) const xl3DPieExploded: i32 = 13;
// Java field: int xlArea
pub(crate) const xlArea: i32 = 14;
// Java field: int xlAreaStacked
pub(crate) const xlAreaStacked: i32 = 15;
// Java field: int xlAreaStacked100
pub(crate) const xlAreaStacked100: i32 = 16;
// Java field: int xlBarClustered
pub(crate) const xlBarClustered: i32 = 17;
// Java field: int xlBarOfPie
pub(crate) const xlBarOfPie: i32 = 18;
// Java field: int xlBarStacked
pub(crate) const xlBarStacked: i32 = 19;
// Java field: int xlBarStacked100
pub(crate) const xlBarStacked100: i32 = 20;
// Java field: int xlBubble
pub(crate) const xlBubble: i32 = 21;
// Java field: int xlBubble3DEffect
pub(crate) const xlBubble3DEffect: i32 = 22;
// Java field: int xlColumnClustered
pub(crate) const xlColumnClustered: i32 = 23;
// Java field: int xlColumnStacked
pub(crate) const xlColumnStacked: i32 = 24;
// Java field: int xlColumnStacked100
pub(crate) const xlColumnStacked100: i32 = 25;
// Java field: int xlConeBarClustered
pub(crate) const xlConeBarClustered: i32 = 26;
// Java field: int xlConeBarStacked
pub(crate) const xlConeBarStacked: i32 = 27;
// Java field: int xlConeBarStacked100
pub(crate) const xlConeBarStacked100: i32 = 28;
// Java field: int xlConeCol
pub(crate) const xlConeCol: i32 = 29;
// Java field: int xlConeColClustered
pub(crate) const xlConeColClustered: i32 = 30;
// Java field: int xlConeColStacked
pub(crate) const xlConeColStacked: i32 = 31;
// Java field: int xlConeColStacked100
pub(crate) const xlConeColStacked100: i32 = 32;
// Java field: int xlCylinderBarClustered
pub(crate) const xlCylinderBarClustered: i32 = 33;
// Java field: int xlCylinderBarStacked
pub(crate) const xlCylinderBarStacked: i32 = 34;
// Java field: int xlCylinderBarStacked100
pub(crate) const xlCylinderBarStacked100: i32 = 35;
// Java field: int xlCylinderCol
pub(crate) const xlCylinderCol: i32 = 36;
// Java field: int xlCylinderColClustered
pub(crate) const xlCylinderColClustered: i32 = 37;
// Java field: int xlCylinderColStacked
pub(crate) const xlCylinderColStacked: i32 = 38;
// Java field: int xlCylinderColStacked100
pub(crate) const xlCylinderColStacked100: i32 = 39;
// Java field: int xlDoughnut
pub(crate) const xlDoughnut: i32 = 40;
// Java field: int xlDoughnutExploded
pub(crate) const xlDoughnutExploded: i32 = 41;
// Java field: int xlLine
pub(crate) const xlLine: i32 = 42;
// Java field: int xlLineMarkers
pub(crate) const xlLineMarkers: i32 = 43;
// Java field: int xlLineMarkersStacked
pub(crate) const xlLineMarkersStacked: i32 = 44;
// Java field: int xlLineMarkersStacked100
pub(crate) const xlLineMarkersStacked100: i32 = 45;
// Java field: int xlLineStacked
pub(crate) const xlLineStacked: i32 = 46;
// Java field: int xlLineStacked100
pub(crate) const xlLineStacked100: i32 = 47;
// Java field: int xlPie
pub(crate) const xlPie: i32 = 48;
// Java field: int xlPieExploded
pub(crate) const xlPieExploded: i32 = 49;
// Java field: int xlPieOfPie
pub(crate) const xlPieOfPie: i32 = 50;
// Java field: int xlPyramidBarClustered
pub(crate) const xlPyramidBarClustered: i32 = 51;
// Java field: int xlPyramidBarStacked
pub(crate) const xlPyramidBarStacked: i32 = 52;
// Java field: int xlPyramidBarStacked100
pub(crate) const xlPyramidBarStacked100: i32 = 53;
// Java field: int xlPyramidCol
pub(crate) const xlPyramidCol: i32 = 54;
// Java field: int xlPyramidColClustered
pub(crate) const xlPyramidColClustered: i32 = 55;
// Java field: int xlPyramidColStacked
pub(crate) const xlPyramidColStacked: i32 = 56;
// Java field: int xlPyramidColStacked100
pub(crate) const xlPyramidColStacked100: i32 = 57;
// Java field: int xlRadar
pub(crate) const xlRadar: i32 = 58;
// Java field: int xlRadarFilled
pub(crate) const xlRadarFilled: i32 = 59;
// Java field: int xlRadarMarkers
pub(crate) const xlRadarMarkers: i32 = 60;
// Java field: int xlStockHLC
pub(crate) const xlStockHLC: i32 = 61;
// Java field: int xlStockOHLC
pub(crate) const xlStockOHLC: i32 = 62;
// Java field: int xlStockVHLC
pub(crate) const xlStockVHLC: i32 = 63;
// Java field: int xlStockVOHLC
pub(crate) const xlStockVOHLC: i32 = 64;
// Java field: int xlSurface
pub(crate) const xlSurface: i32 = 65;
// Java field: int xlSurfaceTopView
pub(crate) const xlSurfaceTopView: i32 = 66;
// Java field: int xlSurfaceTopViewWireframe
pub(crate) const xlSurfaceTopViewWireframe: i32 = 67;
// Java field: int xlSurfaceWireframe
pub(crate) const xlSurfaceWireframe: i32 = 68;
// Java field: int xlXYScatter
pub(crate) const xlXYScatter: i32 = 69;
// Java field: int xlXYScatterLines
pub(crate) const xlXYScatterLines: i32 = 70;
// Java field: int xlXYScatterLinesNoMarkers
pub(crate) const xlXYScatterLinesNoMarkers: i32 = 71;
// Java field: int xlXYScatterSmooth
pub(crate) const xlXYScatterSmooth: i32 = 72;
// Java field: int xlXYScatterSmoothNoMarkers
pub(crate) const xlXYScatterSmoothNoMarkers: i32 = 73;
// Java field: int xlEnd
pub(crate) const xlEnd: i32 = 73;
// Java field: int[] radar
static radar: OnceLock<Vec<i32>> = OnceLock::new();
fn get_radar() -> &'static Vec<i32> { radar.get_or_init(|| vec![58, 59, 60]) }
// Java field: int[] area
static area: OnceLock<Vec<i32>> = OnceLock::new();
fn get_area() -> &'static Vec<i32> { area.get_or_init(|| vec![1, 2, 3, 14, 15, 16]) }
// Java field: int[] line
static line: OnceLock<Vec<i32>> = OnceLock::new();
fn get_line() -> &'static Vec<i32> { line.get_or_init(|| vec![11, 42, 43, 44, 45, 46, 47]) }
// Java field: int[] bar
static bar: OnceLock<Vec<i32>> = OnceLock::new();
fn get_bar() -> &'static Vec<i32> { bar.get_or_init(|| vec![4, 5, 6, 17, 19, 20, 26, 27, 28, 33, 34, 35, 51, 52, 53]) }
// Java field: int[] column
static column: OnceLock<Vec<i32>> = OnceLock::new();
fn get_column() -> &'static Vec<i32> { column.get_or_init(|| vec![7, 8, 9, 10, 23, 24, 25, 29, 30, 31, 32, 36, 37, 38, 39, 54, 55, 56, 57]) }
// Java field: int[] scatter
static scatter: OnceLock<Vec<i32>> = OnceLock::new();
fn get_scatter() -> &'static Vec<i32> { scatter.get_or_init(|| vec![69, 70, 71, 72, 73]) }
// Java field: int[] pie
static pie: OnceLock<Vec<i32>> = OnceLock::new();
fn get_pie() -> &'static Vec<i32> { pie.get_or_init(|| vec![12, 13, 18, 48, 49, 50]) }
// Java field: int[] doughnut
static doughnut: OnceLock<Vec<i32>> = OnceLock::new();
fn get_doughnut() -> &'static Vec<i32> { doughnut.get_or_init(|| vec![40, 41]) }
// Java field: int[] bubbles
static bubbles: OnceLock<Vec<i32>> = OnceLock::new();
fn get_bubbles() -> &'static Vec<i32> { bubbles.get_or_init(|| vec![21, 22]) }
// Java field: int[] stock
static stock: OnceLock<Vec<i32>> = OnceLock::new();
fn get_stock() -> &'static Vec<i32> { stock.get_or_init(|| vec![61, 62, 63, 64]) }
// Java field: int[] surface
static surface: OnceLock<Vec<i32>> = OnceLock::new();
fn get_surface() -> &'static Vec<i32> { surface.get_or_init(|| vec![65, 66, 67, 68]) }
// Java field: int[] three3d
static three3d: OnceLock<Vec<i32>> = OnceLock::new();
fn get_three3d() -> &'static Vec<i32> { three3d.get_or_init(|| vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 51, 52, 53, 54, 55, 56, 57, 65, 68]) }

// Java: public class CT
pub struct CT {
}

// Java: public class HOST_APP
pub struct HOST_APP {
}

// Java: public class XL_CHART_TYPE
pub struct XL_CHART_TYPE {
}

// Java: public class CT
impl CT {
    // Java: CT()
    pub fn new() -> Self {
      // Java: super();
      CT {}
    }

}

// Java: public class HOST_APP
impl HOST_APP {
    // Java: HOST_APP()
    pub fn new() -> Self {
         // Java: super();
         HOST_APP {}
    }

    // Java: static boolean isWriter(int hostAppType)
    pub fn isWriter(hostAppType: i32) -> bool {
         // Java: return hostAppType == 1;
         hostAppType == 1
    }

    // Java: static boolean isSpreadsheet(int hostAppType)
    pub fn isSpreadsheet(hostAppType: i32) -> bool {
         // Java: return hostAppType == 2;
         hostAppType == 2
    }

    // Java: static boolean isPresentation(int hostAppType)
    pub fn isPresentation(hostAppType: i32) -> bool {
         // Java: return hostAppType == 3;
         hostAppType == 3
    }

}

// Java: public class XL_CHART_TYPE
impl XL_CHART_TYPE {
    // Java: XL_CHART_TYPE()
    pub fn new() -> Self {
         // Java: super();
         XL_CHART_TYPE {}
    }

    // Java: static boolean isRadar(int xlChartType)
    pub fn isRadar(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(radar, xlChartType) >= 0;
         get_radar().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isArea(int xlChartType)
    pub fn isArea(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(area, xlChartType) >= 0;
         get_area().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isLine(int xlChartType)
    pub fn isLine(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(line, xlChartType) >= 0;
         get_line().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isBar(int xlChartType)
    pub fn isBar(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(bar, xlChartType) >= 0;
         get_bar().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isColumn(int xlChartType)
    pub fn isColumn(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(column, xlChartType) >= 0;
         get_column().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isScatter(int xlChartType)
    pub fn isScatter(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(scatter, xlChartType) >= 0;
         get_scatter().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isPie(int xlChartType)
    pub fn isPie(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(pie, xlChartType) >= 0;
         get_pie().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isDoughnut(int xlChartType)
    pub fn isDoughnut(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(doughnut, xlChartType) >= 0;
         get_doughnut().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isBubble(int xlChartType)
    pub fn isBubble(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(bubbles, xlChartType) >= 0;
         get_bubbles().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isStock(int xlChartType)
    pub fn isStock(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(stock, xlChartType) >= 0;
         get_stock().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isSurface(int xlChartType)
    pub fn isSurface(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(surface, xlChartType) >= 0;
         get_surface().binary_search(&xlChartType).is_ok()
    }

    // Java: static boolean isPercent(int xlChartType)
    pub fn isPercent(xlChartType: i32) -> bool {
         // Java: return xlChartType == 16 || xlChartType == 47 || xlChartType == 45 || xlChartType == 20 || xlChartType == 25 || xlChartType == 10 || xlChartType == 6 || xlChartType == 3 || xlChartType == 28 || xlChartType == 45 || xlChartType == 35 || xlChartType == 39 || xlChartType == 53 || xlChartType == 57;
         xlChartType == 16 || xlChartType == 47 || xlChartType == 45 || xlChartType == 20 || xlChartType == 25 || xlChartType == 10 || xlChartType == 6 || xlChartType == 3 || xlChartType == 28 || xlChartType == 45 || xlChartType == 35 || xlChartType == 39 || xlChartType == 53 || xlChartType == 57
    }

    // Java: static boolean isColumnStacked(int xlChartType)
    pub fn isColumnStacked(xlChartType: i32) -> bool {
         // Java: return 9 == xlChartType || 10 == xlChartType || 24 == xlChartType || 25 == xlChartType;
         9 == xlChartType || 10 == xlChartType || 24 == xlChartType || 25 == xlChartType
    }

    // Java: static boolean isBarStacked(int xlChartType)
    pub fn isBarStacked(xlChartType: i32) -> bool {
         // Java: return 6 == xlChartType || 5 == xlChartType || 19 == xlChartType || 20 == xlChartType;
         6 == xlChartType || 5 == xlChartType || 19 == xlChartType || 20 == xlChartType
    }

    // Java: static boolean isStacked(int xlChartType)
    pub fn isStacked(xlChartType: i32) -> bool {
         // Java: return xlChartType == 15 || xlChartType == 46 || xlChartType == 44 || xlChartType == 19 || xlChartType == 24;
         xlChartType == 15 || xlChartType == 46 || xlChartType == 44 || xlChartType == 19 || xlChartType == 24
    }

    // Java: static boolean is3D(int xlChartType)
    pub fn is3D(xlChartType: i32) -> bool {
         // Java: return Arrays.binarySearch(three3d, xlChartType) >= 0;
         get_three3d().binary_search(&xlChartType).is_ok()
    }

    // Java: static String getOfPieName(int xlChartType)
    pub fn getOfPieName(xlChartType: i32) -> String {
         // Java: return xlChartType == 18 ? "bar" : "pie";
         if xlChartType == 18 { "bar".to_string() } else { "pie".to_string() }
    }

    // Java: static int getOfPieType(String name)
    pub fn getOfPieType(name: &str) -> i32 {
         // Java: return "bar".equals(name) ? 18 : 50;
         if name == "bar" { 18 } else { 50 }
    }

    // Java: static boolean hasLine(int xlChartType)
    pub fn hasLine(xlChartType: i32) -> bool {
         // Java: boolean normalLine = isLine(xlChartType);
         let normalLine = Self::isLine(xlChartType);
         // Java: boolean scaterLine = isScatter(xlChartType) && xlChartType != 69;
         let scaterLine = Self::isScatter(xlChartType) && xlChartType != 69;
         // Java: boolean radarLine = isRadar(xlChartType);
         let radarLine = Self::isRadar(xlChartType);
         // Java: boolean stockLine = isStock(xlChartType);
         let stockLine = Self::isStock(xlChartType);
         // Java: return normalLine || scaterLine || radarLine || stockLine;
         normalLine || scaterLine || radarLine || stockLine
    }

}
