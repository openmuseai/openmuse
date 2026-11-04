// Java: import cn.wps.core.runtime.Assert
use util::runtime::Assert::Assert;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.list.autonum.txtnum.NumInfo
use crate::cn::wps::moffice::writer::core::list::autonum::txtnum::NumInfo::NumInfo;
// Java: import cn.wps.moffice.writer.function.card.data.CardInfo
use crate::cn::wps::moffice::writer::function::card::data::CardInfo::CardInfo;
// Java: import cn.wps.moffice.writer.function.card.data.ChoiceQuestion
use crate::cn::wps::moffice::writer::function::card::data::ChoiceQuestion::ChoiceQuestion;
// Java: import cn.wps.moffice.writer.function.card.data.TitleCard
use crate::cn::wps::moffice::writer::function::card::data::TitleCard::TitleCard;
// Java: import cn.wps.moffice.writer.function.card.util.CollectCardInfo
use crate::cn::wps::moffice::writer::function::card::util::CollectCardInfo::CollectCardInfo;
// Java: import cn.wps.util.LazzyPattern
use crate::cn::wps::util::LazzyPattern::LazzyPattern;
// Java: import java.util.ArrayList

// java public static final String TITLE = "是什么|有哪些|作文";
pub(crate) const TITLE: i32 = "是什么|有哪些|作文";
// java public static final LazzyPattern LEVEL_0_TITLE = new LazzyPattern("是什么|有哪些|作文");
pub(crate) static LEVEL_0_TITLE: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| LazzyPattern::new("是什么|有哪些|作文"));

// java public class CardPaper
#[derive(Default)]
pub struct CardPaper {
    // java Document document;
    pub(crate) document: Option<DocumentHolder>,
    // java ArrayList<CardInfo> cardInfos;
    pub(crate) cardInfos: Vec<Option<CardInfoHolder>>,
}

// java public class CardPaper
impl CardPaper {
    // java public  CardPaper(Document document)
    pub fn CardPaper(document: &dyn Document) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.document = document;
      Self { document: document, cardInfos: Default::default() }
      */
    }

    // java public CardInfo seekCardInfo(int cp)
    pub fn seekCardInfo(&self, cp: i32) -> Option<CardInfoHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.cardInfos == null)
      if false {
        // java return null;
        return None;
      }
      // java else
      else {
        // java for(CardInfo it : this.cardInfos)
        for it in self.cardInfos.iter() {
          // java if (it.contains(cp))
          if it.contains(cp) {
            // java return it;
            return Some(it);
          }
        }
        // java return null;
        return None;
      }
      */
    }

    // java public boolean collect()
    pub fn collect(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.cardInfos)
      if false {
        // java ArrayList<CardInfo> cardInfos = new ArrayList();
        let mut cardInfos: Vec<CardInfo> = { let __x: Vec<CardInfo> = todo!("no align strategy matched"); __x };
        // java CollectCardInfo collector = new CollectCardInfo(this.document, 0, this.document.getLength());
        let mut collector: CollectCardInfo = CollectCardInfo::new(self.document, 0, self.document.getLength());
        // java collector.collectCardInfo(cardInfos, 0);
        collector.collectCardInfo(cardInfos, 0);
        // java boolean isChineseNfc = NumInfo.isChineseCounting(collector.getNfc());
        let mut isChineseNfc: bool = NumInfo::isChineseCounting(collector.getNfc());
        let mut i: i32 = 0;
        // java for(int i = 0; i < cardInfos.size(); ++i)
        while i < (cardInfos.len() as i32) {
          // java CardInfo cardInfo = (CardInfo)cardInfos.get(i);
          let mut cardInfo: CardInfo = (cardInfos.get(i)).downcast_ref::<CardInfo>().unwrap();
          // java ChoiceQuestion choiceQuestion = ChoiceQuestion.identify(cardInfo, isChineseNfc);
          let mut choiceQuestion: ChoiceQuestion = ChoiceQuestion::identify__cardinfo_boolean(cardInfo, isChineseNfc);
          // java if (null != choiceQuestion)
          if true {
            // java if (choiceQuestion.size() == 0)
            if (choiceQuestion.len() as i32) == 0 {
              // java collector.init(this.document, cardInfo.getTitleEnd(), cardInfo.getMaxEnd());
              collector.init(self.document, cardInfo.getTitleEnd(), cardInfo.getMaxEnd());
              // java choiceQuestion.collect(collector);
              choiceQuestion.collect(collector);
            }
            // java this.addCardInfo(choiceQuestion);
            self.addCardInfo({ let __x: Option<CardInfoHolder> = todo!("no align strategy matched"); __x });
          }
          // java else if (cardInfo.getTitleEnd() < cardInfo.getMaxEnd())
          else if cardInfo.getTitleEnd() < cardInfo.getMaxEnd() {
            // java if (!isChineseNfc && (cardInfo.getTitleRange().getLength() > 50 || this.isUiCardInfo(this.get(this.size() - 1))))
            if !isChineseNfc && cardInfo.getTitleRange().getLength() > 50 || self.isUiCardInfo(self.get(self.size() - 1)) {
              // java cardInfo.ignoreTitle();
              cardInfo.ignoreTitle();
            }
            // java this.addCardInfo(cardInfo);
            self.addCardInfo({ let __x: Option<CardInfoHolder> = todo!("no align strategy matched"); __x });
          }
          // java else if (identifyTitle(cardInfo))
          else if Self::identifyTitle({ let __x: Option<CardInfoHolder> = todo!("no align strategy matched"); __x }) {
            // java this.addCardInfo(cardInfo);
            self.addCardInfo({ let __x: Option<CardInfoHolder> = todo!("no align strategy matched"); __x });
          }
          // java else if (!isChineseNfc && this.isUiCardInfo(this.get(this.size() - 1)))
          else if !isChineseNfc && self.isUiCardInfo(self.get(self.size() - 1)) {
            // java cardInfo.ignoreTitle();
            cardInfo.ignoreTitle();
            // java this.addCardInfo(cardInfo);
            self.addCardInfo({ let __x: Option<CardInfoHolder> = todo!("no align strategy matched"); __x });
          }
        }
      }
      // java boolean result = null != this.cardInfos && !this.cardInfos.isEmpty();
      let mut result: bool = true && !self.cardInfos.is_empty();
      // java if (result)
      if result {
        // java int start = 0;
        let mut start: i32 = 0;
        // java int end = ((CardInfo)this.cardInfos.get(0)).getStart();
        let mut end: i32 = (self.cardInfos.get(0)).downcast_ref::<CardInfo>().unwrap().getStart();
        // java if (start < end)
        if start < end {
          // java this.cardInfos.add(0, new TitleCard(this.document, start, end, 0));
          self.cardInfos.add(0, TitleCard::new(self.document, start, end, 0));
        }
      }
      // java return result;
      return result;
      */
    }

    // java private boolean isUiCardInfo(CardInfo cardInfo)
    pub fn isUiCardInfo(&self, cardInfo: Option<CardInfoHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != cardInfo && cardInfo.getLevel() == 0)
      if let Some(cardInfo) = cardInfo {
        if cardInfo.getLevel() == 0 {
          // java return cardInfo.getTitleRange().getLength() == 0;
          return cardInfo.getTitleRange().getLength() == 0;
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

    // java public static boolean isPaper(Document document)
    pub fn isPaper(document: Option<DocumentHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java CollectCardInfo collector = new CollectCardInfo(document, 0, document.getLength());
      let mut collector: CollectCardInfo = CollectCardInfo::new(document, 0, document.getLength());
      // java ArrayList<CardInfo> cardInfos = new ArrayList();
      let mut cardInfos: Vec<CardInfo> = { let __x: Vec<CardInfo> = todo!("no align strategy matched"); __x };
      // java collector.collectCardInfo(cardInfos, 0);
      collector.collectCardInfo(cardInfos, 0);
      // java return null != cardInfos && !cardInfos.isEmpty();
      return true && !cardInfos.is_empty();
      */
    }

    // java public void addCardInfo(CardInfo cardInfo)
    pub fn addCardInfo(&self, cardInfo: Option<CardInfoHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.cardInfos)
      if false {
        // java this.cardInfos = new ArrayList();
        self.cardInfos = Vec::new();
      }
      // java this.cardInfos.add(cardInfo);
      self.cardInfos.add(cardInfo);
      */
    }

    // java public void test()
    pub fn test(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Assert.assertNotNull(this.cardInfos);
      Assert::assertNotNull(self.cardInfos);
      // java if (null != this.cardInfos)
      if true {
        // java for(CardInfo cardInfo : this.cardInfos)
        for cardInfo in self.cardInfos.iter() {
          // java cardInfo.highlight();
          cardInfo.highlight();
        }
      }
      */
    }

    // java public static boolean identifyTitle(CardInfo cardInfo)
    pub fn identifyTitle(cardInfo: Option<CardInfoHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String txt = cardInfo.getTitleRange().getText();
      let mut txt: String = cardInfo.getTitleRange().getText();
      // java return LEVEL_0_TITLE.matcher(txt).find();
      return LEVEL_0_TITLE::matcher(txt).find();
      */
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return null != this.cardInfos ? this.cardInfos.size() : 0;
      return (if true { (self.cardInfos.len() as i32) } else { 0 });
      */
    }

    // java public CardInfo get(int index)
    pub fn get(&self, index: i32) -> Option<CardInfoHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return null != this.cardInfos && index > 0 && index < this.cardInfos.size() ? (CardInfo)this.cardInfos.get(index) : null;
      return Some((if true && index > 0 && index < (self.cardInfos.len() as i32) { Some((self.cardInfos.get(index)).downcast_ref::<CardInfo>().unwrap()) } else { None }));
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != this.cardInfos)
      if true {
        // java for(CardInfo it : this.cardInfos)
        for it in self.cardInfos.iter() {
          // java it.dispose();
          it.dispose();
        }
        // java this.cardInfos.clear();
        self.cardInfos.clear();
      }
      // java this.cardInfos = null;
      self.cardInfos = None;
      */
    }

}
