// Java: import cn.wps.shaperecognize.shape.Point
use crate::cn::wps::shaperecognize::shape::Point::Point;

// Java field: int MAX_ANGLE
const MAX_ANGLE: i32 = 4;

// Java: public class JarvisMarch
pub struct JarvisMarch {
    // Java field: double currentMinAngle
    currentMinAngle: f64,
    // Java field: List<Point> hullPointList
    pub(crate) hullPointList: Vec<Point>,
    // Java field: Point[] points
    points: Vec<Point>,
    // Java field: int firstIndex
    firstIndex: i32,
}

// Java: public class JarvisMarch
impl JarvisMarch {
    // Java: JarvisMarch(List<Point> points)
    pub fn new(points: &[Point]) -> Self {
        // Java: super();
        let mut instance = JarvisMarch {
            currentMinAngle: 0.0,
            hullPointList: Vec::new(),
            points: Vec::new(),
            firstIndex: 0,
        };
        // Java: this.initialize(points);
        instance.initialize(points);
        instance
    }

    // Java: void initialize(List<Point> points)
    fn initialize(&mut self, points: &[Point]) {
        // Java: this.hullPointList = new LinkedList();
        self.hullPointList = Vec::new();
        // Java: this.initArray(points);
        self.initArray(points);
        // Java: this.firstIndex = this.getFirstIndex();
        self.firstIndex = self.getFirstIndex();
        // Java: this.addToHull(this.firstIndex);
        let first_index = self.firstIndex;
        self.addToHull(first_index);
    }

    // Java: void initArray(List<Point> points)
    fn initArray(&mut self, points: &[Point]) {
        // Java: this.points = new Point[points.size()];
        self.points = Vec::with_capacity(points.len());
        // Java: for(int i = 0; i < points.size(); ++i)
        for i in 0..points.len() {
            // Java: this.points[i] = (Point)points.get(i);
            self.points.push(points[i].clone());
        }
    }

    // Java: int getFirstIndex()
    fn getFirstIndex(&self) -> i32 {
        // Java: int minIndex = 0;
        let mut minIndex: i32 = 0;
        // Java: for(int i = 1; i < this.points.length; ++i)
        for i in 1..self.points.len() as i32 {
            // Java: if (this.points[i].y < this.points[minIndex].y)
            if self.points[i as usize].y < self.points[minIndex as usize].y {
                // Java: minIndex = i;
                minIndex = i;
            }
            // Java: else if (this.points[i].y == this.points[minIndex].y && this.points[i].x < this.points[minIndex].x)
            else if self.points[i as usize].y == self.points[minIndex as usize].y
                && self.points[i as usize].x < self.points[minIndex as usize].x
            {
                // Java: minIndex = i;
                minIndex = i;
            }
        }
        // Java: return minIndex;
        minIndex
    }

    // Java: void addToHull(int index)
    fn addToHull(&mut self, index: i32) {
        // Java: this.hullPointList.add(this.points[index]);
        self.hullPointList.push(self.points[index as usize].clone());
    }

    // Java: List<Point> getHullPointList()
    pub fn getHullPointList(&self) -> &Vec<Point> {
        // Java: return this.hullPointList;
        &self.hullPointList
    }

    // Java: int calculateHull()
    pub fn calculateHull(&mut self) -> i32 {
        // Java: for(int i = this.getNextIndex(this.firstIndex); i != this.firstIndex; i = this.getNextIndex(i))
        let mut i = self.getNextIndex(self.firstIndex);
        while i != self.firstIndex {
            // Java: this.addToHull(i);
            self.addToHull(i);
            i = self.getNextIndex(i);
        }
        // Java: return 0;
        0
    }

    // Java: int getNextIndex(int currentIndex)
    pub fn getNextIndex(&mut self, currentIndex: i32) -> i32 {
        // Java: double minAngle = (double)MAX_ANGLE;
        let mut minAngle: f64 = MAX_ANGLE as f64;
        // Java: int minIndex = 0;
        let mut minIndex: i32 = 0;
        // Java: for(int i = 0; i < this.points.length; ++i)
        for i in 0..self.points.len() as i32 {
            // Java: if (i != currentIndex)
            if i != currentIndex {
                // Java: double dx = (double)(this.points[i].x - this.points[currentIndex].x);
                let dx: f64 = (self.points[i as usize].x - self.points[currentIndex as usize].x) as f64;
                // Java: double dy = (double)(this.points[i].y - this.points[currentIndex].y);
                let dy: f64 = (self.points[i as usize].y - self.points[currentIndex as usize].y) as f64;
                // Java: if (dx != dy || dx != (double)0.0F)
                if dx != dy || dx != 0.0 {
                    // Java: double pseudoAngle = this.getPseudoAngle(dx, dy);
                    let pseudoAngle = self.getPseudoAngle(dx, dy);
                    // Java: if (pseudoAngle >= this.currentMinAngle && pseudoAngle < minAngle)
                    if pseudoAngle >= self.currentMinAngle && pseudoAngle < minAngle {
                        // Java: minAngle = pseudoAngle;
                        minAngle = pseudoAngle;
                        // Java: minIndex = i;
                        minIndex = i;
                    }
                    // Java: else if (pseudoAngle == minAngle && (Math.abs(this.points[i].x - this.points[currentIndex].x) > Math.abs(this.points[minIndex].x - this.points[currentIndex].x) || Math.abs(this.points[i].y - this.points[currentIndex].y) > Math.abs(this.points[minIndex].y - this.points[currentIndex].y)))
                    else if pseudoAngle == minAngle
                        && ((self.points[i as usize].x - self.points[currentIndex as usize].x).abs()
                            > (self.points[minIndex as usize].x - self.points[currentIndex as usize].x).abs()
                            || (self.points[i as usize].y - self.points[currentIndex as usize].y).abs()
                                > (self.points[minIndex as usize].y - self.points[currentIndex as usize].y).abs())
                    {
                        // Java: minIndex = i;
                        minIndex = i;
                    }
                }
            }
        }
        // Java: this.currentMinAngle = minAngle;
        self.currentMinAngle = minAngle;
        // Java: return minIndex;
        minIndex
    }

    // Java: float dst(Point p1, Point p2)
    pub fn dst(&self, p1: &Point, p2: &Point) -> f32 {
        // Java: return (float)Math.sqrt(Math.pow((double)Math.abs(p1.x - p2.x), (double)2.0F) + Math.pow((double)Math.abs(p1.y - p2.y), (double)2.0F));
        (((p1.x - p2.x).abs() as f64).powi(2) + ((p1.y - p2.y).abs() as f64).powi(2)).sqrt() as f32
    }

    // Java: double getPseudoAngle(double dx, double dy)
    pub fn getPseudoAngle(&self, dx: f64, dy: f64) -> f64 {
        // Java: if (dx > (double)0.0F && dy >= (double)0.0F)
        if dx > 0.0 && dy >= 0.0 {
            // Java: return dy / (dx + dy);
            dy / (dx + dy)
        }
        // Java: else if (dx <= (double)0.0F && dy > (double)0.0F)
        else if dx <= 0.0 && dy > 0.0 {
            // Java: return (double)1.0F + Math.abs(dx) / (Math.abs(dx) + dy);
            1.0 + dx.abs() / (dx.abs() + dy)
        }
        // Java: else if (dx < (double)0.0F && dy <= (double)0.0F)
        else if dx < 0.0 && dy <= 0.0 {
            // Java: return (double)2.0F + dy / (dx + dy);
            2.0 + dy / (dx + dy)
        }
        // Java: else if (dx >= (double)0.0F && dy < (double)0.0F)
        else if dx >= 0.0 && dy < 0.0 {
            // Java: return (double)3.0F + dx / (dx + Math.abs(dy));
            3.0 + dx / (dx + dy.abs())
        }
        // Java: else
        else {
            // Java: throw new Error("Impossible");
            panic!("Impossible");
        }
    }
}
