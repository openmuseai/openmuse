// Java: import java.io.IOException
// Java: import java.io.ObjectInputStream
// Java: import java.io.ObjectOutputStream
// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.linear.MatrixUtils
use crate::org::apache::commons::math3::linear::MatrixUtils;
// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;
// Java: import org.apache.commons.math3.linear.ArrayRealVector
use crate::org::apache::commons::math3::linear::ArrayRealVector;

// Java field: long serialVersionUID

// Java: public class LinearObjectiveFunction implements Serializable
pub struct LinearObjectiveFunction {
    // Java field: RealVector coefficients
    pub(crate) coefficients: Box<dyn std::any::Any>,
    // Java field: double constantTerm
    pub(crate) constantTerm: f64,
}

// Java: public class LinearObjectiveFunction implements Serializable
impl LinearObjectiveFunction {
    // Java: LinearObjectiveFunction(double[] coefficients, double constantTerm)
    pub fn LinearObjectiveFunction(&self) {
        // Java: this(new ArrayRealVector(coefficients), constantTerm);
    }

    // Java: LinearObjectiveFunction(RealVector coefficients, double constantTerm)
    pub fn LinearObjectiveFunction_2(&self) {
        // Java: this.coefficients = coefficients;
        // Java: this.constantTerm = constantTerm;
    }

    // Java: RealVector getCoefficients()
    pub fn getCoefficients(&self) -> Box<dyn std::any::Any> {
        // Java: return coefficients;
        todo!()
    }

    // Java: double getConstantTerm()
    pub fn getConstantTerm(&self) -> f64 {
        // Java: return constantTerm;
        todo!()
    }

    // Java: double getValue(final double[] point)
    pub fn getValue(&self) -> f64 {
        // Java: return coefficients.dotProduct(new ArrayRealVector(point, false)) + constantTerm;
        todo!()
    }

    // Java: double getValue(final RealVector point)
    pub fn getValue_2(&self) -> f64 {
        // Java: return coefficients.dotProduct(point) + constantTerm;
        todo!()
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self) -> bool {
      // Java: if (this == other)
      {
        // Java: return true;
      }
      // Java: if (other instanceof LinearObjectiveFunction)
      {
          // Java: LinearObjectiveFunction rhs = (LinearObjectiveFunction) other;
          // Java: return (constantTerm == rhs.constantTerm) && coefficients.equals(rhs.coefficients);
      }
      // Java: return false;
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return Double.valueOf(constantTerm).hashCode() ^ coefficients.hashCode();
        todo!()
    }

    // Java: void writeObject(ObjectOutputStream oos)
    pub fn writeObject(&self) {
    }

    // Java: void readObject(ObjectInputStream ois)
    pub fn readObject(&self) {
    }

}

