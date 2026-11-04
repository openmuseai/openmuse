use super::super::filefmt::biff8::util::CellRangeAddress::CellRangeAddress;use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
pub struct ExcelConst;impl ExcelConst{
// java public static CellRangeAddress adaptRegion(CellRangeAddress range, SpreadsheetVersion version) {
// java if (range.getFirstColumn() > version.getMaxColumns() - 1) {
// java range.setFirstColumn(version.getMaxColumns() - 1);
// java }
// java
// java if (range.getLastColumn() > version.getMaxColumns() - 1) {
// java range.setLastColumn(version.getMaxColumns() - 1);
// java }
// java
// java if (range.getFirstRow() > version.getMaxRows() - 1) {
// java range.setFirstRow(version.getMaxRows() - 1);
// java }
// java
// java if (range.getLastRow() > version.getMaxRows() - 1) {
// java range.setLastRow(version.getMaxRows() - 1);
// java }
// java
// java return range;
// java }
pub fn adaptRegion(mut r:CellRangeAddress,v:SpreadsheetVersion)->CellRangeAddress{if r.getFirstColumn()>v.getMaxColumns()-1{r.setFirstColumn(v.getMaxColumns()-1);}if r.getLastColumn()>v.getMaxColumns()-1{r.setLastColumn(v.getMaxColumns()-1);}if r.getFirstRow()>v.getMaxRows()-1{r.setFirstRow(v.getMaxRows()-1);}if r.getLastRow()>v.getMaxRows()-1{r.setLastRow(v.getMaxRows()-1);}r}}
