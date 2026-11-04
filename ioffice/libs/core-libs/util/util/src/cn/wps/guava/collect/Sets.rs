#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;

// Java: import java.util.Set

// Java: public class Sets
pub struct Sets;

// Java: public class Sets
impl Sets {
    // Java: Sets()
    // Java: super();

    // Java: static boolean equalsImpl(Set<?> s, Object object)
    pub fn equalsImpl(_s: &dyn Any, _object: &dyn Any) -> bool {
        // Java: if (s == object)
        {
            // Java: return true;
        }
        // Java: else if (object instanceof Set)
        {
            // Java: Set<?> o = (Set)object;
            // Java: try
            {
                // Java: return s.size() == o.size() && s.containsAll(o);
            }
            // Java: catch (NullPointerException var4)
            {
                // Java: return false;
            }
            // Java: catch (ClassCastException var5)
            {
                // Java: return false;
            }
        }
        // Java: else
        {
            // Java: return false;
            false
        }
    }

    // Java: static int hashCodeImpl(Set<?> s)
    pub fn hashCodeImpl(_s: &[Option<Box<dyn Any>>]) -> i32 {
        // Java: int hashCode = 0;
        let mut hashCode: i32 = 0;
        // Java: for(Object o : s)
        for o in _s.iter() {
            // Java: hashCode += o != null ? o.hashCode() : 0;
            hashCode += if o.is_some() { 0 } else { 0 };
            // Java: hashCode = ~(~hashCode);
            hashCode = !(!hashCode);
        }
        // Java: return hashCode;
        hashCode
    }
}
