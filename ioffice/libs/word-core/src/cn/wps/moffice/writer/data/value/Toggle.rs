// java public enum Toggle
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Toggle {
    // java OFF(0),
    OFF,
    // java ON(1),
    ON,
    // java CURRENT(128),
    CURRENT,
    // java OPPOSITE(129),
    OPPOSITE,
}

// java public enum Toggle
impl Toggle {
    // java int mData;
    pub fn mData(&self) -> i32 {
        match self {
            // java OFF(0)
            Toggle::OFF => 0,
            // java ON(1)
            Toggle::ON => 1,
            // java CURRENT(128)
            Toggle::CURRENT => 128,
            // java OPPOSITE(129)
            Toggle::OPPOSITE => 129,
        }
    }

    // java private Toggle(int value)
    pub fn from_value(value: i32) -> Self {
        // java this.mData = value;
        match value {
            0 => Toggle::OFF,
            1 => Toggle::ON,
            128 => Toggle::CURRENT,
            129 => Toggle::OPPOSITE,
            _ => Toggle::OFF,
        }
    }
}
