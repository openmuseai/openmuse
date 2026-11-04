// Java: public class AudioItem
pub struct AudioItem {
    // Java field: String mText
    pub(crate) mText: String,
    // Java field: String mAudioPath
    pub(crate) mAudioPath: String,
    // Java field: String mTime
    pub(crate) mTime: String,
    // Java field: boolean mIsPureFile
    pub(crate) mIsPureFile: bool,
}

// Java: public class AudioItem
impl AudioItem {
    // Java: AudioItem(String text, String audioPath, String time, boolean isPureFile)
    pub fn new(_text: &str, _audioPath: &str, _time: &str, _isPureFile: bool) -> Self {
      // Java: super();
      // Java: this.mText = text;
      // Java: this.mAudioPath = audioPath;
      // Java: this.mTime = time;
      // Java: this.mIsPureFile = isPureFile;
      AudioItem {
          mText: _text.to_string(),
          mAudioPath: _audioPath.to_string(),
          mTime: _time.to_string(),
          mIsPureFile: _isPureFile,
      }
    }

}
