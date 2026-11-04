// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import cn.wps.moffice.writer.io.reader.html.HtmlPasteBuriedPointModel
use crate::cn::wps::moffice::writer::io::reader::html::HtmlPasteBuriedPointModel::*;

// java public interface IHtmlReader
pub trait IHtmlReader {
    // java  int read()
    fn read(&mut self) -> i32;

    // java  HtmlPasteBuriedPointModel getHtmlPasteBuriedPointModel()
    fn getHtmlPasteBuriedPointModel(&self) -> &HtmlPasteBuriedPointModel;

}
