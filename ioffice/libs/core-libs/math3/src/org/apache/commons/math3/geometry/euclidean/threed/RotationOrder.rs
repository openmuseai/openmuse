#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

use super::Vector3D::Vector3D;
use std::sync::OnceLock;

// Java field: RotationOrder XYZ
static XYZ: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder XZY
static XZY: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder YXZ
static YXZ: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder YZX
static YZX: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder ZXY
static ZXY: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder ZYX
static ZYX: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder XYX
static XYX: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder XZX
static XZX: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder YXY
static YXY: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder YZY
static YZY: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder ZXZ
static ZXZ: OnceLock<RotationOrder> = OnceLock::new();
// Java field: RotationOrder ZYZ
static ZYZ: OnceLock<RotationOrder> = OnceLock::new();

// Java: public class is
pub struct is {
}

// Java: public class RotationOrder
pub struct RotationOrder {
    // Java field: String name
    pub(crate) name: String,
    // Java field: Vector3D a1
    pub(crate) a1: Vector3D,
    // Java field: Vector3D a2
    pub(crate) a2: Vector3D,
    // Java field: Vector3D a3
    pub(crate) a3: Vector3D,
}

// Java: public class is
impl is {
}

// Java: new RotationOrder("XYZ", Vector3D.PLUS_I, Vector3D.PLUS_J, Vector3D.PLUS_K)
fn get_XYZ() -> &'static RotationOrder {
    XYZ.get_or_init(|| RotationOrder::new("XYZ", Vector3D::new(1.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0), Vector3D::new(0.0, 0.0, 1.0)))
}
// Java: new RotationOrder("XZY", Vector3D.PLUS_I, Vector3D.PLUS_K, Vector3D.PLUS_J)
fn get_XZY() -> &'static RotationOrder {
    XZY.get_or_init(|| RotationOrder::new("XZY", Vector3D::new(1.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(0.0, 1.0, 0.0)))
}
// Java: new RotationOrder("YXZ", Vector3D.PLUS_J, Vector3D.PLUS_I, Vector3D.PLUS_K)
fn get_YXZ() -> &'static RotationOrder {
    YXZ.get_or_init(|| RotationOrder::new("YXZ", Vector3D::new(0.0, 1.0, 0.0), Vector3D::new(1.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0)))
}
// Java: new RotationOrder("YZX", Vector3D.PLUS_J, Vector3D.PLUS_K, Vector3D.PLUS_I)
fn get_YZX() -> &'static RotationOrder {
    YZX.get_or_init(|| RotationOrder::new("YZX", Vector3D::new(0.0, 1.0, 0.0), Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(1.0, 0.0, 0.0)))
}
// Java: new RotationOrder("ZXY", Vector3D.PLUS_K, Vector3D.PLUS_I, Vector3D.PLUS_J)
fn get_ZXY() -> &'static RotationOrder {
    ZXY.get_or_init(|| RotationOrder::new("ZXY", Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(1.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0)))
}
// Java: new RotationOrder("ZYX", Vector3D.PLUS_K, Vector3D.PLUS_J, Vector3D.PLUS_I)
fn get_ZYX() -> &'static RotationOrder {
    ZYX.get_or_init(|| RotationOrder::new("ZYX", Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(0.0, 1.0, 0.0), Vector3D::new(1.0, 0.0, 0.0)))
}
// Java: new RotationOrder("XYX", Vector3D.PLUS_I, Vector3D.PLUS_J, Vector3D.PLUS_I)
fn get_XYX() -> &'static RotationOrder {
    XYX.get_or_init(|| RotationOrder::new("XYX", Vector3D::new(1.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0), Vector3D::new(1.0, 0.0, 0.0)))
}
// Java: new RotationOrder("XZX", Vector3D.PLUS_I, Vector3D.PLUS_K, Vector3D.PLUS_I)
fn get_XZX() -> &'static RotationOrder {
    XZX.get_or_init(|| RotationOrder::new("XZX", Vector3D::new(1.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(1.0, 0.0, 0.0)))
}
// Java: new RotationOrder("YXY", Vector3D.PLUS_J, Vector3D.PLUS_I, Vector3D.PLUS_J)
fn get_YXY() -> &'static RotationOrder {
    YXY.get_or_init(|| RotationOrder::new("YXY", Vector3D::new(0.0, 1.0, 0.0), Vector3D::new(1.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0)))
}
// Java: new RotationOrder("YZY", Vector3D.PLUS_J, Vector3D.PLUS_K, Vector3D.PLUS_J)
fn get_YZY() -> &'static RotationOrder {
    YZY.get_or_init(|| RotationOrder::new("YZY", Vector3D::new(0.0, 1.0, 0.0), Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(0.0, 1.0, 0.0)))
}
// Java: new RotationOrder("ZXZ", Vector3D.PLUS_K, Vector3D.PLUS_I, Vector3D.PLUS_K)
fn get_ZXZ() -> &'static RotationOrder {
    ZXZ.get_or_init(|| RotationOrder::new("ZXZ", Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(1.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0)))
}
// Java: new RotationOrder("ZYZ", Vector3D.PLUS_K, Vector3D.PLUS_J, Vector3D.PLUS_K)
fn get_ZYZ() -> &'static RotationOrder {
    ZYZ.get_or_init(|| RotationOrder::new("ZYZ", Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(0.0, 1.0, 0.0), Vector3D::new(0.0, 0.0, 1.0)))
}

// Java: public class RotationOrder
impl RotationOrder {
    // Java: RotationOrder()
    pub fn new(name: &str, a1: Vector3D, a2: Vector3D, a3: Vector3D) -> Self {
        RotationOrder {
            name: name.to_string(),
            a1,
            a2,
            a3,
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> &str {
        // Java: return name;
        &self.name
    }

    // Java: Vector3D getA1()
    pub fn getA1(&self) -> &Vector3D {
        // Java: return a1;
        &self.a1
    }

    // Java: Vector3D getA2()
    pub fn getA2(&self) -> &Vector3D {
        // Java: return a2;
        &self.a2
    }

    // Java: Vector3D getA3()
    pub fn getA3(&self) -> &Vector3D {
        // Java: return a3;
        &self.a3
    }

}
