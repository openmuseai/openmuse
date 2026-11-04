use super::HLStrHandlerBuilder::HLStrHandlerBuilder;use super::super::HLLabelParser::HLLabelParser;pub struct HLStrLabelParser;impl HLStrLabelParser{// java public static void parseLabel(char[] chars, int start, int end, int type, HLStrHandlerBuilder handler) {
// java ++start;
// java if (type == 4) {
// java ++start;
// java }
// java
// java --end;
// java if (type == 2 || type == 3) {
// java --end;
// java }
// java
// java if (start > end) {
// java throw new IllegalStateException();
// java } else {
// java if (type != 0 && type != 2 && type != 4) {
// java if (type == 1 || type == 3) {
// java int index = HLLabelParser.getNextEmptySpace(chars, start, end);
// java String label = new String(chars, start, index - start);
// java handler.startElement(label);
// java
// java int space;
// java for(int var11 = HLLabelParser.passEmptySpace(chars, index, end); var11 < end; var11 = HLLabelParser.passEmptySpace(chars, space, end)) {
// java space = HLLabelParser.getNextEmptySpaceV2(chars, var11, end);
// java if (space == -1) {
// java parseAttr(chars, var11, end, handler);
// java break;
// java }
// java
// java parseAttr(chars, var11, space - 1, handler);
// java }
// java
// java if (type == 3) {
// java handler.endElement(label);
// java }
// java }
// java } else {
// java String label = new String(chars, start, end - start + 1);
// java switch (type) {
// java case 0:
// java handler.startElement(label);
// java case 1:
// java case 3:
// java default:
// java break;
// java case 2:
// java handler.startElement(label);
// java handler.endElement(label);
// java break;
// java case 4:
// java handler.endElement(label);
// java }
// java }
// java
// java }
// java }
pub fn parseLabel(chars:&[char],mut start:usize,mut end:usize,t:i32,h:&mut HLStrHandlerBuilder) {
start+=1;if t==4{start+=1;}end-=1;if t==2||t==3{end-=1;}assert!(start<=end,"IllegalStateException");if t!=0&&t!=2&&t!=4{if t==1||t==3{let index=HLLabelParser::get_next_empty_space(chars,start,end)as usize;let label=chars[start..index].iter().collect::<String>();h.start_element(&label);let mut pos=HLLabelParser::pass_empty_space(chars,index,end);while pos<end{let space=HLLabelParser::get_next_empty_space_v2(chars,pos,end);if space== -1{Self::parseAttr(chars,pos,end,h);break;}Self::parseAttr(chars,pos,space as usize-1,h);pos=HLLabelParser::pass_empty_space(chars,space as usize,end);}if t==3{h.end_element(&label);}}}else{let label=chars[start..=end].iter().collect::<String>();match t{0=>h.start_element(&label),2=>{h.start_element(&label);h.end_element(&label);},4=>h.end_element(&label),_=>{}}}
}
// java private static void parseAttr(char[] chars, int start, int end, HLStrHandlerBuilder handler) {
// java int equal = HLLabelParser.getNextChar(chars, start, end, '=');
// java if (equal != -1) {
// java while(HLLabelParser.isEmptyChar(chars[end]) && end > equal) {
// java --end;
// java }
// java
// java if (chars[equal + 1] == '\'' && chars[end] == '\'' || chars[equal + 1] == '"' && chars[end] == '"') {
// java String name = new String(chars, start, equal - start);
// java String value = HLLabelParser.buildValue(chars, equal + 2, end - 1, true);
// java handler.addAttr(name, value);
// java }
// java
// java }
// java }
pub fn parseAttr(chars:&[char],start:usize,mut end:usize,h:&mut HLStrHandlerBuilder) {
let eq=HLLabelParser::get_next_char(chars,start,end,'=');if eq!= -1{let eq=eq as usize;while HLLabelParser::is_empty_char(chars[end])&&end>eq{end-=1;}if(chars[eq+1]=='\''&&chars[end]=='\'')||(chars[eq+1]=='"'&&chars[end]=='"'){let name=chars[start..eq].iter().collect::<String>();let value=HLLabelParser::build_value(chars,eq+2,end-1,true);h.add_attr(&name,&value);}}
}
}
