use crate::{org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::AreaReference::AreaReference},cn::wps::moss::util::KmoRect::KmoRect};
// java public class CellRangeAddress extends CellRangeAddressBase
#[derive(Clone)]
pub struct CellRangeAddress{pub base:crate::org::apache::poi::ss::util::CellRangeAddressBase::CellRangeAddressBase}
impl CellRangeAddress{
// java public CellRangeAddress(int firstRow, int lastRow, int firstCol, int lastCol) {
// java super(firstRow, lastRow, firstCol, lastCol);
// java }
pub fn new(firstRow:i32,lastRow:i32,firstCol:i32,lastCol:i32)->Self {
Self{base:crate::org::apache::poi::ss::util::CellRangeAddressBase::CellRangeAddressBase::CellRangeAddressBase(firstRow,lastRow,firstCol,lastCol)}
}
// java private static Range parsePart(String ref) {
// java int _row = Integer.MAX_VALUE;
// java int _col = Integer.MAX_VALUE;
// java int length = ref.length();
// java if (length < 1) {
// java return null;
// java } else {
// java int index = 0;
// java
// java while(index < length) {
// java char charAt = ref.charAt(index);
// java switch (charAt) {
// java case '$':
// java case 'R':
// java case 'r':
// java int rowIndexBegin = index;
// java if (index + 1 < length) {
// java charAt = ref.charAt(index + 1);
// java if (charAt == 'C' || charAt == 'c') {
// java ++index;
// java break;
// java }
// java }
// java
// java do {
// java ++index;
// java ++rowIndexBegin;
// java if (rowIndexBegin < length) {
// java charAt = ref.charAt(index);
// java }
// java } while(index < length && !_isdigit(charAt));
// java
// java do {
// java ++index;
// java if (index < length) {
// java charAt = ref.charAt(index);
// java }
// java } while(index < length && _isdigit(charAt));
// java
// java if (rowIndexBegin < index) {
// java _row = Integer.parseInt(ref.substring(rowIndexBegin, index));
// java }
// java break;
// java case 'C':
// java case 'c':
// java int colIndexBegin = index;
// java if (index + 1 < length) {
// java charAt = ref.charAt(index + 1);
// java if (charAt == '$' || charAt == 'R' || charAt == 'r') {
// java ++index;
// java break;
// java }
// java }
// java
// java do {
// java ++index;
// java ++colIndexBegin;
// java if (index < length) {
// java charAt = ref.charAt(index);
// java }
// java } while(index < length && !_isdigit(charAt));
// java
// java do {
// java ++index;
// java if (index < length) {
// java charAt = ref.charAt(index);
// java }
// java } while(index < length && _isdigit(charAt));
// java
// java if (colIndexBegin < index) {
// java String strCol = ref.substring(colIndexBegin, index);
// java _col = Integer.parseInt(strCol);
// java }
// java break;
// java default:
// java ++index;
// java }
// java }
// java
// java return new Range(_row, _col);
// java }
// java }
pub fn parsePart(reference:&str)->(i32,i32) {
let text=reference.encode_utf16().collect::<Vec<_>>();assert!(!text.is_empty(),"NullPointerException");let(mut row,mut col,mut index)=(i32::MAX,i32::MAX,0);while index<text.len(){let mut c=text[index];let isrow=matches!(c,36|82|114);if isrow||matches!(c,67|99){let mut begin=index;if let Some(next)=text.get(index+1){c=*next;if (isrow&&matches!(c,67|99))||(!isrow&&matches!(c,36|82|114)){index+=1;continue;}}loop{index+=1;begin+=1;if begin<text.len(){c=text[index];}if index>=text.len()||(48..=57).contains(&c){break;}}loop{index+=1;if index<text.len(){c=text[index];}if index>=text.len()||!(48..=57).contains(&c){break;}}if begin<index{let v=String::from_utf16_lossy(&text[begin..index]).parse::<i32>().expect("NumberFormatException");if isrow{row=v;}else{col=v;}}}else{index+=1;}}(row,col)
}
// java public static CellRangeAddress valueOf(String ref, SpreadsheetVersion version) {
// java AreaReference ar = null;
// java
// java try {
// java ar = new AreaReference(ref, version);
// java } catch (Throwable e) {
// java IOLog.e(TAG, "Throwable", e);
// java Pattern pattern = Pattern.compile("[$:C]+");
// java String[] strs = pattern.split(ref);
// java if (strs.length == 5) {
// java return new CellRangeAddress(Integer.parseInt(strs[1]) - 1, Integer.parseInt(strs[3]) - 1, Integer.parseInt(strs[2]) - 1, Integer.parseInt(strs[4]) - 1);
// java }
// java
// java if (_isdigit(ref.charAt(0))) {
// java ref = '$' + ref;
// java }
// java
// java int _lastRow = Integer.MAX_VALUE;
// java int _lastCol = Integer.MAX_VALUE;
// java int _firstRow;
// java int _firstCol;
// java if (ref.contains(":")) {
// java Pattern pattern1 = Pattern.compile(":");
// java String[] strs1 = pattern1.split(ref);
// java Range first = parsePart(strs1[0]);
// java Range last = parsePart(strs1[1]);
// java _firstRow = first._Row;
// java _firstCol = first._Col;
// java _lastRow = last._Row;
// java _lastCol = last._Col;
// java if (_lastCol == Integer.MAX_VALUE && _lastRow < Integer.MAX_VALUE) {
// java _lastCol = _lastRow;
// java _lastRow = _firstRow;
// java }
// java } else {
// java Range r = parsePart(ref);
// java _firstRow = r._Row;
// java _firstCol = r._Col;
// java _lastRow = r._Row;
// java _lastCol = r._Col;
// java }
// java
// java return new CellRangeAddress(_firstRow - 1, _lastRow - 1, _firstCol - 1, _lastCol - 1);
// java }
// java
// java return null == ar ? null : cutOff(ar, version);
// java }
pub fn valueOf(reference:&str,version:SpreadsheetVersion)->Option<Self> {
if let Ok(ar)=AreaReference::new(reference,version){return Some(Self::cutOff(&ar,version));}let re=regex::Regex::new("[$:C]+").unwrap();let mut parts=re.split(reference).collect::<Vec<_>>();while parts.last()==Some(&""){parts.pop();}if parts.len()==5{return Some(Self::new(parts[1].parse::<i32>().unwrap()-1,parts[3].parse::<i32>().unwrap()-1,parts[2].parse::<i32>().unwrap()-1,parts[4].parse::<i32>().unwrap()-1));}let r=if reference.as_bytes().first().is_some_and(|b|b.is_ascii_digit()){format!("${reference}")}else{reference.to_string()};let (fr,fc,mut lr,mut lc)=if let Some((a,b))=r.split_once(':'){let(fr,fc)=Self::parsePart(a);let(lr,lc)=Self::parsePart(b.split(':').next().unwrap());(fr,fc,lr,lc)}else{let(fr,fc)=Self::parsePart(&r);(fr,fc,fr,fc)};if r.contains(':')&&lc==i32::MAX&&lr<i32::MAX{lc=lr;lr=fr;}Some(Self::new(fr.wrapping_sub(1),lr.wrapping_sub(1),fc.wrapping_sub(1),lc.wrapping_sub(1)))
}
// java public static CellRangeAddress cutOff(AreaReference ar, SpreadsheetVersion version) {
// java int firstRow = ar.getFirstCell().getRow();
// java int firstCol = ar.getFirstCell().getCol();
// java int lastRow = ar.getLastCell().getRow();
// java int lastCol = ar.getLastCell().getCol();
// java if (firstRow > version.getLastRowIndex()) {
// java firstRow = version.getLastRowIndex();
// java }
// java
// java if (lastRow > version.getLastRowIndex()) {
// java lastRow = version.getLastRowIndex();
// java }
// java
// java if (firstCol > version.getLastColumnIndex()) {
// java firstCol = version.getLastColumnIndex();
// java }
// java
// java if (lastCol > version.getLastColumnIndex()) {
// java lastCol = version.getLastColumnIndex();
// java }
// java
// java return new CellRangeAddress(firstRow, lastRow, firstCol, lastCol);
// java }
pub fn cutOff(ar:&AreaReference,v:SpreadsheetVersion)->Self {
Self::new(ar.getFirstCell().getRow().min(v.getLastRowIndex()),ar.getLastCell().getRow().min(v.getLastRowIndex()),(ar.getFirstCell().getCol() as i32).min(v.getLastColumnIndex()),(ar.getLastCell().getCol() as i32).min(v.getLastColumnIndex()))
}
// java public final int getFirstRow() {
// java return this._firstRow;
// java }
pub fn getFirstRow(&self)->i32{self.base.getFirstRow()}
// java public final int getLastRow() {
// java return this._lastRow;
// java }
pub fn getLastRow(&self)->i32{self.base.getLastRow()}
// java public final int getFirstColumn() {
// java return this._firstCol;
// java }
pub fn getFirstColumn(&self)->i32{self.base.getFirstColumn()}
// java public final int getLastColumn() {
// java return this._lastCol;
// java }
pub fn getLastColumn(&self)->i32{self.base.getLastColumn()}
// rust: ExpandHelper.range2rect field transfer, retaining the original range order.
pub fn rect(&self)->KmoRect{KmoRect::KmoRect__bounds(self.getFirstRow(),self.getFirstColumn(),self.getLastRow(),self.getLastColumn())}
}

impl CellRangeAddress{
// java public void setFirstRow(int value) { this.firstRow=value; }
pub fn setFirstRow(&mut self,v:i32){self.base.setFirstRow(v);}
// java public void setLastRow(int value) { this.lastRow=value; }
pub fn setLastRow(&mut self,v:i32){self.base.setLastRow(v);}
// java public void setFirstColumn(int value) { this.firstCol=value; }
pub fn setFirstColumn(&mut self,v:i32){self.base.setFirstColumn(v);}
// java public void setLastColumn(int value) { this.lastCol=value; }
pub fn setLastColumn(&mut self,v:i32){self.base.setLastColumn(v);}
pub fn isInRange(&self,r:i32,c:i32)->bool{self.base.isInRange(r,c)}
}
// rust: Java inherited base view, preserving the original base object fields.
impl std::ops::Deref for CellRangeAddress{type Target=crate::org::apache::poi::ss::util::CellRangeAddressBase::CellRangeAddressBase;fn deref(&self)->&Self::Target{&self.base}}
