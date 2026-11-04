#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.graph.FunctionSepField
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::FilterArgs::FilterArgs;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReader::GraphReader;
use crate::cn::wps::moffice::writer::layout::base::graph::IFilter::IFilter;

// java public class FunctionSepField implements IFilter
pub struct FunctionSepField {
    // java private int mSepCP;
    mSepCP: i32,
    // java private GraphReader mGraphReader;
    // Java stores the reader that owns this filter. The pointer is that same object.
    mGraphReader: *mut GraphReader,
}

impl FunctionSepField {
    // java public FunctionSepField(int sepCP, GraphReader graphReader)
    pub fn new(sepCP: i32, graphReader: *mut GraphReader) -> Self {
        Self { mSepCP: sepCP, mGraphReader: graphReader }
    }
}

impl IFilter for FunctionSepField {
    // java public boolean doFilter(FilterArgs args, LayoutState layoutState)
    fn doFilter(&mut self, args: &mut FilterArgs, layoutState: &mut LayoutState) -> bool {
        let reader = self.mGraphReader;
        // java if (args.start > this.mSepCP)
        if args.start > self.mSepCP {
            // java this.mGraphReader.setIgnoreCapsAndSmallCaps(true);
            unsafe { (*reader).setIgnoreCapsAndSmallCaps(true); }
        } else if args.start == self.mSepCP {
            // java this.mGraphReader.readMathChar(' ', this.mSepCP, layoutState);
            unsafe { (*reader).readMathChar(' ', self.mSepCP, layoutState); }
            // java args.next = this.mSepCP + 1;
            args.next = self.mSepCP + 1;
            // java return true;
            return true;
        } else {
            // java this.mGraphReader.setIgnoreCapsAndSmallCaps(false);
            unsafe { (*reader).setIgnoreCapsAndSmallCaps(false); }
            // java args.next = Math.min(args.next, this.mSepCP);
            args.next = args.next.min(self.mSepCP);
            // java args.end = Math.min(args.end, this.mSepCP);
            args.end = args.end.min(self.mSepCP);
        }
        // java return false;
        false
    }
}
