// Java: import java.lang.annotation.Documented
// Java: import java.lang.annotation.Retention
// Java: import java.lang.annotation.RetentionPolicy

// Java: public interface Internal
pub trait Internal {
    // Java: abstract String value()
    fn value(&self) -> String {
        String::new()
    }
}
