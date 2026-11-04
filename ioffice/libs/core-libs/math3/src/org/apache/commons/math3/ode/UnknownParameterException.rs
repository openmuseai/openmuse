// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID

// Java: public class UnknownParameterException extends MathIllegalArgumentException
pub struct UnknownParameterException {
    // Java field: String name
    pub(crate) name: Box<dyn std::any::Any>,
}

// Java: public class UnknownParameterException extends MathIllegalArgumentException
impl UnknownParameterException {
    // Java: UnknownParameterException(final String name)
    pub fn UnknownParameterException(&self) {
        // Java: super(LocalizedFormats.UNKNOWN_PARAMETER, name);
        // Java: this.name = name;
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: return name;
        todo!()
    }

}
