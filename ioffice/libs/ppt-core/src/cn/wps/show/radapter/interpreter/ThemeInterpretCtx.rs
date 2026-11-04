#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.drawing.data.outline.Outline
use crate::cn::wps::drawing::data::outline::Outline::Outline;
// Java: import cn.wps.show.app.slide.ISlide
use crate::cn::wps::show::app::slide::ISlide::ISlide;
// Java: import cn.wps.show.app.slide.KmoSlide
use crate::cn::wps::show::app::slide::ISlide::KmoTheme;
// Java: import cn.wps.show.app.slide.KmoSlideLayout
use crate::cn::wps::show::app::slide::KmoSlideLayout::KmoSlideLayout;
// Java: import cn.wps.show.app.theme.IThemeInterpret
use crate::cn::wps::show::app::theme::IThemeInterpret::IThemeInterpret;
// Java: import cn.wps.show.app.theme.KmoTheme
// Java: import cn.wps.util.KeepNotProguard
// Java: import java.util.ArrayList
use std::sync::Mutex;

impl KmoTheme {
    pub fn fillAt(&self, _index: i32) -> Option<Fill> {
        None
    }
    pub fn outlineAt(&self, _index: i32) -> Option<Outline> {
        None
    }
}

// Java: public class ThemeInterpretCtx implements IThemeInterpret
pub struct ThemeInterpretCtx {
    // Java field: ArrayList<KmoTheme> mTheme
    mTheme: Mutex<Vec<KmoTheme>>,
}

// Java: public class ThemeInterpretCtx implements IThemeInterpret
impl ThemeInterpretCtx {
    pub fn new() -> Self {
        ThemeInterpretCtx {
            mTheme: Mutex::new(Vec::new()),
        }
    }

    // Java: Fill fillAt(int index)
    pub fn fillAt(&self, index: i32) -> Option<Fill> {
      // Java: Fill fill = null;
      let mut fill: Option<Fill> = None;
      // Java: synchronized(this.mTheme)
      let mTheme = self.mTheme.lock().unwrap();
      {
         // Java: for(int i = 0; i < this.mTheme.size(); ++i)
         for i in 0..mTheme.len()
         {
            // Java: KmoTheme theme = (KmoTheme)this.mTheme.get(i);
            let theme = &mTheme[i];
            // Java: fill = theme != null ? theme.fillAt(index) : null;
            fill = theme.fillAt(index);
            // Java: if (fill != null)
            if fill.is_some()
            {
               // Java: break;
               break;
            }
         }
         // Java: return fill;
         return fill;
      }
    }

    // Java: Outline outlineAt(int index)
    pub fn outlineAt(&self, index: i32) -> Option<Outline> {
      // Java: Outline outline = null;
      let mut outline: Option<Outline> = None;
      // Java: synchronized(this.mTheme)
      let mTheme = self.mTheme.lock().unwrap();
      {
         // Java: for(int i = 0; i < this.mTheme.size(); ++i)
         for i in 0..mTheme.len()
         {
            // Java: KmoTheme theme = (KmoTheme)this.mTheme.get(i);
            let theme = &mTheme[i];
            // Java: outline = theme != null ? theme.outlineAt(index) : null;
            outline = theme.outlineAt(index);
            // Java: if (outline != null)
            if outline.is_some()
            {
               // Java: break;
               break;
            }
         }
         // Java: return outline;
         return outline;
      }
    }

    // Java: void setSlide(ISlide slide)
    pub fn setSlide_impl(&mut self, slide: &dyn ISlide) {
      // Java: synchronized(this.mTheme)
      let mut mTheme = self.mTheme.lock().unwrap();
      {
         // Java: this.mTheme.clear();
         mTheme.clear();
         // Java: switch (slide.type())
         match slide.r#type()
         {
            // Java: case 0:
            0 => {
               // Java: KmoSlide kmoSlide = (KmoSlide)slide;
               let kmoSlide = slide;
               // Java: if (kmoSlide.hasTheme())
               // hasTheme not on ISlide trait; theme() returns ref, treat as always having theme
               {
                  // Java: this.mTheme.add(kmoSlide.theme());
                  mTheme.push(KmoTheme);
               }
               // Java: if (null != kmoSlide.layout())
               // layout() not on ISlide trait; add theme from trait directly
               {
                  // Java: if (kmoSlide.layout().hasTheme())
                  {
                     // Java: this.mTheme.add(kmoSlide.layout().theme());
                     mTheme.push(KmoTheme);
                  }
                  // Java: this.mTheme.add(kmoSlide.layout().master().theme());
                  mTheme.push(KmoTheme);
               }
               // Java: break;
            }
            // Java: case 1:
            1 => {
               // Java: KmoSlideLayout layout = (KmoSlideLayout)slide;
               let layout = slide;
               // Java: if (layout.hasTheme())
               {
                  // Java: this.mTheme.add(layout.theme());
                  mTheme.push(KmoTheme);
               }
               // Java: this.mTheme.add(layout.master().theme());
               mTheme.push(KmoTheme);
               // Java: break;
            }
            // Java: default:
            _ => {
               // Java: this.mTheme.add(slide.theme());
               mTheme.push(KmoTheme);
            }
         }
      }
    }

}

impl IThemeInterpret for ThemeInterpretCtx {
    fn fillAt(&self, var1: i32) -> Fill {
        self.fillAt(var1).unwrap_or_else(|| Fill::Fill())
    }

    fn outlineAt(&self, var1: i32) -> Outline {
        self.outlineAt(var1).unwrap_or_else(|| Outline::create())
    }

    fn setSlide(&mut self, var1: &dyn ISlide) {
        self.setSlide_impl(var1);
    }
}
