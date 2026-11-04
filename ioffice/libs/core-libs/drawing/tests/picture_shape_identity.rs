use core_libs_drawing::cn::wps::moffice::drawing::{
    Shape::{BaseShape, Shape},
    graphics::{BlipFill::BlipFill, Picture::Picture},
};
use std::sync::{Arc, Mutex};

#[test]
fn picture_clone_keeps_java_shape_identity_and_media_fallback() {
    let value_shape = Shape::Shape_new(None);
    assert!(matches!(value_shape.getBaseShape(), BaseShape::Borrowed(_)));
    let base = Shape::Shape_new(None).into_shared();
    base.lock().unwrap().getGRF();
    let picture = Arc::new(Mutex::new(Picture::new_with_media_id(42)));
    base.lock().unwrap().setPicture(Some(picture.clone()));
    let attached = picture.lock().unwrap().getShape().as_ref().unwrap().clone()
        .downcast::<Mutex<Shape>>().unwrap();
    assert!(Arc::ptr_eq(&attached, &base));

    let cloned = base.lock().unwrap().cloneLightWeight_default().unwrap().into_shared();
    let cloned_picture = cloned.lock().unwrap().getPicture().unwrap()
        .downcast::<Mutex<Picture>>().unwrap();
    assert!(!Arc::ptr_eq(&picture, &cloned_picture));
    let clone_attached = cloned_picture.lock().unwrap().getShape().as_ref().unwrap().clone()
        .downcast::<Mutex<Shape>>().unwrap();
    assert!(Arc::ptr_eq(&clone_attached, &cloned));
    let base_picture = base.lock().unwrap().getPicture().unwrap()
        .downcast::<Mutex<Picture>>().unwrap();
    assert!(Arc::ptr_eq(&base_picture, &picture));
    assert_eq!(picture.lock().unwrap().getMediaID(), 42);
    cloned_picture.lock().unwrap().setMediaID(-1);
    assert_eq!(cloned_picture.lock().unwrap().getMediaID(), 42);
    assert_eq!(picture.lock().unwrap().getMediaID(), 42);
}

#[test]
fn blip_fill_setter_reuses_the_mutable_fill_instance() {
    let shape = Shape::Shape_new(None).into_shared();
    let fill = Arc::new(Mutex::new(BlipFill::new_with_media_id(3)));
    shape.lock().unwrap().setBlipFill(Some(fill.clone()));
    let stored = shape.lock().unwrap().getBlipFill().unwrap()
        .downcast::<Mutex<BlipFill>>().unwrap();
    assert!(Arc::ptr_eq(&fill, &stored));
    stored.lock().unwrap().setMediaID(9);
    assert_eq!(fill.lock().unwrap().getMediaID(), 9);
    let attached = fill.lock().unwrap().getShape().as_ref().unwrap().clone()
        .downcast::<Mutex<Shape>>().unwrap();
    assert!(Arc::ptr_eq(&attached, &shape));

    let cloned = shape.lock().unwrap().cloneLightWeight_default().unwrap().into_shared();
    let cloned_fill = cloned.lock().unwrap().getBlipFill().unwrap()
        .downcast::<Mutex<BlipFill>>().unwrap();
    assert!(!Arc::ptr_eq(&fill, &cloned_fill));
    cloned_fill.lock().unwrap().setMediaID(-1);
    assert_eq!(cloned_fill.lock().unwrap().getMediaID(), 9);
}
