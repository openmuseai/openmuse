// Java: import java.io.IOException
// Java: import java.util.ArrayList

use super::AudioItem::AudioItem;

// Java: public interface IAudioFileReader
pub trait IAudioFileReader {
    // Java: abstract ArrayList<AudioItem> read()
    fn read(&self) -> Vec<AudioItem>;

}
