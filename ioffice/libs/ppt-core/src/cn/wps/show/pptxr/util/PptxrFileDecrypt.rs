// Rust stub for PptxrFileDecrypt (auto-generated)
// Original: cn/wps/show/pptxr/util/PptxrFileDecrypt.java
use crate::*;

pub struct PptxrFileDecrypt {
    // TODO: fields
}

impl PptxrFileDecrypt {
    pub fn new() -> Self { todo!() }
    pub fn setDecryptTimeListener(listener: DecryptTimeListener) { todo!() }
    pub fn getDecryptTimeListener() -> DecryptTimeListener { todo!() }
    pub fn decryptFile(path: String, raf: RandomAccessFile, pres: KmoPresentation, passProvider: Box<dyn KsoFilterPasswdProvider>) -> String { todo!() }
    pub fn decryptOOXMLFile(path: String, passwd: String) -> Box<dyn std::io::Read> { todo!() }
}

pub trait DecryptTimeListener {}
