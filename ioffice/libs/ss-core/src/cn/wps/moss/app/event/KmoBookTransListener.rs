// java public interface KmoBookTransListener {
pub trait KmoBookTransListener:Send+Sync{fn onUndo(&self);fn onRedo(&self);}
