// Java: import java.util.Map
use std::collections::HashMap;

// Java: public interface StatAnalyticsListener
pub trait StatAnalyticsListener {
    // Java: abstract void onEventHappened(String var1, Map<String, String> var2)
    fn onEventHappened(&self, var1: &str, var2: &HashMap<String, String>);
}

// Java: public class StatAnalytics
pub struct StatAnalytics {
    // Java field: String PPT_TARGET_RESIZE
    // Java field: String PPT_MULTI_SELECT
    // Java field: String EVENTNAME_PAGE_SHOW
    // Java field: String EVENTNAME_POPUPS_TABLEOUT
    // Java field: String EVENTNAME_BUTTON_CLICK
    // Java field: String EVENTNAME_FUNC_RESULT
    // Java field: String KEY_COMP
    // Java field: String PPT
    // Java field: String PPT_TABLE_PASTE_OUT_URL
    // Java field: String PPT_CROSS_PROCESS_PASTE_URL
    // Java field: String PPT_CROSS_PROCESS_TABLE_PASTE_URL
    // Java field: String RESULT_NAME_POPUPS_SHOW
    // Java field: String PASTE_SUCCESS
    // Java field: String PASTE_FAIL
    // Java field: String DATA1_PPT_SLIDE
    // Java field: String DATA1_OBJECTS
    // Java field: String KEY_FUNC_NAME
    // Java field: String FUN_NAME_CROSS_PROCESS_PASTE
    // Java field: String FUN_NAME_CROSS_PROCESS_TABLE_PASTE
    // Java field: String KEY_URL
    // Java field: String KEY_PAGE_NAME
    // Java field: String KEY_BUTTON_NAME
    // Java field: String KEY_RESULT_NAME
    // Java field: String KEY_POSITION
    // Java field: String KEY_DATA1
    // Java field: String KEY_DATA2
    // Java field: String KEY_DATA3
    // Java field: String KEY_DATA4
    // Java field: String KEY_DATA5
    // Java field: StatAnalyticsListener statAnalyticsListener
    statAnalyticsListener: Option<Box<dyn StatAnalyticsListener>>,
}

// Java: public class StatAnalytics
impl StatAnalytics {
    pub const PPT_TARGET_RESIZE: &str = "ppt_target_resize";
    pub const PPT_MULTI_SELECT: &str = "multiSelect";
    pub const EVENTNAME_PAGE_SHOW: &str = "page_show";
    pub const EVENTNAME_POPUPS_TABLEOUT: &str = "popups_tableout";
    pub const EVENTNAME_BUTTON_CLICK: &str = "button_click";
    pub const EVENTNAME_FUNC_RESULT: &str = "func_result";
    pub const KEY_COMP: &str = "comp";
    pub const PPT: &str = "ppt";
    pub const PPT_TABLE_PASTE_OUT_URL: &str = "ppt/table_paste_out";
    pub const PPT_CROSS_PROCESS_PASTE_URL: &str = "ppt/cross_process_paste";
    pub const PPT_CROSS_PROCESS_TABLE_PASTE_URL: &str = "ppt/cross_process_table_paste";
    pub const RESULT_NAME_POPUPS_SHOW: &str = "popups_show";
    pub const PASTE_SUCCESS: &str = "paste_success";
    pub const PASTE_FAIL: &str = "paste_fail";
    pub const DATA1_PPT_SLIDE: &str = "ppt_slides";
    pub const DATA1_OBJECTS: &str = "objects";
    pub const KEY_FUNC_NAME: &str = "func_name";
    pub const FUN_NAME_CROSS_PROCESS_PASTE: &str = "cross_process_paste";
    pub const FUN_NAME_CROSS_PROCESS_TABLE_PASTE: &str = "cross_process_table_paste";
    pub const KEY_URL: &str = "url";
    pub const KEY_PAGE_NAME: &str = "page_name";
    pub const KEY_BUTTON_NAME: &str = "button_name";
    pub const KEY_RESULT_NAME: &str = "result_name";
    pub const KEY_POSITION: &str = "position";
    pub const KEY_DATA1: &str = "data1";
    pub const KEY_DATA2: &str = "data2";
    pub const KEY_DATA3: &str = "data3";
    pub const KEY_DATA4: &str = "data4";
    pub const KEY_DATA5: &str = "data5";

    // Java: StatAnalytics(StatAnalyticsListener listener)
    pub fn new(listener: Box<dyn StatAnalyticsListener>) -> Self {
        // Java: super();
        // Java: this.statAnalyticsListener = listener;
        Self { statAnalyticsListener: Some(listener) }
    }

    // Java: void onEventHappened(String eventType, Map<String, String> event)
    pub fn onEventHappened(&self, event_type: &str, event: &HashMap<String, String>) {
        // Java: if (null != this.statAnalyticsListener)
        if let Some(ref listener) = self.statAnalyticsListener {
            // Java: this.statAnalyticsListener.onEventHappened(eventType, event);
            listener.onEventHappened(event_type, event);
        }
    }
}
