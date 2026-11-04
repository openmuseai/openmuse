#![allow(non_snake_case, dead_code)]

// Java: import java.io.IOException

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IChartAppProxy
pub trait IChartAppProxy {
    // Java: abstract IChart create(int var1, int var2, IHostApp var3)
    fn create(&self, var1: i32, var2: i32, var3: Element) -> Element;

    // Java: abstract IChart create(int var1, int var2, int var3, IHostApp var4)
    fn create_with_type(&self, var1: i32, var2: i32, var3: i32, var4: Element) -> Element;

    // Java: abstract IChart open(ChartOOXmlData var1, IHostApp var2)
    fn open(&self, var1: Element, var2: Element) -> Element;

    // Java: abstract IChartDevice createDevice(IChart var1)
    fn createDevice(&self, var1: Element) -> Element;

}
