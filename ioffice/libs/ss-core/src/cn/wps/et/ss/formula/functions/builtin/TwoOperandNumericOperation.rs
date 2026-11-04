// java package cn.wps.et.ss.formula.functions.builtin;
use super::super::util::MathX::MathX;
use crate::cn::wps::et::ss::formula::eval::{
    ErrorEval::ErrorEval, NumberEval::NumberEval, OperandResolver::OperandResolver, ValueRef,
};
use crate::cn::wps::et::ss::formula::evaluator::EvalResult;
use std::sync::Arc;
// java public abstract class TwoOperandNumericOperation extends ValueOpFunction
pub struct TwoOperandNumericOperation {
    op: u8,
    round15: bool,
}
impl TwoOperandNumericOperation {
    // java anonymous subclasses override evaluate(d0,d1), and Round* call super(true).
    pub const AddEval: Self = Self {
        op: 3,
        round15: false,
    };
    pub const RoundAddEval: Self = Self {
        op: 3,
        round15: true,
    };
    pub const SubtractEval: Self = Self {
        op: 4,
        round15: false,
    };
    pub const RoundSubtractEval: Self = Self {
        op: 4,
        round15: true,
    };
    pub const MultiplyEval: Self = Self {
        op: 5,
        round15: false,
    };
    pub const DivideEval: Self = Self {
        op: 6,
        round15: false,
    };
    pub const PowerEval: Self = Self {
        op: 7,
        round15: false,
    };
    // java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec)
    pub fn evaluateOnce(
        &self,
        _funcRAV: i8,
        args: &[ValueRef],
        ec: &crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext,
    ) -> EvalResult<ValueRef> {
        // java if (args.length != 2) return ErrorEval.VALUE_INVALID;
        if args.len() != 2 {
            return Ok(Arc::new(ErrorEval::VALUE_INVALID));
        }
        // java singleOperandEvaluate: getSingleValue, coerceValueToDouble, optional MathX.round15.
        let mut operands = [0.0; 2];
        for i in 0..2 {
            let ve = match OperandResolver::getSingleValue(&args[i], ec)? {
                Ok(v) => v,
                Err(e) => return Ok(Arc::new(e)),
            };
            let value = match OperandResolver::coerceValueToDouble(&ve)? {
                Ok(n) => n,
                Err(e) => return Ok(Arc::new(e)),
            };
            operands[i] = if self.round15 {
                MathX::round15(value).doubleValue()
            } else {
                value
            };
        }
        let [d0, d1] = operands;
        // java DivideEval.evaluate: d1 == 0.0 throws EvaluationException.DIV_ZERO.
        if self.op == 6 && d1 == 0.0 {
            return Ok(Arc::new(ErrorEval::DIV_ZERO));
        }
        let result = match self.op {
            3 => d0 + d1,
            4 => d0 - d1,
            5 => d0 * d1,
            6 => d0 / d1,
            // java PowerEval.evaluate: zero^zero NaN; negative root with odd reciprocal is negative.
            7 if d0 == 0.0 && d1 == 0.0 => f64::NAN,
            7 if d0 < 0.0
                && d1 > 0.0
                && d1 < 1.0
                && MathX::round15(1.0 / d1).doubleValue() % 2.0 == 1.0 =>
            {
                -(-d0).powf(d1)
            }
            7 => d0.powf(d1),
            _ => unreachable!(),
        };
        // java result == 0.0 && this != SubtractEval returns NumberEval.ZERO.
        if result == 0.0 && !(self.op == 4 && !self.round15) {
            return Ok(Arc::new(NumberEval::ZERO));
        }
        // java NaN/Infinite returns ErrorEval.NUM_ERROR.
        if !result.is_finite() {
            return Ok(Arc::new(ErrorEval::NUM_ERROR));
        }
        Ok(Arc::new(NumberEval::NumberEval(result)))
    }
}

impl super::super::base::MixOpFunction::MixOpFunction for TwoOperandNumericOperation{fn valueArgRequired(&self,_i:usize,_args:&[ValueRef],_ec:&crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext)->bool{true}fn evaluateOnce(&self,rav:i8,args:&[ValueRef],ec:&crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext)->EvalResult<ValueRef>{TwoOperandNumericOperation::evaluateOnce(self,rav,args,ec)}}
