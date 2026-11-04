// Java: import cn.wps.collections.KPList
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::KPList;
// Java: import cn.wps.moffice.writer.cache.TypoPara
use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
// Java: import cn.wps.moffice.writer.cache.TypoRun
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.layout.base.wordbreaker.HyphenFlag
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::wordbreaker::HyphenFlag::HyphenFlag;
// Java: import cn.wps.moffice.writer.layout.base.line.SubLineEnv
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// java public final class LineRuleGraphsPos
#[derive(Default)]
pub struct LineRuleGraphsPos {
    // java private Graphs mGraphs;
    mGraphs: Option<Graphs>,
    // java private int mGraphIndex;
    mGraphIndex: i32,
    // java private Graphs mPrevLimitGraphs;
    mPrevLimitGraphs: Option<Graphs>,
    // java private int mPrevLimitGraphIndex;
    mPrevLimitGraphIndex: i32,
}

// java public final class LineRuleGraphsPos
impl LineRuleGraphsPos {
    // java public void init(Graphs graphs, int graphIdx)
    pub fn init(&mut self, graphs: Graphs, graphIdx: i32) {
      // java this.setGraphsAndIndex(graphs, graphIdx);
      self.setGraphsAndIndex(graphs, graphIdx);
      // java this.setPrevLimitGraphsAndIndex((Graphs)null, Integer.MIN_VALUE);
      self.setPrevLimitGraphsAndIndex(None, i32::MIN);
    }

    // java public void copy(LineRuleGraphsPos pos)
    pub fn copy(&mut self, pos: &LineRuleGraphsPos) {
      // java this.setGraphsAndIndex(pos.mGraphs, pos.mGraphIndex);
      self.setGraphsAndIndex(pos.getGraphs(), pos.mGraphIndex);
      // java this.setPrevLimitGraphsAndIndex(pos.mPrevLimitGraphs, pos.mPrevLimitGraphIndex);
      self.setPrevLimitGraphsAndIndex(pos.mPrevLimitGraphs.as_ref().map(|g| g.clone()), pos.mPrevLimitGraphIndex);
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mGraphs = null;
      self.mGraphs = None;
      // java this.mGraphIndex = Integer.MIN_VALUE;
      self.mGraphIndex = i32::MIN;
    }

    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mGraphs = null;
      self.mGraphs = None;
      // java this.mGraphIndex = Integer.MIN_VALUE;
      self.mGraphIndex = i32::MIN;
      // java this.mPrevLimitGraphs = null;
      self.mPrevLimitGraphs = None;
      // java this.mPrevLimitGraphIndex = Integer.MIN_VALUE;
      self.mPrevLimitGraphIndex = i32::MIN;
    }

    // java private void assertIsValidGraphsAndIndex(Graphs graphs, int graphIdx)
    pub fn assertIsValidGraphsAndIndex(&self, graphs: Option<&Graphs>, graphIdx: i32) {
    }

    // java public void setGraphsAndIndex(Graphs graphs, int graphIdx)
    pub fn setGraphsAndIndex(&mut self, graphs: Graphs, graphIdx: i32) {
      // java this.assertIsValidGraphsAndIndex(graphs, graphIdx);
      self.assertIsValidGraphsAndIndex(Some(&graphs), graphIdx);
      // java this.mGraphs = graphs;
      self.mGraphs = Some(graphs);
      // java this.mGraphIndex = graphIdx;
      self.mGraphIndex = graphIdx;
    }

    // java public void setPrevLimitGraphsAndIndex(Graphs graphs, int graphIdx)
    pub fn setPrevLimitGraphsAndIndex(&mut self, graphs: Option<Graphs>, graphIdx: i32) {
      // java this.assertIsValidGraphsAndIndex(graphs, graphIdx);
      self.assertIsValidGraphsAndIndex(graphs.as_ref(), graphIdx);
      // java this.mPrevLimitGraphs = graphs;
      self.mPrevLimitGraphs = graphs;
      // java this.mPrevLimitGraphIndex = graphIdx;
      self.mPrevLimitGraphIndex = graphIdx;
    }

    // java public void clearPrevLimitGraphsAndIndex()
    pub fn clearPrevLimitGraphsAndIndex(&mut self) {
      // java this.setPrevLimitGraphsAndIndex((Graphs)null, Integer.MIN_VALUE);
      self.setPrevLimitGraphsAndIndex(None, i32::MIN);
    }

    // java public Graphs getGraphs()
    pub fn getGraphs(&self) -> Graphs {
      // java return this.mGraphs;
      match &self.mGraphs {
        Some(g) => {
          let mut c = Graphs::Graphs();
          Graphs::clone(&mut c, g);
          c
        }
        None => Graphs::Graphs(),
      }
    }

    // java public int getGraphIndex()
    pub fn getGraphIndex(&self) -> i32 {
      // java return this.mGraphIndex;
      return self.mGraphIndex;
    }

    // java public int getBeginBdrWidth(int index)
    pub fn getBeginBdrWidth(&self, index: i32) -> i32 {
      // java return index == this.mGraphs.graphIndex && this.mGraphs.hasBeginBdr() ? this.mGraphs.getBdrWidth() : 0;
      return (if self.mGraphs.as_ref().map(|g| index == g.graphIndex && g.hasBeginBdr()).unwrap_or(false) {
        self.mGraphs.as_ref().map(|g| g.getBdrWidth()).unwrap_or(0)
      } else { 0 });
    }

    // java public char getChar(TypoPara typoPara)
    pub fn getChar(&self, typoPara: &TypoPara) -> char {
      // java return typoPara.getChar(this.mGraphIndex);
      return typoPara.getChar(self.mGraphIndex);
    }

    // java public boolean isPartOfComplexGraphs()
    pub fn isPartOfComplexGraphs(&self) -> bool {
      // java Graphs graphs = this.mGraphs;
      let mut graphs: Graphs = self.getGraphs();
      // java if (this.isInvalid())
      if self.isInvalid() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return TypoRun.isComplexLangType(graphs.language) && (graphs.graphLength > 1 || 6 == graphs.language);
        return TypoRun::isComplexLangType(graphs.language) && (graphs.graphLength > 1 || 6 == graphs.language);
      }
    }

    // java public boolean isPartOfWord(int compressLevel, TypoPara typoPara)
    pub fn isPartOfWord(&self, compressLevel: i32, typoPara: &TypoPara) -> bool {
      // java Graphs graphs = this.mGraphs;
      let mut graphs: Graphs = self.getGraphs();
      // java if (this.isInvalid())
      if self.isInvalid() {
        // java return false;
        return false;
      }
      // java else if (this.isPartOfComplexGraphs())
      else if self.isPartOfComplexGraphs() {
        // java return true;
        return true;
      }
      // java else
      else {
        // java int runType = graphs.runType;
        let mut runType: i32 = graphs.runType;
        // java int langType = graphs.language;
        let mut langType: i32 = graphs.language;
        // java if (1 == runType)
        if 1 == runType {
          // java return TypoRun.isWestLangType(graphs.language);
          return TypoRun::isWestLangType(graphs.language);
        }
        // java else
        else {
          // java switch (runType)
          match runType {
            // java case 6:
            6 => {
              // java if (graphs.charsRef.isInMath)
              if graphs.charsRef.as_ref().map(|c| c.isInMath).unwrap_or(false) {
                // java return true;
                return true;
              }
              match langType {
                6 | 7 => return true,
                _ => return false,
              }
            }
            // java case 15:
            15 => {
              // java return true;
              return true;
            }
            // java case 18:
            18 => {
              // java if (0 == compressLevel)
              if 0 == compressLevel {
                // java switch (this.getChar(&typoPara))
                match self.getChar(&typoPara) {
                  // java case '-': case '–': case '—':
                  '-' | '–' | '—' => {
                    // java return false;
                    return false;
                  }
                  // java default:
                  _ => {
                  }
                }
              }
              // java return TypoRun.isWestLangType(graphs.language);
              return TypoRun::isWestLangType(graphs.language);
            }
            // Java's default follows the specific cases. Rust's wildcard
            // must be last or it consumes runType 15/18 before their logic.
            // java default: switch (langType)
            _ => {
              match langType {
                // java case 6: case 7:
                6 | 7 => return true,
                // java default:
                _ => return false,
              }
            }
          }
        }
      }
    }

    // java public boolean isBreakableIfAtleastOnChar()
    pub fn isBreakableIfAtleastOnChar(&self) -> bool {
      // java Graphs graphs = this.mGraphs;
      let mut graphs: Graphs = self.getGraphs();
      // java return !graphs.isAutoNum() && !graphs.isFootEndNote();
      return !graphs.isAutoNum() && !graphs.isFootEndNote();
    }

    // java public boolean shouldBeFollowed()
    pub fn shouldBeFollowed(&self) -> bool {
      // java if (this.isInvalid())
      if self.isInvalid() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Graphs graphs = this.mGraphs;
        let mut graphs: Graphs = self.getGraphs();
        // java return graphs.isAutoNum();
        return graphs.isAutoNum();
      }
    }

    // java private void setInvalidGraphsAndIndex()
    pub fn setInvalidGraphsAndIndex(&mut self) {
      // java this.mGraphs = null;
      self.mGraphs = None;
      // java this.mGraphIndex = Integer.MIN_VALUE;
      self.mGraphIndex = i32::MIN;
    }

    // java public boolean isInvalid()
    pub fn isInvalid(&self) -> bool {
      // java Graphs graphs = this.mGraphs;
      let mut graphs: Graphs = self.getGraphs();
      // java int graphIdx = this.mGraphIndex;
      let mut graphIdx: i32 = self.mGraphIndex;
      // java return null == graphs || Integer.MIN_VALUE == graphIdx;
      return self.mGraphs.is_none() || i32::MIN == graphIdx;
    }

    // java public boolean nextToIndex(int destIndex, KPList<Graphs> graphList)
    pub fn nextToIndex(&mut self, destIndex: i32, graphList: &KPList<Graphs>) -> bool {
      // java if (destIndex <= this.mGraphIndex)
      if destIndex <= self.mGraphIndex {
        // java return false;
        return false;
      }
      // java else
      else {
        // java do (destIndex > this.mGraphIndex)
        loop {
            // java boolean hasNext = this.next(graphList);
            let mut hasNext: bool = self.next(graphList);
            // java if (!hasNext)
            if !hasNext {
              // java return false;
              return false;
            }
            if destIndex <= self.mGraphIndex {
              break;
            }
        }
        // java return destIndex == this.mGraphIndex;
        return destIndex == self.mGraphIndex;
      }
    }

    // java public boolean prev(boolean needToSkipSpaces, KPList<Graphs> graphList, SubLineEnv subLineEnv)
    pub fn prev(&mut self, needToSkipSpaces: bool, graphList: &KPList<Graphs>, subLineEnv: &SubLineEnv) -> bool {
      // java if (this.isInvalid()) return false;
      if self.isInvalid() { return false; }
      // java Graphs curGraphs = this.mGraphs; int curGraphIdx = this.mGraphIndex;
      let cur = self.mGraphs.as_ref().unwrap().clone();
      let same_graph = |a: &Graphs, b: &Graphs| a.graphIndex == b.graphIndex && a.graphLength == b.graphLength;
      let graph_before = |graph: &Graphs| -> Option<Graphs> {
        let index = graphList.nodes.iter().position(|node| same_graph(node, graph))?;
        Some((**graphList.nodes.get(index.checked_sub(1)?)?).clone())
      };
      // Java first walks inside a multi-character Graph without consulting
      // graphList. A stand-alone Graph position has the same behavior.
      let mut previous = if self.mGraphIndex == cur.graphIndex {
        if subLineEnv.firstGraphs.as_ref().is_some_and(|first| same_graph(first, &cur)) {
          None
        } else {
          graph_before(&cur).map(|graph| {
            let index = graph.getEndGraphIndex();
            (graph, index)
          })
        }
      } else {
        Some((cur, self.mGraphIndex - 1))
      };
      // java if (found && prevGraphs == prevLimitGraphs && prevGraphIdx == prevLimitGraphIdx)
      let at_limit = |graph: &Graphs, index: i32| self.mPrevLimitGraphs.as_ref()
        .is_some_and(|limit| same_graph(limit, graph) && index == self.mPrevLimitGraphIndex);
      if previous.as_ref().is_some_and(|(graph, index)| at_limit(graph, *index)) {
        previous = None;
      }
      // java while (found && needToSkipSpaces && TypoRun.isSpace(prevGraphs.runType))
      while needToSkipSpaces && previous.as_ref().is_some_and(|(graph, _)| TypoRun::isSpace(graph.runType)) {
        let (graph, _) = previous.take().unwrap();
        let at_first = subLineEnv.firstGraphs.as_ref().is_some_and(|first| same_graph(first, &graph));
        let at_prev_limit_graph = self.mPrevLimitGraphs.as_ref().is_some_and(|limit| same_graph(limit, &graph));
        if at_first || at_prev_limit_graph { break; }
        previous = graph_before(&graph).map(|before| {
          let index = before.getEndGraphIndex();
          (before, index)
        });
        if previous.as_ref().is_some_and(|(before, index)| at_limit(before, *index)) {
          previous = None;
        }
      }
      // java if (!found) { this.setInvalidGraphsAndIndex(); return false; }
      if let Some((graph, index)) = previous {
        // java this.setGraphsAndIndex(prevGraphs, prevGraphIdx);
        self.setGraphsAndIndex(graph, index);
        true
      } else {
        self.setInvalidGraphsAndIndex();
        false
      }
    }

    // java public boolean next(KPList<Graphs> graphList)
    pub fn next(&mut self, graphList: &KPList<Graphs>) -> bool {
      // java if (this.isInvalid()) return false;
      if self.isInvalid() { return false; }
      // java Graphs curGraphs = this.mGraphs; int curGraphIdx = this.mGraphIndex;
      let cur = self.mGraphs.as_ref().unwrap();
      // java if (curGraphIdx < curGraphs.getEndGraphIndex())
      if self.mGraphIndex < cur.getEndGraphIndex() {
        self.mGraphIndex += 1;
        return true;
      }
      // java Graphs next = (Graphs)graphList.getNext(curGraphs);
      let current_node = graphList.nodes.iter().position(|n| n.graphIndex == cur.graphIndex && n.graphLength == cur.graphLength);
      if let Some(next) = current_node.and_then(|i| graphList.nodes.get(i + 1)) {
        // java this.setGraphsAndIndex(nextGraphs, nextGraphIdx);
        self.setGraphsAndIndex((**next).clone(), next.graphIndex);
        true
      } else {
        // java this.setInvalidGraphsAndIndex(); return false;
        self.setInvalidGraphsAndIndex();
        false
      }
    }

    // java public boolean hasNoBreakHyphen()
    pub fn hasNoBreakHyphen(&self) -> bool {
      // java if (this.isInvalid())
      if self.isInvalid() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return this.mGraphs.graphLength == 1 && HyphenFlag.hasNoBreakHyphen(this.mGraphs.mHyphenFlag);
        return self.mGraphs.as_ref().map(|g| g.graphLength == 1 && (g.mHyphenFlag & (1 << 18)) != 0).unwrap_or(false);
      }
    }

}
