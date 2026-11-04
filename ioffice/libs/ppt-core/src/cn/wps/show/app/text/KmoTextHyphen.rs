#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.text
// Java: import cn.wps.drawing.data.basic.IndexRange
// rust: no-stmt —— **SP3b 阻塞项（实测，R9 口径）**：`cn.wps.drawing.data.basic.IndexRange`
//   的真实 Rust 移植存在于 `drawing/data/basic/IndexRange.rs`（322 行，真实现），但
//   `drawing/data/basic/mod.rs:3` 把它放在 `#[cfg(feature = "full")]` 后，而 `full` 因
//   `libs/core-libs/thirdparty/picparser` 10 个编译错误不可用。本波实测「只把该行 un-gate
//   进默认构建」会引入 **10 个 E0423**（`IndexRange.rs` 全篇把 `KoZone` 当**值**用：
//   `inst.zone = *zone;` / `self.zone = KoZone::defaultZone();`），而默认构建可达的
//   `kso/kproto/KoZone.rs` 是 `KoZone::enter()/defaultZone() -> &'static KoZone` 的**引用**版
//   ⇒ 两者是同一类型的两套互斥口径，`IndexRange` 在默认构建下**不可命名**，且其 `full` 版
//   当前根本无法编译。故 `checkHyphen()` 的返回类型无法表达（见下）。
//   修复动作（SP3b）：统一 `KoZone` 的值/引用口径并把 `basic::IndexRange` port 进默认构建。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.show.app.text.KmoTextOperator（同包，`checkHyphen` 首参类型）
// rust: no-stmt —— `checkHyphen()` 随 `IndexRange` 一并 DEFERRED（见下）。

// Java: public class KmoTextHyphen
//
// 【SP3 落地范围】`checkEmoji()` 为真实的 100% 直译实现；`checkHyphen()` 因返回类型
//   `IndexRange` 在默认构建下不可命名而**签名本身**无法表达 ⇒ DEFERRED（Java 体原文已保留在
//   下方注记中，SP3b 落地 `IndexRange` 后可逐句接回）。
// owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
pub struct KmoTextHyphen;

// Java: public class KmoTextHyphen
impl KmoTextHyphen {
    // Java: public static IndexRange checkHyphen(KmoTextOperator textOperator, int start, int end)
    //
    // DEFERRED_NON_FUNCTION: 返回类型 `cn.wps.drawing.data.basic.IndexRange` 默认构建下不可命名
    //   （见文件头注记）⇒ 方法**签名本身**无法表达。Java 体（`KmoTextHyphen.java:11-40`）原文：
    //   `if (start != 0 && textOperator.seek(start, false))
    //    {
    //       IndexRange range = null;
    //       String text = textOperator.text(start, end);
    //       int startOffset = checkEmoji(text, 0, false);
    //       if (startOffset != 0)
    //       {
    //          String pre = textOperator.text(start + startOffset, start);
    //          if (pre.length() <= 0 || !Character.isHighSurrogate(pre.charAt(0))) { startOffset = 0; }
    //       }
    //       int endOffset = checkEmoji(text, end - start - 1, true);
    //       if (endOffset != 0)
    //       {
    //          String next = textOperator.text(end, end + endOffset);
    //          if (next.length() <= 0 || !Character.isLowSurrogate(next.charAt(0))) { endOffset = 0; }
    //       }
    //       if (startOffset != 0 || endOffset != 0)
    //       {
    //          range = IndexRange.create();
    //          range.putStart(start + startOffset);
    //          range.putEnd(end + endOffset);
    //       }
    //       return range;
    //    }
    //    else { return null; }`
    //   接线时另需 `KmoTextOperator.seek(int, boolean)` 与 `text(int, int)` 两个成员（本波已保留
    //   签名，其体随 `KmoTextBody` 一并 DEFERRED，见 `KmoTextOperator.rs`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public static int checkEmoji(CharSequence str, int index, boolean forward)
    //
    // rust: 偏差登记 —— Java 形参 `CharSequence` 的 `length()`/`charAt()` 是 UTF-16 code unit 口径；
    //   Rust 无对应物，故形参取 `&str` 并在方法内 `encode_utf16()` 还原为 code unit 序列，
    //   语义（代理对判定）与 Java 一致。越界索引与 Java `charAt` 一样直接 panic（负数索引经
    //   `as usize` 越界 panic，等价于 Java `StringIndexOutOfBoundsException`）。
    pub fn checkEmoji(str_: &str, index: i32, forward: bool) -> i32 {
      // rust: Java `CharSequence` → UTF-16 code unit 序列（见方法头注记）
      let units: Vec<u16> = str_.encode_utf16().collect();
      // Java: if (index >= str.length())
      if index >= units.len() as i32 {
         // Java: return 0;
         return 0;
      // Java: else
      } else {
         // Java: char c = str.charAt(index);
         let c = units[index as usize];
         // Java: if (forward)
         if forward {
            // Java: return Character.isHighSurrogate(c) ? 1 : 0;
            if is_high_surrogate(c) { 1 } else { 0 }
         // Java: else
         } else {
            // Java: return Character.isLowSurrogate(c) ? -1 : 0;
            if is_low_surrogate(c) { -1 } else { 0 }
         }
      }
    }
}

// rust: 无 Java 对应（登记为 rust: 辅助）—— `Character.isHighSurrogate(char)` 的等价判据
//   （Java `Character.java`，UTF-16 高位代理区间 U+D800..U+DBFF）。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
fn is_high_surrogate(c: u16) -> bool {
    (0xD800..=0xDBFF).contains(&c)
}

// rust: 无 Java 对应（登记为 rust: 辅助）—— `Character.isLowSurrogate(char)` 的等价判据
//   （Java `Character.java`，UTF-16 低位代理区间 U+DC00..U+DFFF）。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
fn is_low_surrogate(c: u16) -> bool {
    (0xDC00..=0xDFFF).contains(&c)
}
