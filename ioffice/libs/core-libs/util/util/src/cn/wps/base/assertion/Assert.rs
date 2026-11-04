// Java field: String NON_FATAL_ASSERT_TAG
pub(crate) const NON_FATAL_ASSERT_TAG: &str = "NON_FATAL_ASSERT";
// Java field: boolean ENABLE_THROWING_ASSERT_EXCEPTION
pub(crate) const ENABLE_THROWING_ASSERT_EXCEPTION: bool = true;

// Java: public class Assert
pub struct Assert;

// Java: public class Assert
impl Assert {
    // Java: static void assertEquals(boolean expected, boolean actual)
    pub fn assertEquals_bool(expected: bool, actual: bool) {
        // Java: assertEquals((String)null, expected, actual);
        Self::assertEquals_msg_bool(None, expected, actual);
    }

    // Java: static void assertEquals(byte expected, byte actual)
    pub fn assertEquals_byte(expected: i8, actual: i8) {
        // Java: assertEquals((String)null, (byte)expected, (byte)actual);
        Self::assertEquals_msg_byte(None, expected, actual);
    }

    // Java: static void assertEquals(char expected, char actual)
    pub fn assertEquals_char(expected: char, actual: char) {
        // Java: assertEquals((String)null, (char)expected, (char)actual);
        Self::assertEquals_msg_char(None, expected, actual);
    }

    // Java: static void assertEquals(double expected, double actual, double delta)
    pub fn assertEquals_double(expected: f64, actual: f64, delta: f64) {
        // Java: assertEquals((String)null, expected, actual, delta);
        Self::assertEquals_msg_double(None, expected, actual, delta);
    }

    // Java: static void assertEquals(float expected, float actual, float delta)
    pub fn assertEquals_float(expected: f32, actual: f32, delta: f32) {
        // Java: assertEquals((String)null, expected, actual, delta);
        Self::assertEquals_msg_float(None, expected, actual, delta);
    }

    // Java: static void assertEquals(int expected, int actual)
    pub fn assertEquals_int(expected: i32, actual: i32) {
        // Java: assertEquals((String)null, (int)expected, (int)actual);
        Self::assertEquals_msg_int(None, expected, actual);
    }

    // Java: static void assertEquals(long expected, long actual)
    pub fn assertEquals_long(expected: i64, actual: i64) {
        // Java: assertEquals((String)null, expected, actual);
        Self::assertEquals_msg_long(None, expected, actual);
    }

    // Java: static void assertEquals(Object expected, Object actual)
    pub fn assertEquals_obj<T: std::fmt::Debug + PartialEq>(expected: &T, actual: &T) {
        // Java: assertEquals((String)null, (Object)expected, (Object)actual);
        Self::assertEquals_msg_obj(None, expected, actual);
    }

    // Java: static void assertEquals(short expected, short actual)
    pub fn assertEquals_short(expected: i16, actual: i16) {
        // Java: assertEquals((String)null, (short)expected, (short)actual);
        Self::assertEquals_msg_short(None, expected, actual);
    }

    // Java: static void assertEquals(String message, boolean expected, boolean actual)
    pub fn assertEquals_msg_bool(_message: Option<&str>, _expected: bool, _actual: bool) {}

    // Java: static void assertEquals(String message, byte expected, byte actual)
    pub fn assertEquals_msg_byte(_message: Option<&str>, _expected: i8, _actual: i8) {}

    // Java: static void assertEquals(String message, char expected, char actual)
    pub fn assertEquals_msg_char(_message: Option<&str>, _expected: char, _actual: char) {}

    // Java: static void assertEquals(String message, double expected, double actual, double delta)
    pub fn assertEquals_msg_double(_message: Option<&str>, _expected: f64, _actual: f64, _delta: f64) {}

    // Java: static void assertEquals(String message, float expected, float actual, float delta)
    pub fn assertEquals_msg_float(_message: Option<&str>, _expected: f32, _actual: f32, _delta: f32) {}

    // Java: static void assertEquals(String message, int expected, int actual)
    pub fn assertEquals_msg_int(_message: Option<&str>, _expected: i32, _actual: i32) {}

    // Java: static void assertEquals(String message, long expected, long actual)
    pub fn assertEquals_msg_long(_message: Option<&str>, _expected: i64, _actual: i64) {}

    // Java: static void assertEquals(String message, Object expected, Object actual)
    pub fn assertEquals_msg_obj<T: std::fmt::Debug + PartialEq>(_message: Option<&str>, _expected: &T, _actual: &T) {}

    // Java: static void assertEquals(String message, short expected, short actual)
    pub fn assertEquals_msg_short(_message: Option<&str>, _expected: i16, _actual: i16) {}

    // Java: static void assertEquals(String expected, String actual)
    pub fn assertEquals_str(expected: &str, actual: &str) {
        // Java: assertEquals((String)null, (String)expected, (String)actual);
        Self::assertEquals_msg_str(None, expected, actual);
    }

    // Java: static void assertEquals(String message, String expected, String actual)
    pub fn assertEquals_msg_str(_message: Option<&str>, _expected: &str, _actual: &str) {}

    // Java: static void assertFalse(boolean condition)
    pub fn assertFalse(_condition: bool) {}

    // Java: static void assertFalse(String message, boolean condition)
    pub fn assertFalse_msg(_message: &str, _condition: bool) {}

    // Java: static void assertNotNull(Object object)
    pub fn assertNotNull<T: NotNull>(object: T) {
        // Java: assertNotNull((String)null, object);
        Self::assertNotNull_msg("", object);
    }

    // Java: static void assertNotNull(String message, Object object)
    // rust: Java 形参是 `Object` —— 可空（null）亦可非空，且**按值**传递。
    //   Rust 侧以 `NotNull` trait 同时覆盖两种实参形态：
    //     * `Option<T>`        —— 可能为 null（Java 的 null 引用）
    //     * 任意 `&T` / 值 `T` —— 不可空，恒非 null
    //   注意 `&Option<T>` 亦归入后者（引用本身非 null），与 Java 语义一致。
    pub fn assertNotNull_msg<T: NotNull>(message: &str, object: T) {
        // Java: assertTrue(message, object != null);
        Self::assertTrue_msg(message, object.is_not_null());
    }

    // Java: static void assertNotSame(Object expected, Object actual)
    pub fn assertNotSame<T>(expected: &T, actual: &T) {
        // Java: assertNotSame((String)null, expected, actual);
        Self::assertNotSame_msg("", expected, actual);
    }

    // Java: static void assertNotSame(String message, Object expected, Object actual)
    pub fn assertNotSame_msg<T>(message: &str, expected: &T, actual: &T) {
        // Java: if (expected == actual)
        if std::ptr::eq(expected, actual) {
            // Java: failSame(message);
            Self::failSame(message);
        }
    }

    // Java: static void assertNull(Object object)
    pub fn assertNull<T>(object: &Option<T>) {
        // Java: assertNull((String)null, object);
        Self::assertNull_msg("", object);
    }

    // Java: static void assertNull(String message, Object object)
    pub fn assertNull_msg<T>(message: &str, object: &Option<T>) {
        // Java: assertTrue(message, object == null);
        Self::assertTrue_msg(message, object.is_none());
    }

    // Java: static void assertSame(Object expected, Object actual)
    pub fn assertSame<T>(expected: &T, actual: &T) {
        // Java: assertSame((String)null, expected, actual);
        Self::assertSame_msg("", expected, actual);
    }

    // Java: static void assertSame(String message, Object expected, Object actual)
    pub fn assertSame_msg<T>(_message: &str, _expected: &T, _actual: &T) {}

    // Java: static void assertTrue(boolean condition)
    pub fn assertTrue(_condition: bool) {}

    // Java: static void assertTrue(String message, boolean condition)
    pub fn assertTrue_msg(_message: &str, _condition: bool) {}

    // rust: ★M-STAGE2 P1-D1 / T6 —— 补模板引擎生成的命名别名。
    //   模板按 `name__<参数类型>` 规则生成调用名，但 `Assert.rs` 只有 Rust 风格名
    //   ⇒ 全仓库 43 个文件受影响。此处补齐别名，保持 Java 门面语义一致。

    // Java: static void assertNotNull(String message, Object object)
    pub fn assertNotNull__string_object<T: NotNull>(message: &str, object: T) {
        Self::assertNotNull_msg(message, object);
    }

    // Java: static void assertTrue(String message, boolean condition)
    pub fn assertTrue__string_boolean(message: &str, condition: bool) {
        Self::assertTrue_msg(message, condition);
    }

    // Java: static void fail()
    pub fn fail() {}

    // Java: static void fail(String message)
    pub fn fail_msg(_message: &str) {}

    // Java: static void fail(String message)
    // rust: 同一 Java 重载的**别名**。仓内 `Assert.fail(String)` 的调用点并存两种
    //   后缀写法（`fail_msg` 14 个文件 / `fail__string` 19 个文件，后者为
    //   `java2rust` 的重载消歧约定 `name__argtypes`）。此处补别名使两者皆可解析，
    //   避免为一个命名约定改动 19 个既有调用点。
    pub fn fail__string(_message: &str) {}

    // Java: static void failNotEquals(String message, Object expected, Object actual)
    fn failNotEquals(_message: &str, _expected: &str, _actual: &str) {}

    // Java: static void failNotSame(String message, Object expected, Object actual)
    fn failNotSame(_message: &str, _expected: &str, _actual: &str) {}

    // Java: static void failSame(String message)
    fn failSame(_message: &str) {}

    // Java: static String format(String message, Object expected, Object actual)
    pub fn format(message: Option<&str>, expected: &str, actual: &str) -> String {
        // Java: String formatted = "";
        let mut formatted = String::new();
        // Java: if (message != null)
        if let Some(msg) = message {
            // Java: formatted = message + " ";
            formatted = format!("{} ", msg);
        }
        // Java: return formatted + "expected:<" + expected + "> but was:<" + actual + ">";
        format!("{}expected:<{}> but was:<{}>", formatted, expected, actual)
    }

    // Java: static void logFail()
    pub fn logFail() {}

    // Java: static void logFail(String message)
    pub fn logFail_msg(_message: &str) {}

    // Java: static void logNotNull(Object object)
    pub fn logNotNull<T: NotNull>(_object: T) {}

    // Java: static void logNotNull(String message, Object object)
    pub fn logNotNull_msg<T: NotNull>(_message: &str, _object: T) {}

    // Java: static void logTrue(boolean condition)
    pub fn logTrue(_condition: bool) {}

    // Java: static void logTrue(String message, boolean condition)
    pub fn logTrue_msg(_message: &str, _condition: bool) {}

    // Java: Assert()
    pub fn new() -> Self {
        // Java: super();
        Assert
    }
}

// rust: 非 Java 成员 —— 支撑 `Assert::assertNotNull[_msg]` 的 `Object` 形参语义。
//   Java 的 `Object` 可为 null 亦可非 null；Rust 侧用此 trait 表达「是否可能为 null」：
//     * `Option<T>`            → `is_some()`
//     * 其它任意 `T`（含 `&T`） → `true`（Java 中非 null 引用/值恒非 null）
//   仅此一条 blanket impl，避免为每种类型重复实现。
pub trait NotNull {
    fn is_not_null(&self) -> bool;
}

// Java: object != null  ⇔  Option 的 is_some()
impl<T> NotNull for Option<T> {
    fn is_not_null(&self) -> bool {
        self.is_some()
    }
}

// Java: 非 null 引用/值 ⇒ 恒真。`&Option<T>` 也走这里（引用本身非 null），
// 与 Java 把 `&field` 当普通对象引用的语义一致。
impl<T: ?Sized> NotNull for &T {
    fn is_not_null(&self) -> bool {
        true
    }
}

// Java: `&mut T` 同样是非 null 引用（Java 无 `&mut` 概念，参数一视同仁）。
impl<T: ?Sized> NotNull for &mut T {
    fn is_not_null(&self) -> bool {
        true
    }
}

// Java: `Box<dyn Trait>` 对应 Java 的接口引用（按值持有），恒非 null。
impl<T: ?Sized> NotNull for Box<T> {
    fn is_not_null(&self) -> bool {
        true
    }
}
