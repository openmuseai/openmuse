// Java: import android.graphics.Bitmap
// Java: import android.graphics.Canvas
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.RectF
// Java: import android.graphics.Bitmap.Config
// Java: import android.graphics.Paint.Style
// Java: import android.graphics.Path.Direction
// Java: import android.text.TextPaint
// Java: import android.text.TextUtils
// Java: import cn.wps.text.layout.data.ParaInfo
// Java: import cn.wps.text.layout.data.WpsPath
// Java: import cn.wps.text.layout.typo.layout.Layout
// Java: import cn.wps.text.layout.util.KmoAudioNoteUtil

use std::sync::{Mutex, OnceLock};
use crate::cn::wps::text::layout::util::KmoAudioNoteUtil::KmoAudioNoteUtil;
use crate::cn::wps::text::layout::data::WpsPath::{WpsPath, PathDirection};

pub struct Canvas;
impl Canvas {
    pub fn new() -> Self { Canvas }
    pub fn save(&self) {}
    pub fn restore(&self) {}
    pub fn drawRoundRect(&self, _rect: &RectF, _rx: f32, _ry: f32, _paint: &Paint) {}
    pub fn drawText(&self, _text: &str, _x: f32, _y: f32, _paint: &Paint) {}
    pub fn drawBitmap(&self, _bmp: &Bitmap, _x: f32, _y: f32, _paint: &Paint) {}
    pub fn drawArc(&self, _rect: &RectF, _start: f32, _sweep: f32, _use_center: bool, _paint: &Paint) {}
}

pub struct Paint { color: i32, stroke_width: f32 }
impl Paint {
    pub fn new() -> Self { Paint { color: 0, stroke_width: 0.0 } }
    pub fn setAntiAlias(&mut self, _aa: bool) {}
    pub fn setFilterBitmap(&mut self, _f: bool) {}
    pub fn setStrokeWidth(&mut self, w: f32) { self.stroke_width = w; }
    pub fn setStyle(&mut self, _s: Style) {}
    pub fn setColor(&mut self, c: i32) { self.color = c; }
    pub fn setTextSize(&mut self, _s: f32) {}
    pub fn descent(&self) -> f32 { 0.0 }
    pub fn ascent(&self) -> f32 { 0.0 }
}

pub enum Style { FILL, STROKE }

pub struct RectF { pub left: f32, pub top: f32, pub right: f32, pub bottom: f32 }
impl RectF {
    pub fn new(l: f32, t: f32, r: f32, b: f32) -> Self { RectF { left: l, top: t, right: r, bottom: b } }
}

pub struct Bitmap;
impl Bitmap {
    pub fn createBitmap(_w: i32, _h: i32) -> Self { Bitmap }
}

// Java field: int WIDTH
static WIDTH: OnceLock<Mutex<i32>> = OnceLock::new();
fn get_width() -> &'static Mutex<i32> { WIDTH.get_or_init(|| Mutex::new(200)) }

// Java: public interface IKCTextLayoutAudioListener
pub trait IKCTextLayoutAudioListener {
    // Java: abstract void onStart()
    fn onStart(&self);
    // Java: abstract void onStop()
    fn onStop(&self);
}

// Java: public interface PlayListener
pub trait PlayListener {
    // Java: abstract void play(AudioLayout var1)
    fn play(&self, layout: &AudioLayout);
    // Java: abstract void stop(AudioLayout var1)
    fn stop(&self, layout: &AudioLayout);
}

// Java: public class AudioLayout extends WpsLayout
pub struct AudioLayout {
    // Java field: int mTopPadding
    mTopPadding: i32,
    // Java field: int mBottomPadding
    mBottomPadding: i32,
    // Java field: int intTime
    intTime: i32,
    // Java field: int status
    status: i32,
    // Java field: String path
    pub(crate) path: String,
    // Java field: PlayListener playListener
    playListener: Option<Box<dyn PlayListener>>,
    // Java field: Paint paint
    paint: Paint,
    // Java field: Paint voicePaint
    voicePaint: Paint,
    // Java field: int height
    height: i32,
    // Java field: int drawHeight
    drawHeight: i32,
    // Java field: int drawWidth
    pub(crate) drawWidth: i32,
    // Java field: int mediaId
    pub(crate) mediaId: i32,
    // Java field: RectF anim_rect3
    anim_rect3: RectF,
    // Java field: RectF anim_rect2
    anim_rect2: RectF,
    // Java field: RectF anim_rect1
    anim_rect1: RectF,
    // Java field: boolean isPlaying
    pub(crate) isPlaying: bool,
}

// Java: public class AudioLayout extends WpsLayout
impl AudioLayout {
    // Java: AudioLayout(CharSequence text, TextPaint paint, int width, Layout.Alignment align, float spacingMult, float spacingAdd, int vertical)
    pub fn new(text: &str, _paint_arg: &(), _width: i32, _align: i32, _spacingMult: f32, _spacingAdd: f32, _vertical: i32) -> Self {
      // Java: super(text, 0, text.length(), paint, (float)width, align, spacingMult, spacingAdd, (TextUtils.TruncateAt)null, 0, 0, false, (ParaInfo)null, vertical, (short)0);
      let mut layout = AudioLayout {
          mTopPadding: 0, mBottomPadding: 0, intTime: 0, status: 3,
          path: String::new(), playListener: None,
          paint: Paint::new(), voicePaint: Paint::new(),
          height: 0, drawHeight: 0, drawWidth: 0, mediaId: 0,
          anim_rect3: RectF::new(0.0, 0.0, 0.0, 0.0),
          anim_rect2: RectF::new(0.0, 0.0, 0.0, 0.0),
          anim_rect1: RectF::new(0.0, 0.0, 0.0, 0.0),
          isPlaying: false,
      };
      // Java: this.initText(text.toString());
      layout.initText(text);
      // Java: this.initPaint();
      layout.initPaint();
      // Java: this.initDrawEnv();
      layout.initDrawEnv();
      layout
    }

    // Java: boolean isPlaying()
    pub fn isPlaying(&self) -> bool {
      // Java: return this.isPlaying;
      self.isPlaying
    }

    // Java: void setIsPlaying(boolean isPlaying)
    pub fn setIsPlaying(&mut self, isPlaying: bool) {
      // Java: this.isPlaying = isPlaying;
      self.isPlaying = isPlaying;
    }

    // Java: void initText(String text)
    fn initText(&mut self, text: &str) {
      // Java: String textId = text.split(";")[1];
      let parts: Vec<&str> = text.split(';').collect();
      let textId = if parts.len() > 1 { parts[1] } else { "" };
      // Java: String realData = (String)KmoAudioNoteUtil.audioMap.get(textId);
      // Java: String[] datas = realData.split(";");
      // Java: this.path = datas[0];
      // Java: this.intTime = Integer.parseInt(datas[1]);
      // Java: this.mediaId = Integer.parseInt(textId);
      self.mediaId = textId.parse::<i32>().unwrap_or(0);
      self.path = String::new();
      self.intTime = 0;
    }

    // Java: void initPaint()
    fn initPaint(&mut self) {
      // Java: this.paint = new Paint();
      self.paint = Paint::new();
      // Java: this.paint.setAntiAlias(true);
      self.paint.setAntiAlias(true);
      // Java: this.paint.setFilterBitmap(true);
      self.paint.setFilterBitmap(true);
      // Java: this.paint.setStrokeWidth(5.0F);
      self.paint.setStrokeWidth(5.0);
      // Java: this.paint.setStyle(Style.FILL);
      self.paint.setStyle(Style::FILL);
      // Java: this.voicePaint = new Paint();
      self.voicePaint = Paint::new();
      // Java: this.voicePaint.setStyle(Style.STROKE);
      self.voicePaint.setStyle(Style::STROKE);
      // Java: this.voicePaint.setStrokeWidth(3.0F);
      self.voicePaint.setStrokeWidth(3.0);
      // Java: this.voicePaint.setColor(-1);
      self.voicePaint.setColor(-1);
      // Java: this.voicePaint.setAntiAlias(true);
      self.voicePaint.setAntiAlias(true);
      // Java: this.voicePaint.setFilterBitmap(true);
      self.voicePaint.setFilterBitmap(true);
    }

    // Java: void initDrawEnv()
    fn initDrawEnv(&mut self) {
      // Java: this.height = (int)this.getHeight();
      self.height = 100;
      // Java: this.mTopPadding = this.mBottomPadding = (int)((float)this.height * 0.18F);
      self.mTopPadding = (self.height as f32 * 0.18) as i32;
      self.mBottomPadding = self.mTopPadding;
      // Java: this.drawHeight = this.height - this.mTopPadding * 2;
      self.drawHeight = self.height - self.mTopPadding * 2;
      // Java: this.drawWidth = this.drawHeight * 3;
      self.drawWidth = self.drawHeight * 3;
      // Java: WIDTH = this.drawWidth;
      *get_width().lock().unwrap() = self.drawWidth;
      // Java: int padingTop = this.drawHeight / 7;
      let padingTop = self.drawHeight / 7;
      // Java: int radius3 = this.drawHeight / 2 - padingTop;
      let radius3 = self.drawHeight / 2 - padingTop;
      // Java: this.anim_rect3 = new RectF(0.0F, (float)padingTop, (float)(radius3 * 2), (float)(this.drawHeight - padingTop));
      self.anim_rect3 = RectF::new(0.0, padingTop as f32, (radius3 * 2) as f32, (self.drawHeight - padingTop) as f32);
      // Java: this.anim_rect2 = new RectF((float)(radius3 / 3), (float)(padingTop + radius3 / 3), (float)(radius3 * 2 - radius3 / 3), (float)(this.drawHeight - padingTop - radius3 / 3));
      self.anim_rect2 = RectF::new((radius3 / 3) as f32, (padingTop + radius3 / 3) as f32, (radius3 * 2 - radius3 / 3) as f32, (self.drawHeight - padingTop - radius3 / 3) as f32);
      // Java: this.anim_rect1 = new RectF((float)(radius3 * 2 / 3), (float)(padingTop + radius3 * 2 / 3), (float)(radius3 * 2 - radius3 * 2 / 3), (float)(this.drawHeight - padingTop - radius3 * 2 / 3));
      self.anim_rect1 = RectF::new((radius3 * 2 / 3) as f32, (padingTop + radius3 * 2 / 3) as f32, (radius3 * 2 - radius3 * 2 / 3) as f32, (self.drawHeight - padingTop - radius3 * 2 / 3) as f32);
    }

    // Java: void draw(Canvas canvas, Path highlight, Paint highlightPaint, int cursorOffsetVertical)
    pub fn draw(&self, canvas: &Canvas, _highlight: &(), _highlightPaint: &(), _cursorOffsetVertical: i32) {
      // Java: this.drawBackground(canvas);
      self.drawBackground(canvas);
      // Java: this.drawWord(canvas);
      self.drawWord(canvas);
      // Java: this.drawVoice(canvas);
      self.drawVoice(canvas);
    }

    // Java: void drawBackground(Canvas canvas)
    fn drawBackground(&self, canvas: &Canvas) {
      // Java: this.paint.setColor(-12484615);
      // Java: RectF rect = new RectF(0.0F, (float)this.mTopPadding, (float)this.drawWidth, (float)(this.height - this.mBottomPadding));
      let rect = RectF::new(0.0, self.mTopPadding as f32, self.drawWidth as f32, (self.height - self.mBottomPadding) as f32);
      // Java: canvas.drawRoundRect(rect, (float)(this.drawHeight / 5), (float)(this.drawHeight / 5), this.paint);
      canvas.drawRoundRect(&rect, (self.drawHeight / 5) as f32, (self.drawHeight / 5) as f32, &self.paint);
    }

    // Java: void drawWord(Canvas canvas)
    fn drawWord(&self, canvas: &Canvas) {
      // Java: this.paint.setColor(-1);
      // Java: this.paint.setTextSize(30.0F);
      // Java: float baseLine = (float)(this.height / 2) - (this.paint.descent() + this.paint.ascent()) / 2.0F;
      let baseLine = (self.height / 2) as f32 - (self.paint.descent() + self.paint.ascent()) / 2.0;
      // Java: int second = this.intTime / 1000;
      let second = self.intTime / 1000;
      // Java: canvas.drawText((this.intTime % 1000 >= 500 ? second + 1 : second) + "''", (float)(this.drawWidth / 3), baseLine, this.paint);
      let display_second = if self.intTime % 1000 >= 500 { second + 1 } else { second };
      let text = format!("{}''", display_second);
      canvas.drawText(&text, (self.drawWidth / 3) as f32, baseLine, &self.paint);
    }

    // Java: void drawVoice(Canvas canvas)
    fn drawVoice(&self, canvas: &Canvas) {
      // Java: Bitmap lbBitmap = Bitmap.createBitmap(this.drawHeight, this.drawHeight, Config.ARGB_8888);
      let lbBitmap = Bitmap::createBitmap(self.drawHeight, self.drawHeight);
      // Java: Canvas vCanvas = new Canvas(lbBitmap);
      let vCanvas = Canvas::new();
      // Java: vCanvas.save();
      vCanvas.save();
      // Java: this.voicePaint.setStyle(Style.STROKE);
      // Java: if (this.status >= 3)
      if self.status >= 3
      {
         // Java: vCanvas.drawArc(this.anim_rect3, -45.0F, 85.0F, false, this.voicePaint);
         vCanvas.drawArc(&self.anim_rect3, -45.0, 85.0, false, &self.voicePaint);
      }
      // Java: if (this.status >= 2)
      if self.status >= 2
      {
         // Java: vCanvas.drawArc(this.anim_rect2, -45.0F, 85.0F, false, this.voicePaint);
         vCanvas.drawArc(&self.anim_rect2, -45.0, 85.0, false, &self.voicePaint);
      }
      // Java: this.voicePaint.setStyle(Style.FILL);
      // Java: vCanvas.drawArc(this.anim_rect1, -45.0F, 90.0F, true, this.voicePaint);
      vCanvas.drawArc(&self.anim_rect1, -45.0, 90.0, true, &self.voicePaint);
      // Java: vCanvas.restore();
      vCanvas.restore();
      // Java: canvas.drawBitmap(lbBitmap, 0.0F, (float)this.mTopPadding, this.paint);
      canvas.drawBitmap(&lbBitmap, 0.0, self.mTopPadding as f32, &self.paint);
    }

    // Java: boolean skipSelection(WpsPath path)
    pub fn skipSelection(&self, path: &mut WpsPath) -> bool {
      // Java: path.addRect(0.0F, 0.0F, (float)this.drawWidth, (float)this.height, Direction.CCW);
      path.addRect(0.0, 0.0, self.drawWidth as f32, self.height as f32, PathDirection::CCW);
      // Java: path.mergeRects(false);
      path.mergeRects(false);
      // Java: return true;
      true
    }

    // Java: void notifyTime(int time)
    pub fn notifyTime(&mut self, time: i32) {
      // Java: this.intTime = time;
      self.intTime = time;
    }

    // Java: void notifyVoiceStatus(int status)
    pub fn notifyVoiceStatus(&mut self, status: i32) {
      // Java: this.status = status;
      self.status = status;
    }

    // Java: boolean action(float x, float y, PlayListener listener)
    pub fn action(&mut self, _x: f32, _y: f32, listener: Box<dyn PlayListener>) -> bool {
      // Java: if (this.playListener == null)
      if self.playListener.is_none()
      {
         // Java: this.playListener = listener;
         self.playListener = Some(listener);
      }
      // Java: this.playListener.play(this);
      if let Some(ref pl) = self.playListener {
          pl.play(self);
      }
      // Java: return true;
      true
    }

    // Java: int getShapeWidth()
    pub fn getShapeWidth(&self) -> i32 {
      // Java: return this.drawWidth;
      self.drawWidth
    }

    // Java: void setTime(int time)
    pub fn setTime(&mut self, time: i32) {
      // Java: this.intTime = time;
      self.intTime = time;
    }

    // Java: void setPath(String path)
    pub fn setPath(&mut self, path: String) {
      // Java: this.path = path;
      self.path = path;
    }

    // Java: String getPath()
    pub fn getPath(&self) -> &str {
      // Java: return this.path;
      &self.path
    }

    // Java: int getMediaId()
    pub fn getMediaId(&self) -> i32 {
      // Java: return this.mediaId;
      self.mediaId
    }

}
