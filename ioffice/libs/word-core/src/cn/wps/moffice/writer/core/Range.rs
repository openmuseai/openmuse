// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.comment.KComments
use crate::cn::wps::moffice::writer::core::comment::KComments;
// Java: import cn.wps.moffice.writer.core.comment.KReplyComments
use crate::cn::wps::moffice::writer::core::comment::KReplyComments;
// Java: import cn.wps.moffice.writer.core.FindOptions
use crate::cn::wps::moffice::writer::core::FindOptions::*;
// Java: import cn.wps.moffice.writer.core.TextUnits
use crate::cn::wps::moffice::writer::core::TextUnits::*;
// Java: import cn.wps.moffice.writer.core.KParagraphFormat
use crate::cn::wps::moffice::writer::core::KParagraphFormat::*;
// Java: import cn.wps.moffice.writer.core.KFont
// CARGO_BLOCKER_UNUSED_MODULE: KFont（wc-02 字体类）wave0 阶段屏蔽，use 裁剪
// use crate::cn::wps::moffice::writer::core::KFont::*;
// Java: import cn.wps.moffice.writer.core.WtStatistic
use crate::cn::wps::moffice::writer::core::WtStatistic::*;

// java public interface Range
pub trait Range {
    // java  int getStart()
    fn getStart(&self) -> i32;

    // java  int getEnd()
    fn getEnd(&self) -> i32;

    // java  String getText()
    fn getText(&self) -> String;

    // java @Deprecated Range find(String var1, boolean var2, boolean var3, boolean var4)
    fn find__string_boolean_boolean_boolean(&mut self, var1: &str, var2: bool, var3: bool, var4: bool) -> &(dyn Range);

    // java  Range find(String var1, boolean var2, FindOptions var3)
    fn find__string_boolean_findoptions(&mut self, var1: &str, var2: bool, var3: &FindOptions) -> &(dyn Range);

    // java  int deleteRange(boolean var1)
    fn deleteRange__boolean(&mut self, var1: bool) -> i32;

    // java  int deleteRange()
    fn deleteRange(&mut self) -> i32;

    // java  int moveEnd(TextUnits var1, int var2, boolean var3)
    fn moveEnd(&mut self, var1: &TextUnits, var2: i32, var3: bool) -> i32;

    // java  KParagraphFormat getParagraphFormat()
    fn getParagraphFormat(&self) -> &KParagraphFormat;

    // java  KFont getFont()
    fn getFont(&self) -> &KFont;

    // java  void insertText(String var1)
    fn insertText__string(&mut self, var1: &str);

    // java  void insertText(String var1, PropertySet var2)
    fn insertText__string_propertyset(&mut self, var1: &str, var2: &PropertySet);

    // java  void insertText(StringBuilder var1)
    fn insertText__stringbuilder(&mut self, var1: &str);

    // java  void insertText(StringBuilder var1, PropertySet var2)
    fn insertText__stringbuilder_propertyset(&mut self, var1: &str, var2: &PropertySet);

    // java  void insertText(char[] var1)
    fn insertText__chararr(&mut self, var1: &Vec<char>);

    // java  void insertText(char[] var1, PropertySet var2)
    fn insertText__chararr_propertyset(&mut self, var1: &Vec<char>, var2: &PropertySet);

    // java  void replaceText(String var1, PropertySet var2)
    fn replaceText__string_propertyset(&mut self, var1: &str, var2: &PropertySet);

    // java  void replaceText(String var1)
    fn replaceText__string(&mut self, var1: &str);

    // java  void replaceText(StringBuilder var1)
    fn replaceText__stringbuilder(&mut self, var1: &str);

    // java  void replaceText(StringBuilder var1, PropertySet var2)
    fn replaceText__stringbuilder_propertyset(&mut self, var1: &str, var2: &PropertySet);

    // java  void replaceText(char[] var1)
    fn replaceText__chararr(&mut self, var1: &Vec<char>);

    // java  void replaceText(char[] var1, PropertySet var2)
    fn replaceText__chararr_propertyset(&mut self, var1: &Vec<char>, var2: &PropertySet);

    // java  void insertParagraph()
    fn insertParagraph(&mut self);

    // java  int computeStatistics(WtStatistic var1)
    fn computeStatistics__wtstatistic(&mut self, var1: &WtStatistic) -> i32;

    // java  void computeStatistics(WtStatistic[] var1, int[] var2)
    fn computeStatistics__wtstatisticarr_intarr(&mut self, var1: &Vec<WtStatistic>, var2: &Vec<i32>);

    // java  KComments getComments()
    fn getComments(&self) -> &KComments;

    // java  KReplyComments getReplyComments()
    fn getReplyComments(&self) -> &KReplyComments;

    // java  Range getParagraphRange()
    fn getParagraphRange(&self) -> &(dyn Range);

    // java  long getParagraphRANGE()
    fn getParagraphRANGE(&self) -> i64;

    // java  void copy()
    fn copy(&mut self);

    // java  void cut()
    fn cut(&mut self);

    // java  boolean paste(String var1, boolean var2)
    fn paste(&mut self, var1: &str, var2: bool) -> bool;

    // java  void ltrPara()
    fn ltrPara(&mut self);

    // java  void rtlPara()
    fn rtlPara(&mut self);

    // java  int deleteRangeDirectly()
    fn deleteRangeDirectly(&mut self) -> i32;

    // java  int getLength()
    fn getLength(&self) -> i32;

}
