#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathUnsupportedOperationException
use crate::org::apache::commons::math3::exception::MathUnsupportedOperationException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

use std::sync::OnceLock;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 6178746691783484374;
// Java field: Sphere1D INSTANCE
static INSTANCE: OnceLock<Sphere1D> = OnceLock::new();

// Java: public class implements implements a one
pub struct implements {
}

// Java: public class Sphere1D
pub struct Sphere1D {
}

// Java: public class LazyHolder
pub struct LazyHolder {
}

// Java: public class NoSubSpaceException
pub struct NoSubSpaceException {
}

// Java: public class implements implements a one
impl implements {
}

// Java: public class Sphere1D
impl Sphere1D {
    // Java: Sphere1D()
    pub fn new() -> Self {
        Sphere1D {}
    }

    // Java: static Sphere1D getInstance()
    pub fn getInstance() -> &'static Sphere1D {
        // Java: return LazyHolder.INSTANCE;
        INSTANCE.get_or_init(|| Sphere1D::new())
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return 1;
        1
    }

    // Java: Space getSubSpace()
    pub fn getSubSpace(&self) -> Box<dyn std::any::Any> {
        // Java: throw new NoSubSpaceException();
        panic!("NoSubSpaceException");
    }

    // Java: Object readResolve()
    pub fn readResolve(&self) -> &'static Sphere1D {
        // Java: // return the singleton instance
        // Java: return LazyHolder.INSTANCE;
        Sphere1D::getInstance()
    }

}

// Java: public class LazyHolder
impl LazyHolder {
}

// Java: public class NoSubSpaceException
impl NoSubSpaceException {
    // Java: NoSubSpaceException()
    pub fn new() -> Self {
        // Java: super(LocalizedFormats.NOT_SUPPORTED_IN_DIMENSION_N, 1);
        NoSubSpaceException {}
    }

}
