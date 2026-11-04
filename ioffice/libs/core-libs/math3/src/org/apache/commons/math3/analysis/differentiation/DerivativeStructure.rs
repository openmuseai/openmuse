#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut)]

use crate::org::apache::commons::math3::util::FastMath::FastMath;
use crate::org::apache::commons::math3::util::MathArrays::MathArrays;
use crate::org::apache::commons::math3::util::MathUtils::MathUtils;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 20120730;

// Java: public class is
// Java: public class are

// Java: public class DerivativeStructure
#[derive(Clone, Debug)]
pub struct DerivativeStructure {
    // Java field: DSCompiler compiler
    free_parameters: i32,
    order: i32,
    // Java field: double[] data
    data: Vec<f64>,
}

// Java: public class DataTransferObject
pub struct DataTransferObject {
    // Java field: int variables
    pub(crate) variables: i32,
    // Java field: int order
    pub(crate) order: i32,
    // Java field: double[] data
    pub(crate) data: Vec<f64>,
}

// Java: public class DerivativeStructure
impl DerivativeStructure {
    fn get_compiler_size(parameters: i32, order: i32) -> usize {
        let n = parameters as usize;
        let k = order as usize;
        let mut result = 1usize;
        for i in 1..=k {
            result = result * (n + i) / i;
        }
        result
    }

    // Java: DerivativeStructure(final DSCompiler compiler)
    fn new_from_compiler(free_parameters: i32, order: i32, size: usize) -> Self {
        // Java: this.compiler = compiler;
        // Java: this.data     = new double[compiler.getSize()];
        DerivativeStructure {
            free_parameters,
            order,
            data: vec![0.0; size],
        }
    }

    // Java: DerivativeStructure(final int parameters, final int order)
    pub fn new(parameters: i32, order: i32) -> Self {
        let size = Self::get_compiler_size(parameters, order);
        Self::new_from_compiler(parameters, order, size)
    }

    // Java: DerivativeStructure(final int parameters, final int order, final double value)
    pub fn new_with_value(parameters: i32, order: i32, value: f64) -> Self {
        let mut ds = Self::new(parameters, order);
        // Java: this.data[0] = value;
        ds.data[0] = value;
        ds
    }

    // Java: DerivativeStructure()
    pub fn new_with_index(parameters: i32, order: i32, index: i32, value: f64) -> Self {
        let mut ds = Self::new_with_value(parameters, order, value);
        if index >= parameters {
            panic!("NumberIsTooLargeException");
        }
        if order > 0 {
            let idx = Self::get_compiler_size(index, order);
            if idx < ds.data.len() {
                ds.data[idx] = 1.0;
            }
        }
        ds
    }

    // Java: DerivativeStructure()
    pub fn new_linear_combination_2(a1: f64, ds1: &DerivativeStructure, a2: f64, ds2: &DerivativeStructure) -> Self {
        let mut result = Self::new(ds1.free_parameters, ds1.order);
        for i in 0..result.data.len() {
            result.data[i] = a1 * ds1.data[i] + a2 * ds2.data[i];
        }
        result
    }

    // Java: DerivativeStructure()
    pub fn new_linear_combination_3(a1: f64, ds1: &DerivativeStructure, a2: f64, ds2: &DerivativeStructure, a3: f64, ds3: &DerivativeStructure) -> Self {
        let mut result = Self::new(ds1.free_parameters, ds1.order);
        for i in 0..result.data.len() {
            result.data[i] = a1 * ds1.data[i] + a2 * ds2.data[i] + a3 * ds3.data[i];
        }
        result
    }

    // Java: DerivativeStructure()
    pub fn new_linear_combination_4(a1: f64, ds1: &DerivativeStructure, a2: f64, ds2: &DerivativeStructure, a3: f64, ds3: &DerivativeStructure, a4: f64, ds4: &DerivativeStructure) -> Self {
        let mut result = Self::new(ds1.free_parameters, ds1.order);
        for i in 0..result.data.len() {
            result.data[i] = a1 * ds1.data[i] + a2 * ds2.data[i] + a3 * ds3.data[i] + a4 * ds4.data[i];
        }
        result
    }

    // Java: DerivativeStructure(final int parameters, final int order, final double ... derivatives)
    pub fn new_with_derivatives(parameters: i32, order: i32, derivatives: &[f64]) -> Self {
        let mut ds = Self::new(parameters, order);
        if derivatives.len() != ds.data.len() {
            panic!("DimensionMismatchException");
        }
        ds.data.copy_from_slice(derivatives);
        ds
    }

    // Java: DerivativeStructure(final DerivativeStructure ds)
    pub fn copy(ds: &DerivativeStructure) -> Self {
        // Java: this.compiler = ds.compiler;
        // Java: this.data     = ds.data.clone();
        DerivativeStructure {
            free_parameters: ds.free_parameters,
            order: ds.order,
            data: ds.data.clone(),
        }
    }

    // Java: int getFreeParameters()
    pub fn getFreeParameters(&self) -> i32 {
        // Java: return compiler.getFreeParameters();
        self.free_parameters
    }

    // Java: int getOrder()
    pub fn getOrder(&self) -> i32 {
        // Java: return compiler.getOrder();
        self.order
    }

    // Java: DerivativeStructure createConstant(final double c)
    pub fn createConstant(&self, c: f64) -> DerivativeStructure {
        // Java: return new DerivativeStructure(getFreeParameters(), getOrder(), c);
        Self::new_with_value(self.getFreeParameters(), self.getOrder(), c)
    }

    // Java: double getReal()
    pub fn getReal(&self) -> f64 {
        // Java: return data[0];
        self.data[0]
    }

    // Java: double getValue()
    pub fn getValue(&self) -> f64 {
        // Java: return data[0];
        self.data[0]
    }

    // Java: double getPartialDerivative(final int ... orders)
    pub fn getPartialDerivative(&self, orders: &[i32]) -> f64 {
        // Java: return data[compiler.getPartialDerivativeIndex(orders)];
        todo!("requires DSCompiler")
    }

    // Java: double[] getAllDerivatives()
    pub fn getAllDerivatives(&self) -> Vec<f64> {
        // Java: return data.clone();
        self.data.clone()
    }

    // Java: DerivativeStructure add(final double a)
    pub fn add_f64(&self, a: f64) -> DerivativeStructure {
        // Java: final DerivativeStructure ds = new DerivativeStructure(this);
        let mut ds = Self::copy(self);
        // Java: ds.data[0] += a;
        ds.data[0] += a;
        // Java: return ds;
        ds
    }

    // Java: DerivativeStructure add(final DerivativeStructure a)
    pub fn add(&self, a: &DerivativeStructure) -> DerivativeStructure {
        let mut ds = Self::copy(self);
        for i in 0..ds.data.len() {
            ds.data[i] = self.data[i] + a.data[i];
        }
        ds
    }

    // Java: DerivativeStructure subtract(final double a)
    pub fn subtract_f64(&self, a: f64) -> DerivativeStructure {
        // Java: return add(-a);
        self.add_f64(-a)
    }

    // Java: DerivativeStructure subtract(final DerivativeStructure a)
    pub fn subtract(&self, a: &DerivativeStructure) -> DerivativeStructure {
        let mut ds = Self::copy(self);
        for i in 0..ds.data.len() {
            ds.data[i] = self.data[i] - a.data[i];
        }
        ds
    }

    // Java: DerivativeStructure multiply(final int n)
    pub fn multiply_int(&self, n: i32) -> DerivativeStructure {
        // Java: return multiply((double) n);
        self.multiply_f64(n as f64)
    }

    // Java: DerivativeStructure multiply(final double a)
    pub fn multiply_f64(&self, a: f64) -> DerivativeStructure {
        // Java: final DerivativeStructure ds = new DerivativeStructure(this);
        let mut ds = Self::copy(self);
        // Java: for (int i = 0; i < ds.data.length; ++i)
        for i in 0..ds.data.len() {
            // Java: ds.data[i] *= a;
            ds.data[i] *= a;
        }
        // Java: return ds;
        ds
    }

    // Java: DerivativeStructure multiply(final DerivativeStructure a)
    pub fn multiply(&self, a: &DerivativeStructure) -> DerivativeStructure {
        let mut result = Self::new(self.free_parameters, self.order);
        todo!("requires DSCompiler");
    }

    // Java: DerivativeStructure divide(final double a)
    pub fn divide_f64(&self, a: f64) -> DerivativeStructure {
        // Java: final DerivativeStructure ds = new DerivativeStructure(this);
        let mut ds = Self::copy(self);
        // Java: for (int i = 0; i < ds.data.length; ++i)
        for i in 0..ds.data.len() {
            // Java: ds.data[i] /= a;
            ds.data[i] /= a;
        }
        // Java: return ds;
        ds
    }

    // Java: DerivativeStructure divide(final DerivativeStructure a)
    pub fn divide(&self, a: &DerivativeStructure) -> DerivativeStructure {
        let mut result = Self::new(self.free_parameters, self.order);
        todo!("requires DSCompiler");
    }

    // Java: DerivativeStructure remainder(final double a)
    pub fn remainder_f64(&self, a: f64) -> DerivativeStructure {
        // Java: final DerivativeStructure ds = new DerivativeStructure(this);
        let mut ds = Self::copy(self);
        // Java: ds.data[0] = FastMath.IEEEremainder(ds.data[0], a);
        ds.data[0] = ds.data[0] - (ds.data[0] / a).round() * a;
        // Java: return ds;
        ds
    }

    // Java: DerivativeStructure remainder(final DerivativeStructure a)
    pub fn remainder(&self, a: &DerivativeStructure) -> DerivativeStructure {
        let mut result = Self::new(self.free_parameters, self.order);
        todo!("requires DSCompiler");
    }

    // Java: DerivativeStructure negate()
    pub fn negate(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure ds = new DerivativeStructure(compiler);
        let mut ds = Self::new(self.free_parameters, self.order);
        // Java: for (int i = 0; i < ds.data.length; ++i)
        for i in 0..ds.data.len() {
            // Java: ds.data[i] = -data[i];
            ds.data[i] = -self.data[i];
        }
        // Java: return ds;
        ds
    }

    // Java: DerivativeStructure abs()
    pub fn abs(&self) -> DerivativeStructure {
        // Java: if (Double.doubleToLongBits(data[0]) < 0)
        if (f64::to_bits(self.data[0]) as i64) < 0
        {
            // Java: // we use the bits representation to also handle -0.0
            // Java: return negate();
            self.negate()
        }
        // Java: else
        else
        {
            // Java: return this;
            self.clone()
        }
    }

    // Java: DerivativeStructure ceil()
    pub fn ceil(&self) -> DerivativeStructure {
        // Java: return new DerivativeStructure(compiler.getFreeParameters(),
                                       // Java: compiler.getOrder(),
                                       // Java: FastMath.ceil(data[0]));
        Self::new_with_value(self.free_parameters, self.order, self.data[0].ceil())
    }

    // Java: DerivativeStructure floor()
    pub fn floor(&self) -> DerivativeStructure {
        // Java: return new DerivativeStructure(compiler.getFreeParameters(),
                                       // Java: compiler.getOrder(),
                                       // Java: FastMath.floor(data[0]));
        Self::new_with_value(self.free_parameters, self.order, self.data[0].floor())
    }

    // Java: DerivativeStructure rint()
    pub fn rint(&self) -> DerivativeStructure {
        // Java: return new DerivativeStructure(compiler.getFreeParameters(),
                                       // Java: compiler.getOrder(),
                                       // Java: FastMath.rint(data[0]));
        Self::new_with_value(self.free_parameters, self.order, self.data[0].round())
    }

    // Java: long round()
    pub fn round(&self) -> i64 {
        // Java: return FastMath.round(data[0]);
        self.data[0].round() as i64
    }

    // Java: DerivativeStructure signum()
    pub fn signum(&self) -> DerivativeStructure {
        // Java: return new DerivativeStructure(compiler.getFreeParameters(),
                                       // Java: compiler.getOrder(),
                                       // Java: FastMath.signum(data[0]));
        Self::new_with_value(self.free_parameters, self.order,
            if self.data[0] > 0.0 { 1.0 } else if self.data[0] < 0.0 { -1.0 } else { 0.0 })
    }

    // Java: DerivativeStructure copySign(final DerivativeStructure sign)
    pub fn copySign_ds(&self, sign: &DerivativeStructure) -> DerivativeStructure {
        // Java: long m = Double.doubleToLongBits(data[0]);
        let m = f64::to_bits(self.data[0]) as i64;
        // Java: long s = Double.doubleToLongBits(sign.data[0]);
        let s = f64::to_bits(sign.data[0]) as i64;
        // Java: if ((m >= 0 && s >= 0) || (m < 0 && s < 0)) { // Sign is currently OK
        if (m >= 0 && s >= 0) || (m < 0 && s < 0) {
            // Java: return this;
            self.clone()
        } else {
        // Java: return negate(); // flip sign
            self.negate()
        }
    }

    // Java: DerivativeStructure copySign(final double sign)
    pub fn copySign_f64(&self, sign: f64) -> DerivativeStructure {
        // Java: long m = Double.doubleToLongBits(data[0]);
        let m = f64::to_bits(self.data[0]) as i64;
        // Java: long s = Double.doubleToLongBits(sign);
        let s = f64::to_bits(sign) as i64;
        // Java: if ((m >= 0 && s >= 0) || (m < 0 && s < 0)) { // Sign is currently OK
        if (m >= 0 && s >= 0) || (m < 0 && s < 0) {
            // Java: return this;
            self.clone()
        } else {
        // Java: return negate(); // flip sign
            self.negate()
        }
    }

    // Java: int getExponent()
    pub fn getExponent(&self) -> i32 {
        // Java: return FastMath.getExponent(data[0]);
        let bits = f64::to_bits(self.data[0]);
        let exponent = ((bits >> 52) & 0x7FF) as i32;
        exponent - 1023
    }

    // Java: DerivativeStructure scalb(final int n)
    pub fn scalb(&self, n: i32) -> DerivativeStructure {
        // Java: final DerivativeStructure ds = new DerivativeStructure(compiler);
        let mut ds = Self::new(self.free_parameters, self.order);
        // Java: for (int i = 0; i < ds.data.length; ++i)
        for i in 0..ds.data.len()
        {
            // Java: ds.data[i] = FastMath.scalb(data[i], n);
            ds.data[i] = self.data[i] * (2.0_f64).powi(n);
        }
        // Java: return ds;
        ds
    }

    // Java: DerivativeStructure hypot(final DerivativeStructure y)
    pub fn hypot(&self, y: &DerivativeStructure) -> DerivativeStructure {
        if self.data[0].is_infinite() || y.data[0].is_infinite() {
            return Self::new_with_value(self.free_parameters, self.order, f64::INFINITY);
        } else if self.data[0].is_nan() || y.data[0].is_nan() {
            return Self::new_with_value(self.free_parameters, self.order, f64::NAN);
        } else {
            let exp_x = self.getExponent();
            let exp_y = y.getExponent();
            if exp_x > exp_y + 27 {
                return self.abs();
            } else if exp_y > exp_x + 27 {
                return y.abs();
            } else {
                let middle_exp = (exp_x + exp_y) / 2;
                let scaled_x = self.scalb(-middle_exp);
                let scaled_y = y.scalb(-middle_exp);
                let val = (scaled_x.data[0] * scaled_x.data[0] + scaled_y.data[0] * scaled_y.data[0]).sqrt();
                let scaled_h = Self::new_with_value(self.free_parameters, self.order, val);
                return scaled_h.scalb(middle_exp);
            }
        }
    }

    // Java: static DerivativeStructure hypot(final DerivativeStructure x, final DerivativeStructure y)
    pub fn hypot_static(x: &DerivativeStructure, y: &DerivativeStructure) -> DerivativeStructure {
        x.hypot(y)
    }

    // Java: DerivativeStructure compose(final double ... f)
    pub fn compose(&self, f: &[f64]) -> DerivativeStructure {
        todo!("requires DSCompiler")
    }

    // Java: DerivativeStructure reciprocal()
    pub fn reciprocal(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.pow(data, 0, -1, result.data, 0);
        todo!("requires DSCompiler");
        // Java: return result;
    }

    // Java: DerivativeStructure sqrt()
    pub fn sqrt(&self) -> DerivativeStructure {
        // Java: return rootN(2);
        self.rootN(2)
    }

    // Java: DerivativeStructure cbrt()
    pub fn cbrt(&self) -> DerivativeStructure {
        // Java: return rootN(3);
        self.rootN(3)
    }

    // Java: DerivativeStructure rootN(final int n)
    pub fn rootN(&self, n: i32) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.rootN(data, 0, n, result.data, 0);
        result.data[0] = self.data[0].powf(1.0 / n as f64);
        // Java: return result;
        result
    }

    // Java: Field<DerivativeStructure> getField()
    pub fn getField(&self) -> DerivativeStructureField {
        // Java: return new Field<DerivativeStructure>()
        {
            // Java: /** {@inheritDoc} */
            // Java: public DerivativeStructure getZero()
            {
                // Java: return new DerivativeStructure(compiler.getFreeParameters(), compiler.getOrder(), 0.0);
            }
            // Java: /** {@inheritDoc} */
            // Java: public DerivativeStructure getOne()
            {
                // Java: return new DerivativeStructure(compiler.getFreeParameters(), compiler.getOrder(), 1.0);
            }
            // Java: /** {@inheritDoc} */
            // Java: public Class<? extends FieldElement<DerivativeStructure>> getRuntimeClass()
            {
                // Java: return DerivativeStructure.class;
            }
        }
        DerivativeStructureField { free_parameters: self.free_parameters, order: self.order }
    }

    // Java: static DerivativeStructure pow(final double a, final DerivativeStructure x)
    pub fn pow_double_ds(a: f64, x: &DerivativeStructure) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(x.compiler);
        let mut result = Self::new(x.free_parameters, x.order);
        // Java: x.compiler.pow(a, x.data, 0, result.data, 0);
        result.data[0] = a.powf(x.data[0]);
        // Java: return result;
        result
    }

    // Java: DerivativeStructure pow(final double p)
    pub fn pow_f64(&self, p: f64) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.pow(data, 0, p, result.data, 0);
        result.data[0] = self.data[0].powf(p);
        // Java: return result;
        result
    }

    // Java: DerivativeStructure pow(final int n)
    pub fn pow_int(&self, n: i32) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.pow(data, 0, n, result.data, 0);
        result.data[0] = self.data[0].powi(n);
        // Java: return result;
        result
    }

    // Java: DerivativeStructure pow(final DerivativeStructure e)
    pub fn pow_ds(&self, e: &DerivativeStructure) -> DerivativeStructure {
        let mut result = Self::new(self.free_parameters, self.order);
        result.data[0] = self.data[0].powf(e.data[0]);
        result
    }

    // Java: DerivativeStructure exp()
    pub fn exp(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.exp(data, 0, result.data, 0);
        result.data[0] = self.data[0].exp();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure expm1()
    pub fn expm1(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.expm1(data, 0, result.data, 0);
        result.data[0] = self.data[0].exp_m1();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure log()
    pub fn log(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.log(data, 0, result.data, 0);
        result.data[0] = self.data[0].ln();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure log1p()
    pub fn log1p(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.log1p(data, 0, result.data, 0);
        result.data[0] = self.data[0].ln_1p();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure log10()
    pub fn log10(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.log10(data, 0, result.data, 0);
        result.data[0] = self.data[0].log10();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure cos()
    pub fn cos(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.cos(data, 0, result.data, 0);
        result.data[0] = self.data[0].cos();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure sin()
    pub fn sin(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.sin(data, 0, result.data, 0);
        result.data[0] = self.data[0].sin();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure tan()
    pub fn tan(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.tan(data, 0, result.data, 0);
        result.data[0] = self.data[0].tan();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure acos()
    pub fn acos(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.acos(data, 0, result.data, 0);
        result.data[0] = self.data[0].acos();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure asin()
    pub fn asin(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.asin(data, 0, result.data, 0);
        result.data[0] = self.data[0].asin();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure atan()
    pub fn atan(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.atan(data, 0, result.data, 0);
        result.data[0] = self.data[0].atan();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure atan2(final DerivativeStructure x)
    pub fn atan2(&self, x: &DerivativeStructure) -> DerivativeStructure {
        let mut result = Self::new(self.free_parameters, self.order);
        result.data[0] = self.data[0].atan2(x.data[0]);
        result
    }

    // Java: static DerivativeStructure atan2(final DerivativeStructure y, final DerivativeStructure x)
    pub fn atan2_static(y: &DerivativeStructure, x: &DerivativeStructure) -> DerivativeStructure {
        y.atan2(x)
    }

    // Java: DerivativeStructure cosh()
    pub fn cosh(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.cosh(data, 0, result.data, 0);
        result.data[0] = self.data[0].cosh();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure sinh()
    pub fn sinh(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.sinh(data, 0, result.data, 0);
        result.data[0] = self.data[0].sinh();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure tanh()
    pub fn tanh(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.tanh(data, 0, result.data, 0);
        result.data[0] = self.data[0].tanh();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure acosh()
    pub fn acosh(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.acosh(data, 0, result.data, 0);
        result.data[0] = self.data[0].acosh();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure asinh()
    pub fn asinh(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.asinh(data, 0, result.data, 0);
        result.data[0] = self.data[0].asinh();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure atanh()
    pub fn atanh(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure result = new DerivativeStructure(compiler);
        let mut result = Self::new(self.free_parameters, self.order);
        // Java: compiler.atanh(data, 0, result.data, 0);
        result.data[0] = self.data[0].atanh();
        // Java: return result;
        result
    }

    // Java: DerivativeStructure toDegrees()
    pub fn toDegrees(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure ds = new DerivativeStructure(compiler);
        let mut ds = Self::new(self.free_parameters, self.order);
        // Java: for (int i = 0; i < ds.data.length; ++i)
        for i in 0..ds.data.len()
        {
            // Java: ds.data[i] = FastMath.toDegrees(data[i]);
            ds.data[i] = self.data[i].to_degrees();
        }
        // Java: return ds;
        ds
    }

    // Java: DerivativeStructure toRadians()
    pub fn toRadians(&self) -> DerivativeStructure {
        // Java: final DerivativeStructure ds = new DerivativeStructure(compiler);
        let mut ds = Self::new(self.free_parameters, self.order);
        // Java: for (int i = 0; i < ds.data.length; ++i)
        for i in 0..ds.data.len()
        {
            // Java: ds.data[i] = FastMath.toRadians(data[i]);
            ds.data[i] = self.data[i].to_radians();
        }
        // Java: return ds;
        ds
    }

    // Java: double taylor(final double ... delta)
    pub fn taylor(&self, delta: &[f64]) -> f64 {
        // Java: return compiler.taylor(data, 0, delta);
        todo!("requires DSCompiler")
    }

    // Java: DerivativeStructure linearCombination(final DerivativeStructure[] a, final DerivativeStructure[] b)
    pub fn linearCombination_ds_ds(&self, a: &[DerivativeStructure], b: &[DerivativeStructure]) -> DerivativeStructure {
        let v: f64 = a.iter().zip(b.iter()).map(|(ai, bi)| ai.getValue() * bi.getValue()).sum();
        let mut r = Self::new(self.free_parameters, self.order);
        r.data[0] = v;
        r
    }

    // Java: DerivativeStructure linearCombination(final double[] a, final DerivativeStructure[] b)
    pub fn linearCombination_f64_ds(&self, a: &[f64], b: &[DerivativeStructure]) -> DerivativeStructure {
        let v: f64 = a.iter().zip(b.iter()).map(|(ai, bi)| ai * bi.getValue()).sum();
        let mut r = Self::new(self.free_parameters, self.order);
        r.data[0] = v;
        r
    }

    // Java: DerivativeStructure linearCombination()
    pub fn linearCombination_2ds(&self, a1: &DerivativeStructure, b1: &DerivativeStructure, a2: &DerivativeStructure, b2: &DerivativeStructure) -> DerivativeStructure {
        let mut r = Self::new(self.free_parameters, self.order);
        r.data[0] = a1.getValue() * b1.getValue() + a2.getValue() * b2.getValue();
        r
    }

    // Java: DerivativeStructure linearCombination()
    pub fn linearCombination_2f64(&self, a1: f64, b1: &DerivativeStructure, a2: f64, b2: &DerivativeStructure) -> DerivativeStructure {
        let mut r = Self::new(self.free_parameters, self.order);
        r.data[0] = a1 * b1.getValue() + a2 * b2.getValue();
        r
    }

    // Java: DerivativeStructure linearCombination()
    pub fn linearCombination_3ds(&self, a1: &DerivativeStructure, b1: &DerivativeStructure, a2: &DerivativeStructure, b2: &DerivativeStructure, a3: &DerivativeStructure, b3: &DerivativeStructure) -> DerivativeStructure {
        let mut r = Self::new(self.free_parameters, self.order);
        r.data[0] = a1.getValue() * b1.getValue() + a2.getValue() * b2.getValue() + a3.getValue() * b3.getValue();
        r
    }

    // Java: DerivativeStructure linearCombination()
    pub fn linearCombination_3f64(&self, a1: f64, b1: &DerivativeStructure, a2: f64, b2: &DerivativeStructure, a3: f64, b3: &DerivativeStructure) -> DerivativeStructure {
        let mut r = Self::new(self.free_parameters, self.order);
        r.data[0] = a1 * b1.getValue() + a2 * b2.getValue() + a3 * b3.getValue();
        r
    }

    // Java: DerivativeStructure linearCombination()
    pub fn linearCombination_4ds(&self, a1: &DerivativeStructure, b1: &DerivativeStructure, a2: &DerivativeStructure, b2: &DerivativeStructure, a3: &DerivativeStructure, b3: &DerivativeStructure, a4: &DerivativeStructure, b4: &DerivativeStructure) -> DerivativeStructure {
        let mut r = Self::new(self.free_parameters, self.order);
        r.data[0] = a1.getValue() * b1.getValue() + a2.getValue() * b2.getValue() + a3.getValue() * b3.getValue() + a4.getValue() * b4.getValue();
        r
    }

    // Java: DerivativeStructure linearCombination()
    pub fn linearCombination_4f64(&self, a1: f64, b1: &DerivativeStructure, a2: f64, b2: &DerivativeStructure, a3: f64, b3: &DerivativeStructure, a4: f64, b4: &DerivativeStructure) -> DerivativeStructure {
        let mut r = Self::new(self.free_parameters, self.order);
        r.data[0] = a1 * b1.getValue() + a2 * b2.getValue() + a3 * b3.getValue() + a4 * b4.getValue();
        r
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &DerivativeStructure) -> bool {
        // Java: if (this == other)
        {
            // Java: return true;
        }
        // Java: if (other instanceof DerivativeStructure)
        {
            // Java: final DerivativeStructure rhs = (DerivativeStructure)other;
            // Java: return (getFreeParameters() == rhs.getFreeParameters()) &&
                   // Java: (getOrder() == rhs.getOrder()) &&
                   // Java: MathArrays.equals(data, rhs.data);
        }
        // Java: return false;
        self.free_parameters == other.free_parameters
            && self.order == other.order
            && self.data == other.data
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return 227 + 229 * getFreeParameters() + 233 * getOrder() + 239 * MathUtils.hash(data);
        let mut h: i64 = 227 + 229 * self.free_parameters as i64 + 233 * self.order as i64;
        for d in &self.data {
            h = h.wrapping_add(239_i64.wrapping_mul(d.to_bits() as i64));
        }
        h as i32
    }

    // Java: Object writeReplace()
    pub fn writeReplace(&self) -> DataTransferObject {
        // Java: return new DataTransferObject(compiler.getFreeParameters(), compiler.getOrder(), data);
        DataTransferObject { variables: self.free_parameters, order: self.order, data: self.data.clone() }
    }

}

pub struct DerivativeStructureField {
    free_parameters: i32,
    order: i32,
}

impl DerivativeStructureField {
    // Java: /** {@inheritDoc} */
    // Java: public DerivativeStructure getZero()
    pub fn getZero(&self) -> DerivativeStructure {
        // Java: return new DerivativeStructure(compiler.getFreeParameters(), compiler.getOrder(), 0.0);
        DerivativeStructure::new_with_value(self.free_parameters, self.order, 0.0)
    }
    // Java: /** {@inheritDoc} */
    // Java: public DerivativeStructure getOne()
    pub fn getOne(&self) -> DerivativeStructure {
        // Java: return new DerivativeStructure(compiler.getFreeParameters(), compiler.getOrder(), 1.0);
        DerivativeStructure::new_with_value(self.free_parameters, self.order, 1.0)
    }
    // Java: /** {@inheritDoc} */
    // Java: public Class<? extends FieldElement<DerivativeStructure>> getRuntimeClass()
    pub fn getRuntimeClass(&self) -> &'static str {
        // Java: return DerivativeStructure.class;
        "DerivativeStructure"
    }
}

// Java: public class DataTransferObject
impl DataTransferObject {
    // Java: Object readResolve()
    pub fn readResolve(&self) -> DerivativeStructure {
            // Java: return new DerivativeStructure(variables, order, data);
            DerivativeStructure::new_with_derivatives(self.variables, self.order, &self.data)
    }

}
