use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::graphics::{BlipFill::BlipFill, Picture::Picture};
use drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use word_core::cn::wps::moffice::writer::core::shape::DumpInfo::{DumpInfo, FillHandle};
use word_core::cn::wps::moffice::writer::core::shape::PictureDumpMap::PictureDumpMap;
use word_core::cn::wps::moffice::writer::core::shape::PictureLoader::{
    FinishHandlerRef, IDumpBlipHandler, IDumpFinishHandler, PictureLoader,
};

struct Finish(AtomicI32);

impl IDumpFinishHandler for Finish {
    fn onSuccess(&self, media_id: i32) {
        self.0.store(media_id, Ordering::SeqCst);
    }
    fn useThread(&self) -> bool {
        false
    }
}

struct Dump {
    media_id: i32,
    result: bool,
    called: AtomicI32,
}

impl IDumpBlipHandler for Dump {
    fn onHandle(&self, fill: FillHandle, _finish: Option<FinishHandlerRef>) -> bool {
        self.called.fetch_add(1, Ordering::SeqCst);
        fill.setMediaID(self.media_id);
        self.result
    }
}

fn make_shape(id: i32) -> Arc<Mutex<Shape>> {
    let shape = Shape::Shape_new(None).into_shared();
    shape.lock().unwrap().setShapeID(id);
    shape
}

#[test]
fn picture_dump_uses_same_fill_and_java_callback_branches() {
    PictureDumpMap::reset();
    assert!(PictureDumpMap::isFinshed());
    let shape = make_shape(91001);
    let picture = Arc::new(Mutex::new(Picture::new()));
    shape.lock().unwrap().setPicture(Some(picture.clone()));
    let fill = DumpInfo::getFill(&mut shape.lock().unwrap(), DumpInfo::PIC).unwrap();
    assert!(matches!(&fill, FillHandle::Picture(value) if Arc::ptr_eq(value, &picture)));
    assert_eq!(
        DumpInfo::getType(Some(&shape.lock().unwrap()), Some(&fill)),
        DumpInfo::PIC
    );
    assert_eq!(DumpInfo::getType(None, Some(&fill)), DumpInfo::NONE);

    let dump = Arc::new(Dump {
        media_id: 47,
        result: false,
        called: AtomicI32::new(0),
    });
    PictureDumpMap::add(91001, DumpInfo::PIC, Some(dump.clone()));
    assert!(!PictureDumpMap::isFinshed());
    let finish = Arc::new(Finish(AtomicI32::new(-2)));
    let mut loader = PictureLoader::new();
    assert_eq!(loader.getMediaID(Some(finish.clone()), &shape, DumpInfo::PIC), 47);
    assert_eq!(picture.lock().unwrap().getMediaID(), 47);
    assert_eq!(finish.0.load(Ordering::SeqCst), 47);
    assert_eq!(dump.called.load(Ordering::SeqCst), 1);
    assert_eq!(loader.dump(&shape, DumpInfo::PIC), 47);
    assert_eq!(dump.called.load(Ordering::SeqCst), 1);

    let fill_shape = make_shape(91002);
    let blip = Arc::new(Mutex::new(BlipFill::new_with_media_id(-1)));
    fill_shape.lock().unwrap().setBlipFill(Some(blip.clone()));
    let fill = DumpInfo::getFill(&mut fill_shape.lock().unwrap(), DumpInfo::FILL).unwrap();
    assert!(matches!(&fill, FillHandle::BlipFill(value) if Arc::ptr_eq(value, &blip)));
    assert_eq!(
        DumpInfo::getType(Some(&fill_shape.lock().unwrap()), Some(&fill)),
        DumpInfo::FILL
    );
    let dump_true = Arc::new(Dump {
        media_id: 51,
        result: true,
        called: AtomicI32::new(0),
    });
    PictureDumpMap::add(91002, DumpInfo::FILL, Some(dump_true.clone()));
    let finish_true = Arc::new(Finish(AtomicI32::new(-2)));
    assert_eq!(
        loader.getMediaID(Some(finish_true.clone()), &fill_shape, DumpInfo::FILL),
        51
    );
    // Java leaves notification to the handler when onHandle returns true.
    assert_eq!(finish_true.0.load(Ordering::SeqCst), -2);

    let line_shape = make_shape(91003);
    let line_blip = Arc::new(Mutex::new(BlipFill::new_with_media_id(-1)));
    let mut line = LineProperty::new();
    line.setFill(Some(line_blip.clone()));
    line_shape
        .lock()
        .unwrap()
        .setLineProperty(Some(Arc::new(Mutex::new(line))));
    let line_fill = DumpInfo::getFill(&mut line_shape.lock().unwrap(), DumpInfo::LINE).unwrap();
    assert!(matches!(&line_fill, FillHandle::BlipFill(value) if Arc::ptr_eq(value, &line_blip)));
    let finish_missing = Arc::new(Finish(AtomicI32::new(-2)));
    assert_eq!(
        loader.getMediaID(Some(finish_missing.clone()), &line_shape, DumpInfo::LINE),
        -1
    );
    // Java calls onSuccess(-1) when the fill exists but no dump handler is registered.
    assert_eq!(finish_missing.0.load(Ordering::SeqCst), -1);
    let line_dump = Arc::new(Dump {
        media_id: 53,
        result: false,
        called: AtomicI32::new(0),
    });
    PictureDumpMap::add(91003, DumpInfo::LINE, Some(line_dump));
    assert_eq!(loader.dump(&line_shape, DumpInfo::LINE), 53);
    assert_eq!(line_blip.lock().unwrap().getMediaID(), 53);

    let empty = make_shape(91004);
    let finish_empty = Arc::new(Finish(AtomicI32::new(-2)));
    assert_eq!(
        loader.getMediaID(Some(finish_empty.clone()), &empty, DumpInfo::PIC),
        -1
    );
    assert_eq!(finish_empty.0.load(Ordering::SeqCst), -2);

    let unresolved = make_shape(91005);
    unresolved
        .lock()
        .unwrap()
        .setBlipFill(Some(Arc::new(Mutex::new(BlipFill::new_with_media_id(-1)))));
    let unresolved_dump = Arc::new(Dump {
        media_id: -1,
        result: false,
        called: AtomicI32::new(0),
    });
    PictureDumpMap::add(91005, DumpInfo::FILL, Some(unresolved_dump));
    let finish_unresolved = Arc::new(Finish(AtomicI32::new(-2)));
    assert_eq!(
        loader.getMediaID(Some(finish_unresolved.clone()), &unresolved, DumpInfo::FILL),
        -1
    );
    // Java's failed onHandle branch skips onSuccess while the media ID stays -1.
    assert_eq!(finish_unresolved.0.load(Ordering::SeqCst), -2);
    PictureDumpMap::reset();
    assert!(PictureDumpMap::isFinshed());
}
