// Java: import android.text.TextPaint
use crate::android::text::TextPaint;

// Java: public class CharacterStyle
pub struct CharacterStyle {
}

// Java: public class Passthrough
pub struct Passthrough {
    // Java field: CharacterStyle mStyle
    pub(crate) mStyle: Arc<Mutex<CharacterStyle>>,
}

// Java: public class CharacterStyle
impl CharacterStyle {
    // Java: CharacterStyle()
    pub fn CharacterStyle(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: abstract void updateDrawState(TextPaint var1)
    pub fn updateDrawState(&self, TextPaint var1) {
    // TODO_RUST_CODE
    }

    // Java: static CharacterStyle wrap(CharacterStyle cs)
    pub fn wrap(&self, CharacterStyle cs) -> CharacterStyle {
      // Java: return (CharacterStyle)(cs instanceof MetricAffectingSpan ? new MetricAffectingSpan.Passthrough((MetricAffectingSpan)cs) : new Passthrough(cs));
      // TODO_RUST_CODE
    }

    // Java: CharacterStyle getUnderlying()
    pub fn getUnderlying(&self) -> CharacterStyle {
      // Java: return this;
      // TODO_RUST_CODE
    }

}

// Java: public class Passthrough
impl Passthrough {
    // Java: Passthrough(CharacterStyle cs)
    pub fn Passthrough(&self, CharacterStyle cs) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.mStyle = cs;
         // TODO_RUST_CODE
    }

    // Java: void updateDrawState(TextPaint tp)
    pub fn updateDrawState(&self, TextPaint tp) {
         // Java: this.mStyle.updateDrawState(tp);
         // TODO_RUST_CODE
    }

    // Java: CharacterStyle getUnderlying()
    pub fn getUnderlying(&self) -> CharacterStyle {
         // Java: return this.mStyle.getUnderlying();
         // TODO_RUST_CODE
    }

}
