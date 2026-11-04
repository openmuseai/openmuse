use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertySet;
use util_adapter::cn::wps::core::runtime::Platform::Platform;
use std::sync::{Mutex, MutexGuard};
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::core::KParaRunRevision::KParaRunRevision;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use word_core::cn::wps::moffice::writer::data::KTextStream::KTextStream;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use word_core::cn::wps::moffice::writer::data::value::Revision::Revision;

static AUTHOR_LOCK: Mutex<()> = Mutex::new(());

struct AuthorScope {
    _lock: MutexGuard<'static, ()>,
    previous: Option<String>,
}

impl AuthorScope {
    fn new(author: &str) -> Self {
        let lock = AUTHOR_LOCK.lock().expect("author test lock");
        let previous = Platform::getUserName();
        Platform::setUserName(Some(author));
        Self { _lock: lock, previous }
    }
}

impl Drop for AuthorScope {
    fn drop(&mut self) {
        Platform::setUserName(self.previous.as_deref());
    }
}

fn revision_mark(prop: &PropertySet, id: i32) -> Option<i32> {
    prop.getValue(id)
        .and_then(|v| v.as_any().downcast_ref::<Revision>())
        .map(Revision::getMark)
}

#[test]
fn java_run_insert_delete_revisions_keep_original_properties_and_rsids() {
    let _author = AuthorScope::new("same author");
    let core = TextDocument::new();
    let main = core.getMainDocument().expect("Java main subdocument");
    let revision = KParaRunRevision::KParaRunRevision(main as &dyn Document);
    let mut base = MutablePropertySet::new();
    base.setInt(10, 120);
    let base = base.freeze();

    let inserted = revision.setRunInsertRevision(&base);
    assert_eq!(revision_mark(&inserted, 46), Some(Revision::INSERT));
    assert!(inserted.getBoolean__int_boolean(55, false));
    assert_eq!(inserted.getInt(10), 120);
    assert!(revision.checkDeleteRun__propertyset(&inserted));

    let deleted = revision.setRunDeleteRevision(&inserted);
    assert_eq!(revision_mark(&deleted, 45), Some(Revision::DELETE));
    assert!(deleted.getBoolean__int_boolean(54, false));
    assert_eq!(deleted.getInt(10), 120);
    let retained = KParaRunRevision::getInsertDeleteRevision(&deleted);
    assert_eq!(revision_mark(&retained, 46), Some(Revision::INSERT));
    assert_eq!(revision_mark(&retained, 45), Some(Revision::DELETE));
    assert!(!retained.contains(10));

    let rsids = core.getRsids().expect("live rsid list");
    assert_eq!(rsids.len(), 2);
    assert_eq!(rsids[0], inserted.getInt(50));
    assert_eq!(rsids[1], deleted.getInt(49));
}

#[test]
fn java_paragraph_format_revision_creates_and_reuses_rsid() {
    let _author = AuthorScope::new("paragraph author");
    let core = TextDocument::new();
    let main = core.getMainDocument().expect("Java main subdocument");
    let revision = KParaRunRevision::KParaRunRevision(main as &dyn Document);
    let base = MutablePropertySet::new().freeze();
    let created = revision.setParagraphFormatRevision(&base);
    assert_eq!(revision_mark(&created, 237), Some(Revision::FORMATCHANGE));
    assert!(created.getBoolean__int_boolean(238, false));
    let rsid = created.getInt(239);
    let modified = revision.setParagraphFormatRevision(&created);
    assert_eq!(modified.getInt(239), rsid);
    assert_eq!(core.getRsids().expect("live rsid list").len(), 1);
}

#[test]
fn java_check_delete_run_scans_the_chpx_range() {
    let _author = AuthorScope::new("range author");
    let mut core = TextDocument::new();
    core.createSubDocument(0);
    let main = core.getSubdocumentMut(0).expect("main subdocument");
    let base = MutablePropertySet::new().freeze();
    CoreHelper::appendText__string_ps(Some(main as &mut dyn Document), Some("ab\r"), Some(&base));
    let stream = KTextStream::KTextStream(main as &dyn Document);
    let mut first = stream.split_chpx(0);
    stream.split_chpx(1);
    let revision = KParaRunRevision::KParaRunRevision(main as &dyn Document);
    let inserted = revision.setRunInsertRevision(&base);
    ChpxEntry::setProperty(first.as_mut(), Some(&inserted));
    assert!(revision.checkDeleteRun(0, 1));
    assert!(!revision.checkDeleteRun(1, 2));
}
