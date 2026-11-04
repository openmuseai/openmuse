// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public interface IFieldParser
pub trait IFieldParser {
    // java  void parse(PLCField.FieldTriple var1, Document var2, Object var3)
    fn parse(&mut self, var1: &FieldTriple, var2: &dyn Document, var3: &dyn Object);

    // java  void parseByCharBuffer(char[] var1, Document var2, KFieldType var3)
    fn parseByCharBuffer(&mut self, var1: &[char], var2: &dyn Document, var3: KFieldType);

    // java  boolean isReLayout()
    fn isReLayout(&self) -> bool;

    // java  void dispose()
    fn dispose(&mut self);

    // java  String getResult()
    fn getResult(&self) -> Option<String>;

    // java  Long getResultRange()
    fn getResultRange(&self) -> Option<i64>;

}
