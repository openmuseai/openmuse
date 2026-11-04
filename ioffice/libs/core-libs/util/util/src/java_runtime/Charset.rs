// Native Java Charset leaf transport. The Android GB2312 table contains
// [encoded length, byte 0, byte 1] for each UTF-16 unit, captured on PKM110.
// A supplementary scalar is unmappable and uses one replacement byte, matching
// CharsetEncoder's default replacement for one well-formed surrogate pair.
pub fn gb2312Bytes(text:&str)->Vec<u8>{
    let table=include_bytes!("gb2312-android.bin");
    let mut bytes=Vec::with_capacity(text.len());
    for ch in text.chars(){
        if ch as u32>0xffff{bytes.push(b'?');}else{
            let offset=ch as usize*3;let n=table[offset]as usize;
            bytes.extend_from_slice(&table[offset+1..offset+1+n]);
        }
    }
    bytes
}
