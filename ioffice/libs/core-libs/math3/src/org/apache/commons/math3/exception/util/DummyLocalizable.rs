// Java: import java.util.Locale

// Java field: long serialVersionUID

// Java: public class DummyLocalizable implements Localizable
pub struct DummyLocalizable {
    // Java field: String source
    pub(crate) source: Box<dyn std::any::Any>,
}

// Java: public class DummyLocalizable implements Localizable
impl DummyLocalizable {
    // Java: DummyLocalizable(final String source)
    pub fn DummyLocalizable(&self) {
        // Java: this.source = source;
    }

    // Java: String getSourceString()
    pub fn getSourceString(&self) -> String {
        // Java: return source;
        todo!()
    }

    // Java: String getLocalizedString(Locale locale)
    pub fn getLocalizedString(&self) -> String {
        // Java: return source;
        todo!()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return source;
        todo!()
    }

}

