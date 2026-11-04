// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.base.io.ControlChar
use util::base::io::ControlChar::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::*;
// Java: import cn.wps.moffice.writer.data.AbstractTextStreamFilter
use crate::cn::wps::moffice::writer::data::AbstractTextStreamFilter::*;
// Java: import cn.wps.moffice.writer.data.KTextStream
use crate::cn::wps::moffice::writer::data::KTextStream::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
// (generator mis-import; Entry = IChpxTable/IPapxTable::Entry)
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.PropertyAction
use crate::cn::wps::moffice::writer::data::PropertyAction::*;
// Java: import cn.wps.moffice.writer.data.PropertyOperation
use crate::cn::wps::moffice::writer::data::PropertyOperation::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::RuntimeException::RuntimeException;
use crate::cn::wps::moffice::writer::data::PropertyUtil::PropertyUtil;
use crate::cn::wps::moffice::writer::data::ITextStreamFilter::ITextStreamFilter;
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;

// java  class TextStream_NormalFilter extends AbstractTextStreamFilter
#[derive(Default)]
pub struct TextStream_NormalFilter {
    // java AbstractTextStreamFilter super_
    pub(crate) super_: AbstractTextStreamFilter,
}

// java  class TextStream_NormalFilter extends AbstractTextStreamFilter
impl TextStream_NormalFilter {
    // java public  TextStream_NormalFilter(KTextStream pTextStream)
    pub fn new(pTextStream: &KTextStream) -> Self {
      // java super(pTextStream);
      Self { super_: AbstractTextStreamFilter::new__ktextstream(pTextStream) }
    }

    /// Alias matching Java ctor name / older call sites.
    pub fn TextStream_NormalFilter(pTextStream: &KTextStream) -> Self {
      Self::new(pTextStream)
    }

    // java public void insertText(int cp, char[] text, int start, int end, PropertySet chpx)
    pub fn insertText(&mut self, cp: i32, text: &Vec<char>, start: i32, end: i32, chpx: &PropertySet) {
      // java boolean has_spec = false;
      let mut has_spec: bool = false;
      let mut i: i32 = start;
      // java for(int i = start; i < end; ++i)
      while i < end {
        // java if (!ControlChar.is_normal_char(text[i]))
        if !ControlChar::is_normal_char(text[(i) as usize]) {
          // java has_spec = true;
          has_spec = true;
          // java break;
          break;
        }
        i += 1;
      }
      // java if (has_spec)
      if has_spec {
        // java KSLog.w("TextStream_NormalFilter", "Special char found in " + new String(text, start, end - start));
        let slice: String = text[(start as usize)..(end as usize)].iter().collect();
        KSLog::w("TextStream_NormalFilter", &format!("Special char found in {}", slice));
      }
      // java KSAssert.assertEquals(cp, this.getChpxTable().seek(cp).position());
      let chpx_table = self.super_.getChpxTable().expect("TextStream_NormalFilter: null CHPX table");
      KSAssert::assertEquals(cp, unsafe { IChpxTable::seek(&*chpx_table, cp) }.expect("seek").position());
      // java this.getTextRope().insert(cp, text, start, end);
      let rope = self.super_.getTextRope().expect("TextStream_NormalFilter: null text rope");
      unsafe { ITextRope::insert__int_chararr_int_int(&mut *rope, cp, text, start, end); }
      // java IChpxTable.Entry e = this.getChpxTable().seek(cp);
      let mut e: Box<dyn ChpxEntry> = unsafe { IChpxTable::seek(&*chpx_table, cp) }.expect("seek");
      // java e.setProperty(chpx);
      ChpxEntry::setProperty(e.as_mut(), Some(chpx));
    }

    // java public int removeText(int start, int end)
    pub fn removeText(&mut self, start: i32, end: i32) -> i32 {
      // java this.getTextStream().getTextRope().erase(start, end);
      let rope = self.super_.getTextRope().expect("TextStream_NormalFilter: null text rope");
      unsafe { ITextRope::erase(&mut *rope, start, end); }
      // java return end - start;
      return end - start;
    }

    // java private static PropertySet remove_spec(PropertySet chpx)
    pub fn remove_spec(chpx: Option<&PropertySet>) -> Option<PropertySet> {
      // java if (chpx != null && chpx.getBoolean(28, false))
      if chpx.is_some() && chpx.unwrap().getBoolean__int_boolean(28, false) {
          // java MutablePropertySet m = new MutablePropertySet(chpx);
          let mut m: MutablePropertySet = MutablePropertySet::new_from_ps(chpx.unwrap());
          // java m.remove(28);
          m.remove(28);
          // java return m.freeze();
          return Some(m.freeze());
      }
      // java else
      else {
        // java return chpx;
        return chpx.map(|c| c.clone());
      }
    }

    // java public void insertParagraph(int position, char ch, PropertySet chpx, PropertySet papx)
    pub fn insertParagraph(&mut self, position: i32, ch: char, chpx: &PropertySet, papx: &PropertySet) {
      // java chpx = remove_spec(chpx);
      let chpx_owned: Option<PropertySet> = Self::remove_spec(Some(chpx));
      let chpx: &PropertySet = chpx_owned.as_ref().unwrap_or(chpx);
      // java int len = this.getTextRope().length();
      let rope = self.super_.getTextRope().expect("TextStream_NormalFilter: null text rope");
      let len: i32 = unsafe { ITextRope::length(&*rope) };
      // java this.getTextRope().insert(position, ch);
      unsafe { ITextRope::insert__int_char(&mut *rope, position, ch); }
      let chpx_table = self.super_.getChpxTable().expect("TextStream_NormalFilter: null CHPX table");
      // java if (position == len)
      if position == len {
        // java this.getChpxTable().insert(position, chpx);
        unsafe { IChpxTable::insert(&mut *chpx_table, position, Some(chpx)); }
      }
      // java else
      else {
        // java IChpxTable.Entry e = this.getChpxTable().seek(position);
        let mut e: Box<dyn ChpxEntry> = unsafe { IChpxTable::seek(&*chpx_table, position) }.expect("seek");
        // java e.setProperty(chpx);
        ChpxEntry::setProperty(e.as_mut(), Some(chpx));
      }
      let papx_table = self.super_.getPapxTable().expect("TextStream_NormalFilter: null PAPX table");
      // java if (position == len)
      if position == len {
        // java this.getPapxTable().insert(position, papx);
        unsafe { IPapxTable::insert(&mut *papx_table, position, Some(papx)); }
      }
      // java else
      else {
        // java IPapxTable.Entry pExisting = this.getPapxTable().seek(position);
        let mut pExisting: Box<dyn PapxEntry> = unsafe { IPapxTable::seek(&*papx_table, position) }.expect("seek");
        // java this.getPapxTable().insert(position + 1, pExisting.getProperty());
        unsafe { IPapxTable::insert(&mut *papx_table, position + 1, PapxEntry::getProperty(pExisting.as_ref())); }
        // java pExisting.setProperty(papx);
        PapxEntry::setProperty(pExisting.as_mut(), Some(papx));
      }
    }

    // java public void insertPlaceHolder(int position, char ch, PropertySet chpx)
    pub fn insertPlaceHolder(&mut self, position: i32, ch: char, chpx: &PropertySet) {
      // java if (chpx != null && chpx.getBoolean(28, false))
      if chpx.getBoolean__int_boolean(28, false) {
          // java this.getTextStream().getTextRope().insert(position, ch);
          let rope = self.super_.getTextRope().expect("TextStream_NormalFilter: null text rope");
          unsafe { ITextRope::insert__int_char(&mut *rope, position, ch); }
          // java IChpxTable.Entry e = this.getChpxTable().seek(position);
          let chpx_table = self.super_.getChpxTable().expect("TextStream_NormalFilter: null CHPX table");
          let mut e: Box<dyn ChpxEntry> = unsafe { IChpxTable::seek(&*chpx_table, position) }.expect("seek");
          // java e.setProperty(chpx);
          ChpxEntry::setProperty(e.as_mut(), Some(chpx));
      }
      // java else
      else {
        // java throw new RuntimeException("The PropertySet must have run_spec == true");
        panic!("{}", RuntimeException::new__string("The PropertySet must have run_spec == true"));
      }
    }

    // java private static MutablePropertySet interpret(PropertySet pa, PropertyAction action)
    pub fn interpret(pa: Option<&PropertySet>, action: &dyn PropertyAction) -> MutablePropertySet {
      // java MutablePropertySet a = new MutablePropertySet(pa);
      let mut a: MutablePropertySet = match pa {
        Some(p) => MutablePropertySet::new_from_ps(p),
        None => MutablePropertySet::new(),
      };
      // java for(PropertyOperation op : action)
      action.for_each_op(&mut |op| {
        // java PropertySet b = op.getOperand();
        let b: &PropertySet = op.getOperand();
        // java switch (op.getOp())
        match op.getOp() {
          // java case 0:
          0 => {
          }
          // java case 1: case 4:
          1 | 4 => {
            // java a = PropertyUtil.getSpecialRunFormat(pa);
            a = PropertyUtil::getSpecialRunFormat(pa).unwrap_or_else(MutablePropertySet::new);
            // java a.add(b);
            a.add(b);
            // java PropertyUtil.maintainTableInfo(a, pa);
            if let Some(pa_ref) = pa {
              PropertyUtil::maintainTableInfo(&mut a, pa_ref);
              // java PropertyUtil.maintainMultiLanguageInfo(a, pa);
              PropertyUtil::maintainMultiLanguageInfo(&mut a, pa_ref);
            }
          }
          // java case 2:
          2 => {
            // java a.add(b);
            a.add(b);
          }
          // java case 3:
          3 => {
            // java clearProperty(a, b);
            Self::clearProperty(&mut a, b);
          }
          // java default:
          _ => {
            // java throw new RuntimeException("Unknown property operation!");
            panic!("{}", RuntimeException::new__string("Unknown property operation!"));
          }
        }
      });
      // java return a;
      return a;
    }

    // java private static void clearProperty(MutablePropertySet a, PropertySet b)
    pub fn clearProperty(a: &mut MutablePropertySet, b: &PropertySet) {
      // java int n = b.size();
      let mut n: i32 = b.size();
      // java int[] ids = new int[n];
      let mut ids: Vec<i32> = vec![Default::default(); (n) as usize];
      // java b.getAll(ids, (Object[])null);
      b.getAll(&mut ids, None);
      let mut i: i32 = 0;
      // java for(int i = 0; i < n; ++i)
      while i < n {
        // java a.remove(ids[i]);
        a.remove(ids[(i) as usize]);
        i += 1;
      }
    }

    // java public void modifyChpx(IChpxTable.Entry entry, PropertyAction action)
    pub fn modifyChpx(&mut self, entry: &mut dyn ChpxEntry, action: &dyn PropertyAction) {
      // java entry.setProperty(interpret(entry.getProperty(), action).freeze());
      let prop = Self::interpret(ChpxEntry::getProperty(entry), action).freeze();
      ChpxEntry::setProperty(entry, Some(&prop));
    }

    // java public void modifyPapx(IPapxTable.Entry entry, PropertyAction action)
    pub fn modifyPapx(&mut self, entry: &mut dyn PapxEntry, action: &dyn PropertyAction) {
      // java entry.setProperty(interpret(entry.getProperty(), action).freeze());
      let prop = Self::interpret(PapxEntry::getProperty(entry), action).freeze();
      PapxEntry::setProperty(entry, Some(&prop));
    }

}

impl ITextStreamFilter for TextStream_NormalFilter {
    fn getNext(&self) -> Option<&dyn ITextStreamFilter> {
        ITextStreamFilter::getNext(&self.super_)
    }
    fn setNext(&mut self, var1: Option<Box<dyn ITextStreamFilter>>) {
        ITextStreamFilter::setNext(&mut self.super_, var1)
    }
    fn insertText(&mut self, var1: i32, var2: &Vec<char>, var3: i32, var4: i32, var5: &PropertySet) {
        TextStream_NormalFilter::insertText(self, var1, var2, var3, var4, var5)
    }
    fn insertPlaceHolder(&mut self, var1: i32, var2: char, var3: &PropertySet) {
        TextStream_NormalFilter::insertPlaceHolder(self, var1, var2, var3)
    }
    fn removeText(&mut self, var1: i32, var2: i32) -> i32 {
        TextStream_NormalFilter::removeText(self, var1, var2)
    }
    fn insertParagraph(&mut self, var1: i32, var2: char, var3: &PropertySet, var4: &PropertySet) {
        TextStream_NormalFilter::insertParagraph(self, var1, var2, var3, var4)
    }
    fn modifyChpx(&mut self, var1: &mut dyn ChpxEntry, var2: &dyn PropertyAction) {
        TextStream_NormalFilter::modifyChpx(self, var1, var2)
    }
    fn modifyPapx(&mut self, var1: &mut dyn PapxEntry, var2: &dyn PropertyAction) {
        TextStream_NormalFilter::modifyPapx(self, var1, var2)
    }
    fn setEnable(&mut self, var1: bool) {
        ITextStreamFilter::setEnable(&mut self.super_, var1)
    }
    fn getEnable(&self) -> bool {
        ITextStreamFilter::getEnable(&self.super_)
    }
}

impl std::ops::Deref for TextStream_NormalFilter {
    type Target = AbstractTextStreamFilter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TextStream_NormalFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
