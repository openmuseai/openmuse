#![allow(non_snake_case, dead_code)]

// Java: public interface IChartEventListener
pub trait IChartEventListener {
    // Java: abstract void onChartContentsChanged(int var1)
    fn onChartContentsChanged(&self, var1: i32);

    // Java: abstract void onTransactionChanged(boolean var1)
    fn onTransactionChanged(&self, var1: bool);

}
