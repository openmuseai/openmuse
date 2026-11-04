// Java: import com.hp.hpl.inkml.impl.InkCreator
use crate::com::hp::hpl::inkml::r#impl::InkCreator::InkCreator;

// Java: public interface InkMLTouchListener
pub trait InkMLTouchListener {
    // Java: abstract void onTouch(int var1, float var2, float var3, float var4, InkCreator var5)
    fn onTouch(&self, var1: i32, var2: f32, var3: f32, var4: f32, var5: &InkCreator);

}
