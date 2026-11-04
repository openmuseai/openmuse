#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! ★M-STAGE2 P1-D1 / 08-member-assembly-plan T2 —— **真实移植**。
//!
//! 原文件是手写 "Soft" 桩：用 `pub type KStylesPtr = *mut ();`（原始指针伪造）
//! 替代真实 `KStyles`，三个静态方法全部空实现 ⇒ 属 §"stub 消除" 目标。
//! 现按 `StyleListenerBase.java`（56 行）逐句真实移植，依赖 `KStyles.stylesInUse`
//! 与 `PropertySet.getInt(int,int)`（二者均已存在且已登记）。

use crate::cn::wps::moffice::writer::core::KStyles::KStyles;
use util::moffice::property::PropertySet::PropertySet;

pub struct StyleListenerBase {
    // java protected KStyles mStyles;
    pub(crate) mStyles: Option<KStyles>,
}

impl StyleListenerBase {
    // java protected static final int INVALID_PROP_ID = -1;
    pub const INVALID_PROP_ID: i32 = -1;

    // java protected StyleListenerBase(KStyles styles)
    pub fn StyleListenerBase(styles: Option<KStyles>) -> Self {
      // java this.mStyles = styles;
      StyleListenerBase { mStyles: styles }
    }

    // java public static void addStyleRef(KStyles styles, PropertySet data, Integer styleId, int propId)
    pub fn addStyleRef(styles: &mut KStyles, data: Option<&PropertySet>, styleId: Option<i32>, propId: i32) {
      // java if (null != data)
      if let Some(data) = data {
         // java int style = null != styleId ? styleId : data.getInt(propId, -1);
         let style: i32 = if styleId.is_some() { styleId.unwrap() } else { data.getInt__int_int(propId, -1) };
         // java if (-1 != style)
         if -1 != style {
            // java Integer cnt = (Integer)styles.stylesInUse.get(style);
            let mut cnt: i32 = styles.stylesInUse.get(&style).copied().unwrap_or(0);
            // java cnt = null == cnt ? 0 : cnt;
            // rust: no-stmt
            // java cnt = cnt + 1;
            cnt = cnt + 1;
            // java styles.stylesInUse.put(style, cnt);
            styles.stylesInUse.insert(style, cnt);
         }

      }
    }

    // java public static void removeStyleRef(KStyles styles, PropertySet data, Integer styleId, int propId)
    pub fn removeStyleRef(styles: &mut KStyles, data: Option<&PropertySet>, styleId: Option<i32>, propId: i32) {
      // java if (null != data)
      if let Some(data) = data {
         // java int style = null != styleId ? styleId : data.getInt(propId, -1);
         let style: i32 = if styleId.is_some() { styleId.unwrap() } else { data.getInt__int_int(propId, -1) };
         // java if (-1 != style)
         if -1 != style {
            // java Integer cnt = (Integer)styles.stylesInUse.get(style);
            let cnt: Option<i32> = styles.stylesInUse.get(&style).copied();
            // java if (null == cnt)
            if cnt.is_none() {
               // java return;
               return;
            }

            // java cnt = cnt - 1;
            let cnt: i32 = cnt.unwrap() - 1;
            // java styles.stylesInUse.put(style, cnt);
            styles.stylesInUse.insert(style, cnt);
         }

      }
    }

    // java public static void changeStyleRef(KStyles styles, PropertySet oldProp, PropertySet newProp, int propId)
    pub fn changeStyleRef(styles: &mut KStyles, oldProp: Option<&PropertySet>, newProp: Option<&PropertySet>, propId: i32) {
      // java int oldStyle = null == oldProp ? -1 : oldProp.getInt(propId, -1);
      let oldStyle: i32 = if oldProp.is_none() { -1 } else { oldProp.unwrap().getInt__int_int(propId, -1) };
      // java int newStyle = null == newProp ? -1 : newProp.getInt(propId, -1);
      let newStyle: i32 = if newProp.is_none() { -1 } else { newProp.unwrap().getInt__int_int(propId, -1) };
      // java if (oldStyle != newStyle)
      if oldStyle != newStyle {
         // java if (-1 != oldStyle)
         if -1 != oldStyle {
            // java removeStyleRef(styles, oldProp, oldStyle, propId);
            StyleListenerBase::removeStyleRef(styles, oldProp, Some(oldStyle), propId);
         }

         // java if (-1 != newStyle)
         if -1 != newStyle {
            // java addStyleRef(styles, newProp, newStyle, propId);
            StyleListenerBase::addStyleRef(styles, newProp, Some(newStyle), propId);
         }

      }
    }
}
