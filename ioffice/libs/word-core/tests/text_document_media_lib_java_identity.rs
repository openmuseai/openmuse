use drawing::cn::wps::moffice::drawing::media::{
    IMediaLib::IMediaLib, MediaLib::MediaLib, MediaTypeEnum::MediaTypeEnum,
};
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;

#[test]
fn text_document_returns_one_media_lib_for_later_picture_id_backfill() {
    let mut document = TextDocument::TextDocument__boolean(true);
    let root = document.getTmpRootDir();
    assert!(root.contains("writer"));
    let first = document.getMediaLib() as *mut MediaLib;
    let second = document.getMediaLib() as *mut MediaLib;
    assert_eq!(first, second);

    let media_id = document.getMediaLib().addMedia_from_bytes(&[1, 2, 3], MediaTypeEnum::PICTURE);
    let path = document.getMediaLib().getMediaByID(media_id, MediaTypeEnum::PICTURE);
    assert_eq!(std::fs::read(path).unwrap(), [1, 2, 3]);

    // The Java document disposes this directory; this isolated test owns it.
    let _ = std::fs::remove_dir_all(root);
}
