// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java field: long serialVersionUID

// Java: public class are
pub struct are {
}

// Java: public class FieldEquationsMapper
pub struct FieldEquationsMapper {
    // Java field: int[] start
    pub(crate) start: Box<dyn std::any::Any>,
}

// Java: public class are
impl are {
}

// Java: public class FieldEquationsMapper
impl FieldEquationsMapper {
    // Java: int getNumberOfEquations()
    pub fn getNumberOfEquations(&self) -> i32 {
        // Java: return start.length - 1;
        todo!()
    }

    // Java: int getTotalDimension()
    pub fn getTotalDimension(&self) -> i32 {
        // Java: return start[start.length - 1];
        todo!()
    }

    // Java: T[] mapState(final FieldODEState<T> state)
    pub fn mapState(&self) -> Box<dyn std::any::Any> {
        // Java: final T[] y = MathArrays.buildArray(state.getTime().getField(), getTotalDimension());
        // Java: int index = 0;
        // Java: insertEquationData(index, state.getState(), y);
        // Java: while (++index < getNumberOfEquations())
        {
            // Java: insertEquationData(index, state.getSecondaryState(index), y);
        }
        // Java: return y;
        todo!()
    }

    // Java: T[] mapDerivative(final FieldODEStateAndDerivative<T> state)
    pub fn mapDerivative(&self) -> Box<dyn std::any::Any> {
        // Java: final T[] yDot = MathArrays.buildArray(state.getTime().getField(), getTotalDimension());
        // Java: int index = 0;
        // Java: insertEquationData(index, state.getDerivative(), yDot);
        // Java: while (++index < getNumberOfEquations())
        {
            // Java: insertEquationData(index, state.getSecondaryDerivative(index), yDot);
        }
        // Java: return yDot;
        todo!()
    }

    // Java: FieldODEStateAndDerivative<T> mapStateAndDerivative(final T t, final T[] y, final T[] yDot)
    pub fn mapStateAndDerivative(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T[] extractEquationData(final int index, final T[] complete)
    pub fn extractEquationData(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void insertEquationData(final int index, T[] equationData, T[] complete)
    pub fn insertEquationData(&self) {
    }

    // Java: void checkIndex(final int index)
    pub fn checkIndex(&self) {
        // Java: if (index < 0 || index > start.length - 2)
        {
            // Java: throw new MathIllegalArgumentException(LocalizedFormats.ARGUMENT_OUTSIDE_DOMAIN,
                                                   // Java: index, 0, start.length - 2);
        }
    }

}
