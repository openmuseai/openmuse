use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use super::Rule::{Rule,RuleObject,CfRuleTypes};
// java public static enum TimePeriodType {
// java last7Days(18),
// java lastMonth(19),
// java lastWeek(23),
// java nextMonth(20),
// java nextWeek(22),
// java thisMonth(24),
// java thisWeek(21),
// java today(15),
// java tomorrow(16),
// java yesterday(17);
// java
// java public final int value;
// java
// java private TimePeriodType(int value) {
// java this.value = value;
// java }
// java }
// java }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum TimePeriodType{today,tomorrow,yesterday,last7Days,lastMonth,nextMonth,thisWeek,nextWeek,lastWeek,thisMonth}
impl TimePeriodType{pub fn valueOf(s:&str)->Option<Self>{match s{"today"=>Some(Self::today),"tomorrow"=>Some(Self::tomorrow),"yesterday"=>Some(Self::yesterday),"last7Days"=>Some(Self::last7Days),"lastMonth"=>Some(Self::lastMonth),"nextMonth"=>Some(Self::nextMonth),"thisWeek"=>Some(Self::thisWeek),"nextWeek"=>Some(Self::nextWeek),"lastWeek"=>Some(Self::lastWeek),"thisMonth"=>Some(Self::thisMonth),_=>None}}}
// java public class KmoRuleTimePeriod extends Rule
pub struct KmoRuleTimePeriod {base:Rule,timePeriodType:Cell<Option<TimePeriodType>>}
impl std::ops::Deref for KmoRuleTimePeriod {type Target=Rule;fn deref(&self)->&Rule{&self.base}}
impl RuleObject for KmoRuleTimePeriod {fn base(&self)->&Rule{&self.base}fn as_any(&self)->&dyn std::any::Any{self}}
impl KmoRuleTimePeriod {
// java public KmoRuleTimePeriod(SpreadsheetVersion sv) {
// java super(sv);
// java this.setType(Rule.CfRuleTypes.timePeriod);
// java }
pub fn new(sv:SpreadsheetVersion)->Self {
let base=Rule::new(sv);base.setType(Some(CfRuleTypes::timePeriod));Self{base,timePeriodType:Cell::new(None)}
}
// java public TimePeriodType getTimePeriodType() {
// java return this.timePeriodType;
// java }
pub fn getTimePeriodType(&self)->Option<TimePeriodType> {
self.timePeriodType.get()
}
// java public void setTimePeriodType(TimePeriodType timePeriod) {
// java this.timePeriodType = timePeriod;
// java }
pub fn setTimePeriodType(&self,v:Option<TimePeriodType>) {
self.timePeriodType.set(v);
}
}
