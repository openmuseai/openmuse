// java package cn.wps.et.ss.formula.ptg;
// rust: Java String[] access and StringBuffer.append(String) leaf transport.
pub(crate) fn operand(operands:Option<&[Option<String>]>,index:usize)->&str{operands.expect("NullPointerException").get(index).expect("ArrayIndexOutOfBoundsException").as_deref().unwrap_or("null")}
