#![allow(non_snake_case)]

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::KSAssert;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{PropertySet, PropertyValue};
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{self, Entry as ChpxEntry};
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
use crate::cn::wps::moffice::writer::core::Document::{Document, SoftTextDocument};
use crate::cn::wps::moffice::writer::core::KRevision::KRevision;
use std::sync::Arc;

// java public class KParaRunRevision extends KRevision
pub struct KParaRunRevision {
    // java KRevision super_
    pub(crate) super_: KRevision,
    // java private Document mDocument = null;
    mDocument: *const (dyn Document + 'static),
    // java private TextDocument mTextDocument = null;
    mTextDocument: *const (dyn SoftTextDocument + 'static),
}

// java public class KParaRunRevision extends KRevision
impl KParaRunRevision {
    // java public KParaRunRevision(Document document)
    pub fn KParaRunRevision(document: &dyn Document) -> Self {
        // java super(document.getTextDocument());
        let text_document = document.getTextDocument().expect("KParaRunRevision: null TextDocument");
        let super_ = KRevision::KRevision(text_document);
        // java Assert.assertNotNull("document should not be null", document);
        Assert::assertNotNull_msg("document should not be null", Some(document as *const dyn Document as *const ()));
        // java this.mDocument = document;
        // java this.mTextDocument = this.mDocument.getTextDocument();
        Self {
            super_,
            mDocument: unsafe { std::mem::transmute::<*const dyn Document, *const (dyn Document + 'static)>(document as *const dyn Document) },
            mTextDocument: unsafe { std::mem::transmute::<*const dyn SoftTextDocument, *const (dyn SoftTextDocument + 'static)>(text_document as *const dyn SoftTextDocument) },
        }
    }

    pub fn new(document: &dyn Document) -> Self { Self::KParaRunRevision(document) }

    fn document(&self) -> &dyn Document { unsafe { &*self.mDocument } }
    fn textDocument(&self) -> &dyn SoftTextDocument { unsafe { &*self.mTextDocument } }

    fn revision(prop: &PropertySet, id: i32) -> Option<Revision> {
        prop.getValue(id).map(|v| v.as_any().downcast_ref::<Revision>().expect("ClassCastException: Revision").clone())
    }
    fn integer(prop: &PropertySet, id: i32) -> Option<i32> {
        prop.getValue(id).map(|v| *v.as_any().downcast_ref::<i32>().expect("ClassCastException: Integer"))
    }
    fn boolean(prop: &PropertySet, id: i32) -> Option<bool> {
        prop.getValue(id).map(|v| *v.as_any().downcast_ref::<bool>().expect("ClassCastException: Boolean"))
    }
    fn set_revision(prop: &mut MutablePropertySet, id: i32, revision: Revision) {
        prop.setValue(id, Some(Arc::new(revision) as PropertyValue));
    }

    // java public static PropertySet getInsertDeleteRevision(PropertySet prop)
    pub fn getInsertDeleteRevision(prop: &PropertySet) -> PropertySet {
        // java MutablePropertySet propSet = new MutablePropertySet();
        let mut propSet = MutablePropertySet::new();
        // java getInsertRevision(prop, propSet);
        Self::getInsertRevision(prop, &mut propSet);
        // java getDeleteRevision(prop, propSet);
        Self::getDeleteRevision(prop, &mut propSet);
        // java return propSet.freeze();
        propSet.freeze()
    }

    // java private PropertySet getNewRunFormatRevision()
    fn getNewRunFormatRevision(&self) -> PropertySet {
        // java Revision revision = new Revision(3);
        let mut revision = Revision::Revision__int(3);
        // java this.setRevision(revision);
        self.super_.setRevision(&mut revision);
        // java MutablePropertySet propSet = new MutablePropertySet();
        let mut propSet = MutablePropertySet::new();
        // java int rsid = this.assignRsid();
        let rsid = self.super_.assignRsid();
        // java propSet.setValue(48, revision);
        Self::set_revision(&mut propSet, 48, revision);
        // java propSet.setInt(51, rsid);
        propSet.setInt(51, rsid);
        // java propSet.setBoolean(47, true);
        propSet.setBoolean(47, true);
        // java return propSet.freeze();
        propSet.freeze()
    }

    // java public PropertySet setRunFormatRevision(PropertySet prop)
    pub fn setRunFormatRevision(&self, prop: &PropertySet) -> PropertySet {
        // java Assert.assertNotNull("prop should not be null", prop);
        Assert::assertNotNull_msg("prop should not be null", Some(prop));
        // java Assert.assertTrue("the document revision mode should be true!", this.mTextDocument.getTrackChange());
        Assert::assertTrue_msg("the document revision mode should be true!", self.textDocument().getTrackChange());
        // java Revision revision = (Revision)prop.getValue(48);
        let revision = Self::revision(prop, 48);
        // java return null != revision ? this.setModifyRunFormatRevision(revision, prop, true) : this.getNewRunFormatRevision();
        match revision {
            Some(revision) => self.setModifyRunFormatRevision(&revision, prop, true),
            None => self.getNewRunFormatRevision(),
        }
    }

    // java public PropertySet setModifyRunFormatRevision(Revision revision, PropertySet prop, boolean bRevision)
    pub fn setModifyRunFormatRevision(&self, revision: &Revision, prop: &PropertySet, bRevision: bool) -> PropertySet {
        // java Assert.assertNotNull("revision should not be null", revision);
        Assert::assertNotNull_msg("revision should not be null", Some(revision));
        // java MutablePropertySet mutaPropSet = new MutablePropertySet();
        let mut mutaPropSet = MutablePropertySet::new();
        // java Revision newRevision = revision.clone();
        let mut newRevision = revision.clone();
        // java if (bRevision) this.setRevision(newRevision);
        if bRevision { self.super_.setRevision(&mut newRevision); }
        // java mutaPropSet.setValue(48, newRevision);
        Self::set_revision(&mut mutaPropSet, 48, newRevision);
        // java mutaPropSet.setBoolean(47, true);
        mutaPropSet.setBoolean(47, true);
        // java Integer rsid = (Integer)prop.getValue(51);
        let mut rsid = Self::integer(prop, 51);
        // java Assert.logNotNull("rsid should not be null!", rsid);
        Assert::logNotNull_msg("rsid should not be null!", rsid.as_ref());
        // java if (null == rsid) rsid = this.assignRsid();
        if rsid.is_none() { rsid = Some(self.super_.assignRsid()); }
        // java mutaPropSet.setInt(51, rsid);
        mutaPropSet.setInt(51, rsid.expect("rsid"));
        // java return mutaPropSet.freeze();
        mutaPropSet.freeze()
    }

    // java public PropertySet setRunDeleteRevision(PropertySet prop)
    pub fn setRunDeleteRevision(&self, prop: &PropertySet) -> PropertySet {
        // java Assert.assertNotNull("prop should not be null!", prop);
        Assert::assertNotNull_msg("prop should not be null!", Some(prop));
        // java Assert.assertTrue("prop should not has delete revision!", null == prop.getValue(45));
        Assert::assertTrue_msg("prop should not has delete revision!", prop.getValue(45).is_none());
        // java Assert.assertTrue("comment subDocument should not has revision!", 3 != this.mDocument.getType());
        Assert::assertTrue_msg("comment subDocument should not has revision!", 3 != self.document().getType__on_IDocument());
        // java PropertySet deleteProp = this.getNewRunDeleteRevision();
        let deleteProp = self.getNewRunDeleteRevision();
        // java Assert.assertNotNull("deleteProp should not be null!", deleteProp);
        Assert::assertNotNull_msg("deleteProp should not be null!", Some(&deleteProp));
        // java return PropertySet.merge(prop, deleteProp);
        PropertySet::merge(prop, &deleteProp)
    }

    // java public PropertySet setParagraphFormatRevision(PropertySet prop)
    pub fn setParagraphFormatRevision(&self, prop: &PropertySet) -> PropertySet {
        // java Assert.assertNotNull("prop should not be null", prop);
        Assert::assertNotNull_msg("prop should not be null", Some(prop));
        // java Revision revision = (Revision)prop.getValue(237);
        let revision = Self::revision(prop, 237);
        // java if (null != revision) ... else if (null == revision) ...
        match revision {
            Some(revision) => self.setModifyParagraphFormatRevision(&revision, prop, true),
            None => self.getNewParagraphFormatRevision(),
        }
    }

    // java public PropertySet setModifyParagraphFormatRevision(Revision revision, PropertySet prop, boolean bRevision)
    pub fn setModifyParagraphFormatRevision(&self, revision: &Revision, prop: &PropertySet, bRevision: bool) -> PropertySet {
        // java Assert.assertNotNull("revision should not be null", revision);
        Assert::assertNotNull_msg("revision should not be null", Some(revision));
        // java Assert.assertNotNull("prop should not be null", prop);
        Assert::assertNotNull_msg("prop should not be null", Some(prop));
        // java MutablePropertySet p = new MutablePropertySet();
        let mut p = MutablePropertySet::new();
        // java Revision newRevision = revision.clone();
        let mut newRevision = revision.clone();
        // java if (bRevision) this.setRevision(newRevision);
        if bRevision { self.super_.setRevision(&mut newRevision); }
        // java p.setValue(237, newRevision);
        Self::set_revision(&mut p, 237, newRevision);
        // java p.setBoolean(238, true);
        p.setBoolean(238, true);
        // java Integer rsid = (Integer)prop.getValue(239);
        let mut rsid = Self::integer(prop, 239);
        // java Assert.logNotNull("rsid should not be null!", rsid);
        Assert::logNotNull_msg("rsid should not be null!", rsid.as_ref());
        // java if (null == rsid) rsid = this.assignRsid();
        if rsid.is_none() { rsid = Some(self.super_.assignRsid()); }
        // java p.setInt(239, rsid);
        p.setInt(239, rsid.expect("rsid"));
        // java return p.freeze();
        p.freeze()
    }

    // java private PropertySet getNewParagraphFormatRevision()
    fn getNewParagraphFormatRevision(&self) -> PropertySet {
        // java Revision revision = new Revision(3);
        let mut revision = Revision::Revision__int(3);
        // java this.setRevision(revision);
        self.super_.setRevision(&mut revision);
        // java MutablePropertySet propSet = new MutablePropertySet();
        let mut propSet = MutablePropertySet::new();
        // java int rsid = this.assignRsid();
        let rsid = self.super_.assignRsid();
        // java propSet.setValue(237, revision);
        Self::set_revision(&mut propSet, 237, revision);
        // java propSet.setInt(239, rsid);
        propSet.setInt(239, rsid);
        // java propSet.setBoolean(238, true);
        propSet.setBoolean(238, true);
        // java return propSet.freeze();
        propSet.freeze()
    }

    // java public PropertySet setRunInsertRevision(PropertySet prop)
    pub fn setRunInsertRevision(&self, prop: &PropertySet) -> PropertySet {
        // java Assert.assertTrue("prop should not has delete revision! because the run has been delete!", null == prop.getValue(45));
        Assert::assertTrue_msg("prop should not has delete revision! because the run has been delete!", prop.getValue(45).is_none());
        // java Assert.assertTrue("comment subDocument should not has revision!", 3 != this.mDocument.getType());
        Assert::assertTrue_msg("comment subDocument should not has revision!", 3 != self.document().getType__on_IDocument());
        // java Revision revision = new Revision(2);
        let mut revision = Revision::Revision__int(2);
        // java this.setRevision(revision);
        self.super_.setRevision(&mut revision);
        // java MutablePropertySet propSet = new MutablePropertySet(prop);
        let mut propSet = MutablePropertySet::new_from_ps(prop);
        // java int rsid = this.assignRsid();
        let rsid = self.super_.assignRsid();
        // java propSet.setValue(46, revision);
        Self::set_revision(&mut propSet, 46, revision);
        // java propSet.setBoolean(55, true);
        propSet.setBoolean(55, true);
        // java propSet.setInt(50, rsid);
        propSet.setInt(50, rsid);
        // java return propSet.freeze();
        propSet.freeze()
    }

    // java private PropertySet getNewRunDeleteRevision()
    fn getNewRunDeleteRevision(&self) -> PropertySet {
        // java Revision revision = new Revision(1);
        let mut revision = Revision::Revision__int(1);
        // java this.setRevision(revision);
        self.super_.setRevision(&mut revision);
        // java MutablePropertySet propSet = new MutablePropertySet();
        let mut propSet = MutablePropertySet::new();
        // java int rsid = this.assignRsid();
        let rsid = self.super_.assignRsid();
        // java propSet.setValue(45, revision);
        Self::set_revision(&mut propSet, 45, revision);
        // java propSet.setBoolean(54, true);
        propSet.setBoolean(54, true);
        // java propSet.setInt(49, rsid);
        propSet.setInt(49, rsid);
        // java return propSet.freeze();
        propSet.freeze()
    }

    // java public boolean checkDeleteRun(PropertySet prop)
    pub fn checkDeleteRun__propertyset(&self, prop: &PropertySet) -> bool {
        // java Revision revison = (Revision)prop.getValue(46);
        let revision = Self::revision(prop, 46);
        // java if (null == revison) return false;
        let Some(revision) = revision else { return false };
        // java String author = revison.getAuthor();
        let author = revision.getAuthor();
        // java String deleteAuthor = this.mTextDocument.getRevisionsAuthor();
        let deleteAuthor = self.textDocument().getRevisionsAuthor();
        // java Assert.assertNotNull(author); Assert.assertNotNull(deleteAuthor);
        Assert::assertNotNull(author.as_ref());
        Assert::assertNotNull(deleteAuthor.as_ref());
        // java return deleteAuthor.equals(author);
        deleteAuthor == author
    }

    // java public boolean checkDeleteRun(int start, int end)
    pub fn checkDeleteRun(&self, start: i32, end: i32) -> bool {
        // java KSAssert.assertTrue("range should not be null!", start <= end);
        KSAssert::assertTrue(start <= end);
        // java IChpxTable iChpxTable = this.mDocument.getChpxTable();
        let table = self.document().getChpxTable().expect("KParaRunRevision: null CHPX table");
        // java IChpxTable.Entry chpxStart = iChpxTable.seek(start);
        let mut entry = unsafe { IChpxTable::IChpxTable::seek(&*table, start) }.expect("null CHPX start");
        // java IChpxTable.Entry chpxEnd = iChpxTable.seek(end);
        let chpxEnd = unsafe { IChpxTable::IChpxTable::seek(&*table, end) }.expect("null CHPX end");
        // java for(IChpxTable.Entry e = chpxStart; e != chpxEnd; e = e.getNext())
        while entry.position() != chpxEnd.position() {
            // java if (this.checkDeleteRun(e.getProperty())) return true;
            let prop = ChpxEntry::getProperty(entry.as_ref()).expect("null CHPX property");
            if self.checkDeleteRun__propertyset(prop) { return true; }
            entry = ChpxEntry::getNext(entry.as_ref()).expect("CHPX traversal ended before end");
        }
        // java return false;
        false
    }

    // java private static void getInsertRevision(PropertySet prop, MutablePropertySet propSet)
    fn getInsertRevision(prop: &PropertySet, propSet: &mut MutablePropertySet) {
        // java Revision previousRevison = (Revision)prop.getValue(46);
        let _ = Self::revision(prop, 46);
        let previousRevison = prop.getValue(46).cloned();
        // java Boolean previousfInsert = (Boolean)prop.getValue(55);
        let previousfInsert = Self::boolean(prop, 55);
        // java Integer previousRsid = (Integer)prop.getValue(50);
        let previousRsid = Self::integer(prop, 50);
        // java if (null != previousRevison)
        if let Some(revision) = previousRevison {
            // java propSet.setValue(46, previousRevison);
            propSet.setValue(46, Some(revision));
            // java if (null != previousRsid) propSet.setInt(50, previousRsid);
            if let Some(rsid) = previousRsid { propSet.setInt(50, rsid); }
            // java if (null != previousfInsert) propSet.setBoolean(55, previousfInsert);
            if let Some(flag) = previousfInsert { propSet.setBoolean(55, flag); }
        }
    }

    // java private static void getDeleteRevision(PropertySet prop, MutablePropertySet propSet)
    fn getDeleteRevision(prop: &PropertySet, propSet: &mut MutablePropertySet) {
        // java Revision previousRevison = (Revision)prop.getValue(45);
        let _ = Self::revision(prop, 45);
        let previousRevison = prop.getValue(45).cloned();
        // java Boolean previousfDelete = (Boolean)prop.getValue(54);
        let previousfDelete = Self::boolean(prop, 54);
        // java Integer previousRsid = (Integer)prop.getValue(49);
        let previousRsid = Self::integer(prop, 49);
        // java if (null != previousRevison)
        if let Some(revision) = previousRevison {
            // java propSet.setValue(45, previousRevison);
            propSet.setValue(45, Some(revision));
            // java if (null != previousRsid) propSet.setInt(49, previousRsid);
            if let Some(rsid) = previousRsid { propSet.setInt(49, rsid); }
            // java if (null != previousfDelete) propSet.setBoolean(54, previousfDelete);
            if let Some(flag) = previousfDelete { propSet.setBoolean(54, flag); }
        }
    }
}

impl std::ops::Deref for KParaRunRevision {
    type Target = KRevision;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
