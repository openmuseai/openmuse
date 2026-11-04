// Java: import java.util.List
// Java: import java.util.ArrayList

// Java: public class for
pub struct r#for {
}

// Java: public class ArgUtils
pub struct ArgUtils {
}

// Java: public class for
impl r#for {
}

// Java: public class ArgUtils
impl ArgUtils {
    // Java: ArgUtils()
    pub fn ArgUtils(&self) {
    }

    // Java: static Object[] flatten(Object[] array)
    pub fn flatten(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: final List<Object> list = new ArrayList<Object>();
        // Java: if (array != null)
        {
            // Java: for (Object o : array)
            {
                // Java: if (o instanceof Object[])
                {
                    // Java: for (Object oR : flatten((Object[]) o))
                    {
                        // Java: list.add(oR);
                    }
                }
                // Java: else
                {
                    // Java: list.add(o);
                }
            }
        }
        // Java: return list.toArray();
        todo!()
    }

}
