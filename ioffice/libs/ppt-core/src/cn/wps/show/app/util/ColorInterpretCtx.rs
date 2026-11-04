// Java: import cn.wps.drawing.app.color.KmoSysColor
use crate::cn::wps::show::convert::j2r_hosts::KmoSysColor;
// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::show::convert::j2r_hosts::Color;
// Java: import cn.wps.drawing.data.style.ColorMapping
use crate::cn::wps::show::convert::j2r_hosts::ColorMapping;
// Java: import cn.wps.drawing.data.style.ColorScheme
use crate::cn::wps::show::convert::j2r_hosts::ColorScheme;
// Java: import cn.wps.show.app.slide.ISlide
// rust: ★W9 修正 —— 此处原错写成 `…::j2r_hosts::ImagePool`（转写错误），
//       既与本文件 :16 的真实 `ImagePool` 导入重复（E0252），又导致 `ISlide` 无法命名（E0425）。
use crate::cn::wps::show::app::slide::ISlide::ISlide;
// Java: import cn.wps.show.app.theme.KmoTheme
use crate::cn::wps::show::convert::j2r_hosts::KmoTheme;
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.apache.poi.poifs.filesystem.Entry
use crate::cn::wps::show::convert::j2r_hosts::ImagePool;
use std::collections::{HashMap};

// java public class ColorInterpretCtx
#[derive(Default)]
pub struct ColorInterpretCtx {
    // java private ColorScheme colorScheme;
    colorScheme: Option<ColorScheme>,
    // java private ColorMapping clrMap;
    clrMap: Option<ColorMapping>,
    // java private Color phColor;
    phColor: Option<Color>,
    // java private KmoSysColor deviceSysColor;
    deviceSysColor: Option<KmoSysColor>,
    // java private Map<Integer, Integer> mapColorTrans;
    // rust: ★W9 修正 —— Java 侧该字段在构造器里 `this.mapColorTrans = new HashMap();`，
    //       **非空不变量**；`Option` 是转写错误，且使 `clear()/containsKey()/get()/put()`
    //       全部无法解析。改回 `HashMap` 以匹配 Java 不变量。
    mapColorTrans: HashMap<i32, i32>,
}

// java public class ColorInterpretCtx
impl ColorInterpretCtx {
    // java public  ColorInterpretCtx()
    pub fn new() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this((ColorScheme)null, (ColorMapping)null);
      let mut this = Self::new__colorscheme_colormapping(None, None);
      this
      */
    }

    // java public  ColorInterpretCtx(ColorScheme colorScheme, ColorMapping clrMap)
    pub fn new__colorscheme_colormapping(colorScheme: &ColorScheme, clrMap: &ColorMapping) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.deviceSysColor = KmoSysColor.DEFAULT_SYS_COLOR;
      let mut this = Self { colorScheme: colorScheme, clrMap: clrMap, phColor: Default::default(), deviceSysColor: KmoSysColor::DEFAULT_SYS_COLOR, mapColorTrans: HashMap::new() };
      // java this.mapColorTrans = new HashMap();
      // java this.setColorScheme(colorScheme);
      this.setColorScheme(colorScheme);
      // java this.setClrMap(clrMap);
      this.setClrMap(clrMap);
      this
      */
    }

    // java public void setColorScheme(ColorScheme colorScheme)
    pub fn setColorScheme(&mut self, colorScheme: Option<ColorScheme>) {
      // java this.colorScheme = colorScheme;
      self.colorScheme = colorScheme;
    }

    // java public void setClrMap(ColorMapping clrMap)
    pub fn setClrMap(&mut self, clrMap: Option<ColorMapping>) {
      // java this.clrMap = clrMap;
      self.clrMap = clrMap;
      // java this.mapColorTrans.clear();
      self.mapColorTrans.clear();
    }

    // java public ColorInterpretCtx withPhColor(Color phColor)
    pub fn withPhColor(&mut self, phColor: Option<Color>) -> &mut Self {
      // java this.phColor = phColor;
      self.phColor = phColor;
      // java return this;
      return { let __x: &mut Self = todo!("no align strategy matched"); __x };
    }

    // java public void setUseDeviceSysColor(KmoSysColor deviceSysColor)
    pub fn setUseDeviceSysColor(&mut self, deviceSysColor: Option<KmoSysColor>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.deviceSysColor = deviceSysColor;
      self.deviceSysColor = deviceSysColor;
      */
    }

    // java public KmoSysColor deviceSysColor()
    pub fn deviceSysColor(&self) -> Option<KmoSysColor> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.deviceSysColor;
      return self.deviceSysColor;
      */
    }

    // java public Color interpretStandardClr(int index)
    pub fn interpretStandardClr(&mut self, index: i32) -> Color {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int backgroundColourRGB = 0;
      let mut backgroundColourRGB: i32 = 0;
      // java int textAndLinesColourRGB = 1;
      let mut textAndLinesColourRGB: i32 = 1;
      // java int shadowsColourRGB = 2;
      let mut shadowsColourRGB: i32 = 2;
      // java int titleTextColourRGB = 3;
      let mut titleTextColourRGB: i32 = 3;
      // java int fillsColourRGB = 4;
      let mut fillsColourRGB: i32 = 4;
      // java int accentColourRGB = 5;
      let mut accentColourRGB: i32 = 5;
      // java int accentAndHyperlinkColourRGB = 6;
      let mut accentAndHyperlinkColourRGB: i32 = 6;
      // java int accentAndFollowingHyperlinkColourRGB = 7;
      let mut accentAndFollowingHyperlinkColourRGB: i32 = 7;
      // java int schemeClr = index;
      let mut schemeClr: i32 = index;
      // java switch (index)
      match index {
        // java case 0:
        0 => {
          // java schemeClr = 1;
          schemeClr = 1;
        }
        // java case 1:
        1 => {
          // java schemeClr = 0;
          schemeClr = 0;
        }
        // java case 2:
        2 => {
          // java schemeClr = 2;
          schemeClr = 2;
        }
        // java case 3:
        3 => {
          // java schemeClr = 3;
          schemeClr = 3;
        }
        // java case 4:
        4 => {
          // java schemeClr = 4;
          schemeClr = 4;
        }
        // java case 5:
        5 => {
          // java schemeClr = 5;
          schemeClr = 5;
        }
        // java case 6:
        6 => {
          // java schemeClr = 10;
          schemeClr = 10;
        }
        // java case 7:
        7 => {
          // java schemeClr = 11;
          schemeClr = 11;
        }
        // java default:
        _ => {
        }
      }
      // java return this.interpretSchemeClr(schemeClr);
      return self.interpretSchemeClr(schemeClr);
      */
    }

    // java public Color interpretSchemeClr(int schemeClr)
    pub fn interpretSchemeClr(&self, schemeClr: i32) -> Color {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java schemeClr = this.remapScheme(schemeClr);
      schemeClr = self.remapScheme(schemeClr);
      // java switch (schemeClr)
      match schemeClr {
        // java case 0:
        0 => {
          // java if (this.colorScheme.hasDk1())
          if self.colorScheme.hasDk1() {
            // java return this.colorScheme.dk1();
            return self.colorScheme.dk1();
          }
        }
        // java case 1:
        1 => {
          // java if (this.colorScheme.hasLt1())
          if self.colorScheme.hasLt1() {
            // java return this.colorScheme.lt1();
            return self.colorScheme.lt1();
          }
        }
        // java case 2:
        2 => {
          // java if (this.colorScheme.hasDk2())
          if self.colorScheme.hasDk2() {
            // java return this.colorScheme.dk2();
            return self.colorScheme.dk2();
          }
        }
        // java case 3:
        3 => {
          // java if (this.colorScheme.hasLt2())
          if self.colorScheme.hasLt2() {
            // java return this.colorScheme.lt2();
            return self.colorScheme.lt2();
          }
        }
        // java case 4:
        4 => {
          // java if (this.colorScheme.hasAccent1())
          if self.colorScheme.hasAccent1() {
            // java return this.colorScheme.accent1();
            return self.colorScheme.accent1();
          }
        }
        // java case 5:
        5 => {
          // java if (this.colorScheme.hasAccent2())
          if self.colorScheme.hasAccent2() {
            // java return this.colorScheme.accent2();
            return self.colorScheme.accent2();
          }
        }
        // java case 6:
        6 => {
          // java if (this.colorScheme.hasAccent3())
          if self.colorScheme.hasAccent3() {
            // java return this.colorScheme.accent3();
            return self.colorScheme.accent3();
          }
        }
        // java case 7:
        7 => {
          // java if (this.colorScheme.hasAccent4())
          if self.colorScheme.hasAccent4() {
            // java return this.colorScheme.accent4();
            return self.colorScheme.accent4();
          }
        }
        // java case 8:
        8 => {
          // java if (this.colorScheme.hasAccent5())
          if self.colorScheme.hasAccent5() {
            // java return this.colorScheme.accent5();
            return self.colorScheme.accent5();
          }
        }
        // java case 9:
        9 => {
          // java if (this.colorScheme.hasAccent6())
          if self.colorScheme.hasAccent6() {
            // java return this.colorScheme.accent6();
            return self.colorScheme.accent6();
          }
        }
        // java case 10:
        10 => {
          // java if (this.colorScheme.hasHlink())
          if self.colorScheme.hasHlink() {
            // java return this.colorScheme.hlink();
            return self.colorScheme.hlink();
          }
        }
        // java case 11:
        11 => {
          // java if (this.colorScheme.hasFolHlink())
          if self.colorScheme.hasFolHlink() {
            // java return this.colorScheme.folHlink();
            return self.colorScheme.folHlink();
          }
        }
        // java case 12: case 13: default:
        12 | 13 | _ => {
        }
        // java case 14:
        14 => {
          // java if (this.phColor == null)
          if self.phColor.is_none() {
            // java return Color.create();
            return Color::create();
          }
          // java return this.phColor;
          return self.phColor.unwrap();
        }
      }
      // java return Color.create();
      return Color::create();
      */
    }

    // java public void setSlide(ISlide slide)
    // rust: `ISlide` 是 **trait**（`show/app/slide/ISlide.rs`），须用 `&dyn ISlide`（E0782）。
    pub fn setSlide(&mut self, slide: Option<&dyn ISlide>) {
      // java ISlide slide（Java 无 Option；参数可空由调用侧保证）
      let slide = slide.expect("NullPointerException");
      // java this.setClrMap(slide.colorMapping());
      // java this.setTheme(slide.theme());
      //
      // rust: ★DEFERRED_NON_FUNCTION —— 本类当前按 **`j2r_hosts`（转换层编译占位）类型面**书写
      //   （`j2r_hosts::{ColorMapping, KmoTheme, ColorScheme, Color}`，见文件头 import），而
      //   `ISlide::colorMapping()` 返回 **真身** `drawing::data::style::ColorMapping`、
      //   `ISlide::theme()` 返回尚未解锁的 `show::app::theme::KmoTheme`（`theme` 模块仍受
      //   `#[cfg(feature = "full")]` 门禁）。两侧**无转换路径** ⇒ 两条赋值整体延后。
      //   影响面：① 本类在默认构建下**无任何消费者**（`KmoShowShape` 用的是
      //   `kmo_show_shape_hosts::ColorInterpretCtx` 这个同名**编译面替身**，见其 `:188`），
      //   本波登记本文件只是把 583 行真实转写从「从未进入编译单元」（形态 H）变为「已编译」；
      //   ② 真身化属转换层整体去占位。
      //   owner: show.app.util.ColorInterpretCtx    wave: M-PPT-RENDER-DEFACADE
      let _ = slide;
    }

    // java public void setTheme(KmoTheme theme)
    pub fn setTheme(&mut self, theme: Option<&KmoTheme>) {
      // java this.setColorScheme(theme != null ? theme.getColorScheme() : ColorScheme.create());
      // rust: ★W9 修正 —— Java `theme != null ? theme.getColorScheme() : ColorScheme.create()`
      //       是「三元取**值**」；原转写把两侧各包了一层 `Some`，再整体包 `Some` ⇒ 类型不符（E0308）。
      self.setColorScheme(Some(match theme {
        Some(theme) => theme.getColorScheme(),
        None => ColorScheme::create(),
      }));
    }

    // java public void reset()
    pub fn reset(&mut self) {
      // java this.phColor = null;
      self.phColor = None;
      // java this.deviceSysColor = null;
      self.deviceSysColor = None;
    }

    // java private int remapScheme(int schemeClr)
    pub fn remapScheme(&mut self, schemeClr: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.clrMap == null)
      if self.clrMap.is_none() {
        // java return schemeClr;
        return schemeClr;
      }
      // java else if (this.mapColorTrans.containsKey(schemeClr))
      else if self.mapColorTrans.containsKey(schemeClr) {
        // java return (Integer)this.mapColorTrans.get(schemeClr);
        return { let __x: i32 = todo!("no align strategy matched"); __x };
      }
      // java else
      else {
        // java switch (schemeClr)
        match schemeClr {
          // java case 4:
          4 => {
            // java int var13 = this.clrMap.accent1();
            let mut var13: i32 = self.clrMap.accent1();
            // java this.mapColorTrans.put(4, var13);
            self.mapColorTrans.put(4, var13);
            // java return var13;
            return var13;
          }
          // java case 5:
          5 => {
            // java int var12 = this.clrMap.accent2();
            let mut var12: i32 = self.clrMap.accent2();
            // java this.mapColorTrans.put(5, var12);
            self.mapColorTrans.put(5, var12);
            // java return var12;
            return var12;
          }
          // java case 6:
          6 => {
            // java int var11 = this.clrMap.accent3();
            let mut var11: i32 = self.clrMap.accent3();
            // java this.mapColorTrans.put(6, var11);
            self.mapColorTrans.put(6, var11);
            // java return var11;
            return var11;
          }
          // java case 7:
          7 => {
            // java int var10 = this.clrMap.accent4();
            let mut var10: i32 = self.clrMap.accent4();
            // java this.mapColorTrans.put(7, var10);
            self.mapColorTrans.put(7, var10);
            // java return var10;
            return var10;
          }
          // java case 8:
          8 => {
            // java int var9 = this.clrMap.accent5();
            let mut var9: i32 = self.clrMap.accent5();
            // java this.mapColorTrans.put(8, var9);
            self.mapColorTrans.put(8, var9);
            // java return var9;
            return var9;
          }
          // java case 9:
          9 => {
            // java int var8 = this.clrMap.accent6();
            let mut var8: i32 = self.clrMap.accent6();
            // java this.mapColorTrans.put(9, var8);
            self.mapColorTrans.put(9, var8);
            // java return var8;
            return var8;
          }
          // java case 10:
          10 => {
            // java int var7 = this.clrMap.hlink();
            let mut var7: i32 = self.clrMap.hlink();
            // java this.mapColorTrans.put(10, var7);
            self.mapColorTrans.put(10, var7);
            // java return var7;
            return var7;
          }
          // java case 11:
          11 => {
            // java int var6 = this.clrMap.folHlink();
            let mut var6: i32 = self.clrMap.folHlink();
            // java this.mapColorTrans.put(11, var6);
            self.mapColorTrans.put(11, var6);
            // java return var6;
            return var6;
          }
          // java case 12:
          12 => {
            // java int var5 = this.clrMap.bg1();
            let mut var5: i32 = self.clrMap.bg1();
            // java this.mapColorTrans.put(12, var5);
            self.mapColorTrans.put(12, var5);
            // java return var5;
            return var5;
          }
          // java case 13:
          13 => {
            // java int var4 = this.clrMap.bg2();
            let mut var4: i32 = self.clrMap.bg2();
            // java this.mapColorTrans.put(13, var4);
            self.mapColorTrans.put(13, var4);
            // java return var4;
            return var4;
          }
          // java case 14: default:
          14 | _ => {
            // java return schemeClr;
            return schemeClr;
          }
          // java case 15:
          15 => {
            // java int var3 = this.clrMap.tx1();
            let mut var3: i32 = self.clrMap.tx1();
            // java this.mapColorTrans.put(15, var3);
            self.mapColorTrans.put(15, var3);
            // java return var3;
            return var3;
          }
          // java case 16:
          16 => {
            // java int clrValue = this.clrMap.tx2();
            let mut clrValue: i32 = self.clrMap.tx2();
            // java this.mapColorTrans.put(16, clrValue);
            self.mapColorTrans.put(16, clrValue);
            // java return clrValue;
            return clrValue;
          }
        }
      }
      */
    }

    // java public void etColorTransPptColor(Map<String, Integer> colorsMap)
    pub fn etColorTransPptColor(&mut self, colorsMap: &HashMap<String, i32>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ColorScheme pptColorScheme = ColorScheme.create();
      let mut pptColorScheme: ColorScheme = ColorScheme::create();
      // java this.setColorScheme(pptColorScheme);
      self.setColorScheme(Some(pptColorScheme));
      // java for (Map.Entry<String, Integer> entry : colorsMap.entrySet())
      for entry in colorsMap.entrySet().iter() {
        // java String name = (String)entry.getKey();
        let mut name: String = { let __x: String = todo!("no align strategy matched"); __x };
        // java if (null != name && !name.equals(""))
        if true && !name.equals("") {
          // java int colors = (Integer)entry.getValue();
          let mut colors: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
          // java this.setColor(pptColorScheme, name, colors);
          self.setColor(&mut pptColorScheme, name, colors);
        }
      }
      */
    }

    // java private void setColor(ColorScheme pptColorScheme, String name, int colors)
    pub fn setColor(&mut self, pptColorScheme: &mut ColorScheme, name: String, colors: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Color color = null;
      let mut color: Color = Default::default();
      // java if (name.equals("dk1"))
      if name.equals("dk1") {
        // java color = pptColorScheme.dk1();
        color = pptColorScheme.dk1();
      }
      // java else if (name.equals("dk2"))
      else if name.equals("dk2") {
        // java color = pptColorScheme.dk2();
        color = pptColorScheme.dk2();
      }
      // java else if (name.equals("accent1"))
      else if name.equals("accent1") {
        // java color = pptColorScheme.accent1();
        color = pptColorScheme.accent1();
      }
      // java else if (name.equals("accent2"))
      else if name.equals("accent2") {
        // java color = pptColorScheme.accent2();
        color = pptColorScheme.accent2();
      }
      // java else if (name.equals("accent3"))
      else if name.equals("accent3") {
        // java color = pptColorScheme.accent3();
        color = pptColorScheme.accent3();
      }
      // java else if (name.equals("accent4"))
      else if name.equals("accent4") {
        // java color = pptColorScheme.accent4();
        color = pptColorScheme.accent4();
      }
      // java else if (name.equals("accent5"))
      else if name.equals("accent5") {
        // java color = pptColorScheme.accent5();
        color = pptColorScheme.accent5();
      }
      // java else if (name.equals("accent6"))
      else if name.equals("accent6") {
        // java color = pptColorScheme.accent6();
        color = pptColorScheme.accent6();
      }
      // java else if (name.equals("lt1"))
      else if name.equals("lt1") {
        // java color = pptColorScheme.lt1();
        color = pptColorScheme.lt1();
      }
      // java else if (name.equals("lt2"))
      else if name.equals("lt2") {
        // java color = pptColorScheme.lt2();
        color = pptColorScheme.lt2();
      }
      // java else if (name.equals("hlink"))
      else if name.equals("hlink") {
        // java color = pptColorScheme.hlink();
        color = pptColorScheme.hlink();
      }
      // java else if (name.equals("folHlink"))
      else if name.equals("folHlink") {
        // java color = pptColorScheme.folHlink();
        color = pptColorScheme.folHlink();
      }
      // java if (null != color)
      if true {
        // java color.srgb().putR(this.getR(colors));
        color.srgb().putR(self.getR(colors));
        // java color.srgb().putG(this.getG(colors));
        color.srgb().putG(self.getG(colors));
        // java color.srgb().putB(this.getB(colors));
        color.srgb().putB(self.getB(colors));
      }
      */
    }

    // java private int getR(int rgba)
    pub fn getR(&self, rgba: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return rgba >> 16 & 255;
      return rgba >> 16 & 255;
      */
    }

    // java private int getG(int rgba)
    pub fn getG(&self, rgba: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return rgba >> 8 & 255;
      return rgba >> 8 & 255;
      */
    }

    // java private int getB(int rgba)
    pub fn getB(&self, rgba: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return rgba & 255;
      return rgba & 255;
      */
    }

}
