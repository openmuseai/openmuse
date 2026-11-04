// java public enum DateSwitch
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DateSwitch {
    HijriCalendar,
    SakaEraCalendar,
    LastUsed,
    UmAlQuraCalendar,
}

// java public enum DateSwitch
impl DateSwitch {
    pub fn getValue(&self) -> char {
        match self {
            DateSwitch::HijriCalendar => 'h',
            DateSwitch::SakaEraCalendar => 's',
            DateSwitch::LastUsed => 'l',
            DateSwitch::UmAlQuraCalendar => 'u',
        }
    }
}
