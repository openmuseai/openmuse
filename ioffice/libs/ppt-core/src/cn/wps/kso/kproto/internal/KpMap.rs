#![allow(non_snake_case, dead_code)]

use std::collections::HashMap;

use crate::cn::wps::kso::kproto::KoBin::{Factory, IntBin, KoBin, Null};
use crate::cn::wps::kso::kproto::KoZone::KoZone;

// java public class KpMap
//
// rust: ★W6c-6 —— **转写纠错**。原实现把值存成 `HashMap<i32, i32>`
//   （`put` 里 `bytes.asInt()`，`byKey` 里重新包成 `IntBin`），而 Java 真身是
//   `Map<Integer, KoBin>` —— **存 KoBin 对象本身**（`MutableMap.put(key, bytes)`）。
//   该扁平化在 `RealBin`（8 字节、`asInt()` 恒抛 `UnsupportedOperationException`）落地后
//   立刻暴露：`KpMap::put` 会把 `RealBin` 当 int 读 ⇒ `02_function_fill` t03/t04 panic。
//   Java 语义是：`put` 只存引用；真正写字节发生在 `save()` → `KpMapBinTbl::putVal`
//   （那里用 `bin.size()` + `bin.read()`，**天然支持 8 字节 RealBin**）；
//   读回时 `valAt(i)` 依 `sizeAt(i)` 重建对应宽度的 bin。
//   现按 Java 逐字改为 `HashMap<i32, Box<dyn KoBin>>`。
//   owner: ppt.kso.kproto.internal.KpMap    wave: M-PPT-KMOSLIDE-W6c-6
pub struct KpMap {
    map: HashMap<i32, Box<dyn KoBin>>,
}

impl KpMap {
    // java public KpMap()
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    // java public KpMap(KoBin bin)
    pub fn KpMap_from(bin: &dyn KoBin) -> Self {
        let mut map = Self::new();
        map.load(bin);
        map
    }

    // java public KoBin byKey(int key)
    pub fn byKey(&self, key: i32) -> Box<dyn KoBin> {
        // java return (KoBin)this.overMap...get(key)（无则 roMap.byKey(key) ⇒ Null）
        //
        // rust: ★W6c-6 —— 原实现把 int 重新包成 `IntBin`（见结构体注）；现按 Java
        //   返回**存储的 KoBin** 的等价副本（`dup` 语义：同 size / 同字节）。
        //   owner: ppt.kso.kproto.internal.KpMap    wave: M-PPT-KMOSLIDE-W6c-6
        match self.map.get(&key) {
            Some(v) => KpMap::dup_bin(v.as_ref()),
            None => Null::value1(),
        }
    }

    // rust: 无对应 Java 成员 —— `byKey` 需要交出**拥有权**（Java 直接返回引用）。
    //   按 `size` 重建同宽 bin：4 字节 → `IntBin`，其余 → `Factory::wrap`（字节保真）。
    //   owner: ppt.kso.kproto.internal.KpMap    wave: M-PPT-KMOSLIDE-W6c-6
    fn dup_bin(v: &dyn KoBin) -> Box<dyn KoBin> {
        let n = v.size().max(0) as usize;
        if n == 4 {
            let mut ib = IntBin::default();
            ib.from(v.asInt());
            return Box::new(ib);
        }
        let mut buff = vec![0u8; n];
        let _ = v.read(&mut buff, 0);
        let zone = KoZone::enter().clone();
        crate::cn::wps::kso::kproto::KoBin::Factory::wrap(&buff, &zone)
    }

    // java public boolean has(int key)
    pub fn has(&self, key: i32) -> bool {
        self.map.contains_key(&key)
    }

    // java public void put(int key, KoBin bytes)
    pub fn put(&mut self, key: i32, bytes: Box<dyn KoBin>) {
        // java ((MutableMap)this.overMap.get()).put(key, bytes);
        //
        // rust: ★W6c-6 —— 直接**存 KoBin 对象**（原为 `bytes.asInt()`，见结构体注）。
        //   `asInt()` 对 8 字节的 `RealBin` 会 panic，且对 >4 字节的 `ByteArrayBin` 亦然
        //   ⇒ 原写法只在「恰好 4 字节」时成立，属真实转写缺陷。
        //   owner: ppt.kso.kproto.internal.KpMap    wave: M-PPT-KMOSLIDE-W6c-6
        self.map.insert(key, bytes);
    }

    // java public void clear(int key)
    pub fn clear_key(&mut self, key: i32) {
        self.map.remove(&key);
    }

    // java public KoBin save(KoZone zone)
    pub fn save(&self, _zone: &KoZone) -> Box<dyn KoBin> {
        // Java: return KoBin.Encode.fromBH(mutableMap.saveTo(roMap), zone);
        if self.map.is_empty() {
            return Null::value1();
        }
        let bh = crate::cn::wps::kso::kproto::internal::KpTablePool::KpTablePool::with_builder(|builder| {
            let mut keys: Vec<i32> = self.map.keys().copied().collect();
            keys.sort();
            for key in &keys {
                // rust: ★W6c-6 —— 表项宽度取自**该键真实 KoBin 的 `size()`**
                //   （Java `buildTbl` 同样是按 `value.size()` 建表，见 `KpMap.java:151+`）。
                //   原实现硬编码 4 ⇒ 8 字节的 `RealBin` / >4 字节的 `ByteArrayBin` 写回被截断。
                //   owner: ppt.kso.kproto.internal.KpMap    wave: M-PPT-KMOSLIDE-W6c-6
                let n = self.map.get(key).map(|v| v.size()).unwrap_or(4).max(0);
                builder.append_entry(*key, n);
            }
            builder.sort();
            let mut tbl = builder.build();
            for i in 0..tbl.size() {
                let key = tbl.key_at(i);
                // Java: tbl.putVal(i, value) —— 直接写该 bin 的原始字节。
                if let Some(value) = self.map.get(&key) {
                    tbl.put_val(i, value.as_ref());
                }
            }
            tbl.bh()
        });
        let mut out = IntBin::default();
        out.from(bh);
        Box::new(out)
    }

    // java public void load(KoBin bin) — read the map table back into entries
    pub fn load(&mut self, bin: &dyn KoBin) {
        self.map.clear();
        let bh = bin.asInt();
        if bh == 0 {
            return;
        }
        let tbl = crate::cn::wps::kso::kproto::internal::KpTablePool::KpTablePool::get_map(bh);
        for i in 0..tbl.size() {
            // Java: this.map.put(key, roMap.byKey(key)) —— 值为 KoBin 对象。
            self.map.insert(tbl.key_at(i), tbl.val_at(i));
        }
    }

    // rust: ★W6c-6 本方法无对应 Java 声明 —— **只读入口快照**。
    //   Java 侧 `KpMap` 的 `overMap` 可直接遍历（`((MutableMap)this.overMap.get())`）；
    //   Rust 侧 `map` 是私有字段，`Fill::convert_clone(&self)` 等只读拷贝路径需要它。
    //   owner: ppt.kso.kproto.internal.KpMap    wave: M-PPT-KMOSLIDE-W6c-6
    pub fn snapshot_entries(&self) -> Vec<(i32, Box<dyn KoBin>)> {
        let mut keys: Vec<i32> = self.map.keys().copied().collect();
        keys.sort();
        keys.into_iter()
            .filter_map(|k| self.map.get(&k).map(|v| (k, Factory::dup(v.as_ref(), KoZone::enter()))))
            .collect()
    }

    // java public boolean dirty()
    pub fn dirty(&self) -> bool {
        !self.map.is_empty()
    }
}
