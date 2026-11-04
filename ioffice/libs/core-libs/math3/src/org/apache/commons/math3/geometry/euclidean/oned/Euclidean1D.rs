// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathUnsupportedOperationException
use crate::org::apache::commons::math3::exception::MathUnsupportedOperationException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

use std::sync::OnceLock;

// Java field: long serialVersionUID
#[allow(non_upper_case_globals)]
pub(crate) const serialVersionUID: i64 = -1178039568877797126;

// Java field: Euclidean1D INSTANCE
static INSTANCE: OnceLock<Euclidean1D> = OnceLock::new();

// Java: public class implements implements a one

// Java: public class Euclidean1D
#[derive(Debug, Clone)]
pub struct Euclidean1D {
}

// Java: public class LazyHolder

// Java: public class NoSubSpaceException
#[derive(Debug)]
pub struct NoSubSpaceException {
    // Java field: long serialVersionUID
}

#[allow(non_upper_case_globals)]
const noSubSpaceSerialVersionUID: i64 = 20140225;

// Java: public class Euclidean1D
#[allow(non_snake_case)]
impl Euclidean1D {
    // Java: Euclidean1D()
    fn new() -> Self {
        Euclidean1D {}
    }

    // Java: static Euclidean1D getInstance()
    pub fn getInstance() -> &'static Euclidean1D {
        // Java: return LazyHolder.INSTANCE;
        INSTANCE.get_or_init(|| Euclidean1D::new())
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return 1;
        1
    }

    // Java: Space getSubSpace()
    pub fn getSubSpace(&self) -> Box<dyn std::any::Any> {
        // Java: throw new NoSubSpaceException();
        panic!("NoSubSpaceException: NOT_SUPPORTED_IN_DIMENSION_N 1");
    }

    // Java: Object readResolve()
    fn readResolve(&self) -> &'static Euclidean1D {
        // Java: // return the singleton instance
        // Java: return LazyHolder.INSTANCE;
        Euclidean1D::getInstance()
    }
}

impl Space::Space for Euclidean1D {
    fn getDimension(&self) -> i32 {
        1
    }

    fn getSubSpace(&self) -> Box<dyn std::any::Any> {
        panic!("NoSubSpaceException: NOT_SUPPORTED_IN_DIMENSION_N 1");
    }
}

// Java: public class NoSubSpaceException
#[allow(non_snake_case)]
impl NoSubSpaceException {
    // Java: NoSubSpaceException()
    pub fn new() -> Self {
        // Java: super(LocalizedFormats.NOT_SUPPORTED_IN_DIMENSION_N, 1);
        NoSubSpaceException {}
    }
}

impl std::fmt::Display for NoSubSpaceException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoSubSpaceException: NOT_SUPPORTED_IN_DIMENSION_N 1")
    }
}

impl std::error::Error for NoSubSpaceException {}
