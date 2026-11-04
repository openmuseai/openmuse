use crate::com::hp::hpl::inkml::r#impl::estimate::FactoryEstimate::FactoryEstimate;
use crate::com::hp::hpl::inkml::r#impl::Brush::Brush;

// Java field: InkEstimate sInkEstimate
static sInkEstimate: std::sync::OnceLock<std::sync::Mutex<Option<InkEstimate>>> = std::sync::OnceLock::new();
// Java field: FactoryEstimate sFactoryEstimate
static sFactoryEstimate: std::sync::OnceLock<std::sync::Mutex<Option<Box<dyn FactoryEstimate + Send>>>> = std::sync::OnceLock::new();

// Java: public class InkEstimate
pub struct InkEstimate {
    // Java field: float mScaleX
    mScaleX: f32,
    // Java field: float mScaleY
    mScaleY: f32,
    // Java field: float mOffsetX
    mOffsetX: f32,
    // Java field: float mOffsetY
    mOffsetY: f32,
    // Java field: long mLastTime
    mLastTime: i64,
    // Java field: boolean bUse
    bUse: bool,
}

// Java: public class InkEstimate
impl InkEstimate {
    // Java: InkEstimate()
    pub fn InkEstimate() -> Self {
        // Java: super();
        Self {
            mScaleX: 1.0,
            mScaleY: 1.0,
            mOffsetX: 0.0,
            mOffsetY: 0.0,
            mLastTime: 0,
            bUse: false,
        }
    }

    // Java: static InkEstimate getInstance()
    pub fn getInstance() -> Option<InkEstimate> {
        // Java: try
        {
            // Java: if (sFactoryEstimate == null)
            let factory_lock = sFactoryEstimate.get_or_init(|| std::sync::Mutex::new(None));
            let factory_guard = factory_lock.lock().unwrap();
            if factory_guard.is_none() {
                // Java: return null;
                return None;
            }
            // Java: if (sInkEstimate == null && sFactoryEstimate.isParamsOn())
            let ink_lock = sInkEstimate.get_or_init(|| std::sync::Mutex::new(None));
            let mut ink_guard = ink_lock.lock().unwrap();
            if ink_guard.is_none() && factory_guard.as_ref().unwrap().isParamsOn() {
                // Java: Object object = Platform.getApplicationContext();
                // Java: if (object instanceof Application)
                {
                    // Java: Context context = ((Application)object).getApplicationContext();
                    let context: i64 = 0;
                    // Java: sInkEstimate = sFactoryEstimate.isEngineRuntimeAvailable(context) ? new InkEstimate() : null;
                    if factory_guard.as_ref().unwrap().isEngineRuntimeAvailable(context) {
                        *ink_guard = Some(InkEstimate::InkEstimate());
                    } else {
                        *ink_guard = None;
                    }
                }
            }
        }
        // Java: catch (Exception var2)
        {
        }
        // Java: return sInkEstimate;
        let ink_lock = sInkEstimate.get_or_init(|| std::sync::Mutex::new(None));
        let ink_guard = ink_lock.lock().unwrap();
        ink_guard.clone()
    }

    // Java: static void setFactoryEstimate(FactoryEstimate factoryEstimate)
    pub fn setFactoryEstimate(factoryEstimate: Box<dyn FactoryEstimate + Send>) {
        // Java: if (null == sFactoryEstimate)
        let factory_lock = sFactoryEstimate.get_or_init(|| std::sync::Mutex::new(None));
        let mut factory_guard = factory_lock.lock().unwrap();
        if factory_guard.is_none() {
            // Java: sFactoryEstimate = factoryEstimate;
            *factory_guard = Some(factoryEstimate);
        }
    }

    // Java: void setUse()
    pub fn setUse(&mut self) {
        // Java: this.bUse = true;
        self.bUse = true;
    }

    // Java: void setScale(float scaleX, float scaleY)
    pub fn setScale(&mut self, scaleX: f32, scaleY: f32) {
        // Java: this.mScaleX = scaleX;
        self.mScaleX = scaleX;
        // Java: this.mScaleY = scaleY;
        self.mScaleY = scaleY;
    }

    // Java: void setOffset(float offsetX, float offsetY)
    pub fn setOffset(&mut self, offsetX: f32, offsetY: f32) {
        // Java: this.mOffsetX = offsetX;
        self.mOffsetX = offsetX;
        // Java: this.mOffsetY = offsetY;
        self.mOffsetY = offsetY;
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mOffsetX = 0.0F;
        self.mOffsetX = 0.0;
        // Java: this.mOffsetY = 0.0F;
        self.mOffsetY = 0.0;
        // Java: this.mScaleX = 1.0F;
        self.mScaleX = 1.0;
        // Java: this.mScaleY = 1.0F;
        self.mScaleY = 1.0;
        // Java: this.bUse = false;
        self.bUse = false;
        // Java: sFactoryEstimate.clear();
        let factory_lock = sFactoryEstimate.get_or_init(|| std::sync::Mutex::new(None));
        if let Ok(mut guard) = factory_lock.lock() {
            if let Some(ref mut f) = *guard {
                f.clear();
            }
        }
    }

    // Java: void clearPoints()
    pub fn clearPoints(&mut self) {
        // Java: sFactoryEstimate.clear();
        let factory_lock = sFactoryEstimate.get_or_init(|| std::sync::Mutex::new(None));
        if let Ok(mut guard) = factory_lock.lock() {
            if let Some(ref mut f) = *guard {
                f.clear();
            }
        }
    }

    // Java: void addPoint(MotionEvent event)
    pub fn addPoint(&mut self, event: i64) {
        // Java: long time = event.getEventTime();
        let time: i64 = event;
        // Java: if (time != this.mLastTime && time >= this.mLastTime)
        if time != self.mLastTime && time >= self.mLastTime {
            // Java: sFactoryEstimate.addPoint(event, time);
            let factory_lock = sFactoryEstimate.get_or_init(|| std::sync::Mutex::new(None));
            if let Ok(mut guard) = factory_lock.lock() {
                if let Some(ref mut f) = *guard {
                    f.addPoint_event(event, time);
                }
            }
            // Java: this.mLastTime = time;
            self.mLastTime = time;
        }
    }

    // Java: void addHistoryPoint(MotionEvent event)
    pub fn addHistoryPoint(&mut self, event: i64) {
        // Java: sFactoryEstimate.addHistoryPoint(event);
        let factory_lock = sFactoryEstimate.get_or_init(|| std::sync::Mutex::new(None));
        if let Ok(mut guard) = factory_lock.lock() {
            if let Some(ref mut f) = *guard {
                f.addHistoryPoint(event);
            }
        }
    }

    // Java: void addEstimatePoint(float x, float y)
    pub fn addEstimatePoint(&mut self, x: f32, y: f32) {
        // Java: sFactoryEstimate.addPoint(x, y, 1.0F);
        let factory_lock = sFactoryEstimate.get_or_init(|| std::sync::Mutex::new(None));
        if let Ok(mut guard) = factory_lock.lock() {
            if let Some(ref mut f) = *guard {
                f.addPoint(x, y, 1.0);
            }
        }
    }

    // Java: void draw(Canvas canvas, Paint paint, Path path, IPainter painter, TraceDataList list, Brush brush, float traceScaleX, float traceScaleY, float scaleX, float scaleY)
    pub fn draw(&self, _canvas: i64, _paint: i64, _path: i64, _painter: i64, _list: i64, _brush: &Brush, _traceScaleX: f32, _traceScaleY: f32, _scaleX: f32, _scaleY: f32) {
        // Java: if (this.bUse)
        if self.bUse {
            // Java: List<PointF> estimatePoints = sFactoryEstimate.getEstimatePoints();
            let factory_lock = sFactoryEstimate.get_or_init(|| std::sync::Mutex::new(None));
            let guard = factory_lock.lock().unwrap();
            let estimate_points = if let Some(ref f) = *guard {
                f.getEstimatePoints()
            } else {
                Vec::new()
            };
            // Java: if (estimatePoints != null && !estimatePoints.isEmpty())
            if !estimate_points.is_empty() {
                // Java: TraceDataList tdList = new TraceDataList(list.getXYChannleType(), list.getXIndex(), list.getYIndex(), -1);
                let mut td_list: Vec<Vec<f32>> = Vec::new();
                // Java: float scale = 26.458334F;
                let scale: f32 = 26.458334;
                // Java: int channelCount = 2;
                let channel_count: i32 = 2;
                // Java: Object[] first = list.get(list.size() - 1);
                // Java: tdList.add(first);
                // Java: for(PointF info : estimatePoints)
                for info in &estimate_points {
                    // Java: Object[] traceFragment = new Object[channelCount];
                    let mut trace_fragment = vec![0.0f32; channel_count as usize];
                    // Java: traceFragment[0] = (info.x + this.mOffsetX) * scale * this.mScaleX;
                    trace_fragment[0] = (info.0 + self.mOffsetX) * scale * self.mScaleX;
                    // Java: traceFragment[1] = (info.y + this.mOffsetY) * scale * this.mScaleY;
                    trace_fragment[1] = (info.1 + self.mOffsetY) * scale * self.mScaleY;
                    // Java: tdList.add(traceFragment);
                    td_list.push(trace_fragment);
                }
                // Java: if (sFactoryEstimate.openDebug())
                if guard.as_ref().map_or(false, |f| f.openDebug()) {
                    // Java: paint.setColor(-65536);
                }
                // Java: int clr = paint.getColor();
                // Java: float width = paint.getStrokeWidth();
                // Java: Paint.Style style = paint.getStyle();
                // Java: if (painter instanceof WritingBrush)
                {
                    // Java: float f = 0.5F;
                    let mut _f: f32 = 0.5;
                    // Java: if (first.length > 2)
                    {
                        // Java: Object o = first[2];
                        // Java: if (o instanceof Float)
                        {
                            // Java: f = (Float)o;
                        }
                    }
                    // Java: float maxFChannel = brush.getMaxFChannel();
                    let _max_f_channel = _brush.getMaxFChannel();
                    // Java: if (maxFChannel > 0.0F)
                    if _max_f_channel > 0.0 {
                        // Java: f /= maxFChannel;
                        _f /= _max_f_channel;
                    }
                    // Java: paint.setStrokeWidth(brush.getWidth() * 2.0F * f);
                    // Java: paint.setStyle(Style.STROKE);
                    // Java: painter = new NormalPainter();
                }
                // Java: path.reset();
                // Java: InkPainter.drawDataList(canvas, paint, path, painter, tdList, brush, traceScaleX, traceScaleY, scaleX, scaleY);
                // Java: paint.setColor(clr);
                // Java: paint.setStyle(style);
                // Java: paint.setStrokeWidth(width);
            }
        }
    }

    // Java: void onTouchDown(MotionEvent event)
    pub fn onTouchDown(&mut self, event: i64) {
        // Java: sFactoryEstimate.onTouchDown(event);
        let factory_lock = sFactoryEstimate.get_or_init(|| std::sync::Mutex::new(None));
        if let Ok(mut guard) = factory_lock.lock() {
            if let Some(ref mut f) = *guard {
                f.onTouchDown(event);
            }
        }
    }
}

impl Clone for InkEstimate {
    fn clone(&self) -> Self {
        Self {
            mScaleX: self.mScaleX,
            mScaleY: self.mScaleY,
            mOffsetX: self.mOffsetX,
            mOffsetY: self.mOffsetY,
            mLastTime: self.mLastTime,
            bUse: self.bUse,
        }
    }
}
