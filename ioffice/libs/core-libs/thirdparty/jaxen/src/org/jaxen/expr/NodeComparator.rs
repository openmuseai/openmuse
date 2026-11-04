// Java: import java.util.Comparator
// Java: import java.util.Iterator
// Java: import org.jaxen.Navigator
use crate::org::jaxen::Navigator::Navigator;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::Object;

// Java: public class NodeComparator implements Comparator
pub struct NodeComparator {
    // Java field: Navigator navigator
    pub(crate) navigator: Option<Box<dyn Navigator>>,
}

// Java: public class NodeComparator implements Comparator
impl NodeComparator {
    // Java: NodeComparator(Navigator navigator)
    pub fn new(navigator: Option<Box<dyn Navigator>>) -> Self {
      // Java: super();
      // Java: this.navigator = navigator;
      NodeComparator { navigator }
    }

    // Java: int compare(Object o1, Object o2)
    pub fn compare(&self, _o1: &Object, _o2: &Object) -> i32 {
      // Java: if (this.navigator == null)
      // Java: return 0;
      // Java: else if (this.isNonChild(o1) && this.isNonChild(o2))
      // Java: try
      // Java: Object p1 = this.navigator.getParentNode(o1);
      // Java: Object p2 = this.navigator.getParentNode(o2);
      // Java: if (p1 == p2)
      // Java: if (this.navigator.isNamespace(o1) && this.navigator.isAttribute(o2))
      // Java: return -1;
      // Java: if (this.navigator.isNamespace(o2) && this.navigator.isAttribute(o1))
      // Java: return 1;
      // Java: return this.compare(p1, p2);
      // Java: catch (UnsupportedAxisException var9)
      // Java: return 0;
      // Java: else
      // Java: try
      // Java: int depth1 = this.getDepth(o1);
      // Java: int depth2 = this.getDepth(o2);
      // Java: Object a1 = o1;
      // Java: Object a2;
      // Java: for(a2 = o2; depth1 > depth2; --depth1)
      // Java: a1 = this.navigator.getParentNode(a1);
      // Java: if (a1 == o2)
      // Java: return 1;
      // Java: else
      // Java: while(depth2 > depth1)
      // Java: a2 = this.navigator.getParentNode(a2);
      // Java: --depth2;
      // Java: if (a2 == o1)
      // Java: return -1;
      // Java: else
      // Java: while(true)
      // Java: Object p1 = this.navigator.getParentNode(a1);
      // Java: Object p2 = this.navigator.getParentNode(a2);
      // Java: if (p1 == p2)
      // Java: return this.compareSiblings(a1, a2);
      // Java: a1 = p1;
      // Java: a2 = p2;
      // Java: catch (UnsupportedAxisException var10)
      // Java: return 0;
      todo!("NodeComparator::compare")
    }

    // Java: boolean isNonChild(Object o)
    fn isNonChild(&self, _o: &Object) -> bool {
      // Java: return this.navigator.isAttribute(o) || this.navigator.isNamespace(o);
      todo!("NodeComparator::isNonChild")
    }

    // Java: int compareSiblings(Object sib1, Object sib2)
    fn compareSiblings(&self, _sib1: &Object, _sib2: &Object) -> Result<i32, UnsupportedAxisException> {
      // Java: Iterator following = this.navigator.getFollowingSiblingAxisIterator(sib1);
      // Java: while(following.hasNext())
      // Java: Object next = following.next();
      // Java: if (next.equals(sib2))
      // Java: return -1;
      // Java: return 1;
      todo!("NodeComparator::compareSiblings")
    }

    // Java: int getDepth(Object o)
    fn getDepth(&self, _o: &Object) -> Result<i32, UnsupportedAxisException> {
      // Java: int depth = 0;
      // Java: for(Object parent = o; (parent = this.navigator.getParentNode(parent)) != null; ++depth)
      // Java: return depth;
      todo!("NodeComparator::getDepth")
    }
}
