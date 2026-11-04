// Java: import cn.wps.et.ss.formula.parser.rav.DesiredOperandClass
use crate::cn::wps::et::ss::formula::parser::rav::DesiredOperandClass;

// Java field: short FUNCTION_MAX_PARAMS
pub(crate) const FUNCTION_MAX_PARAMS: i16 = 30;

// Java: public class FunctionMetadata
#[derive(Clone, Debug)]
pub struct FunctionMetadata {
    // Java field: int _index
    pub(crate) _index: i32,
    // Java field: String _name
    pub(crate) _name: String,
    // Java field: int _minParams
    pub(crate) _minParams: i32,
    // Java field: int _maxParams
    pub(crate) _maxParams: i32,
    // Java field: byte _returnClassCode
    pub(crate) _returnClassCode: i8,
    // Java field: DesiredOperandClass[] _parameterClassCodes
    pub(crate) _parameterClassCodes: Vec<DesiredOperandClass::DesiredOperandClass>,
    // Java field: boolean _isVolatile
    pub(crate) _isVolatile: bool,
    // Java field: boolean _hasFootnote
    pub(crate) _hasFootnote: bool,
}

// Java: public class FunctionMetadata
impl FunctionMetadata {
    // rust adapter for Java AbstractFunctionPtg.getParameterClass(index).
    pub fn param_class(&self, index: usize) -> DesiredOperandClass::DesiredOperandClass {
        if index >= self._parameterClassCodes.len() { self._parameterClassCodes[self._parameterClassCodes.len() - 1] }
        else { self._parameterClassCodes[index] }
    }

    // rust adapter for Java hasFixedArgsLength().
    pub fn has_fixed_args(&self) -> bool { self.hasFixedArgsLength() }

    // Java: FunctionMetadata(int index, String name, int minParams, int maxParams, byte returnClassCode, DesiredOperandClass[] parameterClassCodes, boolean isVolatile, boolean hasFootnote)
    pub fn new(index: i32, name: String, minParams: i32, maxParams: i32, returnClassCode: i8, parameterClassCodes: Vec<DesiredOperandClass::DesiredOperandClass>, isVolatile: bool, hasFootnote: bool) -> Self {
        FunctionMetadata {
            _index: index,
            _name: name,
            _minParams: minParams,
            _maxParams: maxParams,
            _returnClassCode: returnClassCode,
            _parameterClassCodes: parameterClassCodes,
            _isVolatile: isVolatile,
            _hasFootnote: hasFootnote,
        }
    }

    // Java: int getIndex()
    pub fn getIndex(&self) -> i32 {
      // Java: return this._index;
      self._index
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this._name;
      &self._name
    }

    // Java: int getMinParams()
    pub fn getMinParams(&self) -> i32 {
      // Java: return this._minParams;
      self._minParams
    }

    // Java: int getMaxParams()
    pub fn getMaxParams(&self) -> i32 {
      // Java: return this._maxParams;
      self._maxParams
    }

    // Java: boolean hasFixedArgsLength()
    pub fn hasFixedArgsLength(&self) -> bool {
      // Java: return this._minParams == this._maxParams;
      self._minParams == self._maxParams
    }

    // Java: byte getReturnClassCode()
    pub fn getReturnClassCode(&self) -> i8 {
      // Java: return this._returnClassCode;
      self._returnClassCode
    }

    // Java: DesiredOperandClass[] getParameterClassCodes()
    pub fn getParameterClassCodes(&self) -> Vec<DesiredOperandClass::DesiredOperandClass> {
      // Java: return (DesiredOperandClass[])this._parameterClassCodes.clone();
      self._parameterClassCodes.clone()
    }

    // Java: boolean isVolatile()
    pub fn isVolatile(&self) -> bool {
      // Java: return this._isVolatile;
      self._isVolatile
    }

    // Java: boolean hasFootnote()
    pub fn hasFootnote(&self) -> bool {
      // Java: return this._hasFootnote;
      self._hasFootnote
    }

    // Java: boolean hasUnlimitedVarags()
    pub fn hasUnlimitedVarags(&self) -> bool {
      // Java: return 30 == this._maxParams;
      30 == self._maxParams
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuffer sb = new StringBuffer(64);
      let mut sb = String::with_capacity(64);
      // Java: sb.append(this.getClass().getSimpleName()).append(" [");
      sb.push_str("FunctionMetadata [");
      // Java: sb.append(this._index).append(" ").append(this._name);
      sb.push_str(&format!("{} {}", self._index, self._name));
      // Java: sb.append("]");
      sb.push(']');
      // Java: return sb.toString();
      sb
    }

}
