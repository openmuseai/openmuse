// Java: import java.io.Serializable
// Java: import java.util.Collection
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import java.util.Set
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;

// Java field: long serialVersionUID

// Java: public class TransformerMap implements NumberTransformer, Serializable
pub struct TransformerMap {
    // Java field: NumberTransformer defaultTransformer
    defaultTransformer: Option<Box<dyn std::any::Any>>,
    // Java field: Map<Class<?>, NumberTransformer> map
    pub(crate) map: Box<dyn std::any::Any>,
}

// Java: public class TransformerMap implements NumberTransformer, Serializable
impl TransformerMap {
    // Java: TransformerMap()
    pub fn TransformerMap(&self) {
        // Java: map = new HashMap<Class<?>, NumberTransformer>();
        // Java: defaultTransformer = new DefaultTransformer();
    }

    // Java: boolean containsClass(Class<?> key)
    pub fn containsClass(&self) -> bool {
        // Java: return map.containsKey(key);
        todo!()
    }

    // Java: boolean containsTransformer(NumberTransformer value)
    pub fn containsTransformer(&self) -> bool {
        // Java: return map.containsValue(value);
        todo!()
    }

    // Java: NumberTransformer getTransformer(Class<?> key)
    pub fn getTransformer(&self) -> Box<dyn std::any::Any> {
        // Java: return map.get(key);
        todo!()
    }

    // Java: NumberTransformer putTransformer(Class<?> key, NumberTransformer transformer)
    pub fn putTransformer(&self) -> Box<dyn std::any::Any> {
        // Java: return map.put(key, transformer);
        todo!()
    }

    // Java: NumberTransformer removeTransformer(Class<?> key)
    pub fn removeTransformer(&self) -> Box<dyn std::any::Any> {
        // Java: return map.remove(key);
        todo!()
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: map.clear();
    }

    // Java: public Set<Class<?>> classes()
    pub fn classes(&self) -> Box<dyn std::any::Any> {
        // Java: return map.keySet();
        todo!()
    }

    // Java: Collection<NumberTransformer> transformers()
    pub fn transformers(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: return map.values();
        todo!()
    }

    // Java: double transform(Object o)
    pub fn transform(&self) -> f64 {
        // Java: double value = Double.NaN;
        // Java: if (o instanceof Number || o instanceof String)
        {
            // Java: value = defaultTransformer.transform(o);
        }
        // Java: else
        {
            // Java: NumberTransformer trans = getTransformer(o.getClass());
            // Java: if (trans != null)
            {
                // Java: value = trans.transform(o);
            }
        }
        // Java: return value;
        todo!()
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self) -> bool {
        // Java: if (this == other)
        {
            // Java: return true;
        }
        // Java: if (other instanceof TransformerMap)
        {
            // Java: TransformerMap rhs = (TransformerMap) other;
            // Java: if (! defaultTransformer.equals(rhs.defaultTransformer))
            {
                // Java: return false;
            }
            // Java: if (map.size() != rhs.map.size())
            {
                // Java: return false;
            }
            // Java: for (Map.Entry<Class<?>, NumberTransformer> entry : map.entrySet())
            {
                // Java: if (! entry.getValue().equals(rhs.map.get(entry.getKey())))
                {
                    // Java: return false;
                }
            }
            // Java: return true;
        }
        // Java: return false;
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int hash = defaultTransformer.hashCode();
        // Java: for (NumberTransformer t : map.values())
        {
            // Java: hash = hash * 31 + t.hashCode();
        }
        // Java: return hash;
        todo!()
    }

}


