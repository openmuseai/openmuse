#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.core.runtime.ISharedInfo
// Java: import cn.wps.core.runtime.Platform

// Java: public class DeltaTransform
pub struct DeltaTransform {
    // Java field: String LESHI_PHONE
    _leshi_phone: i32,
}

// Java: public class DeltaTransform
impl DeltaTransform {
    // Java: DeltaTransform()
    pub fn DeltaTransform() -> Self {
      // Java: super();
      Self { _leshi_phone: 0 }
    }

    // Java: static boolean transform(int[] out)
    pub fn transform(out: &mut [i32]) -> bool {
      // Java: int currentDelta = 0;
      let mut current_delta: i32 = 0;
      // Java: int previousDelta = 0;
      let mut previous_delta: i32 = 0;
      // Java: for(int index = 0; index < out.length; ++index)
      for index in 0..out.len()
      {
         // Java: int currentItem = out[index];
         let current_item = out[index];
         // Java: out[index] = out[index] + previousDelta - currentDelta * 2;
         out[index] = out[index] + previous_delta - current_delta * 2;
         // Java: previousDelta = currentDelta;
         previous_delta = current_delta;
         // Java: currentDelta = currentItem;
         current_delta = current_item;
      }
      // Java: return true;
      true
    }

    // Java: static boolean inverseTransform(int[] out)
    pub fn inverseTransform(out: &mut [i32]) -> bool {
      // Java: int currentDelta = 0;
      let mut current_delta: i32 = 0;
      // Java: int previousDelta = 0;
      let mut previous_delta: i32 = 0;
      // Java: for(int index = 0; index < out.length; ++index)
      for index in 0..out.len()
      {
         // Java: if (isLeshiDevice())
         if Self::isLeshiDevice()
         {
            // Java: System.out.println("乐视手机！");
         }
         // Java: int delta = currentDelta * 2 - previousDelta + out[index];
         let delta = current_delta * 2 - previous_delta + out[index];
         // Java: previousDelta = currentDelta;
         previous_delta = current_delta;
         // Java: currentDelta = delta;
         current_delta = delta;
         // Java: out[index] = delta;
         out[index] = delta;
      }
      // Java: return true;
      true
    }

    // Java: static boolean isLeshiDevice()
    pub fn isLeshiDevice() -> bool {
      // Java: ISharedInfo sharedInfo = Platform.getSharedInfo();
      // Java: return sharedInfo == null ? false : sharedInfo.isLeshiDevice();
      false
    }

}
