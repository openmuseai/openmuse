// Java: import cn.wps.moffice.writer.data.field.DateTime.switches.specific.DateSwitch
use crate::cn::wps::moffice::writer::data::field::DateTime::switches::specific::DateSwitch::*;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;

// java public class SwitchRegister
#[derive(Default)]
pub struct SwitchRegister {
}

// java public class SwitchRegister
impl SwitchRegister {
    // java public static void gRegisterDateTimeSwitch(FieldCodeLex codeLex)
    pub fn gRegisterDateTimeSwitch(codeLex: &mut FieldCodeLex) {
      // java codeLex.registerPrivateSwitch(DateSwitch.HijriCalendar.getValue());
      codeLex.registerPrivateSwitch__char(DateSwitch::HijriCalendar.getValue());
      // java codeLex.registerPrivateSwitch(DateSwitch.SakaEraCalendar.getValue());
      codeLex.registerPrivateSwitch__char(DateSwitch::SakaEraCalendar.getValue());
      // java codeLex.registerPrivateSwitch(DateSwitch.UmAlQuraCalendar.getValue());
      codeLex.registerPrivateSwitch__char(DateSwitch::UmAlQuraCalendar.getValue());
      // java codeLex.registerPrivateSwitch(DateSwitch.LastUsed.getValue());
      codeLex.registerPrivateSwitch__char(DateSwitch::LastUsed.getValue());
    }

    // java public static void gRegisterHyperlinkSwitch(FieldCodeLex codeLex)
    pub fn gRegisterHyperlinkSwitch(codeLex: &mut FieldCodeLex) {
      // java codeLex.registerPrivateSwitch('l', true);
      codeLex.registerPrivateSwitch__char_boolean('l', true);
      // java codeLex.registerPrivateSwitch('m');
      codeLex.registerPrivateSwitch__char('m');
      // java codeLex.registerPrivateSwitch('n');
      codeLex.registerPrivateSwitch__char('n');
      // java codeLex.registerPrivateSwitch('o', true);
      codeLex.registerPrivateSwitch__char_boolean('o', true);
      // java codeLex.registerPrivateSwitch('t', true);
      codeLex.registerPrivateSwitch__char_boolean('t', true);
    }

    // java public static void gRegisterPageRefSwitch(FieldCodeLex codeLex)
    pub fn gRegisterPageRefSwitch(codeLex: &mut FieldCodeLex) {
      // java codeLex.registerPrivateSwitch('h');
      codeLex.registerPrivateSwitch__char('h');
      // java codeLex.registerPrivateSwitch('p');
      codeLex.registerPrivateSwitch__char('p');
    }

    // java public static void gRegisterRefSwitch(FieldCodeLex codeLex)
    pub fn gRegisterRefSwitch(codeLex: &mut FieldCodeLex) {
      // java codeLex.registerPrivateSwitch('f');
      codeLex.registerPrivateSwitch__char('f');
      // java codeLex.registerPrivateSwitch('h');
      codeLex.registerPrivateSwitch__char('h');
      // java codeLex.registerPrivateSwitch('n');
      codeLex.registerPrivateSwitch__char('n');
      // java codeLex.registerPrivateSwitch('p');
      codeLex.registerPrivateSwitch__char('p');
      // java codeLex.registerPrivateSwitch('r');
      codeLex.registerPrivateSwitch__char('r');
      // java codeLex.registerPrivateSwitch('t');
      codeLex.registerPrivateSwitch__char('t');
      // java codeLex.registerPrivateSwitch('w');
      codeLex.registerPrivateSwitch__char('w');
    }

    // java public static void gRegisterNoteRefSwitch(FieldCodeLex codeLex)
    pub fn gRegisterNoteRefSwitch(codeLex: &mut FieldCodeLex) {
      // java codeLex.registerPrivateSwitch('f');
      codeLex.registerPrivateSwitch__char('f');
      // java codeLex.registerPrivateSwitch('h');
      codeLex.registerPrivateSwitch__char('h');
      // java codeLex.registerPrivateSwitch('p');
      codeLex.registerPrivateSwitch__char('p');
    }

    // java public static void gRegisterStyleRefSwitch(FieldCodeLex codeLex)
    pub fn gRegisterStyleRefSwitch(codeLex: &mut FieldCodeLex) {
      // java codeLex.registerPrivateSwitch('s');
      codeLex.registerPrivateSwitch__char('s');
      // java codeLex.registerPrivateSwitch('n');
      codeLex.registerPrivateSwitch__char('n');
      // java codeLex.registerPrivateSwitch('r');
      codeLex.registerPrivateSwitch__char('r');
      // java codeLex.registerPrivateSwitch('w');
      codeLex.registerPrivateSwitch__char('w');
      // java codeLex.registerPrivateSwitch('p');
      codeLex.registerPrivateSwitch__char('p');
      // java codeLex.registerPrivateSwitch('t');
      codeLex.registerPrivateSwitch__char('t');
      // java codeLex.registerPrivateSwitch('*');
      codeLex.registerPrivateSwitch__char('*');
    }

    // java public static void gRegisterTCSwitch(FieldCodeLex codeLex)
    pub fn gRegisterTCSwitch(codeLex: &mut FieldCodeLex) {
      // java codeLex.registerPrivateSwitch('f', true);
      codeLex.registerPrivateSwitch__char_boolean('f', true);
      // java codeLex.registerPrivateSwitch('l', true);
      codeLex.registerPrivateSwitch__char_boolean('l', true);
      // java codeLex.registerPrivateSwitch('n');
      codeLex.registerPrivateSwitch__char('n');
    }

    // java public static void gRegisterTOCSwitch(FieldCodeLex codeLex)
    pub fn gRegisterTOCSwitch(codeLex: &mut FieldCodeLex) {
      // java codeLex.registerPrivateSwitch('a', true);
      codeLex.registerPrivateSwitch__char_boolean('a', true);
      // java codeLex.registerPrivateSwitch('b', true);
      codeLex.registerPrivateSwitch__char_boolean('b', true);
      // java codeLex.registerPrivateSwitch('c', true);
      codeLex.registerPrivateSwitch__char_boolean('c', true);
      // java codeLex.registerPrivateSwitch('f', true);
      codeLex.registerPrivateSwitch__char_boolean('f', true);
      // java codeLex.registerPrivateSwitch('h');
      codeLex.registerPrivateSwitch__char('h');
      // java codeLex.registerPrivateSwitch('l', true);
      codeLex.registerPrivateSwitch__char_boolean('l', true);
      // java codeLex.registerPrivateSwitch('n', true);
      codeLex.registerPrivateSwitch__char_boolean('n', true);
      // java codeLex.registerPrivateSwitch('o', true);
      codeLex.registerPrivateSwitch__char_boolean('o', true);
      // java codeLex.registerPrivateSwitch('p', true);
      codeLex.registerPrivateSwitch__char_boolean('p', true);
      // java codeLex.registerPrivateSwitch('s', true);
      codeLex.registerPrivateSwitch__char_boolean('s', true);
      // java codeLex.registerPrivateSwitch('d', true);
      codeLex.registerPrivateSwitch__char_boolean('d', true);
      // java codeLex.registerPrivateSwitch('t', true);
      codeLex.registerPrivateSwitch__char_boolean('t', true);
      // java codeLex.registerPrivateSwitch('u');
      codeLex.registerPrivateSwitch__char('u');
      // java codeLex.registerPrivateSwitch('w');
      codeLex.registerPrivateSwitch__char('w');
      // java codeLex.registerPrivateSwitch('x');
      codeLex.registerPrivateSwitch__char('x');
      // java codeLex.registerPrivateSwitch('z');
      codeLex.registerPrivateSwitch__char('z');
    }

    // java public static void gRegisterSEQSwitch(FieldCodeLex codeLex)
    pub fn gRegisterSEQSwitch(codeLex: &mut FieldCodeLex) {
      // java codeLex.registerPrivateSwitch('c');
      codeLex.registerPrivateSwitch__char('c');
      // java codeLex.registerPrivateSwitch('h');
      codeLex.registerPrivateSwitch__char('h');
      // java codeLex.registerPrivateSwitch('n');
      codeLex.registerPrivateSwitch__char('n');
      // java codeLex.registerPrivateSwitch('r', true);
      codeLex.registerPrivateSwitch__char_boolean('r', true);
      // java codeLex.registerPrivateSwitch('s', true);
      codeLex.registerPrivateSwitch__char_boolean('s', true);
    }

    // java public static void gRegisterIncludePictureSwitch(FieldCodeLex codeLex)
    pub fn gRegisterIncludePictureSwitch(codeLex: &mut FieldCodeLex) {
      // java codeLex.registerPrivateSwitch('c', true);
      codeLex.registerPrivateSwitch__char_boolean('c', true);
      // java codeLex.registerPrivateSwitch('d');
      codeLex.registerPrivateSwitch__char('d');
      // java codeLex.registerPrivateSwitch('z');
      codeLex.registerPrivateSwitch__char('z');
    }

}
