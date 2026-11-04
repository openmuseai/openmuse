// Java: public enum ZOrderCmd
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ZOrderCmd {
    BringForward,
    BringInFrontOfText,
    BringToFront,
    SendBackward,
    SendBehindText,
    SendToBack,
}

// Java: public enum ZOrderCmd
impl ZOrderCmd {
    // Java: ZOrderCmd()
    pub fn _private_ctor() {}
}
