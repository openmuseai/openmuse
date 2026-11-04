// java public class SectionTool
use crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::data::DefaultSectionProperties::DefaultSectionProperties;
use crate::cn::wps::moffice::writer::data::PLCSection::SectionNode;
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use util::moffice::property::PropertySet::PropertySet;
use util::moffice::util::UnitUtil::UnitUtil;

pub struct SectionTool;

impl SectionTool {
    // java public static void calcGrid(PageEnv pageEnv, PropertySet mSecPropSet, TextDocument textDocument)
    pub fn calcGrid(pageEnv: &mut PageEnv, mSecPropSet: &PropertySet, textDocument: Option<&TextDocument>) {
        if std::env::var_os("WORD_PARITY_GRID_TRACE").is_some() {
            eprintln!("grid calcGrid has679={} prop679={} normal_pt={}",
                mSecPropSet.contains(679), mSecPropSet.getFloat(679), Self::normal_font_size_pt(textDocument));
        }
        // java pageEnv.gridModel = mSecPropSet.getInt(677, 0);
        pageEnv.gridModel = mSecPropSet.getInt__int_int(677, 0);
        // java pageEnv.gridHeight = mSecPropSet.getInt(678, 312);
        pageEnv.gridHeight = mSecPropSet.getInt__int_int(678, 312);
        // java if (mSecPropSet.contains(679))
        if mSecPropSet.contains(679) {
            // java pageEnv.gridWidth = UnitUtil.PT2EMU(mSecPropSet.getFloat(679));
            pageEnv.gridWidth = UnitUtil::PT2EMU(mSecPropSet.getFloat(679));
            // java KStyle normalStyle = textDocument.getStyles().getStyleById(0);
            // java PropertySet normalProperty = normalStyle.getFinalProperty();
            // java int fontSize = UnitUtil.PT2TWIPS(normalProperty.getFloat(10, 10.0F));
            let fontSize: i32 = Self::normal_font_size_twips(textDocument);
            // java pageEnv.charSpaceForGrid = UnitUtil.EMU2TWIPS(pageEnv.gridWidth) - (float)fontSize;
            pageEnv.charSpaceForGrid = UnitUtil::EMU2TWIPS(pageEnv.gridWidth) - fontSize as f32;
        } else {
            // java KStyle normalStyle = textDocument.getStyles().getStyleById(0);
            // java PropertySet normalProperty = normalStyle.getFinalProperty();
            // java pageEnv.gridWidth = UnitUtil.PT2EMU(normalProperty.getFloat(10, 10.0F));
            let font_pt: f32 = Self::normal_font_size_pt(textDocument);
            pageEnv.gridWidth = UnitUtil::PT2EMU(font_pt);
            // java pageEnv.charSpaceForGrid = 0.0F;
            pageEnv.charSpaceForGrid = 0.0;
        }
    }

    fn normal_font_size_pt(textDocument: Option<&TextDocument>) -> f32 {
        // Java: KStyle normalStyle = textDocument.getStyles().getStyleById(0);
        // Java: PropertySet normalProperty = normalStyle.getFinalProperty();
        // Java: normalProperty.getFloat(10, 10.0F).
        let styles = textDocument.and_then(TextDocument::getStyles);
        let style = styles.and_then(|styles| styles.getStyleById__int(0));
        let prop = style.and_then(|mut style| style.getFinalProperty());
        let value = prop.as_ref().map(|prop| prop.getFloat__int_float(10, 10.0)).unwrap_or(10.0);
        if std::env::var_os("WORD_PARITY_GRID_TRACE").is_some() {
            eprintln!("grid normal_style has_doc={} has_styles={} has_style0={} has_final={} contains10={} pt={}",
                textDocument.is_some(), styles.is_some(),
                styles.is_some_and(|s| s.getStyleById__int(0).is_some()),
                prop.is_some(), prop.as_ref().is_some_and(|p| p.contains(10)), value);
        }
        value
    }

    fn normal_font_size_twips(textDocument: Option<&TextDocument>) -> i32 {
        UnitUtil::PT2TWIPS(Self::normal_font_size_pt(textDocument))
    }

    // java public static PropertySet calcSectionPropertySet(ViewEnv viewEnv, PLCSection.SectionNode sectionNode)
    pub fn calcSectionPropertySet(view_env: &ViewEnv, section_node: Option<SectionNode>) -> PropertySet {
        let Some(section_node) = section_node.filter(|node| !node.ptr.is_null()) else {
            return DefaultSectionProperties::getProperties();
        };
        let prop = section_node.getPropertySet();
        if !view_env.isRevisionFinal() {
            return prop;
        }
        let revision = prop.getValue(727).and_then(|value| value.as_ref().as_any().downcast_ref::<Revision>().cloned());
        match revision.as_ref().and_then(Revision::getProperty) {
            Some(revision_prop) => PropertySet::merge(&prop, &revision_prop),
            None => prop,
        }
    }

    // java public static boolean checkPageMirrorMargins(boolean isCurPortrait, DocumentEnv docEnv, MainState mainState)
    pub fn checkPageMirrorMargins(is_cur_portrait: bool, doc_env: &DocumentEnv, main: &mut MainState) -> bool {
        if !doc_env.mirrorMargins {
            return false;
        }
        if main.curPageIndex == 0 {
            main.pageOrientationChanged = false;
            return false;
        }
        if main.pageOrientationChanged {
            return false;
        }
        let page_list = main.snapshot.getLayoutPageList();
        let first_page = TypoList::first__int_typosnapshot(page_list, &mut main.snapshot);
        if first_page != 0 {
            let portrait = TypoLayoutPage::height(first_page, &main.snapshot) >= TypoLayoutPage::width__int_typosnapshot(first_page, &main.snapshot);
            if portrait != is_cur_portrait {
                main.pageOrientationChanged = true;
                return false;
            }
        }
        main.curPageIndex % 2 == 1
    }
}
