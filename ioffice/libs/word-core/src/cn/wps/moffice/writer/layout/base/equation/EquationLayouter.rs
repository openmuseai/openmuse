#![allow(dead_code, non_snake_case, unused_variables)]
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.layout.base.env.PropertyTool
use crate::cn::wps::moffice::writer::layout::base::env::PropertyTool::PropertyTool;
// Java: import cn.wps.moffice.writer.layout.base.equation.EquationFactory
use crate::cn::wps::moffice::writer::layout::base::equation::EquationALayouter::EquationALayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationBLayouter::EquationBLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationContentLayouter::EquationContentLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationDLayouter::EquationDLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationFLayouter::EquationFLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationFactory::{EquationCreate, EquationFactory, EquationLayouterKind};
use crate::cn::wps::moffice::writer::layout::base::equation::EquationRLayouter::EquationRLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationILayouter::EquationILayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLLayouter::EquationLLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationLinearLayouter::EquationLinearLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationOLayouter::EquationOLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationParser::EquationParser;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationResult::EquationResult;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationSLayouter::EquationSLayouter;
use crate::cn::wps::moffice::writer::layout::base::equation::EquationXLayouter::EquationXLayouter;

// java public class EquationException
#[derive(Debug)]
pub struct EquationException;

// java public class EquationLayouter
pub struct EquationLayouter;

impl EquationLayouter {
    // java public static int getEquation(..., boolean needCalcGraphHeight)
    pub fn getEquation(
        main_state: &mut MainState,
        document_env: &mut DocumentEnv,
        view_env: &mut ViewEnv,
        document: Option<*mut dyn Document>,
        start: i32,
        end: i32,
        enable_revision: bool,
        revision_final: bool,
        need_calc_graph_height: bool,
    ) -> Result<i32, EquationException> {
        let Some(doc_ptr) = document.filter(|p| !p.is_null()) else {
            return Err(EquationException);
        };
        let doc = unsafe { &*doc_ptr };
        // java documentEnv.propToolFactory.create(doc). Reuse is a cache; a new PropertyTool merges the same props.
        let doc_type = doc.getType__on_IDocument();
        let (para_cache, run_cache) = if doc_type == 0 { (64, 128) } else { (4, 4) };
        let mut property_tool = PropertyTool::PropertyTool(doc, para_cache, run_cache);
        // java EquationResult eqResult = EquationParser.parse(...)
        let Some(eq_result) = EquationParser::parse(doc, &mut property_tool, start, end, enable_revision, revision_final) else {
            // java null == eqResult → return 0
            return Ok(0);
        };
        // java eqResult.isError() → throw new EquationException()
        if eq_result.isError() {
            return Err(EquationException);
        }
        // java IEquationLayouter eqLayouter = EquationFactory.create(...)
        Self::flow_child(
            main_state,
            document_env,
            view_env,
            doc_ptr,
            &eq_result,
            end,
            enable_revision,
            revision_final,
            need_calc_graph_height,
        )
    }

    // java EquationFactory.create then eqLayouter.flow. Linear passes getEndCP() as end.
    pub fn flow_child(
        main_state: &mut MainState,
        document_env: &mut DocumentEnv,
        view_env: &mut ViewEnv,
        document: *mut dyn Document,
        eq_result: &EquationResult,
        end: i32,
        enable_revision: bool,
        revision_final: bool,
        need_calc_graph_height: bool,
    ) -> Result<i32, EquationException> {
        match EquationFactory::create(eq_result, end) {
            EquationCreate::Null => Ok(0),
            EquationCreate::Throw => Err(EquationException),
            EquationCreate::Kind(EquationLayouterKind::Text) => Ok(EquationContentLayouter::get_drawing(
                main_state,
                document_env,
                view_env,
                document,
                eq_result.getTextStart(),
                eq_result.getTextEnd(),
                enable_revision,
                need_calc_graph_height,
            )),
            EquationCreate::Kind(EquationLayouterKind::Linear) => EquationLinearLayouter::flow(
                main_state,
                document_env,
                view_env,
                document,
                eq_result.clone(),
                end,
                enable_revision,
                revision_final,
                need_calc_graph_height,
            ),
            EquationCreate::Kind(EquationLayouterKind::D) => Ok(EquationDLayouter::flow(document, eq_result, end)),
            EquationCreate::Kind(EquationLayouterKind::I) => EquationILayouter::flow(
                main_state,
                document_env,
                view_env,
                document,
                eq_result,
                end,
                enable_revision,
                revision_final,
            ),
            EquationCreate::Kind(EquationLayouterKind::L) => EquationLLayouter::flow(
                main_state,
                document_env,
                view_env,
                document,
                eq_result,
                end,
                enable_revision,
                revision_final,
            ),
            EquationCreate::Kind(EquationLayouterKind::O) => EquationOLayouter::flow(
                main_state,
                document_env,
                view_env,
                document,
                eq_result,
                end,
                enable_revision,
                revision_final,
            ),
            EquationCreate::Kind(EquationLayouterKind::B) => EquationBLayouter::flow(
                main_state,
                document_env,
                view_env,
                document,
                eq_result,
                end,
                enable_revision,
                revision_final,
            ),
            EquationCreate::Kind(EquationLayouterKind::A) => EquationALayouter::flow(
                main_state,
                document_env,
                view_env,
                document,
                eq_result,
                end,
                enable_revision,
                revision_final,
            ),
            EquationCreate::Kind(EquationLayouterKind::S) => EquationSLayouter::flow(
                main_state,
                document_env,
                view_env,
                document,
                eq_result,
                end,
                enable_revision,
                revision_final,
            ),
            EquationCreate::Kind(EquationLayouterKind::X) => EquationXLayouter::flow(
                main_state,
                document_env,
                view_env,
                document,
                eq_result,
                end,
                enable_revision,
                revision_final,
            ),
            EquationCreate::Kind(EquationLayouterKind::F) => EquationFLayouter::flow(
                main_state,
                document_env,
                view_env,
                document,
                eq_result,
                end,
                enable_revision,
                revision_final,
            ),
            EquationCreate::Kind(EquationLayouterKind::R) => EquationRLayouter::flow(
                main_state,
                document_env,
                view_env,
                document,
                eq_result,
                end,
                enable_revision,
                revision_final,
            ),
            EquationCreate::Kind(_) => Ok(0),
        }
    }

    // java public static int getEquation(...) without needCalcGraphHeight
    pub fn getEquation__mainstate_documentenv_viewenv_document_int_int_boolean_boolean(
        main_state: &mut MainState,
        document_env: &mut DocumentEnv,
        view_env: &mut ViewEnv,
        document: Option<*mut dyn Document>,
        start: i32,
        end: i32,
        enable_revision: bool,
        revision_final: bool,
    ) -> Result<i32, EquationException> {
        Self::getEquation(main_state, document_env, view_env, document, start, end, enable_revision, revision_final, false)
    }

    // java public static int getExceptionEquation(...)
    pub fn getExceptionEquation(
        main_state: &mut MainState,
        document_env: &DocumentEnv,
        view_env: &ViewEnv,
        document: Option<*mut dyn Document>,
        final_run_prop: Option<&PropertySet>,
    ) -> i32 {
        let _ = (main_state, document_env, view_env, document, final_run_prop);
        // java EquationContentLayouter.flow; textBox == 0 → return 0
        // EquationContentLayouter is not in this compile closure yet.
        0
    }
}
