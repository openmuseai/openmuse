use super::rule::KmoRuleTimePeriod::TimePeriodType;
pub struct FormulaBuilder;
impl FormulaBuilder{
// java public static String containsBlanks(String ref) {
// java return "LEN(TRIM(" + ref + "))=0";
// java }
pub fn containsBlanks(r:&str)->String {
let mut s=String::new();s.push_str("LEN(TRIM(");s.push_str(r);s.push_str("))=0");s
}
// java public static String notContainsBlanks(String ref) {
// java return "LEN(TRIM(" + ref + "))>0";
// java }
pub fn notContainsBlanks(r:&str)->String {
let mut s=String::new();s.push_str("LEN(TRIM(");s.push_str(r);s.push_str("))>0");s
}
// java public static String containsErrors(String ref) {
// java return "ISERROR(" + ref + ")";
// java }
pub fn containsErrors(r:&str)->String {
let mut s=String::new();s.push_str("ISERROR(");s.push_str(r);s.push_str(")");s
}
// java public static String notContainsErrors(String ref) {
// java return "NOT(ISERROR(" + ref + "))";
// java }
pub fn notContainsErrors(r:&str)->String {
let mut s=String::new();s.push_str("NOT(ISERROR(");s.push_str(r);s.push_str("))");s
}
// java public static String containText(String content) {
// java return "NOT(ISERROR(SEARCH(" + content + ",A1)))";
// java }
pub fn containText(content:&str)->String {
let mut s=String::new();s.push_str("NOT(ISERROR(SEARCH(");s.push_str(content);s.push_str(",A1)))");s
}
// java public static String notContainText(String content) {
// java return "ISERROR(SEARCH(" + content + ",A1))";
// java }
pub fn notContainText(content:&str)->String {
let mut s=String::new();s.push_str("ISERROR(SEARCH(");s.push_str(content);s.push_str(",A1))");s
}
// java public static String beginWith(String content) {
// java return "LEFT(A1,LEN(" + content + "))=" + content + "";
// java }
pub fn beginWith(content:&str)->String {
let mut s=String::new();s.push_str("LEFT(A1,LEN(");s.push_str(content);s.push_str("))=");s.push_str(content);s.push_str("");s
}
// java public static String endWith(String content) {
// java return "RIGHT(A1,LEN(" + content + "))=" + content + "";
// java }
pub fn endWith(content:&str)->String {
let mut s=String::new();s.push_str("RIGHT(A1,LEN(");s.push_str(content);s.push_str("))=");s.push_str(content);s.push_str("");s
}
// java public static String timePeriod(String ref, KmoRuleTimePeriod.TimePeriodType type) {
// java switch (type) {
// java case last7Days:
// java return "AND(TODAY()-FLOOR(" + ref + ",1)<=6,FLOOR(" + ref + ",1)<=TODAY())";
// java case lastMonth:
// java return "AND(MONTH(" + ref + ")=MONTH(EDATE(TODAY(),0-1)),YEAR(" + ref + ")=YEAR(EDATE(TODAY(),0-1)))";
// java case thisMonth:
// java return "AND(MONTH(" + ref + ")=MONTH(TODAY()),YEAR(" + ref + ")=YEAR(TODAY()))";
// java case nextMonth:
// java return "AND(MONTH(" + ref + ")=MONTH(EDATE(TODAY(),0+1)),YEAR(" + ref + ")=YEAR(EDATE(TODAY(),0+1)))";
// java case lastWeek:
// java return "AND(TODAY()-ROUNDDOWN(" + ref + ",0)>=(WEEKDAY(TODAY())),TODAY()-ROUNDDOWN(" + ref + ",0)<(WEEKDAY(TODAY())+7))";
// java case thisWeek:
// java return "AND(TODAY()-ROUNDDOWN(" + ref + ",0)<=WEEKDAY(TODAY())-1,ROUNDDOWN(" + ref + ",0)-TODAY()<=7-WEEKDAY(TODAY()))";
// java case nextWeek:
// java return "AND(ROUNDDOWN(" + ref + ",0)-TODAY()>(7-WEEKDAY(TODAY())),ROUNDDOWN(" + ref + ",0)-TODAY()<(15-WEEKDAY(TODAY())))";
// java case today:
// java return "FLOOR(" + ref + ",1)=TODAY()";
// java case tomorrow:
// java return "FLOOR(" + ref + ",1)=TODAY()+1";
// java case yesterday:
// java return "FLOOR(" + ref + ",1)=TODAY()-1";
// java default:
// java return "";
// java }
// java }
pub fn timePeriod(r:&str,kind:TimePeriodType)->String {
use TimePeriodType::*;match kind{last7Days=>{let mut s=String::new();s.push_str("AND(TODAY()-FLOOR(");s.push_str(r);s.push_str(",1)<=6,FLOOR(");s.push_str(r);s.push_str(",1)<=TODAY())");s},lastMonth=>{let mut s=String::new();s.push_str("AND(MONTH(");s.push_str(r);s.push_str(")=MONTH(EDATE(TODAY(),0-1)),YEAR(");s.push_str(r);s.push_str(")=YEAR(EDATE(TODAY(),0-1)))");s},thisMonth=>{let mut s=String::new();s.push_str("AND(MONTH(");s.push_str(r);s.push_str(")=MONTH(TODAY()),YEAR(");s.push_str(r);s.push_str(")=YEAR(TODAY()))");s},nextMonth=>{let mut s=String::new();s.push_str("AND(MONTH(");s.push_str(r);s.push_str(")=MONTH(EDATE(TODAY(),0+1)),YEAR(");s.push_str(r);s.push_str(")=YEAR(EDATE(TODAY(),0+1)))");s},lastWeek=>{let mut s=String::new();s.push_str("AND(TODAY()-ROUNDDOWN(");s.push_str(r);s.push_str(",0)>=(WEEKDAY(TODAY())),TODAY()-ROUNDDOWN(");s.push_str(r);s.push_str(",0)<(WEEKDAY(TODAY())+7))");s},thisWeek=>{let mut s=String::new();s.push_str("AND(TODAY()-ROUNDDOWN(");s.push_str(r);s.push_str(",0)<=WEEKDAY(TODAY())-1,ROUNDDOWN(");s.push_str(r);s.push_str(",0)-TODAY()<=7-WEEKDAY(TODAY()))");s},nextWeek=>{let mut s=String::new();s.push_str("AND(ROUNDDOWN(");s.push_str(r);s.push_str(",0)-TODAY()>(7-WEEKDAY(TODAY())),ROUNDDOWN(");s.push_str(r);s.push_str(",0)-TODAY()<(15-WEEKDAY(TODAY())))");s},today=>{let mut s=String::new();s.push_str("FLOOR(");s.push_str(r);s.push_str(",1)=TODAY()");s},tomorrow=>{let mut s=String::new();s.push_str("FLOOR(");s.push_str(r);s.push_str(",1)=TODAY()+1");s},yesterday=>{let mut s=String::new();s.push_str("FLOOR(");s.push_str(r);s.push_str(",1)=TODAY()-1");s},}
}
// java public static String top10Formula(int rank, boolean isPercent, boolean bottom, List<String> addrs) {
// java String formula = "";
// java if (isPercent) {
// java formula = topOrBottomNPercent(bottom, rank, addrs);
// java } else {
// java formula = topOrBottomN(bottom, rank, addrs);
// java }
// java
// java return formula;
// java }
pub fn top10Formula(rank:i32,isPercent:bool,bottom:bool,addrs:&[String])->String {
if isPercent{Self::topOrBottomNPercent(bottom,rank,addrs)}else{Self::topOrBottomN(bottom,rank,addrs)}
}
// java private static String topOrBottomN(boolean bottom, int rank, List<String> addrs) {
// java StringBuilder formula = new StringBuilder();
// java if (bottom) {
// java formula.append("SMALL((");
// java } else {
// java formula.append("LARGE((");
// java }
// java
// java StringBuilder addrStr = new StringBuilder();
// java int i = 0;
// java
// java for(int size = addrs.size(); i < size; ++i) {
// java addrStr.append((String)addrs.get(i)).append(",");
// java }
// java
// java addrStr = addrStr.deleteCharAt(addrStr.length() - 1);
// java formula.append(addrStr);
// java formula.append("),MIN(");
// java formula.append(rank);
// java formula.append(",COUNT(");
// java formula.append(addrStr);
// java formula.append(")))");
// java if (bottom) {
// java formula.append(">=A1");
// java } else {
// java formula.append("<=A1");
// java }
// java
// java return formula.toString();
// java }
pub fn topOrBottomN(bottom:bool,rank:i32,addrs:&[String])->String {
assert!(!addrs.is_empty(),"StringIndexOutOfBoundsException");let a=addrs.join(",");format!("{}(({a}),MIN({rank},COUNT({a}))){}A1",if bottom{"SMALL"}else{"LARGE"},if bottom{">="}else{"<="})
}
// java private static String topOrBottomNPercent(boolean bottom, int rank, List<String> addrs) {
// java StringBuilder formula = new StringBuilder();
// java StringBuilder addrStr = new StringBuilder();
// java int i = 0;
// java
// java for(int size = addrs.size(); i < size; ++i) {
// java addrStr.append((String)addrs.get(i)).append(",");
// java }
// java
// java addrStr = addrStr.deleteCharAt(addrStr.length() - 1);
// java formula.append("IF(INT(COUNT(");
// java formula.append(addrStr);
// java formula.append(")*");
// java formula.append(rank);
// java formula.append("%)>0,");
// java if (bottom) {
// java formula.append("SMALL(");
// java } else {
// java formula.append("LARGE(");
// java }
// java
// java formula.append(addrStr);
// java formula.append(",INT(COUNT(");
// java formula.append(addrStr);
// java formula.append(")*");
// java formula.append(rank);
// java formula.append("%)),");
// java if (bottom) {
// java formula.append("MIN(");
// java formula.append(addrStr);
// java formula.append("))>=A1");
// java } else {
// java formula.append("MAX(");
// java formula.append(addrStr);
// java formula.append("))<=A1");
// java }
// java
// java return formula.toString();
// java }
pub fn topOrBottomNPercent(bottom:bool,rank:i32,addrs:&[String])->String {
assert!(!addrs.is_empty(),"StringIndexOutOfBoundsException");let a=addrs.join(",");format!("IF(INT(COUNT({a})*{rank}%)>0,{}({a},INT(COUNT({a})*{rank}%)),{}({a})){}A1",if bottom{"SMALL"}else{"LARGE"},if bottom{"MIN"}else{"MAX"},if bottom{">="}else{"<="})
}
// java public static String uniqueFormula(boolean unique, List<String> addrs) {
// java StringBuilder formula = new StringBuilder();
// java formula.append("AND(");
// java int i = 0;
// java
// java for(int size = addrs.size(); i < size; ++i) {
// java formula.append("COUNTIF(").append((String)addrs.get(i)).append(", A1)+");
// java }
// java
// java formula = formula.deleteCharAt(formula.length() - 1);
// java if (unique) {
// java formula.append("=");
// java } else {
// java formula.append(">");
// java }
// java
// java formula.append("1,NOT(ISBLANK(A1)))");
// java return formula.toString();
// java }
pub fn uniqueFormula(unique:bool,addrs:&[String])->String {
let mut s=String::from("AND(");for a in addrs{s.push_str(&format!("COUNTIF({a}, A1)+"));}s.pop();s.push_str(if unique{"="}else{">"});s.push_str("1,NOT(ISBLANK(A1)))");s
}
// java public static String averageFormula(boolean isAbove, boolean isEqual, int stdv, List<String> addrs) {
// java String formula = "";
// java if (stdv > 0 && stdv != Integer.MAX_VALUE) {
// java formula = stdvFormula(isAbove, stdv, addrs);
// java } else {
// java formula = averageFormula(isAbove, isEqual, addrs);
// java }
// java
// java return formula;
// java }
pub fn averageFormula(isAbove:bool,isEqual:bool,stdv:i32,addrs:&[String])->String {
if stdv>0&&stdv!=i32::MAX{Self::stdvFormula(isAbove,stdv,addrs)}else{Self::averageFormula__base(isAbove,isEqual,addrs)}
}
// java private static String averageFormula(boolean isAbove, boolean isEqual, List<String> addrs) {
// java StringBuilder formula = new StringBuilder();
// java formula.append("AVERAGE(");
// java
// java for(int i = 0; i < addrs.size(); ++i) {
// java String addr = (String)addrs.get(i);
// java formula.append("IF(ISERROR(").append(addr).append("),\"\",IF(ISBLANK(").append(addr);
// java formula.append("),\"\",").append(addr).append(")),");
// java }
// java
// java formula = formula.deleteCharAt(formula.length() - 1);
// java formula.append(")");
// java if (!isAbove) {
// java if (isEqual) {
// java formula = formula.insert(0, "A1<=");
// java } else {
// java formula = formula.insert(0, "A1<");
// java }
// java } else if (isEqual) {
// java formula = formula.insert(0, "A1 >=");
// java } else {
// java formula = formula.insert(0, "A1 >");
// java }
// java
// java return formula.toString();
// java }
pub fn averageFormula__base(isAbove:bool,isEqual:bool,addrs:&[String])->String {
let mut s=String::from("AVERAGE(");for a in addrs{s.push_str(&format!("IF(ISERROR({a}),\"\",IF(ISBLANK({a}),\"\",{a})),"));}s.pop();s.push(')');format!("{}{s}",if !isAbove{if isEqual{"A1<="}else{"A1<"}}else if isEqual{"A1 >="}else{"A1 >"})
}
// java private static String stdvFormula(boolean isAbove, int stdv, List<String> addrs) {
// java StringBuilder formula = new StringBuilder();
// java formula.append("(A1-AVERAGE(");
// java StringBuilder addrStr = new StringBuilder();
// java int i = 0;
// java
// java for(int size = addrs.size(); i < size; ++i) {
// java addrStr.append((String)addrs.get(i)).append(",");
// java }
// java
// java addrStr = addrStr.deleteCharAt(addrStr.length() - 1);
// java formula.append(addrStr);
// java formula.append("))");
// java if (!isAbove) {
// java formula.append("<=");
// java } else {
// java formula.append(">=");
// java }
// java
// java formula.append("STDEVP(");
// java formula.append(addrStr);
// java formula.append(")*(");
// java if (!isAbove) {
// java formula.append(-stdv);
// java } else {
// java formula.append(stdv);
// java }
// java
// java formula.append(")");
// java return formula.toString();
// java }
pub fn stdvFormula(isAbove:bool,stdv:i32,addrs:&[String])->String {
assert!(!addrs.is_empty(),"StringIndexOutOfBoundsException");let a=addrs.join(",");format!("(A1-AVERAGE({a})){}STDEVP({a})*({})",if isAbove{">="}else{"<="},if isAbove{stdv}else{stdv.wrapping_neg()})
}
}
