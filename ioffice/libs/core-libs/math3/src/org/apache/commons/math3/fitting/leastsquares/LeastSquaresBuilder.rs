// Java: import org.apache.commons.math3.analysis.MultivariateMatrixFunction
use crate::org::apache::commons::math3::analysis::MultivariateMatrixFunction;
// Java: import org.apache.commons.math3.analysis.MultivariateVectorFunction
use crate::org::apache::commons::math3::analysis::MultivariateVectorFunction;
// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem.Evaluation
// Java: import org.apache.commons.math3.linear.ArrayRealVector
use crate::org::apache::commons::math3::linear::ArrayRealVector;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;
// Java: import org.apache.commons.math3.optim.ConvergenceChecker
use crate::org::apache::commons::math3::optim::ConvergenceChecker;
// Java: import org.apache.commons.math3.optim.PointVectorValuePair
use crate::org::apache::commons::math3::optim::PointVectorValuePair;

// Java: public class LeastSquaresBuilder
pub struct LeastSquaresBuilder {
    // Java field: int maxEvaluations
    maxEvaluations: i32,
    // Java field: int maxIterations
    maxIterations: i32,
    // Java field: ConvergenceChecker<Evaluation> checker
    pub(crate) checker: Option<Box<dyn std::any::Any>>,
    // Java field: MultivariateJacobianFunction model
    model: Option<Box<dyn std::any::Any>>,
    // Java field: RealVector target
    pub(crate) target: Option<Box<dyn std::any::Any>>,
    // Java field: RealVector start
    pub(crate) start: Option<Box<dyn std::any::Any>>,
    // Java field: RealMatrix weight
    weight: Option<Box<dyn std::any::Any>>,
    // Java field: boolean lazyEvaluation
    lazyEvaluation: bool,
    // Java field: ParameterValidator paramValidator
    paramValidator: Option<Box<dyn std::any::Any>>,
}

// Java: public class LeastSquaresBuilder
impl LeastSquaresBuilder {
    pub fn new() -> Self {
        LeastSquaresBuilder {
            maxEvaluations: 0,
            maxIterations: 0,
            checker: None,
            model: None,
            target: None,
            start: None,
            weight: None,
            lazyEvaluation: false,
            paramValidator: None,
        }
    }

    // Java: LeastSquaresProblem build()
    pub fn build(&self) -> Box<dyn std::any::Any> {
        // Java: return LeastSquaresFactory.create(model,
                                          // Java: target,
                                          // Java: start,
                                          // Java: weight,
                                          // Java: checker,
                                          // Java: maxEvaluations,
                                          // Java: maxIterations,
                                          // Java: lazyEvaluation,
                                          // Java: paramValidator);
        todo!("LeastSquaresBuilder::build")
    }

    // Java: LeastSquaresBuilder maxEvaluations(final int newMaxEvaluations)
    pub fn maxEvaluations(&mut self, new_max_evaluations: i32) -> &mut Self {
        // Java: this.maxEvaluations = newMaxEvaluations;
        self.maxEvaluations = new_max_evaluations;
        // Java: return this;
        self
    }

    // Java: LeastSquaresBuilder maxIterations(final int newMaxIterations)
    pub fn maxIterations(&mut self, new_max_iterations: i32) -> &mut Self {
        // Java: this.maxIterations = newMaxIterations;
        self.maxIterations = new_max_iterations;
        // Java: return this;
        self
    }

    // Java: LeastSquaresBuilder checker(final ConvergenceChecker<Evaluation> newChecker)
    pub fn checker(&mut self, new_checker: Box<dyn std::any::Any>) -> &mut Self {
        // Java: this.checker = newChecker;
        self.checker = Some(new_checker);
        // Java: return this;
        self
    }

    // Java: LeastSquaresBuilder checkerPair(final ConvergenceChecker<PointVectorValuePair> newChecker)
    pub fn checkerPair(&mut self, _new_checker: Box<dyn std::any::Any>) -> &mut Self {
        // Java: return this.checker(LeastSquaresFactory.evaluationChecker(newChecker));
        todo!("LeastSquaresBuilder::checkerPair")
    }

    // Java: LeastSquaresBuilder model()
    pub fn model_from_functions(&mut self, _model_function: Box<dyn std::any::Any>, _jacobian_function: Box<dyn std::any::Any>) -> &mut Self {
        todo!("LeastSquaresBuilder::model_from_functions")
    }

    // Java: LeastSquaresBuilder model(final MultivariateJacobianFunction newModel)
    pub fn model(&mut self, new_model: Box<dyn std::any::Any>) -> &mut Self {
        // Java: this.model = newModel;
        self.model = Some(new_model);
        // Java: return this;
        self
    }

    // Java: LeastSquaresBuilder target(final RealVector newTarget)
    pub fn target(&mut self, new_target: Box<dyn std::any::Any>) -> &mut Self {
        // Java: this.target = newTarget;
        self.target = Some(new_target);
        // Java: return this;
        self
    }

    // Java: LeastSquaresBuilder target(final double[] newTarget)
    pub fn target_array(&mut self, new_target: Vec<f64>) -> &mut Self {
        // Java: return target(new ArrayRealVector(newTarget, false));
        self.target = Some(Box::new(new_target));
        self
    }

    // Java: LeastSquaresBuilder start(final RealVector newStart)
    pub fn start(&mut self, new_start: Box<dyn std::any::Any>) -> &mut Self {
        // Java: this.start = newStart;
        self.start = Some(new_start);
        // Java: return this;
        self
    }

    // Java: LeastSquaresBuilder start(final double[] newStart)
    pub fn start_array(&mut self, new_start: Vec<f64>) -> &mut Self {
        // Java: return start(new ArrayRealVector(newStart, false));
        self.start = Some(Box::new(new_start));
        self
    }

    // Java: LeastSquaresBuilder weight(final RealMatrix newWeight)
    pub fn weight(&mut self, new_weight: Box<dyn std::any::Any>) -> &mut Self {
        // Java: this.weight = newWeight;
        self.weight = Some(new_weight);
        // Java: return this;
        self
    }

    // Java: LeastSquaresBuilder lazyEvaluation(final boolean newValue)
    pub fn lazyEvaluation(&mut self, new_value: bool) -> &mut Self {
        // Java: lazyEvaluation = newValue;
        self.lazyEvaluation = new_value;
        // Java: return this;
        self
    }

    // Java: LeastSquaresBuilder parameterValidator(final ParameterValidator newValidator)
    pub fn parameterValidator(&mut self, new_validator: Box<dyn std::any::Any>) -> &mut Self {
        // Java: paramValidator = newValidator;
        self.paramValidator = Some(new_validator);
        // Java: return this;
        self
    }

}
