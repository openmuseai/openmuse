use core_libs_drawing::cn::wps::moffice::drawing::media::IMediaLib::IMediaLib;
use core_libs_drawing::cn::wps::moffice::drawing::media::MediaLib::MediaLib;
use core_libs_drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::MediaTypeEnum;
use core_libs_drawing::cn::wps::moffice::drawing::media::MediaItem::MediaItem;
use std::sync::{Arc, Mutex};
use core_libs_util::util::manager::FileManager::FileManager;
use core_libs_util::util::zip::model::ZipBaseModel::ZipBaseModel;

#[test]
fn java_media_lib_retains_picture_items_for_id_name_and_path_lookup() {
    let root = std::env::temp_dir().join(format!("picture-media-lib-{}", std::process::id()));
    let media_path = format!("{}/", root.display());
    let mut media = MediaLib::new(media_path.clone());
    let id = media.addMedia_from_bytes(b"picture-bytes", MediaTypeEnum::PICTURE);
    assert_eq!(id, 0);
    assert_eq!(std::fs::read(root.join("0.m")).unwrap(), b"picture-bytes");
    assert_eq!(media.getMediaIDByName("0.m", MediaTypeEnum::PICTURE), 0);
    assert_eq!(media.getMediaByID(0, MediaTypeEnum::PICTURE), format!("{}0.m", media_path));

    let external = root.join("source.png");
    std::fs::write(&external, b"source").unwrap();
    let id = media.addMedia("image.png", external.to_str().unwrap(), MediaTypeEnum::PICTURE);
    assert_eq!(id, 1);
    assert_eq!(media.getMediaIDByName("image.png", MediaTypeEnum::PICTURE), 1);
    assert_eq!(media.getMediaByID2(1, MediaTypeEnum::PICTURE).unwrap()
        .downcast_ref::<Mutex<MediaItem>>().unwrap().lock().unwrap().getName(), "image.png");
    assert_eq!(media.getMediaByID(1, MediaTypeEnum::PICTURE), external.to_str().unwrap());
    assert_eq!(media.getMediaIDByName("absent.png", MediaTypeEnum::PICTURE), -1);
    assert_eq!(media.getMediaByName("image.png", MediaTypeEnum::PICTURE), external.to_str().unwrap());
    let shared_item = media.getMediaByID2(1, MediaTypeEnum::PICTURE).unwrap();
    shared_item.downcast_ref::<Mutex<MediaItem>>().unwrap().lock().unwrap()
        .setName("renamed.png".to_string());
    assert_eq!(media.getMediaIDByName("renamed.png", MediaTypeEnum::PICTURE), 1);
    assert_eq!(media.getMediaIDByName("image.png", MediaTypeEnum::PICTURE), -1);
    let ole_id = media.addMedia("object.bin", external.to_str().unwrap(), MediaTypeEnum::OLE);
    assert_eq!(ole_id, 0);
    assert_eq!(media.getMediaIDByName("object.bin", MediaTypeEnum::OLE), 0);
    assert_eq!(media.getMediaByID(0, MediaTypeEnum::OLE), external.to_str().unwrap());
    let mut zip_stream = std::io::Cursor::new(Vec::<u8>::new());
    let zip_model = Arc::new(Mutex::new(ZipBaseModel::new_default(
        &mut zip_stream, "png", FileManager::new(root.to_str().unwrap()),
    )));
    let zip_id = media.addMedia_with_zip("zip.png", "", MediaTypeEnum::PICTURE, zip_model.clone());
    assert_eq!(zip_id, 2);
    let zip_item_ref = media.getMediaByID2(zip_id, MediaTypeEnum::PICTURE).unwrap();
    let zip_item = zip_item_ref.downcast_ref::<Mutex<MediaItem>>().unwrap().lock().unwrap();
    assert!(Arc::ptr_eq(zip_item.getZipBaseModel().as_ref().unwrap(), &zip_model));
    drop(zip_item);
    media.dispose();
    assert!(zip_item_ref.downcast_ref::<Mutex<MediaItem>>().unwrap()
        .lock().unwrap().getZipBaseModel().is_none());
    assert!(!root.join("0.m").exists());
    assert!(!external.exists());
    std::fs::remove_dir_all(root).unwrap();
}
