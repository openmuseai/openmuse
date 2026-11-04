// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.base.log.Log
use util::base::log::Log::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.core.PropertyTool
use crate::cn::wps::moffice::writer::core::PropertyTool::PropertyTool as WriterPropertyTool;
// Java: import cn.wps.moffice.writer.core.WriterTextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument as WriterTextDocument;
use crate::cn::wps::moffice::writer::core::TextDocument::UULsid;
// Java: import cn.wps.moffice.writer.core.list.ListTemplate
use crate::cn::wps::moffice::writer::core::list::ListTemplate::*;
// Java: import cn.wps.moffice.writer.core.list.gallery.hybridmultilevel.HybridMultileveListGallary
use crate::cn::wps::moffice::writer::core::list::gallery::hybridmultilevel::HybridMultileveListGallary::*;
// Java: import cn.wps.moffice.writer.data.autonum.LSTF
use crate::cn::wps::moffice::writer::data::autonum::LSTF::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstDataTable
use crate::cn::wps::moffice::writer::data::autonum::LstDataTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LvlfData
use crate::cn::wps::moffice::writer::data::autonum::LvlfData::*;
// Java: import cn.wps.util.Pair
use util::util::Pair::*;
// Java: import java.security.SecureRandom
use stubs::java::security::SecureRandom::*;
// Java: import java.util.ArrayList
// Java: import java.util.Collections
// Java: import java.util.Comparator
use stubs::java::util::Comparator::*;
// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import cn.wps.moffice.writer.core.list.impl.KListLevels
use crate::cn::wps::moffice::writer::core::list::r#impl::KListLevels::*;
// Java: import java.lang.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;
// Java: import cn.wps.moffice.writer.core.list.impl.KListLevel
use crate::cn::wps::moffice::writer::core::list::r#impl::KListLevel::*;
// Java: import cn.wps.moffice.writer.core.list.impl.KLists
use crate::cn::wps::moffice::writer::core::list::r#impl::KLists::*;
// Java: import cn.wps.moffice.writer.core.list.impl.KListTemplates
use crate::cn::wps::moffice::writer::core::list::r#impl::KListTemplates::*;
// Java: import cn.wps.moffice.writer.core.list.impl.KList
use crate::cn::wps::moffice::writer::core::list::r#impl::KList::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java private static final String TAG = null;
static TAG: std::sync::LazyLock<Option<String>> =
                    std::sync::LazyLock::new(|| None);

// java public final class KListTemplate implements ListTemplate, Cloneable
#[derive(Default)]
pub struct KListTemplate {
    // java private WriterTextDocument mTextDocument;
    mTextDocument: WriterTextDocument,
    // java private SecureRandom mTplcGenerator;
    mTplcGenerator: SecureRandom,
    // java private LstData mLstData;
    mLstData: Option<LstData>,
    // java private KListLevels mListLevels;
    mListLevels: Option<KListLevels>,
}

// java public final class KListTemplate implements ListTemplate, Cloneable
impl KListTemplate {
    // java   KListTemplate(WriterTextDocument textDocument, LstDataTable lstTable, WriterTextDocument.UULsid uuid, SecureRandom tplcGenerator, int countLevel)
    pub fn new__textdocument_lstdatatable_uulsid_securerandom_int(textDocument: &WriterTextDocument, lstTable: &mut LstDataTable, uuid: &UULsid, tplcGenerator: &SecureRandom, countLevel: i32) -> Self {
      // java Assert.assertNotNull("textDocument should not be null.", textDocument);
      Assert::assertNotNull__string_object("textDocument should not be null.", textDocument);
      // java Assert.assertNotNull("lstTable should not be null.", lstTable);
      Assert::assertNotNull__string_object("lstTable should not be null.", lstTable);
      // java Assert.assertNotNull("uuid should not be null.", uuid);
      Assert::assertNotNull__string_object("uuid should not be null.", uuid);
      // java Assert.assertNotNull("tplcGenerator should not be null.", tplcGenerator);
      Assert::assertNotNull__string_object("tplcGenerator should not be null.", tplcGenerator);
      // java Assert.assertTrue("countLevel >= 1 && countLevel <= 9 should be true.", countLevel >= 1 && countLevel <= 9);
      Assert::assertTrue__string_boolean("countLevel >= 1 && countLevel <= 9 should be true.", countLevel >= 1 && countLevel <= 9);
      // java this.mTextDocument = textDocument;
      let mut lst_data = LstData::new(textDocument);
      let list_levels = KListLevels::new(textDocument, &lst_data, countLevel);
      let mut this = Self { mTextDocument: textDocument.clone(), mTplcGenerator: tplcGenerator.clone(), mLstData: Some(lst_data), mListLevels: Some(list_levels) };
      // java this.mLstData = new LstData(textDocument);
      // java this.mLstData.mLstf = new LSTF();
      this.mLstData.mLstf = LSTF::new();
      // java this.mLstData.mLstf.setLsid(uuid.getNextId());
      this.mLstData.mLstf.setLsid(uuid.getNextId());
      // java lstTable.add(this.mLstData);
      lstTable.add(this.mLstData);
      // java this.mListLevels = new KListLevels(textDocument, this.mLstData, countLevel);
      // java this.mTplcGenerator = tplcGenerator;
      this
    }

    // java   KListTemplate(WriterTextDocument textDocument, SecureRandom tplcGenerator, int countLevel)
    pub fn new__textdocument_securerandom_int(textDocument: &WriterTextDocument, tplcGenerator: &SecureRandom, countLevel: i32) -> Self {
      // java Assert.assertNotNull("textDocument should not be null.", textDocument);
      Assert::assertNotNull__string_object("textDocument should not be null.", textDocument);
      // java Assert.assertNotNull("tplcGenerator should not be null.", tplcGenerator);
      Assert::assertNotNull__string_object("tplcGenerator should not be null.", tplcGenerator);
      // java Assert.assertTrue("countLevel >= 1 && countLevel <= 9 should be true.", countLevel >= 1 && countLevel <= 9);
      Assert::assertTrue__string_boolean("countLevel >= 1 && countLevel <= 9 should be true.", countLevel >= 1 && countLevel <= 9);
      // java this.mTextDocument = textDocument;
      let mut lst_data = LstData::new(textDocument);
      let list_levels = KListLevels::new(textDocument, &lst_data, countLevel);
      let mut this = Self { mTextDocument: textDocument.clone(), mTplcGenerator: tplcGenerator.clone(), mLstData: Some(lst_data), mListLevels: Some(list_levels) };
      // java this.mLstData = new LstData(textDocument);
      // java this.mLstData.mLstf = new LSTF();
      this.mLstData.mLstf = LSTF::new();
      // java this.mLstData.mLstf.setLsid(-1);
      this.mLstData.mLstf.setLsid(-1);
      // java this.mListLevels = new KListLevels(textDocument, this.mLstData, countLevel);
      // java this.mTplcGenerator = tplcGenerator;
      this
    }

    // java   KListTemplate(WriterTextDocument textDocument, LstData lstData, WriterTextDocument.UULsid uuid, SecureRandom tplcGenerator)
    pub fn new__textdocument_lstdata_uulsid_securerandom(textDocument: &WriterTextDocument, lstData: &LstData, uuid: &UULsid, tplcGenerator: &SecureRandom) -> Self {
      // java Assert.assertNotNull("lstData should not be null.", lstData);
      Assert::assertNotNull__string_object("lstData should not be null.", lstData);
      // java Assert.assertNotNull("uuid should not be null.", uuid);
      Assert::assertNotNull__string_object("uuid should not be null.", uuid);
      // java Assert.assertNotNull("tplcGenerator should not be null.", tplcGenerator);
      Assert::assertNotNull__string_object("tplcGenerator should not be null.", tplcGenerator);
      // java this.mTextDocument = textDocument;
      let list_levels = KListLevels::new(lstData.getLvlfDataArray());
      let mut this = Self { mTextDocument: textDocument.clone(), mTplcGenerator: tplcGenerator.clone(), mLstData: Some(lstData.clone()), mListLevels: Some(list_levels) };
      // java this.mLstData = lstData;
      // java this.mListLevels = new KListLevels(this.mLstData.getLvlfDataArray());
      // java this.mTplcGenerator = tplcGenerator;
      this
    }

    // java public KListTemplate clone()
    pub fn clone(&self) -> KListTemplate {
      // java KListTemplate template = (KListTemplate)super.clone();
      let mut template: KListTemplate = (self.super_.clone()).downcast_ref::<KListTemplate>().unwrap();
      // java template.mListLevels = this.mListLevels.clone();
      template.mListLevels = self.mListLevels.clone();
      // java template.mLstData = this.mLstData.clone();
      template.mLstData = self.mLstData.clone();
      // java template.mLstData.setLvlfDataArray(template.mListLevels.getLvlfDatas());
      template.mLstData.setLvlfDataArray(template.mListLevels.getLvlfDatas());
      // java template.mLstData.mLstf.setLsid(-1);
      template.mLstData.mLstf.setLsid(-1);
      // java return template;
      return template;
    }

    // java public ListTemplate.LevelType getLevelType()
    pub fn getLevelType(&self) -> ListTemplateLevelType {
      // java if (this.mLstData.mLstf.isfSimpleList())
      if self.mLstData.mLstf.isfSimpleList() {
        // java return ListTemplate.LevelType.SINGLELEVEL;
        return ListTemplate::LevelType.SINGLELEVEL;
      }
      // java else
      else {
        // java return this.mLstData.mLstf.isfHybrid() ? ListTemplate.LevelType.HYBRIDMULTILEVEL : ListTemplate.LevelType.MULTILEVEL;
        return (if self.mLstData.mLstf.isfHybrid() { ListTemplate::LevelType.HYBRIDMULTILEVEL } else { ListTemplate::LevelType.MULTILEVEL });
      }
    }

    // java public KListLevels getListLevels()
    pub fn getListLevels(&self) -> Option<&KListLevels> {
      // java return this.mListLevels;
      return self.mListLevels.as_ref();
    }

    // java public boolean getOutlineNumbered()
    pub fn getOutlineNumbered(&self) -> bool {
      // java return this.mListLevels.count() > 1;
      return self.mListLevels.count() > 1;
    }

    // java public void setOutlineNumbered(boolean val)
    pub fn setOutlineNumbered(&mut self, val: bool) {
    }

    // java public LstData getLstData()
    pub fn getLstData(&self) -> Option<&LstData> {
      // java return this.mLstData;
      return self.mLstData.as_ref();
    }

    // java public int getLsid()
    pub fn getLsid(&self) -> i32 {
      // java Assert.assertNotNull("mLstData should not be null.", this.mLstData);
      Assert::assertNotNull__string_object("mLstData should not be null.", self.mLstData);
      // java Assert.assertNotNull("mLstData.mLstf should not be null.", this.mLstData.mLstf);
      Assert::assertNotNull__string_object("mLstData.mLstf should not be null.", self.mLstData.mLstf);
      // java return this.mLstData.mLstf.getLsid();
      return self.mLstData.mLstf.getLsid();
    }

    // java public void setLsid(int lsid)
    pub fn setLsid(&mut self, lsid: i32) {
      // java Assert.assertNotNull("mLstData should not be null.", this.mLstData);
      Assert::assertNotNull__string_object("mLstData should not be null.", self.mLstData);
      // java Assert.assertNotNull("mLstData.mLstf should not be null.", this.mLstData.mLstf);
      Assert::assertNotNull__string_object("mLstData.mLstf should not be null.", self.mLstData.mLstf);
      // java this.mLstData.mLstf.setLsid(lsid);
      self.mLstData.mLstf.setLsid(lsid);
    }

    // java public void setLsid(WriterTextDocument.UULsid uuid)
    pub fn setLsid__uulsid(&mut self, uuid: &UULsid) {
      // java Assert.assertNotNull("uuid should not be null.", uuid);
      Assert::assertNotNull__string_object("uuid should not be null.", uuid);
      // java this.mLstData.mLstf.setLsid(uuid.getNextId());
      self.mLstData.mLstf.setLsid(uuid.getNextId());
    }

    // java public short getSti(int level)
    pub fn getSti(&self, level: i32) -> i16 {
      // java Assert.assertTrue("level should be in [1, 9]", level >= 1 && level <= 9);
      Assert::assertTrue__string_boolean("level should be in [1, 9]", level >= 1 && level <= 9);
      // java return this.mLstData.mLstf.getRgistd(level - 1);
      return self.mLstData.mLstf.getRgistd(level - 1);
    }

    // java public void setSti(int level, short sti)
    pub fn setSti(&mut self, level: i32, sti: i16) {
      // java Assert.assertTrue("level should be in [1, 9]", level >= 1 && level <= 9);
      Assert::assertTrue__string_boolean("level should be in [1, 9]", level >= 1 && level <= 9);
      // java this.mLstData.mLstf.setRgistd(level - 1, sti);
      self.mLstData.mLstf.setRgistd(level - 1, sti);
    }

    // java public void clearSti(int level)
    pub fn clearSti(&mut self, level: i32) {
      // java Assert.assertTrue("level should be in [1, 9]", level >= 1 && level <= 9);
      Assert::assertTrue__string_boolean("level should be in [1, 9]", level >= 1 && level <= 9);
      // java this.mLstData.mLstf.initIstd(level - 1);
      self.mLstData.mLstf.initIstd(level - 1);
    }

    // java public void initAllSti()
    pub fn initAllSti(&mut self) {
      // java this.mLstData.mLstf.initRgIstd();
      self.mLstData.mLstf.initRgIstd();
    }

    // java public int getLevelTplc(int level)
    pub fn getLevelTplc(&self, level: i32) -> i32 {
      // java Assert.assertTrue("level should be in [1, 9]", level >= 1 && level <= 9);
      Assert::assertTrue__string_boolean("level should be in [1, 9]", level >= 1 && level <= 9);
      // java int[] rgtplc = this.mLstData.mLstf.getRgtplc();
      let mut rgtplc: Vec<i32> = self.mLstData.mLstf.getRgtplc();
      // java if (null != rgtplc && rgtplc.length > 0)
      if true && (rgtplc.len() as i32) > 0 {
        // java Assert.assertTrue("rgtplc.length should be 9", 9 == rgtplc.length);
        Assert::assertTrue__string_boolean("rgtplc.length should be 9", 9 == (rgtplc.len() as i32));
        // java return rgtplc[level - 1];
        return rgtplc[(level - 1) as usize];
      }
      // java else
      else {
        // java return -1;
        return -1;
      }
    }

    // java public void setLevelTplc(int level, int tplc)
    pub fn setLevelTplc(&mut self, level: i32, tplc: i32) {
      // java Assert.assertTrue("level should be in [1, 9]", level >= 1 && level <= 9);
      Assert::assertTrue__string_boolean("level should be in [1, 9]", level >= 1 && level <= 9);
      // java if (null == this.mLstData.mLstf.getRgtplc() || this.mLstData.mLstf.getRgtplc().length <= 0)
      if self.mLstData.mLstf.getRgtplc().is_none() || (self.mLstData.mLstf.getRgtplc().len() as i32) <= 0 {
        // java this.mLstData.mLstf.initRgTplc();
        self.mLstData.mLstf.initRgTplc();
      }
      // java this.mLstData.mLstf.setRgtplc(level - 1, tplc);
      self.mLstData.mLstf.setRgtplc(level - 1, tplc);
    }

    // java public void initAllTplc()
    pub fn initAllTplc(&mut self) {
      // java this.mLstData.mLstf.initRgTplc();
      self.mLstData.mLstf.initRgTplc();
    }

    // java public int getTplc()
    pub fn getTplc(&self) -> i32 {
      // java return this.mLstData.mLstf.getTplc();
      return self.mLstData.mLstf.getTplc();
    }

    // java public void setTplc(int val)
    pub fn setTplc(&mut self, val: i32) {
      // java this.mLstData.mLstf.setTplc(val);
      self.mLstData.mLstf.setTplc(val);
    }

    // java public boolean getfSimpleList()
    pub fn getfSimpleList(&self) -> bool {
      // java return this.mLstData.mLstf.isfSimpleList();
      return self.mLstData.mLstf.isfSimpleList();
    }

    // java public void setfSimpleList(boolean val)
    pub fn setfSimpleList(&mut self, val: bool) {
      // java this.mLstData.mLstf.setfSimpleList(val);
      self.mLstData.mLstf.setfSimpleList(val);
    }

    // java public boolean getfRestartHdn()
    pub fn getfRestartHdn(&self) -> bool {
      // java return this.mLstData.mLstf.isfRestartHdn();
      return self.mLstData.mLstf.isfRestartHdn();
    }

    // java public void setfRestartHdn(boolean val)
    pub fn setfRestartHdn(&mut self, val: bool) {
      // java this.mLstData.mLstf.setfRestartHdn(val);
      self.mLstData.mLstf.setfRestartHdn(val);
    }

    // java public boolean getfAutoNum()
    pub fn getfAutoNum(&self) -> bool {
      // java return this.mLstData.mLstf.isfAutoNum();
      return self.mLstData.mLstf.isfAutoNum();
    }

    // java public void setfAutoNum(boolean val)
    pub fn setfAutoNum(&mut self, val: bool) {
      // java this.mLstData.mLstf.setfAutoNum(val);
      self.mLstData.mLstf.setfAutoNum(val);
    }

    // java public boolean getfHybrid()
    pub fn getfHybrid(&self) -> bool {
      // java return this.mLstData.mLstf.isfHybrid();
      return self.mLstData.mLstf.isfHybrid();
    }

    // java public void setfHybrid(boolean val)
    pub fn setfHybrid(&mut self, val: bool) {
      // java this.mLstData.mLstf.setfHybrid(val);
      self.mLstData.mLstf.setfHybrid(val);
    }

    // java public byte getGrfhic()
    pub fn getGrfhic(&self) -> i8 {
      // java return this.mLstData.mLstf.getGrfhic();
      return self.mLstData.mLstf.getGrfhic();
    }

    // java public void setGrfhic(byte val)
    pub fn setGrfhic(&mut self, val: i8) {
      // java this.mLstData.mLstf.setGrfhic(val);
      self.mLstData.mLstf.setGrfhic(val);
    }

    // java public int mergeListLevel(KListTemplate template, int level)
    pub fn mergeListLevel(&mut self, template: &KListTemplate, level: i32) -> i32 {
      // java Assert.assertTrue("level >= 1 && level <= 9 should be true.", level >= 1 && level <= 9);
      Assert::assertTrue__string_boolean("level >= 1 && level <= 9 should be true.", level >= 1 && level <= 9);
      // java Assert.assertNotNull("template should not be null.", template);
      Assert::assertNotNull__string_object("template should not be null.", template);
      // java int SRC_LEVEL = 0;
      let mut SRC_LEVEL: i32 = 0;
      // java KListLevel rhs = template.getListLevels().item(0);
      let mut rhs: KListLevel = template.getListLevels().item(0);
      // java Assert.assertNotNull("rhs should not be null.", rhs);
      Assert::assertNotNull__string_object("rhs should not be null.", rhs);
      // java int DST_LEVEL = level - 1;
      let mut DST_LEVEL: i32 = level - 1;
      // java KListLevel lhs = this.mListLevels.item(DST_LEVEL);
      let mut lhs: KListLevel = self.mListLevels.item(DST_LEVEL);
      // java Assert.assertNotNull("lhs should not be null.", lhs);
      Assert::assertNotNull__string_object("lhs should not be null.", lhs);
      // java lhs.setStartAt(rhs.getStartAt());
      lhs.setStartAt(rhs.getStartAt());
      // java lhs.setNfc(rhs.getNfc());
      lhs.setNfc(rhs.getNfc());
      // java lhs.setJc(rhs.getJc());
      lhs.setJc(rhs.getJc());
      // java lhs.setfLegal(rhs.getfLegal());
      lhs.setfLegal(rhs.getfLegal());
      // java lhs.setfNoRestart(rhs.getfNoRestart());
      lhs.setfNoRestart(rhs.getfNoRestart());
      // java lhs.setfIndentSav(rhs.getfIndentSav());
      lhs.setfIndentSav(rhs.getfIndentSav());
      // java lhs.setfConverted(rhs.getfConverted());
      lhs.setfConverted(rhs.getfConverted());
      // java lhs.setfTentative(rhs.getfTentative());
      lhs.setfTentative(rhs.getfTentative());
      // java lhs.setBxchNums((byte[])rhs.getBxchNums().clone());
      lhs.setBxchNums(todo!("cast → Vec<u8>"));
      // java lhs.setIxchFollow(rhs.getIxchFollow());
      lhs.setIxchFollow(rhs.getIxchFollow());
      // java lhs.setDxaIndentSav(rhs.getDxaIndentSav());
      lhs.setDxaIndentSav(rhs.getDxaIndentSav());
      // java lhs.setIRestartAfterLevel(rhs.getIRestartAfterLevel());
      lhs.setIRestartAfterLevel(rhs.getIRestartAfterLevel());
      // java lhs.setGrfhic(rhs.getGrfhic());
      lhs.setGrfhic(rhs.getGrfhic());
      // java String xstStr = rhs.getXstStr();
      let mut xstStr: String = rhs.getXstStr();
      // java lhs.setXstStr(xstStr.replace('\u0000', (char)DST_LEVEL));
      lhs.setXstStr(xstStr.replace('\u{0000}', todo!("cast → char")));
      // java PropertySet rhsChpx = rhs.getChpx();
      let mut rhsChpx: PropertySet = rhs.getChpx();
          // java lhs.setChpx(null != rhsChpx ? rhsChpx.clone() : null);
          lhs.setChpx((if true { rhsChpx.clone() } else { 0 }));
          // java CloneNotSupportedException e
          todo!();
            // java Log.e(TAG, "CloneNotSupportedException", e);
            Log::e__string_string_throwable(TAG, "CloneNotSupportedException", e);
            // java Assert.fail("It should not reach to here.");
            Assert::fail__string("It should not reach to here.");
      // java ListTemplate.LevelType curLevelType = this.getLevelType();
      let mut curLevelType: ListTemplate::LevelType = { let __x: ListTemplate::LevelType = todo!("no align strategy matched"); __x };
      // java if (ListTemplate.LevelType.HYBRIDMULTILEVEL == curLevelType)
      if ListTemplate::LevelType.HYBRIDMULTILEVEL == curLevelType {
        // java this.mLstData.setLevelTplc(DST_LEVEL, template.mLstData.getLevelTplc(0));
        self.mLstData.setLevelTplc(DST_LEVEL, template.mLstData.getLevelTplc(0));
      }
      // java else if (ListTemplate.LevelType.SINGLELEVEL == curLevelType)
      else if ListTemplate::LevelType.SINGLELEVEL == curLevelType {
        // java this.mLstData.setTemplateCode(template.mLstData.getLevelTplc(0));
        self.mLstData.setTemplateCode(template.mLstData.getLevelTplc(0));
      }
      // java this.mLstData.setTemplateCode(this.mTplcGenerator.nextInt());
      self.mLstData.setTemplateCode(self.mTplcGenerator.nextInt());
      // java return level;
      return level;
    }

    // java public int getRightAdjacentLevel(float firstLineAbsPos, boolean hasStyleReference)
    pub fn getRightAdjacentLevel(&self, firstLineAbsPos: f32, hasStyleReference: bool) -> i32 {
      // java return HybridMultileveListGallary.getRightLevel(firstLineAbsPos, hasStyleReference);
      return HybridMultileveListGallary::getRightLevel(firstLineAbsPos, hasStyleReference);
    }

    // java public boolean hasStyleReference()
    pub fn hasStyleReference(&self) -> bool {
      // java int i = 0;
      let mut i: i32 = 0;
      let mut n: i32 = self.mListLevels.count();
      // java for(int n = this.mListLevels.count(); i < n; ++i)
      while i < n {
        // java int sti = this.getSti(i + 1);
        let mut sti: i32 = (self.getSti(i + 1) as i32);
        // java if (4095 != sti)
        if 4095 != sti {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
    }

    // java private boolean isStyleTakenByOther()
    pub fn isStyleTakenByOther(&self) -> bool {
      // java KLists lists = this.mTextDocument.getLists();
      let mut lists: KLists = self.mTextDocument.getLists();
      // java Assert.assertNotNull("lists should not be null.", lists);
      Assert::assertNotNull__string_object("lists should not be null.", lists);
      // java KListTemplates templates = this.mTextDocument.getListTemplates();
      let mut templates: KListTemplates = self.mTextDocument.getListTemplates();
      // java Assert.assertNotNull("templates should not be null.", templates);
      Assert::assertNotNull__string_object("templates should not be null.", templates);
      // java KStyles styles = this.mTextDocument.getStyles();
      let mut styles: KStyles = self.mTextDocument.getStyles();
      // java Assert.assertNotNull("styles should not be null.", styles);
      Assert::assertNotNull__string_object("styles should not be null.", styles);
      // java int myTplc = this.getTplc();
      let mut myTplc: i32 = self.getTplc();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut n: i32 = self.mListLevels.count();
      // java for(int n = this.mListLevels.count(); i < n; ++i)
      while i < n {
        // java int sti = this.getSti(i + 1);
        let mut sti: i32 = (self.getSti(i + 1) as i32);
        // java if (4095 != sti)
        if 4095 != sti {
          // java KStyle style = styles.getStyleById(sti, false);
          let mut style: KStyle = styles.getStyleById(sti, false);
          // java if (null != style)
          if true {
            // java PropertySet props = style.getProperty();
            let mut props: PropertySet = style.getProperty();
            // java if (null != props)
            if true {
              // java Integer numId = (Integer)props.getValue(235);
              let mut numId: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
              // java if (ListUtil.isNumIdExist(numId))
              if ListUtil::isNumIdExist(numId) {
                // java KList curList = lists.find(numId);
                let mut curList: KList = lists.find(numId);
                // java if (null != curList)
                if true {
                  // java KListTemplate template = templates.find(curList.getLsid());
                  let mut template: KListTemplate = templates.find(curList.getLsid());
                  // java if (null != template)
                  if true {
                    // java int tplc = template.getTplc();
                    let mut tplc: i32 = template.getTplc();
                    // java if (tplc != myTplc)
                    if tplc != myTplc {
                      // java return true;
                      return true;
                    }
                  }
                }
              }
            }
          }
        }
      }
      // java return false;
      return false;
    }

    // java private KList clearStyleTakenByMyTplc()
    pub fn clearStyleTakenByMyTplc(&mut self) -> Option<&KList> {
      // java KLists lists = this.mTextDocument.getLists();
      let mut lists: KLists = self.mTextDocument.getLists();
      // java Assert.assertNotNull("lists should not be null.", lists);
      Assert::assertNotNull__string_object("lists should not be null.", lists);
      // java KListTemplates templates = this.mTextDocument.getListTemplates();
      let mut templates: KListTemplates = self.mTextDocument.getListTemplates();
      // java Assert.assertNotNull("templates should not be null.", templates);
      Assert::assertNotNull__string_object("templates should not be null.", templates);
      // java KStyles styles = this.mTextDocument.getStyles();
      let mut styles: KStyles = self.mTextDocument.getStyles();
      // java Assert.assertNotNull("styles should not be null.", styles);
      Assert::assertNotNull__string_object("styles should not be null.", styles);
      // java int myTplc = this.getTplc();
      let mut myTplc: i32 = self.getTplc();
      // java boolean toClearCache = false;
      let mut toClearCache: bool = false;
      // java KList list = null;
      let mut list: KList = Default::default();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut n: i32 = self.mListLevels.count();
      // java for(int n = this.mListLevels.count(); i < n; ++i)
      while i < n {
        // java int sti = this.getSti(i + 1);
        let mut sti: i32 = (self.getSti(i + 1) as i32);
        // java if (4095 != sti)
        if 4095 != sti {
          // java KStyle style = styles.getStyleById(sti, false);
          let mut style: KStyle = styles.getStyleById(sti, false);
          // java if (null != style)
          if true {
            // java PropertySet props = style.getProperty();
            let mut props: PropertySet = style.getProperty();
            // java if (null != props)
            if true {
              // java Integer numId = (Integer)props.getValue(235);
              let mut numId: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
              // java if (ListUtil.isNumIdExist(numId))
              if ListUtil::isNumIdExist(numId) {
                // java KList curList = lists.find(numId);
                let mut curList: KList = lists.find(numId);
                // java if (null != curList)
                if true {
                  // java KListTemplate template = templates.find(curList.getLsid());
                  let mut template: KListTemplate = templates.find(curList.getLsid());
                  // java if (null != template)
                  if true {
                    // java int tplc = template.getTplc();
                    let mut tplc: i32 = template.getTplc();
                    // java if (tplc == myTplc)
                    if tplc == myTplc {
                      // java if (null == list)
                      if false {
                        // java list = curList;
                        list = curList;
                      }
                      // java else if (list != curList)
                      else if list != curList {
                        // java Assert.fail("unexpected contidion occured!");
                        Assert::fail__string("unexpected contidion occured!");
                      }
                      // java MutablePropertySet mutableStyleProps = new MutablePropertySet(props);
                      let mut mutableStyleProps: MutablePropertySet = MutablePropertySet::new(props);
                      // java mutableStyleProps.setValue(235, (Object)null);
                      mutableStyleProps.setValue(235, None);
                      // java mutableStyleProps.setValue(234, (Object)null);
                      mutableStyleProps.setValue(234, None);
                      // java style.setProperty(mutableStyleProps.freeze());
                      style.setProperty(mutableStyleProps.freeze());
                      // java Integer level = (Integer)props.getValue(234);
                      let mut level: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
                      // java if (null == level)
                      if false {
                        // java level = 0;
                        level = 0;
                      }
                      // java Assert.assertTrue("(i == level) should be true.", i == level);
                      Assert::assertTrue__string_boolean("(i == level) should be true.", i == level);
                      // java template.setSti(level + 1, (short)4095);
                      template.setSti(level + 1, ((4095) as i16));
                      // java toClearCache = true;
                      toClearCache = true;
                    }
                  }
                }
              }
            }
          }
        }
      }
      // java this.initAllSti();
      self.initAllSti();
      // java if (toClearCache)
      if toClearCache {
        // java this.updateDocumentAndClearCache();
        self.updateDocumentAndClearCache();
      }
      // java return list;
      return Some(&list);
    }

    // java private KList getStyleTakingList()
    pub fn getStyleTakingList(&self) -> Option<&KList> {
      // java KLists lists = this.mTextDocument.getLists();
      let mut lists: KLists = self.mTextDocument.getLists();
      // java Assert.assertNotNull("lists should not be null.", lists);
      Assert::assertNotNull__string_object("lists should not be null.", lists);
      // java KListTemplates templates = this.mTextDocument.getListTemplates();
      let mut templates: KListTemplates = self.mTextDocument.getListTemplates();
      // java Assert.assertNotNull("templates should not be null.", templates);
      Assert::assertNotNull__string_object("templates should not be null.", templates);
      // java KStyles styles = this.mTextDocument.getStyles();
      let mut styles: KStyles = self.mTextDocument.getStyles();
      // java Assert.assertNotNull("styles should not be null.", styles);
      Assert::assertNotNull__string_object("styles should not be null.", styles);
      // java int myTplc = this.getTplc();
      let mut myTplc: i32 = self.getTplc();
      // java KList list = null;
      let mut list: KList = Default::default();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut n: i32 = self.mListLevels.count();
      // java for(int n = this.mListLevels.count(); i < n; ++i)
      while i < n {
        // java int sti = this.getSti(i + 1);
        let mut sti: i32 = (self.getSti(i + 1) as i32);
        // java if (4095 != sti)
        if 4095 != sti {
          // java KStyle style = styles.getStyleById(sti, false);
          let mut style: KStyle = styles.getStyleById(sti, false);
          // java if (null != style)
          if true {
            // java PropertySet props = style.getProperty();
            let mut props: PropertySet = style.getProperty();
            // java if (null != props)
            if true {
              // java Integer numId = (Integer)props.getValue(235);
              let mut numId: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
              // java if (ListUtil.isNumIdExist(numId))
              if ListUtil::isNumIdExist(numId) {
                // java KList curList = lists.find(numId);
                let mut curList: KList = lists.find(numId);
                // java if (null != curList)
                if true {
                  // java KListTemplate template = templates.find(curList.getLsid());
                  let mut template: KListTemplate = templates.find(curList.getLsid());
                  // java if (null != template)
                  if true {
                    // java int tplc = template.getTplc();
                    let mut tplc: i32 = template.getTplc();
                    // java Assert.assertTrue("All of the referenced style should either be not-in-list or in a list whose tplc is the same as mine.", tplc == myTplc);
                    Assert::assertTrue__string_boolean("All of the referenced style should either be not-in-list or in a list whose tplc is the same as mine.", tplc == myTplc);
                    // java if (tplc == myTplc)
                    if tplc == myTplc {
                      // java if (null == list)
                      if false {
                        // java list = curList;
                        list = curList;
                      }
                      // java else if (list != curList)
                      else if list != curList {
                        // java Assert.fail("unexpected contidion occured!");
                        Assert::fail__string("unexpected contidion occured!");
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      // java return list;
      return Some(&list);
    }

    // java public KList checkStyleReferences(boolean clearStyleTakenByMyTplc)
    pub fn checkStyleReferences(&mut self, clearStyleTakenByMyTplc: bool) -> Option<&KList> {
      // java if (this.isStyleTakenByOther())
      if self.isStyleTakenByOther() {
        // java return clearStyleTakenByMyTplc ? this.clearStyleTakenByMyTplc() : null;
        return Some(&(if clearStyleTakenByMyTplc { Some(self.clearStyleTakenByMyTplc()) } else { None }));
      }
      // java else
      else {
        // java KList list = this.getStyleTakingList();
        let mut list: KList = self.getStyleTakingList().unwrap();
        // java if (null != list)
        if true {
          // java KListTemplates templates = this.mTextDocument.getListTemplates();
          let mut templates: KListTemplates = self.mTextDocument.getListTemplates();
          // java Assert.assertNotNull("templates should not be null.", templates);
          Assert::assertNotNull__string_object("templates should not be null.", templates);
          // java KListTemplate templateRemoved = templates.find(list.getLsid());
          let mut templateRemoved: KListTemplate = templates.find(list.getLsid());
          // java Assert.assertNotNull("templateBeRemoved should not be null.", templateRemoved);
          Assert::assertNotNull__string_object("templateBeRemoved should not be null.", templateRemoved);
          // java int lsid = templateRemoved.getLsid();
          let mut lsid: i32 = templateRemoved.getLsid();
          // java templates.remove(lsid);
          templates.remove(lsid);
          // java templates.add(this, lsid);
          templates.add(self, lsid);
          // java list.referenceListTemplate(this);
          list.referenceListTemplate(self);
          // java list.applyToStyles();
          list.applyToStyles();
        }
        // java return list;
        return Some(&list);
      }
    }

    // java final void adjustAllLevelLeftIndents(Integer firstLineAbsPos)
    pub fn adjustAllLevelLeftIndents(&mut self, firstLineAbsPos: Option<i32>) {
      // java if (0 != firstLineAbsPos)
      if 0 != firstLineAbsPos {
        // java KListLevel listLevel0 = this.mListLevels.item(0);
        let mut listLevel0: KListLevel = self.mListLevels.item(0);
        // java Assert.assertNotNull("listLevel0 should not be null.", listLevel0);
        Assert::assertNotNull__string_object("listLevel0 should not be null.", listLevel0);
        // java LvlfData lvlf0 = listLevel0.getLvlfData();
        let mut lvlf0: LvlfData = listLevel0.getLvlfData();
        // java Assert.assertNotNull("lvlf0 should not be null.", lvlf0);
        Assert::assertNotNull__string_object("lvlf0 should not be null.", lvlf0);
        // java PropertySet papx0 = lvlf0.mPapx;
        let mut papx0: PropertySet = lvlf0.mPapx;
        // java Assert.assertNotNull("papx0 should not be null.", papx0);
        Assert::assertNotNull__string_object("papx0 should not be null.", papx0);
        // java Integer left0 = (Integer)papx0.getValue(210);
        let mut left0: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
        // java Assert.assertNotNull("left0 should not be null.", left0);
        Assert::assertNotNull__string_object("left0 should not be null.", left0);
        // java Integer firstLine0 = (Integer)papx0.getValue(208);
        let mut firstLine0: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
        // java Assert.assertNotNull("firstLine0 should not be null.", firstLine0);
        Assert::assertNotNull__string_object("firstLine0 should not be null.", firstLine0);
        // java Integer newLeft0 = firstLineAbsPos + -firstLine0;
        let mut newLeft0: i32 = firstLineAbsPos + -firstLine0;
        // java Integer deltaLeft = newLeft0 - left0;
        let mut deltaLeft: i32 = newLeft0 - left0;
        // java MutablePropertySet mutableParaProps0 = new MutablePropertySet(papx0);
        let mut mutableParaProps0: MutablePropertySet = MutablePropertySet::new(papx0);
        // java mutableParaProps0.setInt(210, newLeft0);
        mutableParaProps0.setInt(210, newLeft0);
        // java lvlf0.mPapx = mutableParaProps0.freeze();
        lvlf0.mPapx = mutableParaProps0.freeze();
        // java this.adjustLevelLeftIndents(2, deltaLeft);
        self.adjustLevelLeftIndents(2, deltaLeft);
      }
    }

    // java final void adjustLevelLeftIndents(int level, int deltaLeft)
    pub fn adjustLevelLeftIndents(&mut self, level: i32, deltaLeft: i32) {
      // java if (0 != deltaLeft)
      if 0 != deltaLeft {
        // java Assert.assertTrue("level should be more than 1 and less than 9 !", level >= 1 && level <= 9);
        Assert::assertTrue__string_boolean("level should be more than 1 and less than 9 !", level >= 1 && level <= 9);
        // java Assert.assertTrue("the length of listLevels should be equal to 9", this.mListLevels.count() == 9);
        Assert::assertTrue__string_boolean("the length of listLevels should be equal to 9", self.mListLevels.count() == 9);
        // java int index = level - 1;
        let mut index: i32 = level - 1;
        // java int i = index;
        let mut i: i32 = index;
        let mut n: i32 = self.mListLevels.count();
        // java for(int n = this.mListLevels.count(); i < n; ++i)
        while i < n {
          // java KListLevel listLevel = this.mListLevels.item(i);
          let mut listLevel: KListLevel = self.mListLevels.item(i);
          // java Assert.assertNotNull("listLevel should not be null.", listLevel);
          Assert::assertNotNull__string_object("listLevel should not be null.", listLevel);
          // java LvlfData lvlf = listLevel.getLvlfData();
          let mut lvlf: LvlfData = listLevel.getLvlfData();
          // java Assert.assertNotNull("lvlf should not be null.", lvlf);
          Assert::assertNotNull__string_object("lvlf should not be null.", lvlf);
          // java PropertySet papx = lvlf.mPapx;
          let mut papx: PropertySet = lvlf.mPapx;
          // java Assert.assertNotNull("papx should not be null.", papx);
          Assert::assertNotNull__string_object("papx should not be null.", papx);
          // java Integer left = (Integer)papx.getValue(210);
          let mut left: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
          // java Assert.assertNotNull("left should not be null.", left);
          Assert::assertNotNull__string_object("left should not be null.", left);
          // java MutablePropertySet mutableParaProps = new MutablePropertySet(papx);
          let mut mutableParaProps: MutablePropertySet = MutablePropertySet::new(papx);
          // java int newLeft = left + deltaLeft;
          let mut newLeft: i32 = left + deltaLeft;
          // java mutableParaProps.setInt(210, newLeft);
          mutableParaProps.setInt(210, newLeft);
          // java lvlf.mPapx = mutableParaProps.freeze();
          lvlf.mPapx = mutableParaProps.freeze();
        }
      }
    }

    // java private void updateDocumentAndClearCache()
    pub fn updateDocumentAndClearCache(&mut self) {
      // java PropertyTool.clearCache();
      WriterPropertyTool::clearCache();
      // java this.mTextDocument.updateDocument();
      self.mTextDocument.updateDocument();
    }

    // java public int getRightLevel(float firstLineAbsPos)
    pub fn getRightLevel(&self, firstLineAbsPos: f32) -> i32 {
      // java int n = this.mListLevels.count();
      let mut n: i32 = self.mListLevels.count();
      // java ArrayList<Pair<Integer, Integer>> indents = new ArrayList(n);
      let mut indents: Vec<Pair> = { let __x: Vec<Pair> = todo!("no align strategy matched"); __x };
      let mut i: i32 = 0;
      // java for(int i = 0; i < n; ++i)
      while i < n {
        // java KListLevel listLevel = this.mListLevels.item(i);
        let mut listLevel: KListLevel = self.mListLevels.item(i);
        // java Assert.assertNotNull("listLevel should not be null.", listLevel);
        Assert::assertNotNull__string_object("listLevel should not be null.", listLevel);
        // java LvlfData lvlf = listLevel.getLvlfData();
        let mut lvlf: LvlfData = listLevel.getLvlfData();
        // java Assert.assertNotNull("lvlf should not be null.", lvlf);
        Assert::assertNotNull__string_object("lvlf should not be null.", lvlf);
        // java PropertySet papx = lvlf.mPapx;
        let mut papx: PropertySet = lvlf.mPapx;
        // java Assert.assertNotNull("mergedPapx should not be null.", papx);
        Assert::assertNotNull__string_object("mergedPapx should not be null.", papx);
        // java Integer left = (Integer)papx.getValue(210);
        let mut left: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
        // java Integer first = (Integer)papx.getValue(208);
        let mut first: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
        // java if (!IndentsUtil.isExistTwipsIndent(left))
        if !IndentsUtil::isExistTwipsIndent(left) {
          // java left = 0;
          left = 0;
        }
        // java if (!IndentsUtil.isExistTwipsIndent(first))
        if !IndentsUtil::isExistTwipsIndent(first) {
          // java first = 0;
          first = 0;
        }
        // java int firstLinePos = left + first;
        let mut firstLinePos: i32 = left + first;
        // java int level = i + 1;
        let mut level: i32 = i + 1;
        // java indents.add(new Pair(firstLinePos, level));
        indents.add(Pair::new(firstLinePos, level));
      }
      // java Collections.sort(indents, new Comparator<Pair<Integer, Integer>>() { public int compare(Pair<Integer, Integer> o1, Pair<Integer, Integer> o2) { return (Integer)o1.first - (Integer)o2.first; } });
      Collections::sort(indents, Comparator::new());
      // java if (this.pressMultilevel4(indents))
      if self.pressMultilevel4({ let __x: &Vec<Pair<i32, i32>> = todo!("no align strategy matched"); __x }) {
        // java return this.getRightAdjacentLevel(firstLineAbsPos, true);
        return self.getRightAdjacentLevel(firstLineAbsPos, true);
      }
      // java else
      else {
        // java int size = indents.size();
        let mut size: i32 = (indents.len() as i32);
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java Pair<Integer, Integer> indent = (Pair)indents.get(i);
          let mut indent: Pair = (indents.get(i)).downcast_ref::<Pair>().unwrap();
          // java Assert.assertNotNull("indent should not be null.", indent);
          Assert::assertNotNull__string_object("indent should not be null.", indent);
          // java if (firstLineAbsPos <= (float)(Integer)indent.first)
          if firstLineAbsPos <= ((((indent.first) as i32)) as f32) {
            // java return (Integer)indent.second;
            return { let __x: i32 = todo!("no align strategy matched"); __x };
          }
        }
        // java if (size >= 1)
        if size >= 1 {
          // java Pair<Integer, Integer> maxFirstline = (Pair)indents.get(size - 1);
          let mut maxFirstline: Pair = (indents.get(size - 1)).downcast_ref::<Pair>().unwrap();
          // java Assert.assertNotNull("maxFirstline should not be null.", maxFirstline);
          Assert::assertNotNull__string_object("maxFirstline should not be null.", maxFirstline);
          // java return (Integer)maxFirstline.second;
          return { let __x: i32 = todo!("no align strategy matched"); __x };
        }
        // java else
        else {
          // java Assert.fail("It should not reach to here");
          Assert::fail__string("It should not reach to here");
          // java return 0;
          return 0;
        }
      }
    }

    // java private boolean pressMultilevel4(ArrayList<Pair<Integer, Integer>> indents)
    pub fn pressMultilevel4(&self, indents: &Vec<Pair<i32, i32>>) -> bool {
      // java int size = indents.size();
      let mut size: i32 = (indents.len() as i32);
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java Pair<Integer, Integer> indent = (Pair)indents.get(i);
        let mut indent: Pair = (indents.get(i)).downcast_ref::<Pair>().unwrap();
        // java Assert.assertNotNull("indent should not be null.", indent);
        Assert::assertNotNull__string_object("indent should not be null.", indent);
        // java if ((Integer)indent.first != 0)
        if ((indent.first) as i32) != 0 {
          // java return false;
          return false;
        }
      }
      // java return true;
      return true;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (null != this.mListLevels)
      if let Some(mListLevels) = self.mListLevels.as_mut() {
        // java this.mListLevels.dispose();
        mListLevels.dispose();
        // java this.mListLevels = null;
        mListLevels = None;
      }
      // java if (null != this.mLstData)
      if let Some(mLstData) = self.mLstData.as_mut() {
        // java this.mLstData.dispose();
        mLstData.dispose();
        // java this.mLstData = null;
        mLstData = None;
      }
    }

}
