#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class ModifyWorldTransform extends EMFRecord
pub struct ModifyWorldTransform {
    // Java field: int MWT_IDENTITY
    // Java field: int MWT_LEFTMULTIPLY
    // Java field: int MWT_RIGHTMULTIPLY
    // Java field: int MWT_SET
    // Java field: Matrix transform
    transform: Option<i32>,
    // Java field: int mode
    mode: i32,
}

pub(crate) const MWT_IDENTITY: i32 = 1;

pub(crate) const MWT_LEFTMULTIPLY: i32 = 2;

pub(crate) const MWT_RIGHTMULTIPLY: i32 = 3;

pub(crate) const MWT_SET: i32 = 4;

impl ModifyWorldTransform {
    // Java: ModifyWorldTransform()
    pub fn new() -> Self {
        // Java: super();
        ModifyWorldTransform {
            transform: None,
            mode: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: ModifyWorldTransform tag = new ModifyWorldTransform();
        // Java: tag.transform = emf.readMatrix();
        // Java: tag.mode = emf.readUnsignedInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: GdiBoundContext context = renderer.getDC().getContext();
        // Java: if (this.mode == 1)
        {
        // Java: context.resetWorldTransform();
        // Java: context.resetTransform();
        }
        // Java: else if (this.mode == 4)
        {
        // Java: context.setWorldTransform(this.transform);
        // Java: context.resetTransform();
        }
        // Java: else if (this.mode == 2)
        {
        // Java: context.worldTransformPostConcat(this.transform);
        // Java: context.transform(this.transform);
        }
        // Java: else
        {
        // Java: context.worldTransformPreConcat(this.transform);
        // Java: context.transform(this.transform);
        }
    }

}
