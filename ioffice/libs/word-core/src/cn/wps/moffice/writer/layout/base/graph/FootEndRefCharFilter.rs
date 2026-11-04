#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.graph.FootEndRefCharFilter
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::FilterArgs::FilterArgs;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReader::GraphReader;
use crate::cn::wps::moffice::writer::layout::base::graph::IFilter::IFilter;

const BUF_SIZE: i32 = 128;

// java public class FootEndRefCharFilter implements IFilter
pub struct FootEndRefCharFilter {
    reader: *mut GraphReader,
    max: i32,
    buff: Vec<char>,
    buff_start: i32,
    buff_len: i32,
    ref_cp: i32,
}

impl FootEndRefCharFilter {
    // java FootEndRefCharFilter(int start, int end, GraphReader graphReader, LayoutState layoutState)
    pub fn new(start: i32, end: i32, reader: *mut GraphReader, layout: &LayoutState) -> Self {
        let cap = (end - start).min(BUF_SIZE).max(0) as usize;
        let mut filter = Self {
            reader,
            max: end,
            buff: vec!['\u{0}'; cap],
            buff_start: -1,
            buff_len: 0,
            ref_cp: end,
        };
        filter.ref_cp = filter.find_ref(start, layout);
        filter
    }

    // java private int findRefChar(int start, LayoutState layoutState)
    fn find_ref(&mut self, start: i32, layout: &LayoutState) -> i32 {
        let Some(doc) = layout.document.filter(|p| !p.is_null()) else { return self.max; };
        let Some(rope) = (unsafe { (*doc).getTextRope() }) else { return self.max; };
        let rope = unsafe { &mut *rope };
        let mut cur = start;
        while cur < self.max {
            self.read_buff(cur, rope);
            let mut offset = cur - self.buff_start;
            let max = self.buff_len;
            while offset < max {
                // java if (buff[offset] == 2)
                if self.buff.get(offset as usize).copied() == Some('\u{2}') {
                    return offset + self.buff_start;
                }
                offset += 1;
            }
            cur = offset + self.buff_start;
        }
        self.max
    }

    // java private void readBuff(char[] buff, int curCP, ITextRope textRope)
    fn read_buff(&mut self, cur_cp: i32, rope: &mut dyn ITextRope) {
        if cur_cp >= self.max {
            return;
        }
        if self.buff_start < 0 || cur_cp < self.buff_start || cur_cp >= self.buff_start + self.buff_len {
            self.buff_start = cur_cp;
            self.buff_len = (self.buff.len() as i32).min(self.max - cur_cp);
            rope.getText(self.buff_start, self.buff_start + self.buff_len, &mut self.buff, 0);
        }
    }
}

impl IFilter for FootEndRefCharFilter {
    // java public boolean doFilter(FilterArgs args, LayoutState layoutState)
    fn doFilter(&mut self, args: &mut FilterArgs, layout: &mut LayoutState) -> bool {
        if args.start > self.ref_cp {
            self.ref_cp = self.find_ref(args.start, layout);
        }
        if args.start == self.ref_cp {
            unsafe { (*self.reader).readFootEndNoteRef(self.ref_cp, layout); }
            args.next = self.ref_cp + 1;
            true
        } else {
            args.end = args.end.min(self.ref_cp);
            args.next = args.next.min(self.ref_cp);
            false
        }
    }
}
