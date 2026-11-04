// Java: import cn.wps.io.uof.reader.UofrPackage
use crate::cn::wps::io::uof::reader::UofrPackage::UofrPackage;
// Java: import cn.wps.io.uof.reader.importer.uos.IHyperLinkImporter
use crate::cn::wps::io::uof::reader::importer::uos::IHyperLinkImporter::IHyperLinkImporter;
// Java: import cn.wps.io.uof.reader.importer.uos.INamesImporter
use crate::cn::wps::io::uof::reader::importer::uos::INamesImporter::INamesImporter;
// Java: import cn.wps.io.uof.reader.importer.uos.IRulesImporter
use crate::cn::wps::io::uof::reader::importer::uos::IRulesImporter::IRulesImporter;
// Java: import cn.wps.io.uof.reader.importer.uos.IStyleImporter
use crate::cn::wps::io::uof::reader::importer::uos::IStyleImporter::IStyleImporter;
// Java: import cn.wps.io.uof.reader.importer.uos.IWorkBookImporter
use crate::cn::wps::io::uof::reader::importer::uos::IWorkBookImporter::IWorkBookImporter;
// Java: import cn.wps.io.uof.reader.importer.uos.IWorkSheetImporter
use crate::cn::wps::io::uof::reader::importer::uos::IWorkSheetImporter::IWorkSheetImporter;

use super::UosContext::UosContext;

// Java: public interface IUosImporter extends IUofImporter
pub trait IUosImporter {
    // Java: abstract UosContext getContext()
    fn getContext(&self) -> &UosContext;

    // Java: abstract UofrPackage getPackage()
    fn getPackage(&self) -> &UofrPackage;

    // Java: abstract IWorkBookImporter getWorkBookImporter()
    fn getWorkBookImporter(&self) -> Box<dyn IWorkBookImporter>;

    // Java: abstract IWorkSheetImporter getWorkSheetImporter()
    fn getWorkSheetImporter(&self) -> Box<dyn IWorkSheetImporter>;

    // Java: abstract IStyleImporter getStyleImporter()
    fn getStyleImporter(&self) -> Box<dyn IStyleImporter>;

    // Java: abstract IHyperLinkImporter getHyperLinkImporter()
    fn getHyperLinkImporter(&self) -> Box<dyn IHyperLinkImporter>;

    // Java: abstract INamesImporter getNamesImporter()
    fn getNamesImporter(&self) -> Box<dyn INamesImporter>;

    // Java: abstract IRulesImporter getRulesImporter()
    fn getRulesImporter(&self) -> Box<dyn IRulesImporter>;

}
