#![allow(non_snake_case, dead_code)]

// java public class KoZone
pub struct KoZone {}

// java private static final KoZone defaultZone = new KoZone()
static DEFAULT_ZONE: KoZone = KoZone {};

impl KoZone {
    // java public static KoZone enter()
    pub fn enter() -> &'static KoZone {
        // java return defaultZone;
        &DEFAULT_ZONE
    }

    // java public <Value> Value allocate(Class<Value> clazz)
    pub fn allocate(&self) {}

    // java public void leave()
    pub fn leave(&self) {}

    // java public static KoZone defaultZone()
    pub fn defaultZone() -> &'static KoZone {
        // java return defaultZone;
        &DEFAULT_ZONE
    }
}
