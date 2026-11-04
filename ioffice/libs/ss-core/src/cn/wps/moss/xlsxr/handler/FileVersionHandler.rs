#![allow(non_snake_case)]
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use crate::cn::wps::moss::app::FileVersion::FileVersion;

// Java: class FileVersionHandler extends DefaultElementHandler
pub struct FileVersionHandler { mFileVersion: Rc<RefCell<FileVersion>> }

impl FileVersionHandler {
    // Java: public FileVersionHandler(FileVersion fileVersion)
    pub fn FileVersionHandler(fileVersion: Rc<RefCell<FileVersion>>) -> Self {
        Self { mFileVersion: fileVersion }
    }
}

impl ElementHandler for FileVersionHandler {
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: attrAt = attrsNotCopy.getByName(5271); ... setAppName(appName);
        if let Some(attr) = attrsNotCopy.get_by_name(5271) { self.mFileVersion.borrow_mut().setAppName(attr.to_string()); }
        // Java: attrAt = attrsNotCopy.getByName(5272); ... setLastEdited(lastEdited);
        if let Some(attr) = attrsNotCopy.get_by_name(5272) { self.mFileVersion.borrow_mut().setLastEdited(attr.to_string()); }
        // Java: attrAt = attrsNotCopy.getByName(5273); ... setLowestEdited(lowestEdited);
        if let Some(attr) = attrsNotCopy.get_by_name(5273) { self.mFileVersion.borrow_mut().setLowestEdited(attr.to_string()); }
        // Java: attrAt = attrsNotCopy.getByName(5274); ... setRupBuild(rupBuild);
        if let Some(attr) = attrsNotCopy.get_by_name(5274) { self.mFileVersion.borrow_mut().setRupBuild(attr.to_string()); }
        // Java: attrAt = attrsNotCopy.getByName(4838); ... setCodeName(codeName);
        if let Some(attr) = attrsNotCopy.get_by_name(4838) { self.mFileVersion.borrow_mut().setCodeName(attr.to_string()); }
    }
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    fn enterSubElement(&self, _: i32) -> Option<Box<dyn ElementHandler>> { None }
}
