// Java: import java.io.Serializable
// (no Rust equivalent needed for Serializable)
// Java: import java.util.Locale
pub type Locale = String;

// Java: public interface Localizable extends Serializable
pub trait Localizable {
    // Java: abstract String getSourceString()
    fn getSourceString(&self) -> String;

    // Java: abstract String getLocalizedString(Locale locale)
    fn getLocalizedString(&self, locale: &Locale) -> String;
}
