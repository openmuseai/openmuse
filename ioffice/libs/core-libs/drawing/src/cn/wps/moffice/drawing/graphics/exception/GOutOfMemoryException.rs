#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.media.IMediaLib
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::{OnceLock, Mutex};

use super::GBigBitmapException::GBigBitmapException;
use super::IGraphicsException::IGraphicsException;

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java field: HashMap<Integer, GOutOfMemoryException> gcExceptions
static GC_EXCEPTIONS: OnceLock<Mutex<Option<HashMap<i32, GOutOfMemoryException>>>> = OnceLock::new();

// Java: public class GOutOfMemoryException extends GBigBitmapException
pub struct GOutOfMemoryException {
    pub base: GBigBitmapException,
}

// Java: public class GOutOfMemoryException extends GBigBitmapException
impl GOutOfMemoryException {
    // Java: GOutOfMemoryException(IMediaLib mediaLib, int mediaId, int expectedWidth, int expectedHeight)
    pub fn new(_mediaLib: Element, mediaId: i32, expectedWidth: i32, expectedHeight: i32) -> Self {
      // Java: super(mediaId, expectedWidth, expectedHeight);
      let mut exc = GOutOfMemoryException { base: GBigBitmapException::new(mediaId, expectedWidth, expectedHeight) };
      // Java: this.setType(1);
      exc.base.base.setType(1);
      // Java: put(mediaLib, this);
      // TODO: put(mediaLib, this) - cannot register in static from constructor easily
      exc
    }

    // Java: static void put(IMediaLib mediaLib, GOutOfMemoryException exception)
    pub fn put(_mediaLib: Element, exception_media_id: i32, exception_width: i32, exception_height: i32) {
      // Java: if (null == gcExceptions)
      let gc = GC_EXCEPTIONS.get_or_init(|| Mutex::new(None));
      let mut guard = gc.lock().unwrap();
      if guard.is_none()
      {
         // Java: gcExceptions = new HashMap();
         *guard = Some(HashMap::new());
         // Java: mediaLib.registerMediaLibEventListener(new IMediaLib.MediaLibEventListener()
         // TODO: register listener for media lib event
         // Java: public void onMediaLibEvent(int mediaId)
         // TODO: onMediaLibEvent callback
         // Java: if (GOutOfMemoryException.gcExceptions.containsKey(mediaId))
         // TODO: check if contains key
         // Java: GOutOfMemoryException.gcExceptions.remove(mediaId);
         // TODO: remove from map
      }
      // Java: gcExceptions.put(exception.getMediaId(), exception);
      if let Some(ref mut map) = *guard {
          let exc = GOutOfMemoryException { base: GBigBitmapException::new(exception_media_id, exception_width, exception_height) };
          map.insert(exception_media_id, exc);
      }
    }

    // Java: static boolean isStillOutOfMemory(int mediaID, int width, int height)
    pub fn isStillOutOfMemory(mediaID: i32, width: i32, height: i32) -> bool {
      // Java: GOutOfMemoryException exception = null != gcExceptions ? (GOutOfMemoryException)gcExceptions.get(mediaID) : null;
      let gc = GC_EXCEPTIONS.get_or_init(|| Mutex::new(None));
      let guard = gc.lock().unwrap();
      let exception = guard.as_ref().and_then(|map| map.get(&mediaID));
      // Java: if (exception == null)
      if exception.is_none()
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         let exception = exception.unwrap();
         // Java: int expectedWidth = exception.getExpectedWidth();
         let expectedWidth = exception.base.base.getExpectedWidth();
         // Java: int expectedHeight = exception.getExpectedHeight();
         let expectedHeight = exception.base.base.getExpectedHeight();
         // Java: return width >= expectedWidth || height >= expectedHeight;
         return width >= expectedWidth || height >= expectedHeight;
      }
    }

}
