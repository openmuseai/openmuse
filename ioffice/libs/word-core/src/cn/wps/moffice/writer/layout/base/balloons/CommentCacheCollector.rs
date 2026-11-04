// Java: import cn.wps.collections.KList
use util::collections::KList::KList;
// Java: import cn.wps.core.runtime.Assert
use util::runtime::Assert::Assert;
// Java: import cn.wps.kfc.api.list.IIntList
use util::kfc::api::list::IIntList::IIntList;
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
// Java: import cn.wps.moffice.writer.cache.LocateTool
use crate::cn::wps::moffice::writer::cache::LocateTool::LocateTool;
// Java: import cn.wps.moffice.writer.cache.TypoBalloonItem
use crate::cn::wps::moffice::writer::cache::TypoBalloonItem::TypoBalloonItem;
// Java: import cn.wps.moffice.writer.cache.TypoComments
use crate::cn::wps::moffice::writer::cache::TypoComments::TypoComments;
// Java: import cn.wps.moffice.writer.cache.TypoDocument
use crate::cn::wps::moffice::writer::cache::TypoDocument::TypoDocument;
// Java: import cn.wps.moffice.writer.cache.TypoExtendRanges
use crate::cn::wps::moffice::writer::cache::TypoExtendRanges::TypoExtendRanges;
// Java: import cn.wps.moffice.writer.cache.TypoLayoutPage
use crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage;
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
// Java: import cn.wps.moffice.writer.cache.TypoObjBase
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
// Java: import cn.wps.moffice.writer.cache.TypoPage
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoPara
use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTextLine
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.BookmarkNode
use crate::cn::wps::moffice::writer::data::BookmarkNode::BookmarkNode;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::PLC;
// Java: import cn.wps.moffice.writer.data.PLCBookmarkEnd
use crate::cn::wps::moffice::writer::data::PLCBookmarkEnd::PLCBookmarkEnd;
// Java: import cn.wps.moffice.writer.data.PLCBookmarkStart
use crate::cn::wps::moffice::writer::data::PLCBookmarkStart::PLCBookmarkStart;
// Java: import cn.wps.moffice.writer.data.PLCCommentRef
use crate::cn::wps::moffice::writer::data::PLCCommentRef::PLCCommentRef;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;
// Java: import java.util.ArrayList
// Java: import java.util.Collections
// Java: import java.util.Comparator
use stubs::java::util::Comparator::Comparator;
// Java: import java.util.List
// Java: import cn.wps.moffice.writer.layout.base.balloons.MergedSpans
use crate::cn::wps::moffice::writer::layout::base::balloons::MergedSpans::MergedSpans;
// Java: import cn.wps.moffice.writer.layout.base.balloons.BalloonCacheLayouter
use crate::cn::wps::moffice::writer::layout::base::balloons::BalloonCacheLayouter::BalloonCacheLayouter;
// Java: import cn.wps.io.dom.Node
use io_wps_io::cn::wps::io::dom::Node::Node;

// java protected static final Comparator<PLCCommentRef.CommentRefNode> CMP_ASC_REF_NODE = new Comparator<PLCCommentRef.CommentRefNode>() { public int compare(PLCCommentRef.CommentRefNode o1, PLCCommentRef.CommentRefNode o2) { return o1.getCP() - o2.getCP(); } };
pub(crate) static CMP_ASC_REF_NODE: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| Comparator::new());

// java public class CommentCacheCollector
#[derive(Default)]
pub struct CommentCacheCollector {
    // java private boolean mSkipSubPageComments = false;
    mSkipSubPageComments: bool,
    // java private boolean mIsShowComment = true;
    mIsShowComment: bool,
    // java private WIntArrayList mCommentRefCPs = new WIntArrayList();
    mCommentRefCPs: Option<WIntArrayListHolder>,
    // java protected MergedSpans mPageSpans = new MergedSpans();
    pub(crate) mPageSpans: Option<MergedSpansHolder>,
    // java protected KList<CommentCacheItem> mCacheItems = new KList();
    pub(crate) mCacheItems: Vec<Option<CommentCacheItemHolder>>,
    // java protected KList<CommentCacheItem> mEndCacheItems = new KList();
    pub(crate) mEndCacheItems: Vec<Option<CommentCacheItemHolder>>,
    // java protected List<PLCCommentRef.CommentRefNode> mTmpList = new ArrayList();
    pub(crate) mTmpList: Vec<Option<PLCCommentRef_CommentRefNodeHolder>>,
    // java protected BalloonCacheLayouter mCacheLayouter = null;
    pub(crate) mCacheLayouter: Option<BalloonCacheLayouterHolder>,
    // java protected Pools mPools = null;
    pub(crate) mPools: Option<PoolsHolder>,
    // java protected DocumentEnv mDocEnv;
    pub(crate) mDocEnv: Option<DocumentEnvHolder>,
    // java protected MainState mMainState;
    pub(crate) mMainState: Option<MainStateHolder>,
    // java protected TypoComments mTypoComments;
    pub(crate) mTypoComments: Option<TypoCommentsHolder>,
    // java private WIntArrayList tmpLines = new WIntArrayList();
    tmpLines: Option<WIntArrayListHolder>,
    // java private List<CommentCacheItem> tmpItems = new ArrayList();
    tmpItems: Vec<Option<CommentCacheItemHolder>>,
    // java private List<Boolean> tmpIsCommentEnd = new ArrayList();
    tmpIsCommentEnd: Vec<Option<BooleanHolder>>,
}

// java public static class CommentCacheItem extends KList.RecycableNode
#[derive(Default)]
pub struct CommentCacheItem {
    // java KList.RecycableNode super_
    pub(crate) super_: RecycableNode,
    // java public boolean startNodeCollected;
    pub(crate) startNodeCollected: bool,
    // java public boolean endNodeCollected;
    pub(crate) endNodeCollected: bool,
    // java public PLCBookmarkStart.BookmarkStartNode commentStartNode;
    pub(crate) commentStartNode: Option<PLCBookmarkStart_BookmarkStartNodeHolder>,
}

// java public static class Alloc extends KList.RecycableNodeAlloc<CommentCacheItem>
#[derive(Default)]
pub struct Alloc {
    // java KList.RecycableNodeAlloc<CommentCacheItem> super_
    pub(crate) super_: RecycableNodeAlloc<CommentCacheItem>,
}

// java public class CommentCacheCollector
impl CommentCacheCollector {
    // java public  CommentCacheCollector(DocumentEnv documentEnv, MainState mainState, Pools pools)
    pub fn CommentCacheCollector(documentEnv: &DocumentEnv, mainState: &MainState, pools: &Pools) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainState = mainState;
      Self { mSkipSubPageComments: Default::default(), mIsShowComment: Default::default(), mCommentRefCPs: Default::default(), mPageSpans: Default::default(), mCacheItems: Default::default(), mEndCacheItems: Default::default(), mTmpList: Default::default(), mCacheLayouter: Default::default(), mPools: pools, mDocEnv: documentEnv, mMainState: mainState, mTypoComments: TypoComments::new(), tmpLines: Default::default(), tmpItems: Default::default(), tmpIsCommentEnd: Default::default() }
      // java this.mPools = pools;
      // java this.mDocEnv = documentEnv;
      // java this.mTypoComments = new TypoComments();
      */
    }

    // java private void initPageSpans(TypoLayoutPage layoutPage, MergedSpans spans)
    pub fn initPageSpans(&mut self, layoutPage: Option<TypoLayoutPageHolder>, spans: Option<MergedSpansHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Document doc = layoutPage.getDocument();
      let mut doc: Box<dyn Document> = layoutPage.getDocument();
      // java PLCBookmarkStart commentStarts = doc.getCommentStarts();
      let mut commentStarts: PLCBookmarkStart = doc.getCommentStarts();
      // java if (!commentStarts.isEmpty())
      if !commentStarts.is_empty() {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java int minCP = Integer.MAX_VALUE;
        let mut minCP: i32 = i32::MAX;
        // java int maxCP = Integer.MIN_VALUE;
        let mut maxCP: i32 = i32::MIN;
        // java int pageStart = layoutPage.getStartCP();
        let mut pageStart: i32 = layoutPage.getStartCP();
        // java int pageEnd = layoutPage.getEndCP();
        let mut pageEnd: i32 = layoutPage.getEndCP();
        // java minCP = Math.min(minCP, pageStart);
        minCP = (minCP).min(pageStart);
        // java maxCP = Math.max(maxCP, pageEnd);
        maxCP = (maxCP).max(pageEnd);
        // java spans.add(pageStart, pageEnd);
        spans.add(pageStart, pageEnd);
        // java int pExtendRanges = layoutPage.getExtendRanges();
        let mut pExtendRanges: i32 = layoutPage.getExtendRanges();
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = TypoExtendRanges::size(pExtendRanges, snapshot);
        // java for(int size = TypoExtendRanges.size(pExtendRanges, snapshot); i < size; ++i)
        while i < size {
          // java int refCP = snapshot.getCPRef(TypoExtendRanges.getRef(i, pExtendRanges, snapshot));
          let mut refCP: i32 = snapshot.getCPRef(TypoExtendRanges::getRef(i, pExtendRanges, snapshot));
          // java int startCP = refCP + TypoExtendRanges.getStartOffset(i, pExtendRanges, snapshot);
          let mut startCP: i32 = refCP + TypoExtendRanges::getStartOffset(i, pExtendRanges, snapshot);
          // java int endCP = refCP + TypoExtendRanges.getEndOffset(i, pExtendRanges, snapshot);
          let mut endCP: i32 = refCP + TypoExtendRanges::getEndOffset(i, pExtendRanges, snapshot);
          // java minCP = Math.min(minCP, startCP);
          minCP = (minCP).min(startCP);
          // java maxCP = Math.max(maxCP, endCP);
          maxCP = (maxCP).max(endCP);
          // java spans.add(startCP, endCP);
          spans.add(startCP, endCP);
        }
        // java i = commentStarts.begin().getCP();
        i = commentStarts.begin().getCP();
        // java int commentMaxEnd = doc.getCommentEnds().last().getCP();
        let mut commentMaxEnd: i32 = doc.getCommentEnds().last().getCP();
        // java if (maxCP > i && minCP <= commentMaxEnd)
        if maxCP > i && minCP <= commentMaxEnd {
          // java spans.merge();
          spans.merge();
        }
        // java else
        else {
          // java spans.reset();
          spans.reset();
        }
      }
      */
    }

    // java public void collect(boolean isShowComment, boolean skipSubPageComments, BalloonCacheLayouter balloonCacheLayouter, boolean ignoreBalloonCache, boolean ignoreCommentCache)
    pub fn collect(&mut self, isShowComment: bool, skipSubPageComments: bool, balloonCacheLayouter: Option<BalloonCacheLayouterHolder>, ignoreBalloonCache: bool, ignoreCommentCache: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSkipSubPageComments = skipSubPageComments;
      self.mSkipSubPageComments = skipSubPageComments;
      // java this.mIsShowComment = isShowComment;
      self.mIsShowComment = isShowComment;
      // java this.mCacheLayouter = balloonCacheLayouter;
      self.mCacheLayouter = balloonCacheLayouter;
      // java this.resetContainers();
      self.resetContainers();
      // java this.initPageSpans(this.mMainState.layoutPage, this.mPageSpans);
      self.initPageSpans(Some(self.mMainState.layoutPage), self.mPageSpans);
      // java if (this.mPageSpans.getSpanCount() > 0)
      if self.mPageSpans.getSpanCount() > 0 {
        // java Document mainDoc = this.mDocEnv.textDocument.getMainDocument();
        let mut mainDoc: Box<dyn Document> = self.mDocEnv.textDocument.getMainDocument();
        // java PLCBookmarkStart commentStarts = mainDoc.getCommentStarts();
        let mut commentStarts: PLCBookmarkStart = mainDoc.getCommentStarts();
        // java this.collectCommentNodes(commentStarts, this.mPageSpans, this.mTmpList, this.mCacheItems);
        self.collectCommentNodes({ let __x: Option<PLCBookmarkStartHolder> = todo!("no align strategy matched"); __x }, self.mPageSpans, self.mTmpList, self.mCacheItems);
        // java Pools pools = this.mPools;
        let mut pools: Pools = { let __x: Pools = todo!("no align strategy matched"); __x };
        // java int layoutPage = this.mMainState.layoutPage.getPointer();
        let mut layoutPage: i32 = self.mMainState.layoutPage.getPointer();
        // java TypoDocument typoDocument = this.mDocEnv.typoDocument;
        let mut typoDocument: TypoDocument = self.mDocEnv.typoDocument;
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java TypoComments comments = this.mTypoComments;
        let mut comments: TypoComments = { let __x: TypoComments = todo!("no align strategy matched"); __x };
        // java comments.init(snapshot);
        comments.init(snapshot);
        // java CommentCacheItem item;
        let mut item: CommentCacheItem = CommentCacheItem::default();
        // java for(!this.mCacheItems.isEmpty() pools.recycleCommentCacheItem(item))
        while !self.mCacheItems.is_empty() {
          // java item = (CommentCacheItem)this.mCacheItems.removeFirst();
          item = (self.mCacheItems.removeFirst()).downcast_ref::<CommentCacheItem>().unwrap();
          // java PLCBookmarkStart.BookmarkStartNode startNode = item.commentStartNode;
          let mut startNode: PLCBookmarkStart::BookmarkStartNode = { let __x: PLCBookmarkStart::BookmarkStartNode = todo!("no align strategy matched"); __x };
          // java PLCBookmarkEnd.BookmarkEndNode endNote = startNode.getEnd();
          let mut endNote: PLCBookmarkEnd::BookmarkEndNode = startNode.getEnd();
          // java PLCCommentRef.CommentRefNode refNode = startNode.getCommentRef();
          let mut refNode: PLCCommentRef::CommentRefNode = startNode.getCommentRef();
          // java int startCP = startNode.getCP();
          let mut startCP: i32 = startNode.getCP();
          // java int endCP = endNote.getCP();
          let mut endCP: i32 = endNote.getCP();
          // java int textLine = 0;
          let mut textLine: i32 = 0;
          // java if (endCP > startCP && this.mPageSpans.getRangeIntersectState(startCP, startCP) == 1)
          if endCP > startCP && self.mPageSpans.getRangeIntersectState(startCP, startCP) == 1 {
            // java textLine = LocateTool.quickFindLineFromLayoutPage(layoutPage, mainDoc, startCP, snapshot);
            textLine = LocateTool::quickFindLineFromLayoutPage(layoutPage, mainDoc, startCP, snapshot);
            // java if (TypoMemoryObj.getType(textLine, snapshot) == 3)
            if TypoMemoryObj::getType__int_typosnapshot(textLine, snapshot) == 3 {
              // java int graphIndex = TypoPara.getGraphIndexFromCP(startCP, true, false, TypoTextLine.getPara(textLine, snapshot), snapshot);
              let mut graphIndex: i32 = TypoPara::getGraphIndexFromCP__int_boolean_boolean_int_typosnapshot(startCP, true, false, TypoTextLine::getPara__int_typosnapshot(textLine, snapshot), snapshot);
              // java int adjustLine = textLine;
              let mut adjustLine: i32 = textLine;
              // java while (graphIndex < TypoTextLine.getGraphIndexStart(adjustLine, snapshot))
              while graphIndex < TypoTextLine::getGraphIndexStart__int_typosnapshot(adjustLine, snapshot) {
                // java adjustLine = LocateTool.findPrevLine(snapshot, adjustLine, true);
                adjustLine = LocateTool::findPrevLine(snapshot, adjustLine, true);
                // java if (adjustLine == 0)
                if adjustLine == 0 {
                  // java break;
                  break;
                }
                // java if (startCP >= TypoTextLine.getEndCP(adjustLine, snapshot))
                if startCP >= TypoTextLine::getEndCP__int_typosnapshot(adjustLine, snapshot) {
                  // java adjustLine = 0;
                  adjustLine = 0;
                  // java break;
                  break;
                }
              }
              // java if (adjustLine != 0)
              if adjustLine != 0 {
                // java textLine = adjustLine;
                textLine = adjustLine;
              }
            }
            // java if (0 != textLine)
            if 0 != textLine {
              // java if (TypoMemoryObj.getType(textLine, snapshot) == 3)
              if TypoMemoryObj::getType__int_typosnapshot(textLine, snapshot) == 3 {
                // java TypoTextLine.setHasComment(true, textLine, snapshot);
                TypoTextLine::setHasComment(true, textLine, snapshot);
              }
              // java if (!ignoreBalloonCache)
              if !ignoreBalloonCache {
                // java item.startNodeCollected = true;
                item.startNodeCollected = true;
                // java int balloonItem = this.addCommentCache(mainDoc, 0, textLine, startNode);
                let mut balloonItem: i32 = self.addCommentCache({ let __x: Option<DocumentHolder> = todo!("no align strategy matched"); __x }, 0, textLine, { let __x: Option<BookmarkNodeHolder> = todo!("no align strategy matched"); __x });
                // java if (0 != balloonItem)
                if 0 != balloonItem {
                  // java comments.addComment(startCP, endCP, refNode.getCP(), typoDocument.getAuthorId(refNode.getAuthor()));
                  comments.addComment(startCP, endCP, refNode.getCP(), typoDocument.getAuthorId(refNode.getAuthor()));
                }
              }
            }
          }
          // java int contentEnd = Math.max(startCP, endCP - 1);
          let mut contentEnd: i32 = (startCP).max(endCP - 1);
          // java if (this.mPageSpans.getRangeIntersectState(contentEnd, contentEnd) == 1)
          if self.mPageSpans.getRangeIntersectState(contentEnd, contentEnd) == 1 {
            // java if (0 == textLine || TypoMemoryObj.getType(textLine, snapshot) != 3 || !TypoTextLine.contains(contentEnd, textLine, snapshot))
            if 0 == textLine || TypoMemoryObj::getType__int_typosnapshot(textLine, snapshot) != 3 || !TypoTextLine::contains(contentEnd, textLine, snapshot) {
              // java textLine = LocateTool.quickFindLineFromLayoutPage(layoutPage, mainDoc, contentEnd, snapshot);
              textLine = LocateTool::quickFindLineFromLayoutPage(layoutPage, mainDoc, contentEnd, snapshot);
            }
            // java if (TypoMemoryObj.getType(textLine, snapshot) == 3)
            if TypoMemoryObj::getType__int_typosnapshot(textLine, snapshot) == 3 {
              // java int graphIndex = TypoPara.getGraphIndexFromCP(contentEnd, false, true, TypoTextLine.getPara(textLine, snapshot), snapshot);
              let mut graphIndex: i32 = TypoPara::getGraphIndexFromCP__int_boolean_boolean_int_typosnapshot(contentEnd, false, true, TypoTextLine::getPara__int_typosnapshot(textLine, snapshot), snapshot);
              // java int adjustLine = textLine;
              let mut adjustLine: i32 = textLine;
              // java while (graphIndex > TypoTextLine.getGraphIndexEnd(adjustLine, snapshot))
              while graphIndex > TypoTextLine::getGraphIndexEnd__int_typosnapshot(adjustLine, snapshot) {
                // java adjustLine = LocateTool.findNextLine(snapshot, adjustLine, true, true);
                adjustLine = LocateTool::findNextLine(snapshot, adjustLine, true, true);
                // java if (adjustLine == 0)
                if adjustLine == 0 {
                  // java break;
                  break;
                }
                // java if (contentEnd < TypoTextLine.getStartCP(adjustLine, snapshot))
                if contentEnd < TypoTextLine::getStartCP__int_typosnapshot(adjustLine, snapshot) {
                  // java adjustLine = 0;
                  adjustLine = 0;
                  // java break;
                  break;
                }
              }
              // java if (adjustLine != 0)
              if adjustLine != 0 {
                // java textLine = adjustLine;
                textLine = adjustLine;
              }
            }
            // java if (0 != textLine)
            if 0 != textLine {
              // java if (TypoMemoryObj.getType(textLine, snapshot) == 3)
              if TypoMemoryObj::getType__int_typosnapshot(textLine, snapshot) == 3 {
                // java if (contentEnd < TypoTextLine.getStartCP(textLine, snapshot))
                if contentEnd < TypoTextLine::getStartCP__int_typosnapshot(textLine, snapshot) {
                  // java int preLine = LocateTool.findPrevLine(snapshot, textLine, true);
                  let mut preLine: i32 = LocateTool::findPrevLine(snapshot, textLine, true);
                  // java if (preLine != 0)
                  if preLine != 0 {
                    // java int para = TypoTextLine.getPara(textLine, snapshot);
                    let mut para: i32 = TypoTextLine::getPara__int_typosnapshot(textLine, snapshot);
                    // java int prePara = TypoTextLine.getPara(preLine, snapshot);
                    let mut prePara: i32 = TypoTextLine::getPara__int_typosnapshot(preLine, snapshot);
                    // java if (para == prePara)
                    if para == prePara {
                      // java textLine = preLine;
                      textLine = preLine;
                    }
                  }
                }
                // java TypoTextLine.setHasComment(true, textLine, snapshot);
                TypoTextLine::setHasComment(true, textLine, snapshot);
              }
              // java if (!ignoreBalloonCache)
              if !ignoreBalloonCache {
                // java item.endNodeCollected = true;
                item.endNodeCollected = true;
                // java int balloonItem = this.addCommentCache(mainDoc, 0, textLine, endNote);
                let mut balloonItem: i32 = self.addCommentCache({ let __x: Option<DocumentHolder> = todo!("no align strategy matched"); __x }, 0, textLine, { let __x: Option<BookmarkNodeHolder> = todo!("no align strategy matched"); __x });
                // java if (0 != balloonItem)
                if 0 != balloonItem {
                  // java comments.addComment(startCP, endCP, refNode.getCP(), typoDocument.getAuthorId(refNode.getAuthor()));
                  comments.addComment(startCP, endCP, refNode.getCP(), typoDocument.getAuthorId(refNode.getAuthor()));
                }
              }
            }
          }
          // java if (!ignoreCommentCache && !item.startNodeCollected && !item.endNodeCollected)
          if !ignoreCommentCache && !item.startNodeCollected && !item.endNodeCollected {
            // java comments.addComment(startCP, endCP, refNode.getCP(), typoDocument.getAuthorId(refNode.getAuthor()));
            comments.addComment(startCP, endCP, refNode.getCP(), typoDocument.getAuthorId(refNode.getAuthor()));
          }
        }
        // java if (!ignoreCommentCache && this.mIsShowComment)
        if !ignoreCommentCache && self.mIsShowComment {
          // java this.mMainState.layoutPage.setCommentData(comments.commit());
          self.mMainState.layoutPage.setCommentData(comments.commit());
        }
        // java else
        else {
          // java this.mMainState.layoutPage.setCommentData(0);
          self.mMainState.layoutPage.setCommentData(0);
        }
      }
      */
    }

    // java private int findAndRemoveItem(int targetNodeCP, int refNodeCP, WIntArrayList items, boolean isCommentEnd)
    pub fn findAndRemoveItem(&self, targetNodeCP: i32, refNodeCP: i32, items: Option<WIntArrayListHolder>, isCommentEnd: bool) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (items.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int size = items.size(); i < size; ++i)
      while i < size {
        // java int pItem = items.getQuick(i);
        let mut pItem: i32 = items.getQuick(i);
        // java int refCP = snapshot.getCPRef(TypoBalloonItem.getDataCPRef(pItem, snapshot));
        let mut refCP: i32 = snapshot.getCPRef(TypoBalloonItem::getDataCPRef__int_typosnapshot(pItem, snapshot));
        // java if (refCP == refNodeCP && targetNodeCP == TypoBalloonItem.getStartCP(pItem, snapshot) && isCommentEnd == TypoBalloonItem.isCommentEndOrRevision(pItem, snapshot))
        if refCP == refNodeCP && targetNodeCP == TypoBalloonItem::getStartCP__int_typosnapshot(pItem, snapshot) && isCommentEnd == TypoBalloonItem::isCommentEndOrRevision__int_typosnapshot(pItem, snapshot) {
          // java items.remove(pItem);
          items.remove(pItem);
          // java return pItem;
          return pItem;
        }
      }
      // java return 0;
      return 0;
      */
    }

    // java public void updateItemPos(Document document, int startCP, int endCP, WIntArrayList items, IIntList newLines, BalloonCacheLayouter balloonCacheLayouter)
    pub fn updateItemPos(&self, document: Option<DocumentHolder>, startCP: i32, endCP: i32, items: Option<WIntArrayListHolder>, newLines: Option<IIntListHolder>, balloonCacheLayouter: Option<BalloonCacheLayouterHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (document.getType() == 0)
      if document.getType() == 0 {
        // java PLCBookmarkStart commentStarts = document.getCommentStarts();
        let mut commentStarts: PLCBookmarkStart = document.getCommentStarts();
        // java if (!commentStarts.isEmpty())
        if !commentStarts.is_empty() {
          // java this.resetContainers();
          self.resetContainers();
          // java this.mPageSpans.add(startCP, endCP);
          self.mPageSpans.add(startCP, endCP);
          // java this.mPageSpans.merge();
          self.mPageSpans.merge();
          // java this.collectCommentNodes(commentStarts, this.mPageSpans, this.mTmpList, this.mCacheItems);
          self.collectCommentNodes({ let __x: Option<PLCBookmarkStartHolder> = todo!("no align strategy matched"); __x }, self.mPageSpans, self.mTmpList, self.mCacheItems);
          // java if (!this.mCacheItems.isEmpty())
          if !self.mCacheItems.is_empty() {
            // java TypoLayoutPage layoutPage = this.mMainState.layoutPage;
            let mut layoutPage: TypoLayoutPage = self.mMainState.layoutPage;
            // java TypoSnapshot snapshot = this.mMainState.snapshot;
            let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
            // java boolean needUpdateItem = false;
            let mut needUpdateItem: bool = false;
            // java boolean needRecommit = false;
            let mut needRecommit: bool = false;
            // java int commentData = layoutPage.getCommentData();
            let mut commentData: i32 = layoutPage.getCommentData();
            // java if (commentData == 0)
            if commentData == 0 {
              // java this.mTypoComments.init(snapshot);
              self.mTypoComments.init({ let __x: Option<CommentCacheItemHolder> = todo!("no align strategy matched"); __x });
              // java needRecommit = true;
              needRecommit = true;
            }
            // java else
            else {
              // java this.mTypoComments.assignment(commentData, snapshot);
              self.mTypoComments.assignment(commentData, snapshot);
              // java this.mTypoComments.clearInvalid(this.mDocEnv.textDocument.getMainDocument());
              self.mTypoComments.clearInvalid(self.mDocEnv.textDocument.getMainDocument());
            }
            // java this.mSkipSubPageComments = true;
            self.mSkipSubPageComments = true;
            // java this.mCacheLayouter = balloonCacheLayouter;
            self.mCacheLayouter = balloonCacheLayouter;
            // java int lineIndex = -1;
            let mut lineIndex: i32 = -1;
            // java int curLine = 0;
            let mut curLine: i32 = 0;
            // java int curLineStart = -1;
            let mut curLineStart: i32 = -1;
            // java int curLineEnd = -1;
            let mut curLineEnd: i32 = -1;
            // java while (!this.mCacheItems.isEmpty())
            while !self.mCacheItems.is_empty() {
              // java CommentCacheItem item = (CommentCacheItem)this.mCacheItems.removeFirst();
              let mut item: CommentCacheItem = (self.mCacheItems.removeFirst()).downcast_ref::<CommentCacheItem>().unwrap();
              // java PLCBookmarkStart.BookmarkStartNode startNode = item.commentStartNode;
              let mut startNode: PLCBookmarkStart::BookmarkStartNode = { let __x: PLCBookmarkStart::BookmarkStartNode = todo!("no align strategy matched"); __x };
              // java PLCBookmarkEnd.BookmarkEndNode endNode = startNode.getEnd();
              let mut endNode: PLCBookmarkEnd::BookmarkEndNode = startNode.getEnd();
              // java PLCCommentRef.CommentRefNode refNode = startNode.getCommentRef();
              let mut refNode: PLCCommentRef::CommentRefNode = startNode.getCommentRef();
              // java int refNodeCP = startNode.getCommentRef().getCP();
              let mut refNodeCP: i32 = startNode.getCommentRef().getCP();
              // java int startNodeCP = startNode.getCP();
              let mut startNodeCP: i32 = startNode.getCP();
              // java int endNodeCP = endNode.getCP();
              let mut endNodeCP: i32 = endNode.getCP();
              // java if (endNodeCP > startNodeCP && this.mPageSpans.getRangeIntersectState(startNodeCP, startNodeCP) == 1)
              if endNodeCP > startNodeCP && self.mPageSpans.getRangeIntersectState(startNodeCP, startNodeCP) == 1 {
                // java if (lineIndex < 0)
                if lineIndex < 0 {
                  // java lineIndex = this.findLineInList(startNodeCP, 0, newLines.size(), newLines);
                  lineIndex = self.findLineInList(startNodeCP, 0, (newLines.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32), newLines);
                  // java curLine = newLines.get(lineIndex);
                  curLine = newLines.get(lineIndex);
                  // java curLineStart = TypoTextLine.getStartCP(curLine, snapshot);
                  curLineStart = TypoTextLine::getStartCP__int_typosnapshot(curLine, snapshot);
                  // java curLineEnd = TypoTextLine.getEndCP(curLine, snapshot);
                  curLineEnd = TypoTextLine::getEndCP__int_typosnapshot(curLine, snapshot);
                }
                // java else if (startNodeCP < curLineStart)
                else if startNodeCP < curLineStart {
                  // java lineIndex = this.findLineInList(startNodeCP, 0, lineIndex + 1, newLines);
                  lineIndex = self.findLineInList(startNodeCP, 0, lineIndex + 1, newLines);
                  // java curLine = newLines.get(lineIndex);
                  curLine = newLines.get(lineIndex);
                  // java curLineStart = TypoTextLine.getStartCP(curLine, snapshot);
                  curLineStart = TypoTextLine::getStartCP__int_typosnapshot(curLine, snapshot);
                  // java curLineEnd = TypoTextLine.getEndCP(curLine, snapshot);
                  curLineEnd = TypoTextLine::getEndCP__int_typosnapshot(curLine, snapshot);
                }
                // java else if (startNodeCP >= curLineEnd)
                else if startNodeCP >= curLineEnd {
                  // java lineIndex = this.findLineInList(startNodeCP, lineIndex + 1, newLines.size(), newLines);
                  lineIndex = self.findLineInList(startNodeCP, lineIndex + 1, (newLines.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32), newLines);
                  // java curLine = newLines.get(lineIndex);
                  curLine = newLines.get(lineIndex);
                  // java curLineStart = TypoTextLine.getStartCP(curLine, snapshot);
                  curLineStart = TypoTextLine::getStartCP__int_typosnapshot(curLine, snapshot);
                  // java curLineEnd = TypoTextLine.getEndCP(curLine, snapshot);
                  curLineEnd = TypoTextLine::getEndCP__int_typosnapshot(curLine, snapshot);
                }
                // java if (TypoMemoryObj.getType(curLine, snapshot) == 3)
                if TypoMemoryObj::getType__int_typosnapshot(curLine, snapshot) == 3 {
                  // java TypoTextLine.setHasComment(true, curLine, snapshot);
                  TypoTextLine::setHasComment(true, curLine, snapshot);
                }
                // java int pItem = this.findAndRemoveItem(startNodeCP, refNodeCP, items, false);
                let mut pItem: i32 = self.findAndRemoveItem(startNodeCP, refNodeCP, items, false);
                // java if (0 != pItem)
                if 0 != pItem {
                  // java this.mCacheLayouter.updateItemPos(pItem, curLine, refNode);
                  self.mCacheLayouter.updateItemPos__int_int_object({ let __x: Option<DocumentHolder> = todo!("no align strategy matched"); __x }, curLine, { let __x: i32 = todo!("no align strategy matched"); __x });
                  // java needUpdateItem = true;
                  needUpdateItem = true;
                }
                // java else
                else {
                  // java this.tmpLines.add(curLine);
                  self.tmpLines.add__int(curLine);
                  // java this.tmpItems.add(item);
                  self.tmpItems.add(item);
                  // java this.tmpIsCommentEnd.add(false);
                  self.tmpIsCommentEnd.add(false);
                }
              }
              // java if (this.mPageSpans.getRangeIntersectState(endNodeCP, endNodeCP) == 1)
              if self.mPageSpans.getRangeIntersectState(endNodeCP, endNodeCP) == 1 {
                // java int searchCP = endNodeCP - 1;
                let mut searchCP: i32 = endNodeCP - 1;
                // java if (lineIndex < 0)
                if lineIndex < 0 {
                  // java lineIndex = this.findLineInList(searchCP, 0, newLines.size(), newLines);
                  lineIndex = self.findLineInList(searchCP, 0, (newLines.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32), newLines);
                  // java curLine = newLines.get(lineIndex);
                  curLine = newLines.get(lineIndex);
                  // java curLineStart = TypoTextLine.getStartCP(curLine, snapshot);
                  curLineStart = TypoTextLine::getStartCP__int_typosnapshot(curLine, snapshot);
                  // java curLineEnd = TypoTextLine.getEndCP(curLine, snapshot);
                  curLineEnd = TypoTextLine::getEndCP__int_typosnapshot(curLine, snapshot);
                }
                // java else if (searchCP < curLineStart)
                else if searchCP < curLineStart {
                  // java lineIndex = this.findLineInList(searchCP, 0, lineIndex + 1, newLines);
                  lineIndex = self.findLineInList(searchCP, 0, lineIndex + 1, newLines);
                  // java curLine = newLines.get(lineIndex);
                  curLine = newLines.get(lineIndex);
                  // java curLineStart = TypoTextLine.getStartCP(curLine, snapshot);
                  curLineStart = TypoTextLine::getStartCP__int_typosnapshot(curLine, snapshot);
                  // java curLineEnd = TypoTextLine.getEndCP(curLine, snapshot);
                  curLineEnd = TypoTextLine::getEndCP__int_typosnapshot(curLine, snapshot);
                }
                // java else if (searchCP >= curLineEnd)
                else if searchCP >= curLineEnd {
                  // java lineIndex = this.findLineInList(searchCP, lineIndex + 1, newLines.size(), newLines);
                  lineIndex = self.findLineInList(searchCP, lineIndex + 1, (newLines.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32), newLines);
                  // java curLine = newLines.get(lineIndex);
                  curLine = newLines.get(lineIndex);
                  // java curLineStart = TypoTextLine.getStartCP(curLine, snapshot);
                  curLineStart = TypoTextLine::getStartCP__int_typosnapshot(curLine, snapshot);
                  // java curLineEnd = TypoTextLine.getEndCP(curLine, snapshot);
                  curLineEnd = TypoTextLine::getEndCP__int_typosnapshot(curLine, snapshot);
                }
                // java if (TypoMemoryObj.getType(curLine, snapshot) == 3)
                if TypoMemoryObj::getType__int_typosnapshot(curLine, snapshot) == 3 {
                  // java TypoTextLine.setHasComment(true, curLine, snapshot);
                  TypoTextLine::setHasComment(true, curLine, snapshot);
                }
                // java int pItem = this.findAndRemoveItem(endNodeCP, refNodeCP, items, true);
                let mut pItem: i32 = self.findAndRemoveItem(endNodeCP, refNodeCP, items, true);
                // java if (0 != pItem)
                if 0 != pItem {
                  // java int flag;
                  let mut flag: i32;
                  // java if (startNodeCP == endNodeCP)
                  if startNodeCP == endNodeCP {
                    // java flag = 4;
                    flag = 4;
                  }
                  // java else
                  else {
                    // java flag = -5;
                    flag = -5;
                  }
                  // java TypoBalloonItem.setFlags(flag, 4, pItem, snapshot);
                  TypoBalloonItem::setFlags__int_int_int_typosnapshot(flag, 4, pItem, snapshot);
                  // java this.mCacheLayouter.updateItemPos(pItem, curLine, refNode);
                  self.mCacheLayouter.updateItemPos__int_int_object({ let __x: Option<DocumentHolder> = todo!("no align strategy matched"); __x }, curLine, { let __x: i32 = todo!("no align strategy matched"); __x });
                  // java needUpdateItem = true;
                  needUpdateItem = true;
                }
                // java else
                else {
                  // java this.tmpLines.add(curLine);
                  self.tmpLines.add__int(curLine);
                  // java this.tmpItems.add(item);
                  self.tmpItems.add(item);
                  // java this.tmpIsCommentEnd.add(true);
                  self.tmpIsCommentEnd.add(true);
                }
              }
            }
            // java if (needRecommit)
            if needRecommit {
              // java int finalComments = this.mTypoComments.commit();
              let mut finalComments: i32 = self.mTypoComments.commit();
              // java this.mMainState.layoutPage.setCommentData(finalComments);
              self.mMainState.layoutPage.setCommentData(finalComments);
            }
            // java else if (needUpdateItem)
            else if needUpdateItem {
              // java this.mTypoComments.commitTo(commentData);
              self.mTypoComments.commitTo(commentData);
            }
          }
        }
      }
      */
    }

    // java public void addItems(Document document)
    pub fn addItems(&self, document: Option<DocumentHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.tmpItems.isEmpty() && document.getType() == 0)
      if !self.tmpItems.is_empty() && document.getType() == 0 {
        // java Assert.assertEquals(this.tmpLines.size(), this.tmpItems.size());
        Assert::assertEquals(self.tmpLines.size(), (self.tmpItems.len() as i32));
        // java TypoLayoutPage layoutPage = this.mMainState.layoutPage;
        let mut layoutPage: TypoLayoutPage = self.mMainState.layoutPage;
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java int size = this.tmpItems.size();
        let mut size: i32 = (self.tmpItems.len() as i32);
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java CommentCacheItem item = (CommentCacheItem)this.tmpItems.get(i);
          let mut item: CommentCacheItem = (self.tmpItems.get(i)).downcast_ref::<CommentCacheItem>().unwrap();
          // java int curLine = this.tmpLines.get(i);
          let mut curLine: i32 = self.tmpLines.get(i);
          // java boolean isCommentEnd = (Boolean)this.tmpIsCommentEnd.get(i);
          let mut isCommentEnd: bool = { let __x: bool = todo!("no align strategy matched"); __x };
          // java PLCBookmarkStart.BookmarkStartNode startNode = item.commentStartNode;
          let mut startNode: PLCBookmarkStart::BookmarkStartNode = { let __x: PLCBookmarkStart::BookmarkStartNode = todo!("no align strategy matched"); __x };
          // java PLCBookmarkEnd.BookmarkEndNode endNode = startNode.getEnd();
          let mut endNode: PLCBookmarkEnd::BookmarkEndNode = startNode.getEnd();
          // java int refNodeCP = startNode.getCommentRef().getCP();
          let mut refNodeCP: i32 = startNode.getCommentRef().getCP();
          // java int startNodeCP = startNode.getCP();
          let mut startNodeCP: i32 = startNode.getCP();
          // java int endNodeCP = endNode.getCP();
          let mut endNodeCP: i32 = endNode.getCP();
          // java if (isCommentEnd)
          if isCommentEnd {
            // java int balloonItem = this.addCommentCache(document, 0, curLine, endNode);
            let mut balloonItem: i32 = self.addCommentCache(document, 0, curLine, { let __x: Option<BookmarkNodeHolder> = todo!("no align strategy matched"); __x });
            // java if (0 != balloonItem)
            if 0 != balloonItem {
              // java this.mTypoComments.addComment(endNodeCP, endNodeCP, refNodeCP, TypoBalloonItem.getAuthorID(balloonItem, snapshot));
              self.mTypoComments.addComment(endNodeCP, endNodeCP, refNodeCP, TypoBalloonItem::getAuthorID__int_typosnapshot(balloonItem, snapshot));
            }
          }
          // java else
          else {
            // java int balloonItem = this.addCommentCache(document, 0, curLine, startNode);
            let mut balloonItem: i32 = self.addCommentCache(document, 0, curLine, { let __x: Option<BookmarkNodeHolder> = todo!("no align strategy matched"); __x });
            // java if (0 != balloonItem)
            if 0 != balloonItem {
              // java this.mTypoComments.addComment(startNodeCP, endNodeCP, refNodeCP, TypoBalloonItem.getAuthorID(balloonItem, snapshot));
              self.mTypoComments.addComment(startNodeCP, endNodeCP, refNodeCP, TypoBalloonItem::getAuthorID__int_typosnapshot(balloonItem, snapshot));
            }
          }
        }
        // java int oldCommentData = layoutPage.getCommentData();
        let mut oldCommentData: i32 = layoutPage.getCommentData();
        // java int newCommentsData = this.mTypoComments.commit();
        let mut newCommentsData: i32 = self.mTypoComments.commit();
        // java if (oldCommentData != newCommentsData)
        if oldCommentData != newCommentsData {
          // java layoutPage.setCommentData(newCommentsData);
          layoutPage.setCommentData(newCommentsData);
        }
      }
      */
    }

    // java private int findLineInList(int cp, int min, int max, IIntList newLines)
    pub fn findLineInList(&self, cp: i32, min: i32, max: i32, newLines: Option<IIntListHolder>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java while (min < max)
      while min < max {
        // java int mid = (min + max) / 2;
        let mut mid: i32 = min + max / 2;
        // java int curLine = newLines.get(mid);
        let mut curLine: i32 = newLines.get(mid);
        // java int lineStart = TypoTextLine.getStartCP(curLine, snapshot);
        let mut lineStart: i32 = TypoTextLine::getStartCP__int_typosnapshot(curLine, snapshot);
        // java if (cp < lineStart)
        if cp < lineStart {
          // java max = mid;
          max = mid;
        }
        // java else
        else {
          // java int lineEnd = TypoTextLine.getEndCP(curLine, snapshot);
          let mut lineEnd: i32 = TypoTextLine::getEndCP__int_typosnapshot(curLine, snapshot);
          // java if (cp < lineEnd)
          if cp < lineEnd {
            // java return mid;
            return mid;
          }
          // java min = mid + 1;
          min = mid + 1;
        }
      }
      // java return Math.min(min, newLines.size() - 1);
      return (min).min((newLines.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1);
      */
    }

    // java protected int addCommentCache(Document doc, int page, int line, BookmarkNode node)
    pub fn addCommentCache(&self, doc: Option<DocumentHolder>, page: i32, line: i32, node: Option<BookmarkNodeHolder>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.mIsShowComment)
      if !self.mIsShowComment {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java if (this.mSkipSubPageComments)
        if self.mSkipSubPageComments {
          // java int pageType;
          let mut pageType: i32;
          // java if (0 == page)
          if 0 == page {
            // java TypoSnapshot snapshot = this.mMainState.snapshot;
            let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
            // java pageType = TypoPage.getType(TypoObjBase.getPage(line, this.mMainState.snapshot), snapshot);
            pageType = TypoPage::getType(TypoObjBase::getPage__int_typosnapshot(line, self.mMainState.snapshot), snapshot);
          }
          // java else
          else {
            // java pageType = TypoPage.getType(page, this.mMainState.snapshot);
            pageType = TypoPage::getType(page, self.mMainState.snapshot);
          }
          // java if (2 != pageType)
          if 2 != pageType {
            // java return 0;
            return 0;
          }
        }
        // java if (node instanceof PLCBookmarkStart.BookmarkStartNode)
        if node.downcast_ref::<PLCBookmarkStart::BookmarkStartNode>().is_some() {
          // java return this.mCacheLayouter.addCommentCache((PLCBookmarkStart.BookmarkStartNode)node, line, doc);
          return self.mCacheLayouter.addCommentCache__bookmarkstartnode_int_document({ let __x: Option<DocumentHolder> = todo!("no align strategy matched"); __x }, line, { let __x: i32 = todo!("no align strategy matched"); __x });
        }
        // java else
        else {
          // java return node instanceof PLCBookmarkEnd.BookmarkEndNode ? this.mCacheLayouter.addCommentCache((PLCBookmarkEnd.BookmarkEndNode)node, line, doc) : 0;
          return (if node.downcast_ref::<PLCBookmarkEnd::BookmarkEndNode>().is_some() { self.mCacheLayouter.addCommentCache__bookmarkendnode_int_document({ let __x: Option<DocumentHolder> = todo!("no align strategy matched"); __x }, line, { let __x: i32 = todo!("no align strategy matched"); __x }) } else { 0 });
        }
      }
      */
    }

    // java private static boolean canConvertCommentStart(int startCP, int endCP, int nodeStartCP, int nodeEndCP)
    pub fn canConvertCommentStart(startCP: i32, endCP: i32, nodeStartCP: i32, nodeEndCP: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return startCP <= nodeStartCP && nodeStartCP < endCP;
      return startCP <= nodeStartCP && nodeStartCP < endCP;
      */
    }

    // java private static boolean canConvertCommentEnd(int startCP, int endCP, int nodeStartCP, int nodeEndCP)
    pub fn canConvertCommentEnd(startCP: i32, endCP: i32, nodeStartCP: i32, nodeEndCP: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (nodeStartCP == nodeEndCP)
      if nodeStartCP == nodeEndCP {
        // java return canConvertCommentStart(startCP, endCP, nodeStartCP, nodeEndCP);
        return Self::canConvertCommentStart(startCP, endCP, nodeStartCP, nodeEndCP);
      }
      // java else
      else {
        // java return startCP < nodeEndCP && nodeEndCP <= endCP;
        return startCP < nodeEndCP && nodeEndCP <= endCP;
      }
      */
    }

    // java private void resetContainers()
    pub fn resetContainers(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != this.mPageSpans)
      if let Some(mPageSpans) = self.mPageSpans.as_mut() {
        // java this.mPageSpans.reset();
        mPageSpans.reset();
      }
      // java if (null != this.mTmpList)
      if true {
        // java this.mTmpList.clear();
        self.mTmpList.clear();
      }
      // java if (null != this.mCommentRefCPs)
      if let Some(mCommentRefCPs) = self.mCommentRefCPs.as_mut() {
        // java this.mCommentRefCPs.resetQuick();
        mCommentRefCPs.resetQuick();
      }
      // java if (null != this.tmpItems)
      if true {
        // java this.tmpItems.clear();
        self.tmpItems.clear();
      }
      // java if (null != this.tmpIsCommentEnd)
      if true {
        // java this.tmpIsCommentEnd.clear();
        self.tmpIsCommentEnd.clear();
      }
      // java if (null != this.tmpLines)
      if let Some(tmpLines) = self.tmpLines.as_mut() {
        // java this.tmpLines.clear();
        tmpLines.clear();
      }
      // java if (null != this.mPools && null != this.mCacheItems)
      if let Some(mPools) = self.mPools.as_mut() {
        if true {
          // java this.mPools.recycleCommentCacheItems(this.mCacheItems);
          mPools.recycleCommentCacheItems(self.mCacheItems);
        }
      }
      */
    }

    // java private void collectCommentNodes(PLCBookmarkStart commentStarts, MergedSpans spans, List<PLCCommentRef.CommentRefNode> tmpList, KList<CommentCacheItem> outItemList)
    pub fn collectCommentNodes(&self, commentStarts: Option<PLCBookmarkStartHolder>, spans: Option<MergedSpansHolder>, tmpList: Vec<Option<PLCCommentRef_CommentRefNodeHolder>>, outItemList: Vec<Option<CommentCacheItemHolder>>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java tmpList.clear();
      tmpList.clear();
      // java int spanCount = spans.getSpanCount();
      let mut spanCount: i32 = spans.getSpanCount();
      // java if (spanCount > 0)
      if spanCount > 0 {
        // java int maxPageEnd = spans.getSpanEnd(spanCount - 1);
        let mut maxPageEnd: i32 = spans.getSpanEnd(spanCount - 1);
        // java int minPageStart = spans.getSpanStart(0);
        let mut minPageStart: i32 = spans.getSpanStart(0);
        let mut node: Box<dyn Node> = commentStarts.begin();
        // java for(PLC.Node node = commentStarts.begin(); !commentStarts.isEnd(node) node = node.getNext())
        while !commentStarts.isEnd(node) {
          // java PLCBookmarkStart.BookmarkStartNode startNode = (PLCBookmarkStart.BookmarkStartNode)node;
          let mut startNode: PLCBookmarkStart::BookmarkStartNode = { let __x: PLCBookmarkStart::BookmarkStartNode = todo!("no align strategy matched"); __x };
          // java int commentStartCP = startNode.getCP();
          let mut commentStartCP: i32 = startNode.getCP();
          // java if (commentStartCP >= maxPageEnd)
          if commentStartCP >= maxPageEnd {
            // java break;
            break;
          }
          // java int commentEndCP = startNode.getEnd().getCP();
          let mut commentEndCP: i32 = startNode.getEnd().getCP();
          // java if (commentEndCP == commentStartCP)
          if commentEndCP == commentStartCP {
            // java if (commentEndCP < minPageStart)
            if commentEndCP < minPageStart {
              // java continue;
              continue;
            }
          }
          // java else if (commentEndCP <= minPageStart)
          else if commentEndCP <= minPageStart {
            // java continue;
            continue;
          }
          // java int ret = spans.getRangeIntersectState(commentStartCP, commentEndCP);
          let mut ret: i32 = spans.getRangeIntersectState(commentStartCP, commentEndCP);
          // java if (1 == ret || 2 == ret)
          if 1 == ret || 2 == ret {
            // java tmpList.add(startNode.getCommentRef());
            tmpList.add(startNode.getCommentRef());
          }
        }
        // java Collections.sort(tmpList, CMP_ASC_REF_NODE);
        Collections::sort(tmpList, CMP_ASC_REF_NODE);
        // java Pools pools = this.mPools;
        let mut pools: Pools = { let __x: Pools = todo!("no align strategy matched"); __x };
        // java pools.recycleCommentCacheItems(outItemList);
        pools.recycleCommentCacheItems(outItemList);
        // java int i = 0;
        let mut i: i32 = 0;
        let mut n: i32 = (tmpList.len() as i32);
        // java for(int n = tmpList.size(); i < n; ++i)
        while i < n {
          // java CommentCacheItem item = pools.obtainCommentCacheItem();
          let mut item: CommentCacheItem = pools.obtainCommentCacheItem();
          // java item.commentStartNode = ((PLCCommentRef.CommentRefNode)tmpList.get(i)).getCommentStart();
          item.commentStartNode = (tmpList.get(i)).downcast_ref::<PLCCommentRef::CommentRefNode>().unwrap().getCommentStart();
          // java outItemList.add(item);
          outItemList.add(item);
        }
        // java tmpList.clear();
        tmpList.clear();
      }
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.resetContainers();
      self.resetContainers();
      // java if (null != this.mCacheItems)
      if true {
        // java this.mCacheItems.dispose();
        self.mCacheItems.dispose();
      }
      // java if (null != this.mPageSpans)
      if let Some(mPageSpans) = self.mPageSpans.as_mut() {
        // java this.mPageSpans.dispose();
        mPageSpans.dispose();
      }
      // java this.mCommentRefCPs = null;
      self.mCommentRefCPs = None;
      // java this.mPageSpans = null;
      self.mPageSpans = None;
      // java this.mCacheItems = null;
      self.mCacheItems = None;
      // java this.mTmpList = null;
      self.mTmpList = None;
      // java this.mPools = null;
      self.mPools = None;
      */
    }

    // java public void removeAfterCP(Document document, int cp)
    pub fn removeAfterCP(&self, document: Option<DocumentHolder>, cp: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (document.getType() == 0)
      if document.getType() == 0 {
        // java int oldCommentData = this.mMainState.layoutPage.getCommentData();
        let mut oldCommentData: i32 = self.mMainState.layoutPage.getCommentData();
        // java if (0 != oldCommentData)
        if 0 != oldCommentData {
          // java TypoComments comments = this.mTypoComments;
          let mut comments: TypoComments = { let __x: TypoComments = todo!("no align strategy matched"); __x };
          // java comments.assignment(oldCommentData, this.mMainState.snapshot);
          comments.assignment(oldCommentData, self.mMainState.snapshot);
          // java boolean hasRemove = comments.removeAfterCP(cp);
          let mut hasRemove: bool = comments.removeAfterCP(cp);
          // java if (hasRemove)
          if hasRemove {
            // java this.mMainState.layoutPage.setCommentData(comments.commit());
            self.mMainState.layoutPage.setCommentData(comments.commit());
          }
        }
      }
      */
    }

}

// java public static class CommentCacheItem extends KList.RecycableNode
impl CommentCacheItem {
    // java private  CommentCacheItem()
    pub fn CommentCacheItem(&self) -> Option<CommentCacheItemHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: RecycableNode::new(), startNodeCollected: Default::default(), endNodeCollected: Default::default(), commentStartNode: Default::default() }
      */
    }

    // java public String toString()
    pub fn toString(&self) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int startCP = this.commentStartNode == null ? -1 : this.commentStartNode.getCP();
      let mut startCP: i32 = (if self.commentStartNode.is_none() { -1 } else { self.commentStartNode.getCP() });
      // java int endCP = this.commentStartNode == null ? -1 : this.commentStartNode.getEnd().getCP();
      let mut endCP: i32 = (if self.commentStartNode.is_none() { -1 } else { self.commentStartNode.getEnd().getCP() });
      // java String rightBracket = startCP == endCP ? "]" : ")";
      let mut rightBracket: String = (if startCP == endCP { "]".to_string() } else { ")".to_string() });
      // java return String.format("CP:[%d, %d]  Collected:[%s, %s%s", startCP, endCP, this.startNodeCollected, this.endNodeCollected, rightBracket);
      return Some(String::format("CP:[%d, %d]  Collected:[%s, %s%s", startCP, endCP, self.startNodeCollected, self.endNodeCollected, rightBracket));
      */
    }

}

impl std::ops::Deref for CommentCacheItem {
    type Target = RecycableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CommentCacheItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class Alloc extends KList.RecycableNodeAlloc<CommentCacheItem>
impl Alloc {
    // java public void init(CommentCacheItem node)
    pub fn init(&mut self, node: Option<CommentCacheItemHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.init(node);
      self.super_.init(node);
      // java node.startNodeCollected = false;
      node.startNodeCollected = false;
      // java node.endNodeCollected = false;
      node.endNodeCollected = false;
      // java node.commentStartNode = null;
      node.commentStartNode = None;
      */
    }

    // java public CommentCacheItem alloc()
    pub fn alloc(&self) -> Option<CommentCacheItemHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return new CommentCacheItem();
      return { let __x: Option<CommentCacheItemHolder> = todo!("no align strategy matched"); __x };
      */
    }

}

impl std::ops::Deref for Alloc {
    type Target = RecycableNodeAlloc<CommentCacheItem>;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Alloc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
