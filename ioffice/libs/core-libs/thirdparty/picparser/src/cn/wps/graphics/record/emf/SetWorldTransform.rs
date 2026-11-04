#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SetWorldTransform extends EMFRecord
pub struct SetWorldTransform {
    // Java field: Matrix transform
    transform: Option<i32>,
}

impl SetWorldTransform {
    // Java: SetWorldTransform()
    pub fn new() -> Self {
        // Java: super();
        SetWorldTransform {
            transform: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SetWorldTransform tag = new SetWorldTransform();
        // Java: tag.transform = emf.readMatrix();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: GdiBoundContext context = renderer.getContext();
        // Java: if (renderer.beginningPath())
        {
        // Java: context.setWorldTransform(this.transform);
        }
        // Java: else
        {
        // Java: context.setWorldTransform(this.transform);
        // Java: context.resetTransformation();
        }
    }

}
