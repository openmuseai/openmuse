// Rust stub for ProtectionListener (auto-generated)
// Original: cn/wps/moffice/writer/core/protection/ProtectionListener.java
use crate::cn::wps::moffice::writer::core::protection::KDocumentProtection::KDocumentProtection;

pub trait ProtectionListener {
    fn onProtectionChanged(&mut self, protection: &KDocumentProtection);
}
