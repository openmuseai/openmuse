#![allow(dead_code, non_snake_case)]
// Java: public class EquationParser
use std::any::Any;

use util::moffice::property::PropertySet::PropertySet;
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::IChpxTable::{Entry as ChpxEntry, IChpxTable};
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
use crate::cn::wps::moffice::writer::data::ITable::Entry as TableEntry;
use crate::cn::wps::moffice::writer::layout::base::env::PropertyTool::PropertyTool;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReaderTool::GraphReaderTool;

const MAX_BUF_SIZE: i32 = 128;
const STATE_NONE: i32 = 0;
const STATE_START_OP: i32 = 1;
const STATE_IN_OP: i32 = 2;
const STATE_START_VALUE: i32 = 4;
const STATE_IN_VALUE: i32 = 5;
const STATE_TEXT: i32 = 6;
const STATE_START_COMMENT: i32 = 7;
const STATE_IN_COMMENT: i32 = 8;
const STATE_AFTER_COMMENT: i32 = 9;

// java public class EquationParser
pub struct EquationParser;

impl EquationParser {
    // java public static EquationResult parse(Document doc, IPropertyTool propertyTool, int start, int end, boolean enableRevision, boolean revisionFinal)
    pub fn parse(
        doc: &dyn Document,
        property_tool: &mut PropertyTool,
        start: i32,
        end: i32,
        enable_revision: bool,
        revision_final: bool,
    ) -> Option<EquationResult> {
        // java if (start >= end) return null;
        if start >= end {
            return None;
        }
        let mut eq_result = EquationResult::EquationResult(start);
        let text_rope = match doc.getTextRope() {
            Some(rope) if !rope.is_null() => rope,
            _ => {
                eq_result.setError();
                return Some(eq_result);
            }
        };
        let papx_ptr = match doc.getPapxTable() {
            Some(p) if !p.is_null() => p,
            _ => {
                eq_result.setError();
                return Some(eq_result);
            }
        };
        let chpx_ptr = match doc.getChpxTable() {
            Some(p) if !p.is_null() => p,
            _ => {
                eq_result.setError();
                return Some(eq_result);
            }
        };
        let para_prop = {
            let papx = unsafe { &*papx_ptr };
            let para = match IPapxTable::seek(papx, start) {
                Some(entry) => entry,
                None => {
                    eq_result.setError();
                    return Some(eq_result);
                }
            };
            property_tool.getFinalParaProp(para.as_ref(), revision_final).clone()
        };
        // java char[] buf = new char[Math.min(end - start, 128)];
        let buf_len = (end - start).min(MAX_BUF_SIZE).max(0) as usize;
        let mut buf = vec!['\0'; buf_len];
        let mut cur_cp = start;
        let mut state = STATE_NONE;
        let mut embed_level = 0i32;
        let mut last_ch = '\0';
        let chpx = unsafe { &*chpx_ptr };
        let rope = unsafe { &*text_rope };

        while cur_cp < end {
            let run = match IChpxTable::seek(chpx, cur_cp) {
                Some(entry) => entry,
                None => {
                    eq_result.setError();
                    return Some(eq_result);
                }
            };
            let prop = property_tool
                .getFinalRunPropert(run.as_ref(), &para_prop, revision_final)
                .clone();
            let run_end_cp = TableEntry::endCp(run.as_ref());
            // java if (isHiden(prop, enableRevision, revisionFinal)) curCP = runEndCP;
            if is_hiden(&prop, enable_revision, revision_final) {
                cur_cp = run_end_cp;
                continue;
            }
            let char_symbol = prop.getValue(20).and_then(|v| v.as_any().downcast_ref::<i32>().copied());
            let font_symbol = prop.getValue(19).and_then(|v| v.as_any().downcast_ref::<String>().cloned());
            // java boolean needConvertSymbol = null != charSymbol && null != fontSymbol;
            let need_convert_symbol = char_symbol.is_some() && font_symbol.is_some();
            let max = run_end_cp.min(end);
            while cur_cp < max {
                let mut cur_index = 0usize;
                let max_index = ((max - cur_cp) as usize).min(buf.len());
                if max_index == 0 {
                    break;
                }
                rope.getText(cur_cp, cur_cp + max_index as i32, &mut buf, 0);
                while cur_index < max_index {
                    let mut ch = buf[cur_index];
                    cur_index += 1;
                    if need_convert_symbol && ch == '(' {
                        ch = font_symbol.as_ref().unwrap().chars().next().unwrap_or('(');
                    }
                    let stop = step_char(
                        doc,
                        &mut eq_result,
                        &mut state,
                        &mut embed_level,
                        &mut cur_cp,
                        &mut cur_index,
                        max_index,
                        start,
                        ch,
                        last_ch,
                    );
                    if stop {
                        return Some(eq_result);
                    }
                    last_ch = ch;
                    cur_cp += 1;
                }
            }
        }

        // java if (6 == state) commitTextResult else if (8 != state && 9 != state) setError else commitCommentResult
        if state == STATE_TEXT {
            eq_result.commitTextResult(cur_cp);
        } else if state != STATE_IN_COMMENT && state != STATE_AFTER_COMMENT {
            eq_result.setError();
        } else {
            eq_result.commitCommentResult(cur_cp);
        }
        Some(eq_result)
    }
}

fn step_char(
    doc: &dyn Document,
    eq_result: &mut EquationResult,
    state: &mut i32,
    embed_level: &mut i32,
    cur_cp: &mut i32,
    cur_index: &mut usize,
    max_index: usize,
    start: i32,
    ch: char,
    last_ch: char,
) -> bool {
    match *state {
        STATE_NONE => match ch {
            '\u{0013}' => {
                if !take_field(doc, eq_result, cur_cp, cur_index, max_index, true) {
                    return true;
                }
                *state = STATE_TEXT;
            }
            '\\' => *state = STATE_START_OP,
            _ => {
                *state = STATE_TEXT;
                eq_result.addText(*cur_cp);
            }
        },
        STATE_START_OP => match ch {
            '*' => {
                // java if (start + 1 == curCP) state = 7; else { state = 2; addOP }
                if start + 1 == *cur_cp {
                    *state = STATE_START_COMMENT;
                } else {
                    *state = STATE_IN_OP;
                    eq_result.addOP(ch, *cur_cp);
                }
            }
            '\\' => {
                *state = STATE_TEXT;
                eq_result.addText(*cur_cp);
            }
            _ => {
                *state = STATE_IN_OP;
                eq_result.addOP(ch, *cur_cp);
            }
        },
        STATE_IN_OP => match ch {
            ' ' => {}
            '(' => {
                eq_result.commitOP();
                *state = STATE_START_VALUE;
                *embed_level += 1;
            }
            '*' => {
                eq_result.setError();
                return true;
            }
            '\\' => {
                eq_result.commitOP();
                *state = STATE_START_OP;
            }
            _ => eq_result.addOP(ch, *cur_cp),
        },
        STATE_START_VALUE => match ch {
            '\u{0013}' => {
                if !take_field(doc, eq_result, cur_cp, cur_index, max_index, false) {
                    return true;
                }
                *state = STATE_IN_VALUE;
            }
            '(' => {
                *embed_level += 1;
                eq_result.addValue(*cur_cp);
            }
            ')' => {
                *embed_level -= 1;
                if *embed_level < 0 {
                    eq_result.setError();
                    return true;
                }
                if *embed_level > 0 {
                    eq_result.addValue(*cur_cp);
                } else if *embed_level == 0 {
                    eq_result.commitEquationResult(*cur_cp);
                    return true;
                }
            }
            ',' => eq_result.addEmptyValue(*cur_cp),
            _ => {
                *state = STATE_IN_VALUE;
                eq_result.addValue(*cur_cp);
            }
        },
        STATE_IN_VALUE => match ch {
            '\u{0013}' => {
                if !take_field(doc, eq_result, cur_cp, cur_index, max_index, false) {
                    return true;
                }
            }
            '(' => {
                if last_ch != '\\' {
                    *embed_level += 1;
                }
                eq_result.addValue(*cur_cp);
            }
            ')' => {
                if last_ch != '\\' {
                    *embed_level -= 1;
                }
                if *embed_level < 0 {
                    eq_result.setError();
                    return true;
                }
                if *embed_level > 0 {
                    eq_result.addValue(*cur_cp);
                } else if *embed_level == 0 {
                    eq_result.commitEquationResult(*cur_cp);
                    return true;
                }
            }
            ',' => {
                if *embed_level == 1 {
                    eq_result.commitValue();
                    *state = STATE_START_VALUE;
                } else {
                    eq_result.addValue(*cur_cp);
                }
            }
            _ => eq_result.addValue(*cur_cp),
        },
        STATE_TEXT => {
            if ch == '\\' {
                eq_result.commitTextResult(*cur_cp);
                return true;
            }
            if ch == '\u{0013}' {
                if !take_field(doc, eq_result, cur_cp, cur_index, max_index, true) {
                    return true;
                }
            } else {
                eq_result.addText(*cur_cp);
            }
        }
        STATE_START_COMMENT => {
            if ch != ' ' {
                *state = STATE_IN_COMMENT;
            }
            if ch == '"' {
                *embed_level += 1;
            }
        }
        STATE_IN_COMMENT => {
            if ch == '"' {
                if *embed_level == 0 {
                    *embed_level += 1;
                } else {
                    *embed_level -= 1;
                }
            }
            if ch == ' ' && *embed_level == 0 {
                *state = STATE_AFTER_COMMENT;
            }
        }
        STATE_AFTER_COMMENT => {
            if ch != ' ' {
                eq_result.commitCommentResult(*cur_cp);
                return true;
            }
        }
        _ => {}
    }
    false
}

fn take_field(
    doc: &dyn Document,
    eq_result: &mut EquationResult,
    cur_cp: &mut i32,
    cur_index: &mut usize,
    max_index: usize,
    as_text: bool,
) -> bool {
    let field_ptr = match doc.getPLCField() {
        Some(p) if !p.is_null() => p,
        _ => {
            eq_result.setError();
            return false;
        }
    };
    let field = unsafe { &mut *field_ptr };
    let Some(triple) = field.getContainingField(*cur_cp) else {
        eq_result.setError();
        return false;
    };
    if as_text {
        eq_result.addText(*cur_cp);
    } else {
        eq_result.addValue(*cur_cp);
    }
    *cur_index = max_index;
    *cur_cp = triple.getEndCP();
    if as_text {
        eq_result.addText(*cur_cp);
    } else {
        eq_result.addValue(*cur_cp);
    }
    true
}

// java private static boolean isHiden(PropertySet prop, boolean enableRevision, boolean revisionFinal)
fn is_hiden(prop: &PropertySet, enable_revision: bool, revision_final: bool) -> bool {
    // java if (GraphReaderTool.isVanishedText(prop)) return true;
    if GraphReaderTool::isVanishedText(prop) {
        return true;
    }
    if enable_revision {
        // java revisionFinal ? prop.getValue(45) : prop.getValue(46)
        let id = if revision_final { 45 } else { 46 };
        if prop.getValue(id).is_some() {
            return true;
        }
    }
    false
}
