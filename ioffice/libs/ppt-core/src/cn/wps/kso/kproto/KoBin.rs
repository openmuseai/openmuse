#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code, unused_unsafe)]

// Java: import cn.wps.kfc.util.KcByte
use super::kobin_hosts::KcByte;
// Java: import cn.wps.kso.kproto.memory.KpBytePool
use super::kobin_hosts::KpBytePool;
// Java: import cn.wps.kso.kproto.memory.KpStringPool
use super::kobin_hosts::KpStringPool;
// Java: import java.nio.charset.Charset
use stubs::java::nio::charset::Charset::Charset;
// Java: import cn.wps.kso.kproto.KoZone
use super::KoZone::KoZone;
// Java: import cn.wps.kso.kproto.KoAr
use super::kobin_hosts::KoAr;
// Java: import java.lang.Class
use stubs::java::lang::Class::Class;
use super::kobin_hosts::{Value1, IllegalArgumentException, UnsupportedOperationException, System};

#[derive(Clone, Copy, Default)]
struct NullInst;

// java private static final KoBin inst = new KoBin() { public int size() { return 0; } public int read(byte[] b, int off) { return 0; } public int asInt() { return 0; } public long asKey() { return 0L; } };
static inst: NullInst = NullInst;

// java public static class Encode
#[derive(Default)]
pub struct Encode {
}

// java public static class Decode
#[derive(Default)]
pub struct Decode {
}

// java public static class Factory
#[derive(Default)]
pub struct Factory {
}

// java public static class ByteArrayBin implements KoBin
#[derive(Default)]
pub struct ByteArrayBin {
    // java private byte[] buffer;
    buffer: Option<Vec<u8>>,
    // java private int offset;
    offset: i32,
    // java private int size;
    size: i32,
}

// java public static class IntBin implements KoBin
#[derive(Default)]
pub struct IntBin {
    // java private int value;
    value: i32,
}

// java public static class RealBin implements KoBin
#[derive(Default)]
pub struct RealBin {
    // java private long value;
    value: i64,
}

// java public static class TextBin extends ByteArrayBin
#[derive(Default)]
pub struct TextBin {
    // java ByteArrayBin super_
    pub(crate) super_: ByteArrayBin,
}

// java public static class Null
#[derive(Default)]
pub struct Null {
}

// java public interface KoBin
pub trait KoBin {
    // java  int size()
    fn size(&self) -> i32;

    // java  int read(byte[] var1, int var2)
    fn read(&self, var1: &mut Vec<u8>, var2: i32) -> i32;

    // java  int asInt()
    fn asInt(&self) -> i32;

    // java  long asKey()
    fn asKey(&self) -> i64;

}

impl KoBin for NullInst {
    fn size(&self) -> i32 { 0 }
    fn read(&self, _var1: &mut Vec<u8>, _var2: i32) -> i32 { 0 }
    fn asInt(&self) -> i32 { 0 }
    fn asKey(&self) -> i64 { 0 }
}

// Java: KpStringPool.memory1() — index 0 is the null sentinel, later puts append.
fn string_pool() -> &'static std::sync::Mutex<Vec<String>> {
    static POOL: std::sync::OnceLock<std::sync::Mutex<Vec<String>>> = std::sync::OnceLock::new();
    POOL.get_or_init(|| std::sync::Mutex::new(vec![String::new()]))
}

fn string_pool_put(val: &str) -> i32 {
    let mut pool = string_pool().lock().expect("KpStringPool");
    pool.push(val.to_string());
    (pool.len() - 1) as i32
}

fn string_pool_get(index: i32) -> String {
    if index <= 0 {
        return String::new();
    }
    string_pool()
        .lock()
        .expect("KpStringPool")
        .get(index as usize)
        .cloned()
        .unwrap_or_default()
}

impl Encode {
    // java public static KoBin from(boolean value, KoZone zone)
    pub fn from__boolean_kozone(value: bool, zone: &KoZone) -> Box<dyn KoBin> {
      // java return KoBin.Factory.fromBool(value, zone);
      //
      // rust: ★2B-③-2 —— 真实实现（原为 `todo!("DEFERRED_NON_FUNCTION")`）。
      //   Java: `fromBool` = `IntBin.from(value ? -1 : 0)`，故此处与 `from__int_kozone` 同构。
      //   owner: ppt.kso.kproto.KoBin.Encode    wave: M-PPT-SHAPE-DISPATCH
      let mut ib = IntBin::default();
      ib.from(if value { -1 } else { 0 });
      Box::new(ib)
    }

    // java public static KoBin from(int value, KoZone zone)
    pub fn from__int_kozone(value: i32, _zone: &KoZone) -> Box<dyn KoBin> {
      // java return KoBin.Factory.fromInt(value, zone);
      let mut ib = IntBin::default();
      ib.from(value);
      Box::new(ib)
    }

    pub fn from_int(value: i32, zone: &KoZone) -> Box<dyn KoBin> {
      Self::from__int_kozone(value, zone)
    }

    pub fn from_bool(value: bool, zone: &KoZone) -> Box<dyn KoBin> {
      Self::from__boolean_kozone(value, zone)
    }

    pub fn from_double(value: f64, zone: &KoZone) -> Box<dyn KoBin> {
      Self::from__double_kozone(value, zone)
    }

    pub fn from_string(value: &str, zone: &KoZone) -> Box<dyn KoBin> {
      Self::from__string_kozone(value, zone)
    }

    // java public static KoBin from(double value, KoZone zone)
    pub fn from__double_kozone(value: f64, zone: &KoZone) -> Box<dyn KoBin> {
      // java return KoBin.Factory.fromDouble(value, zone);
      //
      // rust: ★W6c-6 —— 真实实现（原为 `todo!("DEFERRED_NON_FUNCTION")`）。
      //   `RealBin` 已整类收口并 `impl KoBin`，`Factory.fromDouble` 亦已落地 ⇒
      //   f64 型属性（`a:*` 的百分比角度 / `a:ln` 的 `w` 等）现在**真实落值**。
      //   owner: ppt.kso.kproto.KoBin.Encode    wave: M-PPT-KMOSLIDE-W6c-6
      Factory::fromDouble(value, zone)
    }

    // java public static KoBin from(Boolean value, KoZone zone)
    pub fn from__boolean_kozone__2(value: bool, zone: &KoZone) -> Box<dyn KoBin> {
      // java return KoBin.Factory.fromBool(value, zone);
      //
      // rust: ★2B-③-2 —— 真实实现（原为 `todo!("DEFERRED_NON_FUNCTION")`）。
      //   Java: `fromBool` = `IntBin.from(value ? -1 : 0)`，故此处与 `from__int_kozone` 同构。
      //   owner: ppt.kso.kproto.KoBin.Encode    wave: M-PPT-SHAPE-DISPATCH
      let mut ib = IntBin::default();
      ib.from(if value { -1 } else { 0 });
      Box::new(ib)
    }

    // java public static KoBin from(Integer value, KoZone zone)
    pub fn from__integer_kozone(value: i32, zone: &KoZone) -> Box<dyn KoBin> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return KoBin.Factory.fromInt(value, zone);
      return KoBin::Factory.fromInt(value, zone);

      */
    }

    // java public static KoBin from(Double value, KoZone zone)
    pub fn from__double_kozone__2(value: f64, zone: &KoZone) -> Box<dyn KoBin> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return KoBin.Factory.fromDouble(value, zone);
      return KoBin::Factory.fromDouble(value, zone);

      */
    }

    // java public static KoBin from(String value, KoZone zone)
    pub fn from__string_kozone(value: &str, zone: &KoZone) -> Box<dyn KoBin> {
      // java return KoBin.Factory.fromText(value, zone);
      Factory::fromText_live(value, zone)
    }

    // java public static KoBin from(T value, KoZone zone)
    pub fn from__t_kozone(value: &mut KoAr, zone: &KoZone) -> Box<dyn KoBin> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return value.saveBin(zone);
      return value.saveBin(zone);

      */
    }

    // java public static KoBin from(T value, KoZone zone)
    pub fn from__t_kozone__2(value: &KoAr, zone: &KoZone) -> Box<dyn KoBin> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Class clazz = value.getClass();
      let mut clazz: Box<dyn std::any::Any + Send + Sync> = value.getClass();
      // java if (clazz == Boolean.class)
      if clazz == todo!() {
        // java return KoBin.Factory.fromBool((Boolean)value, zone);
        return KoBin::Factory.fromBool(todo!("cast → bool"), zone);
      }
      // java else if (clazz == Integer.class)
      else if clazz == todo!() {
        // java return KoBin.Factory.fromInt((Integer)value, zone);
        return KoBin::Factory.fromInt(((value) as i32), zone);
      }
      // java else if (clazz == Double.class)
      else if clazz == todo!() {
        // java return KoBin.Factory.fromDouble((Double)value, zone);
        return KoBin::Factory.fromDouble(((value) as f64), zone);
      }
      // java else if (clazz == String.class)
      else if clazz == todo!() {
        // java return KoBin.Factory.fromText((String)value, zone);
        return KoBin::Factory.fromText(value.to_string(), zone);
      }
      // java else if (KoAr.class.isAssignableFrom(clazz))
      else if todo!().isAssignableFrom(clazz) {
        // java return ((KoAr)value).saveBin(zone);
        return todo!("cast → Box<dyn KoAr>").saveBin(zone);
      }
      // java else
      else {
        // java throw new IllegalArgumentException();
        panic!("{}", IllegalArgumentException::new());
      }

      */
    }

    // java public static KoBin fromBH(int bh, KoZone zone)
    pub fn fromBH(bh: i32, zone: &KoZone) -> Box<dyn KoBin> {
      // java return KoBin.Factory.fromInt(bh, zone);
      // Java `Factory.fromInt(value, zone)` = `((IntBin)zone.allocate(IntBin.class)).from(value)`；
      // Rust 侧等价实现即 `Encode.from(int, KoZone)`（直接构造 IntBin，不依赖反射分配）。
      Encode::from__int_kozone(bh, zone)
    }

}

// java public static class Decode
impl Decode {
    // java public static boolean toBool(KoBin bin)
    //
    // rust: ★2B-③-2 —— 真实实现（原为 `todo!("DEFERRED_NON_FUNCTION")`）。
    //   owner: ppt.kso.kproto.KoBin.Decode    wave: M-PPT-SHAPE-DISPATCH
    pub fn toBool(bin: &dyn KoBin) -> bool {
      Factory::toBool(bin)
    }

    // java public static int toInt(KoBin bin)
    pub fn toInt(bin: &dyn KoBin) -> i32 {
      // java return KoBin.Factory.toInt(bin);
      bin.asInt()
    }

    // java public static double toDouble(KoBin bin)
    pub fn toDouble(bin: &dyn KoBin) -> f64 {
      // java return KoBin.Factory.toDouble(bin);
      //
      // rust: ★W6c-6 —— 真实实现（原为 `todo!`）。`Factory::toDouble` 随之落地。
      //   owner: ppt.kso.kproto.KoBin.Decode    wave: M-PPT-KMOSLIDE-W6c-6
      Factory::toDouble(bin)
    }

    // java public static String toText(KoBin bin)
    pub fn toText(bin: &dyn KoBin) -> String {
      // java return KoBin.Factory.toText(bin);
      Factory::toText_live(bin)
    }

    // java public static int toBH(KoBin bin)
    pub fn toBH(bin: &dyn KoBin) -> i32 {
      // java return toInt(bin);
      Self::toInt(bin)
    }

    // rust: ★M-PPT-SHAPE-DISPATCH 2B（本行无对应 Java 语句）——
    //   Java `KoBin.Decode.toAny(Class, KoBin, KoZone)` 的 `KoAr` 分支是
    //   `zone.allocate(clazz)` + `((KoAr)inst).loadBin(bin, zone)`，即**按目标 Rust 类型直接构造**，
    //   不需要任何运行时类型信息。Rust 侧每个具体模型（`Fill`/`BlipFill`/`Transform2D`/…）
    //   都已有 `open(&dyn KoBinTrait) -> Self`（= `initialize` + `loadBin`），正是该分支的等价物。
    //   ⇒ 把这些访问器（Java 里由 `Class<T>` 驱动）改写为**在类型已知处调用 `<T>::open(bin)`** 即可，
    //   无需引入 `dyn Any`、也无需把 `bin` 的载荷类型化。
    //
    //   ★因此下面的 Java 原样转写体 `toAny()` **保留登记但不再使用**：它是"按 `Class` 泛化派发"
    //   的转写，而该泛化在 Rust 侧不可表达（`Class<T>` → 无对应物）。它不可移出（历史登记
    //   `DEFERRED_NON_FUNCTION`），但**任何新代码都不得调用它** —— 调用点一律用 `<T>::open(bin)`。
    //   owner: ppt.kso.kproto.kobin    wave: M-PPT-SHAPE-DISPATCH
    //
    // java public static Value1 toAny(Class<Value1> clazz, KoBin bin, KoZone zone)
    pub fn toAny() {
      todo!("DEFERRED_NON_FUNCTION: Class<T> 泛化派发在 Rust 侧不可表达；调用点改用 <T>::open(bin)");
      /* DEFERRED_NON_FUNCTION
      // java Value1 inst = null;
      let mut inst: Value1 = Default::default();
      // java if (clazz == Boolean.class)
      if clazz == todo!() {
        // java inst = clazz.cast(Boolean.valueOf(toBool(bin)));
        inst = clazz.cast(Boolean::valueOf(Self::toBool(bin)));
      }
      // java else if (clazz == Integer.class)
      else if clazz == todo!() {
        // java inst = clazz.cast(Integer.valueOf(toInt(bin)));
        inst = clazz.cast(Integer::valueOf(Self::toInt(bin)));
      }
      // java else if (clazz == Double.class)
      else if clazz == todo!() {
        // java inst = clazz.cast(Double.valueOf(toDouble(bin)));
        inst = clazz.cast(Double::valueOf(Self::toDouble(bin)));
      }
      // java else if (clazz == String.class)
      else if clazz == todo!() {
        // java inst = clazz.cast(toText(bin));
        inst = clazz.cast(Self::toText(bin));
      }
      // java else if (KoAr.class.isAssignableFrom(clazz))
      else if todo!().isAssignableFrom(clazz) {
        // java inst = (Value1)zone.allocate(clazz);
        inst = (zone.allocate(clazz)).downcast_ref::<Value1>().unwrap();
        // java ((KoAr)inst).loadBin(bin, zone);
        todo!("cast → Box<dyn KoAr>").loadBin(bin, zone);
      }
      // java if (inst == null)
      if false {
        // java throw new IllegalArgumentException();
        panic!("{}", IllegalArgumentException::new());
      }
      // java else
      else {
        // java return inst;
        return { let __x: () = todo!("no align strategy matched"); __x };
      }
      */
    }

}

// java public static class Factory
impl Factory {
    // java public static KoBin wrap(byte[] b, KoZone zone)
    pub fn wrap(b: &[u8], zone: &KoZone) -> Box<dyn KoBin> {
      // java return ((ByteArrayBin)zone.allocate(ByteArrayBin.class)).from(b);
      //
      // rust: ★W6c-6 —— 真实实现（原为 `todo!`）。同 `fromDouble`：Java 的
      //   `zone.allocate` 只是**池化复用**，本 port 直接构造，语义等价。
      //   owner: ppt.kso.kproto.KoBin.Factory    wave: M-PPT-KMOSLIDE-W6c-6
      let _ = zone;
      let mut bab = ByteArrayBin::default();
      bab.from__bytearr(b);
      Box::new(bab)
    }

    // java public static KoBin wrap(byte[] b, int off, int len, KoZone zone)
    pub fn wrap__bytearr_int_int_kozone(b: &[u8], off: i32, len: i32, zone: &KoZone) -> Box<dyn KoBin> {
      // java return ((ByteArrayBin)zone.allocate(ByteArrayBin.class)).from(b, off, len);
      //
      // rust: ★W6c-6 —— 真实实现（原为 `todo!`），同 `wrap(byte[])`。
      //   owner: ppt.kso.kproto.KoBin.Factory    wave: M-PPT-KMOSLIDE-W6c-6
      let _ = zone;
      let mut bab = ByteArrayBin::default();
      bab.from__bytearr_int_int(b, off, len);
      Box::new(bab)
    }

    // java public static int toInt(KoBin bytes)
    pub fn toInt(bytes: &dyn KoBin) -> i32 {
      // java return bytes.asInt();
      //
      // rust: ★W6c-6 —— 真实实现（原为 `todo!`）。
      //   owner: ppt.kso.kproto.KoBin.Factory    wave: M-PPT-KMOSLIDE-W6c-6
      bytes.asInt()
    }

    // java public static double toDouble(KoBin bytes)
    pub fn toDouble(bytes: &dyn KoBin) -> f64 {
      // java byte[] buff = new byte[8];
      let mut buff = vec![0u8; 8];
      // java bytes.read(buff, 0);
      bytes.read(&mut buff, 0);
      // java return Double.longBitsToDouble(KcByte.byteArrayToLong(buff, 0));
      //
      // rust: ★W6c-6 —— 真实实现（原为 `todo!`）。`KcByte.byteArrayToLong` 与
      //   `longToByteArray` 互逆，且同 `byteArrayToInt` 一样是**大端**
      //   （`bytes[0]`=最高有效字节，见 `KcByte.java:4-8` 与 `RealBin::read` 注）。
      //   `KpBytePool.free(buff)` 在此无对应（Vec 自动释放）。
      //   owner: ppt.kso.kproto.KoBin.Factory    wave: M-PPT-KMOSLIDE-W6c-6
      let mut bits: u64 = 0;
      for i in 0..8usize {
        bits = (bits << 8) | buff[i] as u64;
      }
      f64::from_bits(bits)
    }

    // java public static boolean toBool(KoBin bin)
    //
    // rust: ★2B-③-2 —— 真实实现（原为 `todo!("DEFERRED_NON_FUNCTION")`）。
    //   Java: `return toInt(bin) != 0;` —— 与 `fromBool` 写入的 -1/0 编码对应。
    //   owner: ppt.kso.kproto.KoBin.Factory    wave: M-PPT-SHAPE-DISPATCH
    pub fn toBool(bin: &dyn KoBin) -> bool {
      // rust: Java `toInt(bin)` 等价于 `bin.asInt()`（见 `Decode::toInt` 的 port）。
      bin.asInt() != 0
    }

    // java public static String toText(KoBin bytes)
    pub fn toText() {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int index = toInt(bytes);
      let mut index: i32 = Self::toInt(bytes);
      // java return KpStringPool.memory1().get(index);
      return KpStringPool::memory1().get(index);
      */
    }

    // java public static KoBin fromBool(boolean value, KoZone zone)
    pub fn fromBool() {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((IntBin)zone.allocate(IntBin.class)).from(value ? -1 : 0);
      return (zone.allocate(todo!())).downcast_ref::<IntBin>().unwrap().from((if value { -1 } else { 0 }));
      */
    }

    // java public static KoBin fromInt(int value, KoZone zone)
    pub fn fromInt() {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((IntBin)zone.allocate(IntBin.class)).from(value);
      return (zone.allocate(todo!())).downcast_ref::<IntBin>().unwrap().from(value);
      */
    }

    // java public static KoBin fromText(String str, KoZone zone)
    pub fn fromText_live(str: &str, _zone: &KoZone) -> Box<dyn KoBin> {
      // java return ((IntBin)zone.allocate(IntBin.class)).from(KpStringPool.memory1().put(str));
      let index = string_pool_put(str);
      let mut ib = IntBin::default();
      ib.from(index);
      Box::new(ib)
    }

    // java public static String toText(KoBin bytes)
    pub fn toText_live(bytes: &dyn KoBin) -> String {
      // java int index = toInt(bytes); return KpStringPool.memory1().get(index);
      string_pool_get(Decode::toInt(bytes))
    }

    // java public static KoBin fromDouble(double r, KoZone zone)
    pub fn fromDouble(r: f64, zone: &KoZone) -> Box<dyn KoBin> {
      // java return ((RealBin)zone.allocate(RealBin.class)).from(r);
      //
      // rust: ★W6c-6 —— 真实实现（原为 `todo!`）。
      //   Java 经 `zone.allocate(RealBin.class)` 从**池**里取一个 `RealBin`；本 port 的
      //   `KoZone::allocate` 仍是编译面占位（`kobin_hosts::KoZone` 是单元结构体）
      //   ⇒ 此处直接构造 `RealBin`，**语义等价**（池只为复用，`from(r)` 会整体覆写 `value`）。
      //   偏差登记：池化复用未复刻（无行为差异）。
      //   owner: ppt.kso.kproto.KoZone    wave: M-PPT-KPROTO-REALBIN
      let mut rb = RealBin::default();
      rb.from_double(r);
      Box::new(rb)
    }

    // java public static KoBin dup(KoBin bin, KoZone zone)
    pub fn dup(bin: &dyn KoBin, zone: &KoZone) -> Box<dyn KoBin> {
      // java if (bin.size() != 4)
      if bin.size() != 4 {
        // java byte[] buff = new byte[bin.size()];
        let mut buff: Vec<u8> = vec![0u8; bin.size().max(0) as usize];
        // java bin.read(buff, 0);
        bin.read(&mut buff, 0);
        // java return wrap(buff, zone);
        //
        // rust: ★W6c-6 —— **转写纠错**。原实现是
        //   `IntBin::from(bin.asInt()); Box::new(ib)` —— 把非 4 字节的 bin **降级成 int**
        //   （注释自称 "compile-surface copy via asInt"），这**不是** Java 语义：
        //   Java 走 `wrap(byte[])`，即把**完整字节**包成 `ByteArrayBin`。
        //   `RealBin`（8 字节，`asInt()` 恒抛）此前因 `Base` 为 `todo!` 而未暴露该偏差，
        //   本波 `RealBin` 收口后立即暴露（`02_function_fill` t03/t04 panic）。
        //   现按 Java 逐字实现；`Factory::wrap` 亦随之落地。
        //   owner: ppt.kso.kproto.KoBin.Factory    wave: M-PPT-KMOSLIDE-W6c-6
        return Factory::wrap(&buff, zone);
      }
      // java else
      else {
        // java return fromInt(bin.asInt(), zone);
        Encode::from__int_kozone(bin.asInt(), zone)
      }
    }

}

// java public static class ByteArrayBin implements KoBin
//
// rust: ★W6c-6 —— 整类收口（原为 **8 个 `todo!` 全占位**）。Java 侧 8 个成员全部是
//   纯字节/位运算，无外部依赖（`KcByte.byteArrayToInt` / `intBitMask` 亦已在下方就地实现，
//   因为 `kfc_alpha::KcByte` 真身仍是未移植桩）。`Factory.dup` / `Factory.wrap` 依赖本类。
//   owner: ppt.kso.kproto.KoBin.ByteArrayBin    wave: M-PPT-KMOSLIDE-W6c-6
impl ByteArrayBin {
    // java public ByteArrayBin wrap(byte[] buffer, int off, int size)
    pub fn wrap(&mut self, buffer: &[u8], off: i32, size: i32) -> &mut Self {
      // java this.buffer = buffer;
      self.buffer = Some(buffer.to_vec());
      // java this.offset = off;
      self.offset = off;
      // java this.size = size;
      self.size = size;
      // java return this;
      self
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return this.size;
      self.size
    }

    // java public int read(byte[] b, int off)
    pub fn read(&self, b: &mut Vec<u8>, off: i32) -> i32 {
      // java int copied = Math.min(b.length - off, this.size);
      let off = off.max(0) as usize;
      let buf = self.buffer.clone().unwrap_or_default();
      let mut copied: i32 = ((b.len() as i32) - (off as i32)).min(self.size);
      if copied < 0 {
        copied = 0;
      }
      // java System.arraycopy(this.buffer, this.offset, b, off, copied);
      let src = self.offset.max(0) as usize;
      for i in 0..(copied as usize) {
        let v = *buf.get(src + i).unwrap_or(&0);
        if off + i < b.len() {
          b[off + i] = v;
        } else {
          b.push(v);
        }
      }
      // java return copied;
      copied
    }

    // java public int asInt()
    pub fn asInt(&self) -> i32 {
      // java if (this.size() > 4)
      if self.size() > 4 {
        // java throw new UnsupportedOperationException();
        let _ = UnsupportedOperationException::new();
        panic!("UnsupportedOperationException");
      }
      // java else
      else {
        // java return KcByte.byteArrayToInt(this.buffer, this.offset) & KcByte.intBitMask(this.size);
        self.byte_array_to_int(self.offset) & Self::int_bit_mask(self.size)
      }
    }

    // java public long asKey()
    pub fn asKey(&self) -> i64 {
      // java return (long)this.buffer.hashCode();
      //
      // rust: Java 的 `byte[].hashCode()` 是**身份哈希**（`System.identityHashCode` 语义），
      //   不是内容哈希；本 port 无对应概念 ⇒ 以**内容哈希**近似（同内容同键）。
      //   偏差登记：`asKey` 只用于 `KpMap` 的键去重，内容哈希不改变可观测行为。
      //   owner: ppt.kso.kproto.KoBin.ByteArrayBin    wave: M-PPT-KMOSLIDE-W6c-6
      let buf = self.buffer.clone().unwrap_or_default();
      let mut h: i64 = 0;
      for byte in buf.iter() {
        h = h.wrapping_mul(31).wrapping_add(*byte as i64);
      }
      h
    }

    // rust: 无对应 Java 成员 —— `KcByte.byteArrayToInt(byte[] b, int off)` 的就地实现
    //   （`kfc_alpha::KcByte` 真身仍是未移植桩；Java 语义见 `KcByte.java:44-53`）。
    //   owner: ppt.kso.kproto.KoBin.ByteArrayBin    wave: M-PPT-KMOSLIDE-W6c-6
    fn byte_array_to_int(&self, off: i32) -> i32 {
      let buf = self.buffer.clone().unwrap_or_default();
      let off = off.max(0) as usize;
      let size = (buf.len().saturating_sub(off)).min(4) as i32;
      let mut value: i32 = 0;
      for i in 0..(size as usize) {
        // java value |= bytes[i].setValue(value, b[off + i]);
        //
        // rust: ★W6c-6 —— **转写纠错（大端）**。`KcByte.bytes` 的掩码是
        //   `b0=0xFF000000, b1=0x00FF0000, b2=0x0000FF00, b3=0x000000FF`
        //   （见 `KcByte.java:4-8`），`byteArrayToInt` 以 `bytes[i]` 处理 `b[off+i]`
        //   ⇒ **第 0 字节是最高有效字节（大端）**，与 `intToByteArray`（先写 `bytes[3]`）
        //   互逆。原实现按小端左移 `8*i` ⇒ `RGB.putR(0x11)` 读回 `0x11000000`
        //   （实测 `(285212672, 570425344, 855638016)` 对应 `0x11/0x22/0x33` 左移 24 位）。
        //   owner: ppt.kso.kproto.KoBin.ByteArrayBin    wave: M-PPT-KMOSLIDE-W6c-6
        value |= ((buf[off + i] as i32) & 0xFF) << ((size as usize - 1 - i) * 8);
      }
      value
    }

    // rust: 无对应 Java 成员 —— `KcByte.intBitMask(int size)` 就地实现
    //   （`KcByte.java:87-96`）。owner/wave 同上。
    fn int_bit_mask(size: i32) -> i32 {
      let mask_len = size.min(4);
      let mut mask: i32 = 0;
      for i in 0..mask_len {
        mask |= (255i32) << (i * 8);
      }
      mask
    }

    // java public ByteArrayBin from(byte[] bytes, int off, int len)
    pub fn from__bytearr_int_int(&mut self, bytes: &[u8], off: i32, len: i32) -> &mut Self {
      // java return this.wrap(bytes, off, len);
      self.wrap(bytes, off, len)
    }

    // java public ByteArrayBin from(byte[] bytes)
    pub fn from__bytearr(&mut self, bytes: &[u8]) -> &mut Self {
      // java return this.wrap(bytes, 0, bytes.length);
      let len = bytes.len() as i32;
      self.wrap(bytes, 0, len)
    }

}

// java public static class IntBin implements KoBin
impl IntBin {
    // java public IntBin from(int value)
    pub fn from(&mut self, value: i32) -> &mut Self {
      // java this.value = value;
      self.value = value;
      // java return this;
      self
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return 4;
      4
    }

    // java public int read(byte[] b, int off)
    pub fn read(&self, b: &mut Vec<u8>, off: i32) -> i32 {
      // java KcByte.intToByteArray(this.value, b, off);
      let off = off.max(0) as usize;
      let bytes = self.value.to_be_bytes();
      if b.len() < off + 4 {
        b.resize(off + 4, 0);
      }
      let n = 4.min(b.len().saturating_sub(off));
      b[off..off + n].copy_from_slice(&bytes[..n]);
      // java return 4;
      4
    }

    // java public int asInt()
    pub fn asInt(&self) -> i32 {
      // java return this.value;
      self.value
    }

    // java public long asKey()
    pub fn asKey(&self) -> i64 {
      // java return (long)this.value;
      return ((self.value) as i64);
    }

}

// java public static class RealBin implements KoBin
impl RealBin {
    // rust: ★2B-③-2 —— 真实实现（原为 `todo!`）。Java: `this.value = Double.doubleToLongBits(r)`。
    //   owner: ppt.kso.kproto.KoBin.RealBin    wave: M-PPT-SHAPE-DISPATCH
    pub fn from_double(&mut self, r: f64) -> &mut Self {
      self.value = r.to_bits() as i64;
      self
    }

    // java public RealBin from(double r)
    pub fn from() {
      todo!("from() 无参形式不可表达；调用点改用 from_double(r)");
      /* DEFERRED_NON_FUNCTION
      // java this.value = Double.doubleToLongBits(r);
      self.value = Double::doubleToLongBits(r);
      // java return this;
      return { let __x: () = todo!("no align strategy matched"); __x };
      */
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return 8;
      //
      // rust: ★W6c-6 —— 真实实现（原为 `todo!("DEFERRED_NON_FUNCTION")`）。
      //   `RealBin` 是 Java `KoBin` 的**双精度承载**（`value` = `doubleToLongBits`），
      //   下列成员与 `IntBin` 同构，纯位运算、**无外部依赖** ⇒ 可整类收口。
      //   owner: ppt.kso.kproto.KoBin.RealBin    wave: M-PPT-KMOSLIDE-W6c-6
      8
    }

    // java public int read(byte[] b, int off)
    pub fn read(&self, b: &mut Vec<u8>, off: i32) -> i32 {
      // java KcByte.longToByteArray(this.value, b, off);
      //
      // rust: `KcByte::longToByteArray` 是**未移植桩**（`kfc_alpha/.../KcByte.rs` 全 `todo!()`），
      //   而 Java 真身只有两行、逐字等价于「低 32 位在前、高 32 位在后的大端写入」
      //   （`intToByteArray((int)(value & 0xFFFFFFFFL), b, offset)` +
      //     `intToByteArray((int)(value >> 32), b, offset + 4)`）。
      //   此处**就地**按该语义实现，避免依赖未移植桩；`KcByte` 真身落地后可直接换回调用。
      //   owner: ppt.kso.kproto.KoBin.RealBin    wave: M-PPT-KMOSLIDE-W6c-6
      let off = off.max(0) as usize;
      // rust: ★W6c-6 —— **大端**。Java `KcByte.longToByteArray` 组合两个 `intToByteArray`
      //   （高 32 位在 off、低 32 位在 off+4），而 `intToByteArray` 是
      //   `b[off]=bytes[3].getValue(v)`（低 8 位打头）⇒ 整体即 **最高有效字节在前**，
      //   与 `Factory::toDouble` 的 `byteArrayToLong` 互为逆运算。
      let bytes = self.value.to_be_bytes();
      if b.len() < off + 8 {
        b.resize(off + 8, 0);
      }
      let n = 8.min(b.len().saturating_sub(off));
      b[off..off + n].copy_from_slice(&bytes[..n]);
      // java return 8;
      8
    }

    // java public int asInt()
    pub fn asInt__4(&self) -> i32 {
      // java throw new UnsupportedOperationException();
      //
      // rust: ★W6c-6 —— 真实实现（原为 `todo!`）。Java 侧 `RealBin.asInt()` **恒抛**
      //   （双精度不能隐式降为 int），故此处与 Java 逐字同形地 panic。
      //   owner: ppt.kso.kproto.KoBin.RealBin    wave: M-PPT-KMOSLIDE-W6c-6
      //   ⚠ `kobin_hosts::UnsupportedOperationException` 是单元结构体且未实现 `Display`
      //   （`panic!("{}", ..)` 在此不可用）⇒ 直接以类名 panic，与 Java 的异常语义同形。
      let _ = UnsupportedOperationException::new();
      panic!("UnsupportedOperationException");
    }

    // java public long asKey()
    pub fn asKey__4(&self) -> i64 {
      // java return this.value;
      //
      // rust: ★W6c-6 —— 真实实现（原为 `todo!`）。
      //   owner: ppt.kso.kproto.KoBin.RealBin    wave: M-PPT-KMOSLIDE-W6c-6
      self.value
    }

}

// java public static class TextBin extends ByteArrayBin
impl TextBin {
    // java public TextBin from(String s)
    pub fn from() {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.from(s.getBytes(Charset.forName("UTF-8")));
      self.super_.from(s.getBytes__object(Charset::forName("UTF-8")));
      // java return this;
      return { let __x: () = todo!("no align strategy matched"); __x };
      */
    }

}

impl std::ops::Deref for TextBin {
    type Target = ByteArrayBin;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TextBin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class Null
impl Null {
    // java public static KoBin value1()
    pub fn value1() -> Box<dyn KoBin> {
      // java return inst;
      Box::new(inst)
    }

    // java public static KoBin empty()
    pub fn empty() -> Box<dyn KoBin> {
      // java return value1();
      return Self::value1();
    }

    // java public static boolean equalsEmpty(KoBin rhs)
    pub fn equalsEmpty(rhs: Option<&dyn KoBin>) -> bool {
      // java return !notEmpty(rhs);
      return !Self::notEmpty(rhs);
    }

    // java public static boolean notEmpty(KoBin rhs)
    pub fn notEmpty(rhs: Option<&dyn KoBin>) -> bool {
      // java return rhs != null && rhs.size() > 0;
      return rhs.is_some() && rhs.unwrap().size() > 0;
    }

}

// rust: ★W6c-6 —— `RealBin` 原先**未实现** `KoBin`（这正是 `Encode::from__double_kozone`
//   无法返回 `Box<dyn KoBin>` 的直接原因）。Java 侧 `RealBin implements KoBin`；
//   此处按 Java 的 `size`/`read`/`asInt`/`asKey` 四个成员逐一对齐。
//   owner: ppt.kso.kproto.KoBin.RealBin    wave: M-PPT-KMOSLIDE-W6c-6
// rust: ★W6c-6 —— `ByteArrayBin` 原先**未实现** `KoBin`（`Factory.wrap` 因此无法返回
//   `Box<dyn KoBin>`）。Java 侧 `ByteArrayBin implements KoBin`；此处按 Java 的
//   `size`/`read`/`asInt`/`asKey` 四个成员逐一对齐。
//   owner: ppt.kso.kproto.KoBin.ByteArrayBin    wave: M-PPT-KMOSLIDE-W6c-6
impl KoBin for ByteArrayBin {
    fn size(&self) -> i32 { ByteArrayBin::size(self) }
    fn read(&self, var1: &mut Vec<u8>, var2: i32) -> i32 { ByteArrayBin::read(self, var1, var2) }
    fn asInt(&self) -> i32 { ByteArrayBin::asInt(self) }
    fn asKey(&self) -> i64 { ByteArrayBin::asKey(self) }
}

impl KoBin for RealBin {
    fn size(&self) -> i32 { RealBin::size(self) }
    fn read(&self, var1: &mut Vec<u8>, var2: i32) -> i32 { RealBin::read(self, var1, var2) }
    fn asInt(&self) -> i32 { RealBin::asInt__4(self) }
    fn asKey(&self) -> i64 { RealBin::asKey__4(self) }
}

impl KoBin for IntBin {
    fn size(&self) -> i32 { IntBin::size(self) }
    fn read(&self, var1: &mut Vec<u8>, var2: i32) -> i32 { IntBin::read(self, var1, var2) }
    fn asInt(&self) -> i32 { IntBin::asInt(self) }
    fn asKey(&self) -> i64 { IntBin::asKey(self) }
}
