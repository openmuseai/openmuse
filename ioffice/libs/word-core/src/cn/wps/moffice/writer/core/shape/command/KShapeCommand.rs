// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.transaction.CommandBase
use util::moffice::transaction::CommandBase::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public class KShapeCommand extends CommandBase
#[derive(Default)]
pub struct KShapeCommand {
    // java CommandBase super_
    pub(crate) super_: CommandBase,
    // java Shape mShape;
    pub(crate) mShape: Option<Shape>,
    // java int mPropID;
    pub(crate) mPropID: i32,
    // java Object mOld;
    pub(crate) mOld: Option<Box<dyn Object>>,
    // java Object mNew;
    pub(crate) mNew: Option<Box<dyn Object>>,
}

// java public class KShapeCommand extends CommandBase
impl KShapeCommand {
    // java public  KShapeCommand(Shape shape, int propID, Object oldProp, Object newProp)
    pub fn KShapeCommand(shape: &Shape, propID: i32, oldProp: &dyn Object, newProp: &dyn Object) -> Self {
      // java this.mShape = shape;
      Self { super_: CommandBase::new(), mShape: shape, mPropID: propID, mOld: oldProp, mNew: newProp }
      // java this.mPropID = propID;
      // java this.mOld = oldProp;
      // java this.mNew = newProp;
    }

    // java public void execute()
    pub fn execute(&mut self) {
      // java this.execute(this.mNew);
      self.execute__object(&self.mNew);
    }

    // java public void unexecute()
    pub fn unexecute(&mut self) {
      // java this.execute(this.mOld);
      self.execute__object(&self.mOld);
    }

    // java protected void execute(Object prop)
    pub fn execute__object(&self, prop: &dyn Object) {
      // java this.mShape.getProperty().setValue(this.mPropID, prop);
      self.mShape.getProperty().setValue(self.mPropID, prop);
    }

}

impl std::ops::Deref for KShapeCommand {
    type Target = CommandBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KShapeCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
