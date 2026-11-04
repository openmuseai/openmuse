use word_core::cn::wps::moffice::writer::core::Document::SoftTextDocument;
use word_core::cn::wps::moffice::writer::core::KRevision::KRevision;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::data::value::Revision::Revision;

#[test]
fn java_rsid_is_appended_to_the_live_text_document_list() {
    let document = Box::new(TextDocument::TextDocument());
    let revision = KRevision::KRevision(document.as_ref() as &dyn SoftTextDocument);
    assert_eq!(document.getRsids().as_ref().map(|ids| ids.len()), Some(0));
    let rsid = revision.assignRsid();
    let rsids = document.getRsids().expect("Java constructor creates mRsids");
    assert_eq!(rsids.as_slice(), &[rsid]);
}

#[test]
fn java_revision_uses_platform_author_and_local_date() {
    let mut document = Box::new(TextDocument::TextDocument());
    let prior_author = document.getRevisionsAuthor();
    document.setRevisionsAuthor(Some("Word parity author"));
    let revision = KRevision::KRevision(document.as_ref() as &dyn SoftTextDocument);
    let mut value = Revision::Revision__int(Revision::INSERT);
    revision.setRevision(&mut value);
    assert_eq!(value.getAuthor().as_deref(), Some("Word parity author"));
    let date = value.getDateTime().expect("Java setRevision assigns DateTime");
    assert!((1..=12).contains(&date.month));
    assert!((1..=31).contains(&date.day));
    assert!((0..=6).contains(&date.weekday));
    document.setRevisionsAuthor(prior_author.as_deref());
}
