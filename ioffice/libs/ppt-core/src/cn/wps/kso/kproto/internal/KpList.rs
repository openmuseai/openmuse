#![allow(non_snake_case, dead_code)]

use crate::cn::wps::kso::kproto::KoBin::{IntBin, KoBin, Null};
use crate::cn::wps::kso::kproto::KoZone::KoZone;

// java public class KpList
pub struct KpList {
    items: Vec<i32>,
    // Java field: boolean dirtied
    dirtied: bool,
}

impl KpList {
    // java public KpList(KoZone zone)
    pub fn new(zone: &KoZone) -> Self {
        let mut this = Self { items: Vec::new(), dirtied: false };
        // Java: this.load(KoBin.Null.value1(), zone);
        this.load(Null::value1().as_ref(), zone);
        this
    }

    // java public KpList(KoBin bin, KoZone zone)
    pub fn KpList_from(bin: &dyn KoBin, zone: &KoZone) -> Self {
        let mut this = Self { items: Vec::new(), dirtied: false };
        this.load(bin, zone);
        this
    }

    pub fn clear(&mut self) {
        // Java: if (this.array.size() > 0) { this.dirtied = true; this.array.clear(); }
        if !self.items.is_empty() {
            self.dirtied = true;
            self.items.clear();
        }
    }

    pub fn size(&self) -> i32 {
        self.items.len() as i32
    }

    pub fn add(&mut self, bin: Box<dyn KoBin>) {
        // Java: this.dirtied = true; this.array.add(bin);
        self.dirtied = true;
        self.items.push(bin.asInt());
    }

    pub fn get(&self, i: i32) -> Box<dyn KoBin> {
        match self.items.get(i as usize) {
            Some(&v) => {
                let mut ib = IntBin::default();
                ib.from(v);
                Box::new(ib)
            }
            None => Null::value1(),
        }
    }

    pub fn load(&mut self, bin: &dyn KoBin, _zone: &KoZone) {
        // Java: this.dirtied = false; this.array = this.listArray.load(this.bufferArray.load(bin), zone);
        // ListArray/BufferArray 尚未移植。元素按写入顺序存成 KpTablePool 页（key=下标, val=asInt），
        // load 按 key 排序还原。同进程 get(i).asInt() 与 Java 一致。
        self.dirtied = false;
        self.items.clear();
        let bh = bin.asInt();
        if bh == 0 {
            return;
        }
        let tbl = crate::cn::wps::kso::kproto::internal::KpTablePool::KpTablePool::get_map(bh);
        let mut pairs: Vec<(i32, i32)> = Vec::new();
        for i in 0..tbl.size() {
            pairs.push((tbl.key_at(i), tbl.val_at(i).asInt()));
        }
        pairs.sort_by_key(|p| p.0);
        self.items = pairs.into_iter().map(|p| p.1).collect();
    }

    pub fn dirty(&self) -> bool {
        // Java: return this.dirtied;
        self.dirtied
    }

    pub fn hashCode(&self) -> i32 {
        self.items.len() as i32
    }

    pub fn save(&mut self, zone: &KoZone) -> Box<dyn KoBin> {
        // Java: this.dirtied = false; return KoBin.Encode.fromBH(this.array.saveTo(), zone);
        // 页布局见 load()：下标 → asInt，经 Encode.fromBH 交出。
        self.dirtied = false;
        if self.items.is_empty() {
            return Null::value1();
        }
        let bh = crate::cn::wps::kso::kproto::internal::KpTablePool::KpTablePool::with_builder(|builder| {
            for (index, value) in self.items.iter().enumerate() {
                builder.append_entry(index as i32, 4);
                let _ = value;
            }
            builder.sort();
            let mut tbl = builder.build();
            for (index, value) in self.items.iter().enumerate() {
                let mut ib = IntBin::default();
                ib.from(*value);
                tbl.put_val(index as i32, &ib);
            }
            tbl.bh()
        });
        crate::cn::wps::kso::kproto::KoBin::Encode::fromBH(bh, zone)
    }
}
