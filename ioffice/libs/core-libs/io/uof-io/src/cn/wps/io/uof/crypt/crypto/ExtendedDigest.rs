use super::Digest::Digest;

// Java: public interface ExtendedDigest extends Digest
pub trait ExtendedDigest: Digest {
    // Java: abstract int getByteLength()
    fn getByteLength(&self) -> i32;

}
