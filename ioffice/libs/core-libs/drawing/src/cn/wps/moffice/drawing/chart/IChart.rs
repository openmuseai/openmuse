// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;

pub struct ChartOOXmlData;
pub struct IHostApp;

// Java: public interface IChart
pub trait IChart {
    // Java: abstract ChartOOXmlData save(boolean var1)
    fn save(&self, var1: bool) -> ChartOOXmlData;

    // Java: abstract void save(String var1, String var2, String var3, int var4)
    fn save_to_path(&self, var1: String, var2: String, var3: String, var4: i32);

    // Java: abstract boolean generatePicture(String var1, int var2, int var3, float var4, boolean var5)
    fn generatePicture(&self, var1: String, var2: i32, var3: i32, var4: f32, var5: bool) -> bool;

    // Java: abstract RectF getEffectRect(RectF var1)
    fn getEffectRect(&self, var1: RectF) -> RectF;

    // Java: abstract IHostApp hostApp()
    fn hostApp(&self) -> &IHostApp;

    // Java: abstract boolean canEditDataSource()
    fn canEditDataSource(&self) -> bool;

    // Java: abstract boolean canChangeChartType()
    fn canChangeChartType(&self) -> bool;

    // Java: abstract boolean canChangeChartStyle()
    fn canChangeChartStyle(&self) -> bool;

    // Java: abstract boolean isSupportQuickLayout()
    fn isSupportQuickLayout(&self) -> bool;

    // Java: abstract boolean canSwapRowCol()
    fn canSwapRowCol(&self) -> bool;

    // Java: abstract void swapRowCol()
    fn swapRowCol(&mut self);

    // Java: abstract void onEditData(int var1, int var2)
    fn onEditData(&mut self, var1: i32, var2: i32);

    // Java: abstract void resetDataSource()
    fn resetDataSource(&mut self);

    // Java: abstract int getColColor(int var1)
    fn getColColor(&self, var1: i32) -> i32;

    // Java: abstract int getRowColor(int var1)
    fn getRowColor(&self, var1: i32) -> i32;

    // Java: abstract int xlChartType()
    fn xlChartType(&self) -> i32;

    // Java: abstract int primaryXlChartType()
    fn primaryXlChartType(&self) -> i32;

    // Java: abstract int styleId()
    fn styleId(&self) -> i32;

    // Java: abstract int colorStyleId()
    fn colorStyleId(&self) -> i32;

    // Java: abstract void setChartType(int var1, int var2, int var3)
    fn setChartType(&mut self, var1: i32, var2: i32, var3: i32);

    // Java: abstract void setStyleId(int var1)
    fn setStyleId(&mut self, var1: i32);

    // Java: abstract void setColorStyleId(int var1)
    fn setColorStyleId(&mut self, var1: i32);

    // Java: abstract void setSourceDefault()
    fn setSourceDefault(&mut self);

    // Java: abstract IChart copy()
    fn copy(&self) -> Box<dyn IChart>;

    // Java: abstract void destroy()
    fn destroy(&mut self);

    // Java: abstract int getPlotBy()
    fn getPlotBy(&self) -> i32;

    // Java: abstract void setCellRawValue(int var1, int var2, String var3)
    fn setCellRawValue(&mut self, var1: i32, var2: i32, var3: String);

    // Java: abstract String getCellRawValue(int var1, int var2)
    fn getCellRawValue(&self, var1: i32, var2: i32) -> String;

}
