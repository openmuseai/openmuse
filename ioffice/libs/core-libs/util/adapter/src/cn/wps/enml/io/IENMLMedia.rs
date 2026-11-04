use super::ENMLMediaType::ENMLMediaType;

// Java: public interface IENMLMedia
pub trait IENMLMedia {
    // Java: abstract ENMLMediaType getType()
    fn getType(&self) -> ENMLMediaType;

    // Java: abstract String getHash()
    fn getHash(&self) -> String;

}
