// Java: import java.util.UUID

// Java: public class UUIDUtil
pub struct UUIDUtil;

// Java: public class UUIDUtil
impl UUIDUtil {
    // Java: UUIDUtil()
    pub fn new() -> Self {
        // Java: super();
        UUIDUtil
    }

    // Java: static String generate()
    pub fn generate() -> String {
        // Java: return UUID.randomUUID().toString().toLowerCase();
        random_uuid().to_string().to_lowercase()
    }

    // Java: static String generate32ID()
    pub fn generate32ID() -> String {
        // Java: return UUID.randomUUID().toString().replaceAll("-", "");
        random_uuid().to_string().replace('-', "")
    }
}

// Native entropy adapter. Production uses the host CSPRNG; a scoped source lets
// conformance tests replay entropy without changing UUIDUtil/ET decisions.
thread_local! { static ENTROPY: std::cell::RefCell<Option<Box<dyn FnMut() -> [u8;16]>>> = std::cell::RefCell::new(None); }
pub fn withEntropy<T>(source: impl FnMut()->[u8;16]+'static, f:impl FnOnce()->T)->T {
    struct Reset(Option<Box<dyn FnMut()->[u8;16]>>);
    impl Drop for Reset {fn drop(&mut self){ENTROPY.with(|s|*s.borrow_mut()=self.0.take());}}
    let previous=ENTROPY.with(|s|s.replace(Some(Box::new(source))));let _reset=Reset(previous);f()
}
fn random_uuid() -> uuid::Uuid {
    let mut bytes=ENTROPY.with(|s|s.borrow_mut().as_mut().map(|f|f()).unwrap_or_else(||*uuid::Uuid::new_v4().as_bytes()));
    // java UUID.randomUUID(): randomBytes[6] &= 0x0f; randomBytes[6] |= 0x40;
    bytes[6]=(bytes[6]&0x0f)|0x40;
    // java UUID.randomUUID(): randomBytes[8] &= 0x3f; randomBytes[8] |= 0x80;
    bytes[8]=(bytes[8]&0x3f)|0x80;
    // java return new UUID(randomBytes);
    uuid::Uuid::from_bytes(bytes)
}
