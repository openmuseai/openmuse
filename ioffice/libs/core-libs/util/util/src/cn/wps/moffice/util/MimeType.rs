// Java: import java.util.HashSet
// Java: import java.util.Set
use std::collections::HashSet;
use std::sync::OnceLock;

// Java field: MimeType Audio
static AUDIO: OnceLock<MimeType> = OnceLock::new();
// Java field: MimeType Video
static VIDEO: OnceLock<MimeType> = OnceLock::new();
// Java field: MimeType Archive
static ARCHIVE: OnceLock<MimeType> = OnceLock::new();
// Java field: MimeType Image
static IMAGE_MIME: OnceLock<MimeType> = OnceLock::new();

// Java: public class MimeType
pub struct MimeType {
    // Java field: String mType
    pub(crate) m_type: String,
    // Java field: Set<String> mSet
    pub(crate) m_set: HashSet<String>,
}

// Java: public class MimeType
impl MimeType {
    // Java: MimeType(String type, Object unused)
    pub(crate) fn new_with_unused(type_str: &str, _unused: Option<()>) -> Self {
        // Java: this(type);
        Self::new(type_str)
    }

    // Java: MimeType(String type)
    fn new(type_str: &str) -> Self {
        // Java: super();
        // Java: this.mSet = new HashSet();
        // Java: this.mType = type;
        MimeType {
            m_type: type_str.to_string(),
            m_set: HashSet::new(),
        }
    }

    // Java: String getMimeType(String subType)
    pub fn get_mime_type(&self, sub_type: &str) -> String {
        // Java: return this.mType + "/" + subType;
        format!("{}/{}", self.m_type, sub_type)
    }

    // Java: boolean contains(String subType)
    pub fn contains(&self, sub_type: &str) -> bool {
        // Java: return this.mSet.contains(subType);
        self.m_set.contains(sub_type)
    }

    pub fn audio() -> &'static MimeType {
        AUDIO.get_or_init(|| Self::init_audio())
    }

    pub fn video() -> &'static MimeType {
        VIDEO.get_or_init(|| Self::init_video())
    }

    pub fn archive() -> &'static MimeType {
        ARCHIVE.get_or_init(|| Self::init_archive())
    }

    pub fn image() -> &'static MimeType {
        IMAGE_MIME.get_or_init(|| Self::init_image())
    }

    // Java: public class Audio
    // Java: Audio()
    fn init_audio() -> MimeType {
        // Java: super("audio", null);
        let mut mt = MimeType::new_with_unused("audio", None);
        // Java: this.mSet.add("wav");
        mt.m_set.insert("wav".to_string());
        // Java: this.mSet.add("mp3");
        mt.m_set.insert("mp3".to_string());
        // Java: this.mSet.add("wma");
        mt.m_set.insert("wma".to_string());
        // Java: this.mSet.add("amr");
        mt.m_set.insert("amr".to_string());
        // Java: this.mSet.add("aac");
        mt.m_set.insert("aac".to_string());
        // Java: this.mSet.add("flac");
        mt.m_set.insert("flac".to_string());
        // Java: this.mSet.add("mid");
        mt.m_set.insert("mid".to_string());
        // Java: this.mSet.add("mp2");
        mt.m_set.insert("mp2".to_string());
        // Java: this.mSet.add("ac3");
        mt.m_set.insert("ac3".to_string());
        // Java: this.mSet.add("ogg");
        mt.m_set.insert("ogg".to_string());
        // Java: this.mSet.add("ape");
        mt.m_set.insert("ape".to_string());
        // Java: this.mSet.add("m4a");
        mt.m_set.insert("m4a".to_string());
        mt
    }

    // Java: public class Video
    // Java: Video()
    fn init_video() -> MimeType {
        // Java: super("video", null);
        let mut mt = MimeType::new_with_unused("video", None);
        // Java: this.mSet.add("mp4");
        mt.m_set.insert("mp4".to_string());
        // Java: this.mSet.add("avi");
        mt.m_set.insert("avi".to_string());
        // Java: this.mSet.add("mpg");
        mt.m_set.insert("mpg".to_string());
        // Java: this.mSet.add("mov");
        mt.m_set.insert("mov".to_string());
        // Java: this.mSet.add("swf");
        mt.m_set.insert("swf".to_string());
        // Java: this.mSet.add("3gp");
        mt.m_set.insert("3gp".to_string());
        // Java: this.mSet.add("flv");
        mt.m_set.insert("flv".to_string());
        // Java: this.mSet.add("wmv");
        mt.m_set.insert("wmv".to_string());
        // Java: this.mSet.add("vob");
        mt.m_set.insert("vob".to_string());
        // Java: this.mSet.add("rmvb");
        mt.m_set.insert("rmvb".to_string());
        // Java: this.mSet.add("rm");
        mt.m_set.insert("rm".to_string());
        // Java: this.mSet.add("mkv");
        mt.m_set.insert("mkv".to_string());
        mt
    }

    // Java: public class Archive
    // Java: Archive()
    fn init_archive() -> MimeType {
        // Java: super("application", null);
        let mut mt = MimeType::new_with_unused("application", None);
        // Java: this.mSet.add("rar");
        mt.m_set.insert("rar".to_string());
        // Java: this.mSet.add("z");
        mt.m_set.insert("z".to_string());
        // Java: this.mSet.add("7z");
        mt.m_set.insert("7z".to_string());
        mt
    }

    // Java: public class Image
    // Java: Image()
    fn init_image() -> MimeType {
        // Java: super("image", null);
        let mut mt = MimeType::new_with_unused("image", None);
        // Java: this.mSet.add("jpg");
        mt.m_set.insert("jpg".to_string());
        // Java: this.mSet.add("gif");
        mt.m_set.insert("gif".to_string());
        // Java: this.mSet.add("png");
        mt.m_set.insert("png".to_string());
        // Java: this.mSet.add("jpeg");
        mt.m_set.insert("jpeg".to_string());
        // Java: this.mSet.add("bmp");
        mt.m_set.insert("bmp".to_string());
        // Java: this.mSet.add("webp");
        mt.m_set.insert("webp".to_string());
        // Java: this.mSet.add("tif");
        mt.m_set.insert("tif".to_string());
        // Java: this.mSet.add("tga");
        mt.m_set.insert("tga".to_string());
        // Java: this.mSet.add("ico");
        mt.m_set.insert("ico".to_string());
        // Java: this.mSet.add("heic");
        mt.m_set.insert("heic".to_string());
        // Java: this.mSet.add("heif");
        mt.m_set.insert("heif".to_string());
        mt
    }
}
