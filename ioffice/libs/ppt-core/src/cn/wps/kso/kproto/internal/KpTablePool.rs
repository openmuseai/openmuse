use super::KpMapBinTbl::{Builder, KpMapBinTbl};

// Java: public class KpTablePool
pub struct KpTablePool;

impl KpTablePool {
    // Java: static KpMapBinTbl getMap(int bh)
    pub fn get_map(bh: i32) -> KpMapBinTbl {
        KpMapBinTbl::bound(bh)
    }

    // Java: static KpMapBinTbl.Builder（Java 侧 `KpMap.saveTo` 内 `new Builder()` 就地新建）
    //
    // rust: ★W6c-6 —— **转写纠错**。原实现把 `Builder` 放在 thread-local 上并在每次调用时
    //   `clear()`，但 `KpMap::save` 是**可重入**的：外层 map 保存（`Fill.map`）
    //   会在 `builder.build()` 的过程中经 `tbl.put_val(i, value)` → `value.read(...)`
    //   触发**内层 map 的保存**（`Color.map` → `RGB.map`）。共享同一个 `Builder` 且每次
    //   `clear()` ⇒ 外层刚 `append_entry` 的条目被内层调用清空，外层随后 `build()`
    //   只看到内层遗留条目（或空表）⇒ 嵌套子对象（填充颜色）在 KProto 往返后整体丢失。
    //   实测：`Fill::saveBin` 的 `map[2]` 有值、`Color::saveBin` 也产出了 bin，
    //   但读回 `Outline::fill().solidFill().hasSrgb()==false`（颜色恒为 `(0,0,0)`）。
    //   现改为**每次调用新建独立 `Builder`**（与 Java `new Builder()` 一致）。
    //   owner: ppt.kso.kproto.internal.KpTablePool    wave: M-PPT-KMOSLIDE-W6c-6
    pub fn with_builder<R>(f: impl FnOnce(&mut Builder) -> R) -> R {
        let mut b = Builder::new();
        f(&mut b)
    }
}
