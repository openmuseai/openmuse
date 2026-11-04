use super::IInkStroker::IInkStroker;
use super::IEventParser::IEventParser;
use crate::com::hp::hpl::inkml::r#impl::InkEstimate::InkEstimate;

// Java: public class EventParserImpl implements IEventParser
pub struct EventParserImpl {
    // Java field: IInkStroker mInkStroker
    mInkStroker: Box<dyn IInkStroker>,
    // Java field: boolean mIsGesturing
    mIsGesturing: bool,
}

// Java: public class EventParserImpl implements IEventParser
impl EventParserImpl {
    // Java: EventParserImpl(IInkStroker inkStroker)
    pub fn EventParserImpl(inkStroker: Box<dyn IInkStroker>) -> Self {
        // Java: super();
        // Java: this.mInkStroker = inkStroker;
        Self {
            mInkStroker: inkStroker,
            mIsGesturing: false,
        }
    }

    // Java: void parseEvent(MotionEvent event)
    pub fn parseEvent(&mut self, event: i64) {
        // Java: if (null != this.mInkStroker)
        {
            // Java: this.processEvent(event);
            self.processEvent(event);
        }
    }

    // Java: void processEvent(MotionEvent event)
    fn processEvent(&mut self, event: i64) {
        // Java: switch (event.getAction())
        let action = event as i32;
        match action {
            // Java: case 0:
            0 => {
                // Java: this.touchDown(event);
                self.touchDown(event);
                // Java: case 2:
                // Java: this.touchMove(event);
                self.touchMove(event);
                // Java: break;
            }
            2 => {
                // Java: this.touchMove(event);
                self.touchMove(event);
                // Java: break;
            }
            // Java: case 1:
            1 => {
                // Java: this.touchUp(event);
                self.touchUp(event);
                // Java: break;
            }
            // Java: case 3:
            3 => {
                // Java: this.touchUp(event);
                self.touchUp(event);
            }
            _ => {}
        }
    }

    // Java: void touchDown(MotionEvent event)
    fn touchDown(&mut self, event: i64) {
        // Java: this.mIsGesturing = false;
        self.mIsGesturing = false;
        // Java: this.moveOnce(event.getX(), event.getY());
        self.moveOnce(0.0, 0.0);
        // Java: InkEstimate inkEstimate = InkEstimate.getInstance();
        let ink_estimate = InkEstimate::getInstance();
        // Java: if (inkEstimate != null)
        if let Some(mut ink_estimate) = ink_estimate {
            // Java: inkEstimate.onTouchDown(event);
            ink_estimate.onTouchDown(event);
        }
    }

    // Java: void touchMove(MotionEvent event)
    fn touchMove(&mut self, event: i64) {
        // Java: int N = event.getHistorySize();
        let n: i32 = 0;
        // Java: int P = event.getPointerCount();
        let p: i32 = 1;
        // Java: for(int i = 0; i < N; ++i)
        for _i in 0..n {
            // Java: for(int j = 0; j < P; ++j)
            for _j in 0..p {
                // Java: this.moveOnce(event.getHistoricalX(j, i), event.getHistoricalY(j, i));
                self.moveOnce(0.0, 0.0);
            }
        }
        // Java: for(int j = 0; j < P; ++j)
        for _j in 0..p {
            // Java: this.moveOnce(event.getX(j), event.getY(j));
            self.moveOnce(0.0, 0.0);
        }
        // Java: InkEstimate inkEstimate = InkEstimate.getInstance();
        let ink_estimate = InkEstimate::getInstance();
        // Java: if (inkEstimate != null)
        if let Some(mut ink_estimate) = ink_estimate {
            // Java: inkEstimate.addHistoryPoint(event);
            ink_estimate.addHistoryPoint(event);
            // Java: inkEstimate.addPoint(event);
            ink_estimate.addPoint(event);
        }
    }

    // Java: void touchUp(MotionEvent event)
    fn touchUp(&mut self, _event: i64) {
        // Java: if (this.mIsGesturing)
        if self.mIsGesturing {
            // Java: this.mInkStroker.onFinish();
            self.mInkStroker.onFinish();
        }
        // Java: this.mIsGesturing = false;
        self.mIsGesturing = false;
    }

    // Java: void moveOnce(float x, float y)
    fn moveOnce(&mut self, x: f32, y: f32) {
        // Java: if (!this.mIsGesturing)
        if !self.mIsGesturing {
            // Java: this.mIsGesturing = true;
            self.mIsGesturing = true;
            // Java: this.mInkStroker.onStart(x, y, 0.0F);
            self.mInkStroker.onStart(x, y, 0.0);
        // Java: else
        } else {
            // Java: this.mInkStroker.onMove(x, y, 0.0F);
            self.mInkStroker.onMove(x, y, 0.0);
        }
    }

    // Java: void touchCancel()
    pub fn touchCancel(&mut self) {
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
    }

    // Java: void clear()
    pub fn clear(&mut self) {
    }
}

impl IEventParser for EventParserImpl {
    fn parseEvent(&mut self, var1: i64) {
        self.parseEvent(var1);
    }
    fn touchCancel(&mut self) {
        EventParserImpl::touchCancel(self);
    }
    fn clear(&mut self) {
        EventParserImpl::clear(self);
    }
}
