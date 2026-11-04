// Java: import java.lang.reflect.Array
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID

// Java: public class defines
pub struct defines {
}

// Java: public class JacobianMatrices
pub struct JacobianMatrices {
    // Java field: ExpandableStatefulODE efode
    efode: Option<Box<dyn std::any::Any>>,
    // Java field: int index
    index: i32,
    // Java field: MainStateJacobianProvider jode
    jode: Option<Box<dyn std::any::Any>>,
    // Java field: ParameterizedODE pode
    pode: Option<Box<dyn std::any::Any>>,
    // Java field: int stateDim
    pub(crate) stateDim: i32,
    // Java field: ParameterConfiguration[] selectedParameters
    selectedParameters: Option<Vec<Box<dyn std::any::Any>>>,
    // Java field: List<ParameterJacobianProvider> jacobianProviders
    jacobianProviders: Option<Vec<Box<dyn std::any::Any>>>,
    // Java field: int paramDim
    pub(crate) paramDim: i32,
    // Java field: boolean dirtyParameter
    dirtyParameter: bool,
    // Java field: double[] matricesData
    matricesData: Option<Vec<f64>>,
}

// Java: public class JacobiansSecondaryEquations
pub struct JacobiansSecondaryEquations {
}

// Java: public class MainStateJacobianWrapper
pub struct MainStateJacobianWrapper {
    // Java field: FirstOrderDifferentialEquations ode
    pub(crate) ode: Box<dyn std::any::Any>,
    // Java field: double[] hY
    hY: Vec<f64>,
}

// Java: public class MismatchedEquations
pub struct MismatchedEquations {
}

// Java: public class defines
impl defines {
}

// Java: public class JacobianMatrices
impl JacobianMatrices {
    // Java: JacobianMatrices()
    pub fn JacobianMatrices(&self) {
    }

    // Java: JacobianMatrices()
    pub fn JacobianMatrices_2(&self) {
    }

    // Java: void registerVariationalEquations(final ExpandableStatefulODE expandable)
    pub fn registerVariationalEquations(&self) {
    }

    // Java: void addParameterJacobianProvider(final ParameterJacobianProvider provider)
    pub fn addParameterJacobianProvider(&self) {
        // Java: jacobianProviders.add(provider);
    }

    // Java: void setParameterizedODE(final ParameterizedODE parameterizedOde)
    pub fn setParameterizedODE(&self) {
        // Java: this.pode = parameterizedOde;
        // Java: dirtyParameter = true;
    }

    // Java: void setParameterStep(final String parameter, final double hP)
    pub fn setParameterStep(&self) {
    }

    // Java: void setInitialMainStateJacobian(final double[][] dYdY0)
    pub fn setInitialMainStateJacobian(&self) {
    }

    // Java: void setInitialParameterJacobian(final String pName, final double[] dYdP)
    pub fn setInitialParameterJacobian(&self) {
    }

    // Java: void getCurrentMainSetJacobian(final double[][] dYdY0)
    pub fn getCurrentMainSetJacobian(&self) {
        // Java: // get current state for this set of equations from the expandable fode
        // Java: double[] p = efode.getSecondaryState(index);
        // Java: int j = 0;
        // Java: for (int i = 0; i < stateDim; i++)
        {
            // Java: System.arraycopy(p, j, dYdY0[i], 0, stateDim);
            // Java: j += stateDim;
        }
    }

    // Java: void getCurrentParameterJacobian(String pName, final double[] dYdP)
    pub fn getCurrentParameterJacobian(&self) {
        // Java: // get current state for this set of equations from the expandable fode
        // Java: double[] p = efode.getSecondaryState(index);
        // Java: int i = stateDim * stateDim;
        // Java: for (ParameterConfiguration param: selectedParameters)
        {
            // Java: if (param.getParameterName().equals(pName))
            {
                // Java: System.arraycopy(p, i, dYdP, 0, stateDim);
                // Java: return;
            }
            // Java: i += stateDim;
        }
    }

    // Java: void checkDimension(final int expected, final Object array)
    pub fn checkDimension(&self) {
    }

}

// Java: public class JacobiansSecondaryEquations
impl JacobiansSecondaryEquations {
    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
            // Java: return stateDim * (stateDim + paramDim);
        todo!()
    }

    // Java: void computeDerivatives()
    pub fn computeDerivatives(&self) {
    }

}

// Java: public class MainStateJacobianWrapper
impl MainStateJacobianWrapper {
    // Java: int getDimension()
    pub fn getDimension_2(&self) -> i32 {
            // Java: return ode.getDimension();
        todo!()
    }

    // Java: void computeDerivatives(double t, double[] y, double[] yDot)
    pub fn computeDerivatives_2(&self) {
    }

    // Java: void computeMainStateJacobian(double t, double[] y, double[] yDot, double[][] dFdY)
    pub fn computeMainStateJacobian(&self) {
    }

}

// Java: public class MismatchedEquations
impl MismatchedEquations {
    // Java: MismatchedEquations()
    pub fn MismatchedEquations(&self) {
            // Java: super(LocalizedFormats.UNMATCHED_ODE_IN_EXPANDED_SET);
    }

}
