#![allow(non_snake_case, dead_code)]
use super::ColorChoice::ColorChoice;

// Java: public class Duotone
#[derive(Default)]
pub struct Duotone {
    // Java: ArrayList<ColorChoice> mColorChoice = new ArrayList();
    pub mColorChoice: Vec<ColorChoice>,
}
impl Duotone {
    // Java: implicit Duotone()
    pub fn new() -> Self { Self::default() }
    // Java: public void addColor(ColorChoice colorChoice)
    pub fn addColor(&mut self, colorChoice: ColorChoice) { self.mColorChoice.push(colorChoice); }
    // Java: public ColorChoice getColorChoiceByIndex(int index)
    pub fn getColorChoiceByIndex(&self, index: i32) -> &ColorChoice {
        &self.mColorChoice[index as usize]
    }
    // Java: public int size()
    pub fn size(&self) -> i32 { self.mColorChoice.len() as i32 }
}
