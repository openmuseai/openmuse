// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;

// Java: public class represents
pub struct represents {
}

// Java: public class ExpandableStatefulODE
pub struct ExpandableStatefulODE {
    // Java field: FirstOrderDifferentialEquations primary
    pub(crate) primary: Box<dyn std::any::Any>,
    // Java field: EquationsMapper primaryMapper
    pub(crate) primaryMapper: Box<dyn std::any::Any>,
    // Java field: double time
    pub(crate) time: f64,
    // Java field: double[] primaryState
    pub(crate) primaryState: Box<dyn std::any::Any>,
    // Java field: double[] primaryStateDot
    pub(crate) primaryStateDot: Box<dyn std::any::Any>,
    // Java field: List<SecondaryComponent> components
    pub(crate) components: Box<dyn std::any::Any>,
}

// Java: public class SecondaryComponent
pub struct SecondaryComponent {
    // Java field: SecondaryEquations equation
    equation: Box<dyn std::any::Any>,
    // Java field: EquationsMapper mapper
    pub(crate) mapper: Box<dyn std::any::Any>,
    // Java field: double[] state
    pub(crate) state: Box<dyn std::any::Any>,
    // Java field: double[] stateDot
    pub(crate) stateDot: Box<dyn std::any::Any>,
}

// Java: public class represents
impl represents {
}

// Java: public class ExpandableStatefulODE
impl ExpandableStatefulODE {
    // Java: ExpandableStatefulODE(final FirstOrderDifferentialEquations primary)
    pub fn ExpandableStatefulODE(&self) {
        // Java: final int n          = primary.getDimension();
        // Java: this.primary         = primary;
        // Java: this.primaryMapper   = new EquationsMapper(0, n);
        // Java: this.time            = Double.NaN;
        // Java: this.primaryState    = new double[n];
        // Java: this.primaryStateDot = new double[n];
        // Java: this.components      = new ArrayList<ExpandableStatefulODE.SecondaryComponent>();
    }

    // Java: FirstOrderDifferentialEquations getPrimary()
    pub fn getPrimary(&self) -> Box<dyn std::any::Any> {
        // Java: return primary;
        todo!()
    }

    // Java: int getTotalDimension()
    pub fn getTotalDimension(&self) -> i32 {
        // Java: if (components.isEmpty())
        {
            // Java: // there are no secondary equations, the complete set is limited to the primary set
            // Java: return primaryMapper.getDimension();
        }
        // Java: else
        {
            // Java: // there are secondary equations, the complete set ends after the last set
            // Java: final EquationsMapper lastMapper = components.get(components.size() - 1).mapper;
            // Java: return lastMapper.getFirstIndex() + lastMapper.getDimension();
        }
        todo!()
    }

    // Java: void computeDerivatives(final double t, final double[] y, final double[] yDot)
    pub fn computeDerivatives(&self) {
    }

    // Java: int addSecondaryEquations(final SecondaryEquations secondary)
    pub fn addSecondaryEquations(&self) -> i32 {
        // Java: final int firstIndex;
        // Java: if (components.isEmpty())
        {
            // Java: // lazy creation of the components list
            // Java: components = new ArrayList<ExpandableStatefulODE.SecondaryComponent>();
            // Java: firstIndex = primary.getDimension();
        }
        // Java: else
        {
            // Java: final SecondaryComponent last = components.get(components.size() - 1);
            // Java: firstIndex = last.mapper.getFirstIndex() + last.mapper.getDimension();
        }
        // Java: components.add(new SecondaryComponent(secondary, firstIndex));
        // Java: return components.size() - 1;
        todo!()
    }

    // Java: EquationsMapper getPrimaryMapper()
    pub fn getPrimaryMapper(&self) -> Box<dyn std::any::Any> {
        // Java: return primaryMapper;
        todo!()
    }

    // Java: EquationsMapper[] getSecondaryMappers()
    pub fn getSecondaryMappers(&self) -> Box<dyn std::any::Any> {
        // Java: final EquationsMapper[] mappers = new EquationsMapper[components.size()];
        // Java: for (int i = 0; i < mappers.length; ++i)
        {
            // Java: mappers[i] = components.get(i).mapper;
        }
        // Java: return mappers;
        todo!()
    }

    // Java: void setTime(final double time)
    pub fn setTime(&self) {
        // Java: this.time = time;
    }

    // Java: double getTime()
    pub fn getTime(&self) -> f64 {
        // Java: return time;
        todo!()
    }

    // Java: void setPrimaryState(final double[] primaryState)
    pub fn setPrimaryState(&self) {
        // Java: // safety checks
        // Java: if (primaryState.length != this.primaryState.length)
        {
            // Java: throw new DimensionMismatchException(primaryState.length, this.primaryState.length);
        }
        // Java: // set the data
        // Java: System.arraycopy(primaryState, 0, this.primaryState, 0, primaryState.length);
    }

    // Java: double[] getPrimaryState()
    pub fn getPrimaryState(&self) -> Vec<f64> {
        // Java: return primaryState.clone();
        todo!()
    }

    // Java: double[] getPrimaryStateDot()
    pub fn getPrimaryStateDot(&self) -> Vec<f64> {
        // Java: return primaryStateDot.clone();
        todo!()
    }

    // Java: void setSecondaryState(final int index, final double[] secondaryState)
    pub fn setSecondaryState(&self) {
    }

    // Java: double[] getSecondaryState(final int index)
    pub fn getSecondaryState(&self) -> Vec<f64> {
        // Java: return components.get(index).state.clone();
        todo!()
    }

    // Java: double[] getSecondaryStateDot(final int index)
    pub fn getSecondaryStateDot(&self) -> Vec<f64> {
        // Java: return components.get(index).stateDot.clone();
        todo!()
    }

    // Java: void setCompleteState(final double[] completeState)
    pub fn setCompleteState(&self) {
    }

    // Java: double[] getCompleteState()
    pub fn getCompleteState(&self) -> Vec<f64> {
        // Java: // allocate complete array
        // Java: double[] completeState = new double[getTotalDimension()];
        // Java: // set the data
        // Java: primaryMapper.insertEquationData(primaryState, completeState);
        // Java: for (final SecondaryComponent component : components)
        {
            // Java: component.mapper.insertEquationData(component.state, completeState);
        }
        // Java: return completeState;
        todo!()
    }

}

// Java: public class SecondaryComponent
impl SecondaryComponent {
}
