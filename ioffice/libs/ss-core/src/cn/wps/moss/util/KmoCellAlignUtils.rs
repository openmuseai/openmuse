pub struct KmoCellAlignUtils;
impl KmoCellAlignUtils{
// java public static short calcFinalHAlign(short align, short angle, int cellType) {
// java if (align == 0) {
// java switch (cellType) {
// java case 1:
// java if ((angle <= 0 || angle >= 90) && angle != -90) {
// java align = 3;
// java } else {
// java align = 1;
// java }
// java break;
// java case 2:
// java if ((angle <= 0 || angle >= 90) && angle != -90 && angle != 0) {
// java align = 3;
// java } else {
// java align = 1;
// java }
// java case 3:
// java case 4:
// java default:
// java break;
// java case 5:
// java case 6:
// java align = 2;
// java }
// java }
// java
// java return align;
// java }
pub fn calcFinalHAlign(mut align:i16,angle:i16,cellType:i32)->i16{if align==0{match cellType{1=>align=if (angle<=0||angle>=90)&&angle!= -90{3}else{1},2=>align=if (angle<=0||angle>=90)&&angle!= -90&&angle!=0{3}else{1},5|6=>align=2,_=>{}}}align}}
