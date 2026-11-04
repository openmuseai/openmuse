use crate::org::jaxen::Object;

// Java: public class SingletonList extends AbstractList
pub struct SingletonList {
    // Java field: Object element
    pub(crate) element: Object,
}

// Java: public class SingletonList extends AbstractList
impl SingletonList {
    // Java: SingletonList(Object element)
    pub fn new(element: Object) -> Self {
        // Java: super();
        // Java: this.element = element;
        SingletonList { element }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return 1;
        1
    }

    // Java: Object get(int index)
    pub fn get(&self, index: i32) -> &Object {
        // Java: if (index == 0)
        if index == 0 {
            // Java: return this.element;
            &self.element
        }
        // Java: else
        else {
            // Java: throw new IndexOutOfBoundsException(index + " != 0");
            panic!("{} != 0", index);
        }
    }
}
