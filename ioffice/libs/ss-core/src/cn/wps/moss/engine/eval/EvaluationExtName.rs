// java package cn.wps.moss.engine.eval;

// java import cn.wps.et.ss.formula.ptg.Ptg;
use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;
// java import cn.wps.et.ss.interfaces.IEvalExtName;
use crate::cn::wps::et::ss::interfaces::IEvalExtName::IEvalExtName;
// java import cn.wps.et.ss.interfaces.IEvalNameBase;
use crate::cn::wps::et::ss::interfaces::IEvalNameBase::IEvalNameBase;
// java import cn.wps.moss.engine.linktb.ExternalNameRecord;
use crate::cn::wps::moss::engine::linktb::ExternalNameRecord::ExternalNameRecord;

// java public class EvaluationExtName implements IEvalExtName
pub struct EvaluationExtName {
    // java private final ExternalNameRecord extNameRecord;
    extNameRecord: crate::java_runtime::Rc<ExternalNameRecord>,
}

impl EvaluationExtName {
    // java public EvaluationExtName(ExternalNameRecord rec)
    pub fn EvaluationExtName(rec: crate::java_runtime::Rc<ExternalNameRecord>) -> Self {
        // java this.extNameRecord = rec;
        Self { extNameRecord: rec }
    }

    // java public Ptg[] getNameDefinition()
    pub fn getNameDefinition(&self) -> Option<Vec<Ptg>> {
        // java return this.extNameRecord.getParsedExpression();
        self.extNameRecord.getParsedExpression()
    }

    // java public String getNameText()
    pub fn getNameText(&self) -> &str {
        // java return this.extNameRecord.getText();
        self.extNameRecord.getText()
    }

    // java public int getSheetIndexOneBased()
    pub fn getSheetIndexOneBased(&self) -> i32 {
        // java return this.extNameRecord.getIx();
        self.extNameRecord.getIx() as i32
    }
}

// java EvaluationExtName implements IEvalNameBase through IEvalExtName.
impl IEvalNameBase for EvaluationExtName {
    fn getNameText(&self) -> String { EvaluationExtName::getNameText(self).into() }
    fn getSheetIndexOneBased(&self) -> i32 { EvaluationExtName::getSheetIndexOneBased(self) }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_ext_name(&self)->Option<&dyn IEvalExtName>{Some(self)}
}

// java EvaluationExtName implements IEvalExtName.
impl IEvalExtName for EvaluationExtName {
    fn getNameDefinition(&self) -> Option<Vec<Ptg>> { EvaluationExtName::getNameDefinition(self) }
}
