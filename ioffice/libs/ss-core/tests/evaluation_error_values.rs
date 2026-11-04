use ss_core::cn::wps::et::ss::formula::eval::{BlankEval::BlankEval, ErrorEval::ErrorEval, MissingArgEval::MissingArgEval};
use ss_core::cn::wps::et::ss::formula::evaluator::EvaluationException;

#[test]
fn java_error_values_and_checked_exceptions_reuse_their_exact_singletons() {
    for (code, text) in [(0, "#NULL!"), (7, "#DIV/0!"), (15, "#VALUE!"), (23, "#REF!"), (29, "#NAME?"), (36, "#NUM!"), (42, "#N/A")] {
        let error = ErrorEval::valueOf(code);
        assert!(std::ptr::eq(error, ErrorEval::valueOf__string(text).unwrap()));
        assert_eq!(error.getErrorCode(), code);
        assert_eq!(error.hashCode(), code);
        assert_eq!(ErrorEval::getText(code), text);
        assert!(error.equals(error));
        assert!(!error.equals(BlankEval::instance));
        let exception = EvaluationException::valueOf(error);
        assert!(std::ptr::eq(exception.getErrorEval(), error));
        assert!(std::ptr::eq(exception, EvaluationException::valueOf(error)));
    }
    assert!(std::ptr::eq(EvaluationException::DIV_ZERO, EvaluationException::valueOf(ErrorEval::DIV_ZERO)));
    assert_eq!(ErrorEval::DIV_ZERO.toString(), "cn.wps.et.ss.formula.eval.ErrorEval [#DIV/0!]");
    assert_eq!(ErrorEval::getText(-1), "~non~std~err(-1)~");
    assert!(std::panic::catch_unwind(|| ErrorEval::valueOf(8)).is_err());
    for text in ["", "#div/0!", "#N/A ", " #N/A", "N/A"] { assert!(ErrorEval::valueOf__string(text).is_none()); }
    assert!(BlankEval::instance.equals(BlankEval::instance));
    assert!(!BlankEval::instance.equals(MissingArgEval::instance));
    assert_eq!(BlankEval::instance.hashCode(), 17);
}
