use crate::cn::wps::moffice::drawing::graphics::BlipFill::BlipFill;

// Java: public class PatternFill extends BlipFill
pub struct PatternFill {
    pub base: BlipFill,
}

impl std::ops::Deref for PatternFill {
    type Target = BlipFill;
    fn deref(&self) -> &BlipFill { &self.base }
}

impl std::ops::DerefMut for PatternFill {
    fn deref_mut(&mut self) -> &mut BlipFill { &mut self.base }
}

// Java: public class PatternFill extends BlipFill
impl PatternFill {
    // Java: PatternFill(int mediaID)
    pub fn new_with_media_id(mediaID: i32) -> Self {
      // Java: super(mediaID, 1);
      PatternFill {
          base: BlipFill::new_with_media_id_and_type(mediaID, 1),
      }
    }

    // Java: PatternFill()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: merged into the PatternFill struct literal
      // Java: this.setFillType(1);
      // LLM JavaLine Aggregation: merged into setFillType call below
      let mut result = PatternFill {
          base: BlipFill::new(),
      };
      result.base.base.setFillType(1);
      result
    }

}
