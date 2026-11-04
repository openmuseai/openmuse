use std::collections::HashMap;
// Java: import cn.wps.libs.inkml.InkMLSettingListener
use crate::cn::wps::libs::inkml::InkMLSettingListener::InkMLSettingListener;
// Java: import com.hp.hpl.inkml.impl.Brush
// Java: import java.util.HashMap
// Java: import java.util.Map

// Java: public class TraceSetting
pub struct TraceSetting {
    // Java field: String tip
    pub(crate) tip: String,
    // Java field: Map<String, Integer> colors
    pub(crate) colors: HashMap<String, i32>,
    // Java field: Map<String, Float> strokeWidths
    pub(crate) strokeWidths: HashMap<String, f32>,
    // Java field: InkMLSettingListener listener
    listener: Option<Box<dyn InkMLSettingListener>>,
    // Java field: float minStrokeWidth
    pub(crate) minStrokeWidth: f32,
}

// Java: public class TraceSetting
impl TraceSetting {
    // Java: TraceSetting()
    pub fn TraceSetting() -> Self {
      // Java: super();
      Self {
          tip: String::new(),
          colors: HashMap::new(),
          strokeWidths: HashMap::new(),
          listener: None,
          minStrokeWidth: 0.4,
      }
    }

    // Java: void setTip(String tip)
    pub fn setTip(&mut self, tip: &str) {
      // Java: if (tip != null && !tip.isEmpty() && !this.tip.equals(tip))
      if !tip.is_empty() && self.tip != tip
      {
         // Java: this.tip = tip;
         self.tip = tip.to_string();
         // Java: if (this.listener != null)
         if let Some(ref listener) = self.listener
         {
            // Java: this.listener.onTipChanged(tip);
            listener.onTipChanged(tip);
         }
      }
    }

    // Java: String getTip()
    pub fn getTip(&self) -> &str {
      // Java: return this.tip;
      &self.tip
    }

    // Java: void setColor(int color)
    pub fn setColor(&mut self, color: i32) {
      // Java: this.colors.put(this.tip, color);
      self.colors.insert(self.tip.clone(), color);
    }

    // Java: int getColor()
    pub fn getColor(&self) -> i32 {
      // Java: return this.colors.containsKey(this.tip) ? (Integer)this.colors.get(this.tip) : -16777216;
      *self.colors.get(&self.tip).unwrap_or(&-16777216)
    }

    // Java: void setStrokeWidth(float width)
    pub fn setStrokeWidth(&mut self, width: f32) {
      // Java: this.strokeWidths.put(this.tip, width);
      self.strokeWidths.insert(self.tip.clone(), width);
    }

    // Java: float getStrokeWidth()
    pub fn getStrokeWidth(&self) -> f32 {
      // Java: return this.strokeWidths.containsKey(this.tip) ? (Float)this.strokeWidths.get(this.tip) : 0.75F;
      *self.strokeWidths.get(&self.tip).unwrap_or(&0.75)
    }

    // Java: static float getDefaultMinStrokeWidth()
    pub fn getDefaultMinStrokeWidth() -> f32 {
      // Java: return 0.4F;
      0.4
    }

    // Java: float getMinStrokeWidth()
    pub fn getMinStrokeWidth(&self) -> f32 {
      // Java: return this.minStrokeWidth;
      self.minStrokeWidth
    }

    // Java: void setMinStrokeWidth(float width)
    pub fn setMinStrokeWidth(&mut self, width: f32) {
      // Java: this.minStrokeWidth = width;
      self.minStrokeWidth = width;
    }

    // Java: boolean isWritingPen()
    pub fn isWritingPen(&self) -> bool {
      // Java: return "TIP_WRITING".equals(this.tip);
      self.tip == "TIP_WRITING"
    }

    // Java: boolean isHighlight()
    pub fn isHighlight(&self) -> bool {
      // Java: return "TIP_HIGHLIGHTER".equals(this.tip);
      self.tip == "TIP_HIGHLIGHTER"
    }

    // Java: boolean isEraser()
    pub fn isEraser(&self) -> bool {
      // Java: return "TIP_ERASER".equals(this.tip);
      self.tip == "TIP_ERASER"
    }

    // Java: void setListener(InkMLSettingListener listener)
    pub fn setListener(&mut self, listener: Option<Box<dyn InkMLSettingListener>>) {
      // Java: this.listener = listener;
      self.listener = listener;
    }

}
