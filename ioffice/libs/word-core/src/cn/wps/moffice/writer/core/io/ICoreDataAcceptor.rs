// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::{TextDocument};
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;

// java public interface ICoreDataAcceptor
pub trait ICoreDataAcceptor {
    // java  void begin()
    fn begin(&mut self);

    // java  void end()
    fn end(&mut self);

    // java  TextDocument getMainDocument()
    fn getMainDocument(&self) -> &TextDocument;

    // java  void addSettings(PropertySet var1)
    fn addSettings(&mut self, var1: &PropertySet);

    // java  void addSectionProperty(PropertySet var1)
    fn addSectionProperty(&mut self, var1: &PropertySet);

    // java  KStyle addStyle(KStyle var1)
    fn addStyle(&mut self, var1: &KStyle) -> &KStyle;

    // java  void setRPrDefault(PropertySet var1)
    fn setRPrDefault(&mut self, var1: &PropertySet);

    // java  void setPPrDefault(PropertySet var1)
    fn setPPrDefault(&mut self, var1: &PropertySet);

    // java  LstData addList(LstData var1)
    fn addList(&mut self, var1: &LstData) -> &LstData;

    // java  LfoData addListOverride(int var1, LfoData var2)
    fn addListOverride(&mut self, var1: i32, var2: &LfoData) -> &LfoData;

}
