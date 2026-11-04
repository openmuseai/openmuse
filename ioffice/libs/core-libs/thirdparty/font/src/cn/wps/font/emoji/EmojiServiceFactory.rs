// Java: public class EmojiServiceFactory
pub struct EmojiServiceFactory {
}

// Java: public class EmojiServiceFactory
impl EmojiServiceFactory {
    // Java: EmojiServiceFactory()
    pub fn new() -> Self {
        // Java: super();
        EmojiServiceFactory {}
    }

    // Java: static IEmojiService create()
    pub fn create() -> EmojiServiceImpl {
        // Java: return new EmojiServiceImpl();
        EmojiServiceImpl::new()
    }
}

pub struct EmojiServiceImpl;

impl EmojiServiceImpl {
    pub fn new() -> Self {
        EmojiServiceImpl
    }
}
