use std::sync::{Mutex, OnceLock};

// Java: public class KpPaged
pub struct KpPaged;

fn pages() -> &'static Mutex<Vec<Vec<u8>>> {
    static PAGES: OnceLock<Mutex<Vec<Vec<u8>>>> = OnceLock::new();
    PAGES.get_or_init(|| {
        // Java: bigPageLst.add(nullBytes) so bh 0 is empty
        Mutex::new(vec![Vec::new()])
    })
}

impl KpPaged {
    // Java: static KpPaged memory1()
    pub fn memory1() -> KpPaged {
        KpPaged
    }

    // Java: int allocate(int byteSize, int dataCount, boolean isMap)
    pub fn allocate(&self, byte_size: i32, data_count: i32, is_map: bool) -> i32 {
        let mut pages = pages().lock().expect("KpPaged");
        let mut buff = vec![0u8; byte_size.max(0) as usize];
        if !buff.is_empty() {
            if is_map {
                // Java: KcByte.byteArrayIntToByte — low 8 bits
                buff[0] = data_count as u8;
            } else {
                let be = data_count.to_be_bytes();
                let n = be.len().min(buff.len());
                buff[..n].copy_from_slice(&be[..n]);
            }
        }
        pages.push(buff);
        (pages.len() - 1) as i32
    }

    // Java: byte[] bhBuff(int bh)
    pub fn bh_buff(&self, bh: i32) -> Vec<u8> {
        let pages = pages().lock().expect("KpPaged");
        if bh < 0 || bh as usize >= pages.len() {
            Vec::new()
        } else {
            pages[bh as usize].clone()
        }
    }

    pub fn write_at(&self, bh: i32, off: usize, bytes: &[u8]) {
        let mut pages = pages().lock().expect("KpPaged");
        if bh < 0 || bh as usize >= pages.len() {
            return;
        }
        let buff = &mut pages[bh as usize];
        let end = off + bytes.len();
        if end <= buff.len() {
            buff[off..end].copy_from_slice(bytes);
        }
    }
}
