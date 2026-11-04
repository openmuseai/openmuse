#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;

// Java: public class Predicates
pub struct Predicates;

// Java: public class Predicates
impl Predicates {
    // Java: Predicates()
    // Java: super();

    // Java: static public static <T> Predicate<T> alwaysTrue()
    pub fn alwaysTrue() -> Box<dyn super::Predicate::Predicate> {
        // Java: return Predicates.ObjectPredicate.ALWAYS_TRUE.<T>withNarrowedType();
        Box::new(ObjectPredicate::AlwaysTrue)
    }

    // Java: static public static <T> Predicate<T> equalTo(T target)
    pub fn equalTo(target: Option<Box<dyn Any>>) -> Box<dyn super::Predicate::Predicate> {
        // Java: return (Predicate<T>)(target == null ? isNull() : new IsEqualToPredicate(target));
        if target.is_none() {
            Self::isNull()
        } else {
            Box::new(IsEqualToPredicate { target })
        }
    }

    // Java: static public static <T> Predicate<T> isNull()
    pub fn isNull() -> Box<dyn super::Predicate::Predicate> {
        // Java: return Predicates.ObjectPredicate.IS_NULL.<T>withNarrowedType();
        Box::new(ObjectPredicate::IsNull)
    }
}

// Java: public class IsEqualToPredicate
pub struct IsEqualToPredicate {
    // Java field: T target
    pub target: Option<Box<dyn Any>>,
}

// Java: public class IsEqualToPredicate
impl IsEqualToPredicate {
    // Java: IsEqualToPredicate(T target)
    pub fn new(target: Option<Box<dyn Any>>) -> Self {
        // Java: super();
        // Java: this.target = target;
        IsEqualToPredicate { target }
    }

    // Java: boolean apply(T t)
    pub fn apply(&self, _t: &dyn Any) -> bool {
        // Java: return this.target.equals(t);
        false
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.target.hashCode();
        0
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, _obj: &dyn Any) -> bool {
        // Java: if (obj instanceof IsEqualToPredicate)
        {
            // Java: IsEqualToPredicate<?> that = (IsEqualToPredicate)obj;
            // Java: return this.target.equals(that.target);
        }
        // Java: else
        {
            // Java: return false;
            false
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "Predicates.equalTo(" + this.target + ")";
        String::from("Predicates.equalTo(...)")
    }
}

impl super::Predicate::Predicate for IsEqualToPredicate {
    fn apply(&self, _t: &dyn Any) -> bool {
        false
    }

    fn equals(&self, _obj: &dyn Any) -> bool {
        false
    }
}

// Java: public enum ObjectPredicate
#[derive(Clone, Copy)]
pub enum ObjectPredicate {
    AlwaysTrue,
    AlwaysFalse,
    IsNull,
    NotNull,
}

// Java: public enum ObjectPredicate
impl ObjectPredicate {
    // Java: ObjectPredicate()

    // Java: <T> Predicate<T> withNarrowedType()
    pub fn withNarrowedType(self) -> Box<dyn super::Predicate::Predicate> {
        // Java: return (Predicate<T>)this;
        Box::new(self)
    }
}

impl super::Predicate::Predicate for ObjectPredicate {
    fn apply(&self, _o: &dyn Any) -> bool {
        match self {
            ObjectPredicate::AlwaysTrue => true,
            ObjectPredicate::AlwaysFalse => false,
            ObjectPredicate::IsNull => false,
            ObjectPredicate::NotNull => true,
        }
    }

    fn equals(&self, _var1: &dyn Any) -> bool {
        false
    }
}
