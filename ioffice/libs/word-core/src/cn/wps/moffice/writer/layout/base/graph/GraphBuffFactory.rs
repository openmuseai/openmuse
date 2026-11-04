// Java: cn.wps.moffice.writer.layout.base.graph.GraphBuffFactory
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphFactory::GraphFactory;

// java public class GraphBuffFactory
pub struct GraphBuffFactory {}

impl GraphBuffFactory {
    // java public static GraphBuff createGraphBuff(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn createGraphBuff(mainState: MainState, docEnv: DocumentEnv, viewEnv: ViewEnv) -> GraphFactory {
        // java return new GraphFactory(mainState, docEnv, viewEnv);
        GraphFactory::GraphFactory(mainState, docEnv, viewEnv)
    }
}
