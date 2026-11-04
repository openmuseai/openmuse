// Native java.lang.Character(char) leaf adapter for the reference Android
// runtime. Tables exhaust all 65,536 UTF-16 units; no Unicode-version guesses.
// java public static char toLowerCase(char ch)
pub fn toLowerCase(ch:u16)->u16{let i=ch as usize*2;u16::from_le_bytes(include_bytes!("character-lower.bin")[i..i+2].try_into().unwrap())}
// java public static char toUpperCase(char ch)
pub fn toUpperCase(ch:u16)->u16{let i=ch as usize*2;u16::from_le_bytes(include_bytes!("character-upper.bin")[i..i+2].try_into().unwrap())}
// rust: String.equalsIgnoreCase against an ASCII constant, using the original
// UTF-16 regionMatchesCI comparisons. Surrogates cannot equal ASCII units.
pub fn equalsIgnoreCaseAscii(text:&[u16],ascii:&[u8])->bool{
    if text.len()!=ascii.len(){return false;}
    for (c1,c2)in text.iter().copied().zip(ascii.iter().map(|c|*c as u16)){
        // java if (c1 == c2) { continue; }
        if c1==c2{continue;}
        // java char u1 = Character.toUpperCase(c1); char u2 = Character.toUpperCase(c2);
        let u1=toUpperCase(c1);let u2=toUpperCase(c2);
        // java if (u1 == u2 || Character.toLowerCase(u1) == Character.toLowerCase(u2)) { continue; }
        if u1==u2||toLowerCase(u1)==toLowerCase(u2){continue;}
        return false;
    }
    true
}
// java String.regionMatchesCI(char[], char[]): original per-unit comparisons.
pub fn equalsIgnoreCase(a:&[u16],b:&[u16])->bool{
    if a.len()!=b.len(){return false;}
    for (&c1,&c2)in a.iter().zip(b){
        // java if (c1 == c2) { continue; }
        if c1==c2{continue;}
        // java char u1 = Character.toUpperCase(c1); char u2 = Character.toUpperCase(c2);
        let u1=toUpperCase(c1);let u2=toUpperCase(c2);
        // java if (u1 == u2 || Character.toLowerCase(u1) == Character.toLowerCase(u2)) { continue; }
        if u1==u2||toLowerCase(u1)==toLowerCase(u2){continue;}
        return false;
    }true
}
// Reference Android java.lang.Character(char) properties, all UTF16 units.
// java public static boolean isDigit(char ch)
pub fn isDigit(ch:u16)->bool{include_bytes!("character-flags.bin")[ch as usize]&1!=0}
// java public static boolean isLetter(char ch)
pub fn isLetter(ch:u16)->bool{include_bytes!("character-flags.bin")[ch as usize]&2!=0}
// java public static boolean isUpperCase(char ch)
pub fn isUpperCase(ch:u16)->bool{include_bytes!("character-flags.bin")[ch as usize]&4!=0}
// java public static boolean isLowerCase(char ch)
pub fn isLowerCase(ch:u16)->bool{include_bytes!("character-flags.bin")[ch as usize]&8!=0}
// java public static int getNumericValue(char ch)
// rust: R53 original Android observations, every UTF16 unit, signed little-endian int.
pub fn getNumericValue(ch:u16)->i32{let i=ch as usize*4;i32::from_le_bytes(include_bytes!("character-numeric.bin")[i..i+4].try_into().unwrap())}
