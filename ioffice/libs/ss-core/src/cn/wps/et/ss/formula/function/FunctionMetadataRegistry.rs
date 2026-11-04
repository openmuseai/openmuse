// Java: import cn.wps.et.ss.formula.parser.rav.DesiredOperandClass
use crate::cn::wps::et::ss::formula::parser::rav::DesiredOperandClass;
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::Arc;
// Java: import java.util.Map

use crate::cn::wps::et::ss::formula::function::FunctionMetadata::FunctionMetadata;

// Java field: String FUNCTION_NAME_IF
pub(crate) const FUNCTION_NAME_IF: &str = "IF";
// Java field: short FUNCTION_INDEX_IF
pub(crate) const FUNCTION_INDEX_IF: i16 = 1;
// Java field: short FUNCTION_INDEX_SUM
pub(crate) const FUNCTION_INDEX_SUM: i16 = 4;
// Java field: short FUNCTION_INDEX_RAND
pub(crate) const FUNCTION_INDEX_RAND: i16 = 63;
// Java field: short FUNCTION_INDEX_NOW
pub(crate) const FUNCTION_INDEX_NOW: i16 = 74;
// Java field: short FUNCTION_INDEX_OFFSET
pub(crate) const FUNCTION_INDEX_OFFSET: i16 = 78;
// Java field: short FUNCTION_INDEX_CHOOSE
pub(crate) const FUNCTION_INDEX_CHOOSE: i16 = 100;
// Java field: short FUNCTION_INDEX_CELL
pub(crate) const FUNCTION_INDEX_CELL: i16 = 125;
// Java field: short FUNCTION_INDEX_INDIRECT
pub(crate) const FUNCTION_INDEX_INDIRECT: i16 = 148;
// Java field: short FUNCTION_INDEX_TODAY
pub(crate) const FUNCTION_INDEX_TODAY: i16 = 221;
// Java field: short FUNCTION_INDEX_INFO
pub(crate) const FUNCTION_INDEX_INFO: i16 = 244;
// Java field: short FUNCTION_INDEX_EXTERNAL
pub(crate) const FUNCTION_INDEX_EXTERNAL: i16 = 255;
// Java field: short FUNCTION_INDEX_EVALUATION
pub(crate) const FUNCTION_INDEX_EVALUATION: i16 = 257;
// Java field: short FUNCTION_INDEX_SUBTOTAL
pub(crate) const FUNCTION_INDEX_SUBTOTAL: i16 = 344;
// Java field: short MAX_FUNCTION_INDEX
pub(crate) const MAX_FUNCTION_INDEX: i16 = 367;
// Java field: FunctionMetadataRegistry _instance
static _INSTANCE: std::sync::OnceLock<FunctionMetadataRegistry> = std::sync::OnceLock::new();

// Java: public class FunctionMetadataRegistry
pub struct FunctionMetadataRegistry {
    // Java field: FunctionMetadata[] _functionDataByIndex
    pub(crate) _functionDataByIndex: Vec<Option<Arc<FunctionMetadata>>>,
    // Java field: Map<String, FunctionMetadata> _functionDataByName
    pub(crate) _functionDataByName: HashMap<String, Arc<FunctionMetadata>>,
}

// Java: public class FunctionMetadataRegistry
impl FunctionMetadataRegistry {
    pub fn new() -> Self {
        FunctionMetadataRegistry {
            _functionDataByIndex: (0..368).map(|_| None).collect(),
            _functionDataByName: HashMap::with_capacity(550),
        }
    }

    // Java: static FunctionMetadataRegistry getInstance()
    fn getInstance() -> &'static FunctionMetadataRegistry {
      // Java: if (_instance == null)
      // Java: _instance = FunctionDataHolder.createFunctionMetadataRegister();
      // Java: return _instance;
      _INSTANCE.get_or_init(|| {
          crate::cn::wps::et::ss::formula::function::FunctionDataHolder::FunctionDataHolder::createFunctionMetadataRegister()
      })
    }

    // Java: void add(int functionIndex, String functionName, int minParams, int maxParams, byte returnClassCode, DesiredOperandClass[] parameterClassCodes, boolean isVolatile, boolean hasFootnote)
    pub fn add(&mut self, functionIndex: i32, functionName: String, minParams: i32, maxParams: i32, returnClassCode: i8, parameterClassCodes: Vec<DesiredOperandClass::DesiredOperandClass>, isVolatile: bool, hasFootnote: bool) {
      // Java: FunctionMetadata fm = new FunctionMetadata(functionIndex, functionName, minParams, maxParams, returnClassCode, parameterClassCodes, isVolatile, hasFootnote);
      let fm = Arc::new(FunctionMetadata::new(functionIndex, functionName.clone(), minParams, maxParams, returnClassCode, parameterClassCodes, isVolatile, hasFootnote));
      // Java: FunctionMetadata prevFM = (FunctionMetadata)this._functionDataByName.put(functionName, fm);
      let mut prevFM = self._functionDataByName.insert(functionName.clone(), Arc::clone(&fm));
      // Java: if (prevFM == null)
      if prevFM.is_none() {
         // Java: prevFM = this._functionDataByIndex[functionIndex];
         prevFM = self._functionDataByIndex[functionIndex as usize].clone();
      }
      // Java: this._functionDataByIndex[functionIndex] = fm;
      self._functionDataByIndex[functionIndex as usize] = Some(fm);
      // Java: if (prevFM != null)
      if prevFM.is_some() {
         // Java: throw new RuntimeException("Multiple entries for function " + functionIndex + " '" + functionName + "'");
         panic!("Multiple entries for function {} '{}'", functionIndex, functionName);
      }
    }

    // Java: void addFootnote(int functionIndex, String footnote)
    pub fn addFootnote(&mut self, functionIndex: i32, footnote: String) {
      // Java: FunctionMetadata prevFM = this._functionDataByIndex[functionIndex];
      let prevFM = self._functionDataByIndex[functionIndex as usize].clone();
      // Java: if (prevFM != null && prevFM.hasFootnote())
      if let Some(prev) = prevFM {
          if prev.hasFootnote() {
             // Java: prevFM = (FunctionMetadata)this._functionDataByName.put(footnote, prevFM);
             let old = self._functionDataByName.insert(footnote.clone(), prev);
             // Java: if (prevFM != null)
             if old.is_some() {
                // Java: throw new RuntimeException("Function footnote redefined '" + footnote + "'");
                panic!("Function footnote redefined '{}'", footnote);
             }
          } else {
             // Java: else
             // Java: throw new RuntimeException("Function not found " + functionIndex);
             panic!("Function not found {}", functionIndex);
          }
      }
      // Java: else
      else {
         // Java: throw new RuntimeException("Function not found " + functionIndex);
         panic!("Function not found {}", functionIndex);
      }
    }

    // Java: static FunctionMetadata getFunctionByIndex(int index)
    pub fn getFunctionByIndex(index: i32) -> Option<&'static FunctionMetadata> {
      // Java: return getInstance().getFunctionByIndexInternal(index);
      Self::getInstance().getFunctionByIndexInternal(index)
    }

    // Java: FunctionMetadata getFunctionByIndexInternal(int index)
    fn getFunctionByIndexInternal(&self, index: i32) -> Option<&FunctionMetadata> {
      // Java: return this._functionDataByIndex[index];
      self._functionDataByIndex[index as usize].as_deref()
    }

    // Java: static short lookupIndexByName(String name)
    pub fn lookupIndexByName(name: &str) -> i16 {
      // Java: FunctionMetadata fd = getInstance().getFunctionByNameInternal(name);
      let fd = Self::getInstance().getFunctionByNameInternal(name);
      // Java: return fd == null ? -1 : (short)fd.getIndex();
      match fd {
          Some(f) => f.getIndex() as i16,
          None => -1,
      }
    }

    // Java: FunctionMetadata getFunctionByNameInternal(String name)
    fn getFunctionByNameInternal(&self, name: &str) -> Option<&FunctionMetadata> {
      // Java: return (FunctionMetadata)this._functionDataByName.get(name);
      self._functionDataByName.get(name).map(Arc::as_ref)
    }

    // Java: static FunctionMetadata getFunctionByName(String name)
    pub fn getFunctionByName(name: &str) -> Option<&'static FunctionMetadata> {
      // Java: return getInstance().getFunctionByNameInternal(name);
      Self::getInstance().getFunctionByNameInternal(name)
    }

}
