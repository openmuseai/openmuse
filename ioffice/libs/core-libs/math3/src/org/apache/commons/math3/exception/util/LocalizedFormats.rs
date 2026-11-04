// Java: import java.util.Locale
use super::Localizable::{Localizable, Locale};
// Java: import java.util.MissingResourceException
// Java: import java.util.ResourceBundle

// Java: public enum LocalizedFormats implements Localizable
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct LocalizedFormats {
    // Java field: String sourceFormat
    pub(crate) sourceFormat: String,
}

// Java: public enum LocalizedFormats implements Localizable
#[allow(non_snake_case, non_upper_case_globals)]
impl LocalizedFormats {
    pub const fn new(_source: &'static str) -> Self {
        LocalizedFormats { sourceFormat: String::new() }
    }

    pub fn from_str(source: &str) -> Self {
        LocalizedFormats { sourceFormat: source.to_string() }
    }

    pub fn ARGUMENT_OUTSIDE_DOMAIN() -> Self { Self::from_str("Argument {0} outside domain [{1} ; {2}]") }
    pub fn ARRAY_SIZE_EXCEEDS_MAX_VARIABLES() -> Self { Self::from_str("array size cannot be greater than {0}") }
    pub fn ARRAY_SIZES_SHOULD_HAVE_DIFFERENCE_1() -> Self { Self::from_str("array sizes should have difference 1 ({0} != {1} + 1)") }
    pub fn ARRAY_SUMS_TO_ZERO() -> Self { Self::from_str("array sums to zero") }
    pub fn AT_LEAST_ONE_COLUMN() -> Self { Self::from_str("matrix must have at least one column") }
    pub fn AT_LEAST_ONE_ROW() -> Self { Self::from_str("matrix must have at least one row") }
    pub fn BANDWIDTH() -> Self { Self::from_str("bandwidth ({0})") }
    pub fn BINOMIAL_INVALID_PARAMETERS_ORDER() -> Self { Self::from_str("must have n >= k for binomial coefficient (n, k), got k = {0}, n = {1}") }
    pub fn BINOMIAL_NEGATIVE_PARAMETER() -> Self { Self::from_str("must have n >= 0 for binomial coefficient (n, k), got n = {0}") }
    pub fn CANNOT_CLEAR_STATISTIC_CONSTRUCTED_FROM_EXTERNAL_MOMENTS() -> Self { Self::from_str("statistics constructed from external moments cannot be cleared") }
    pub fn CANNOT_COMPUTE_0TH_ROOT_OF_UNITY() -> Self { Self::from_str("cannot compute 0-th root of unity, indefinite result") }
    pub fn CONVERGENCE_FAILED() -> Self { Self::from_str("convergence failed") }
    pub fn DIMENSIONS_MISMATCH() -> Self { Self::from_str("dimensions mismatch") }
    pub fn DIMENSIONS_MISMATCH_SIMPLE() -> Self { Self::from_str("{0} != {1}") }
    pub fn EMPTY_POLYNOMIALS_COEFFICIENTS_ARRAY() -> Self { Self::from_str("empty polynomials coefficients array") }
    pub fn EVALUATIONS() -> Self { Self::from_str("evaluations") }
    pub fn EVALUATION() -> Self { Self::from_str("evaluation") }
    pub fn FAILED_BRACKETING() -> Self { Self::from_str("number of iterations={4}, maximum iterations={5}, initial={6}, lower bound={7}, upper bound={8}, final a value={0}, final b value={1}, f(a)={2}, f(b)={3}") }
    pub fn ILLEGAL_STATE() -> Self { Self::from_str("illegal state") }
    pub fn INDEX() -> Self { Self::from_str("index ({0})") }
    pub fn INDEX_OUT_OF_RANGE() -> Self { Self::from_str("index {0} out of allowed range [{1}, {2}]") }
    pub fn INFINITE_BOUND() -> Self { Self::from_str("interval bounds must be finite") }
    pub fn ITERATIONS() -> Self { Self::from_str("iterations") }
    pub fn LOWER_BOUND_NOT_BELOW_UPPER_BOUND() -> Self { Self::from_str("lower bound ({0}) must be strictly less than upper bound ({1})") }
    pub fn MAX_COUNT_EXCEEDED() -> Self { Self::from_str("maximal count ({0}) exceeded") }
    pub fn MAX_ITERATIONS_EXCEEDED() -> Self { Self::from_str("maximal number of iterations ({0}) exceeded") }
    pub fn NO_CONVERGENCE_WITH_ANY_START_POINT() -> Self { Self::from_str("none of the {0} start points lead to convergence") }
    pub fn NO_DATA() -> Self { Self::from_str("no data") }
    pub fn NO_OPTIMUM_COMPUTED_YET() -> Self { Self::from_str("no optimum computed yet") }
    pub fn NOT_FINITE_NUMBER() -> Self { Self::from_str("{0} is not a finite number") }
    pub fn NOT_POSITIVE_DEFINITE_MATRIX() -> Self { Self::from_str("not positive definite matrix") }
    pub fn NULL_NOT_ALLOWED() -> Self { Self::from_str("null is not allowed") }
    pub fn NUMBER_TOO_LARGE() -> Self { Self::from_str("{0} is larger than the maximum ({1})") }
    pub fn NUMBER_TOO_SMALL() -> Self { Self::from_str("{0} is smaller than the minimum ({1})") }
    pub fn NUMBER_TOO_LARGE_BOUND_EXCLUDED() -> Self { Self::from_str("{0} is larger than, or equal to, the maximum ({1})") }
    pub fn NUMBER_TOO_SMALL_BOUND_EXCLUDED() -> Self { Self::from_str("{0} is smaller than, or equal to, the minimum ({1})") }
    pub fn OUT_OF_RANGE() -> Self { Self::from_str("out of range") }
    pub fn OUT_OF_RANGE_SIMPLE() -> Self { Self::from_str("{0} out of [{1}, {2}] range") }
    pub fn OVERFLOW() -> Self { Self::from_str("overflow") }
    pub fn SIMPLE_MESSAGE() -> Self { Self::from_str("{0}") }
    pub fn SINGULAR_MATRIX() -> Self { Self::from_str("matrix is singular") }
    pub fn UNSUPPORTED_OPERATION() -> Self { Self::from_str("unsupported operation") }
    pub fn ARITHMETIC_EXCEPTION() -> Self { Self::from_str("arithmetic exception") }
    pub fn USER_EXCEPTION() -> Self { Self::from_str("exception generated in user code") }
    pub fn ZERO_DENOMINATOR() -> Self { Self::from_str("denominator must be different from 0") }
    pub fn ZERO_NORM() -> Self { Self::from_str("zero norm") }
    pub fn CANNOT_NORMALIZE_A_ZERO_NORM_VECTOR() -> Self { Self::from_str("cannot normalize a zero norm vector") }
    pub fn ZERO_NORM_FOR_ROTATION_DEFINING_VECTOR() -> Self { Self::from_str("zero norm for rotation defining vector") }
    pub fn NOT_STRICTLY_INCREASING_SEQUENCE() -> Self { Self::from_str("points {3} and {2} are not strictly increasing ({1} >= {0})") }
    pub fn NOT_STRICTLY_DECREASING_SEQUENCE() -> Self { Self::from_str("points {3} and {2} are not strictly decreasing ({1} <= {0})") }
    pub fn NOT_INCREASING_SEQUENCE() -> Self { Self::from_str("points {3} and {2} are not increasing ({1} > {0})") }
    pub fn NOT_DECREASING_SEQUENCE() -> Self { Self::from_str("points {3} and {2} are not decreasing ({1} < {0})") }
    pub fn NON_SYMMETRIC_MATRIX() -> Self { Self::from_str("non symmetric matrix: the difference between entries at ({0},{1}) and ({1},{0}) is larger than {2}") }
    pub fn NO_REGRESSORS() -> Self { Self::from_str("Regression model must include at least one regressor") }
    pub fn INVALID_REGRESSION_OBSERVATION() -> Self { Self::from_str("length of regressor array = {0} does not match the number of variables = {1} in the model") }
    pub fn TOO_MANY_REGRESSORS() -> Self { Self::from_str("too many regressors ({0}) specified, only {1} in the model") }
    pub fn NOT_ENOUGH_DATA_FOR_NUMBER_OF_PREDICTORS() -> Self { Self::from_str("not enough data ({0} rows) for this many predictors ({1} predictors)") }
    pub fn INDEX_LARGER_THAN_MAX() -> Self { Self::from_str("the index specified: {0} is larger than the current maximal index {1}") }

    // Java: String getSourceString()
    pub fn getSourceString(&self) -> String {
        // Java: return sourceFormat;
        self.sourceFormat.clone()
    }

    // Java: String getLocalizedString(final Locale locale)
    pub fn getLocalizedString(&self, _locale: &Locale) -> String {
        // Java: try
        // Java: final String path = LocalizedFormats.class.getName().replaceAll("\\.", "/");
        // Java: ResourceBundle bundle =
        // Java: ResourceBundle.getBundle("assets/" + path, locale);
        // Java: if (bundle.getLocale().getLanguage().equals(locale.getLanguage()))
        {
            // Java: // the value of the resource is the translated format
            // Java: return bundle.getString(toString());
        }
        // Java: // do nothing here
        // Java: // either the locale is not supported or the resource is unknown
        // Java: // don't translate and fall back to using the source format
        // Java: return sourceFormat;
        self.sourceFormat.clone()
    }
}

impl Localizable for LocalizedFormats {
    fn getSourceString(&self) -> String {
        self.sourceFormat.clone()
    }

    fn getLocalizedString(&self, _locale: &Locale) -> String {
        self.sourceFormat.clone()
    }
}
