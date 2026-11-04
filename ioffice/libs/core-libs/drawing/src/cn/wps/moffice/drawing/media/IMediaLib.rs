#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.util.zip.model.ZipBaseModel
use core_libs_util::util::zip::model::ZipBaseModel::ZipBaseModel;

use super::MediaTypeEnum::MediaTypeEnum;
use std::sync::Arc;
use std::sync::Mutex;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IMediaLib
pub trait IMediaLib {
    // Java: abstract void registerMediaLibEventListener(MediaLibEventListener var1)
    fn registerMediaLibEventListener(&mut self, var1: Box<dyn MediaLibEventListenerTrait>);

    // Java: abstract void dispose()
    fn dispose(&mut self);

    // Java: abstract int addMedia(String var1, String var2, MediaTypeEnum var3)
    fn addMedia(&mut self, var1: &str, var2: &str, var3: MediaTypeEnum) -> i32;

    // Java: abstract int addMedia(String var1, String var2, MediaTypeEnum var3, ZipBaseModel var4)
    fn addMedia_with_zip(&mut self, var1: &str, var2: &str, var3: MediaTypeEnum, var4: Arc<Mutex<ZipBaseModel>>) -> i32;

    // Java: abstract int addMedia(byte[] var1, MediaTypeEnum var2)
    fn addMedia_from_bytes(&mut self, var1: &[u8], var2: MediaTypeEnum) -> i32;

    // Java: abstract String getMediaByID(int var1, MediaTypeEnum var2)
    fn getMediaByID(&self, var1: i32, var2: MediaTypeEnum) -> String;

    // Java: abstract MediaItem getMediaByID2(int var1, MediaTypeEnum var2)
    fn getMediaByID2(&self, var1: i32, var2: MediaTypeEnum) -> Element;

    // Java: abstract String getMediaByName(String var1, MediaTypeEnum var2)
    fn getMediaByName(&self, var1: &str, var2: MediaTypeEnum) -> String;

    // Java: abstract int getMediaIDByName(String var1, MediaTypeEnum var2)
    fn getMediaIDByName(&self, var1: &str, var2: MediaTypeEnum) -> i32;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract int mediaSize()
    fn mediaSize(&self) -> i32;

    // Java: abstract int oleSize()
    fn oleSize(&self) -> i32;

    // Java: abstract String getMediaByIndex(int var1, MediaTypeEnum var2)
    fn getMediaByIndex(&self, var1: i32, var2: MediaTypeEnum) -> String;

    // Java: abstract int getMediaIDByIndex(int var1, MediaTypeEnum var2)
    fn getMediaIDByIndex(&self, var1: i32, var2: MediaTypeEnum) -> i32;

}

// Java: public interface MediaLibEventListener
pub trait MediaLibEventListenerTrait {
    // Java: abstract void onMediaLibEvent(int var1)
    fn onMediaLibEvent(&self, var1: i32);
}

pub struct MediaLibEventListener;

impl MediaLibEventListenerTrait for MediaLibEventListener {
    fn onMediaLibEvent(&self, _var1: i32) {
        todo!()
    }
}
