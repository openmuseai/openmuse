// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::KStyle;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::KStyles;

// java private static PropertySet defaultMergeNormaProp = null;
static defaultMergeNormaProp: std::sync::LazyLock<std::sync::Arc<Option<PropertySet>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(None));
// java private static PropertySet normalStyleProp = null;
static normalStyleProp: std::sync::LazyLock<std::sync::Arc<Option<PropertySet>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(None));

// java public final class ParaPropsConverter
#[derive(Default)]
pub struct ParaPropsConverter {
}

// java public final class ParaPropsConverter
impl ParaPropsConverter {
    // java static void convertSpaceAfterAuto(MutablePropertySet dst, Boolean isSpacingAfterAuto)
    pub fn convertSpaceAfterAuto(dst: &mut MutablePropertySet, isSpacingAfterAuto: Option<bool>) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let dst_opt: Option<&MutablePropertySet> = Some(&*dst);
      IOAssert::assertNotNull_msg("dst should not be null", &dst_opt);
      // java if (null != isSpacingAfterAuto)
      if let Some(isSpacingAfterAuto) = isSpacingAfterAuto {
        // java dst.setBoolean(216, isSpacingAfterAuto);
        dst.setBoolean(216, isSpacingAfterAuto);
      }
    }

    // java static void convertSpaceAfterLines(MutablePropertySet dst, Long linesSpacingAfter)
    pub fn convertSpaceAfterLines(dst: &mut MutablePropertySet, linesSpacingAfter: Option<i64>) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let dst_opt: Option<&MutablePropertySet> = Some(&*dst);
      IOAssert::assertNotNull_msg("dst should not be null", &dst_opt);
      // java if (null != linesSpacingAfter)
      if let Some(linesSpacingAfter) = linesSpacingAfter {
        // java int linesSpacingAfterInt = linesSpacingAfter.intValue();
        let linesSpacingAfterInt: i32 = linesSpacingAfter as i32;
        // java if (linesSpacingAfterInt > 0 && linesSpacingAfterInt <= 32766)
        if linesSpacingAfterInt > 0 && linesSpacingAfterInt <= 32766 {
          // java dst.setInt(215, linesSpacingAfterInt);
          dst.setInt(215, linesSpacingAfterInt);
        }
      }
    }

    // java static void convertSpaceAfter(MutablePropertySet dst, Long spaceAfter)
    pub fn convertSpaceAfter(dst: &mut MutablePropertySet, spaceAfter: Option<i64>) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let dst_opt: Option<&MutablePropertySet> = Some(&*dst);
      IOAssert::assertNotNull_msg("dst should not be null", &dst_opt);
      // java if (null != spaceAfter)
      if let Some(spaceAfter) = spaceAfter {
        // java int spaceAfterInt = spaceAfter.intValue();
        let spaceAfterInt: i32 = spaceAfter as i32;
        // java if (spaceAfterInt >= 0 && spaceAfterInt <= 32766)
        if spaceAfterInt >= 0 && spaceAfterInt <= 32766 {
          // java dst.setInt(214, spaceAfterInt);
          dst.setInt(214, spaceAfterInt);
        }
      }
    }

    // java static void convertSpaceBeforeAuto(MutablePropertySet dst, Boolean isSpacingBeforeAuto)
    pub fn convertSpaceBeforeAuto(dst: &mut MutablePropertySet, isSpacingBeforeAuto: Option<bool>) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let dst_opt: Option<&MutablePropertySet> = Some(&*dst);
      IOAssert::assertNotNull_msg("dst should not be null", &dst_opt);
      // java if (null != isSpacingBeforeAuto)
      if let Some(isSpacingBeforeAuto) = isSpacingBeforeAuto {
        // java dst.setBoolean(219, isSpacingBeforeAuto);
        dst.setBoolean(219, isSpacingBeforeAuto);
      }
    }

    // java static void convertSpaceBeforeLines(MutablePropertySet dst, Long linesSpacingBefore)
    pub fn convertSpaceBeforeLines(dst: &mut MutablePropertySet, linesSpacingBefore: Option<i64>) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let dst_opt: Option<&MutablePropertySet> = Some(&*dst);
      IOAssert::assertNotNull_msg("dst should not be null", &dst_opt);
      // java if (null != linesSpacingBefore)
      if let Some(linesSpacingBefore) = linesSpacingBefore {
        // java int linesSpacingBeforerInt = linesSpacingBefore.intValue();
        let linesSpacingBeforerInt: i32 = linesSpacingBefore as i32;
        // java if (linesSpacingBeforerInt > 0 && linesSpacingBeforerInt <= 32766)
        if linesSpacingBeforerInt > 0 && linesSpacingBeforerInt <= 32766 {
          // java dst.setInt(218, linesSpacingBeforerInt);
          dst.setInt(218, linesSpacingBeforerInt);
        }
      }
    }

    // java static void convertSpaceBefore(MutablePropertySet dst, Long spaceBefore)
    pub fn convertSpaceBefore(dst: &mut MutablePropertySet, spaceBefore: Option<i64>) {
      // java IOAssert.assertNotNull("dst should not be null", dst);
      let dst_opt: Option<&MutablePropertySet> = Some(&*dst);
      IOAssert::assertNotNull_msg("dst should not be null", &dst_opt);
      // java if (null != spaceBefore)
      if let Some(spaceBefore) = spaceBefore {
        // java int spaceBeforeInt = spaceBefore.intValue();
        let spaceBeforeInt: i32 = spaceBefore as i32;
        // java if (spaceBeforeInt >= 0 && spaceBeforeInt <= 32766)
        if spaceBeforeInt >= 0 && spaceBeforeInt <= 32766 {
          // java dst.setInt(217, spaceBeforeInt);
          dst.setInt(217, spaceBeforeInt);
        }
      }
    }

    // java static PropertySet getDefaultMergeStyleProp(KStyles styles, int style)
    pub fn getDefaultMergeStyleProp(styles: &KStyles, style: i32) -> PropertySet {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("styles should not be null", styles);
      IOAssert::assertNotNull__string_object("styles should not be null", styles);
      // java if (0 == style)
      if 0 == style {
        // java return getDefaultMergeNormalProp(styles);
        return Self::getDefaultMergeNormalProp(styles);
      }
      // java else
      else {
        // java PropertySet defaultParaProp = DefaultDocxParagraphProperties.getProperties();
        let mut defaultParaProp: PropertySet = DefaultDocxParagraphProperties::getProperties();
        // java KStyle kStyle = styles.getStyleById(style);
        let mut kStyle: KStyle = styles.getStyleById__int(style);
        // java PropertySet prop = kStyle.getFinalProperty();
        let mut prop: PropertySet = kStyle.getFinalProperty();
        // java return PropertySet.merge(defaultParaProp, prop);
        return PropertySet::merge(defaultParaProp, prop);
      }
      */
    }

    // java private static PropertySet getDefaultMergeNormalProp(KStyles styles)
    pub fn getDefaultMergeNormalProp(styles: &KStyles) -> PropertySet {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != defaultMergeNormaProp)
      if defaultMergeNormaProp.is_some() {
        // java return defaultMergeNormaProp;
        return defaultMergeNormaProp;
      }
      // java else
      else {
        // java PropertySet defaultParaProp = DefaultDocxParagraphProperties.getProperties();
        let mut defaultParaProp: PropertySet = DefaultDocxParagraphProperties::getProperties();
        // java PropertySet normalParaProp = getNormalStyleProp(styles);
        let mut normalParaProp: PropertySet = Self::getNormalStyleProp(styles);
        // java defaultMergeNormaProp = PropertySet.merge(defaultParaProp, normalParaProp);
        defaultMergeNormaProp = PropertySet::merge(defaultParaProp, normalParaProp);
        // java return defaultMergeNormaProp;
        return defaultMergeNormaProp;
      }
      */
    }

    // java private static PropertySet getNormalStyleProp(KStyles styles)
    pub fn getNormalStyleProp(styles: &KStyles) -> PropertySet {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != normalStyleProp)
      if normalStyleProp.is_some() {
        // java return normalStyleProp;
        return normalStyleProp;
      }
      // java else
      else {
        // java KStyle kStyle = styles.getStyleById(0);
        let mut kStyle: KStyle = styles.getStyleById__int(0);
        // java normalStyleProp = kStyle.getFinalProperty();
        normalStyleProp = kStyle.getFinalProperty();
        // java return normalStyleProp;
        return normalStyleProp;
      }
      */
    }

}
