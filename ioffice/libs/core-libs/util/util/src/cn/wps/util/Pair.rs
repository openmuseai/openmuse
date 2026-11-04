use std::any::Any;
use std::sync::Arc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class Pair
pub struct Pair {
    // Java field: F first
    pub first: Element,
    // Java field: S second
    pub second: Element,
}

// Java: public class Pair
impl Pair {
    // Java: Pair(F first, S second)
    pub fn new(first: Element, second: Element) -> Self {
      // Java: super();
      // Java: this.first = first;
      // Java: this.second = second;
      Pair { first, second }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Pair) -> bool {
      // Java: if (o == this)
      if std::ptr::eq(self, o)
      {
         // Java: return true;
         return true;
      }
      // Java: else if (!(o instanceof Pair))
      {
         // Java: return false;
      }
      // Java: else
      {
         // Java: Pair<F, S> other;
         // Java: try
         {
            // Java: other = (Pair)o;
         }
         // Java: catch (ClassCastException var4)
         {
            // Java: return false;
            return false;
         }
         // Java: return this.first.equals(other.first) && this.second.equals(other.second);
         return are_elements_equal(&self.first, &o.first)
             && are_elements_equal(&self.second, &o.second);
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int result = 17;
      let mut result: i32 = 17;
      // Java: result = 31 * result + this.first.hashCode();
      result = 31i32.wrapping_mul(result).wrapping_add(element_hash(&self.first));
      // Java: result = 31 * result + this.second.hashCode();
      result = 31i32.wrapping_mul(result).wrapping_add(element_hash(&self.second));
      // Java: return result;
      result
    }

    // Java: static public static <A, B> Pair<A, B> create(A a, B b)
    pub fn create(a: Element, b: Element) -> Pair {
      // Java: return new Pair<A, B>(a, b);
      Pair::new(a, b)
    }
}

fn are_elements_equal(a: &Element, b: &Element) -> bool {
    match (a, b) {
        (Some(va), Some(vb)) => Arc::ptr_eq(va, vb),
        (None, None) => true,
        _ => false,
    }
}

fn element_hash(e: &Element) -> i32 {
    match e {
        Some(v) => {
            let mut hasher = DefaultHasher::new();
            Arc::as_ptr(v).hash(&mut hasher);
            hasher.finish() as i32
        }
        None => 0,
    }
}
