#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;
// Java: import org.apache.commons.math3.geometry.spherical.oned.Sphere1D
use crate::org::apache::commons::math3::geometry::spherical::oned::Sphere1D;

use std::sync::OnceLock;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 6016936143773733296;
// Java field: Sphere2D INSTANCE
static INSTANCE: OnceLock<Sphere2D> = OnceLock::new();

// Java: public class implements implements a two
pub struct implements {
}

// Java: public class Sphere2D
pub struct Sphere2D {
}

// Java: public class LazyHolder
pub struct LazyHolder {
}

// Java: public class implements implements a two
impl implements {
}

// Java: public class Sphere2D
impl Sphere2D {
    // Java: Sphere2D()
    pub fn new() -> Self {
        Sphere2D {}
    }

    // Java: static Sphere2D getInstance()
    pub fn getInstance() -> &'static Sphere2D {
        // Java: return LazyHolder.INSTANCE;
        INSTANCE.get_or_init(|| Sphere2D::new())
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return 2;
        2
    }

    // Java: Sphere1D getSubSpace()
    pub fn getSubSpace(&self) -> &'static Sphere1D::Sphere1D {
        // Java: return Sphere1D.getInstance();
        Sphere1D::Sphere1D::getInstance()
    }

    // Java: Object readResolve()
    pub fn readResolve(&self) -> &'static Sphere2D {
        // Java: // return the singleton instance
        // Java: return LazyHolder.INSTANCE;
        Sphere2D::getInstance()
    }

}

// Java: public class LazyHolder
impl LazyHolder {
}
