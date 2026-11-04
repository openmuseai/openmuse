// Java: import android.content.BroadcastReceiver
// Java: import android.content.Context
// Java: import android.content.Intent
// Java: import android.content.IntentFilter
// Java: import android.os.Bundle
// Java: import android.os.Handler
// Java: import android.os.IBinder
// Java: import android.os.ResultReceiver
// Java: import android.view.View
// Java: import android.view.inputmethod.InputMethodManager
// Java: import cn.wps.core.runtime.ISharedInfo
// Java: import cn.wps.core.runtime.Platform

// Java field: InputMethodManager sIMM

// Java: public class SoftKeyboardUtil
pub struct SoftKeyboardUtil {
}

// Java: public class SoftKeyboardResultReceiver
pub struct SoftKeyboardResultReceiver {
    // Java field: Runnable mRunable
    pub(crate) mRunable: Option<Box<dyn Fn()>>,
    // Java field: Handler mSoftKeyboardHandler
    pub(crate) mSoftKeyboardHandler: Option<Box<dyn std::any::Any>>,
}

// Java: public class SoftKeyboardUtil
impl SoftKeyboardUtil {
    // Java: SoftKeyboardUtil()
    pub fn new() -> Self {
      // Java: super();
      SoftKeyboardUtil {}
    }

    // Java: static InputMethodManager getIMM(Context context)
    pub fn getIMM(_context: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
      // Java: return null == context ? sIMM : (sIMM = (InputMethodManager)context.getSystemService("input_method"));
      todo!()
    }

    // Java: static boolean showSoftKeyboard(View view)
    pub fn showSoftKeyboard(_view: Option<&dyn std::any::Any>) -> bool {
      // Java: if (null == view)
      // Java: return false;
      // Java: else
      // Java: InputMethodManager imm = getIMM(view.getContext());
      // Java: ISharedInfo versionInfo = Platform.getSharedInfo();
      // Java: boolean isAndroidN = null == versionInfo ? false : versionInfo.isAndroidN();
      // Java: if (isAndroidN)
      // Java: imm.restartInput(view);
      // Java: return imm != null && imm.showSoftInput(view, 0);
      todo!()
    }

    // Java: static boolean hideSoftKeyboard(Context context, IBinder windowToken, ResultReceiver resultReceiver)
    pub fn hideSoftKeyboard_with_receiver(_context: &dyn std::any::Any, _windowToken: &dyn std::any::Any, _resultReceiver: Option<&dyn std::any::Any>) -> bool {
      // Java: InputMethodManager imm = getIMM(context);
      // Java: return null != imm && imm.hideSoftInputFromWindow(windowToken, 0, resultReceiver);
      todo!()
    }

    // Java: static boolean hideSoftKeyboard(View view, ResultReceiver resultReceiver)
    pub fn hideSoftKeyboard_view_with_receiver(_view: Option<&dyn std::any::Any>, _resultReceiver: Option<&dyn std::any::Any>) -> bool {
      // Java: return null == view ? false : hideSoftKeyboard(view.getContext(), view.getWindowToken(), resultReceiver);
      todo!()
    }

    // Java: static boolean hideSoftKeyboard(Context context, IBinder windowToken)
    pub fn hideSoftKeyboard_context(_context: &dyn std::any::Any, _windowToken: &dyn std::any::Any) -> bool {
      // Java: return hideSoftKeyboard(context, windowToken, (ResultReceiver)null);
      todo!()
    }

    // Java: static boolean hideSoftKeyboard(View view)
    pub fn hideSoftKeyboard(_view: Option<&dyn std::any::Any>) -> bool {
      // Java: return null == view ? false : hideSoftKeyboard(view.getContext(), view.getWindowToken(), (ResultReceiver)null);
      todo!()
    }

    // Java: static void restartInput(View view)
    pub fn restartInput(_view: Option<&dyn std::any::Any>) {
      // Java: if (view != null)
      // Java: getIMM(view.getContext()).restartInput(view);
      todo!()
    }

    // Java: static boolean isSoftKeyboardActive(View view)
    pub fn isSoftKeyboardActive_view(_view: Option<&dyn std::any::Any>) -> bool {
      // Java: if (null == view)
      // Java: return false;
      // Java: else
      // Java: InputMethodManager imm = getIMM(view.getContext());
      // Java: return null != imm && imm.isActive(view);
      todo!()
    }

    // Java: static boolean isSoftKeyboardActive(Context context)
    pub fn isSoftKeyboardActive(_context: &dyn std::any::Any) -> bool {
      // Java: InputMethodManager imm = getIMM(context);
      // Java: return null != imm && imm.isActive();
      todo!()
    }

    // Java: static boolean isHardKeyboardActive(Context context)
    pub fn isHardKeyboardActive(_context: &dyn std::any::Any) -> bool {
      // Java: return context.getResources().getConfiguration().hardKeyboardHidden == 1;
      todo!()
    }

    // Java: static void showInputMethodPicker(Context context)
    pub fn showInputMethodPicker(_context: &dyn std::any::Any) {
      // Java: InputMethodManager imm = getIMM(context);
      // Java: if (null != imm)
      // Java: imm.showInputMethodPicker();
      todo!()
    }

    // Java: static void toggleSoftInput(Context context)
    pub fn toggleSoftInput(_context: &dyn std::any::Any) {
      // Java: InputMethodManager imm = getIMM(context);
      // Java: imm.toggleSoftInput(0, 2);
      todo!()
    }

    // Java: static void toggleSoftInput(View view)
    pub fn toggleSoftInput_view(_view: Option<&dyn std::any::Any>) {
      // Java: if (null != view)
      // Java: toggleSoftInput(view.getContext());
      todo!()
    }

    // Java: static boolean hideSoftKeyboardIfNeed(View view, Runnable callBack)
    pub fn hideSoftKeyboardIfNeed(_view: Option<&dyn std::any::Any>, _callBack: Option<Box<dyn Fn()>>) -> bool {
      // Java: ResultReceiver resultreceiver = new SoftKeyboardResultReceiver(new Handler(), callBack);
      // Java: boolean result = hideSoftKeyboard(view, resultreceiver);
      // Java: if (!result && null != callBack)
      // Java: callBack.run();
      // Java: return result;
      todo!()
    }

    // Java: static BroadcastReceiver bindInputMethodChange(View view, final Runnable runnable)
    pub fn bindInputMethodChange(_view: &dyn std::any::Any, _runnable: Box<dyn Fn()>) -> Box<dyn std::any::Any> {
      // Java: BroadcastReceiver receiver = new BroadcastReceiver()
      // Java: public void onReceive(Context context, Intent intent)
      // Java: runnable.run();
      // Java: view.getContext().registerReceiver(receiver, new IntentFilter("android.intent.action.INPUT_METHOD_CHANGED"));
      // Java: return receiver;
      todo!()
    }

}

// Java: public class SoftKeyboardResultReceiver
impl SoftKeyboardResultReceiver {
    // Java: SoftKeyboardResultReceiver(Handler handler, Runnable run)
    pub fn new(_handler: Box<dyn std::any::Any>, _run: Option<Box<dyn Fn()>>) -> Self {
         // Java: super(handler);
         // Java: this.mSoftKeyboardHandler = handler;
         // Java: this.mRunable = run;
         SoftKeyboardResultReceiver {
             mRunable: _run,
             mSoftKeyboardHandler: Some(_handler),
         }
    }

    // Java: void onReceiveResult(int resultCode, Bundle resultData)
    pub fn onReceiveResult(&self, _resultCode: i32, _resultData: &dyn std::any::Any) {
         // Java: this.mSoftKeyboardHandler.postDelayed(new Runnable()
         // Java: public void run()
         // Java: if (SoftKeyboardResultReceiver.this.mRunable != null)
         // Java: SoftKeyboardResultReceiver.this.mRunable.run();
         if let Some(ref runnable) = self.mRunable {
             runnable();
         }
    }

}
