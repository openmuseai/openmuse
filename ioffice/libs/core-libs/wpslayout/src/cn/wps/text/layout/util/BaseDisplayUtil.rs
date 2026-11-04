#![allow(dead_code, unused_variables, non_snake_case)]

use std::sync::{OnceLock, Mutex};

// Java: import android.content.Context
struct Context;
// Java: import android.util.DisplayMetrics
struct DisplayMetrics;
// Java: import android.util.TypedValue
struct TypedValue;
// Java: import android.view.WindowManager
struct WindowManager;
// Java: import cn.wps.core.runtime.ISharedInfo
struct ISharedInfo;
// Java: import cn.wps.core.runtime.Platform
struct Platform;

// Java field: float mDip
static mDip: OnceLock<Mutex<f32>> = OnceLock::new();
// Java field: DisplayMetrics metrics
static metrics: OnceLock<Mutex<i32>> = OnceLock::new();

// Java: public class BaseDisplayUtil
pub struct BaseDisplayUtil {
}

// Java: public class BaseDisplayUtil
impl BaseDisplayUtil {
    // Java: BaseDisplayUtil()
    pub fn new(&self) {
      // Java: super();

    }

    // Java: static int getDisplayWidth(Context context)
    pub fn getDisplayWidth(&self, _context: i32) -> i32 {
      // Java: return getDisplayWidthAndHeight(context)[0];

      0
    }

    // Java: static int getDisplayHeight(Context context)
    pub fn getDisplayHeight(&self, _context: i32) -> i32 {
      // Java: return getDisplayWidthAndHeight(context)[1];

      0
    }

    // Java: static int[] getDisplayWidthAndHeight(Context context)
    pub fn getDisplayWidthAndHeight(&self, _context: i32) -> Vec<i32> {
      // Java: ISharedInfo sharedInfo = Platform.getSharedInfo();

      // Java: if (null != sharedInfo)

      {
         // Java: int[] result = sharedInfo.getDisplayWidthAndHeight(context);

         // Java: if (result.length == 2)

         {
            // Java: return result;

         }
      }
      // Java: DisplayMetrics metrics = new DisplayMetrics();

      // Java: ((WindowManager)context.getSystemService("window")).getDefaultDisplay().getMetrics(metrics);

      // Java: return new int[]{metrics.widthPixels, metrics.heightPixels};

      Vec::new()
    }

    // Java: static float getDip(Context context)
    pub fn getDip(&self, _context: i32) -> f32 {
      // Java: if (mDip <= 0.0F)

      {
         // Java: mDip = TypedValue.applyDimension(1, 1.0F, context.getResources().getDisplayMetrics());

      }
      // Java: return mDip;

      0.0
    }

    // Java: static float getDensity(Context context)
    pub fn getDensity(&self, _context: i32) -> f32 {
      // Java: WindowManager wm = (WindowManager)context.getSystemService("window");

      // Java: wm.getDefaultDisplay().getMetrics(metrics);

      // Java: return metrics.density;

      0.0
    }

    // Java: static boolean isPadScreen(Context context)
    pub fn isPadScreen(&self, _context: i32) -> bool {
      // Java: ISharedInfo sharedInfo = Platform.getSharedInfo();

      // Java: return null == sharedInfo ? false : sharedInfo.isPadScreen(context);

      false
    }

    // Java: static boolean isRecordVersion()
    pub fn isRecordVersion(&self) -> bool {
      // Java: ISharedInfo sharedInfo = Platform.getSharedInfo();

      // Java: return null == sharedInfo ? false : sharedInfo.isRecordVersion();

      false
    }

    // Java: static int dip2px(Context context, float dpValue)
    pub fn dip2px(&self, _context: i32, _dpValue: f32) -> i32 {
      // Java: float scale = context.getResources().getDisplayMetrics().density;

      // Java: return (int)(dpValue * scale + 0.5F);

      0
    }

}
