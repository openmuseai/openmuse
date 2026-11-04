use super::IInkStroker::IInkStroker;
use super::IEventParser::IEventParser;
use crate::cn::wps::libs::inkml::event::IEventDataSupplier::{IEventDataSupplier, EventDtaCallback};
use crate::com::hp::hpl::inkml::r#impl::InkEstimate::InkEstimate;

// Java: public class PretendEventParser implements IEventParser
pub struct PretendEventParser {
    // Java field: IEventDataSupplier mEventSupplier
    mEventSupplier: Box<dyn IEventDataSupplier>,
    // Java field: IInkStroker mStroker
    mStroker: Box<dyn IInkStroker>,
    // Java field: int GAP_SIZE
    #[allow(dead_code)]
    GAP_SIZE: i32,
}

struct CallbackImpl {
    stroker: *mut dyn IInkStroker,
}

unsafe impl Send for CallbackImpl {}

impl EventDtaCallback for CallbackImpl {
    fn onData(&self, event_type: i32, x: &[f32], y: &[f32], width: &[f32], offset: i32) {
        let stroker = unsafe { &mut *self.stroker };
        if !x.is_empty() && !y.is_empty() && !width.is_empty() {
            let mut len = if offset >= 0 && (offset as usize) < x.len() { offset as usize } else { x.len() };
            if event_type == 0 {
                stroker.onStart(x[0], y[0], width[0]);
            } else if event_type == 2 {
                for i in 0..len {
                    stroker.onMove(x[i], y[i], width[i]);
                }
                if offset >= 0 && x.len() as i32 - offset > 0 {
                    len = x.len() - offset as usize;
                    let estimate = InkEstimate::getInstance();
                    if let Some(mut estimate) = estimate {
                        estimate.clearPoints();
                        estimate.addEstimatePoint(x[offset as usize], y[offset as usize]);
                        for i in 0..len {
                            estimate.addEstimatePoint(x[i + offset as usize], y[i + offset as usize]);
                        }
                    }
                }
            } else if event_type == 1 {
                for i in 0..len {
                    stroker.onMove(x[i], y[i], width[i]);
                }
                stroker.onFinish();
            }
        }
    }
}

// Java: public class PretendEventParser implements IEventParser
impl PretendEventParser {
    // Java: PretendEventParser(IInkStroker stroker, IEventDataSupplier supplier)
    pub fn PretendEventParser(stroker: Box<dyn IInkStroker>, supplier: Box<dyn IEventDataSupplier>) -> Self {
        // Java: super();
        // Java: this.mEventSupplier = supplier;
        // Java: this.mStroker = stroker;
        let mut result = Self {
            mEventSupplier: supplier,
            mStroker: stroker,
            GAP_SIZE: 10,
        };
        // Java: this.mEventSupplier.setCallback((eventType, x, y, width, offset) ->
        {
            // Java: if (x != null && y != null && width != null)
            {
                // Java: int len = offset >= 0 && offset < x.length ? offset : x.length;
                // Java: if (eventType == 0)
                {
                    // Java: this.mStroker.onStart(x[0], y[0], width[0]);
                }
                // Java: else if (eventType == 2)
                {
                    // Java: for(int i = 0; i < len; ++i)
                    {
                        // Java: this.mStroker.onMove(x[i], y[i], width[i]);
                    }
                    // Java: if (offset >= 0 && x.length - offset > 0)
                    {
                        // Java: len = x.length - offset;
                        // Java: InkEstimate estimate = InkEstimate.getInstance();
                        // Java: if (estimate != null)
                        {
                            // Java: estimate.clearPoints();
                            // Java: estimate.addEstimatePoint(x[offset], y[offset]);
                            // Java: for(int i = 0; i < len; ++i)
                            {
                                // Java: estimate.addEstimatePoint(x[i + offset], y[i + offset]);
                            }
                        }
                    }
                }
                // Java: else if (eventType == 1)
                {
                    // Java: for(int i = 0; i < len; ++i)
                    {
                        // Java: this.mStroker.onMove(x[i], y[i], width[i]);
                    }
                    // Java: this.mStroker.onFinish();
                }
            }
        }
        let callback = CallbackImpl {
            stroker: &mut *result.mStroker as *mut dyn IInkStroker,
        };
        result.mEventSupplier.setCallback(Box::new(callback));
        result
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
    }

    // Java: void parseEvent(MotionEvent event)
    pub fn parseEvent(&mut self, event: i64) {
        // Java: this.mEventSupplier.postEvent(event, true);
        self.mEventSupplier.postEvent(event, true);
    }

    // Java: void touchCancel()
    pub fn touchCancel(&mut self) {
        // Java: if (this.mStroker != null)
        {
            // Java: this.mStroker.onFinish();
            self.mStroker.onFinish();
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
    }
}

impl IEventParser for PretendEventParser {
    fn parseEvent(&mut self, var1: i64) {
        self.parseEvent(var1);
    }
    fn touchCancel(&mut self) {
        PretendEventParser::touchCancel(self);
    }
    fn clear(&mut self) {
        PretendEventParser::clear(self);
    }
}
