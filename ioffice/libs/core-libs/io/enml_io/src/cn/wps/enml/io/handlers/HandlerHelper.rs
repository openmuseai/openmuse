#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.enml.io.IENMLDocumentImporter
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
// Java: import cn.wps.enml.io.handlers.block.BlockQuoteHandler
use crate::cn::wps::enml::io::handlers::block::BlockQuoteHandler::BlockQuoteHandler;
// Java: import cn.wps.enml.io.handlers.block.DivHandler
use crate::cn::wps::enml::io::handlers::block::DivHandler::DivHandler;
// Java: import cn.wps.enml.io.handlers.block.HrHandler
use crate::cn::wps::enml::io::handlers::block::HrHandler::HrHandler;
// Java: import cn.wps.enml.io.handlers.block.LiHandler
use crate::cn::wps::enml::io::handlers::block::LiHandler::LiHandler;
// Java: import cn.wps.enml.io.handlers.block.OlHandler
use crate::cn::wps::enml::io::handlers::block::OlHandler::OlHandler;
// Java: import cn.wps.enml.io.handlers.block.PHandler
use crate::cn::wps::enml::io::handlers::block::PHandler::PHandler;
// Java: import cn.wps.enml.io.handlers.block.TableHandler
use crate::cn::wps::enml::io::handlers::block::TableHandler::TableHandler;
// Java: import cn.wps.enml.io.handlers.block.UlHandler
use crate::cn::wps::enml::io::handlers::block::UlHandler::UlHandler;
// Java: import cn.wps.enml.io.handlers.block.heading.H1Handler
use crate::cn::wps::enml::io::handlers::block::heading::H1Handler::H1Handler;
// Java: import cn.wps.enml.io.handlers.block.heading.H2Handler
use crate::cn::wps::enml::io::handlers::block::heading::H2Handler::H2Handler;
// Java: import cn.wps.enml.io.handlers.block.heading.H3Handler
use crate::cn::wps::enml::io::handlers::block::heading::H3Handler::H3Handler;
// Java: import cn.wps.enml.io.handlers.block.heading.H4Handler
use crate::cn::wps::enml::io::handlers::block::heading::H4Handler::H4Handler;
// Java: import cn.wps.enml.io.handlers.block.heading.H5Handler
use crate::cn::wps::enml::io::handlers::block::heading::H5Handler::H5Handler;
// Java: import cn.wps.enml.io.handlers.block.heading.H6Handler
use crate::cn::wps::enml::io::handlers::block::heading::H6Handler::H6Handler;
// Java: import cn.wps.enml.io.handlers.inline.evernote.EnMediaHandler
use crate::cn::wps::enml::io::handlers::inline::evernote::EnMediaHandler::EnMediaHandler;
// Java: import cn.wps.enml.io.handlers.inline.fontstyle.BHandler
use crate::cn::wps::enml::io::handlers::inline::fontstyle::BHandler::BHandler;
// Java: import cn.wps.enml.io.handlers.inline.fontstyle.BigHandler
use crate::cn::wps::enml::io::handlers::inline::fontstyle::BigHandler::BigHandler;
// Java: import cn.wps.enml.io.handlers.inline.fontstyle.IHandler
use crate::cn::wps::enml::io::handlers::inline::fontstyle::IHandler::IHandler;
// Java: import cn.wps.enml.io.handlers.inline.fontstyle.SHandler
use crate::cn::wps::enml::io::handlers::inline::fontstyle::SHandler::SHandler;
// Java: import cn.wps.enml.io.handlers.inline.fontstyle.SmallHandler
use crate::cn::wps::enml::io::handlers::inline::fontstyle::SmallHandler::SmallHandler;
// Java: import cn.wps.enml.io.handlers.inline.fontstyle.StrikeHandler
use crate::cn::wps::enml::io::handlers::inline::fontstyle::StrikeHandler::StrikeHandler;
// Java: import cn.wps.enml.io.handlers.inline.fontstyle.TTHandler
use crate::cn::wps::enml::io::handlers::inline::fontstyle::TTHandler::TTHandler;
// Java: import cn.wps.enml.io.handlers.inline.fontstyle.UHandler
use crate::cn::wps::enml::io::handlers::inline::fontstyle::UHandler::UHandler;
// Java: import cn.wps.enml.io.handlers.inline.phrase.AbbrHandler
use crate::cn::wps::enml::io::handlers::inline::phrase::AbbrHandler::AbbrHandler;
// Java: import cn.wps.enml.io.handlers.inline.phrase.AcronymHandler
use crate::cn::wps::enml::io::handlers::inline::phrase::AcronymHandler::AcronymHandler;
// Java: import cn.wps.enml.io.handlers.inline.phrase.CiteHandler
use crate::cn::wps::enml::io::handlers::inline::phrase::CiteHandler::CiteHandler;
// Java: import cn.wps.enml.io.handlers.inline.phrase.CodeHandler
use crate::cn::wps::enml::io::handlers::inline::phrase::CodeHandler::CodeHandler;
// Java: import cn.wps.enml.io.handlers.inline.phrase.DfnHandler
use crate::cn::wps::enml::io::handlers::inline::phrase::DfnHandler::DfnHandler;
// Java: import cn.wps.enml.io.handlers.inline.phrase.EmHandler
use crate::cn::wps::enml::io::handlers::inline::phrase::EmHandler::EmHandler;
// Java: import cn.wps.enml.io.handlers.inline.phrase.KbdHandler
use crate::cn::wps::enml::io::handlers::inline::phrase::KbdHandler::KbdHandler;
// Java: import cn.wps.enml.io.handlers.inline.phrase.SampHandler
use crate::cn::wps::enml::io::handlers::inline::phrase::SampHandler::SampHandler;
// Java: import cn.wps.enml.io.handlers.inline.phrase.StrongHandler
use crate::cn::wps::enml::io::handlers::inline::phrase::StrongHandler::StrongHandler;
// Java: import cn.wps.enml.io.handlers.inline.phrase.VarHandler
use crate::cn::wps::enml::io::handlers::inline::phrase::VarHandler::VarHandler;
// Java: import cn.wps.enml.io.handlers.inline.special.AHandler
use crate::cn::wps::enml::io::handlers::inline::special::AHandler::AHandler;
// Java: import cn.wps.enml.io.handlers.inline.special.BrHandler
use crate::cn::wps::enml::io::handlers::inline::special::BrHandler::BrHandler;
// Java: import cn.wps.enml.io.handlers.inline.special.FontHandler
use crate::cn::wps::enml::io::handlers::inline::special::FontHandler::FontHandler;
// Java: import cn.wps.enml.io.handlers.inline.special.QHandler
use crate::cn::wps::enml::io::handlers::inline::special::QHandler::QHandler;
// Java: import cn.wps.enml.io.handlers.inline.special.SpanHandler
use crate::cn::wps::enml::io::handlers::inline::special::SpanHandler::SpanHandler;
// Java: import cn.wps.enml.io.handlers.inline.special.SubHandler
use crate::cn::wps::enml::io::handlers::inline::special::SubHandler::SubHandler;
// Java: import cn.wps.enml.io.handlers.inline.special.SupHandler
use crate::cn::wps::enml::io::handlers::inline::special::SupHandler::SupHandler;
// Java: import org.dom4j.ElementPath
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::ElementPath;
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;

// Java: public class HandlerHelper
pub struct HandlerHelper {
}

// Java: public class HandlerHelper
impl HandlerHelper {
    // Java: HandlerHelper()
    pub fn new() -> Self {
      // Java: super();
      HandlerHelper {}
    }

    // Java: static void registerBlockLevelElements(ElementPath elementPath, HandlerPool pool, IENMLDocumentImporter importer, int tableLayer)
    pub fn registerBlockLevelElements(elementPath: &mut dyn ElementPath, pool: &mut HandlerPool, importer: &dyn IENMLDocumentImporter, tableLayer: i32) {
      // Java: Assert.assertNotNull("elementPath should not be null", elementPath);
      // Java: Assert.assertNotNull("pool should not be null", pool);
      // Java: Assert.assertNotNull("importer should not be null", importer);
      // Java: Assert.assertNotNull("tableLayer >= 0 should be true", tableLayer >= 0);
      assert!(tableLayer >= 0, "tableLayer >= 0 should be true");
      // Java: DivHandler divHandler = (DivHandler)pool.getHandler("div", DivHandler.class, pool, importer);
      // Java: divHandler.setTableLayer(tableLayer);
      // Java: elementPath.addHandler(divHandler.filterName(), divHandler);
      // Java: PHandler pHandler = (PHandler)pool.getHandler("p", PHandler.class, pool, importer);
      // Java: pHandler.setTableLayer(tableLayer);
      // Java: elementPath.addHandler(pHandler.filterName(), pHandler);
      // Java: BlockQuoteHandler blockQuoteHandler = (BlockQuoteHandler)pool.getHandler("blockquote", BlockQuoteHandler.class, pool, importer);
      // Java: blockQuoteHandler.setTableLayer(tableLayer);
      // Java: elementPath.addHandler(blockQuoteHandler.filterName(), blockQuoteHandler);
      // Java: H1Handler h1Handler = (H1Handler)pool.getHandler("h1", H1Handler.class, pool, importer);
      // Java: elementPath.addHandler(h1Handler.filterName(), h1Handler);
      // Java: H2Handler h2Handler = (H2Handler)pool.getHandler("h2", H2Handler.class, pool, importer);
      // Java: elementPath.addHandler(h2Handler.filterName(), h2Handler);
      // Java: H3Handler h3Handler = (H3Handler)pool.getHandler("h3", H3Handler.class, pool, importer);
      // Java: elementPath.addHandler(h3Handler.filterName(), h3Handler);
      // Java: H4Handler h4Handler = (H4Handler)pool.getHandler("h4", H4Handler.class, pool, importer);
      // Java: elementPath.addHandler(h4Handler.filterName(), h4Handler);
      // Java: H5Handler h5Handler = (H5Handler)pool.getHandler("h5", H5Handler.class, pool, importer);
      // Java: elementPath.addHandler(h5Handler.filterName(), h5Handler);
      // Java: H6Handler h6Handler = (H6Handler)pool.getHandler("h6", H6Handler.class, pool, importer);
      // Java: elementPath.addHandler(h6Handler.filterName(), h6Handler);
      // Java: UlHandler ulHandler = (UlHandler)pool.getHandler("ul", UlHandler.class, pool, importer);
      // Java: ulHandler.setTableLayer(tableLayer);
      // Java: elementPath.addHandler(ulHandler.filterName(), ulHandler);
      // Java: OlHandler olHandler = (OlHandler)pool.getHandler("ol", OlHandler.class, pool, importer);
      // Java: olHandler.setTableLayer(tableLayer);
      // Java: elementPath.addHandler(olHandler.filterName(), olHandler);
      // Java: LiHandler liHandler = (LiHandler)pool.getHandler("li", LiHandler.class, pool, importer);
      // Java: liHandler.setTableLayer(tableLayer);
      // Java: elementPath.addHandler(liHandler.filterName(), liHandler);
      // Java: HrHandler hrHandler = (HrHandler)pool.getHandler("hr", HrHandler.class, pool, importer);
      // Java: hrHandler.setTableLayer(tableLayer);
      // Java: elementPath.addHandler(hrHandler.filterName(), hrHandler);
      // Java: TableHandler tableHandler = (TableHandler)pool.getHandler("table", TableHandler.class, pool, importer);
      // Java: tableHandler.setTableLayer(tableLayer + 1);
      // Java: elementPath.addHandler(tableHandler.filterName(), tableHandler);
      todo!("HandlerPool.getHandler requires reflection-based factory pattern")
    }

    // Java: static void registerInlinelElements(ElementPath elementPath, HandlerPool pool, IENMLDocumentImporter importer)
    pub fn registerInlinelElements(elementPath: &mut dyn ElementPath, pool: &mut HandlerPool, importer: &dyn IENMLDocumentImporter) {
      // Java: Assert.assertNotNull("elementPath should not be null", elementPath);
      // Java: Assert.assertNotNull("pool should not be null", pool);
      // Java: Assert.assertNotNull("importer should not be null", importer);
      // Java: TTHandler ttHandler = (TTHandler)pool.getHandler("tt", TTHandler.class, pool, importer);
      // Java: elementPath.addHandler(ttHandler.filterName(), ttHandler);
      // Java: BHandler bHandler = (BHandler)pool.getHandler("b", BHandler.class, pool, importer);
      // Java: elementPath.addHandler(bHandler.filterName(), bHandler);
      // Java: IHandler iHandler = (IHandler)pool.getHandler("i", IHandler.class, pool, importer);
      // Java: elementPath.addHandler(iHandler.filterName(), iHandler);
      // Java: BigHandler bigHandler = (BigHandler)pool.getHandler("big", BigHandler.class, pool, importer);
      // Java: elementPath.addHandler(bigHandler.filterName(), bigHandler);
      // Java: SmallHandler smallHandler = (SmallHandler)pool.getHandler("small", SmallHandler.class, pool, importer);
      // Java: elementPath.addHandler(smallHandler.filterName(), smallHandler);
      // Java: UHandler uHandler = (UHandler)pool.getHandler("u", UHandler.class, pool, importer);
      // Java: elementPath.addHandler(uHandler.filterName(), uHandler);
      // Java: StrikeHandler strikeHandler = (StrikeHandler)pool.getHandler("strike", StrikeHandler.class, pool, importer);
      // Java: elementPath.addHandler(strikeHandler.filterName(), strikeHandler);
      // Java: SHandler sHandler = (SHandler)pool.getHandler("s", SHandler.class, pool, importer);
      // Java: elementPath.addHandler(sHandler.filterName(), sHandler);
      // Java: AbbrHandler abbrHandler = (AbbrHandler)pool.getHandler("abbr", AbbrHandler.class, pool, importer);
      // Java: elementPath.addHandler(abbrHandler.filterName(), abbrHandler);
      // Java: AcronymHandler acronymHandler = (AcronymHandler)pool.getHandler("acronym", AcronymHandler.class, pool, importer);
      // Java: elementPath.addHandler(acronymHandler.filterName(), acronymHandler);
      // Java: CiteHandler citeHandler = (CiteHandler)pool.getHandler("cite", CiteHandler.class, pool, importer);
      // Java: elementPath.addHandler(citeHandler.filterName(), citeHandler);
      // Java: CodeHandler codeHandler = (CodeHandler)pool.getHandler("code", CodeHandler.class, pool, importer);
      // Java: elementPath.addHandler(codeHandler.filterName(), codeHandler);
      // Java: DfnHandler dfnHandler = (DfnHandler)pool.getHandler("dfn", DfnHandler.class, pool, importer);
      // Java: elementPath.addHandler(dfnHandler.filterName(), dfnHandler);
      // Java: EmHandler emHandler = (EmHandler)pool.getHandler("em", EmHandler.class, pool, importer);
      // Java: elementPath.addHandler(emHandler.filterName(), emHandler);
      // Java: KbdHandler kbdHandler = (KbdHandler)pool.getHandler("kbd", KbdHandler.class, pool, importer);
      // Java: elementPath.addHandler(kbdHandler.filterName(), kbdHandler);
      // Java: SampHandler sampHandler = (SampHandler)pool.getHandler("samp", SampHandler.class, pool, importer);
      // Java: elementPath.addHandler(sampHandler.filterName(), sampHandler);
      // Java: StrongHandler strongHandler = (StrongHandler)pool.getHandler("strong", StrongHandler.class, pool, importer);
      // Java: elementPath.addHandler(strongHandler.filterName(), strongHandler);
      // Java: VarHandler varHandler = (VarHandler)pool.getHandler("var", VarHandler.class, pool, importer);
      // Java: elementPath.addHandler(varHandler.filterName(), varHandler);
      // Java: AHandler aHandler = (AHandler)pool.getHandler("a", AHandler.class, pool, importer);
      // Java: elementPath.addHandler(aHandler.filterName(), aHandler);
      // Java: FontHandler fontHandler = (FontHandler)pool.getHandler("font", FontHandler.class, pool, importer);
      // Java: elementPath.addHandler(fontHandler.filterName(), fontHandler);
      // Java: BrHandler brHandler = (BrHandler)pool.getHandler("br", BrHandler.class, pool, importer);
      // Java: elementPath.addHandler(brHandler.filterName(), brHandler);
      // Java: QHandler qHandler = (QHandler)pool.getHandler("q", QHandler.class, pool, importer);
      // Java: elementPath.addHandler(qHandler.filterName(), qHandler);
      // Java: SpanHandler spanHandler = (SpanHandler)pool.getHandler("span", SpanHandler.class, pool, importer);
      // Java: elementPath.addHandler(spanHandler.filterName(), spanHandler);
      // Java: SubHandler subHandler = (SubHandler)pool.getHandler("sub", SubHandler.class, pool, importer);
      // Java: elementPath.addHandler(subHandler.filterName(), subHandler);
      // Java: SupHandler supHandler = (SupHandler)pool.getHandler("sup", SupHandler.class, pool, importer);
      // Java: elementPath.addHandler(supHandler.filterName(), supHandler);
      // Java: EnMediaHandler enMediaHandler = (EnMediaHandler)pool.getHandler("en-media", EnMediaHandler.class, pool, importer);
      // Java: elementPath.addHandler(enMediaHandler.filterName(), enMediaHandler);
      todo!("HandlerPool.getHandler requires reflection-based factory pattern")
    }
}
