// java public static final int TYPE_EQU = 0;
pub(crate) const TYPE_EQU: i32 = 0;
// java public static final int TYPE_OP = 1;
pub(crate) const TYPE_OP: i32 = 1;
// java public static final int TYPE_EDITION = 2;
pub(crate) const TYPE_EDITION: i32 = 2;

// java public abstract class Item
#[derive(Default)]
pub struct Item {
}

// java public abstract class Item
impl Item {
    // java public  Item()
    pub fn Item() -> Self {
      // java super();
      Self {}
    }

    // java public abstract int type()
    pub fn r#type(&mut self) -> i32 {
      panic!("Item::type not implemented");
    }

    // java public abstract boolean cal()
    pub fn cal(&mut self) -> bool {
      panic!("Item::cal not implemented");
    }

}
