// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import java.util.HashSet
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::collections::{HashSet};
// Java: import java.lang.Math
use stubs::java::lang::Math::*;
// Java: import java.lang.System
use stubs::java::lang::System::*;
// Java: import cn.wps.moffice.writer.data.PropertyId
use crate::cn::wps::moffice::writer::data::PropertyId::*;
use std::sync::Arc;

// java private static HashSet<Integer> _special_rid = null;
static _special_rid: std::sync::LazyLock<std::sync::Mutex<Option<HashSet<i32>>>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
// java private static HashSet<Integer> _notFollow_rid = null;
static _notFollow_rid: std::sync::LazyLock<std::sync::Mutex<Option<HashSet<i32>>>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
// java private static HashSet<Integer> _special_multiLanguage_rid = null;
static _special_multiLanguage_rid: std::sync::LazyLock<std::sync::Mutex<Option<HashSet<i32>>>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
// java private static HashSet<Integer> _special_run_rid = null;
static _special_run_rid: std::sync::LazyLock<std::sync::Mutex<Option<HashSet<i32>>>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
// java private static HashSet<Integer> _special_pid = null;
static _special_pid: std::sync::LazyLock<std::sync::Mutex<Option<HashSet<i32>>>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
// java private static HashSet<Integer> _notFollow_pid = null;
static _notFollow_pid: std::sync::LazyLock<std::sync::Mutex<Option<HashSet<i32>>>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
// java private static HashSet<Integer> _notFollow_sid = null;
static _notFollow_sid: std::sync::LazyLock<std::sync::Mutex<Option<HashSet<i32>>>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// java public class PropertyUtil
#[derive(Default)]
pub struct PropertyUtil {
}

// rust: ★Java `PropertyId` 是 **interface**（602 个隐式 `public static final` 常量），
//   Rust 忠实建模为 `trait PropertyId` + `const` 项。Java 接口字段由实现者继承 ⇒
//   此处 `impl PropertyId for PropertyUtil`，并以 `<PropertyUtil as PropertyId>::X` 取值。
impl PropertyId for PropertyUtil {}

// java public class PropertyUtil
impl PropertyUtil {
    // java private  PropertyUtil()
    pub fn PropertyUtil() -> Self {
      Self {}
    }

    // java public static PropertySet getFinalRunFormat(PropertySet prop)
    pub fn getFinalRunFormat(prop: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("prop should not be null!", prop);
      Assert::assertNotNull__string_object(&"prop should not be null!", &Some(&prop));
      // java Revision revision = (Revision)prop.getValue(48);
      let revision: Option<&Revision> = prop.getValue(48).and_then(|v| v.as_ref().as_any().downcast_ref::<Revision>());
      // java if (null != revision)
      if revision.is_some() {
        // java PropertySet originalProp = removeRunRevision(prop);
        let mut originalProp: PropertySet = Self::removeRunRevision(&prop);
        // java return getMergeRunRevisionProp(originalProp, revision);
        return Self::getMergeRunRevisionProp(&originalProp, revision.unwrap());
      }
      // java else
      else {
        // java return prop;
        return prop.clone();
      }
    }

    // java private static PropertySet removeRunRevision(PropertySet prop)
    pub fn removeRunRevision(prop: &PropertySet) -> PropertySet {
      // java MutablePropertySet mutaProp = new MutablePropertySet(prop);
      let mut mutaProp: MutablePropertySet = MutablePropertySet::new_from_ps(prop);
      // java mutaProp.remove(48);
      mutaProp.remove(48);
      // java mutaProp.remove(47);
      mutaProp.remove(47);
      // java return mutaProp.freeze();
      return mutaProp.freeze();
    }

    // java public static PropertySet getFinalParagraphFormat(PropertySet prop)
    pub fn getFinalParagraphFormat(prop: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("prop should not be null!", prop);
      Assert::assertNotNull__string_object(&"prop should not be null!", &Some(&prop));
      // java Revision revision = (Revision)prop.getValue(237);
      let revision: Option<&Revision> = prop.getValue(237).and_then(|v| v.as_ref().as_any().downcast_ref::<Revision>());
      // java if (null != revision)
      if revision.is_some() {
        // java PropertySet originalProp = removeParaRevision(prop);
        let mut originalProp: PropertySet = Self::removeParaRevision(&prop);
        // java return getMergeParagraphRevisionProp(originalProp, revision);
        return Self::getMergeParagraphRevisionProp(&originalProp, revision.unwrap());
      }
      // java else
      else {
        // java return prop;
        return prop.clone();
      }
    }

    // java public static PropertySet getFinalTableFormat(KStyles styles, PropertySet prop)
    pub fn getFinalTableFormat(styles: &KStyles, prop: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("prop should not be null!", prop);
      Assert::assertNotNull__string_object(&"prop should not be null!", &Some(&prop));
      // java Revision revision = (Revision)prop.getValue(363);
      let revision: Option<&Revision> = prop.getValue(363).and_then(|v| v.as_ref().as_any().downcast_ref::<Revision>());
      // java if (null != revision)
      if revision.is_some() {
        // java PropertySet orignalProp = removeTableRevision(prop);
        let mut orignalProp: PropertySet = Self::removeTableRevision(&prop);
        // java return getMergeTableRevisionProp(styles, orignalProp, revision);
        return Self::getMergeTableRevisionProp(&styles, &orignalProp, revision.unwrap());
      }
      // java else
      else {
        // java return prop;
        return prop.clone();
      }
    }

    // java private static PropertySet removeTableRevision(PropertySet prop)
    pub fn removeTableRevision(prop: &PropertySet) -> PropertySet {
      // java MutablePropertySet mutaProp = new MutablePropertySet(prop);
      let mut mutaProp: MutablePropertySet = MutablePropertySet::new_from_ps(prop);
      // java mutaProp.remove(363);
      mutaProp.remove(363);
      // java mutaProp.remove(364);
      mutaProp.remove(364);
      // java return mutaProp.freeze();
      return mutaProp.freeze();
    }

    // java private static PropertySet removeParaRevision(PropertySet prop)
    pub fn removeParaRevision(prop: &PropertySet) -> PropertySet {
      // java MutablePropertySet mutaProp = new MutablePropertySet(prop);
      let mut mutaProp: MutablePropertySet = MutablePropertySet::new_from_ps(prop);
      // java mutaProp.remove(237);
      mutaProp.remove(237);
      // java mutaProp.remove(238);
      mutaProp.remove(238);
      // java return mutaProp.freeze();
      return mutaProp.freeze();
    }

    // java public static PropertySet getFinalNormalRunFormat(PropertySet prop)
    pub fn getFinalNormalRunFormat(prop: &PropertySet) -> PropertySet {
      // java PropertySet normalProp = getNormalRunFormat(prop);
      let mut normalProp: PropertySet = Self::getNormalRunFormat(Some(prop)).expect("PropertyUtil: asserted non-null");
      // java Assert.assertNotNull("normalProp should not be null !", normalProp);
      Assert::assertNotNull__string_object(&"normalProp should not be null !", &Some(&normalProp));
      // java Revision revision = (Revision)prop.getValue(48);
      let revision: Option<&Revision> = prop.getValue(48).and_then(|v| v.as_ref().as_any().downcast_ref::<Revision>());
      // java return null != revision ? getMergeRunRevisionProp(normalProp, revision) : normalProp;
      return if revision.is_some() { Self::getMergeRunRevisionProp(&normalProp, revision.unwrap()) } else { normalProp };
    }

    // java public static PropertySet getFinalNotFollowRunFormat(PropertySet prop)
    pub fn getFinalNotFollowRunFormat(prop: &PropertySet) -> PropertySet {
      // java PropertySet notFollowProp = removeNotFollowRunFormat(prop);
      let mut notFollowProp: PropertySet = Self::removeNotFollowRunFormat(&prop);
      // java Assert.assertNotNull("notFollowProp should not be null !", notFollowProp);
      Assert::assertNotNull__string_object(&"notFollowProp should not be null !", &Some(&notFollowProp));
      // java Revision revision = (Revision)prop.getValue(48);
      let revision: Option<&Revision> = prop.getValue(48).and_then(|v| v.as_ref().as_any().downcast_ref::<Revision>());
      // java return null != revision ? getMergeRunRevisionProp(notFollowProp, revision) : notFollowProp;
      return if revision.is_some() { Self::getMergeRunRevisionProp(&notFollowProp, revision.unwrap()) } else { notFollowProp };
    }

    // java public static PropertySet getFinalNormalParageraphFormat(PropertySet prop)
    pub fn getFinalNormalParageraphFormat(prop: &PropertySet) -> PropertySet {
      // java PropertySet normalProp = getNormalParagraphFormat(prop);
      let mut normalProp: PropertySet = Self::getNormalParagraphFormat(Some(prop)).expect("PropertyUtil: asserted non-null");
      // java Assert.assertNotNull("normalProp should not be null !", normalProp);
      Assert::assertNotNull__string_object(&"normalProp should not be null !", &Some(&normalProp));
      // java Revision revision = (Revision)prop.getValue(237);
      let revision: Option<&Revision> = prop.getValue(237).and_then(|v| v.as_ref().as_any().downcast_ref::<Revision>());
      // java return null != revision ? getMergeParagraphRevisionProp(normalProp, revision) : normalProp;
      return if revision.is_some() { Self::getMergeParagraphRevisionProp(&normalProp, revision.unwrap()) } else { normalProp };
    }

    // java public static PropertySet getFinalNormalSectionFormat(PropertySet prop)
    pub fn getFinalNormalSectionFormat(prop: &PropertySet) -> PropertySet {
      // java PropertySet normalProp = getNormalSectionFormat(prop);
      let mut normalProp: PropertySet = Self::getNormalSectionFormat(Some(prop)).expect("PropertyUtil: asserted non-null");
      // java Revision revision = (Revision)prop.getValue(727);
      let revision: Option<&Revision> = prop.getValue(727).and_then(|v| v.as_ref().as_any().downcast_ref::<Revision>());
      // java return null != revision ? getMergedSectionRevisionProp(normalProp, revision) : normalProp;
      return if revision.is_some() { Self::getMergedSectionRevisionProp(&normalProp, revision.unwrap()) } else { normalProp };
    }

    // java private static PropertySet getMergeRunRevisionProp(PropertySet normalProp, Revision revision)
    pub fn getMergeRunRevisionProp(normalProp: &PropertySet, revision: &Revision) -> PropertySet {
      // java Assert.assertNotNull("normalProp should not be null !", normalProp);
      Assert::assertNotNull__string_object(&"normalProp should not be null !", &Some(&normalProp));
      // java Assert.assertNotNull("revision should not be null !", revision);
      Assert::assertNotNull__string_object(&"revision should not be null !", &Some(&revision));
      // java PropertySet changeFormat = revision.getProperty();
      let mut changeFormat: PropertySet = revision.getProperty().expect("PropertyUtil: revision.getProperty asserted non-null");
      // java Assert.assertNotNull("changeFormat should not be null!", changeFormat);
      Assert::assertNotNull__string_object(&"changeFormat should not be null!", &Some(&changeFormat));
      // java Integer runStyle = (Integer)changeFormat.getValue(2);
      let runStyle: Option<i32> = changeFormat.getValue(2).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
      // java return null != runStyle ? changeFormat : PropertySet.compose(normalProp, changeFormat, 2);
      return if runStyle.is_some() { changeFormat.clone() } else { PropertySet::compose(&normalProp, &changeFormat, 2) };
    }

    // java private static PropertySet getMergeParagraphRevisionProp(PropertySet normalProp, Revision revision)
    pub fn getMergeParagraphRevisionProp(normalProp: &PropertySet, revision: &Revision) -> PropertySet {
      // java Assert.assertNotNull("normalProp should not be null !", normalProp);
      Assert::assertNotNull__string_object(&"normalProp should not be null !", &Some(&normalProp));
      // java Assert.assertNotNull("revision should not be null !", revision);
      Assert::assertNotNull__string_object(&"revision should not be null !", &Some(&revision));
      // java PropertySet changeFormat = revision.getProperty();
      let mut changeFormat: PropertySet = revision.getProperty().expect("PropertyUtil: revision.getProperty asserted non-null");
      // java Assert.assertNotNull("changeFormat should not be null!", changeFormat);
      Assert::assertNotNull__string_object(&"changeFormat should not be null!", &Some(&changeFormat));
      // java Integer paraStyle = (Integer)changeFormat.getValue(191);
      let paraStyle: Option<i32> = changeFormat.getValue(191).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
      // java return null != paraStyle ? changeFormat : PropertySet.compose(normalProp, changeFormat, 2);
      return if paraStyle.is_some() { changeFormat.clone() } else { PropertySet::compose(&normalProp, &changeFormat, 2) };
    }

    // java private static PropertySet getMergedSectionRevisionProp(PropertySet normalProp, Revision revision)
    pub fn getMergedSectionRevisionProp(normalProp: &PropertySet, revision: &Revision) -> PropertySet {
      // java Assert.assertNotNull("normalProp should not be null !", normalProp);
      Assert::assertNotNull__string_object(&"normalProp should not be null !", &Some(&normalProp));
      // java Assert.assertNotNull("revision should not be null !", revision);
      Assert::assertNotNull__string_object(&"revision should not be null !", &Some(&revision));
      // java PropertySet changeFormat = revision.getProperty();
      let mut changeFormat: PropertySet = revision.getProperty().expect("PropertyUtil: revision.getProperty asserted non-null");
      // java Assert.assertNotNull("changeFormat should not be null!", changeFormat);
      Assert::assertNotNull__string_object(&"changeFormat should not be null!", &Some(&changeFormat));
      // java return PropertySet.compose(normalProp, changeFormat, 2);
      return PropertySet::compose(&normalProp, &changeFormat, 2);
    }

    // java public static PropertySet getMergeTableRevisionProp(KStyles styles, PropertySet normalProp, Revision revision)
    pub fn getMergeTableRevisionProp(styles: &KStyles, normalProp: &PropertySet, revision: &Revision) -> PropertySet {
      // java Assert.assertNotNull("normalProp should not be null !", normalProp);
      Assert::assertNotNull__string_object(&"normalProp should not be null !", &Some(&normalProp));
      // java Assert.assertNotNull("revision should not be null !", revision);
      Assert::assertNotNull__string_object(&"revision should not be null !", &Some(&revision));
      // java PropertySet changeFormat = revision.getProperty();
      let mut changeFormat: PropertySet = revision.getProperty().expect("PropertyUtil: revision.getProperty asserted non-null");
      // java Assert.assertNotNull("changeFormat should not be null!", changeFormat);
      Assert::assertNotNull__string_object(&"changeFormat should not be null!", &Some(&changeFormat));
      // java Integer tableStyle = (Integer)changeFormat.getValue(292);
      let tableStyle: Option<i32> = changeFormat.getValue(292).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
      // java if (null != tableStyle)
      if tableStyle.is_some() {
        // java return convertTableFormatChange(styles, normalProp, changeFormat, tableStyle);
        return Self::convertTableFormatChange(&styles, &normalProp, &changeFormat, tableStyle.unwrap());
      }
      // java else
      else {
        // java MutablePropertySet propertySet = new MutablePropertySet(normalProp);
        let mut propertySet: MutablePropertySet = MutablePropertySet::new_from_ps(normalProp);
        // java propertySet.add(changeFormat);
        propertySet.add(&changeFormat);
        // java propertySet.add(getMergedBrcCv(normalProp, changeFormat));
        propertySet.add(&Self::getMergedBrcCv(&normalProp, &changeFormat));
        // java return propertySet.freeze();
        return propertySet.freeze();
      }
    }

    // java public static PropertySet getMergedBrcCv(PropertySet normalProp, PropertySet revisionProp)
    pub fn getMergedBrcCv(normalProp: &PropertySet, revisionProp: &PropertySet) -> PropertySet {
      // java MutablePropertySet propertySet = new MutablePropertySet();
      let mut propertySet: MutablePropertySet = MutablePropertySet::new();
      // java mergeBrcCv(propertySet, normalProp, revisionProp, 323);
      Self::mergeBrcCv(&mut propertySet, &normalProp, &revisionProp, 323);
      // java mergeBrcCv(propertySet, normalProp, revisionProp, 324);
      Self::mergeBrcCv(&mut propertySet, &normalProp, &revisionProp, 324);
      // java mergeBrcCv(propertySet, normalProp, revisionProp, 325);
      Self::mergeBrcCv(&mut propertySet, &normalProp, &revisionProp, 325);
      // java mergeBrcCv(propertySet, normalProp, revisionProp, 326);
      Self::mergeBrcCv(&mut propertySet, &normalProp, &revisionProp, 326);
      // java return propertySet.freeze();
      return propertySet.freeze();
    }

    // java private static void mergeBrcCv(MutablePropertySet propertySet, PropertySet normalProp, PropertySet revisionProp, int id)
    pub fn mergeBrcCv(propertySet: &mut MutablePropertySet, normalProp: &PropertySet, revisionProp: &PropertySet, id: i32) {
      // java int[] normalCv = (int[])normalProp.getValue(id);
      let mut normalCv: Option<Vec<i32>> = normalProp.getValue(id).and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<i32>>().cloned());
      // java int[] revisionCv = (int[])revisionProp.getValue(id);
      let mut revisionCv: Option<Vec<i32>> = revisionProp.getValue(id).and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<i32>>().cloned());
      // java if (revisionCv == null)
      if revisionCv.is_none() {
        // java propertySet.setValue(id, normalCv);
        propertySet.setValue(id, normalCv.map(|v| Arc::new(v) as PropertyValue));
      }
      // java else if (normalCv == null)
      else if normalCv.is_none() {
        // java propertySet.setValue(id, revisionCv);
        propertySet.setValue(id, revisionCv.map(|v| Arc::new(v) as PropertyValue));
      }
      // java else
      else {
        // java int n = Math.max(normalCv.length, revisionCv.length);
        let mut n: i32 = Math::max((normalCv.as_ref().unwrap().len() as i32), (revisionCv.as_ref().unwrap().len() as i32));
        // java int[] resultCv = new int[n];
        let mut resultCv: Vec<i32> = vec![Default::default(); (n) as usize];
        // java System.arraycopy(normalCv, 0, resultCv, 0, normalCv.length);
        System::arraycopy(normalCv.as_ref().unwrap().as_slice(), 0, &mut resultCv, 0, normalCv.as_ref().unwrap().len());
        // java System.arraycopy(revisionCv, 0, resultCv, 0, revisionCv.length);
        System::arraycopy(revisionCv.as_ref().unwrap().as_slice(), 0, &mut resultCv, 0, revisionCv.as_ref().unwrap().len());
        // java propertySet.setValue(id, resultCv);
        propertySet.setValue(id, Some(Arc::new(resultCv)));
      }
    }

    // java private static PropertySet convertTableFormatChange(KStyles styles, PropertySet originalPAP, PropertySet revisionProp, Integer styleValue)
    pub fn convertTableFormatChange(styles: &KStyles, originalPAP: &PropertySet, revisionProp: &PropertySet, styleValue: i32) -> PropertySet {
      // java Assert.assertNotNull("styles should not be null!", styles);
      Assert::assertNotNull__string_object(&"styles should not be null!", &Some(&styles));
      // java Assert.assertNotNull("originalPAP should not be null!", originalPAP);
      Assert::assertNotNull__string_object(&"originalPAP should not be null!", &Some(&originalPAP));
      // java Assert.assertNotNull("revisionProp should not be null!", revisionProp);
      Assert::assertNotNull__string_object(&"revisionProp should not be null!", &Some(&revisionProp));
      // java Assert.assertNotNull("styleValue should not be null!", styleValue);
      Assert::assertNotNull__string_object(&"styleValue should not be null!", &Some(&styleValue));
      // java if (!revisionProp.contains(306))
      if !revisionProp.contains(306) {
        // java KStyle style = styles.getStyleById(styleValue);
        let mut style: KStyle = styles.getStyleById__int(styleValue).expect("PropertyUtil: style id must resolve");
        // java PropertySet tableStyleProp = style.getFinalProperty();
        let mut tableStyleProp: PropertySet = style.getFinalProperty().expect("PropertyUtil: asserted non-null");
        // java PropertySet originalFinalPAP = PropertySet.merge(originalPAP, tableStyleProp);
        let mut originalFinalPAP: PropertySet = PropertySet::merge(&originalPAP, &tableStyleProp);
        // java return PropertySet.merge(originalFinalPAP, revisionProp);
        return PropertySet::merge(&originalFinalPAP, &revisionProp);
      }
      // java else
      else {
        // java return setSpecialProp(originalPAP, revisionProp);
        return Self::setSpecialProp(&originalPAP, &revisionProp);
      }
    }

    // java public static PropertySet setSpecialProp(PropertySet originalProp, PropertySet revisionProp)
    pub fn setSpecialProp(originalProp: &PropertySet, revisionProp: &PropertySet) -> PropertySet {
      // java MutablePropertySet muRevision = new MutablePropertySet(revisionProp);
      let mut muRevision: MutablePropertySet = MutablePropertySet::new_from_ps(revisionProp);
      // java muRevision.addLimit(originalProp, 191, 291);
      muRevision.addLimit(originalProp, 191, 291);
      // java return muRevision.freeze();
      return muRevision.freeze();
    }

    // java public static PropertySet getNormalRunFormat(PropertySet prop)
    pub fn getNormalRunFormat(prop: Option<&PropertySet>) -> Option<PropertySet> {
      // java if (prop == null)
      if prop.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java HashSet<Integer> specialRids = fill_special_rid();
        let mut specialRids: HashSet<i32> = Self::fill_special_rid();
        // java HashSet<Integer> notFollowRsids = fill_notFollow_rid();
        let mut notFollowRsids: HashSet<i32> = Self::fill_notFollow_rid();
        // java Assert.assertNotNull("specialRids should not be null !", specialRids);
        Assert::assertNotNull__string_object(&"specialRids should not be null !", &Some(&specialRids));
        // java Assert.assertNotNull("notFollowRsids should not be null !", notFollowRsids);
        Assert::assertNotNull__string_object(&"notFollowRsids should not be null !", &Some(&notFollowRsids));
        // java HashSet<Integer> rids = new HashSet();
        let mut rids: HashSet<i32> = HashSet::new();
        // java rids.addAll(specialRids);
        rids.extend(specialRids);
        // java rids.addAll(notFollowRsids);
        rids.extend(notFollowRsids);
        // java int[] ids = new int[prop.size()];
        let mut ids: Vec<i32> = vec![Default::default(); (prop.as_ref().map(|p| p.size()).unwrap_or(0)) as usize];
        // java Object[] values = new Object[prop.size()];
        let mut values: Vec<Option<PropertyValue>> = vec![None; (prop.as_ref().map(|p| p.size()).unwrap_or(0)) as usize];
        // java prop.getAll(ids, values);
        prop.as_ref().unwrap().getAll(&mut ids, Some(&mut values));
        // java MutablePropertySet r = new MutablePropertySet();
        let mut r: MutablePropertySet = MutablePropertySet::new();
        let mut i: i32 = 0;
        // java for(int i = 0; i < ids.length; ++i)
        while i < (ids.len() as i32) {
          // java if (!rids.contains(ids[i]))
          if !rids.contains(&ids[(i) as usize]) {
            // java r.setValue(ids[i], values[i]);
            r.setValue(ids[(i) as usize], values[(i) as usize].clone());
          }
        }
        // java return r.freeze();
        return Some(r.freeze());
      }
    }

    // java public static PropertySet clearMultiLanguageProperty(PropertySet prop)
    pub fn clearMultiLanguageProperty(prop: &PropertySet) -> PropertySet {
      // java int[] ids = new int[prop.size()];
      let mut ids: Vec<i32> = vec![Default::default(); (prop.size()) as usize];
      // java Object[] values = new Object[prop.size()];
      let mut values: Vec<Option<PropertyValue>> = vec![None; (prop.size()) as usize];
      // java prop.getAll(ids, values);
      prop.getAll(&mut ids, Some(&mut values));
      // java HashSet<Integer> rids = new HashSet();
      let mut rids: HashSet<i32> = HashSet::new();
      // java HashSet<Integer> multiRids = fill_multiLanguage_rid();
      let mut multiRids: HashSet<i32> = Self::fill_multiLanguage_rid();
      // java rids.addAll(multiRids);
      rids.extend(multiRids);
      // java MutablePropertySet r = new MutablePropertySet();
      let mut r: MutablePropertySet = MutablePropertySet::new();
      let mut i: i32 = 0;
      // java for(int i = 0; i < ids.length; ++i)
      while i < (ids.len() as i32) {
        // java if (!rids.contains(ids[i]))
        if !rids.contains(&ids[(i) as usize]) {
          // java r.setValue(ids[i], values[i]);
          r.setValue(ids[(i) as usize], values[(i) as usize].clone());
        }
      }
      // java return r.freeze();
      return r.freeze();
    }

    // java public static PropertySet clearSpecicalRunProperty(PropertySet prop)
    pub fn clearSpecicalRunProperty(prop: &PropertySet) -> PropertySet {
      // java int[] ids = new int[prop.size()];
      let mut ids: Vec<i32> = vec![Default::default(); (prop.size()) as usize];
      // java Object[] values = new Object[prop.size()];
      let mut values: Vec<Option<PropertyValue>> = vec![None; (prop.size()) as usize];
      // java prop.getAll(ids, values);
      prop.getAll(&mut ids, Some(&mut values));
      // java HashSet<Integer> rids = new HashSet();
      let mut rids: HashSet<i32> = HashSet::new();
      // java HashSet<Integer> specialRunRids = fill_special_run_rid();
      let mut specialRunRids: HashSet<i32> = Self::fill_special_run_rid();
      // java rids.addAll(specialRunRids);
      rids.extend(specialRunRids);
      // java MutablePropertySet r = new MutablePropertySet();
      let mut r: MutablePropertySet = MutablePropertySet::new();
      let mut i: i32 = 0;
      // java for(int i = 0; i < ids.length; ++i)
      while i < (ids.len() as i32) {
        // java if (!rids.contains(ids[i]))
        if !rids.contains(&ids[(i) as usize]) {
          // java r.setValue(ids[i], values[i]);
          r.setValue(ids[(i) as usize], values[(i) as usize].clone());
        }
      }
      // java return r.freeze();
      return r.freeze();
    }

    // java private static PropertySet removeNotFollowRunFormat(PropertySet prop)
    pub fn removeNotFollowRunFormat(prop: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("prop should not be null!", prop);
      Assert::assertNotNull__string_object(&"prop should not be null!", &Some(&prop));
      // java HashSet<Integer> notFollowRsids = fill_notFollow_rid();
      let mut notFollowRsids: HashSet<i32> = Self::fill_notFollow_rid();
      // java Assert.assertNotNull("notFollowRsids should not be null !", notFollowRsids);
      Assert::assertNotNull__string_object(&"notFollowRsids should not be null !", &Some(&notFollowRsids));
      // java HashSet<Integer> rids = new HashSet();
      let mut rids: HashSet<i32> = HashSet::new();
      // java rids.addAll(notFollowRsids);
      rids.extend(notFollowRsids);
      // java int[] ids = new int[prop.size()];
      let mut ids: Vec<i32> = vec![Default::default(); (prop.size()) as usize];
      // java Object[] values = new Object[prop.size()];
      let mut values: Vec<Option<PropertyValue>> = vec![None; (prop.size()) as usize];
      // java prop.getAll(ids, values);
      prop.getAll(&mut ids, Some(&mut values));
      // java MutablePropertySet r = new MutablePropertySet();
      let mut r: MutablePropertySet = MutablePropertySet::new();
      let mut i: i32 = 0;
      // java for(int i = 0; i < ids.length; ++i)
      while i < (ids.len() as i32) {
        // java if (!rids.contains(ids[i]))
        if !rids.contains(&ids[(i) as usize]) {
          // java r.setValue(ids[i], values[i]);
          r.setValue(ids[(i) as usize], values[(i) as usize].clone());
        }
      }
      // java return r.freeze();
      return r.freeze();
    }

    // java private static synchronized HashSet<Integer> fill_multiLanguage_rid()
    pub fn fill_multiLanguage_rid() -> HashSet<i32> {
      // java if (_special_multiLanguage_rid == null)
      if _special_multiLanguage_rid.lock().unwrap().is_none() {
        // java _special_multiLanguage_rid = new HashSet();
        *_special_multiLanguage_rid.lock().unwrap() = Some(HashSet::new());
        // java for(int rid : PropertyId.special_multiLanguage_rid)
        for rid in <PropertyUtil as PropertyId>::special_multiLanguage_rid.iter() {
          // java _special_multiLanguage_rid.add(rid);
          _special_multiLanguage_rid.lock().unwrap().as_mut().unwrap().insert(*rid);
        }
      }
      // java return _special_multiLanguage_rid;
      return _special_multiLanguage_rid.lock().unwrap().clone().unwrap_or_default();
    }

    // java private static synchronized HashSet<Integer> fill_special_run_rid()
    pub fn fill_special_run_rid() -> HashSet<i32> {
      // java if (_special_run_rid == null)
      if _special_run_rid.lock().unwrap().is_none() {
        // java _special_run_rid = new HashSet();
        *_special_run_rid.lock().unwrap() = Some(HashSet::new());
        // java for(int rid : PropertyId.special_run_rid)
        for rid in <PropertyUtil as PropertyId>::special_run_rid.iter() {
          // java _special_run_rid.add(rid);
          _special_run_rid.lock().unwrap().as_mut().unwrap().insert(*rid);
        }
      }
      // java return _special_run_rid;
      return _special_run_rid.lock().unwrap().clone().unwrap_or_default();
    }

    // java private static synchronized HashSet<Integer> fill_notFollow_rid()
    pub fn fill_notFollow_rid() -> HashSet<i32> {
      // java if (_notFollow_rid == null)
      if _notFollow_rid.lock().unwrap().is_none() {
        // java _notFollow_rid = new HashSet();
        *_notFollow_rid.lock().unwrap() = Some(HashSet::new());
        // java for(int rid : PropertyId.notFollow_rid)
        for rid in <PropertyUtil as PropertyId>::notFollow_rid.iter() {
          // java _notFollow_rid.add(rid);
          _notFollow_rid.lock().unwrap().as_mut().unwrap().insert(*rid);
        }
      }
      // java return _notFollow_rid;
      return _notFollow_rid.lock().unwrap().clone().unwrap_or_default();
    }

    // java private static synchronized HashSet<Integer> fill_special_rid()
    pub fn fill_special_rid() -> HashSet<i32> {
      // java if (_special_rid == null)
      if _special_rid.lock().unwrap().is_none() {
        // java _special_rid = new HashSet();
        *_special_rid.lock().unwrap() = Some(HashSet::new());
        // java for(int rid : PropertyId.special_rid)
        for rid in <PropertyUtil as PropertyId>::special_rid.iter() {
          // java _special_rid.add(rid);
          _special_rid.lock().unwrap().as_mut().unwrap().insert(*rid);
        }
      }
      // java return _special_rid;
      return _special_rid.lock().unwrap().clone().unwrap_or_default();
    }

    // java public static MutablePropertySet getSpecialRunFormat(PropertySet prop)
    pub fn getSpecialRunFormat(prop: Option<&PropertySet>) -> Option<MutablePropertySet> {
      // java if (prop == null)
      if prop.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java HashSet<Integer> rids = fill_special_rid();
        let mut rids: HashSet<i32> = Self::fill_special_rid();
        // java MutablePropertySet r = new MutablePropertySet();
        let mut r: MutablePropertySet = MutablePropertySet::new();
        // java int[] ids = new int[prop.size()];
        let mut ids: Vec<i32> = vec![Default::default(); (prop.as_ref().map(|p| p.size()).unwrap_or(0)) as usize];
        // java Object[] values = new Object[prop.size()];
        let mut values: Vec<Option<PropertyValue>> = vec![None; (prop.as_ref().map(|p| p.size()).unwrap_or(0)) as usize];
        // java prop.getAll(ids, values);
        prop.as_ref().unwrap().getAll(&mut ids, Some(&mut values));
        let mut i: i32 = 0;
        // java for(int i = 0; i < ids.length; ++i)
        while i < (ids.len() as i32) {
          // java if (rids.contains(ids[i]))
          if rids.contains(&ids[(i) as usize]) {
            // java r.setValue(ids[i], values[i]);
            r.setValue(ids[(i) as usize], values[(i) as usize].clone());
          }
          i += 1;
        }
        // java return r;
        return Some(r);
      }
    }

    // java public static void maintainTableInfo(MutablePropertySet pa, PropertySet prop)
    pub fn maintainTableInfo(pa: &mut MutablePropertySet, prop: &PropertySet) {
      // java for(int id : PropertyId.special_table_pid)
      for id in <PropertyUtil as PropertyId>::special_table_pid.iter() {
        // java Object value = prop.getValue(id);
        let value: Option<PropertyValue> = prop.getValue(*id).cloned();
        // java if (null != value)
        if value.is_some() {
          // java pa.setValue(id, value);
          pa.setValue(*id, value);
        }
      }
    }

    // java public static void maintainMultiLanguageInfo(MutablePropertySet pa, PropertySet prop)
    pub fn maintainMultiLanguageInfo(pa: &mut MutablePropertySet, prop: &PropertySet) {
      // java for(int id : PropertyId.special_multiLanguage_rid)
      for id in <PropertyUtil as PropertyId>::special_multiLanguage_rid.iter() {
        // java Object value = prop.getValue(id);
        let value: Option<PropertyValue> = prop.getValue(*id).cloned();
        // java if (null != value)
        if value.is_some() {
          // java pa.setValue(id, value);
          pa.setValue(*id, value);
        }
      }
    }

    // java public static void maintainSpecicalRunProperty(MutablePropertySet pa, PropertySet prop)
    pub fn maintainSpecicalRunProperty(pa: &mut MutablePropertySet, prop: &PropertySet) {
      // java for(int id : PropertyId.special_run_rid)
      for id in <PropertyUtil as PropertyId>::special_run_rid.iter() {
        // java Object value = prop.getValue(id);
        let value: Option<PropertyValue> = prop.getValue(*id).cloned();
        // java if (null != value)
        if value.is_some() {
          // java pa.setValue(id, value);
          pa.setValue(*id, value);
        }
      }
    }

    // java public static synchronized PropertySet getSpecialParagraphFormat(PropertySet prop)
    pub fn getSpecialParagraphFormat(prop: &PropertySet) -> PropertySet {
      // java if (_special_pid == null)
      if _special_pid.lock().unwrap().is_none() {
        // java _special_pid = new HashSet();
        *_special_pid.lock().unwrap() = Some(HashSet::new());
        // java for(int pid : PropertyId.special_pid)
        for pid in <PropertyUtil as PropertyId>::special_pid.iter() {
          // java _special_pid.add(pid);
          _special_pid.lock().unwrap().as_mut().unwrap().insert(*pid);
        }
      }
      // java MutablePropertySet r = new MutablePropertySet();
      let mut r: MutablePropertySet = MutablePropertySet::new();
      // java int[] ids = new int[prop.size()];
      let mut ids: Vec<i32> = vec![Default::default(); (prop.size()) as usize];
      // java Object[] values = new Object[prop.size()];
      let mut values: Vec<Option<PropertyValue>> = vec![None; (prop.size()) as usize];
      // java prop.getAll(ids, values);
      prop.getAll(&mut ids, Some(&mut values));
      let mut i: i32 = 0;
      // java for(int i = 0; i < ids.length; ++i)
      while i < (ids.len() as i32) {
        // java if (_special_pid.contains(ids[i]))
        if _special_pid.lock().unwrap().as_ref().map(|h| h.contains(&ids[(i) as usize])).unwrap_or(false) {
          // java r.setValue(ids[i], values[i]);
          r.setValue(ids[(i) as usize], values[(i) as usize].clone());
        }
      }
      // java if (r.size() == 0)
      if (r.size() as i32) == 0 {
        // java return PropertySet.empty;
        return PropertySet::empty();
      }
      // java else
      else {
        // java return r.freeze();
        return r.freeze();
      }
    }

    // java public static PropertySet getNormalParagraphFormat(PropertySet prop)
    pub fn getNormalParagraphFormat(prop: Option<&PropertySet>) -> Option<PropertySet> {
      // java if (prop == null)
      if prop.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java PropertySet normalProp = prop;
        let mut normalProp: PropertySet = prop.unwrap().clone();
        // java if (!prop.isEmpty())
        if !prop.as_ref().unwrap().isEmpty() {
          // java normalProp = getParagraphFormatRemoveSpecial(prop);
          normalProp = Self::getParagraphFormatRemoveSpecial(prop.as_ref().unwrap());
        }
        // java Assert.assertNotNull("notSpecialProp should not be null!", normalProp);
        Assert::assertNotNull__string_object(&"notSpecialProp should not be null!", &Some(&normalProp));
        // java if (!normalProp.isEmpty())
        if !normalProp.isEmpty() {
          // java normalProp = getParagraphFormatRemoveNotFollow(normalProp);
          normalProp = Self::getParagraphFormatRemoveNotFollow(&normalProp);
        }
        // java return normalProp;
        return Some(normalProp);
      }
    }

    // java public static PropertySet getNormalSectionFormat(PropertySet prop)
    pub fn getNormalSectionFormat(prop: Option<&PropertySet>) -> Option<PropertySet> {
      // java if (prop == null)
      if prop.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java PropertySet normalProp = prop;
        let mut normalProp: PropertySet = prop.unwrap().clone();
        // java Assert.assertNotNull("notSpecialProp should not be null!", prop);
        Assert::assertNotNull__string_object(&"notSpecialProp should not be null!", &Some(&prop));
        // java if (!prop.isEmpty())
        if !prop.as_ref().unwrap().isEmpty() {
          // java normalProp = getSectionFormatRemoveNotFollow(prop);
          normalProp = Self::getSectionFormatRemoveNotFollow(prop.as_ref().unwrap());
        }
        // java return normalProp;
        return Some(normalProp);
      }
    }

    // java public static synchronized PropertySet getParagraphFormatRemoveSpecial(PropertySet prop)
    pub fn getParagraphFormatRemoveSpecial(prop: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("prop should not be null!", prop);
      Assert::assertNotNull__string_object(&"prop should not be null!", &Some(&prop));
      // java if (_special_pid == null)
      if _special_pid.lock().unwrap().is_none() {
        // java _special_pid = new HashSet();
        *_special_pid.lock().unwrap() = Some(HashSet::new());
        // java for(int pid : PropertyId.special_pid)
        for pid in <PropertyUtil as PropertyId>::special_pid.iter() {
          // java _special_pid.add(pid);
          _special_pid.lock().unwrap().as_mut().unwrap().insert(*pid);
        }
      }
      // java int[] ids = new int[prop.size()];
      let mut ids: Vec<i32> = vec![Default::default(); (prop.size()) as usize];
      // java Object[] values = new Object[prop.size()];
      let mut values: Vec<Option<PropertyValue>> = vec![None; (prop.size()) as usize];
      // java prop.getAll(ids, values);
      prop.getAll(&mut ids, Some(&mut values));
      // java MutablePropertySet r = new MutablePropertySet();
      let mut r: MutablePropertySet = MutablePropertySet::new();
      let mut i: i32 = 0;
      // java for(int i = 0; i < ids.length; ++i)
      while i < (ids.len() as i32) {
        // java if (!_special_pid.contains(ids[i]))
        if !_special_pid.lock().unwrap().as_ref().map(|h| h.contains(&ids[(i) as usize])).unwrap_or(false) {
          // java r.setValue(ids[i], values[i]);
          r.setValue(ids[(i) as usize], values[(i) as usize].clone());
        }
      }
      // java if (r.size() == 0)
      if (r.size() as i32) == 0 {
        // java return PropertySet.empty;
        return PropertySet::empty();
      }
      // java else
      else {
        // java return r.freeze();
        return r.freeze();
      }
    }

    // java private static synchronized PropertySet getParagraphFormatRemoveNotFollow(PropertySet prop)
    pub fn getParagraphFormatRemoveNotFollow(prop: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("prop should not be null!", prop);
      Assert::assertNotNull__string_object(&"prop should not be null!", &Some(&prop));
      // java if (_notFollow_pid == null)
      if _notFollow_pid.lock().unwrap().is_none() {
        // java _notFollow_pid = new HashSet();
        *_notFollow_pid.lock().unwrap() = Some(HashSet::new());
        // java for(int pid : PropertyId.notFollow_pid)
        for pid in <PropertyUtil as PropertyId>::notFollow_pid.iter() {
          // java _notFollow_pid.add(pid);
          _notFollow_pid.lock().unwrap().as_mut().unwrap().insert(*pid);
        }
      }
      // java int[] ids = new int[prop.size()];
      let mut ids: Vec<i32> = vec![Default::default(); (prop.size()) as usize];
      // java Object[] values = new Object[prop.size()];
      let mut values: Vec<Option<PropertyValue>> = vec![None; (prop.size()) as usize];
      // java prop.getAll(ids, values);
      prop.getAll(&mut ids, Some(&mut values));
      // java MutablePropertySet r = new MutablePropertySet();
      let mut r: MutablePropertySet = MutablePropertySet::new();
      let mut i: i32 = 0;
      // java for(int i = 0; i < ids.length; ++i)
      while i < (ids.len() as i32) {
        // java if (!_notFollow_pid.contains(ids[i]))
        if !_notFollow_pid.lock().unwrap().as_ref().map(|h| h.contains(&ids[(i) as usize])).unwrap_or(false) {
          // java r.setValue(ids[i], values[i]);
          r.setValue(ids[(i) as usize], values[(i) as usize].clone());
        }
      }
      // java if (r.size() == 0)
      if (r.size() as i32) == 0 {
        // java return PropertySet.empty;
        return PropertySet::empty();
      }
      // java else
      else {
        // java return r.freeze();
        return r.freeze();
      }
    }

    // java private static synchronized PropertySet getSectionFormatRemoveNotFollow(PropertySet prop)
    pub fn getSectionFormatRemoveNotFollow(prop: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("prop should not be null!", prop);
      Assert::assertNotNull__string_object(&"prop should not be null!", &Some(&prop));
      // java if (_notFollow_sid == null)
      if _notFollow_sid.lock().unwrap().is_none() {
        // java _notFollow_sid = new HashSet();
        *_notFollow_sid.lock().unwrap() = Some(HashSet::new());
        // java for(int pid : PropertyId.notFollow_sid)
        for pid in <PropertyUtil as PropertyId>::notFollow_sid.iter() {
          // java _notFollow_sid.add(pid);
          _notFollow_sid.lock().unwrap().as_mut().unwrap().insert(*pid);
        }
      }
      // java int[] ids = new int[prop.size()];
      let mut ids: Vec<i32> = vec![Default::default(); (prop.size()) as usize];
      // java Object[] values = new Object[prop.size()];
      let mut values: Vec<Option<PropertyValue>> = vec![None; (prop.size()) as usize];
      // java prop.getAll(ids, values);
      prop.getAll(&mut ids, Some(&mut values));
      // java MutablePropertySet r = new MutablePropertySet();
      let mut r: MutablePropertySet = MutablePropertySet::new();
      let mut i: i32 = 0;
      // java for(int i = 0; i < ids.length; ++i)
      while i < (ids.len() as i32) {
        // java if (!_notFollow_sid.contains(ids[i]))
        if !_notFollow_sid.lock().unwrap().as_ref().map(|h| h.contains(&ids[(i) as usize])).unwrap_or(false) {
          // java r.setValue(ids[i], values[i]);
          r.setValue(ids[(i) as usize], values[(i) as usize].clone());
        }
      }
      // java if (r.size() == 0)
      if (r.size() as i32) == 0 {
        // java return PropertySet.empty;
        return PropertySet::empty();
      }
      // java else
      else {
        // java return r.freeze();
        return r.freeze();
      }
    }

    // java public static synchronized PropertySet getParagraphFormatRemoveFramePr(PropertySet prop)
    pub fn getParagraphFormatRemoveFramePr(prop: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("prop should not be null!", prop);
      Assert::assertNotNull__string_object(&"prop should not be null!", &Some(&prop));
      // java HashSet<Integer> _framePr_pid = new HashSet();
      let mut _framePr_pid: HashSet<i32> = HashSet::new();
      // java for(int pid : PropertyId.framePr_pid)
      for pid in <PropertyUtil as PropertyId>::framePr_pid.iter() {
        // java _framePr_pid.add(pid);
        _framePr_pid.insert(*pid);
      }
      // java int[] ids = new int[prop.size()];
      let mut ids: Vec<i32> = vec![Default::default(); (prop.size()) as usize];
      // java Object[] values = new Object[prop.size()];
      let mut values: Vec<Option<PropertyValue>> = vec![None; (prop.size()) as usize];
      // java prop.getAll(ids, values);
      prop.getAll(&mut ids, Some(&mut values));
      // java MutablePropertySet r = new MutablePropertySet();
      let mut r: MutablePropertySet = MutablePropertySet::new();
      let mut i: i32 = 0;
      // java for(int i = 0; i < ids.length; ++i)
      while i < (ids.len() as i32) {
        // java if (!_framePr_pid.contains(ids[i]))
        if !_framePr_pid.contains(&ids[(i) as usize]) {
          // java r.setValue(ids[i], values[i]);
          r.setValue(ids[(i) as usize], values[(i) as usize].clone());
        }
      }
      // java if (r.size() == 0)
      if (r.size() as i32) == 0 {
        // java return PropertySet.empty;
        return PropertySet::empty();
      }
      // java else
      else {
        // java return r.freeze();
        return r.freeze();
      }
    }

    // java public static synchronized PropertySet getParagraphFormatForInsertBreak(PropertySet property)
    pub fn getParagraphFormatForInsertBreak(property: &PropertySet) -> PropertySet {
      // java PropertySet prop = getFinalNormalParageraphFormat(property);
      let mut prop: PropertySet = Self::getFinalNormalParageraphFormat(&property);
      // java return getParagraphFormatRemoveFramePr(prop);
      return Self::getParagraphFormatRemoveFramePr(&prop);
    }

}
