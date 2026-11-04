// java public abstract class LayouterBase
#[derive(Default)]
pub struct LayouterBase {
    // java protected boolean bStarted;
    pub bStarted: bool,
}

// java public interface IStateEnvoy
pub trait IStateEnvoy {}

// java public abstract class LayouterBase
impl LayouterBase {
    // java public void verifyEnvoy(E stateEnvoy)
    pub fn verifyEnvoy(&self, stateEnvoy: Option<&dyn IStateEnvoy>) {
      // java if (null == stateEnvoy)
      if stateEnvoy.is_none() {
        // java throw new IllegalStateException("illegal state");
        panic!("illegal state");
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
    }

    // java public void start(E stateEnvoy)
    pub fn start(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java if (!this.bStarted)
      if !self.bStarted {
        // java this.verifyEnvoy(stateEnvoy);
        self.verifyEnvoy(Some(stateEnvoy));
        // java this.onStart(stateEnvoy);
        self.onStart(stateEnvoy);
        // java this.bStarted = true;
        self.bStarted = true;
      }
    }

    // java public void end(E stateEnvoy)
    pub fn end(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java if (this.bStarted)
      if self.bStarted {
        // java this.onEnd(stateEnvoy);
        self.onEnd(stateEnvoy);
      }
      // java this.bStarted = false;
      self.bStarted = false;
    }

    // java public boolean hasStart()
    pub fn hasStart(&self) -> bool {
      // java return this.bStarted;
      return self.bStarted;
    }

    // java public void reset()
    pub fn reset(&mut self) {
      // java this.bStarted = false;
      self.bStarted = false;
    }

    // java protected abstract void onStart(E var1)
    pub fn onStart(&mut self, var1: &mut dyn IStateEnvoy) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("LayouterBase::onStart not implemented");
      */
    }

    // java protected abstract void onEnd(E var1)
    pub fn onEnd(&mut self, var1: &mut dyn IStateEnvoy) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("LayouterBase::onEnd not implemented");
      */
    }

}
