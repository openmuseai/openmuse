// Java: import cn.wps.shaperecognize.ShapeConfig
use crate::cn::wps::shaperecognize::ShapeConfig::ShapeConfig;
use super::GestureShape::{GestureShapeBase, GestureShapeTrait};
use super::Line::Line;
use super::Point::Point;

// Java: public class ShapeInsert extends GestureShape
pub struct ShapeInsert {
    pub base: GestureShapeBase,
    // Java field: Point insertPoint
    insertPoint: Option<Point>,
}

// Java: public class ShapeInsert extends GestureShape
impl ShapeInsert {
    // Java: ShapeInsert(List<Point> points, ShapeConfig config)
    pub fn new(points: &[Point], config: &ShapeConfig) -> Self {
        // Java: super(points, config);
        let mut base = GestureShapeBase::new(config.clone());
        base.init_with_points(points);
        let mut shape = ShapeInsert {
            base,
            insertPoint: None,
        };
        if !shape.base.points.is_empty() {
            let pts: Vec<Point> = shape.base.points.clone();
            if shape.isMatch(&pts) {
                shape.initType();
            }
        }
        shape
    }

    // Java: Point getInsertPoint()
    pub fn getInsertPoint(&self) -> &Option<Point> {
        // Java: return this.insertPoint;
        &self.insertPoint
    }
}

impl GestureShapeTrait for ShapeInsert {
    fn base(&self) -> &GestureShapeBase { &self.base }
    fn base_mut(&mut self) -> &mut GestureShapeBase { &mut self.base }

    // Java: boolean isMatch(List<Point> points)
    fn isMatch(&mut self, points: &[Point]) -> bool {
        // Java: if (points != null && points.size() != 0)
        if points.is_empty() { return false; }

        // Java: int indexMaxY = 0;
        let mut indexMaxY: usize = 0;
        // Java: float maxY = Float.MIN_VALUE;
        let mut maxY = f32::MIN;

        // Java: for(int i = 0; i < points.size(); ++i)
        for i in 0..points.len() {
            // Java: if (((Point)points.get(i)).y > maxY)
            if points[i].y > maxY {
                // Java: maxY = ((Point)points.get(i)).y;
                maxY = points[i].y;
                // Java: indexMaxY = i;
                indexMaxY = i;
            }
        }

        // Java: if (indexMaxY != 0 && indexMaxY < points.size() - 1)
        if indexMaxY == 0 || indexMaxY >= points.len() - 1 {
            // Java: return false;
            return false;
        }

        // Java: Line leftLine = Line.getAbsoluteFitLine(points, 0, indexMaxY, 15.0F);
        let leftLine = Line::getAbsoluteFitLine(points, 0, indexMaxY as i32, 15.0);
        // Java: if (leftLine == null)
        if leftLine.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        let leftLine = leftLine.unwrap();

        // Java: Line rightLine = Line.getAbsoluteFitLine(points, indexMaxY, points.size(), 15.0F);
        let rightLine = Line::getAbsoluteFitLine(points, indexMaxY as i32, points.len() as i32, 15.0);
        // Java: if (rightLine == null)
        if rightLine.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        let rightLine = rightLine.unwrap();

        // Java: float lk = leftLine.getSlope();
        let lk = leftLine.getSlope();
        // Java: float rk = rightLine.getSlope();
        let rk = rightLine.getSlope();

        // Java: if ((!(lk > 0.0F) || !(lk < 0.65F)) && (!(lk < 0.0F) || !(lk > -1.0F)))
        if (lk > 0.0 && lk < 0.65) || (lk < 0.0 && lk > -1.0) {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: if ((!(rk > 0.0F) || !(rk < 1.0F)) && (!(rk < 0.0F) || !(rk > -0.65F)))
        if (rk > 0.0 && rk < 1.0) || (rk < 0.0 && rk > -0.65) {
            // Java: return false;
            return false;
        }
        // Java: else

        // Java: this.insertPoint = (Point)points.get(indexMaxY);
        self.insertPoint = Some(points[indexMaxY].clone());
        // Java: return true;
        true
        // Java: else
        // Java: return false;
        // Java: else
        // Java: return false;
        // Java: else
        // Java: return false;
        // Java: else
        // Java: return false;
    }

    // Java: void initType()
    fn initType(&mut self) {
        // Java: this.type = 256;
        self.base.type_ = 256;
    }
}
