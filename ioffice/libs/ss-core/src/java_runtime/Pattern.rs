//! Native Android Pattern character classes. Membership comes from the original
//! runtime for every Unicode code point; Rust's Unicode version is not substituted.
use std::sync::LazyLock;
static D:LazyLock<String>=LazyLock::new(||classBody(include_bytes!("regex-d.bin")));
static W:LazyLock<String>=LazyLock::new(||classBody(include_bytes!("regex-w.bin")));
static S:LazyLock<String>=LazyLock::new(||classBody(include_bytes!("regex-s.bin")));
fn classBody(bits:&[u8])->String{
    let mut body=String::new();let mut c=0u32;
    while c<0x110000{
        if bits[c as usize/8]&(1<<(c%8))==0{c+=1;continue;}
        let start=c;c+=1;while c<0x110000&&bits[c as usize/8]&(1<<(c%8))!=0{c+=1;}
        body.push_str(&format!("\\x{{{start:x}}}"));if c-start>1{body.push('-');body.push_str(&format!("\\x{{{:x}}}",c-1));}
    }
    body
}
// java Pattern.compile(pattern), for the original manual-recogniser pattern syntax.
// Capturing groups and branch order remain the caller's original expression.
pub fn compile(pattern:&str)->Result<regex::Regex,regex::Error>{
    let mut out=String::new();let mut chars=pattern.chars();
    while let Some(c)=chars.next(){
        if c!='\\'{out.push(c);continue;}
        let Some(next)=chars.next()else{out.push(c);break;};
        let body=match next{'d'|'D'=>Some(&*D),'w'|'W'=>Some(&*W),'s'|'S'=>Some(&*S),_=>None};
        if let Some(body)=body{out.push('[');if next.is_ascii_uppercase(){out.push('^');}out.push_str(body);out.push(']');}
        else{out.push('\\');out.push(next);}
    }
    regex::Regex::new(&out)
}
// java Pattern.compile(pattern,flags).matcher(text).matches(): noncapturing
// full-region wrapper, with the Java caller's CASE_INSENSITIVE flag when set.
pub fn compileMatches(pattern:&str,caseInsensitive:bool)->Result<regex::Regex,regex::Error>{
    compile(&if caseInsensitive{format!(r"\A(?i:{pattern})\z")}else{format!(r"\A(?:{pattern})\z")})
}
