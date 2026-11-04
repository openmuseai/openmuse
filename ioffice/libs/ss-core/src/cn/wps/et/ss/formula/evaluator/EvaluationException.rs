// java package cn.wps.et.ss.formula.evaluator;

use crate::cn::wps::et::ss::formula::eval::ErrorEval::ErrorEval;

// java public final class EvaluationException extends Exception
pub struct EvaluationException {
    // java private final ErrorEval _errorEval;
    _errorEval: &'static ErrorEval,
}

// java static initializer: one exception instance for each ErrorEval singleton.
static NULL_VALUE: EvaluationException = EvaluationException { _errorEval: ErrorEval::NULL_INTERSECTION };
static DIV_ZERO_VALUE: EvaluationException = EvaluationException { _errorEval: ErrorEval::DIV_ZERO };
static VALUE_INVALID_VALUE: EvaluationException = EvaluationException { _errorEval: ErrorEval::VALUE_INVALID };
static REF_INVALID_VALUE: EvaluationException = EvaluationException { _errorEval: ErrorEval::REF_INVALID };
static NAME_INVALID_VALUE: EvaluationException = EvaluationException { _errorEval: ErrorEval::NAME_INVALID };
static NUM_ERROR_VALUE: EvaluationException = EvaluationException { _errorEval: ErrorEval::NUM_ERROR };
static NA_VALUE: EvaluationException = EvaluationException { _errorEval: ErrorEval::NA };

impl EvaluationException {
    pub const NULL: &'static Self = &NULL_VALUE;
    pub const DIV_ZERO: &'static Self = &DIV_ZERO_VALUE;
    pub const VALUE_INVALID: &'static Self = &VALUE_INVALID_VALUE;
    pub const REF_INVALID: &'static Self = &REF_INVALID_VALUE;
    pub const NAME_INVALID: &'static Self = &NAME_INVALID_VALUE;
    pub const NUM_ERROR: &'static Self = &NUM_ERROR_VALUE;
    pub const NA: &'static Self = &NA_VALUE;

    // java public static EvaluationException valueOf(ErrorEval ee)
    pub fn valueOf(error: &ErrorEval) -> &'static Self {
        // java switch (ee.getErrorCode())
        match error.getErrorCode() {
            0 => Self::NULL,
            7 => Self::DIV_ZERO,
            15 => Self::VALUE_INVALID,
            23 => Self::REF_INVALID,
            29 => Self::NAME_INVALID,
            36 => Self::NUM_ERROR,
            42 => Self::NA,
            // java default: throw new RuntimeException();
            _ => panic!("RuntimeException"),
        }
    }

    // java public ErrorEval getErrorEval() { return this._errorEval; }
    pub fn getErrorEval(&self) -> &'static ErrorEval { self._errorEval }
}
