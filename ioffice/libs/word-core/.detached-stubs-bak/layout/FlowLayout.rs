//! Full-relayout placeholder: invalidate drops the paintable snapshot.

pub struct FlowLayout {
    snapshot_usable: bool,
    version: i64,
}

impl Default for FlowLayout {
    fn default() -> Self {
        FlowLayout {
            snapshot_usable: false,
            version: 0,
        }
    }
}

impl FlowLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn layout(&mut self) {
        self.snapshot_usable = true;
        self.version += 1;
    }

    /// Any document change: snapshot must not be used for paint.
    pub fn invalidate(&mut self) {
        self.snapshot_usable = false;
    }

    pub fn can_paint(&self) -> bool {
        self.snapshot_usable
    }

    pub fn version(&self) -> i64 {
        self.version
    }
}
