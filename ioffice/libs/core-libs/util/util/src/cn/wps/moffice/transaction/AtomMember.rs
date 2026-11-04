// Java: import java.lang.annotation.ElementType
// Java: import java.lang.annotation.Retention
// Java: import java.lang.annotation.RetentionPolicy
// Java: import java.lang.annotation.Target

// Java: public interface AtomMember
pub trait AtomMember {
    // Java: abstract int value()
    fn value(&self) -> i32 { 0 }
}
