// Java: public class FlyweightText extends AbstractText
pub struct FlyweightText {
    // Java field: String text
    pub(crate) text: Option<String>,
}

// Java: public class FlyweightText extends AbstractText
impl FlyweightText {
    // Java: FlyweightText()
    pub fn new() -> Self {
      // Java: super();
      todo!();
    }

    // Java: FlyweightText(String text)
    pub fn new_1(text: &str) -> Self {
      // Java: super(text);
      FlyweightText { text: Some(text.to_string()) }
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.text;
      self.text.clone().unwrap_or_default()
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: &str) {
      // Java: this.text = text;
      self.text = Some(text.to_string());
    }

}
