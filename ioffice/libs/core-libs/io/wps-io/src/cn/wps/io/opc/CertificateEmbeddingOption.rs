// Java: public enum CertificateEmbeddingOption
pub enum CertificateEmbeddingOption {
    IN_CERTIFICATE_PART,
    IN_SIGNATURE_PART,
    NOT_EMBEDDED,
}

// Java: public enum CertificateEmbeddingOption
impl CertificateEmbeddingOption {
    // Java: CertificateEmbeddingOption()
    pub fn new() -> Self {
        CertificateEmbeddingOption::IN_CERTIFICATE_PART
    }
}
