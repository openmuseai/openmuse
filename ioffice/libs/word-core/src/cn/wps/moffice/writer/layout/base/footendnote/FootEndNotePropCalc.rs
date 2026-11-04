// Java: cn.wps.moffice.writer.layout.base.footendnote.FootEndNotePropCalc
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::data::PLCSection::SectionNode;
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
use stubs::java::lang::Object::Object;
use crate::cn::wps::moffice::writer::layout::base::env::FootEndNoteEnv::FootEndNoteEnv;
use util::moffice::property::PropertySet::PropertySet;

// java public class FootEndNotePropCalc
pub struct FootEndNotePropCalc;

impl FootEndNotePropCalc {
    // java public static void calc(FootEndNoteEnv footEndNoteEnv, PLCSection.SectionNode sectionNode, PropertySet secProp)
    pub fn calc(foot_end_note_env: &mut FootEndNoteEnv, section_node: Option<SectionNode>, sec_prop: &PropertySet) {
        // java footEndNoteEnv.endNoteRefMsoNfc = secProp.getInt(689, 2);
        foot_end_note_env.endNoteRefMsoNfc = sec_prop.getInt__int_int(689, 2);
        // java footEndNoteEnv.footNoteRefMsoNfc = secProp.getInt(688, 0);
        foot_end_note_env.footNoteRefMsoNfc = sec_prop.getInt__int_int(688, 0);
        // java footEndNoteEnv.footNoteStart = secProp.getInt(722, 1);
        foot_end_note_env.footNoteStart = sec_prop.getInt__int_int(722, 1);
        // java footEndNoteEnv.endNoteStart = secProp.getInt(723, 1);
        foot_end_note_env.endNoteStart = sec_prop.getInt__int_int(723, 1);
        // java footEndNoteEnv.footNoteRestart = secProp.getInt(724, 0);
        foot_end_note_env.footNoteRestart = sec_prop.getInt__int_int(724, 0);
        // java footEndNoteEnv.endNoteRestart = secProp.getInt(725, 0);
        foot_end_note_env.endNoteRestart = sec_prop.getInt__int_int(725, 0);
        // java footEndNoteEnv.footNotePosition = secProp.getInt(726, 1);
        foot_end_note_env.footNotePosition = sec_prop.getInt__int_int(726, 1);
        // java footEndNoteEnv.isEndNoteSuppressed = isEndNoteSuppressed(sectionNode);
        foot_end_note_env.isEndNoteSuppressed = Self::isEndNoteSuppressed__sectionnode(section_node);
        // java footEndNoteEnv.curSectionNode = sectionNode;
        foot_end_note_env.curSectionNode = section_node;
        // java footEndNoteEnv.hasInitSecProp = true;
        foot_end_note_env.hasInitSecProp = true;
    }

    // java private static boolean isEndNoteSuppressed(PropertySet prop)
    pub fn isEndNoteSuppressed__propertyset(prop: &PropertySet) -> bool {
        // java return !prop.getBoolean(704, true);
        !prop.getBoolean__int_boolean(704, true)
    }

    // java public static boolean isEndNoteSuppressed(PLCSection.SectionNode section)
    pub fn isEndNoteSuppressed__sectionnode(section: Option<SectionNode>) -> bool {
        // java if (null == section) return false;
        let Some(section) = section.filter(|node| !node.ptr.is_null()) else { return false; };
        // java PropertySet prop = section.sepx;
        let prop = section.getPropertySet();
        // java else if (isEndNoteSuppressed(prop)) return true;
        if Self::isEndNoteSuppressed__propertyset(&prop) {
            return true;
        }
        // java Revision revision = (Revision)prop.getObject(727, null);
        let Some(revision) = prop.getValue(727).and_then(|value| value.as_ref().as_any().downcast_ref::<Revision>().cloned()) else {
            return false;
        };
        // java return null != revision && isEndNoteSuppressed(revision.getProperty());
        revision.getProperty().as_ref().map(|p| Self::isEndNoteSuppressed__propertyset(p)).unwrap_or(false)
    }

    // java public static boolean shouldShowEndNotesAtTheEndOfSection(PropertySet docProp)
    pub fn shouldShowEndNotesAtTheEndOfSection(doc_prop: &PropertySet) -> bool {
        // java int epc = docProp.getInt(641, 3);
        let epc = doc_prop.getInt__int_int(641, 3);
        // java return 0 == epc;
        0 == epc
    }
}
