#![allow(dead_code, non_snake_case)]
// Java: public class EquationFactory
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;

// java new EquationLinearLayouter / EquationTextLayouter / EquationALayouter ...
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EquationLayouterKind {
    Linear,
    Text,
    A,
    B,
    D,
    F,
    I,
    L,
    O,
    R,
    S,
    X,
}

// java IEquationLayouter create(...) — null, throw EquationException, or a layouter
pub enum EquationCreate {
    Null,
    Throw,
    Kind(EquationLayouterKind),
}

// java public class EquationFactory
pub struct EquationFactory;

impl EquationFactory {
    // java public static IEquationLayouter create(...)
    pub fn create(eq_result: &EquationResult, end: i32) -> EquationCreate {
        // java if (eqResult.getEndCP() < end) return new EquationLinearLayouter(...)
        if eq_result.getEndCP() < end {
            return EquationCreate::Kind(EquationLayouterKind::Linear);
        }
        // java else if (2 == eqResult.getState()) return null;
        if eq_result.getState() == 2 {
            return EquationCreate::Null;
        }
        // java else if (1 == eqResult.getState()) return new EquationTextLayouter(...)
        if eq_result.getState() == 1 {
            return EquationCreate::Kind(EquationLayouterKind::Text);
        }
        // java switch (eqResult.getMainOP())
        match eq_result.getMainOP() {
            'a' => EquationCreate::Kind(EquationLayouterKind::A),
            'b' => EquationCreate::Kind(EquationLayouterKind::B),
            'd' => EquationCreate::Kind(EquationLayouterKind::D),
            'f' => EquationCreate::Kind(EquationLayouterKind::F),
            'i' => EquationCreate::Kind(EquationLayouterKind::I),
            'l' => EquationCreate::Kind(EquationLayouterKind::L),
            'o' => EquationCreate::Kind(EquationLayouterKind::O),
            'r' => EquationCreate::Kind(EquationLayouterKind::R),
            's' => {
                // java if (eqResult.getValuesSize() > 1) return EquationALayouter else EquationSLayouter
                if eq_result.getValuesSize() > 1 {
                    EquationCreate::Kind(EquationLayouterKind::A)
                } else {
                    EquationCreate::Kind(EquationLayouterKind::S)
                }
            }
            'x' => EquationCreate::Kind(EquationLayouterKind::X),
            // java case 'c','e','g',... and default: throw new EquationException()
            _ => EquationCreate::Throw,
        }
    }
}
