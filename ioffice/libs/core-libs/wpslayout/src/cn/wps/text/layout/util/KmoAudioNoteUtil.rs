// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Java field: HashMap<String, String> audioMap
static AUDIO_MAP: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn get_audio_map() -> &'static Mutex<HashMap<String, String>> {
    AUDIO_MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

// Java: public class KmoAudioNoteUtil
pub struct KmoAudioNoteUtil {
}

// Java: public class KmoAudioNoteUtil
impl KmoAudioNoteUtil {
    // Java: KmoAudioNoteUtil()
    pub fn new() -> Self {
      // Java: super();
      KmoAudioNoteUtil {}
    }

    // Java: static void addAudio(String id, String content)
    pub fn addAudio(id: &str, content: &str) {
      // Java: audioMap.put(id, content);
      get_audio_map().lock().unwrap().insert(id.to_string(), content.to_string());
    }

    // Java: static void removeAudio(String id)
    pub fn removeAudio(id: &str) {
      // Java: audioMap.remove(id);
      get_audio_map().lock().unwrap().remove(id);
    }

}
