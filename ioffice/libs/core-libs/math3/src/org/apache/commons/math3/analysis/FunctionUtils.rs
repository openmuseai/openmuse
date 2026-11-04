#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unreachable_code)]
// Java: import org.apache.commons.math3.analysis.differentiation.DerivativeStructure
use crate::org::apache::commons::math3::analysis::differentiation::DerivativeStructure;
// Java: import org.apache.commons.math3.analysis.differentiation.MultivariateDifferentiableFunction
use crate::org::apache::commons::math3::analysis::differentiation::MultivariateDifferentiableFunction;
// Java: import org.apache.commons.math3.analysis.differentiation.MultivariateDifferentiableVectorFunction
use crate::org::apache::commons::math3::analysis::differentiation::MultivariateDifferentiableVectorFunction;
// Java: import org.apache.commons.math3.analysis.differentiation.UnivariateDifferentiableFunction
use crate::org::apache::commons::math3::analysis::differentiation::UnivariateDifferentiableFunction;
// Java: import org.apache.commons.math3.analysis.function.Identity
use crate::org::apache::commons::math3::analysis::function::Identity;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class FunctionUtils
pub struct FunctionUtils;

// Java: public class FunctionUtils
impl FunctionUtils {
    // Java: FunctionUtils()
    pub fn new() -> Self { FunctionUtils }

    // Java: static UnivariateFunction compose(final UnivariateFunction ... f)
    pub fn compose(f: Vec<Box<dyn Fn(f64) -> f64>>) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new UnivariateFunction()
        Box::new(move |x: f64| {
            // Java: /** {@inheritDoc} */
            // Java: public double value(double x)
            // Java: double r = x;
            let mut r = x;
            // Java: for (int i = f.length - 1; i >= 0; i--)
            for i in (0..f.len()).rev() {
                // Java: r = f[i].value(r);
                r = f[i](r);
            }
            // Java: return r;
            r
        })
    }

    // Java: static UnivariateDifferentiableFunction compose(final UnivariateDifferentiableFunction ... f)
    pub fn compose_diff(f: Vec<Box<dyn Fn(f64) -> f64>>) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new UnivariateDifferentiableFunction()
        Box::new(move |x: f64| {
            // Java: /** {@inheritDoc} */
            // Java: public double value(final double t)
            // Java: double r = t;
            let mut r = x;
            // Java: for (int i = f.length - 1; i >= 0; i--)
            for i in (0..f.len()).rev() {
                // Java: r = f[i].value(r);
                r = f[i](r);
            }
            // Java: return r;
            r
            // Java: /** {@inheritDoc} */
            // Java: public DerivativeStructure value(final DerivativeStructure t)
            // Java: DerivativeStructure r = t;
            // Java: for (int i = f.length - 1; i >= 0; i--)
            // Java: r = f[i].value(r);
            // Java: return r;
        })
    }

    // Java: static DifferentiableUnivariateFunction compose(final DifferentiableUnivariateFunction ... f)
    pub fn compose_deprecated(f: Vec<Box<dyn Fn(f64) -> f64>>) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new DifferentiableUnivariateFunction()
        Box::new(move |x: f64| {
            // Java: /** {@inheritDoc} */
            // Java: public double value(double x)
            // Java: double r = x;
            let mut r = x;
            // Java: for (int i = f.length - 1; i >= 0; i--)
            for i in (0..f.len()).rev() {
                // Java: r = f[i].value(r);
                r = f[i](r);
            }
            // Java: return r;
            r
            // Java: /** {@inheritDoc} */
            // Java: public UnivariateFunction derivative()
            // Java: return new UnivariateFunction()
            // Java: /** {@inheritDoc} */
            // Java: public double value(double x)
            // Java: double p = 1;
            // Java: double r = x;
            // Java: for (int i = f.length - 1; i >= 0; i--)
            // Java: p *= f[i].derivative().value(r);
            // Java: r = f[i].value(r);
            // Java: return p;
        })
    }

    // Java: static UnivariateFunction add(final UnivariateFunction ... f)
    pub fn add(f: Vec<Box<dyn Fn(f64) -> f64>>) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new UnivariateFunction()
        Box::new(move |x: f64| {
            // Java: /** {@inheritDoc} */
            // Java: public double value(double x)
            // Java: double r = f[0].value(x);
            let mut r = f[0](x);
            // Java: for (int i = 1; i < f.length; i++)
            for i in 1..f.len() {
                // Java: r += f[i].value(x);
                r += f[i](x);
            }
            // Java: return r;
            r
        })
    }

    // Java: static UnivariateDifferentiableFunction add(final UnivariateDifferentiableFunction ... f)
    pub fn add_diff(f: Vec<Box<dyn Fn(f64) -> f64>>) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new UnivariateDifferentiableFunction()
        Box::new(move |x: f64| {
            // Java: /** {@inheritDoc} */
            // Java: public double value(final double t)
            // Java: double r = f[0].value(t);
            let mut r = f[0](x);
            // Java: for (int i = 1; i < f.length; i++)
            for i in 1..f.len() {
                // Java: r += f[i].value(t);
                r += f[i](x);
            }
            // Java: return r;
            r
            // Java: /** {@inheritDoc}
            // Java: * @throws DimensionMismatchException if functions are not consistent with each other
            // Java: */
            // Java: public DerivativeStructure value(final DerivativeStructure t)
            // Java: throws DimensionMismatchException
            // Java: DerivativeStructure r = f[0].value(t);
            // Java: for (int i = 1; i < f.length; i++)
            // Java: r = r.add(f[i].value(t));
            // Java: return r;
        })
    }

    // Java: static DifferentiableUnivariateFunction add(final DifferentiableUnivariateFunction ... f)
    pub fn add_deprecated(f: Vec<Box<dyn Fn(f64) -> f64>>) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new DifferentiableUnivariateFunction()
        Box::new(move |x: f64| {
            // Java: /** {@inheritDoc} */
            // Java: public double value(double x)
            // Java: double r = f[0].value(x);
            let mut r = f[0](x);
            // Java: for (int i = 1; i < f.length; i++)
            for i in 1..f.len() {
                // Java: r += f[i].value(x);
                r += f[i](x);
            }
            // Java: return r;
            r
            // Java: /** {@inheritDoc} */
            // Java: public UnivariateFunction derivative()
            // Java: return new UnivariateFunction()
            // Java: /** {@inheritDoc} */
            // Java: public double value(double x)
            // Java: double r = f[0].derivative().value(x);
            // Java: for (int i = 1; i < f.length; i++)
            // Java: r += f[i].derivative().value(x);
            // Java: return r;
        })
    }

    // Java: static UnivariateFunction multiply(final UnivariateFunction ... f)
    pub fn multiply(f: Vec<Box<dyn Fn(f64) -> f64>>) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new UnivariateFunction()
        Box::new(move |x: f64| {
            // Java: /** {@inheritDoc} */
            // Java: public double value(double x)
            // Java: double r = f[0].value(x);
            let mut r = f[0](x);
            // Java: for (int i = 1; i < f.length; i++)
            for i in 1..f.len() {
                // Java: r *= f[i].value(x);
                r *= f[i](x);
            }
            // Java: return r;
            r
        })
    }

    // Java: static UnivariateDifferentiableFunction multiply(final UnivariateDifferentiableFunction ... f)
    pub fn multiply_diff(f: Vec<Box<dyn Fn(f64) -> f64>>) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new UnivariateDifferentiableFunction()
        Box::new(move |x: f64| {
            // Java: /** {@inheritDoc} */
            // Java: public double value(final double t)
            // Java: double r = f[0].value(t);
            let mut r = f[0](x);
            // Java: for (int i = 1; i < f.length; i++)
            for i in 1..f.len() {
                // Java: r  *= f[i].value(t);
                r *= f[i](x);
            }
            // Java: return r;
            r
            // Java: /** {@inheritDoc} */
            // Java: public DerivativeStructure value(final DerivativeStructure t)
            // Java: DerivativeStructure r = f[0].value(t);
            // Java: for (int i = 1; i < f.length; i++)
            // Java: r = r.multiply(f[i].value(t));
            // Java: return r;
        })
    }

    // Java: static DifferentiableUnivariateFunction multiply(final DifferentiableUnivariateFunction ... f)
    pub fn multiply_deprecated(f: Vec<Box<dyn Fn(f64) -> f64>>) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new DifferentiableUnivariateFunction()
        Box::new(move |x: f64| {
            // Java: /** {@inheritDoc} */
            // Java: public double value(double x)
            // Java: double r = f[0].value(x);
            let mut r = f[0](x);
            // Java: for (int i = 1; i < f.length; i++)
            for i in 1..f.len() {
                // Java: r *= f[i].value(x);
                r *= f[i](x);
            }
            // Java: return r;
            r
            // Java: /** {@inheritDoc} */
            // Java: public UnivariateFunction derivative()
            // Java: return new UnivariateFunction()
            // Java: /** {@inheritDoc} */
            // Java: public double value(double x)
            // Java: double sum = 0;
            // Java: for (int i = 0; i < f.length; i++)
            // Java: double prod = f[i].derivative().value(x);
            // Java: for (int j = 0; j < f.length; j++)
            // Java: if (i != j)
            // Java: prod *= f[j].value(x);
            // Java: sum += prod;
            // Java: return sum;
        })
    }

    // Java: static UnivariateFunction combine()
    pub fn combine(f: Box<dyn Fn(f64, f64) -> f64>, g: Box<dyn Fn(f64) -> f64>, h: Box<dyn Fn(f64) -> f64>) -> Box<dyn Fn(f64) -> f64> {
        Box::new(move |x: f64| f(g(x), h(x)))
    }

    // Java: static MultivariateFunction collector()
    pub fn collector(combiner: Box<dyn Fn(f64, f64) -> f64>, init: f64) -> Box<dyn Fn(&[f64]) -> f64> {
        Box::new(move |a: &[f64]| {
            let mut r = init;
            for &v in a { r = combiner(r, v); }
            r
        })
    }

    // Java: static MultivariateFunction collector()
    pub fn collector_with_final(combiner: Box<dyn Fn(f64, f64) -> f64>, finisher: Box<dyn Fn(f64) -> f64>, init: f64) -> Box<dyn Fn(&[f64]) -> f64> {
        Box::new(move |a: &[f64]| {
            let mut r = init;
            for &v in a { r = combiner(r, v); }
            finisher(r)
        })
    }

    // Java: static UnivariateFunction fix1stArgument()
    pub fn fix1stArgument(f: Box<dyn Fn(f64, f64) -> f64>, fixed: f64) -> Box<dyn Fn(f64) -> f64> {
        Box::new(move |x: f64| f(fixed, x))
    }

    // Java: static UnivariateFunction fix2ndArgument()
    pub fn fix2ndArgument(f: Box<dyn Fn(f64, f64) -> f64>, fixed: f64) -> Box<dyn Fn(f64) -> f64> {
        Box::new(move |x: f64| f(x, fixed))
    }

    // Java: static double[] sample(UnivariateFunction f, double min, double max, int n)
    pub fn sample(f: &dyn Fn(f64) -> f64, min: f64, max: f64, n: i32) -> Vec<f64> {
        if n <= 0 { panic!("n must be positive"); }
        let step = (max - min) / n as f64;
        (0..n).map(|i| f(min + (i as f64 + 0.5) * step)).collect()
    }

    // Java: static DifferentiableUnivariateFunction toDifferentiableUnivariateFunction(final UnivariateDifferentiableFunction f)
    pub fn toDifferentiableUnivariateFunction(_f: &dyn Fn(f64) -> f64) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new DifferentiableUnivariateFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double value(final double x)
        // Java: return f.value(x);
        // Java: /** {@inheritDoc} */
        // Java: public UnivariateFunction derivative()
        // Java: return new UnivariateFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double value(final double x)
        // Java: return f.value(new DerivativeStructure(1, 1, 0, x)).getPartialDerivative(1);
        todo!()
    }

    // Java: static UnivariateDifferentiableFunction toUnivariateDifferential(final DifferentiableUnivariateFunction f)
    pub fn toUnivariateDifferential(_f: &dyn Fn(f64) -> f64) -> Box<dyn Fn(f64) -> f64> {
        // Java: return new UnivariateDifferentiableFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double value(final double x)
        // Java: return f.value(x);
        // Java: /** {@inheritDoc}
        // Java: * @exception NumberIsTooLargeException if derivation order is greater than 1
        // Java: */
        // Java: public DerivativeStructure value(final DerivativeStructure t)
        // Java: throws NumberIsTooLargeException
        // Java: switch (t.getOrder())
        // Java: case 0 :
        // Java: return new DerivativeStructure(t.getFreeParameters(), 0, f.value(t.getValue()));
        // Java: case 1 :
        // Java: final int parameters = t.getFreeParameters();
        // Java: final double[] derivatives = new double[parameters + 1];
        // Java: derivatives[0] = f.value(t.getValue());
        // Java: final double fPrime = f.derivative().value(t.getValue());
        // Java: int[] orders = new int[parameters];
        // Java: for (int i = 0; i < parameters; ++i)
        // Java: orders[i] = 1;
        // Java: derivatives[i + 1] = fPrime * t.getPartialDerivative(orders);
        // Java: orders[i] = 0;
        // Java: return new DerivativeStructure(parameters, 1, derivatives);
        // Java: default :
        // Java: throw new NumberIsTooLargeException(t.getOrder(), 1, true);
        todo!()
    }

    // Java: static DifferentiableMultivariateFunction toDifferentiableMultivariateFunction(final MultivariateDifferentiableFunction f)
    pub fn toDifferentiableMultivariateFunction(_f: &dyn Fn(&[f64]) -> f64) -> Box<dyn Fn(&[f64]) -> f64> {
        // Java: return new DifferentiableMultivariateFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double value(final double[] x)
        // Java: return f.value(x);
        // Java: /** {@inheritDoc} */
        // Java: public MultivariateFunction partialDerivative(final int k)
        // Java: return new MultivariateFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double value(final double[] x)
        // Java: final int n = x.length;
        // Java: // delegate computation to underlying function
        // Java: final DerivativeStructure[] dsX = new DerivativeStructure[n];
        // Java: for (int i = 0; i < n; ++i)
        // Java: if (i == k)
        // Java: dsX[i] = new DerivativeStructure(1, 1, 0, x[i]);
        // Java: else
        // Java: dsX[i] = new DerivativeStructure(1, 1, x[i]);
        // Java: final DerivativeStructure y = f.value(dsX);
        // Java: // extract partial derivative
        // Java: return y.getPartialDerivative(1);
        // Java: /** {@inheritDoc} */
        // Java: public MultivariateVectorFunction gradient()
        // Java: return new MultivariateVectorFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double[] value(final double[] x)
        // Java: final int n = x.length;
        // Java: // delegate computation to underlying function
        // Java: final DerivativeStructure[] dsX = new DerivativeStructure[n];
        // Java: for (int i = 0; i < n; ++i)
        // Java: dsX[i] = new DerivativeStructure(n, 1, i, x[i]);
        // Java: final DerivativeStructure y = f.value(dsX);
        // Java: // extract gradient
        // Java: final double[] gradient = new double[n];
        // Java: final int[] orders = new int[n];
        // Java: for (int i = 0; i < n; ++i)
        // Java: orders[i]   = 1;
        // Java: gradient[i] = y.getPartialDerivative(orders);
        // Java: orders[i]   = 0;
        // Java: return gradient;
        todo!()
    }

    // Java: static MultivariateDifferentiableFunction toMultivariateDifferentiableFunction(final DifferentiableMultivariateFunction f)
    pub fn toMultivariateDifferentiableFunction(_f: &dyn Fn(&[f64]) -> f64) -> Box<dyn Fn(&[f64]) -> f64> {
        // Java: return new MultivariateDifferentiableFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double value(final double[] x)
        // Java: return f.value(x);
        // Java: /** {@inheritDoc}
        // Java: * @exception NumberIsTooLargeException if derivation order is higher than 1
        // Java: * @exception DimensionMismatchException if numbers of free parameters are inconsistent
        // Java: */
        // Java: public DerivativeStructure value(final DerivativeStructure[] t)
        // Java: throws DimensionMismatchException, NumberIsTooLargeException
        // Java: // check parameters and orders limits
        // Java: final int parameters = t[0].getFreeParameters();
        // Java: final int order      = t[0].getOrder();
        // Java: final int n          = t.length;
        // Java: if (order > 1)
        // Java: throw new NumberIsTooLargeException(order, 1, true);
        // Java: // check all elements in the array are consistent
        // Java: for (int i = 0; i < n; ++i)
        // Java: if (t[i].getFreeParameters() != parameters)
        // Java: throw new DimensionMismatchException(t[i].getFreeParameters(), parameters);
        // Java: if (t[i].getOrder() != order)
        // Java: throw new DimensionMismatchException(t[i].getOrder(), order);
        // Java: // delegate computation to underlying function
        // Java: final double[] point = new double[n];
        // Java: for (int i = 0; i < n; ++i)
        // Java: point[i] = t[i].getValue();
        // Java: final double value      = f.value(point);
        // Java: final double[] gradient = f.gradient().value(point);
        // Java: // merge value and gradient into one DerivativeStructure
        // Java: final double[] derivatives = new double[parameters + 1];
        // Java: derivatives[0] = value;
        // Java: final int[] orders = new int[parameters];
        // Java: for (int i = 0; i < parameters; ++i)
        // Java: orders[i] = 1;
        // Java: for (int j = 0; j < n; ++j)
        // Java: derivatives[i + 1] += gradient[j] * t[j].getPartialDerivative(orders);
        // Java: orders[i] = 0;
        // Java: return new DerivativeStructure(parameters, order, derivatives);
        todo!()
    }

    // Java: static DifferentiableMultivariateVectorFunction toDifferentiableMultivariateVectorFunction(final MultivariateDifferentiableVectorFunction f)
    pub fn toDifferentiableMultivariateVectorFunction(_f: &dyn Fn(&[f64]) -> Vec<f64>) -> Box<dyn Fn(&[f64]) -> Vec<f64>> {
        // Java: return new DifferentiableMultivariateVectorFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double[] value(final double[] x)
        // Java: return f.value(x);
        // Java: /** {@inheritDoc} */
        // Java: public MultivariateMatrixFunction jacobian()
        // Java: return new MultivariateMatrixFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double[][] value(final double[] x)
        // Java: final int n = x.length;
        // Java: // delegate computation to underlying function
        // Java: final DerivativeStructure[] dsX = new DerivativeStructure[n];
        // Java: for (int i = 0; i < n; ++i)
        // Java: dsX[i] = new DerivativeStructure(n, 1, i, x[i]);
        // Java: final DerivativeStructure[] y = f.value(dsX);
        // Java: // extract Jacobian
        // Java: final double[][] jacobian = new double[y.length][n];
        // Java: final int[] orders = new int[n];
        // Java: for (int i = 0; i < y.length; ++i)
        // Java: for (int j = 0; j < n; ++j)
        // Java: orders[j]      = 1;
        // Java: jacobian[i][j] = y[i].getPartialDerivative(orders);
        // Java: orders[j]      = 0;
        // Java: return jacobian;
        todo!()
    }

    // Java: static MultivariateDifferentiableVectorFunction toMultivariateDifferentiableVectorFunction(final DifferentiableMultivariateVectorFunction f)
    pub fn toMultivariateDifferentiableVectorFunction(_f: &dyn Fn(&[f64]) -> Vec<f64>) -> Box<dyn Fn(&[f64]) -> Vec<f64>> {
        // Java: return new MultivariateDifferentiableVectorFunction()
        // Java: /** {@inheritDoc} */
        // Java: public double[] value(final double[] x)
        // Java: return f.value(x);
        // Java: /** {@inheritDoc}
        // Java: * @exception NumberIsTooLargeException if derivation order is higher than 1
        // Java: * @exception DimensionMismatchException if numbers of free parameters are inconsistent
        // Java: */
        // Java: public DerivativeStructure[] value(final DerivativeStructure[] t)
        // Java: throws DimensionMismatchException, NumberIsTooLargeException
        // Java: // check parameters and orders limits
        // Java: final int parameters = t[0].getFreeParameters();
        // Java: final int order      = t[0].getOrder();
        // Java: final int n          = t.length;
        // Java: if (order > 1)
        // Java: throw new NumberIsTooLargeException(order, 1, true);
        // Java: // check all elements in the array are consistent
        // Java: for (int i = 0; i < n; ++i)
        // Java: if (t[i].getFreeParameters() != parameters)
        // Java: throw new DimensionMismatchException(t[i].getFreeParameters(), parameters);
        // Java: if (t[i].getOrder() != order)
        // Java: throw new DimensionMismatchException(t[i].getOrder(), order);
        // Java: // delegate computation to underlying function
        // Java: final double[] point = new double[n];
        // Java: for (int i = 0; i < n; ++i)
        // Java: point[i] = t[i].getValue();
        // Java: final double[] value      = f.value(point);
        // Java: final double[][] jacobian = f.jacobian().value(point);
        // Java: // merge value and Jacobian into a DerivativeStructure array
        // Java: final DerivativeStructure[] merged = new DerivativeStructure[value.length];
        // Java: for (int k = 0; k < merged.length; ++k)
        // Java: final double[] derivatives = new double[parameters + 1];
        // Java: derivatives[0] = value[k];
        // Java: final int[] orders = new int[parameters];
        // Java: for (int i = 0; i < parameters; ++i)
        // Java: orders[i] = 1;
        // Java: for (int j = 0; j < n; ++j)
        // Java: derivatives[i + 1] += jacobian[k][j] * t[j].getPartialDerivative(orders);
        // Java: orders[i] = 0;
        // Java: merged[k] = new DerivativeStructure(parameters, order, derivatives);
        // Java: return merged;
        todo!()
    }
}
