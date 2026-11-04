use crate::native_runtime::Character;
// java public class CompareUtil {
pub struct CompareUtil;
impl CompareUtil{
    // java public CompareUtil(){super();}
    pub fn new()->Self{Self}
    // java public static boolean isStringEqual(String src,int start,String pattern,boolean ignoreCase) {
    pub fn isStringEqual(src:&str,start:i32,pattern:&str,ignoreCase:bool)->bool{
        // java int patLen=pattern.length();int srcLen=src.length();if(start+patLen>srcLen){return false;}else if(!ignoreCase&&start==0&&srcLen==patLen){return src.equals(pattern);}else{
        let pat:Vec<_>=pattern.encode_utf16().collect();let src:Vec<_>=src.encode_utf16().collect();let patLen=pat.len()as i32;let srcLen=src.len()as i32;if start.wrapping_add(patLen)>srcLen{false}else if !ignoreCase&&start==0&&srcLen==patLen{src==pat}else{
            // java for(int i=0;i<patLen;++i){char l=pattern.charAt(i);char r=src.charAt(start++);if(l!=r&&(!ignoreCase||Character.toUpperCase(l)!=Character.toUpperCase(r))){return false;}}return true;
            let mut start=start;for i in 0..patLen{let l=pat[i as usize];let r=src[start as usize];start=start.wrapping_add(1);if l!=r&&(!ignoreCase||Character::toUpperCase(l)!=Character::toUpperCase(r)){return false;}}true
        }
    }
}
