// java public interface IInterruptState
pub trait IInterruptState {
    // java default boolean isWaitCancle()
    fn isWaitCancle(&self) -> bool {
        false
    }

    // java default boolean isWaitExit()
    fn isWaitExit(&self) -> bool {
        false
    }

    // java default boolean isWaitClose()
    fn isWaitClose(&self) -> bool {
        false
    }

    // java default boolean isWaitInterrupt()
    fn isWaitInterrupt(&self) -> bool {
        self.isWaitCancle() || self.isWaitExit() || self.isWaitClose()
    }

    // java default void setInterrupted(boolean interrupted)
    fn setInterrupted(&mut self, _interrupted: bool) {
    }

    // java default boolean isInterrupted()
    fn isInterrupted(&self) -> bool {
        false
    }

    // java default boolean checkInterruptedToExit()
    fn checkInterruptedToExit(&mut self) -> bool {
        if self.isWaitInterrupt() {
            self.setInterrupted(true);
            true
        } else {
            false
        }
    }

}
