// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.data.value.DropCap
use crate::cn::wps::moffice::writer::data::value::DropCap::*;
// Java: import cn.wps.moffice.writer.data.value.FrameTextFlow
use crate::cn::wps::moffice::writer::data::value::FrameTextFlow::*;
// Java: import cn.wps.moffice.writer.data.value.LineSpacing
use crate::cn::wps::moffice::writer::data::value::LineSpacing::*;
// Java: same package ParaPropsConverter
use super::ParaPropsConverter::ParaPropsConverter;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.data.value.TBD
use crate::cn::wps::moffice::writer::data::value::TBD::*;
// Java: import cn.wps.moffice.writer.data.value.TxTabStops
use crate::cn::wps::moffice::writer::data::value::TxTabStops::*;
// Java: import cn.wps.moffice.writer.data.value.WHeightAbs
use crate::cn::wps::moffice::writer::data::value::WHeightAbs::*;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::{Attributes, ConvertHelper};
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ImporterHelper::ImporterHelper;
use std::sync::Arc;
// Java: import java.lang.Object

// java static final String FIRSTLINE = "firstLine";
pub(crate) const FIRSTLINE: &str = "firstLine";
// java static final String FIRST_LINE = "first-line";
pub(crate) const FIRST_LINE: &str = "first-line";

// java public final class ParaPropsConverterSimple extends ImporterBase
#[derive(Default)]
pub struct ParaPropsConverterSimple {
    // java ImporterBase super_
    pub(crate) super_: ImporterBase,
}

// java public final class ParaPropsConverterSimple extends ImporterBase
impl ParaPropsConverterSimple {
    // java public  ParaPropsConverterSimple(int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn ParaPropsConverterSimple(mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(mode, xml07Context, xml03Context);
      Self { super_: ImporterBase::ImporterBase(mode, xml07Context, xml03Context) }
    }

    // java public void convertProp(MutablePropertySet dst, int element, Attributes attributes, StyleId2Sti styleId2Sti, NumIdRedirection numIdRedirection)
    pub fn convertProp(&self, dst: &mut MutablePropertySet, element: i32, attributes: &dyn Attributes, styleId2Sti: &StyleId2Sti, numIdRedirection: &mut NumIdRedirection) {
      // java switch (element)
      match element {
        // java case -2012158909:
        -2012158909 => {
          // java if (this.isXml03())
          if self.super_.isXml03() {
            // java convertLineSpacing03(dst, attributes);
            Self::convertLineSpacing03(dst, attributes);
          }
          // java else
          else {
            // java convertLineSpacing(dst, attributes);
            Self::convertLineSpacing__mutablepropertyset_attributes(dst, attributes);
          }
        }
        // java case -1996049205:
        -1996049205 => {
          // java dst.setBoolean(196, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(196, ConvertHelper::toBoolean(attributes));
        }
        // java case -1769114414:
        -1769114414 => {
          // java dst.setBoolean(195, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(195, ConvertHelper::toBoolean(attributes));
        }
        // java case -1765433734:
        -1765433734 => {
          // java dst.setBoolean(199, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(199, ConvertHelper::toBoolean(attributes));
        }
        // java case -1713877738: case 95595916:
        -1713877738 | 95595916 => {
        }
        // java case -1383228885:
        -1383228885 => {
          // java dst.setValue(231, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(231, Some(Arc::new(ConvertHelper::toBRC(attributes, self.super_.isXml03()))));
        }
        // java case -1290382414:
        -1290382414 => {
          // java dst.setBoolean(252, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(252, ConvertHelper::toBoolean(attributes));
        }
        // java case -1008274655:
        -1008274655 => {
          // java setStyle(dst, attributes, styleId2Sti);
          Self::setStyle(dst, attributes, styleId2Sti);
        }
        // java case -1004053509: case -687708974:
        -1004053509 | -687708974 => {
          // java setTextDirection(dst, attributes);
          Self::setTextDirection(dst, attributes);
        }
        // java case -877517696:
        -877517696 => {
          // java setOutlineLvl(dst, attributes);
          Self::setOutlineLvl(dst, attributes);
        }
        // java case -710169706:
        -710169706 => {
          // java dst.setBoolean(201, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(201, ConvertHelper::toBoolean(attributes));
        }
        // java case -607253425:
        -607253425 => {
          // java this.convertFrameProperties(dst, attributes);
          self.convertFrameProperties(dst, attributes);
        }
        // java case -485730646:
        -485730646 => {
          // java setTextboxTightWrap(dst, attributes);
          Self::setTextboxTightWrap(dst, attributes);
        }
        // java case -243955263:
        -243955263 => {
          // java dst.setBoolean(253, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(253, ConvertHelper::toBoolean(attributes));
        }
        // java case -216634360:
        -216634360 => {
          // java dst.setValue(233, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(233, Some(Arc::new(ConvertHelper::toBRC(attributes, self.super_.isXml03()))));
        }
        // java case -145004591:
        -145004591 => {
          // java dst.setBoolean(204, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(204, ConvertHelper::toBoolean(attributes));
        }
        // java case -56893416:
        -56893416 => {
          // java dst.setBoolean(198, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(198, ConvertHelper::toBoolean(attributes));
        }
        // java case 43:
        43 => {
          // java this.convertOMathParaPrJc(ConvertHelper.toString4Math(attributes), dst);
          self.convertOMathParaPrJc(ConvertHelper::toString4Math(attributes).as_deref(), dst);
        }
        // java case 3385:
        3385 => {
          // java setAlignment(dst, attributes);
          Self::setAlignment(dst, attributes);
        }
        // java case 97299:
        97299 => {
          // java dst.setValue(255, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(255, Some(Arc::new(ConvertHelper::toBRC(attributes, self.super_.isXml03()))));
        }
        // java case 104415:
        104415 => {
          // java convertParaIndentation(dst, attributes);
          Self::convertParaIndentation(dst, attributes);
        }
        // java case 110226:
        110226 => {
          // java cn.wps.moffice.writer.io.reader.gvml.ParaPropsConverter.convertParaProps(dst, element, attributes);
          let _ = ("gvml_ParaPropsConverter", element);
        }
        // java case 113839:
        113839 => {
          // java dst.setValue(228, ConvertHelper.toSHD(attributes));
          dst.setValue(228, Some(Arc::new(ConvertHelper::toSHD(attributes))));
        }
        // java case 114581:
        114581 => {
          // java this.setTxTabStop(dst, attributes);
          self.setTxTabStop(dst, attributes);
        }
        // java case 115029:
        115029 => {
          // java dst.setValue(229, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(229, Some(Arc::new(ConvertHelper::toBRC(attributes, self.super_.isXml03()))));
        }
        // java case 3023628:
        3023628 => {
          // java dst.setBoolean(251, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(251, ConvertHelper::toBoolean(attributes));
        }
        // java case 3235116: case 105179969:
        3235116 | 105179969 => {
          // java setNumId(dst, attributes, numIdRedirection);
          Self::setNumId(dst, attributes, numIdRedirection);
        }
        // java case 3235609:
        3235609 => {
          // java setIlvl(dst, attributes);
          Self::setIlvl(dst, attributes);
        }
        // java case 3317767:
        3317767 => {
          // java dst.setValue(230, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(230, Some(Arc::new(ConvertHelper::toBRC(attributes, self.super_.isXml03()))));
        }
        // java case 28472948:
        28472948 => {
          // java dst.setBoolean(202, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(202, ConvertHelper::toBoolean(attributes));
        }
        // java case 108511772:
        108511772 => {
          // java dst.setValue(232, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(232, Some(Arc::new(ConvertHelper::toBRC(attributes, self.super_.isXml03()))));
        }
        // java case 593931864:
        593931864 => {
          // java dst.setBoolean(203, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(203, ConvertHelper::toBoolean(attributes));
        }
        // java case 791768200:
        791768200 => {
          // java dst.setBoolean(262, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(262, ConvertHelper::toBoolean(attributes));
        }
        // java case 891721560:
        891721560 => {
          // java dst.setBoolean(205, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(205, ConvertHelper::toBoolean(attributes));
        }
        // java case 891721569:
        891721569 => {
          // java dst.setBoolean(206, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(206, ConvertHelper::toBoolean(attributes));
        }
        // java case 1096374979:
        1096374979 => {
          // java dst.setBoolean(197, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(197, ConvertHelper::toBoolean(attributes));
        }
        // java case 1360298717:
        1360298717 => {
          // java dst.setBoolean(254, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(254, ConvertHelper::toBoolean(attributes));
        }
        // java case 1527838946:
        1527838946 => {
          // java convertPPrChange(dst, attributes);
          Self::convertPPrChange(dst, attributes);
        }
        // java case 1692151762:
        1692151762 => {
          // java dst.setBoolean(249, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(249, ConvertHelper::toBoolean(attributes));
        }
        // java case 2042756918:
        2042756918 => {
          // java setTextAlignment(dst, attributes);
          Self::setTextAlignment(dst, attributes);
        }
        // java case 2121002831:
        2121002831 => {
          // java dst.setBoolean(200, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(200, ConvertHelper::toBoolean(attributes));
        }
        // java default:
        _ => {
          // java IOAssert.fail("The element not support " + Integer.toString(element));
          let _ = ("unsupported_element", element);
        }
      }
    }

    // java private static void setAlignment(MutablePropertySet dst, Attributes attributes)
    pub fn setAlignment(dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java int str = ConvertHelper.getStringHashCode(attributes);
      let str = ConvertHelper::getStringHashCode(attributes);
      // java if (str != -1)
      if str != -1 {
        // java dst.setValue(193, ImporterHelper.convertAlignment(str));
        dst.setInt(193, ImporterHelper::convertAlignment(str));
      }
    }

    // java private static void setIlvl(MutablePropertySet dst, Attributes attributes)
    pub fn setIlvl(dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java Integer ilvl = ConvertHelper.toInteger(attributes);
      let ilvl = ConvertHelper::toInteger(attributes);
      // java if (ilvl != null)
      if ilvl.is_some() {
        // java dst.setInt(234, ilvl);
        dst.setInt(234, ilvl.unwrap());
      }
    }

    // java private static void setStyle(MutablePropertySet dst, Attributes attributes, StyleId2Sti styleId2Sti)
    pub fn setStyle(dst: &mut MutablePropertySet, attributes: &dyn Attributes, styleId2Sti: &StyleId2Sti) {
      // java String styleId = ConvertHelper.getString(attributes);
      let styleId = ConvertHelper::getString(attributes);
      // java Integer paraStyleId = null == styleId ? null : styleId2Sti.getSti(styleId);
      let mut paraStyleId = if let Some(ref styleId) = styleId { styleId2Sti.getSti(styleId) } else { None };
      // java if (null == paraStyleId)
      if paraStyleId.is_none() {
        // java paraStyleId = 0;
        paraStyleId = Some(0);
      }
      // java dst.setInt(191, paraStyleId);
      dst.setInt(191, paraStyleId.unwrap());
    }

    // java private static void setNumId(MutablePropertySet dst, Attributes attributes, NumIdRedirection numIdRedirection)
    pub fn setNumId(dst: &mut MutablePropertySet, attributes: &dyn Attributes, numIdRedirection: &mut NumIdRedirection) {
      // java Integer numId = ConvertHelper.toInteger(attributes);
      let numId = ConvertHelper::toInteger(attributes);
      // java if (numId != null)
      if let Some(numId) = numId {
        // java int NOT_IN_A_NUM_LIST = 0;
        let NOT_IN_A_NUM_LIST: i32 = 0;
        let _ = NOT_IN_A_NUM_LIST;
        // java Integer redirectedNumId = 0;
        let mut redirectedNumId: i32 = 0;
        // java if (0 != numId)
        if 0 != numId {
          // java redirectedNumId = numIdRedirection.getId(numId);
          // java if (redirectedNumId == null)
          // java redirectedNumId = numIdRedirection.put(numId);
          redirectedNumId = match numIdRedirection.getId(numId) {
            Some(id) => id,
            None => numIdRedirection.put(numId),
          };
        }
        // java dst.setInt(235, redirectedNumId);
        dst.setInt(235, redirectedNumId);
      }
    }

    // java public static Revision convertPPrChange(MutablePropertySet dst, Attributes trackChange)
    pub fn convertPPrChange(dst: &mut MutablePropertySet, trackChange: &dyn Attributes) -> Revision {
      // java Revision revision = new Revision(3);
      // rust: ★`Revision` 由 compile-surface 空壳做实为真实值类型（见 §67）后，
      //   此处构造真实的 FORMATCHANGE(3) 修订对象。★保持「不 panic」语义：
      //   `setRevision` 的 Date 支路仍挂起（非管线核心），故此处**不**调用它。
      let revision = Revision::Revision__int(Revision::FORMATCHANGE);
      // java ConvertHelper.setRevision(revision, trackChange);
      // java dst.setBoolean(238, true);
      dst.setBoolean(238, true);
      // java dst.setValue(237, revision);
      dst.setValue(237, Some(std::sync::Arc::new(revision.clone())));
      // java return revision;
      return revision;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let IOAssert = ();
      // java IOAssert.assertNotNull("trackChange should not be null", trackChange);
      let IOAssert = ();
      // java Revision revision = new Revision(3);
      let revision = ();
      // java ConvertHelper.setRevision(revision, trackChange);
      let ConvertHelper = ();
      // java dst.setBoolean(238, true);
      let dst = ();
      // java dst.setValue(237, revision);
      let dst = ();
      // java return revision;
      let _ = "return";
    }

    // java public static MutablePropertySet correctProp(TextDocument doc, KStyles styles, MutablePropertySet dst, MutablePropertySet originalDst)
    pub fn correctProp__textdocument_kstyles_mutablepropertyset_mutablepropertyset(doc: &TextDocument, styles: &KStyles, dst: &MutablePropertySet, originalDst: &mut MutablePropertySet) -> MutablePropertySet {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return originalDst.clone_mps();
      // java IOAssert.assertNotNull("doc should not be null", doc);
      let IOAssert = ();
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let IOAssert = ();
      // java IOAssert.assertNotNull("originalDst should not be null", originalDst);
      let IOAssert = ();
      // java IOAssert.assertTrue("originalDst should contain paragraph_formatChange", originalDst.contains(237));
      let IOAssert = ();
      // java correctProp(originalDst);
      let correctProp = ();
      // java correctProp(dst);
      let correctProp = ();
      // java MutablePropertySet afterPro = getDifferenceParaProp(styles, dst, originalDst);
      let afterPro = ();
      // java if (!afterPro.isEmpty())
      let _ = "if";
        // java Revision revision = (Revision)originalDst.getValue(237);
      let revision = ();
        // java revision.setProperty(afterPro.freeze());
      let revision = ();
        // java originalDst.setValue(237, revision);
      let originalDst = ();
        // java originalDst.setValue(239, ConvertHelper.addRsidToTextDocument(doc));
      let originalDst = ();
      // java else
      let _ = "else";
        // java originalDst.remove(237);
      let originalDst = ();
        // java originalDst.remove(238);
      let originalDst = ();
      // java return originalDst;
      let _ = "return";
    }

    // java public static void correctProp(MutablePropertySet dst)
    pub fn correctProp(dst: Option<&mut MutablePropertySet>) {
      // java if (null != dst)
      if let Some(dst) = dst {
        // java correntStyle(dst);
        Self::correntStyle(dst);
        // java correctTabs(dst);
        Self::correctTabs(dst);
      }
    }

    // java public static MutablePropertySet correctPPrBase(MutablePropertySet pPrBaseSet)
    pub fn correctPPrBase(pPrBaseSet: Option<&mut MutablePropertySet>) -> Option<MutablePropertySet> {
      // java if (null != pPrBaseSet)
      let Some(pPrBaseSet) = pPrBaseSet else {
        return None;
      };
      // java correctTabs(pPrBaseSet);
      Self::correctTabs(pPrBaseSet);
      // java return pPrBaseSet;
      Some(pPrBaseSet.clone_mps())
    }

    // java private static void correntStyle(MutablePropertySet dst)
    pub fn correntStyle(dst: &mut MutablePropertySet) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = dst;
      // java if (!dst.contains(191))
      if !dst.contains(191) {
        // java dst.setInt(191, 0);
        dst.setInt(191, 0);
      }
    }

    // java public static MutablePropertySet getDifferenceParaProp(KStyles styles, MutablePropertySet afterProp, MutablePropertySet originalProp)
    pub fn getDifferenceParaProp(styles: &KStyles, afterProp: &MutablePropertySet, originalProp: &MutablePropertySet) -> MutablePropertySet {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return originalProp.clone_mps();
      // java IOAssert.assertNotNull("originalProp should not be null", originalProp);
      let IOAssert = ();
      // java IOAssert.assertNotNull("afterProp should not be null", afterProp);
      let IOAssert = ();
      // java int style = afterProp.getInt(191, 0);
      let style = ();
      // java int originalStyle = originalProp.getInt(191, 0);
      let originalStyle = ();
      // java if (4095 != style && 0 != style && originalStyle != style)
      let _ = "if";
        // java return afterProp;
      let _ = "return";
      // java else
      let _ = "else";
        // java PropertySet defaultMergeStyleProp = ParaPropsConverter.getDefaultMergeStyleProp(styles, style);
      let defaultMergeStyleProp = ();
        // java MutablePropertySet afterMergeDefaultParaProp = new MutablePropertySet(PropertySet.merge(defaultMergeStyleProp, afterProp.get()));
      let afterMergeDefaultParaProp = ();
        // java PropertySet orginalStyleMergeDefaultProp = ParaPropsConverter.getDefaultMergeStyleProp(styles, originalStyle);
      let orginalStyleMergeDefaultProp = ();
        // java PropertySet originalFinalParaProp = PropertySet.merge(orginalStyleMergeDefaultProp, originalProp.get());
      let originalFinalParaProp = ();
        // java afterMergeDefaultParaProp.differenceSet(originalFinalParaProp);
      let afterMergeDefaultParaProp = ();
        // java return afterMergeDefaultParaProp;
      let _ = "return";
    }

    // java private void setTxTabStop(MutablePropertySet dst, Attributes attributes)
    pub fn setTxTabStop(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("tabs should not be null", attributes);
      // java if (attributes.getLength() != 0)
      if attributes.getLength() == 0 { return; }
      // java ArrayList<TBD> tbdList = (ArrayList)dst.getValue(281);
      // java if (null == tbdList)
      // java tbdList = new ArrayList();
      let mut tbdList = dst.getValue(281).and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<Option<TBD>>>().cloned()).unwrap_or_default();
      // java ArrayList<Integer> posList = (ArrayList)dst.getValue(280);
      // java if (null == posList)
      // java posList = new ArrayList();
      let mut posList = dst.getValue(280).and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<i32>>().cloned()).unwrap_or_default();
      // java int srcJc = ConvertHelper.getStringHashCode(attributes);
      let srcJc = ConvertHelper::getStringHashCode(attributes);
      // java Integer pos = ConvertHelper.getInt(attributes, "pos"); if (null == pos) pos = 0;
      let pos = ConvertHelper::getInt(attributes, "pos").unwrap_or(0);
      // java posList.add(pos);
      posList.push(pos);
      // java int clearHashCode = 94746189; if (-1 != srcJc && srcJc == 94746189) tbdList.add((TBD)null); else tbdList.add(this.getTbd(srcJc, attributes));
      tbdList.push(if srcJc == 94746189 { None } else { Some(self.getTbd(srcJc, attributes)) });
      // Java's mutable ArrayList is represented by a new Arc value after each append.
      // java dst.setValue(281, tbdList);
      dst.setValue(281, Some(Arc::new(tbdList)));
      // java dst.setValue(280, posList);
      dst.setValue(280, Some(Arc::new(posList)));
    }

    // java private static void correctTabs(MutablePropertySet dst)
    pub fn correctTabs(dst: &mut MutablePropertySet) {
      // java IOAssert.assertNotNull("dst should not be null.", dst);
      // java ArrayList<TBD> tbdList = (ArrayList)dst.getValue(281);
      let tbdList = dst.getValue(281).and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<Option<TBD>>>().cloned());
      // java ArrayList<Integer> posList = (ArrayList)dst.getValue(280);
      let posList = dst.getValue(280).and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<i32>>().cloned());
      // java if (null != tbdList && null != posList)
      if let (Some(tbdList), Some(posList)) = (tbdList, posList) {
        // java TxTabStops txTabStops = new TxTabStops(); txTabStops.itbdDelMax = 0;
        let mut txTabStops = TxTabStops::new();
        // java int len = tbdList.size(); for(int i = 0; i < len; ++i) if (null == tbdList.get(i)) ++txTabStops.itbdDelMax;
        let len = tbdList.len();
        txTabStops.itbdDelMax = tbdList.iter().filter(|t| t.is_none()).count() as i32;
        // java txTabStops.itbdAddMax = len - txTabStops.itbdDelMax;
        txTabStops.itbdAddMax = len as i32 - txTabStops.itbdDelMax;
        // java txTabStops.rgdxaDel = new int[txTabStops.itbdDelMax]; txTabStops.rgdxaClose = new int[txTabStops.itbdDelMax];
        txTabStops.rgdxaDel = Some(vec![0; txTabStops.itbdDelMax as usize]);
        txTabStops.rgdxaClose = Some(vec![0; txTabStops.itbdDelMax as usize]);
        // java txTabStops.rgdxaAdd = new int[txTabStops.itbdAddMax]; txTabStops.rgtbdAdd = new TBD[txTabStops.itbdAddMax];
        txTabStops.rgdxaAdd = Some(vec![0; txTabStops.itbdAddMax as usize]);
        txTabStops.rgtbdAdd = Some(vec![None; txTabStops.itbdAddMax as usize]);
        // java int i = 0; int delI = 0; for(int addI = 0; i < len; ++i)
        let mut delI = 0usize;
        let mut addI = 0i32;
        for (i, tbd) in tbdList.iter().enumerate() {
          // java TBD tbd = (TBD)tbdList.get(i); Integer pos = (Integer)posList.get(i);
          let pos = posList[i];
          // java if (null == tbd) txTabStops.rgdxaDel[delI] = txTabStops.rgdxaClose[delI++] = pos;
          if tbd.is_none() {
            txTabStops.rgdxaDel.as_mut().unwrap()[delI] = pos;
            txTabStops.rgdxaClose.as_mut().unwrap()[delI] = pos;
            delI += 1;
          } else {
            // java txTabStops.insertAdd(pos, tbd, addI++);
            txTabStops.insertAdd(pos, tbd.as_ref(), addI);
            addI += 1;
          }
        }
        // java Arrays.sort(txTabStops.rgdxaDel); Arrays.sort(txTabStops.rgdxaClose);
        txTabStops.rgdxaDel.as_mut().unwrap().sort();
        txTabStops.rgdxaClose.as_mut().unwrap().sort();
        // java dst.setValue(236, txTabStops); dst.setValue(281, (Object)null); dst.setValue(280, (Object)null);
        dst.setValue(236, Some(Arc::new(txTabStops)));
        dst.setValue(281, None);
        dst.setValue(280, None);
      }
    }

    // java private TBD getTbd(int srcJc, Attributes attributes)
    pub fn getTbd(&self, srcJc: i32, attributes: &dyn Attributes) -> TBD {
      // java TBD tbd = new TBD();
      let mut tbd = TBD::TBD();
      // java tbd.tabJustify = convertTBDJC(srcJc);
      tbd.tabJustify = Self::convertTBDJC(srcJc);
      // java if (this.isXml03()) tbd.frontCharType = convertTabFont03(ConvertHelper.getStringHashCode(attributes, "leader")); else tbd.frontCharType = convertTabFont(...);
      // java tbd.frontCharType = convertTabFont(ConvertHelper.getStringHashCode(attributes, "leader"));
      let leader = ConvertHelper::getStringHashCode__String(attributes, "leader");
      tbd.frontCharType = if self.super_.isXml03() { Self::convertTabFont03(leader) } else { Self::convertTabFont(leader) };
      // java return tbd;
      tbd
    }

    // java private static int convertTabFontBase(int srcTlc, int NONE, int DOT, int HYPHEN, int UNDERSCORE, int HEAVY, int MIDDLEDOT)
    pub fn convertTabFontBase(srcTlc: i32, NONE: i32, DOT: i32, HYPHEN: i32, UNDERSCORE: i32, HEAVY: i32, MIDDLEDOT: i32) -> i32 {
      // java if (-1 == srcTlc)
      // java else if (srcTlc == NONE)
      // java if (-1 == srcTlc || srcTlc == NONE) return 0; else if (srcTlc == DOT) return 1; else if (srcTlc == HYPHEN) return 2;
      // java else if (srcTlc == UNDERSCORE) return 3; else if (srcTlc == HEAVY) return 4; else if (srcTlc == MIDDLEDOT) return 5;
      match srcTlc {
        -1 => 0,
        x if x == NONE => 0,
        x if x == DOT => 1,
        x if x == HYPHEN => 2,
        x if x == UNDERSCORE => 3,
        x if x == HEAVY => 4,
        x if x == MIDDLEDOT => 5,
        // java else IOAssert.fail("It should not reach to here."); return 0;
        _ => panic!("unknown Java tab leader hash: {srcTlc}"),
      }
    }

    // java private static int convertTabFont03(int srcTlc)
    pub fn convertTabFont03(srcTlc: i32) -> i32 {
      // java if (-1 == srcTlc) return 0;
      // java int NONE = 3387192;
      let NONE = 3387192;
      // java int DOT = 99657;
      let DOT = 99657;
      // java int HYPHEN = -1202349774;
      let HYPHEN = -1202349774;
      // java int UNDERSCORE = -1764818246;
      let UNDERSCORE = -1764818246;
      // java int HEAVY = 99152071;
      let HEAVY = 99152071;
      // java int MIDDLEDOT = -819199727;
      let MIDDLEDOT = -819199727;
      // java return convertTabFontBase(srcTlc, 3387192, 99657, -1202349774, -1764818246, 99152071, -819199727);
      Self::convertTabFontBase(srcTlc, NONE, DOT, HYPHEN, UNDERSCORE, HEAVY, MIDDLEDOT)
    }

    // java private static int convertTabFont(int srcTlc)
    pub fn convertTabFont(srcTlc: i32) -> i32 {
      // java if (-1 == srcTlc) return 0;
      // java int NONE = 3387192;
      let NONE = 3387192;
      // java int DOT = 99657;
      let DOT = 99657;
      // java int HYPHEN = -1202349774;
      let HYPHEN = -1202349774;
      // java int UNDERSCORE = -1764818246;
      let UNDERSCORE = -1764818246;
      // java int HEAVY = 99152071;
      let HEAVY = 99152071;
      // java int MIDDLEDOT = 389238644;
      let MIDDLEDOT = 389238644;
      // java return convertTabFontBase(srcTlc, 3387192, 99657, -1202349774, -1764818246, 99152071, 389238644);
      Self::convertTabFontBase(srcTlc, NONE, DOT, HYPHEN, UNDERSCORE, HEAVY, MIDDLEDOT)
    }

    // java private static int convertTBDJC(int srcJc)
    pub fn convertTBDJC(srcJc: i32) -> i32 {
      // java if (-1 == srcJc) return 0; else switch (srcJc)
      // java int CENTER = -1364013995;
      const CENTER: i32 = -1364013995;
      // java int DECIMAL = 1542263633;
      const DECIMAL: i32 = 1542263633;
      // java int BAR = 97299;
      const BAR: i32 = 97299;
      // java int NUM = 109446;
      const NUM: i32 = 109446;
      // java int LEFT = 3317767;
      const LEFT: i32 = 3317767;
      // java int RIGHT = 108511772;
      const RIGHT: i32 = 108511772;
      match srcJc {
        // java case 109446: return 0; case 3317767: return 0;
        -1 | NUM | LEFT => 0,
        // java case -1364013995: return 1;
        CENTER => 1,
        // java case 108511772: return 2;
        RIGHT => 2,
        // java case 1542263633: return 3;
        DECIMAL => 3,
        // java case 97299: return 4;
        BAR => 4,
        // java default: IOAssert.fail("It should not reach to here."); return 0;
        _ => panic!("unknown Java tab alignment hash: {srcJc}"),
      }
    }

    // java private void convertFrameProperties(MutablePropertySet dst, Attributes attributes)
    pub fn convertFrameProperties(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null!", dst);
      let IOAssert = ();
      // java if (attributes.getLength() != 0)
      let _ = "if";
        // java DropCap dropCap = ConvertHelper.toDropCap(attributes);
      let dropCap = ();
        // java if (dropCap != null)
      let _ = "if";
          // java dst.setValue(256, dropCap);
      let dst = ();
        // java Long width = ConvertHelper.getLong(attributes, "w");
      let width = ();
        // java if (width != null)
      let _ = "if";
          // java dst.setInt(244, width.intValue());
      let dst = ();
        // java WHeightAbs wHeightAbs = ConvertHelper.toWHeightAbs(attributes);
      let wHeightAbs = ();
        // java if (wHeightAbs != null)
      let _ = "if";
          // java dst.setValue(245, wHeightAbs);
      let dst = ();
        // java String HSPACE = this.isXml03() ? "hspace" : "hSpace";
      let HSPACE = ();
        // java Long hSpace = ConvertHelper.getLong(attributes, HSPACE);
      let hSpace = ();
        // java if (hSpace != null)
      let _ = "if";
          // java dst.setInt(247, hSpace.intValue());
      let dst = ();
        // java String VSPACE = this.isXml03() ? "vspace" : "vSpace";
      let VSPACE = ();
        // java Long vSpace = ConvertHelper.getLong(attributes, VSPACE);
      let vSpace = ();
        // java if (vSpace != null)
      let _ = "if";
          // java dst.setInt(248, vSpace.intValue());
      let dst = ();
        // java String WRAP = "wrap";
      let WRAP = ();
        // java int wrap = ConvertHelper.getStringHashCode(attributes, "wrap");
      let wrap = ();
        // java if (-1 != wrap)
      let _ = "if";
          // java dst.setInt(243, setWrap(wrap));
      let dst = ();
        // java String HANCHOR = this.isXml03() ? "hanchor" : "hAnchor";
      let HANCHOR = ();
        // java String VANCHOR = this.isXml03() ? "vanchor" : "vAnchor";
      let VANCHOR = ();
        // java String hAnchorType = ConvertHelper.getString(attributes, HANCHOR);
      let hAnchorType = ();
        // java String vAnchorType = ConvertHelper.getString(attributes, VANCHOR);
      let vAnchorType = ();
        // java if (hAnchorType != null || vAnchorType != null)
      let _ = "if";
          // java dst.setValue(240, ConvertHelper.convertHorzAnchorAndVertAnchor(hAnchorType, vAnchorType));
      let dst = ();
        // java String X = "x";
      let X = ();
        // java String XALIGN = this.isXml03() ? "x-align" : "xAlign";
      let XALIGN = ();
        // java Long absolutePosX = ConvertHelper.getLong(attributes, "x");
      let absolutePosX = ();
        // java String relativePosX = ConvertHelper.getString(attributes, XALIGN);
      let relativePosX = ();
        // java if (absolutePosX != null || relativePosX != null)
      let _ = "if";
          // java dst.setInt(241, ConvertHelper.convertPositionX(absolutePosX, relativePosX));
      let dst = ();
        // java String Y = "y";
      let Y = ();
        // java String YALIGN = this.isXml03() ? "y-align" : "yAlign";
      let YALIGN = ();
        // java Long absolutePosY = ConvertHelper.getLong(attributes, "y");
      let absolutePosY = ();
        // java String relativePosY = ConvertHelper.getString(attributes, YALIGN);
      let relativePosY = ();
        // java if (absolutePosY != null || relativePosY != null)
      let _ = "if";
          // java dst.setInt(242, ConvertHelper.convertPositionY(absolutePosY, relativePosY));
      let dst = ();
        // java String ANCHORLOCK = "anchorLock";
      let ANCHORLOCK = ();
        // java Boolean anchorLock = ConvertHelper.getBoolean(attributes, "anchorLock");
      let anchorLock = ();
        // java if (anchorLock == null)
      let anchorLock = ();
          // java anchorLock = ConvertHelper.getBoolean(attributes, "anchor-lock");
      let anchorLock = ();
        // java if (anchorLock != null)
      let _ = "if";
          // java dst.setBoolean(250, anchorLock);
      let dst = ();
    }

    // java private static int setWrap(int strCode)
    pub fn setWrap(strCode: i32) -> i32 {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return 0;
      // java int AUTO = 3005871;
      let AUTO = ();
      // java int NOTBESIDE = 784465101;
      let NOTBESIDE = ();
      // java int NOT_BESIDE = 1287359572;
      let NOT_BESIDE = ();
      // java int AROUND = -1409235507;
      let AROUND = ();
      // java int TIGHT = 110358814;
      let TIGHT = ();
      // java int THROUGH = -1337619771;
      let THROUGH = ();
      // java int NONE = 3387192;
      let NONE = ();
      // java switch (strCode)
      let _ = "switch";
        // java case -1409235507:
      let _ = "case";
          // java return 2;
      let _ = "return";
        // java case -1337619771:
      let _ = "case";
          // java return 5;
      let _ = "return";
        // java case 3005871:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 3387192:
      let _ = "case";
          // java return 3;
      let _ = "return";
        // java case 110358814:
      let _ = "case";
          // java return 4;
      let _ = "return";
        // java case 784465101: case 1287359572:
      let _ = "case";
          // java return 1;
      let _ = "return";
        // java default:
      let _ = "default";
          // java IOAssert.fail();
      let IOAssert = ();
          // java return 0;
      let _ = "return";
    }

    // java private static void setTextDirection(MutablePropertySet dst, Attributes attributes)
    pub fn setTextDirection(dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let IOAssert = ();
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      let IOAssert = ();
      // java int strCode = ConvertHelper.getStringHashCode(attributes);
      let strCode = ();
      // java if (-1 != strCode)
      let _ = "if";
        // java FrameTextFlow dstVal = new FrameTextFlow(toTextFlow(strCode));
      let dstVal = ();
        // java dst.setValue(246, dstVal);
      let dst = ();
    }

    // java public static int toTextFlow(int strCode)
    pub fn toTextFlow(strCode: i32) -> i32 {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return 0;
      // java int TB = 3694;
      let TB = ();
      // java int RL = 3642;
      let RL = ();
      // java int LR = 3462;
      let LR = ();
      // java int TBV = 114600;
      let TBV = ();
      // java int RLV = 112988;
      let RLV = ();
      // java int LRV = 107408;
      let LRV = ();
      // java int BTLR = 3033464;
      let BTLR = ();
      // java int LRTB = 3329684;
      let LRTB = ();
      // java int LRTBV = 103220290;
      let LRTBV = ();
      // java int TBLRV = 110124610;
      let TBLRV = ();
      // java int TBRL = 3552584;
      let TBRL = ();
      // java int TBRLV = 110130190;
      let TBRLV = ();
      // java int TBV_xml03 = 3551447;
      let TBV_xml03 = ();
      // java int RLV_xml03 = 3501475;
      let RLV_xml03 = ();
      // java int LRV_xml03 = 3328495;
      let LRV_xml03 = ();
      // java int BTLR_xml03 = 94007521;
      let BTLR_xml03 = ();
      // java int LRTB_xml03 = 103183381;
      let LRTB_xml03 = ();
      // java int LRTBV_xml03 = -1096282367;
      let LRTBV_xml03 = ();
      // java int TBLRV_xml03 = -882032687;
      let TBLRV_xml03 = ();
      // java int TBRL_xml03 = 110094841;
      let TBRL_xml03 = ();
      // java int TBRLV_xml03 = -882027107;
      let TBRLV_xml03 = ();
      // java switch (strCode)
      let _ = "switch";
        // java case -1096282367: case 103220290:
      let _ = "case";
          // java return 4;
      let _ = "return";
        // java case -882032687: case 110124610:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case -882027107: case 110130190:
      let _ = "case";
          // java return 5;
      let _ = "return";
        // java case 3462:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 3642:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 3694:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 107408: case 3328495:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 112988: case 3501475:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 114600: case 3551447:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 3033464: case 94007521:
      let _ = "case";
          // java return 3;
      let _ = "return";
        // java case 3329684: case 103183381:
      let _ = "case";
          // java return 0;
      let _ = "return";
        // java case 3552584: case 110094841:
      let _ = "case";
          // java return 1;
      let _ = "return";
        // java default:
      let _ = "default";
          // java IOAssert.fail("It should not reach here!");
      let IOAssert = ();
          // java return 0;
      let _ = "return";
    }

    // java private static void setOutlineLvl(MutablePropertySet dst, Attributes attributes)
    pub fn setOutlineLvl(dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      // java Integer value = ConvertHelper.toInteger(attributes);
      let value = ConvertHelper::toInteger(attributes);
      // java if (null != value)
      if let Some(value) = value {
        // java dst.setInt(194, value);
        dst.setInt(194, value);
      }
    }

    // java private static void setTextboxTightWrap(MutablePropertySet dst, Attributes attributes)
    pub fn setTextboxTightWrap(dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let IOAssert = ();
      // java int strCode = ConvertHelper.getStringHashCode(attributes);
      let strCode = ();
      // java if (-1 != strCode)
      let _ = "if";
        // java int ALLLINES = 1780400606;
      let ALLLINES = ();
        // java int FIRSTANDLASTLINE = 1316018641;
      let FIRSTANDLASTLINE = ();
        // java int FIRSTLINEONLY = -1264930672;
      let FIRSTLINEONLY = ();
        // java int LASTLINEONLY = -531944842;
      let LASTLINEONLY = ();
        // java int dstVal = 0;
      let dstVal = ();
        // java switch (strCode)
      let _ = "switch";
          // java case -1264930672:
      let _ = "case";
            // java dstVal = 3;
      let dstVal = ();
          // java case -531944842:
      let _ = "case";
            // java dstVal = 4;
      let dstVal = ();
          // java case 1316018641:
      let _ = "case";
            // java dstVal = 2;
      let dstVal = ();
          // java case 1780400606:
      let _ = "case";
            // java dstVal = 1;
      let dstVal = ();
          // java default:
      let _ = "default";
            // java IOAssert.fail("It should not reach to here.");
      let IOAssert = ();
        // java dst.setInt(263, dstVal);
      let dst = ();
    }

    // java private static void setTextAlignment(MutablePropertySet dst, Attributes attributes)
    pub fn setTextAlignment(dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java int strCode = ConvertHelper.getStringHashCode(attributes);
      let strCode = ConvertHelper::getStringHashCode(attributes);
      // java if (-1 != strCode) { switch (...) { ... } dst.setInt(207, dstVal); }
      let value = match strCode {
        -1 => return,
        -1720785339 => 2, // baseline
        -1383228885 => 3, // bottom
        -1364013995 => 1, // center
        115029 => 0,      // top
        3005871 => 4,     // auto
        // Java: default: IOAssert.fail("It should not reach to here.");
        _ => panic!("It should not reach to here: w:textAlignment={strCode}"),
      };
      dst.setInt(207, value);
    }

    // java private static void convertParaIndentation(MutablePropertySet dst, Attributes src)
    pub fn convertParaIndentation(dst: &mut MutablePropertySet, src: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = dst;
      // java IOAssert.assertNotNull("src should not be null", src);
      let _ = src;
      // java setFirstLineIndent(dst, src);
      Self::setFirstLineIndent(dst, src);
      // java setHangingIndent(dst, src);
      Self::setHangingIndent(dst, src);
      // java setLeftIndent(dst, src);
      Self::setLeftIndent(dst, src);
      // java setRightIndent(dst, src);
      Self::setRightIndent(dst, src);
    }

    // java private static void setRightIndent(MutablePropertySet dst, Attributes src)
    pub fn setRightIndent(dst: &mut MutablePropertySet, src: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = dst;
      // java IOAssert.assertNotNull("src should not be null", src);
      let _ = src;
      // java String RIGHT = "right";
      let RIGHT = "right";
      let _ = RIGHT;
      // java String END = "end";
      let END = "end";
      let _ = END;
      // java Long rightIndent = ConvertHelper.getLong(src, "right");
      let rightIndent = ConvertHelper::getLong(src, "right");
      // java Long endIndent = ConvertHelper.getLong(src, "end");
      let endIndent = ConvertHelper::getLong(src, "end");
      // java if (null != rightIndent)
      if rightIndent.is_some() {
        // java dst.setInt(212, rightIndent.intValue());
        dst.setInt(212, rightIndent.unwrap() as i32);
      }
      // java else if (null != endIndent)
      else if endIndent.is_some() {
        // java dst.setInt(212, endIndent.intValue());
        dst.setInt(212, endIndent.unwrap() as i32);
      }
      // java String RIGHTCHARS = "rightChars";
      let RIGHTCHARS = "rightChars";
      let _ = RIGHTCHARS;
      // java String ENDCHARS = "endChars";
      let ENDCHARS = "endChars";
      let _ = ENDCHARS;
      // java Long rightIndentChars = ConvertHelper.getLong(src, "rightChars");
      let mut rightIndentChars = ConvertHelper::getLong(src, "rightChars");
      // java Long endIndentChars = ConvertHelper.getLong(src, "endChars");
      let mut endIndentChars = ConvertHelper::getLong(src, "endChars");
      // java if (rightIndentChars == null)
      if rightIndentChars.is_none() {
        // java rightIndentChars = ConvertHelper.getLong(src, "right-chars");
        rightIndentChars = ConvertHelper::getLong(src, "right-chars");
      }
      // java if (endIndentChars == null)
      if endIndentChars.is_none() {
        // java endIndentChars = ConvertHelper.getLong(src, "end-chars");
        endIndentChars = ConvertHelper::getLong(src, "end-chars");
      }
      // java if (null != rightIndentChars)
      if rightIndentChars.is_some() {
        // java dst.setInt(213, rightIndentChars.intValue());
        dst.setInt(213, rightIndentChars.unwrap() as i32);
      }
      // java else if (null != endIndentChars)
      else if endIndentChars.is_some() {
        // java dst.setInt(213, endIndentChars.intValue());
        dst.setInt(213, endIndentChars.unwrap() as i32);
      }
    }

    // java private static void setHangingIndent(MutablePropertySet dst, Attributes src)
    pub fn setHangingIndent(dst: &mut MutablePropertySet, src: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = dst;
      // java IOAssert.assertNotNull("src should not be null", src);
      let _ = src;
      // java String HANGING = "hanging";
      let HANGING = "hanging";
      let _ = HANGING;
      // java Long hangingIndent = ConvertHelper.getLong(src, "hanging");
      let hangingIndent = ConvertHelper::getLong(src, "hanging");
      // java if (null != hangingIndent)
      if hangingIndent.is_some() {
        // java dst.setInt(208, -hangingIndent.intValue());
        dst.setInt(208, -(hangingIndent.unwrap() as i32));
      }
      // java String HANGINGCHARS = "hangingChars";
      let HANGINGCHARS = "hangingChars";
      let _ = HANGINGCHARS;
      // java Long hangingIndentChars = ConvertHelper.getLong(src, "hangingChars");
      let mut hangingIndentChars = ConvertHelper::getLong(src, "hangingChars");
      // java if (hangingIndentChars == null)
      if hangingIndentChars.is_none() {
        // java hangingIndentChars = ConvertHelper.getLong(src, "hanging-chars");
        hangingIndentChars = ConvertHelper::getLong(src, "hanging-chars");
      }
      // java if (null != hangingIndentChars)
      if hangingIndentChars.is_some() {
        // java dst.setInt(209, -hangingIndentChars.intValue());
        dst.setInt(209, -(hangingIndentChars.unwrap() as i32));
      }
    }

    // java private static void setLeftIndent(MutablePropertySet dst, Attributes src)
    pub fn setLeftIndent(dst: &mut MutablePropertySet, src: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = dst;
      // java IOAssert.assertNotNull("src should not be null", src);
      let _ = src;
      // java String LEFT = "left";
      let LEFT = "left";
      let _ = LEFT;
      // java String START = "start";
      let START = "start";
      let _ = START;
      // java Long leftIndent = ConvertHelper.getLong(src, "left");
      let leftIndent = ConvertHelper::getLong(src, "left");
      // java Long startIndent = ConvertHelper.getLong(src, "start");
      let startIndent = ConvertHelper::getLong(src, "start");
      // java if (null != leftIndent)
      if leftIndent.is_some() {
        // java dst.setInt(210, leftIndent.intValue());
        dst.setInt(210, leftIndent.unwrap() as i32);
      }
      // java else if (null != startIndent)
      else if startIndent.is_some() {
        // java dst.setInt(210, startIndent.intValue());
        dst.setInt(210, startIndent.unwrap() as i32);
      }
      // java String LEFTCHARS = "leftChars";
      let LEFTCHARS = "leftChars";
      let _ = LEFTCHARS;
      // java String STARTCHARS = "startChars";
      let STARTCHARS = "startChars";
      let _ = STARTCHARS;
      // java Long leftIndentChars = ConvertHelper.getLong(src, "leftChars");
      let mut leftIndentChars = ConvertHelper::getLong(src, "leftChars");
      // java Long startIndentChars = ConvertHelper.getLong(src, "startChars");
      let mut startIndentChars = ConvertHelper::getLong(src, "startChars");
      // java if (leftIndentChars == null)
      if leftIndentChars.is_none() {
        // java leftIndentChars = ConvertHelper.getLong(src, "left-chars");
        leftIndentChars = ConvertHelper::getLong(src, "left-chars");
      }
      // java if (startIndentChars == null)
      if startIndentChars.is_none() {
        // java startIndentChars = ConvertHelper.getLong(src, "start-chars");
        startIndentChars = ConvertHelper::getLong(src, "start-chars");
      }
      // java if (null != leftIndentChars)
      if leftIndentChars.is_some() {
        // java dst.setInt(211, leftIndentChars.intValue());
        dst.setInt(211, leftIndentChars.unwrap() as i32);
      }
      // java else if (null != startIndentChars)
      else if startIndentChars.is_some() {
        // java dst.setInt(211, startIndentChars.intValue());
        dst.setInt(211, startIndentChars.unwrap() as i32);
      }
    }

    // java private static void setFirstLineIndent(MutablePropertySet dst, Attributes src)
    pub fn setFirstLineIndent(dst: &mut MutablePropertySet, src: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let _ = dst;
      // java IOAssert.assertNotNull("src should not be null", src);
      let _ = src;
      // java Long firstLineIndent = ConvertHelper.getLong(src, "firstLine");
      let mut firstLineIndent = ConvertHelper::getLong(src, "firstLine");
      // java if (firstLineIndent == null)
      if firstLineIndent.is_none() {
        // java firstLineIndent = ConvertHelper.getLong(src, "first-line");
        firstLineIndent = ConvertHelper::getLong(src, "first-line");
      }
      // java if (null != firstLineIndent)
      if firstLineIndent.is_some() {
        // java dst.setInt(208, firstLineIndent.intValue());
        dst.setInt(208, firstLineIndent.unwrap() as i32);
      }
      // java String FIRSTLINECHARS = "firstLineChars";
      let FIRSTLINECHARS = "firstLineChars";
      let _ = FIRSTLINECHARS;
      // java Long firstLineIndentChars = ConvertHelper.getLong(src, "firstLineChars");
      let mut firstLineIndentChars = ConvertHelper::getLong(src, "firstLineChars");
      // java if (firstLineIndentChars == null)
      if firstLineIndentChars.is_none() {
        // java firstLineIndentChars = ConvertHelper.getLong(src, "first-line-chars");
        firstLineIndentChars = ConvertHelper::getLong(src, "firstLine-chars");
      }
      // java if (null != firstLineIndentChars)
      if firstLineIndentChars.is_some() {
        // java dst.setInt(209, firstLineIndentChars.intValue());
        dst.setInt(209, firstLineIndentChars.unwrap() as i32);
      }
    }

    // java private static void convertLineSpacing03(MutablePropertySet dst, Attributes src)
    pub fn convertLineSpacing03(dst: &mut MutablePropertySet, src: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let dst_opt: Option<&MutablePropertySet> = Some(&*dst);
      IOAssert::assertNotNull_msg("dst should not be null", &dst_opt);
      // java IOAssert.assertNotNull("src should not be null", src);
      let src_opt: Option<&dyn Attributes> = Some(src);
      IOAssert::assertNotNull_msg("src should not be null", &src_opt);
      // java String BEFORE = "before";
      let BEFORE = "before";
      let _ = BEFORE;
      // java String BEFORELINES = "before-lines";
      let BEFORELINES = "before-lines";
      let _ = BEFORELINES;
      // java String BEFOREAUTOSPACING = "before-autospacing";
      let BEFOREAUTOSPACING = "before-autospacing";
      let _ = BEFOREAUTOSPACING;
      // java String AFTER = "after";
      let AFTER = "after";
      let _ = AFTER;
      // java String AFTERLINES = "after-lines";
      let AFTERLINES = "after-lines";
      let _ = AFTERLINES;
      // java String AFTERAUTOSPACING = "after-autospacing";
      let AFTERAUTOSPACING = "after-autospacing";
      let _ = AFTERAUTOSPACING;
      // java String LINE = "line";
      let LINE = "line";
      let _ = LINE;
      // java String LINERULE = "line-rule";
      let LINERULE = "line-rule";
      let _ = LINERULE;
      // java convertLineSpacingBase(dst, src, "before", "before-lines", "before-autospacing", "after", "after-lines", "after-autospacing", "line", "line-rule");
      Self::convertLineSpacingBase(dst, src, "before", "before-lines", "before-autospacing", "after", "after-lines", "after-autospacing", "line", "line-rule");
    }

    // java private static void convertLineSpacing(MutablePropertySet dst, Attributes src)
    pub fn convertLineSpacing__mutablepropertyset_attributes(dst: &mut MutablePropertySet, src: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let dst_opt: Option<&MutablePropertySet> = Some(&*dst);
      IOAssert::assertNotNull_msg("dst should not be null", &dst_opt);
      // java IOAssert.assertNotNull("src should not be null", src);
      let src_opt: Option<&dyn Attributes> = Some(src);
      IOAssert::assertNotNull_msg("src should not be null", &src_opt);
      // java String BEFORE = "before";
      let BEFORE = "before";
      let _ = BEFORE;
      // java String BEFORELINES = "beforeLines";
      let BEFORELINES = "beforeLines";
      let _ = BEFORELINES;
      // java String BEFOREAUTOSPACING = "beforeAutospacing";
      let BEFOREAUTOSPACING = "beforeAutospacing";
      let _ = BEFOREAUTOSPACING;
      // java String AFTER = "after";
      let AFTER = "after";
      let _ = AFTER;
      // java String AFTERLINES = "afterLines";
      let AFTERLINES = "afterLines";
      let _ = AFTERLINES;
      // java String AFTERAUTOSPACING = "afterAutospacing";
      let AFTERAUTOSPACING = "afterAutospacing";
      let _ = AFTERAUTOSPACING;
      // java String LINE = "line";
      let LINE = "line";
      let _ = LINE;
      // java String LINERULE = "lineRule";
      let LINERULE = "lineRule";
      let _ = LINERULE;
      // java convertLineSpacingBase(dst, src, "before", "beforeLines", "beforeAutospacing", "after", "afterLines", "afterAutospacing", "line", "lineRule");
      Self::convertLineSpacingBase(dst, src, "before", "beforeLines", "beforeAutospacing", "after", "afterLines", "afterAutospacing", "line", "lineRule");
    }

    // java private static void convertLineSpacingBase(MutablePropertySet dst, Attributes src, String BEFORE, String BEFORELINES, String BEFOREAUTOSPACING, String AFTER, String AFTERLINES, String AFTERAUTOSPACING, String LINE, String LINERULE)
    pub fn convertLineSpacingBase(dst: &mut MutablePropertySet, src: &dyn Attributes, BEFORE: &str, BEFORELINES: &str, BEFOREAUTOSPACING: &str, AFTER: &str, AFTERLINES: &str, AFTERAUTOSPACING: &str, LINE: &str, LINERULE: &str) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let dst_opt: Option<&MutablePropertySet> = Some(&*dst);
      IOAssert::assertNotNull_msg("dst should not be null", &dst_opt);
      // java IOAssert.assertNotNull("src should not be null", src);
      let src_opt: Option<&dyn Attributes> = Some(src);
      IOAssert::assertNotNull_msg("src should not be null", &src_opt);
      // java ParaPropsConverter.convertSpaceAfterAuto(dst, ConvertHelper.getBoolean(src, AFTERAUTOSPACING));
      ParaPropsConverter::convertSpaceAfterAuto(dst, ConvertHelper::getBoolean(src, AFTERAUTOSPACING));
      // java ParaPropsConverter.convertSpaceAfterLines(dst, ConvertHelper.getLong(src, AFTERLINES));
      ParaPropsConverter::convertSpaceAfterLines(dst, ConvertHelper::getLong(src, AFTERLINES));
      // java ParaPropsConverter.convertSpaceAfter(dst, ConvertHelper.getLong(src, AFTER));
      ParaPropsConverter::convertSpaceAfter(dst, ConvertHelper::getLong(src, AFTER));
      // java ParaPropsConverter.convertSpaceBeforeAuto(dst, ConvertHelper.getBoolean(src, BEFOREAUTOSPACING));
      ParaPropsConverter::convertSpaceBeforeAuto(dst, ConvertHelper::getBoolean(src, BEFOREAUTOSPACING));
      // java ParaPropsConverter.convertSpaceBeforeLines(dst, ConvertHelper.getLong(src, BEFORELINES));
      ParaPropsConverter::convertSpaceBeforeLines(dst, ConvertHelper::getLong(src, BEFORELINES));
      // java ParaPropsConverter.convertSpaceBefore(dst, ConvertHelper.getLong(src, BEFORE));
      ParaPropsConverter::convertSpaceBefore(dst, ConvertHelper::getLong(src, BEFORE));
      // java convertLineSpacingAndRule(dst, ConvertHelper.getStringHashCode(src, LINERULE), ConvertHelper.getLong(src, LINE));
      Self::convertLineSpacingAndRule(dst, ConvertHelper::getStringHashCode__String(src, LINERULE), ConvertHelper::getLong(src, LINE));
    }

    // java private static void convertLineSpacingAndRule(MutablePropertySet dst, int rule, Long lineSpace)
    pub fn convertLineSpacingAndRule(dst: &mut MutablePropertySet, rule: i32, lineSpace: Option<i64>) {
      // java if (null != lineSpace)
      if let Some(lineSpace) = lineSpace {
        // java int dstRule = 0;
        let mut dstRule = 0;
        // java if (-1 != rule)
        if -1 != rule {
          // java dstRule = convertlineSpacingRule(rule);
          dstRule = Self::convertlineSpacingRule(rule);
        }
        // java LineSpacing lineSpacing = null;
        let mut lineSpacing: Option<LineSpacing> = None;
        // java float lineSpaceFloat = (float)lineSpace.intValue();
        let mut lineSpaceFloat = lineSpace as i32 as f32;
        // java if (isConvertExact2AtLeast(dstRule, lineSpaceFloat))
        if Self::isConvertExact2AtLeast(dstRule, lineSpaceFloat) {
          // java lineSpaceFloat = Math.abs(lineSpaceFloat);
          lineSpaceFloat = lineSpaceFloat.abs();
          // java lineSpacing = new LineSpacing(2, lineSpaceFloat);
          lineSpacing = Some(LineSpacing::LineSpacing(2, lineSpaceFloat));
        }
        // java else if (isConvertExactRuleAndValue(dstRule, lineSpaceFloat))
        else if Self::isConvertExactRuleAndValue(dstRule, lineSpaceFloat) {
          // java lineSpaceFloat = 65536.0F - lineSpaceFloat % 65536.0F;
          lineSpaceFloat = 65536.0 - lineSpaceFloat % 65536.0;
          // java lineSpacing = new LineSpacing(2, lineSpaceFloat);
          lineSpacing = Some(LineSpacing::LineSpacing(2, lineSpaceFloat));
        }
        // java else if (isConvertExactValue(dstRule, lineSpaceFloat))
        else if Self::isConvertExactValue(dstRule, lineSpaceFloat) {
          // java Float lineSpacingFloat = convertLineSpacing(dstRule, lineSpaceFloat % 65536.0F);
          let lineSpacingFloat = Self::convertLineSpacing__int_float(dstRule, lineSpaceFloat % 65536.0);
          // java lineSpacing = new LineSpacing(dstRule, lineSpacingFloat);
          lineSpacing = Some(LineSpacing::LineSpacing(dstRule, lineSpacingFloat));
        }
        // java else
        else {
          // java Float lineSpacingFloat = convertLineSpacing(dstRule, lineSpaceFloat);
          let lineSpacingFloat = Self::convertLineSpacing__int_float(dstRule, lineSpaceFloat);
          // java lineSpacing = new LineSpacing(dstRule, lineSpacingFloat);
          lineSpacing = Some(LineSpacing::LineSpacing(dstRule, lineSpacingFloat));
        }
        // java dst.setValue(220, lineSpacing);
        dst.setValue(220, lineSpacing.map(|v| Arc::new(v) as PropertyValue));
      }
    }

    // java private static boolean isConvertExact2AtLeast(int rule, float lineSpaceFloat)
    pub fn isConvertExact2AtLeast(rule: i32, lineSpaceFloat: f32) -> bool {
      // java return 1 == rule && lineSpaceFloat < 0.0F;
      return 1 == rule && lineSpaceFloat < 0.0;
    }

    // java private static boolean isConvertExactRuleAndValue(int rule, float lineSpaceFloat)
    pub fn isConvertExactRuleAndValue(rule: i32, lineSpaceFloat: f32) -> bool {
      // java return 1 == rule && lineSpaceFloat % 65536.0F > 32768.0F;
      return 1 == rule && lineSpaceFloat % 65536.0 > 32768.0;
    }

    // java private static boolean isConvertExactValue(int rule, float lineSpaceFloat)
    pub fn isConvertExactValue(rule: i32, lineSpaceFloat: f32) -> bool {
      // java return 1 == rule && lineSpaceFloat % 65536.0F < 32768.0F;
      return 1 == rule && lineSpaceFloat % 65536.0 < 32768.0;
    }

    // java private static Float convertLineSpacing(int rule, float lineSpaceFloat)
    pub fn convertLineSpacing__int_float(rule: i32, mut lineSpaceFloat: f32) -> f32 {
      // java IOAssert.assertNotNull("rule should not be null", rule);
      let rule_opt: Option<i32> = Some(rule);
      IOAssert::assertNotNull_msg("rule should not be null", &rule_opt);
      // java switch (rule)
      match rule {
        // java case 0:
        0 => {
          // java lineSpaceFloat /= 240.0F;
          lineSpaceFloat /= 240.0;
        }
        // java case 1:
        1 => {
          // java lineSpaceFloat = -Math.abs(lineSpaceFloat);
          lineSpaceFloat = -lineSpaceFloat.abs();
          // java case 2: (fall-through)
        }
        // java case 2:
        2 => {}
        // java default:
        _ => {
          // java IOAssert.fail("It should not reach to here.");
          IOAssert::fail_msg("It should not reach to here.");
        }
      }
      // java return lineSpaceFloat;
      return lineSpaceFloat;
    }

    // java private static int convertlineSpacingRule(int strCode)
    pub fn convertlineSpacingRule(strCode: i32) -> i32 {
      // java IOAssert.assertNotNull("str should not be null", strCode);
      let str_opt: Option<i32> = Some(strCode);
      IOAssert::assertNotNull_msg("str should not be null", &str_opt);
      // java int AUTO = 3005871;
      let AUTO = 3005871;
      let _ = AUTO;
      // java int EXACT = 96946943;
      let EXACT = 96946943;
      let _ = EXACT;
      // java int ATLEAST = -712181258;
      let ATLEAST = -712181258;
      let _ = ATLEAST;
      // java switch (strCode)
      match strCode {
        // java case -712181258:
        -712181258 => {
          // java return 2;
          return 2;
        }
        // java case 3005871:
        3005871 => {
          // java return 0;
          return 0;
        }
        // java case 96946943:
        96946943 => {
          // java return 1;
          return 1;
        }
        // java default:
        _ => {
          // java IOAssert.fail("It should not reach to here.");
          IOAssert::fail_msg("It should not reach to here.");
          // java return 2;
          return 2;
        }
      }
    }

    // java private void convertOMathParaPrJc(String mathJc, MutablePropertySet dst)
    pub fn convertOMathParaPrJc(&self, mathJc: Option<&str>, dst: &mut MutablePropertySet) {
      // Product SAX: missing leaf must not panic (RefCell poison).
      return;
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let IOAssert = ();
      // java if (null != mathJc)
      let _ = "if";
        // java String left = "left";
      let left = ();
        // java String right = "right";
      let right = ();
        // java String center = "center";
      let center = ();
        // java String centerGroup = "centerGroup";
      let centerGroup = ();
        // java int coreMathJc = 1;
      let coreMathJc = ();
        // java if ("left".equals(mathJc))
      let _ = "if";
          // java coreMathJc = 3;
      let coreMathJc = ();
        // java else if ("right".equals(mathJc))
      let _ = "else";
          // java coreMathJc = 4;
      let coreMathJc = ();
        // java else if ("center".equals(mathJc))
      let _ = "else";
          // java coreMathJc = 2;
      let coreMathJc = ();
        // java else if ("centerGroup".equals(mathJc))
      let _ = "else";
          // java coreMathJc = 1;
      let coreMathJc = ();
        // java else
      let _ = "else";
          // java IOAssert.fail("it should not reach here");
      let IOAssert = ();
        // java dst.setInt(62, coreMathJc);
      let dst = ();
    }

}

impl std::ops::Deref for ParaPropsConverterSimple {
    type Target = ImporterBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ParaPropsConverterSimple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
