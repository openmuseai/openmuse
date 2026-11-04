// Java: public class IChartEditInterfaces
pub struct IChartEditInterfaces {
}

// Java: public interface IChartEdit
pub trait IChartEdit {
    // Java: abstract void onCancel()
    fn onCancel(&self);
    // Java: abstract void onOK()
    fn onOK(&self);
}

// Java: public interface IChartStyleSelect
pub trait IChartStyleSelect {
    // Java: abstract void onCancel(int var1, int var2, int var3)
    fn onCancel(&self, _var1: i32, _var2: i32, _var3: i32);
    // Java: abstract void onSelectSubmit(int var1, int var2, int var3)
    fn onSelectSubmit(&self, _var1: i32, _var2: i32, _var3: i32);
}

// Java: public class IChartEditInterfaces
impl IChartEditInterfaces {
    // Java: IChartEditInterfaces()
    pub fn new() -> Self {
      // Java: super();
      IChartEditInterfaces {}
    }

}
