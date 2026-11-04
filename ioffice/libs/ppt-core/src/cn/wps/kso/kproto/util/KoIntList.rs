use crate::cn::wps::kso::kproto::KoBin::{Decode, Encode, Factory, KoBin, Null};
use crate::cn::wps::kso::kproto::KoZone::KoZone;
use crate::cn::wps::kso::kproto::internal::KpBinBox::KpBinBox;
use crate::cn::wps::kso::kproto::internal::KpList::KpList;

// Java: public class KoIntList implements KoAr
pub struct KoIntList {
    lst: Vec<i32>,
    dirtied: bool,
    bin_box: KpBinBox,
    array: KpList,
}

impl KoIntList {
    pub fn new() -> Self {
        let bin_box = KpBinBox::new();
        let array = KpList::KpList_from(bin_box.get().as_ref(), KoZone::defaultZone());
        Self {
            lst: Vec::new(),
            dirtied: false,
            bin_box,
            array,
        }
    }

    pub fn size(&self) -> i32 {
        self.lst.len() as i32
    }

    pub fn get(&self, index: i32) -> i32 {
        self.lst.get(index as usize).copied().unwrap_or(0)
    }

    pub fn add(&mut self, value: i32) -> bool {
        self.dirtied = true;
        self.lst.push(value);
        true
    }

    pub fn dirty(&self) -> bool {
        self.dirtied
    }

    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBin> {
        if self.dirtied {
            self.dirtied = false;
            self.array.load(Null::empty().as_ref(), zone);
            let size = self.size();
            for i in 0..size {
                self.array.add(Encode::from_int(self.get(i), zone));
            }
            let saved = self.array.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
        }
        self.bin_box.get()
    }

    pub fn loadBin(&mut self, bin: &dyn KoBin, zone: &KoZone) {
        self.bin_box.set(Some(bin));
        self.dirtied = false;
        self.array.load(bin, zone);
        self.lst.clear();
        let size = self.array.size();
        for i in 0..size {
            self.lst.push(Decode::toInt(self.array.get(i).as_ref()));
        }
    }

    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBin> {
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    pub fn equals(&self, o: &KoIntList) -> bool {
        self.lst == o.lst
    }

    pub fn hashCode(&self) -> i32 {
        self.lst.iter().sum()
    }
}
