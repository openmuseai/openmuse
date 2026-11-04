// Java: import android.graphics.Bitmap
// Java: import android.graphics.Paint
// Java: import android.graphics.Rect
// Java: import android.graphics.RectF
// Java: import android.graphics.Typeface
// Java: import cn.wps.text.layout.typo.ArtTransform
// Java: import cn.wps.text.layout.typo.PaintFontHelper

pub struct Bitmap {
    width: i32,
    height: i32,
}
impl Bitmap {
    pub fn getWidth(&self) -> i32 { self.width }
    pub fn getHeight(&self) -> i32 { self.height }
}

pub struct Paint {
    text_size: f32,
    typeface: Option<Typeface>,
}
impl Paint {
    pub fn new() -> Self { Paint { text_size: 0.0, typeface: None } }
    pub fn reset(&mut self) { self.text_size = 0.0; self.typeface = None; }
    pub fn setTextSize(&mut self, size: f32) { self.text_size = size; }
    pub fn setTypeface(&mut self, _tf: Typeface) {}
    pub fn measureText(&self, _text: &str) -> f32 { 0.0 }
    pub fn getFontMetrics(&self, metrics: &mut FontMetrics) {
        metrics.ascent = -self.text_size * 0.8;
        metrics.descent = self.text_size * 0.2;
    }
    pub fn getTextBounds(&self, _text: &str, _start: i32, _end: i32, rect: &mut Rect) {
        rect.top = 0;
        rect.bottom = self.text_size as i32;
        rect.left = 0;
        rect.right = self.text_size as i32;
    }
}

pub struct FontMetrics {
    pub ascent: f32,
    pub descent: f32,
}
impl FontMetrics {
    pub fn new() -> Self { FontMetrics { ascent: 0.0, descent: 0.0 } }
}

pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl Rect {
    pub fn new() -> Self { Rect { left: 0, top: 0, right: 0, bottom: 0 } }
    pub fn height(&self) -> i32 { self.bottom - self.top }
}

#[derive(Clone)]
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl RectF {
    pub fn width(&self) -> f32 { self.right - self.left }
}

#[derive(Clone)]
pub struct Typeface;
impl Typeface {
    pub const DEFAULT: Typeface = Typeface;
}

pub struct ArtTransform;

pub struct PaintFontHelper;
impl PaintFontHelper {
    pub fn getTypeface(_font_name: &str, _bold: bool, _italic: bool) -> Option<Typeface> {
        Some(Typeface)
    }
}

pub struct BulletsMatcher {
    matched_font_name: String,
    matched_bullet: String,
}
impl BulletsMatcher {
    pub fn obtain() -> Self { BulletsMatcher { matched_font_name: String::new(), matched_bullet: String::new() } }
    pub fn match_(&self, _font_name: &str, _bullet_str: &str) -> bool { false }
    pub fn getMatchedFontName(&self) -> String { self.matched_font_name.clone() }
    pub fn getMatchedBullet(&self) -> String { self.matched_bullet.clone() }
    pub fn recycle(self) {}
}

// Java field: float percentDivisor
const PERCENT_DIVISOR: f32 = 100000.0;
// Java field: char TEST_TEXTHEIGHT_CHAR
const TEST_TEXTHEIGHT_CHAR: char = '国';

// Java: public class BulletsInfo
pub struct BulletsInfo {
    // Java field: String BULLET_DEFALT
    pub BULLET_DEFALT: String,
    // Java field: boolean isRightToLeft
    pub isRightToLeft: bool,
    // Java field: boolean visible
    pub(crate) visible: bool,
    // Java field: int bulletType
    pub(crate) bulletType: i32,
    // Java field: float x
    pub(crate) x: f32,
    // Java field: float y
    pub(crate) y: f32,
    // Java field: int color
    pub(crate) color: i32,
    // Java field: float bulletSize
    pub(crate) bulletSize: f32,
    // Java field: int ansizePercent
    pub(crate) ansizePercent: i32,
    // Java field: String fontName
    pub(crate) fontName: String,
    // Java field: float width
    pub(crate) width: f32,
    // Java field: float height
    pub(crate) height: f32,
    // Java field: int mediaId
    pub(crate) mediaId: i32,
    // Java field: Bitmap bmp
    pub(crate) bmp: Option<Bitmap>,
    // Java field: float bmpScale
    pub(crate) bmpScale: f32,
    // Java field: String bulletStr
    pub(crate) bulletStr: String,
    // Java field: int bulletStyle
    pub(crate) bulletStyle: i32,
    // Java field: boolean isFontFollowText
    pub(crate) isFontFollowText: bool,
    // Java field: boolean isColorFollowText
    pub(crate) isColorFollowText: bool,
    // Java field: boolean isSizeFollowText
    pub(crate) isSizeFollowText: bool,
    // Java field: int rotateDegrees
    pub(crate) rotateDegrees: i32,
    // Java field: boolean isEastVertical
    pub(crate) isEastVertical: bool,
    // Java field: boolean isAllVertical
    pub(crate) isAllVertical: bool,
    // Java field: ArtTransform artTransform
    pub(crate) artTransform: Option<ArtTransform>,
}

// Java field: int TxList_None
pub const TxList_None: i32 = -1;
// Java field: int TxList_Bullet
pub const TxList_Bullet: i32 = 1;
// Java field: int TxList_AutoNum
pub const TxList_AutoNum: i32 = 2;
// Java field: int TxList_Image
pub const TxList_Image: i32 = 3;
// Java field: byte TxListVisibleMode_Hide
pub const TxListVisibleMode_Hide: u8 = 0;
// Java field: byte TxListVisibleMode_Show
pub const TxListVisibleMode_Show: u8 = 1;
// Java field: byte TxListVisibleMode_Translucent
pub const TxListVisibleMode_Translucent: u8 = 2;

static mut S_PAINT: Option<Paint> = None;
static mut S_FONT_METRICS: Option<FontMetrics> = None;

fn get_sPaint() -> &'static mut Paint {
    unsafe {
        if S_PAINT.is_none() { S_PAINT = Some(Paint::new()); }
        S_PAINT.as_mut().unwrap()
    }
}

fn get_sFontMetrics() -> &'static mut FontMetrics {
    unsafe {
        if S_FONT_METRICS.is_none() { S_FONT_METRICS = Some(FontMetrics::new()); }
        S_FONT_METRICS.as_mut().unwrap()
    }
}

// Java: public class BulletsInfo
impl BulletsInfo {
    // Java: BulletsInfo()
    pub fn new() -> Self {
        // Java: super();
        let bullet_default = String::from("●");
        BulletsInfo {
            // Java: this.visible = true;
            visible: true,
            // Java: this.bulletType = 1;
            bulletType: 1,
            // Java: this.x = 0.0F;
            x: 0.0,
            // Java: this.y = 0.0F;
            y: 0.0,
            // Java: this.color = -16777216;
            color: -16777216,
            // Java: this.bulletSize = 40.0F;
            bulletSize: 40.0,
            // Java: this.ansizePercent = 100000;
            ansizePercent: 100000,
            // Java: this.fontName = "Times New Roman";
            fontName: String::from("Times New Roman"),
            // Java: this.bulletStr = this.BULLET_DEFALT;
            bulletStr: bullet_default.clone(),
            // Java: this.width = 0.0F;
            width: 0.0,
            // Java: this.bulletStyle = 0;
            bulletStyle: 0,
            // Java: this.isFontFollowText = false;
            isFontFollowText: false,
            // Java: this.isColorFollowText = false;
            isColorFollowText: false,
            // Java: this.isSizeFollowText = false;
            isSizeFollowText: false,
            // Java: this.rotateDegrees = 0;
            rotateDegrees: 0,
            // Java: this.isRightToLeft = false;
            isRightToLeft: false,
            BULLET_DEFALT: bullet_default,
            height: 0.0,
            mediaId: -1,
            bmp: None,
            bmpScale: 0.0,
            isEastVertical: false,
            isAllVertical: false,
            artTransform: None,
        }
    }

    // Java: BulletsInfo(BulletsInfo bullets)
    pub fn new_from(bullets: &BulletsInfo) -> Self {
        // Java: super();
        BulletsInfo {
            // Java: this.visible = bullets.visible;
            visible: bullets.visible,
            // Java: this.bulletType = bullets.bulletType;
            bulletType: bullets.bulletType,
            // Java: this.x = bullets.x;
            x: bullets.x,
            // Java: this.y = bullets.y;
            y: bullets.y,
            // Java: this.color = bullets.color;
            color: bullets.color,
            // Java: this.bulletSize = bullets.bulletSize;
            bulletSize: bullets.bulletSize,
            // Java: this.ansizePercent = bullets.ansizePercent;
            ansizePercent: bullets.ansizePercent,
            // Java: this.fontName = bullets.fontName;
            fontName: bullets.fontName.clone(),
            // Java: this.width = bullets.width;
            width: bullets.width,
            // Java: this.height = bullets.height;
            height: bullets.height,
            // Java: this.bmp = bullets.bmp;
            bmp: None,
            // Java: this.bmpScale = bullets.bmpScale;
            bmpScale: bullets.bmpScale,
            // Java: this.bulletStr = bullets.bulletStr;
            bulletStr: bullets.bulletStr.clone(),
            // Java: this.bulletStyle = bullets.bulletStyle;
            bulletStyle: bullets.bulletStyle,
            // Java: this.isFontFollowText = bullets.isFontFollowText;
            isFontFollowText: bullets.isFontFollowText,
            // Java: this.isColorFollowText = bullets.isColorFollowText;
            isColorFollowText: bullets.isColorFollowText,
            // Java: this.isSizeFollowText = bullets.isSizeFollowText;
            isSizeFollowText: bullets.isSizeFollowText,
            // Java: this.rotateDegrees = bullets.rotateDegrees;
            rotateDegrees: bullets.rotateDegrees,
            // Java: this.artTransform = bullets.artTransform;
            artTransform: None,
            // Java: this.isRightToLeft = bullets.isRightToLeft;
            isRightToLeft: bullets.isRightToLeft,
            BULLET_DEFALT: bullets.BULLET_DEFALT.clone(),
            mediaId: bullets.mediaId,
            isEastVertical: bullets.isEastVertical,
            isAllVertical: bullets.isAllVertical,
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.visible = true;
        self.visible = true;
        // Java: this.bulletType = 1;
        self.bulletType = 1;
        // Java: this.x = 0.0F;
        self.x = 0.0;
        // Java: this.y = 0.0F;
        self.y = 0.0;
        // Java: this.color = -16777216;
        self.color = -16777216;
        // Java: this.bulletSize = 40.0F;
        self.bulletSize = 40.0;
        // Java: this.ansizePercent = 100000;
        self.ansizePercent = 100000;
        // Java: this.fontName = "Times New Roman";
        self.fontName = String::from("Times New Roman");
        // Java: this.bulletStr = this.BULLET_DEFALT;
        self.bulletStr = self.BULLET_DEFALT.clone();
        // Java: this.width = 0.0F;
        self.width = 0.0;
        // Java: this.bulletStyle = 0;
        self.bulletStyle = 0;
        // Java: this.isFontFollowText = false;
        self.isFontFollowText = false;
        // Java: this.isColorFollowText = false;
        self.isColorFollowText = false;
        // Java: this.isSizeFollowText = false;
        self.isSizeFollowText = false;
        // Java: this.rotateDegrees = 0;
        self.rotateDegrees = 0;
        // Java: this.isRightToLeft = false;
        self.isRightToLeft = false;
    }

    // Java: boolean isVisible()
    pub fn isVisible(&self) -> bool {
        // Java: return this.visible;
        self.visible
    }

    // Java: void setVisible(boolean visible)
    pub fn setVisible(&mut self, visible: bool) {
        // Java: this.visible = visible;
        self.visible = visible;
    }

    // Java: int getBulletType()
    pub fn getBulletType(&self) -> i32 {
        // Java: return this.bulletType;
        self.bulletType
    }

    // Java: void setBulletType(int bulletType)
    pub fn setBulletType(&mut self, bulletType: i32) {
        // Java: this.bulletType = bulletType;
        self.bulletType = bulletType;
    }

    // Java: float getX()
    pub fn getX(&self) -> f32 {
        // Java: return this.x;
        self.x
    }

    // Java: void setX(float x)
    pub fn setX(&mut self, x: f32) {
        // Java: this.x = x;
        self.x = x;
    }

    // Java: float getY()
    pub fn getY(&self) -> f32 {
        // Java: return this.y;
        self.y
    }

    // Java: void setY(float y)
    pub fn setY(&mut self, y: f32) {
        // Java: this.y = y;
        self.y = y;
    }

    // Java: int getColor()
    pub fn getColor(&self) -> i32 {
        // Java: return this.color;
        self.color
    }

    // Java: void setColor(int color)
    pub fn setColor(&mut self, color: i32) {
        // Java: this.color = color;
        self.color = color;
    }

    // Java: float getBulletSize()
    pub fn getBulletSize(&self) -> f32 {
        // Java: return this.bulletSize;
        self.bulletSize
    }

    // Java: void setBulletSize(float bulletSize)
    pub fn setBulletSize(&mut self, bulletSize: f32) {
        // Java: this.bulletSize = bulletSize;
        self.bulletSize = bulletSize;
    }

    // Java: float getAnsizePercent()
    pub fn getAnsizePercent(&self) -> f32 {
        // Java: return (float)this.ansizePercent / 100000.0F;
        self.ansizePercent as f32 / PERCENT_DIVISOR
    }

    // Java: void setAnsizePercent(int ansizePercent)
    pub fn setAnsizePercent(&mut self, ansizePercent: i32) {
        // Java: this.ansizePercent = ansizePercent;
        self.ansizePercent = ansizePercent;
    }

    // Java: String getFontName()
    pub fn getFontName(&self) -> &str {
        // Java: return this.fontName;
        &self.fontName
    }

    // Java: void setFontName(String fontName)
    pub fn setFontName(&mut self, fontName: String) {
        // Java: this.fontName = fontName;
        self.fontName = fontName;
    }

    // Java: float getWidth()
    pub fn getWidth(&self) -> f32 {
        // Java: return this.width;
        self.width
    }

    // Java: void setWidth(float width)
    pub fn setWidth(&mut self, width: f32) {
        // Java: this.width = width;
        self.width = width;
    }

    // Java: float getHeight()
    pub fn getHeight(&self) -> f32 {
        // Java: return this.height;
        self.height
    }

    // Java: void setHeight(float height)
    pub fn setHeight(&mut self, height: f32) {
        // Java: this.height = height;
        self.height = height;
    }

    // Java: Bitmap getBmp()
    pub fn getBmp(&self) -> &Option<Bitmap> {
        // Java: return this.bmp;
        &self.bmp
    }

    // Java: void setBmp(Bitmap bmp)
    pub fn setBmp(&mut self, bmp: Option<Bitmap>) {
        // Java: this.bmp = bmp;
        self.bmp = bmp;
    }

    // Java: int getMediaId()
    pub fn getMediaId(&self) -> i32 {
        // Java: return this.mediaId;
        self.mediaId
    }

    // Java: void setMediaId(int mediaId)
    pub fn setMediaId(&mut self, mediaId: i32) {
        // Java: this.mediaId = mediaId;
        self.mediaId = mediaId;
    }

    // Java: float getBmpScale()
    pub fn getBmpScale(&self) -> f32 {
        // Java: return this.bmpScale;
        self.bmpScale
    }

    // Java: void setBmpScale(float bmpScale)
    pub fn setBmpScale(&mut self, bmpScale: f32) {
        // Java: this.bmpScale = bmpScale;
        self.bmpScale = bmpScale;
    }

    // Java: String getDefaultBullet()
    pub fn getDefaultBullet(&self) -> &str {
        // Java: return this.BULLET_DEFALT;
        &self.BULLET_DEFALT
    }

    // Java: void setDefaultBullet(String defaultBullet)
    pub fn setDefaultBullet(&mut self, defaultBullet: String) {
        // Java: this.BULLET_DEFALT = defaultBullet;
        self.BULLET_DEFALT = defaultBullet;
    }

    // Java: String getBulletStr()
    pub fn getBulletStr(&self) -> &str {
        // Java: return this.bulletStr;
        &self.bulletStr
    }

    // Java: void setBulletStr(String bulletStr)
    pub fn setBulletStr(&mut self, bulletStr: String) {
        // Java: this.bulletStr = bulletStr;
        self.bulletStr = bulletStr;
    }

    // Java: void checkBulletStr()
    pub fn checkBulletStr(&mut self) {
        // Java: if (this.bulletStr.length() > 0 && 1 == this.bulletType)
        if self.bulletStr.len() > 0 && 1 == self.bulletType {
            // Java: BulletsMatcher matcher = BulletsMatcher.obtain();
            let matcher = BulletsMatcher::obtain();
            // Java: if (matcher.match(this.fontName, this.bulletStr))
            if matcher.match_(&self.fontName, &self.bulletStr) {
                // Java: this.fontName = matcher.getMatchedFontName();
                self.fontName = matcher.getMatchedFontName();
                // Java: this.bulletStr = matcher.getMatchedBullet();
                self.bulletStr = matcher.getMatchedBullet();
            }
            // Java: matcher.recycle();
            matcher.recycle();
        }
    }

    // Java: int getBulletStyle()
    pub fn getBulletStyle(&self) -> i32 {
        // Java: return this.bulletStyle;
        self.bulletStyle
    }

    // Java: void setBulletStyle(int bulletStyle)
    pub fn setBulletStyle(&mut self, bulletStyle: i32) {
        // Java: this.bulletStyle = bulletStyle;
        self.bulletStyle = bulletStyle;
    }

    // Java: void calBulletsWidAndHei(float fontsize, boolean isAllVert)
    pub fn calBulletsWidAndHei(&mut self, fontsize: f32, isAllVert: bool) {
        // Java: Paint paint = sPaint;
        let paint = get_sPaint();
        // Java: paint.reset();
        paint.reset();
        // Java: if (this.bulletSize != 0.0F)
        if self.bulletSize != 0.0 {
            // Java: paint.setTextSize(this.bulletSize);
            paint.setTextSize(self.bulletSize);
        } else {
            // Java: paint.setTextSize(fontsize);
            paint.setTextSize(fontsize);
        }

        // Java: switch (this.bulletType)
        match self.bulletType {
            // Java: case -1:
            -1 => {
                // Java: this.width = 0.0F;
                self.width = 0.0;
                // Java: this.height = 0.0F;
                self.height = 0.0;
                // Java: return;
                return;
            }
            // Java: case 1:
            // Java: case 2:
            1 | 2 => {
                // Java: if (this.bulletStr.length() <= 0)
                if self.bulletStr.len() <= 0 {
                    // Java: this.width = 0.0F;
                    self.width = 0.0;
                    // Java: this.height = 0.0F;
                    self.height = 0.0;
                    // Java: return;
                    return;
                }

                // Java: Typeface typeface = PaintFontHelper.getTypeface(this.fontName, false, false);
                let typeface = PaintFontHelper::getTypeface(&self.fontName, false, false);
                // Java: if (typeface != null)
                if let Some(tf) = typeface {
                    // Java: paint.setTypeface(typeface);
                    paint.setTypeface(tf);
                } else {
                    // Java: paint.setTypeface(Typeface.DEFAULT);
                    paint.setTypeface(Typeface::DEFAULT);
                }

                // Java: if (this.isEastVertical() && TextData.isChinese(this.bulletStr.charAt(0)))
                if self.isEastVertical() && is_chinese(self.bulletStr.chars().next().unwrap_or('\0')) {
                    // Java: this.width = paint.measureText(this.bulletStr.substring(0, 1)) * (float)this.bulletStr.length();
                    let first_char: String = self.bulletStr.chars().take(1).collect();
                    self.width = paint.measureText(&first_char) * self.bulletStr.chars().count() as f32;
                } else {
                    // Java: this.width = paint.measureText(this.bulletStr);
                    self.width = paint.measureText(&self.bulletStr);
                }

                // Java: paint.getFontMetrics(sFontMetrics);
                let fm = get_sFontMetrics();
                paint.getFontMetrics(fm);
                // Java: this.height = sFontMetrics.descent - sFontMetrics.ascent;
                self.height = fm.descent - fm.ascent;
            }
            // Java: case 3:
            3 => {
                // Java: paint.setTypeface(Typeface.DEFAULT);
                paint.setTypeface(Typeface::DEFAULT);
                // Java: paint.setTextSize(fontsize);
                paint.setTextSize(fontsize);
                // Java: Rect rect = new Rect();
                let mut rect = Rect::new();
                // Java: paint.getTextBounds(String.valueOf("TEST_IMGHEIGHT_CHAR"), 0, 1, rect);
                paint.getTextBounds("T", 0, 1, &mut rect);
                // Java: this.height = (float)rect.height();
                self.height = rect.height() as f32;
                // Java: if (null != this.bmp)
                if let Some(ref bmp) = self.bmp {
                    // Java: this.bmpScale = this.height / (float)this.bmp.getHeight();
                    self.bmpScale = self.height / bmp.getHeight() as f32;
                    // Java: this.width = (float)this.bmp.getWidth() * this.bmpScale;
                    self.width = bmp.getWidth() as f32 * self.bmpScale;
                }
            }
            // Java: default:
            _ => {}
        }

        // Java: this.width *= this.getAnsizePercent();
        let ansize = self.getAnsizePercent();
        self.width *= ansize;
        // Java: if (isAllVert)
        if isAllVert {
            // Java: float temp = this.width;
            let temp = self.width;
            // Java: this.width = this.height;
            self.width = self.height;
            // Java: this.height = temp;
            self.height = temp;
        }
    }

    // Java: void alterY(float alterY)
    pub fn alterY(&mut self, alterY: f32) {
        // Java: if (this.isAllVertical)
        if self.isAllVertical {
            // Java: this.x += alterY;
            self.x += alterY;
        } else {
            // Java: this.y += alterY;
            self.y += alterY;
        }
    }

    // Java: void alterX(float alterX)
    pub fn alterX(&mut self, alterX: f32) {
        // Java: if (this.isAllVertical)
        if self.isAllVertical {
            // Java: this.y += alterX;
            self.y += alterX;
        } else {
            // Java: this.x += alterX;
            self.x += alterX;
        }
    }

    // Java: boolean isFontFollowText()
    pub fn isFontFollowText(&self) -> bool {
        // Java: return this.isFontFollowText;
        self.isFontFollowText
    }

    // Java: void setFontFollowText(boolean isFontFollowText)
    pub fn setFontFollowText(&mut self, isFontFollowText: bool) {
        // Java: this.isFontFollowText = isFontFollowText;
        self.isFontFollowText = isFontFollowText;
    }

    // Java: boolean isColorFollowText()
    pub fn isColorFollowText(&self) -> bool {
        // Java: return this.isColorFollowText;
        self.isColorFollowText
    }

    // Java: void setColorFollowText(boolean isColorFollowText)
    pub fn setColorFollowText(&mut self, isColorFollowText: bool) {
        // Java: this.isColorFollowText = isColorFollowText;
        self.isColorFollowText = isColorFollowText;
    }

    // Java: boolean isSizeFollowText()
    pub fn isSizeFollowText(&self) -> bool {
        // Java: return this.isSizeFollowText;
        self.isSizeFollowText
    }

    // Java: void setSizeFollowText(boolean isSizeFollowText)
    pub fn setSizeFollowText(&mut self, isSizeFollowText: bool) {
        // Java: this.isSizeFollowText = isSizeFollowText;
        self.isSizeFollowText = isSizeFollowText;
    }

    // Java: void calBulletsXAndY(RectF textRect, float moveL, float textWid, short mHAlignment, float textOffsetY, float firstLineBaseline, float fontsize, boolean mIsRightToLeft, int verticalText, float firstLineHeight)
    pub fn calBulletsXAndY(&mut self, textRect: &RectF, moveL: f32, textWid: f32, mHAlignment: i16, textOffsetY: f32, firstLineBaseline: f32, fontsize: f32, mIsRightToLeft: bool, verticalText: i32, firstLineHeight: f32) {
        // Java: if (this.bulletType != -1)
        if self.bulletType != -1 {
            // Java: this.calBulletsX(textRect, moveL, textWid, mHAlignment, mIsRightToLeft);
            self.calBulletsX(textRect, moveL, textWid, mHAlignment, mIsRightToLeft);
            // Java: this.calBulletsY(textRect, textOffsetY, firstLineBaseline, fontsize);
            self.calBulletsY(textRect, textOffsetY, firstLineBaseline, fontsize);
            // Java: if (verticalText == 3 || verticalText == 6)
            if verticalText == 3 || verticalText == 6 {
                // Java: float temp = this.y;
                let temp = self.y;
                // Java: this.y = this.x;
                self.y = self.x;
                // Java: this.x = temp;
                self.x = temp;
                // Java: if (verticalText == 6)
                if verticalText == 6 {
                    // Java: this.x -= firstLineHeight;
                    self.x -= firstLineHeight;
                }
            }
        }
    }

    // Java: void calBulletsX(RectF textRect, float moveL, float textWid, short mHAlignment, boolean mIsRightToLeft)
    fn calBulletsX(&mut self, textRect: &RectF, moveL: f32, textWid: f32, mHAlignment: i16, mIsRightToLeft: bool) {
        // Java: float rectWidth = textRect.width();
        let rectWidth = textRect.width();
        // Java: if (!mIsRightToLeft)
        if !mIsRightToLeft {
            // Java: this.x = textRect.left - moveL;
            self.x = textRect.left - moveL;
            // Java: switch (mHAlignment)
            match mHAlignment {
                // Java: case 2:
                // Java: case 6:
                // Java: case 7:
                2 | 6 | 7 => {
                    // Java: this.x += (rectWidth - textWid) / 2.0F;
                    self.x += (rectWidth - textWid) / 2.0;
                }
                // Java: case 3:
                3 => {
                    // Java: this.x += rectWidth - textWid;
                    self.x += rectWidth - textWid;
                }
                // Java: default:
                _ => {}
            }
        } else {
            // Java: this.x = textRect.right + moveL - this.getWidth();
            let w = self.getWidth();
            self.x = textRect.right + moveL - w;
            // Java: switch (mHAlignment)
            match mHAlignment {
                // Java: case 0:
                // Java: case 1:
                0 | 1 => {
                    // Java: this.x += rectWidth - textWid;
                    self.x += rectWidth - textWid;
                }
                // Java: case 2:
                // Java: case 6:
                // Java: case 7:
                2 | 6 | 7 => {
                    // Java: this.x += (rectWidth - textWid) / 2.0F;
                    self.x += (rectWidth - textWid) / 2.0;
                }
                // Java: default:
                _ => {}
            }
        }
    }

    // Java: void calBulletsY(RectF textRect, float textOffsetY, float firstLineBaseline, float fontsize)
    fn calBulletsY(&mut self, _textRect: &RectF, textOffsetY: f32, firstLineBaseline: f32, fontsize: f32) {
        // Java: Paint paint = sPaint;
        let paint = get_sPaint();
        // Java: paint.reset();
        paint.reset();
        // Java: paint.setTypeface(Typeface.DEFAULT);
        paint.setTypeface(Typeface::DEFAULT);
        // Java: paint.setTextSize(fontsize);
        paint.setTextSize(fontsize);
        // Java: Rect rect = new Rect();
        let mut rect = Rect::new();
        // Java: paint.getTextBounds(String.valueOf('国'), 0, 1, rect);
        paint.getTextBounds(&TEST_TEXTHEIGHT_CHAR.to_string(), 0, 1, &mut rect);
        // Java: float fontHei = (float)rect.height();
        let fontHei = rect.height() as f32;
        // Java: this.y = textOffsetY + firstLineBaseline - fontHei / 2.0F - this.height / 2.0F;
        self.y = textOffsetY + firstLineBaseline - fontHei / 2.0 - self.height / 2.0;
    }

    // Java: void setRotateDegrees(int degrees)
    pub fn setRotateDegrees(&mut self, degrees: i32) {
        // Java: this.rotateDegrees = degrees;
        self.rotateDegrees = degrees;
    }

    // Java: int getRotateDegrees()
    pub fn getRotateDegrees(&self) -> i32 {
        // Java: return this.rotateDegrees;
        self.rotateDegrees
    }

    // Java: boolean isEastVertical()
    pub fn isEastVertical(&self) -> bool {
        // Java: return this.isEastVertical;
        self.isEastVertical
    }

    // Java: void setEastVertical(boolean isEastVertical)
    pub fn setEastVertical(&mut self, isEastVertical: bool) {
        // Java: this.isEastVertical = isEastVertical;
        self.isEastVertical = isEastVertical;
    }

    // Java: boolean isAllVertical()
    pub fn isAllVertical(&self) -> bool {
        // Java: return this.isAllVertical;
        self.isAllVertical
    }

    // Java: void setAllVertical(boolean allVertical)
    pub fn setAllVertical(&mut self, allVertical: bool) {
        // Java: this.isAllVertical = allVertical;
        self.isAllVertical = allVertical;
    }

    // Java: void setArtTransform(ArtTransform artTransform)
    pub fn setArtTransform(&mut self, artTransform: Option<ArtTransform>) {
        // Java: this.artTransform = artTransform;
        self.artTransform = artTransform;
    }

    // Java: ArtTransform getArtTransform()
    pub fn getArtTransform(&self) -> &Option<ArtTransform> {
        // Java: return this.artTransform;
        &self.artTransform
    }

    // Java: void setIsRightToLeft(boolean isRight2Left)
    pub fn setIsRightToLeft(&mut self, isRight2Left: bool) {
        // Java: this.isRightToLeft = isRight2Left;
        self.isRightToLeft = isRight2Left;
    }

    // Java: boolean isRightToLeft()
    pub fn isRightToLeft(&self) -> bool {
        // Java: return this.isRightToLeft;
        self.isRightToLeft
    }
}

fn is_chinese(c: char) -> bool {
    let cp = c as u32;
    (0x4E00..=0x9FFF).contains(&cp) ||
    (0x3400..=0x4DBF).contains(&cp) ||
    (0xF900..=0xFAFF).contains(&cp) ||
    (0x3000..=0x303F).contains(&cp) ||
    (0xFF00..=0xFFEF).contains(&cp) ||
    (0x30A0..=0x30FF).contains(&cp) ||
    (0x3040..=0x309F).contains(&cp) ||
    (0x2460..=0x24FF).contains(&cp)
}
