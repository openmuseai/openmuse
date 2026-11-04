// Java: import android.text.TextPaint
pub struct TextPaint;

// Java: public class CharacterStyle
pub trait CharacterStyleTrait {
    fn updateDrawState(&self, var1: &mut TextPaint);
    fn getUnderlying(&self) -> &dyn CharacterStyleTrait;
}

pub struct CharacterStyleBase;

// Java: public class CharacterStyle
impl CharacterStyleBase {
    // Java: CharacterStyle()
    pub fn new() -> Self {
      // Java: super();
      CharacterStyleBase
    }

    // Java: abstract void updateDrawState(TextPaint var1)

    // Java: static CharacterStyle wrap(CharacterStyle cs)
    pub fn wrap(cs: Box<dyn CharacterStyleTrait>) -> Box<dyn CharacterStyleTrait> {
      // Java: return (CharacterStyle)(cs instanceof MetricAffectingSpan ? new MetricAffectingSpan.Passthrough((MetricAffectingSpan)cs) : new Passthrough(cs));
      Box::new(Passthrough::new(cs))
    }

    // Java: CharacterStyle getUnderlying()
    pub fn getUnderlying(&self) -> &Self {
      // Java: return this;
      self
    }

}

// Java: public class Passthrough
pub struct Passthrough {
    // Java field: CharacterStyle mStyle
    pub(crate) mStyle: Box<dyn CharacterStyleTrait>,
}

// Java: public class Passthrough
impl Passthrough {
    // Java: Passthrough(CharacterStyle cs)
    pub fn new(cs: Box<dyn CharacterStyleTrait>) -> Self {
         // Java: super();
         // Java: this.mStyle = cs;
         Passthrough { mStyle: cs }
    }

}

impl CharacterStyleTrait for Passthrough {
    // Java: void updateDrawState(TextPaint tp)
    fn updateDrawState(&self, tp: &mut TextPaint) {
         // Java: this.mStyle.updateDrawState(tp);
         self.mStyle.updateDrawState(tp);
    }

    // Java: CharacterStyle getUnderlying()
    fn getUnderlying(&self) -> &dyn CharacterStyleTrait {
         // Java: return this.mStyle.getUnderlying();
         self.mStyle.getUnderlying()
    }
}
