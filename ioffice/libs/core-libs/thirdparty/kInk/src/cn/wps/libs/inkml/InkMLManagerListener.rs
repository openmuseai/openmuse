// Java: import com.hp.hpl.inkml.impl.InkCreator
use crate::com::hp::hpl::inkml::r#impl::InkCreator::InkCreator;

// Java: public interface InkMLManagerListener
pub trait InkMLManagerListener {
    // Java: abstract void onInkAdd(InkCreator var1)
    fn onInkAdd(&self, var1: &InkCreator);

    // Java: abstract void onInkDelete(InkCreator var1)
    fn onInkDelete(&self, var1: &InkCreator);

}
