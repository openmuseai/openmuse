// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::{KoBin as KoBinTrait};

// Java: public interface KmoProtocol
pub trait KmoProtocol {
    // Java: abstract KoBin protocol()
    fn protocol(&self) -> Box<dyn KoBinTrait>;

    // Java: abstract void updateProtocol(KoBin var1)
    fn updateProtocol(&mut self, var1: &dyn KoBinTrait);

}
