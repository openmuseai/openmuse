// Java: public class Scene3d
pub struct Scene3d {
    // Java field: float mXRotate
    pub(crate) mXRotate: f32,
    // Java field: float mYRotate
    pub(crate) mYRotate: f32,
    // Java field: float mZRotate
    pub(crate) mZRotate: f32,
    // Java field: float mPersp
    pub(crate) mPersp: f32,
}

// Java: public class Scene3d
impl Scene3d {
    // Java: Scene3d()
    pub fn new() -> Self {
      // Java: super();
      Scene3d { mXRotate: 0.0, mYRotate: 0.0, mZRotate: 0.0, mPersp: 0.0 }
    }

}
