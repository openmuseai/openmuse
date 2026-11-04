#![allow(dead_code, non_snake_case, unused_variables)]

// Java: import android.graphics.Matrix
// (android.graphics.Matrix not available in Rust)
// Java: import java.util.Vector
// (mapped to Vec<T>)

use super::KPdfObject::KPdfObject;

pub type KPdfDevice = ();

// Java: public class KPdfFormXObject extends KPdfStream
pub struct KPdfFormXObject {
    // Java field: Vector<KPdfObject> mResources
    mResources: Vec<KPdfObject>,
}

// Java: public class KPdfFormXObject extends KPdfStream
#[allow(non_snake_case)]
impl KPdfFormXObject {
    // Java: KPdfFormXObject(KPdfDevice device)
    pub fn new(_device: &KPdfDevice) -> Self {
      // Java: super();
      let mut this = Self { mResources: Vec::new() };
      // Java: device.getResources(this.mResources, false);
      // Java: KStream data = device.content();
      // Java: this.setData(data);
      // Java: this.insertName("Type", "XObject");
      // Java: this.insertName("Subtype", "Form");
      // Java: this.insert("BBox", device.copyMediaBox());
      // Java: this.insert("Resources", device.getResourceDict());
      // Java: Matrix matrix = device.getInitMatrix();
      // Java: if (matrix != null && !matrix.isIdentity())
      {
         // Java: Matrix inverse = new Matrix();
         // Java: matrix.invert(inverse);
         // Java: this.insert("Matrix", KPdfUtils.matrixToArray(inverse));
      }
      // Java: KPdfDict group = new KPdfDict("Group");
      // Java: group.insertName("S", "Transparency");
      // Java: group.insert((String)"I", new KPdfBool(true));
      // Java: this.insert("Group", group);
      this
    }

    // Java: void getResources(Vector<KPdfObject> resources)
    pub fn getResources(&self, _resources: &mut Vec<KPdfObject>) {
      // Java: this.getResourcesHelper(this.mResources, resources);
    }

}
