#![allow(non_snake_case, dead_code, unused_variables)]

use super::KoBin::KoBin;
use super::KoZone::KoZone;

// java public interface KoAr
pub trait KoAr {
    // java void loadBin(KoBin var1, KoZone var2)
    fn loadBin(&mut self, var1: &dyn KoBin, var2: &KoZone);
    // java KoBin saveBin(KoZone var1)
    fn saveBin(&self, var1: &KoZone) -> Box<dyn KoBin>;
    // java boolean dirty()
    fn dirty(&self) -> bool;
    // java void discard(KoZone var1)
    fn discard(&mut self, var1: &KoZone);
    // java KoBin lastBin(KoZone var1)
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBin>;
}
