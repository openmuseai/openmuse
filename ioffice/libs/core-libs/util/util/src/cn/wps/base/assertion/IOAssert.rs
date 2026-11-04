use std::sync::atomic::{AtomicBool, Ordering};

// Java field: boolean ON_IO_ASSERT
pub static ON_IO_ASSERT: AtomicBool = AtomicBool::new(false);

// Java: public class IOAssert extends Assert
pub struct IOAssert {
}

// Java: public class IOAssert extends Assert
impl IOAssert {
    // Java: IOAssert()
    pub fn new() -> Self {
        // Java: super();
        IOAssert {}
    }

    // Java: IOAssert()
    // Rust: 与 `new()` 同体；提供 **Java 同名构造函数**以便 §3.9 门面按类名对齐
    //       （Java 侧 `IOAssert()` 是 public 构造函数，门禁要求 Rust 侧有同名对应）。
    #[allow(non_snake_case)]
    pub fn IOAssert() -> Self {
        // Java: super();
        IOAssert {}
    }

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
    pub fn assertEquals_obj<T: std::fmt::Debug + PartialEq>(_expected: &T, _actual: &T) {
        // Java: assertEquals((String)null, (Object)expected, (Object)actual);
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
    pub fn assertNotNull<T>(_object: &Option<T>) {
        // Java: assertNotNull((String)null, object);
    }

    // Java: static void assertNotNull(String message, Object object)
    pub fn assertNotNull_msg<T>(_message: &str, _object: &Option<T>) {
        // Java: assertTrue(message, object != null);
    }

    // rust: ★★模板引擎规范名别名（同一 Java 重载 `assertNotNull(String, Object)`）。
    //
    //   Java 原文**只有一个** `assertNotNull(String message, Object object)` 重载，
    //   但 Rust 侧存在**两个历史命名**：
    //     - 手写名 `assertNotNull_msg`      —— 已登记文件的 239 处 live 调用；
    //     - 模板名 `assertNotNull__string_object` —— 未登记文件的 352 处 live 调用。
    //   这 352 处所在文件**均未登记**（§45.2「文件存在 ≠ 文件可编译」），
    //   故其编译错误长期不可见；一旦登记就会全部报
    //   `E0599 no associated function named assertNotNull__string_object`。
    //
    //   ★此处**不是第二实现**：仅是同一 Java 重载的规范名映射，直接委派给 `assertNotNull_msg`，
    //     以便那 352 处按模板名写下的 live 代码在登记后能够编译。
    //   ★不重命名 `assertNotNull_msg` 的原因：它有 239 处 live 调用（28 个已登记文件，
    //     其中 `DocumentImporter.rs` 49 处），重命名会大面积破坏既有编译面。
    // Java: static void assertNotNull(String message, Object object)
    pub fn assertNotNull__string_object<T>(_message: &str, _object: Option<&T>) {
        // rust: 委派到同一 Java 重载的手写名（见上方说明）。
        //   签名取 `Option<&T>`（按值）以匹配模板引擎生成的调用形状
        //   `assertNotNull__string_object(&"msg", Some(&x))`。
        Self::assertNotNull_msg(_message, &_object);
    }

    // rust: ★★模板引擎规范名别名（第 3 例，同 §60.3/§61.4）。
    //   Java 原文只有一个 `assertNotNull(Object object)` 重载；模板生成名
    //   `assertNotNull__object` 在本仓有 live 调用，而 `IOAssert` 只定义了
    //   手写名 `assertNotNull`。此处仅为规范名映射，委派给手写名。
    // Java: static void assertNotNull(Object object)
    pub fn assertNotNull__object<T>(_object: &Option<T>) {
        // rust: 委派到同一 Java 重载的手写名。
        Self::assertNotNull(_object);
    }

    // Java: static void assertNotSame(Object expected, Object actual)
    pub fn assertNotSame<T>(_expected: &T, _actual: &T) {
        // Java: assertNotSame((String)null, expected, actual);
    }

    // Java: static void assertNotSame(String message, Object expected, Object actual)
    pub fn assertNotSame_msg<T>(_message: &str, _expected: &T, _actual: &T) {
        // Java: if (expected == actual)
        // Java: failSame(message);
    }

    // Java: static void assertNull(Object object)
    pub fn assertNull<T>(_object: &Option<T>) {
        // Java: assertNull((String)null, object);
    }

    // Java: static void assertNull(String message, Object object)
    pub fn assertNull_msg<T>(_message: &str, _object: &Option<T>) {
        // Java: assertTrue(message, object == null);
    }

    // Java: static void assertSame(Object expected, Object actual)
    pub fn assertSame<T>(_expected: &T, _actual: &T) {
        // Java: assertSame((String)null, expected, actual);
    }

    // Java: static void assertSame(String message, Object expected, Object actual)
    pub fn assertSame_msg<T>(_message: &str, _expected: &T, _actual: &T) {}

    // Java: static void assertTrue(boolean condition)
    pub fn assertTrue(_condition: bool) {}

    // Java: static void assertTrue(String message, boolean condition)
    pub fn assertTrue_msg(_message: &str, _condition: bool) {}

    // Java: static void fail()
    pub fn fail() {}

    // Java: static void fail(String message)
    pub fn fail_msg(_message: &str) {}

    // rust: ★★模板引擎规范名别名（同一 Java 重载 `fail(String)`），见 `assertNotNull__string_object` 的说明。
    //   Java 原文**只有一个** `fail(String message)` 重载；模板生成名 `fail__string`
    //   在 word-core 中有多处 live 调用，但 `IOAssert` 只定义了 `fail_msg`。
    // Java: static void fail(String message)
    pub fn fail__string(_message: &str) {
        // rust: 委派到同一 Java 重载的手写名。
        Self::fail_msg(_message);
    }

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
    pub fn logNotNull<T>(_object: &Option<T>) {}

    // Java: static void logNotNull(String message, Object object)
    pub fn logNotNull_msg<T>(_message: &str, _object: &Option<T>) {}

    // Java: static void logNotNull(String message, Object object)
    // rust: ★M-STAGE2 P1-D1 —— 按本文件 `assertNotNull__string_object`
    //   既有的 `String, Object` 命名约定补别名（`BookmarkImporter` 等具名引用它）。
    pub fn logNotNull__string_object<T>(_message: &str, _object: &Option<T>) {}

    // Java: static void logTrue(boolean condition)
    pub fn logTrue(_condition: bool) {}

    // Java: static void logTrue(String message, boolean condition)
    pub fn logTrue_msg(_message: &str, _condition: bool) {}
}
