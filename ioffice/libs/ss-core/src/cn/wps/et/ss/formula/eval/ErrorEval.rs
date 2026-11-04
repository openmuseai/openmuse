// java package cn.wps.et.ss.formula.eval;

use super::IValueEval;
use crate::org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants;

// java public final class ErrorEval implements IValueEval
pub struct ErrorEval {
    // java private final int _errorCode;
    _errorCode: i32,
}

static NULL_INTERSECTION_VALUE: ErrorEval = ErrorEval { _errorCode: 0 };
static DIV_ZERO_VALUE: ErrorEval = ErrorEval { _errorCode: 7 };
static VALUE_INVALID_VALUE: ErrorEval = ErrorEval { _errorCode: 15 };
static REF_INVALID_VALUE: ErrorEval = ErrorEval { _errorCode: 23 };
static NAME_INVALID_VALUE: ErrorEval = ErrorEval { _errorCode: 29 };
static NUM_ERROR_VALUE: ErrorEval = ErrorEval { _errorCode: 36 };
static NA_VALUE: ErrorEval = ErrorEval { _errorCode: 42 };

impl ErrorEval {
    // java public static final ErrorEval NULL_INTERSECTION = new ErrorEval(0);
    pub const NULL_INTERSECTION: &'static Self = &NULL_INTERSECTION_VALUE;
    // java public static final ErrorEval DIV_ZERO = new ErrorEval(7);
    pub const DIV_ZERO: &'static Self = &DIV_ZERO_VALUE;
    // java public static final ErrorEval VALUE_INVALID = new ErrorEval(15);
    pub const VALUE_INVALID: &'static Self = &VALUE_INVALID_VALUE;
    // java public static final ErrorEval REF_INVALID = new ErrorEval(23);
    pub const REF_INVALID: &'static Self = &REF_INVALID_VALUE;
    // java public static final ErrorEval NAME_INVALID = new ErrorEval(29);
    pub const NAME_INVALID: &'static Self = &NAME_INVALID_VALUE;
    // java public static final ErrorEval NUM_ERROR = new ErrorEval(36);
    pub const NUM_ERROR: &'static Self = &NUM_ERROR_VALUE;
    // java public static final ErrorEval NA = new ErrorEval(42);
    pub const NA: &'static Self = &NA_VALUE;

    // java public static ErrorEval valueOf(int errorCode)
    pub fn valueOf(error_code: i32) -> &'static Self {
        // java switch (errorCode)
        match error_code {
            0 => Self::NULL_INTERSECTION,
            7 => Self::DIV_ZERO,
            15 => Self::VALUE_INVALID,
            23 => Self::REF_INVALID,
            29 => Self::NAME_INVALID,
            36 => Self::NUM_ERROR,
            42 => Self::NA,
            // java default: throw new RuntimeException("Unexpected error code (" + errorCode + ")");
            _ => panic!("Unexpected error code ({error_code})"),
        }
    }

    // java public static ErrorEval valueOf(String value)
    pub fn valueOf__string(value: &str) -> Option<&'static Self> {
        // java if (value.length() >= 4 && value.charAt(0) == '#'), exact case-sensitive comparisons.
        match value {
            "#NULL!" => Some(Self::NULL_INTERSECTION),
            "#DIV/0!" => Some(Self::DIV_ZERO),
            "#VALUE!" => Some(Self::VALUE_INVALID),
            "#REF!" => Some(Self::REF_INVALID),
            "#NAME?" => Some(Self::NAME_INVALID),
            "#NUM!" => Some(Self::NUM_ERROR),
            "#N/A" => Some(Self::NA),
            // java return null;
            _ => None,
        }
    }

    // java public static String getText(int errorCode)
    pub fn getText(error_code: i32) -> String {
        // java return ErrorConstants.isValidCode(errorCode) ? ErrorConstants.getText(errorCode) : "~non~std~err(" + errorCode + ")~";
        if ErrorConstants::isValidCode(error_code) { ErrorConstants::getText(error_code) }
        else { format!("~non~std~err({error_code})~") }
    }

    // java public int getErrorCode()
    pub fn getErrorCode(&self) -> i32 { self._errorCode }
    // java public boolean equals(Object o) { return o == this; }
    pub fn equals(&self, other: &dyn IValueEval) -> bool {
        other.as_any().downcast_ref::<Self>().is_some_and(|other| std::ptr::eq(self, other))
    }
    // java public int hashCode() { return this._errorCode; }
    pub fn hashCode(&self) -> i32 { self._errorCode }
    // java public String toString()
    pub fn toString(&self) -> String {
        format!("cn.wps.et.ss.formula.eval.ErrorEval [{}]", Self::getText(self._errorCode))
    }
}

impl IValueEval for ErrorEval {
    fn as_any(&self) -> &dyn std::any::Any { self }
}
