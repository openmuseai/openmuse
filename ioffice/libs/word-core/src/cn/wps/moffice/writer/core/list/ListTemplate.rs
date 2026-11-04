// Java: import cn.wps.moffice.writer.core.list.ListLevels
use crate::cn::wps::moffice::writer::core::list::ListLevels::*;

// java public static enum LevelType
#[derive(Copy, Clone, Debug)]
pub enum LevelType {
    SINGLELEVEL,
    MULTILEVEL,
    HYBRIDMULTILEVEL,
}

// java public interface ListTemplate
pub trait ListTemplate {
    // java  LevelType getLevelType()
    fn getLevelType(&self) -> LevelType;

    // java  ListLevels getListLevels()
    fn getListLevels(&self) -> &(dyn ListLevels);

    // java  boolean getOutlineNumbered()
    fn getOutlineNumbered(&self) -> bool;

    // java  void setOutlineNumbered(boolean var1)
    fn setOutlineNumbered(&mut self, var1: bool);

    // java  int getTplc()
    fn getTplc(&self) -> i32;

    // java  int getLevelTplc(int var1)
    fn getLevelTplc(&self, var1: i32) -> i32;

}

// java public static enum LevelType
impl LevelType {
}
