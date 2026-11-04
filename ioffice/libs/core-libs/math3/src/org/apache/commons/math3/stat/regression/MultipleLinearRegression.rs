// Java: public interface MultipleLinearRegression
pub trait MultipleLinearRegression {
    // Java: abstract double[] estimateRegressionParameters()
    fn estimateRegressionParameters(&self) -> Vec<f64>;

    // Java: abstract double[][] estimateRegressionParametersVariance()
    fn estimateRegressionParametersVariance(&self) -> Vec<Vec<f64>>;

    // Java: abstract double[] estimateResiduals()
    fn estimateResiduals(&self) -> Vec<f64>;

    // Java: abstract double estimateRegressandVariance()
    fn estimateRegressandVariance(&self) -> f64;

    // Java: abstract double[] estimateRegressionParametersStandardErrors()
    fn estimateRegressionParametersStandardErrors(&self) -> Vec<f64>;

}
