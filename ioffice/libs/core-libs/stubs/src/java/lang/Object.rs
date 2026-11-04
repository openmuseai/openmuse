//! Java `java.lang.Object` → Rust trait.
//!
//! blanket `impl<T: Any + 'static> Object for T`：业务 struct 无需再写 `impl Object`。
//! `as_any` / `as_any_mut` 必须在 blanket 里实现（`Self` 在 `dyn Object` 上不定长，
//! 不能写在 trait 默认方法里做 `&mut Self as &mut dyn Any`）。

use std::any::Any;

/// Java Object 行为。
pub trait Object: Any + 'static {
    fn equals(&self, obj: &dyn Object) -> bool {
        std::ptr::eq(
            self.as_any() as *const dyn Any as *const (),
            obj.as_any() as *const dyn Any as *const (),
        )
    }

    fn hashCode(&self) -> i32 {
        0
    }

    fn toString(&self) -> String {
        std::any::type_name_of_val(self.as_any()).to_string()
    }

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 所有 `'static` 类型自动实现 Object。
impl<T: Any + 'static> Object for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// 兼容占位：blanket 已覆盖，此宏为空操作。
#[macro_export]
macro_rules! impl_java_object {
    ($($t:ty),+ $(,)?) => {};
}

/// Java `new Object()` 占位实例
#[derive(Default)]
pub struct ObjectImpl;
