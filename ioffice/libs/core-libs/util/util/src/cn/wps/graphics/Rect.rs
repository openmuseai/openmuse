// Java: import java.io.PrintWriter
// Java: import java.util.regex.Matcher
// Java: import java.util.regex.Pattern

// Java field: Pattern FLATTENED_PATTERN
static FLATTENED_PATTERN: &str = r"(-?\d+) (-?\d+) (-?\d+) (-?\d+)";

// Java: public class Rect
#[derive(Clone)]
pub struct Rect {
    // Java field: int bottom
    pub bottom: i32,
    // Java field: int left
    pub left: i32,
    // Java field: int right
    pub right: i32,
    // Java field: int top
    pub top: i32,
}

impl Rect {
    // Java: static boolean intersects(Rect a, Rect b)
    pub fn intersects_static(a: &Rect, b: &Rect) -> bool {
        // Java: return a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom;
        a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom
    }

    // Java: static Rect unflattenFromString(String str)
    pub fn unflattenFromString(str: &str) -> Option<Rect> {
        // Java: Matcher matcher = FLATTENED_PATTERN.matcher(str);
        let matcher: Vec<&str> = str.split_whitespace().collect();
        // Java: return !matcher.matches() ? null : new Rect(Integer.parseInt(matcher.group(1)), Integer.parseInt(matcher.group(2)), Integer.parseInt(matcher.group(3)), Integer.parseInt(matcher.group(4)));
        if matcher.len() != 4 {
            return None;
        }
        let left = matcher[0].parse::<i32>().ok()?;
        let top = matcher[1].parse::<i32>().ok()?;
        let right = matcher[2].parse::<i32>().ok()?;
        let bottom = matcher[3].parse::<i32>().ok()?;
        Some(Rect { left, top, right, bottom })
    }

    // Java: Rect()
    pub fn new() -> Self {
        // Java: super();
        Rect { bottom: 0, left: 0, right: 0, top: 0 }
    }

    // Java: Rect(int left, int top, int right, int bottom)
    pub fn new_with(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        // Java: super();
        // Java: this.left = left;
        // Java: this.top = top;
        // Java: this.right = right;
        // Java: this.bottom = bottom;
        Rect { left, top, right, bottom }
    }

    // Java: Rect(Rect r)
    pub fn from_rect(r: &Rect) -> Self {
        // Java: super();
        // Java: this.left = r.left;
        // Java: this.top = r.top;
        // Java: this.right = r.right;
        // Java: this.bottom = r.bottom;
        Rect { left: r.left, top: r.top, right: r.right, bottom: r.bottom }
    }

    // Java: int centerX()
    pub fn centerX(&self) -> i32 {
        // Java: return this.left + this.right >> 1;
        (self.left + self.right) >> 1
    }

    // Java: int centerY()
    pub fn centerY(&self) -> i32 {
        // Java: return this.top + this.bottom >> 1;
        (self.top + self.bottom) >> 1
    }

    // Java: boolean contains(int x, int y)
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        // Java: return this.left < this.right && this.top < this.bottom && x >= this.left && x < this.right && y >= this.top && y < this.bottom;
        self.left < self.right && self.top < self.bottom && x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }

    // Java: boolean contains(int left, int top, int right, int bottom)
    pub fn contains_rect_vals(&self, left: i32, top: i32, right: i32, bottom: i32) -> bool {
        // Java: return this.left < this.right && this.top < this.bottom && this.left <= left && this.top <= top && this.right >= right && this.bottom >= bottom;
        self.left < self.right && self.top < self.bottom && self.left <= left && self.top <= top && self.right >= right && self.bottom >= bottom
    }

    // Java: boolean contains(Rect r)
    pub fn contains(&self, r: &Rect) -> bool {
        // Java: return this.left < this.right && this.top < this.bottom && this.left <= r.left && this.top <= r.top && this.right >= r.right && this.bottom >= r.bottom;
        self.left < self.right && self.top < self.bottom && self.left <= r.left && self.top <= r.top && self.right >= r.right && self.bottom >= r.bottom
    }

    // Java: int describeContents()
    pub fn describeContents(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &Rect) -> bool {
        // Java: if (obj instanceof Rect)
        if true {
            // Java: Rect r = (Rect)obj;
            let r = obj;
            // Java: if (r != null)
            if true {
                // Java: return this.left == r.left && this.top == r.top && this.right == r.right && this.bottom == r.bottom;
                return self.left == r.left && self.top == r.top && self.right == r.right && self.bottom == r.bottom;
            }
        }
        // Java: return false;
        false
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.left + this.top + this.right + this.bottom;
        self.left + self.top + self.right + self.bottom
    }

    // Java: float exactCenterX()
    pub fn exactCenterX(&self) -> f32 {
        // Java: return (float)(this.left + this.right) * 0.5F;
        (self.left + self.right) as f32 * 0.5f32
    }

    // Java: float exactCenterY()
    pub fn exactCenterY(&self) -> f32 {
        // Java: return (float)(this.top + this.bottom) * 0.5F;
        (self.top + self.bottom) as f32 * 0.5f32
    }

    // Java: String flattenToString()
    pub fn flattenToString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder(32);
        let mut sb = String::with_capacity(32);
        // Java: sb.append(this.left);
        sb.push_str(&self.left.to_string());
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append(this.top);
        sb.push_str(&self.top.to_string());
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append(this.right);
        sb.push_str(&self.right.to_string());
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append(this.bottom);
        sb.push_str(&self.bottom.to_string());
        // Java: return sb.toString();
        sb
    }

    // Java: int height()
    pub fn height(&self) -> i32 {
        // Java: return this.bottom - this.top;
        self.bottom - self.top
    }

    // Java: void inset(int dx, int dy)
    pub fn inset(&mut self, dx: i32, dy: i32) {
        // Java: this.left += dx;
        self.left += dx;
        // Java: this.top += dy;
        self.top += dy;
        // Java: this.right -= dx;
        self.right -= dx;
        // Java: this.bottom -= dy;
        self.bottom -= dy;
    }

    // Java: boolean intersect(int left, int top, int right, int bottom)
    pub fn intersect(&mut self, left: i32, top: i32, right: i32, bottom: i32) -> bool {
        // Java: if (this.left < right && left < this.right && this.top < bottom && top < this.bottom)
        if self.left < right && left < self.right && self.top < bottom && top < self.bottom {
            // Java: if (this.left < left)
            if self.left < left {
                // Java: this.left = left;
                self.left = left;
            }
            // Java: if (this.top < top)
            if self.top < top {
                // Java: this.top = top;
                self.top = top;
            }
            // Java: if (this.right > right)
            if self.right > right {
                // Java: this.right = right;
                self.right = right;
            }
            // Java: if (this.bottom > bottom)
            if self.bottom > bottom {
                // Java: this.bottom = bottom;
                self.bottom = bottom;
            }
            // Java: return true;
            true
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: boolean intersect(Rect r)
    pub fn intersect_from_rect(&mut self, r: &Rect) -> bool {
        // Java: return this.intersect(r.left, r.top, r.right, r.bottom);
        self.intersect(r.left, r.top, r.right, r.bottom)
    }

    // Java: boolean intersects(int left, int top, int right, int bottom)
    pub fn intersects_vals(&self, left: i32, top: i32, right: i32, bottom: i32) -> bool {
        // Java: return this.left < right && left < this.right && this.top < bottom && top < this.bottom;
        self.left < right && left < self.right && self.top < bottom && top < self.bottom
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.left >= this.right || this.top >= this.bottom;
        self.left >= self.right || self.top >= self.bottom
    }

    // Java: void offset(int dx, int dy)
    pub fn offset(&mut self, dx: i32, dy: i32) {
        // Java: this.left += dx;
        self.left += dx;
        // Java: this.top += dy;
        self.top += dy;
        // Java: this.right += dx;
        self.right += dx;
        // Java: this.bottom += dy;
        self.bottom += dy;
    }

    // Java: void offsetTo(int newLeft, int newTop)
    pub fn offsetTo(&mut self, newLeft: i32, newTop: i32) {
        // Java: this.right += newLeft - this.left;
        self.right += newLeft - self.left;
        // Java: this.bottom += newTop - this.top;
        self.bottom += newTop - self.top;
        // Java: this.left = newLeft;
        self.left = newLeft;
        // Java: this.top = newTop;
        self.top = newTop;
    }

    // Java: void printShortString(PrintWriter pw)
    pub fn printShortString(&self, pw: &mut String) {
        // Java: pw.print('[');
        pw.push('[');
        // Java: pw.print(this.left);
        pw.push_str(&self.left.to_string());
        // Java: pw.print(',');
        pw.push(',');
        // Java: pw.print(this.top);
        pw.push_str(&self.top.to_string());
        // Java: pw.print("][");
        pw.push_str("][");
        // Java: pw.print(this.right);
        pw.push_str(&self.right.to_string());
        // Java: pw.print(',');
        pw.push(',');
        // Java: pw.print(this.bottom);
        pw.push_str(&self.bottom.to_string());
        // Java: pw.print(']');
        pw.push(']');
    }

    // Java: void scale(float scale)
    pub fn scale(&mut self, scale: f32) {
        // Java: if (scale != 1.0F)
        if scale != 1.0f32 {
            // Java: this.left = (int)((float)this.left * scale + 0.5F);
            self.left = (self.left as f32 * scale + 0.5f32) as i32;
            // Java: this.top = (int)((float)this.top * scale + 0.5F);
            self.top = (self.top as f32 * scale + 0.5f32) as i32;
            // Java: this.right = (int)((float)this.right * scale + 0.5F);
            self.right = (self.right as f32 * scale + 0.5f32) as i32;
            // Java: this.bottom = (int)((float)this.bottom * scale + 0.5F);
            self.bottom = (self.bottom as f32 * scale + 0.5f32) as i32;
        }
    }

    // Java: void scale(float sx, float sy)
    pub fn scale_xy(&mut self, sx: f32, sy: f32) {
        // Java: if (sx != 1.0F)
        if sx != 1.0f32 {
            // Java: this.left = (int)((float)this.left * sx + 0.5F);
            self.left = (self.left as f32 * sx + 0.5f32) as i32;
            // Java: this.right = (int)((float)this.right * sx + 0.5F);
            self.right = (self.right as f32 * sx + 0.5f32) as i32;
        }
        // Java: if (sy != 1.0F)
        if sy != 1.0f32 {
            // Java: this.top = (int)((float)this.top * sy + 0.5F);
            self.top = (self.top as f32 * sy + 0.5f32) as i32;
            // Java: this.bottom = (int)((float)this.bottom * sy + 0.5F);
            self.bottom = (self.bottom as f32 * sy + 0.5f32) as i32;
        }
    }

    // Java: void expand(int ex, int ey)
    pub fn expand(&mut self, ex: i32, ey: i32) {
        // Java: this.left -= ex;
        self.left -= ex;
        // Java: this.top -= ey;
        self.top -= ey;
        // Java: this.right += ex;
        self.right += ex;
        // Java: this.bottom += ey;
        self.bottom += ey;
    }

    // Java: void set(int left, int top, int right, int bottom)
    pub fn set(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        // Java: this.left = left;
        self.left = left;
        // Java: this.top = top;
        self.top = top;
        // Java: this.right = right;
        self.right = right;
        // Java: this.bottom = bottom;
        self.bottom = bottom;
    }

    // Java: void set(Rect src)
    pub fn set_from(&mut self, src: &Rect) {
        // Java: this.left = src.left;
        self.left = src.left;
        // Java: this.top = src.top;
        self.top = src.top;
        // Java: this.right = src.right;
        self.right = src.right;
        // Java: this.bottom = src.bottom;
        self.bottom = src.bottom;
    }

    // Java: void setEmpty()
    pub fn setEmpty(&mut self) {
        // Java: this.left = this.right = this.top = this.bottom = 0;
        self.left = 0;
        self.right = 0;
        self.top = 0;
        self.bottom = 0;
    }

    // Java: boolean setIntersect(Rect a, Rect b)
    pub fn setIntersect(&mut self, a: &Rect, b: &Rect) -> bool {
        // Java: if (a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom)
        if a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom {
            // Java: this.left = Math.max(a.left, b.left);
            self.left = a.left.max(b.left);
            // Java: this.top = Math.max(a.top, b.top);
            self.top = a.top.max(b.top);
            // Java: this.right = Math.min(a.right, b.right);
            self.right = a.right.min(b.right);
            // Java: this.bottom = Math.min(a.bottom, b.bottom);
            self.bottom = a.bottom.min(b.bottom);
            // Java: return true;
            true
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: void sort()
    pub fn sort(&mut self) {
        // Java: if (this.left > this.right)
        if self.left > self.right {
            // Java: int temp = this.left;
            let temp = self.left;
            // Java: this.left = this.right;
            self.left = self.right;
            // Java: this.right = temp;
            self.right = temp;
        }
        // Java: if (this.top > this.bottom)
        if self.top > self.bottom {
            // Java: int temp = this.top;
            let temp = self.top;
            // Java: this.top = this.bottom;
            self.top = self.bottom;
            // Java: this.bottom = temp;
            self.bottom = temp;
        }
    }

    // Java: String toShortString()
    pub fn toShortString(&self) -> String {
        // Java: return this.toShortString(new StringBuilder(32));
        self.toShortString_sb(&mut String::with_capacity(32))
    }

    // Java: String toShortString(StringBuilder sb)
    pub fn toShortString_sb(&self, sb: &mut String) -> String {
        // Java: sb.setLength(0);
        sb.clear();
        // Java: sb.append('[');
        sb.push('[');
        // Java: sb.append(this.left);
        sb.push_str(&self.left.to_string());
        // Java: sb.append(',');
        sb.push(',');
        // Java: sb.append(this.top);
        sb.push_str(&self.top.to_string());
        // Java: sb.append("][");
        sb.push_str("][");
        // Java: sb.append(this.right);
        sb.push_str(&self.right.to_string());
        // Java: sb.append(',');
        sb.push(',');
        // Java: sb.append(this.bottom);
        sb.push_str(&self.bottom.to_string());
        // Java: sb.append(']');
        sb.push(']');
        // Java: return sb.toString();
        sb.clone()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder(32);
        let mut sb = String::with_capacity(32);
        // Java: sb.append("Rect(");
        sb.push_str("Rect(");
        // Java: sb.append(this.left);
        sb.push_str(&self.left.to_string());
        // Java: sb.append(", ");
        sb.push_str(", ");
        // Java: sb.append(this.top);
        sb.push_str(&self.top.to_string());
        // Java: sb.append(" - ");
        sb.push_str(" - ");
        // Java: sb.append(this.right);
        sb.push_str(&self.right.to_string());
        // Java: sb.append(", ");
        sb.push_str(", ");
        // Java: sb.append(this.bottom);
        sb.push_str(&self.bottom.to_string());
        // Java: sb.append(")");
        sb.push(')');
        // Java: return sb.toString();
        sb
    }

    // Java: void union(int x, int y)
    pub fn union_point(&mut self, x: i32, y: i32) {
        // Java: if (x < this.left)
        if x < self.left {
            // Java: this.left = x;
            self.left = x;
        }
        // Java: else if (x > this.right)
        else if x > self.right {
            // Java: this.right = x;
            self.right = x;
        }
        // Java: if (y < this.top)
        if y < self.top {
            // Java: this.top = y;
            self.top = y;
        }
        // Java: else if (y > this.bottom)
        else if y > self.bottom {
            // Java: this.bottom = y;
            self.bottom = y;
        }
    }

    // Java: void union(int left, int top, int right, int bottom)
    pub fn union(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        // Java: if (left < right && top < bottom)
        if left < right && top < bottom {
            // Java: if (this.left < this.right && this.top < this.bottom)
            if self.left < self.right && self.top < self.bottom {
                // Java: if (this.left > left)
                if self.left > left {
                    // Java: this.left = left;
                    self.left = left;
                }
                // Java: if (this.top > top)
                if self.top > top {
                    // Java: this.top = top;
                    self.top = top;
                }
                // Java: if (this.right < right)
                if self.right < right {
                    // Java: this.right = right;
                    self.right = right;
                }
                // Java: if (this.bottom < bottom)
                if self.bottom < bottom {
                    // Java: this.bottom = bottom;
                    self.bottom = bottom;
                }
            }
            // Java: else
            else {
                // Java: this.left = left;
                self.left = left;
                // Java: this.top = top;
                self.top = top;
                // Java: this.right = right;
                self.right = right;
                // Java: this.bottom = bottom;
                self.bottom = bottom;
            }
        }
    }

    // Java: void union(Rect r)
    pub fn union_from_rect(&mut self, r: &Rect) {
        // Java: this.union(r.left, r.top, r.right, r.bottom);
        self.union(r.left, r.top, r.right, r.bottom);
    }

    // Java: int width()
    pub fn width(&self) -> i32 {
        // Java: return this.right - this.left;
        self.right - self.left
    }
}
