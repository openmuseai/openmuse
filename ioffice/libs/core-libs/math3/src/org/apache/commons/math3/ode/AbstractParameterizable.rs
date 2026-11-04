// Java: import java.util.ArrayList
// Java: import java.util.Collection

// Java: public class provides
pub struct provides {
}

// Java: public class AbstractParameterizable
pub struct AbstractParameterizable {
    // Java field: Collection<String> parametersNames
    pub(crate) parametersNames: Box<dyn std::any::Any>,
}

// Java: public class provides
impl provides {
}

// Java: public class AbstractParameterizable
impl AbstractParameterizable {
    // Java: AbstractParameterizable(final String ... names)
    pub fn AbstractParameterizable(&self) {
        // Java: parametersNames = new ArrayList<String>();
        // Java: for (final String name : names)
        {
            // Java: parametersNames.add(name);
        }
    }

    // Java: AbstractParameterizable(final Collection<String> names)
    pub fn AbstractParameterizable_2(&self) {
        // Java: parametersNames = new ArrayList<String>();
        // Java: parametersNames.addAll(names);
    }

    // Java: Collection<String> getParametersNames()
    pub fn getParametersNames(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: return parametersNames;
        todo!()
    }

    // Java: boolean isSupported(final String name)
    pub fn isSupported(&self) -> bool {
        // Java: for (final String supportedName : parametersNames)
        {
            // Java: if (supportedName.equals(name))
            {
                // Java: return true;
            }
        }
        // Java: return false;
        todo!()
    }

    // Java: void complainIfNotSupported(final String name)
    pub fn complainIfNotSupported(&self) {
    }

}
