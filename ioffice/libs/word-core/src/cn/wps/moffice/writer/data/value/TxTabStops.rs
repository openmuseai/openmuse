// Java: package cn.wps.moffice.writer.data.value;
// Java: import cn.wps.moffice.util.SearchUtil;
use crate::cn::wps::moffice::util::SearchUtil::SearchUtil;
// Java: import java.util.Arrays;
use super::TBD::TBD;
use stubs::java::lang::Object::Object;

// Java: public class TxTabStops {
#[allow(non_snake_case)]
#[derive(Default, Clone)]
pub struct TxTabStops {
    // Java: public int itbdDelMax;
    pub itbdDelMax: i32,
    // Java: public int[] rgdxaDel;
    pub rgdxaDel: Option<Vec<i32>>,
    // Java: public int[] rgdxaClose;
    pub rgdxaClose: Option<Vec<i32>>,
    // Java: public int itbdAddMax;
    pub itbdAddMax: i32,
    // Java: public int[] rgdxaAdd;
    pub rgdxaAdd: Option<Vec<i32>>,
    // Java: public TBD[] rgtbdAdd;
    pub rgtbdAdd: Option<Vec<Option<TBD>>>,
}

#[allow(non_snake_case)]
impl TxTabStops {
    // java public static final int MAXTABNUM = 65;
    pub const MAXTABNUM: i32 = 65;

    // java public  TxTabStops()
    pub fn TxTabStops() -> Self {
        // java this.itbdDelMax = this.itbdAddMax = 0;
        Self::default()
    }
    pub fn new() -> Self { Self::TxTabStops() }

    // java public  TxTabStops(int delMax, int addMax)
    pub fn TxTabStops__int_int(delMax: i32, addMax: i32) -> Self {
        // Java: this.rgdxaDel = new int[delMax]; this.rgdxaClose = new int[delMax];
        // Java: this.rgdxaAdd = new int[addMax]; this.rgtbdAdd = new TBD[addMax];
        // Java: this.itbdAddMax = addMax; this.itbdDelMax = delMax;
        Self { itbdDelMax: delMax, rgdxaDel: Some(vec![0; delMax as usize]), rgdxaClose: Some(vec![0; delMax as usize]), itbdAddMax: addMax, rgdxaAdd: Some(vec![0; addMax as usize]), rgtbdAdd: Some(vec![None; addMax as usize]) }
    }
    pub fn new__int_int(delMax: i32, addMax: i32) -> Self { Self::TxTabStops__int_int(delMax, addMax) }

    // java public  TxTabStops(int delMax, int addMax, int[] rgdxaDel, int[] rgdxaClose, int[] rgdxaAdd, TBD[] rgtbdAdd)
    pub fn TxTabStops__int_int_intarr_intarr_intarr_tbdarr(delMax: i32, addMax: i32, rgdxaDel: Option<Vec<i32>>, rgdxaClose: Option<Vec<i32>>, rgdxaAdd: Option<Vec<i32>>, rgtbdAdd: Option<Vec<Option<TBD>>>) -> Self {
        // Java: this.rgdxaDel = rgdxaDel; this.rgdxaClose = rgdxaClose;
        // Java: this.rgdxaAdd = rgdxaAdd; this.rgtbdAdd = rgtbdAdd;
        // Java: this.itbdAddMax = addMax; this.itbdDelMax = delMax;
        Self { itbdDelMax: delMax, rgdxaDel, rgdxaClose, itbdAddMax: addMax, rgdxaAdd, rgtbdAdd }
    }
    pub fn new__int_int_intarr_intarr_intarr_tbdarr(delMax: i32, addMax: i32, rgdxaDel: Option<Vec<i32>>, rgdxaClose: Option<Vec<i32>>, rgdxaAdd: Option<Vec<i32>>, rgtbdAdd: Option<Vec<Option<TBD>>>) -> Self {
        Self::TxTabStops__int_int_intarr_intarr_intarr_tbdarr(delMax, addMax, rgdxaDel, rgdxaClose, rgdxaAdd, rgtbdAdd)
    }

    // Java: public static TxTabStops merge(TxTabStops tab1, TxTabStops tab2) {
    pub fn merge(tab1: Option<&TxTabStops>, tab2: Option<&TxTabStops>) -> Option<TxTabStops> {
        // Java: if (null == tab1) return tab2; else if (null == tab2) return tab1;
        let (tab1, tab2) = match (tab1, tab2) {
            (None, None) => return None,
            (None, Some(t)) | (Some(t), None) => return Some(t.clone()),
            (Some(a), Some(b)) => (a, b),
        };
        // Java: TxTabStops newTab = new TxTabStops(tab1.itbdDelMax + tab2.itbdDelMax, tab1.itbdAddMax + tab2.itbdAddMax);
        let mut newTab = Self::new__int_int(tab1.itbdDelMax + tab2.itbdDelMax, tab1.itbdAddMax + tab2.itbdAddMax);
        // java if (null != tab1.rgdxaDel)
        // Java: System.arraycopy(tab1.rgdxaDel, 0, newTab.rgdxaDel, 0, tab1.rgdxaDel.length);
        // java if (null != tab1.rgdxaClose)
        // java System.arraycopy(tab1.rgdxaClose, 0, newTab.rgdxaClose, 0, tab1.rgdxaClose.length);
        // java if (null != tab2.rgdxaDel)
        // Java: System.arraycopy(tab2.rgdxaDel, 0, newTab.rgdxaDel, tab1.itbdDelMax, tab2.rgdxaDel.length);
        // java if (null != tab2.rgdxaClose)
        // java System.arraycopy(tab2.rgdxaClose, 0, newTab.rgdxaClose, tab1.itbdDelMax, tab2.rgdxaClose.length);
        for (src, offset) in [(tab1, 0), (tab2, tab1.itbdDelMax as usize)] {
            if let Some(a) = &src.rgdxaDel { newTab.rgdxaDel.as_mut().unwrap()[offset..offset + a.len()].copy_from_slice(a); }
            // Java: System.arraycopy(...rgdxaClose..., ...);
            if let Some(a) = &src.rgdxaClose { newTab.rgdxaClose.as_mut().unwrap()[offset..offset + a.len()].copy_from_slice(a); }
        }
        // Java: Arrays.sort(newTab.rgdxaDel); Arrays.sort(newTab.rgdxaClose);
        newTab.rgdxaDel.as_mut().unwrap().sort();
        newTab.rgdxaClose.as_mut().unwrap().sort();
        // Java: for(int i = 0; i < tab1.itbdAddMax; ++i) newTab.insertAdd(tab1.rgdxaAdd[i], tab1.rgtbdAdd[i], i);
        for i in 0..tab1.itbdAddMax as usize {
            newTab.insertAdd(tab1.rgdxaAdd.as_ref().unwrap()[i], tab1.rgtbdAdd.as_ref().unwrap()[i].as_ref(), i as i32);
        }
        // Java: for(int i = 0; i < tab2.itbdAddMax; ++i) newTab.insertAdd(tab2.rgdxaAdd[i], tab2.rgtbdAdd[i], i + tab1.itbdAddMax);
        for i in 0..tab2.itbdAddMax as usize {
            newTab.insertAdd(tab2.rgdxaAdd.as_ref().unwrap()[i], tab2.rgtbdAdd.as_ref().unwrap()[i].as_ref(), i as i32 + tab1.itbdAddMax);
        }
        // Java: return newTab;
        Some(newTab)
    }

    // Java: public void insertAdd(int pos, TBD tbd, int len) {
    pub fn insertAdd(&mut self, pos: i32, tbd: Option<&TBD>, len: i32) {
        // Java: int insertIndex = SearchUtil.binarySearch((int[])this.rgdxaAdd, 0, len, (int)pos);
        let found = SearchUtil::binarySearch__intarr_int_int_int(self.rgdxaAdd.as_ref().unwrap(), 0, len, pos);
        // Java: insertIndex = insertIndex < 0 ? -insertIndex - 1 : insertIndex;
        let insertIndex = if found < 0 { -found - 1 } else { found } as usize;
        // Java: for(int k = len; k > insertIndex; --k) { this.rgdxaAdd[k] = this.rgdxaAdd[k - 1]; this.rgtbdAdd[k] = this.rgtbdAdd[k - 1]; }
        let positions = self.rgdxaAdd.as_mut().unwrap();
        let values = self.rgtbdAdd.as_mut().unwrap();
        for k in (insertIndex + 1..=len as usize).rev() {
            positions[k] = positions[k - 1];
            values[k] = values[k - 1].clone();
        }
        // Java: this.rgdxaAdd[insertIndex] = pos; this.rgtbdAdd[insertIndex] = tbd;
        positions[insertIndex] = pos;
        values[insertIndex] = tbd.cloned();
    }

    // Java: public boolean contain(int pos, TBD tbd) {
    pub fn contain(&self, pos: i32, tbd: &TBD) -> bool {
        // java for(int i = 0; i < this.rgdxaAdd.length; ++i)
        // java if (this.rgdxaAdd[i] == pos && this.rgtbdAdd[i].frontCharType == tbd.frontCharType && this.rgtbdAdd[i].tabJustify == tbd.tabJustify)
        // java return true;
        self.rgdxaAdd.as_ref().unwrap().iter().zip(self.rgtbdAdd.as_ref().unwrap()).any(|(p, value)| {
            *p == pos && value.as_ref().is_some_and(|v| v.frontCharType == tbd.frontCharType && v.tabJustify == tbd.tabJustify)
        })
        // Java: return false;
    }

    // Java: public int hashCode() {
    pub fn hashCode(&self) -> i32 {
        // Java: int h = this.itbdDelMax + this.itbdAddMax + this.array_hash(this.rgdxaDel, Integer.MAX_VALUE) + this.array_hash(this.rgdxaClose, Integer.MAX_VALUE) + this.array_hash(this.rgdxaAdd, Integer.MAX_VALUE);
        let mut h = self.itbdDelMax.wrapping_add(self.itbdAddMax);
        for a in [&self.rgdxaDel, &self.rgdxaClose, &self.rgdxaAdd] { h = h.wrapping_add(self.array_hash(a.as_ref(), i32::MAX)); }
        // Java: if (this.rgtbdAdd != null) for(TBD t : this.rgtbdAdd) h += t == null ? 0 : t.hashCode();
        if let Some(values) = &self.rgtbdAdd { for t in values { h = h.wrapping_add(t.as_ref().map_or(0, TBD::hashCode)); } }
        // Java: return h;
        h
    }

    // Java: int array_hash(int[] a, int max) {
    pub fn array_hash(&self, a: Option<&Vec<i32>>, max: i32) -> i32 {
        // Java: int n = a == null ? 0 : a.length; if (n > max) n = max;
        let n = a.map_or(0, Vec::len).min(max as usize);
        // Java: int h = 0; for(int i = 0; i < n; ++i) h += a[i]; return h;
        a.map_or(0, |values| values[..n].iter().fold(0i32, |h, v| h.wrapping_add(*v)))
    }

    // Java: public boolean equals(Object o) {
    pub fn equals(&self, o: &dyn Object) -> bool {
        // Java: if (this == o) return true; else if (!(o instanceof TxTabStops)) return false;
        // java TxTabStops t = (TxTabStops)o;
        let Some(t) = o.as_any().downcast_ref::<TxTabStops>() else { return false };
        if std::ptr::eq(self, t) { return true; }
        // java return this.itbdDelMax == t.itbdDelMax && this.itbdAddMax == t.itbdAddMax && this.array_equals(this.rgdxaDel, t.rgdxaDel, Integer.MAX_VALUE) && this.array_equals(this.rgdxaClose, t.rgdxaClose, Integer.MAX_VALUE) && this.array_equals(this.rgdxaAdd, t.rgdxaAdd, Integer.MAX_VALUE) && this.array_equals(this.rgtbdAdd, t.rgtbdAdd, Integer.MAX_VALUE);
        self.itbdDelMax == t.itbdDelMax && self.itbdAddMax == t.itbdAddMax
            && self.array_equals__intarr_intarr_int(self.rgdxaDel.as_ref(), t.rgdxaDel.as_ref(), i32::MAX)
            && self.array_equals__intarr_intarr_int(self.rgdxaClose.as_ref(), t.rgdxaClose.as_ref(), i32::MAX)
            && self.array_equals__intarr_intarr_int(self.rgdxaAdd.as_ref(), t.rgdxaAdd.as_ref(), i32::MAX)
            && self.array_equals__tbdarr_tbdarr_int(self.rgtbdAdd.as_ref(), t.rgtbdAdd.as_ref(), i32::MAX)
    }

    // Java: boolean array_equals(int[] a, int[] b, int max) {
    pub fn array_equals__intarr_intarr_int(&self, a: Option<&Vec<i32>>, b: Option<&Vec<i32>>, max: i32) -> bool {
        // Java: int na = a == null ? 0 : a.length; int nb = b == null ? 0 : b.length; if (na > max) na = max; if (nb > max) nb = max;
        let na = a.map_or(0, Vec::len).min(max as usize);
        let nb = b.map_or(0, Vec::len).min(max as usize);
        // Java: if (na != nb) return false; else for(int i = 0; i < na; ++i) if (a[i] != b[i]) return false; return true;
        na == nb && (0..na).all(|i| a.unwrap()[i] == b.unwrap()[i])
    }

    // Java: boolean array_equals(TBD[] a, TBD[] b, int max) {
    pub fn array_equals__tbdarr_tbdarr_int(&self, a: Option<&Vec<Option<TBD>>>, b: Option<&Vec<Option<TBD>>>, max: i32) -> bool {
        // Java: int na = a == null ? 0 : a.length; int nb = b == null ? 0 : b.length; if (na > max) na = max; if (nb > max) nb = max;
        let na = a.map_or(0, Vec::len).min(max as usize);
        let nb = b.map_or(0, Vec::len).min(max as usize);
        // Java: if (na != nb) return false; else for(int i = 0; i < na; ++i) if (a[i] == null || !a[i].equals(b[i])) return false; return true;
        na == nb && (0..na).all(|i| a.unwrap()[i].as_ref().is_some_and(|v| b.unwrap()[i].as_ref().is_some_and(|other| v.equals(other))))
    }
}
