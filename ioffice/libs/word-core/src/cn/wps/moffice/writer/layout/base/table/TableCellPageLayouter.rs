#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::page::PageLayouter::PageLayouter;
use crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell;
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode;
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;

// java public class TableCellPageLayouter extends PageLayouter<TableCellPageLayouter.IStateEnvoy>
pub struct TableCellPageLayouter {
    // java extends PageLayouter
    pub super_: PageLayouter,
}

impl TableCellPageLayouter {
    // java public TableCellPageLayouter(DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn TableCellPageLayouter(docEnv: DocumentEnv, viewEnv: ViewEnv) -> Self {
        Self { super_: PageLayouter::PageLayouter(docEnv, viewEnv) }
    }

    // java protected void onStart(IStateEnvoy stateEnvoy)
    pub fn onStart(&mut self, state: &mut LayoutState, cell_paint: Option<&(Option<CellInfoNode>, Option<SHD>)>) {
        // java TypoTableCell cell = (TypoTableCell)stateEnvoy.getCurPage();
        let Some(page) = state.curPage.as_ref() else { return };
        let mut cell = TypoTableCell::default();
        cell.assignment(page.snapshot_pointer, page.snapshot);
        // java cell.setDocumentType(stateEnvoy.getDocument().getType());
        if let Some(doc) = state.document.filter(|doc| !doc.is_null()) {
            cell.setDocumentType(unsafe { (&*doc).getType() });
        }
        // java cell.setLeft(cellInfo.getRgdxaLeft());
        // java cell.setRight(TableCellLayoutState.getMergedCellRgdxaRight(cellInfo));
        // java this.calcMargins(stateEnvoy);
        // The row's newCell/prepareEnvs already sets geometry and page margins.
        // java cell.setSHDAndBRCs(cellInfo);
        if let Some((Some(info), shd)) = cell_paint {
            cell.setSHDAndBRCs(info, shd.as_ref());
        }
        // java PageEnv pageEnv = stateEnvoy.getPageEnv();
        let pageEnv = &mut state.pageEnv;
        // java pageEnv.width = cell.width();
        // flow_text receives cell.width() directly from TableRowLayouter.
        // java pageEnv.height = pageEnv.maxLayoutHeight + pageEnv.marginTop + pageEnv.marginBottom;
        pageEnv.height = pageEnv.maxLayoutHeight
            .wrapping_add(pageEnv.marginTop)
            .wrapping_add(pageEnv.marginBottom);
        // java pageEnv.maxLayoutHeight = pageEnv.getCoreHeight();
        pageEnv.maxLayoutHeight = pageEnv.getCoreHeight();
        // java pageEnv.minLayoutHeight = 0;
        pageEnv.minLayoutHeight = 0;
        // java stateEnvoy.setCurLayoutHeight(pageEnv.minLayoutHeight);
        state.curLayoutHeight = pageEnv.minLayoutHeight;
    }
}
