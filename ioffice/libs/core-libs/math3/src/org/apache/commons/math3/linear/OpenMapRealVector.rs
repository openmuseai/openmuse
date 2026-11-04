// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.OpenIntToDoubleHashMap
use crate::org::apache::commons::math3::util::OpenIntToDoubleHashMap;
// Java: import org.apache.commons.math3.util.OpenIntToDoubleHashMap.Iterator
use crate::org::apache::commons::math3::util::OpenIntToDoubleHashMap::Iterator;

// Java: public class implements implements the
pub struct implements {
}

// Java: public class OpenMapRealVector extends SparseRealVector
pub struct OpenMapRealVector {
}

// Java: public class OpenMapEntry extends Entry
pub struct OpenMapEntry {
    // Java field: Iterator iter
    pub(crate) iter: Box<dyn std::any::Any>,
}

// Java: public class to
pub struct to {
}

// Java: public class OpenMapSparseIterator
pub struct OpenMapSparseIterator {
    // Java field: Iterator iter
    pub(crate) iter: Box<dyn std::any::Any>,
    // Java field: Entry current
    pub(crate) current: Box<dyn std::any::Any>,
}

// Java: public class implements implements the
impl implements {
}


// Java: public class OpenMapRealVector extends SparseRealVector
impl OpenMapRealVector {
}

// Java: public class OpenMapEntry extends Entry
impl OpenMapEntry {
    // Java: OpenMapEntry(Iterator iter)
    pub fn OpenMapEntry(&self) {
            // Java: this.iter = iter;
    }

    // Java: double getValue()
    pub fn getValue(&self) -> f64 {
            // Java: return iter.value();
        todo!()
    }

    // Java: void setValue(double value)
    pub fn setValue(&self) {
            // Java: entries.put(iter.key(), value);
    }

    // Java: int getIndex()
    pub fn getIndex(&self) -> i32 {
            // Java: return iter.key();
        todo!()
    }

}

// Java: public class to
impl to {
}

// Java: public class OpenMapSparseIterator
impl OpenMapSparseIterator {
    // Java: OpenMapSparseIterator()
    pub fn OpenMapSparseIterator(&self) {
            // Java: iter = entries.iterator();
            // Java: current = new OpenMapEntry(iter);
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
            // Java: return iter.hasNext();
        todo!()
    }

    // Java: Entry next()
    pub fn next(&self) -> Box<dyn std::any::Any> {
            // Java: iter.advance();
            // Java: return current;
        todo!()
    }

    // Java: void remove()
    pub fn remove(&self) {
            // Java: throw new UnsupportedOperationException("Not supported");
    }

}
