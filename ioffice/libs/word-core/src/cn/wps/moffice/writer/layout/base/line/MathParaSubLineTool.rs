#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.line.MathParaSubLineTool
use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// Java: MathOperatorCharacter static block — OP_OTHER + ∂ × √ ∛ ∜ + - ± ∓, then sort.
fn is_math_operator_char(ch: char) -> bool {
    use std::sync::OnceLock;
    static OPS: OnceLock<Vec<char>> = OnceLock::new();
    let ops = OPS.get_or_init(|| {
        let mut data: Vec<char> = "÷∝∕*∘∙⋅∩∪⊓⊔∧∨=≠<>≤≥≮≰≯≱≡∼≃≈≅≢≄≉≇∝≪≫∈∋∉⊂⊃⊆⊇≺≻≼≽⊏⊐⊑⊒∥⊥⊢⊣∔∸∖⋒⋓⊟⊠⊡⊞⋇⋉⋊⋋⋌⋏⋎⊝⊺⊕⊖⊗⊘⊙⊛⊚†‡⋆⋄≀△∴∵⋘⋙≦≧≲≳⋖⋗≶⋚≷⋛≑≒≓∽≊⋍≼≽⋞⋟≾≿⋜⋝⊆⊇⊲⊳⊴⊵⊨⋐⋑⊏⊐⊩⊪≖≗≜≏≎∝≬⋔≐⋈"
            .chars()
            .collect();
        data.push('∂');
        data.push('×');
        data.extend(['√', '∛', '∜', '+', '-', '±', '∓']);
        data.sort_unstable();
        data.dedup();
        data
    });
    ops.binary_search(&ch).is_ok()
}

fn is_math_operator(cur: *mut Graphs, typo_para: &TypoPara) -> bool {
    if cur.is_null() || unsafe { (*cur).graphLength } > 1 {
        return false;
    }
    let ch = typo_para.getChar(unsafe { (*cur).graphIndex });
    is_math_operator_char(ch)
}

fn is_math_operator_equal(cur: *mut Graphs, typo_para: &TypoPara) -> bool {
    if cur.is_null() || unsafe { (*cur).graphLength } > 1 {
        return false;
    }
    typo_para.getChar(unsafe { (*cur).graphIndex }) == '='
}

// java public class MathParaSubLineTool
pub struct MathParaSubLineTool;

impl MathParaSubLineTool {
    // java public static Graphs getFirstRollbackGraphs(LayoutState env, SubLineEnv subLineEnv, Graphs run)
    pub fn getFirstRollbackGraphs(env: &mut LayoutState, sub_line: &SubLineEnv, run: *mut Graphs) -> *mut Graphs {
        if run.is_null() {
            return std::ptr::null_mut();
        }
        let first = sub_line.firstGraphNode.unwrap_or(std::ptr::null_mut());
        let mut cur = run;
        while !std::ptr::eq(first, cur) && !is_math_operator(cur, &env.typoPara) {
            cur = env.getGraphList().getPrev(cur);
            if cur.is_null() {
                return std::ptr::null_mut();
            }
        }
        if std::ptr::eq(cur, first) {
            return std::ptr::null_mut();
        }
        let mut first_opt = cur;
        if !is_math_operator_equal(cur, &env.typoPara) {
            let mut prev = env.getGraphList().getPrev(cur);
            while !std::ptr::eq(first, prev) && !prev.is_null() {
                if is_math_operator_equal(prev, &env.typoPara) {
                    first_opt = prev;
                    break;
                }
                prev = env.getGraphList().getPrev(prev);
            }
        }
        first_opt
    }
}
