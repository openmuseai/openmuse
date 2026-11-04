// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java: public class FieldODEState extends RealFieldElement
pub struct FieldODEState {
    // Java field: T time
    pub(crate) time: Box<dyn std::any::Any>,
    // Java field: T[] state
    pub(crate) state: Box<dyn std::any::Any>,
    // Java field: T[][] secondaryState
    pub(crate) secondaryState: Box<dyn std::any::Any>,
}

// Java: public class FieldODEState extends RealFieldElement
impl FieldODEState {
    // Java: FieldODEState(T time, T[] state)
    pub fn FieldODEState(&self) {
        // Java: this(time, state, null);
    }

    // Java: FieldODEState(T time, T[] state, T[][] secondaryState)
    pub fn FieldODEState_2(&self) {
        // Java: this.time           = time;
        // Java: this.state          = state.clone();
        // Java: this.secondaryState = copy(time.getField(), secondaryState);
    }

    // Java: T[][] copy(final Field<T> field, final T[][] original)
    pub fn copy(&self) -> Box<dyn std::any::Any> {
        // Java: // special handling of null arrays
        // Java: if (original == null)
        {
            // Java: return null;
        }
        // Java: // allocate the array
        // Java: final T[][] copied = MathArrays.buildArray(field, original.length, -1);
        // Java: // copy content
        // Java: for (int i = 0; i < original.length; ++i)
        {
            // Java: copied[i] = original[i].clone();
        }
        // Java: return copied;
        todo!()
    }

    // Java: T getTime()
    pub fn getTime(&self) -> Box<dyn std::any::Any> {
        // Java: return time;
        todo!()
    }

    // Java: int getStateDimension()
    pub fn getStateDimension(&self) -> i32 {
        // Java: return state.length;
        todo!()
    }

    // Java: T[] getState()
    pub fn getState(&self) -> Box<dyn std::any::Any> {
        // Java: return state.clone();
        todo!()
    }

    // Java: int getNumberOfSecondaryStates()
    pub fn getNumberOfSecondaryStates(&self) -> i32 {
        // Java: return secondaryState == null ? 0 : secondaryState.length;
        todo!()
    }

    // Java: int getSecondaryStateDimension(final int index)
    pub fn getSecondaryStateDimension(&self) -> i32 {
        // Java: return index == 0 ? state.length : secondaryState[index - 1].length;
        todo!()
    }

    // Java: T[] getSecondaryState(final int index)
    pub fn getSecondaryState(&self) -> Box<dyn std::any::Any> {
        // Java: return index == 0 ? state.clone() : secondaryState[index - 1].clone();
        todo!()
    }

}
