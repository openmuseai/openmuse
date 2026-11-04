#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::PointF;
// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import com.hp.hpl.inkml.Channel
use crate::com::hp::hpl::inkml::Channel::ChannelType;
// Java: import com.hp.hpl.inkml.TraceDataList
use crate::com::hp::hpl::inkml::TraceDataList::TraceDataList;
// Java: import java.util.ArrayList
// Java: import java.util.Iterator

use super::Brush::Brush;

pub struct TraceAdapter {
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
    pub(crate) trace_data: TraceDataList,
    pub(crate) brush: Brush,
}

impl TraceAdapter {
    pub fn getScaleX(&self) -> f32 { self.scale_x }
    pub fn getScaleY(&self) -> f32 { self.scale_y }
    pub fn getTraceData(&self) -> &TraceDataList { &self.trace_data }
    pub fn getBrush(&self) -> &Brush { &self.brush }
}

pub struct Geometry;

impl Geometry {
    pub fn getDot(x1: f32, y1: f32, x2: f32, y2: f32, x0: f32, y0: f32) -> [f32; 2] {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let t = if dx == 0.0 && dy == 0.0 {
            0.0
        } else {
            ((x0 - x1) * dx + (y0 - y1) * dy) / (dx * dx + dy * dy)
        };
        [x1 + t * dx, y1 + t * dy]
    }

    pub fn getPointDistance(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
        let dx = x1 - x0;
        let dy = y1 - y0;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn isInLineAB(x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> bool {
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);
        x >= min_x && x <= max_x && y >= min_y && y <= max_y
    }

    pub fn isInCircle(px: f32, py: f32, cx: f32, cy: f32, r: f32) -> bool {
        Self::getPointDistance(px, py, cx, cy) <= r
    }

    pub fn isIntersectCircle(x1: f32, y1: f32, x2: f32, y2: f32, cx: f32, cy: f32, r: f32) -> bool {
        let dot = Self::getDot(x1, y1, x2, y2, cx, cy);
        let dist = Self::getPointDistance(cx, cy, dot[0], dot[1]);
        dist <= r && Self::isInLineAB(x1, y1, x2, y2, dot[0], dot[1])
    }

    pub fn isIntersectLine(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) -> bool {
        let d1 = Self::direction(x3, y3, x4, y4, x1, y1);
        let d2 = Self::direction(x3, y3, x4, y4, x2, y2);
        let d3 = Self::direction(x1, y1, x2, y2, x3, y3);
        let d4 = Self::direction(x1, y1, x2, y2, x4, y4);
        if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
            && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
        {
            return true;
        }
        if d1 == 0.0 && Self::onSegment(x3, y3, x4, y4, x1, y1) { return true; }
        if d2 == 0.0 && Self::onSegment(x3, y3, x4, y4, x2, y2) { return true; }
        if d3 == 0.0 && Self::onSegment(x1, y1, x2, y2, x3, y3) { return true; }
        if d4 == 0.0 && Self::onSegment(x1, y1, x2, y2, x4, y4) { return true; }
        false
    }

    fn direction(xi: f32, yi: f32, xj: f32, yj: f32, xk: f32, yk: f32) -> f32 {
        (xk - xi) * (yj - yi) - (yk - yi) * (xj - xi)
    }

    fn onSegment(xi: f32, yi: f32, xj: f32, yj: f32, xk: f32, yk: f32) -> bool {
        xk >= xi.min(xj) && xk <= xi.max(xj) && yk >= yi.min(yj) && yk <= yi.max(yj)
    }
}

// Java field: int DEFAULT_NO_COLLISION_SIGN
pub const DEFAULT_NO_COLLISION_SIGN: i32 = -1;

// Java: public class HitService
pub struct HitService {
}

// Java: public class HitService
impl HitService {
    // Java: HitService()
    pub fn HitService() -> Self {
        // Java: super();
        Self {}
    }

    // Java: int hit(ArrayList<TraceAdapter> traces, PointF pointA, PointF pointB, float deviation)
    pub fn hit_points(&self, traces: &Vec<TraceAdapter>, pointA: &PointF, pointB: &PointF, deviation: f32) -> i32 {
        // Java: return hit(traces, pointA.x, pointA.y, pointB.x, pointB.y, 1.0F, 1.0F, deviation);
        Self::hit_static(traces, pointA.x, pointA.y, pointB.x, pointB.y, 1.0, 1.0, deviation)
    }

    // Java: static int hit(ArrayList<TraceAdapter> traces, float pointA_x, float pointA_y, float pointB_x, float pointB_y, float scaleX, float scaleY, float deviation)
    pub fn hit_static(traces: &Vec<TraceAdapter>, pointA_x: f32, pointA_y: f32, pointB_x: f32, pointB_y: f32, scaleX: f32, scaleY: f32, deviation: f32) -> i32 {
        // Java: int hit = -1;
        let mut hit: i32 = -1;
        // Java: int traceIndex = 0;
        let mut traceIndex: usize = 0;
        // Java: for(int traceSize = traces.size(); traceIndex < traceSize; ++traceIndex)
        let traceSize = traces.len();
        while traceIndex < traceSize {
            // Java: TraceAdapter trace = (TraceAdapter)traces.get(traceIndex);
            let trace = &traces[traceIndex];
            // Java: float pAx = pointA_x * trace.getScaleX();
            let pAx = pointA_x * trace.getScaleX();
            // Java: float pAy = pointA_y * trace.getScaleY();
            let pAy = pointA_y * trace.getScaleY();
            // Java: float pBx = pointB_x * trace.getScaleX();
            let pBx = pointB_x * trace.getScaleX();
            // Java: float pBy = pointB_y * trace.getScaleY();
            let pBy = pointB_y * trace.getScaleY();
            // Java: TraceDataList list = trace.getTraceData();
            let list = trace.getTraceData();
            // Java: Brush brush = trace.getBrush();
            let brush = trace.getBrush();
            // Java: float width = trace.getScaleX() != 1.0F ? brush.getWidth() : brush.getWidth() / 26.458334F;
            let width = if trace.getScaleX() != 1.0 { brush.getWidth() } else { brush.getWidth() / 26.458334 };
            // Java: float height = trace.getScaleY() != 1.0F ? brush.getHeight() : brush.getHeight() / 26.458334F;
            let height = if trace.getScaleY() != 1.0 { brush.getHeight() } else { brush.getHeight() / 26.458334 };
            // Java: if (width == height)
            if width == height {
                // Java: float radius = (width + height) / 4.0F;
                let radius = (width + height) / 4.0;
                // Java: float dev = deviation * Math.max(trace.getScaleX(), trace.getScaleY()) + radius;
                let dev = deviation * trace.getScaleX().max(trace.getScaleY()) + radius;
                // Java: if (list.size() == 1 && hitPoint(list, pAx, pAy, pBx, pBy, scaleX, scaleY, dev))
                if list.size() == 1 && Self::hitPoint(list, pAx, pAy, pBx, pBy, scaleX, scaleY, dev) {
                    // Java: hit = traceIndex;
                    hit = traceIndex as i32;
                }
                // Java: else if (list.size() > 1 && hitLine(list, pAx, pAy, pBx, pBy, scaleX, scaleY, dev))
                else if list.size() > 1 && Self::hitLine(list, pAx, pAy, pBx, pBy, scaleX, scaleY, dev) {
                    // Java: hit = traceIndex;
                    hit = traceIndex as i32;
                }
            }
            // Java: else
            else {
                // Java: float var22 = deviation * Math.max(trace.getScaleX(), trace.getScaleY());
                let var22 = deviation * trace.getScaleX().max(trace.getScaleY());
                // Java: if (hitRect(list, pAx, pAy, pBx, pBy, width, height, scaleX, scaleY, var22))
                if Self::hitRect(list, pAx, pAy, pBx, pBy, width, height, scaleX, scaleY, var22) {
                    // Java: hit = traceIndex;
                    hit = traceIndex as i32;
                }
            }
            // Java: if (hit >= 0)
            if hit >= 0 {
                // Java: break;
                break;
            }
            traceIndex += 1;
        }
        // Java: return hit;
        hit
    }

    // Java: ArrayList<Integer> hitEx(ArrayList<TraceAdapter> traces, PointF pointA, PointF pointB, float deviation)
    pub fn hitEx_points(&self, traces: &Vec<TraceAdapter>, pointA: &PointF, pointB: &PointF, deviation: f32) -> Vec<i32> {
        // Java: return hitEx(traces, pointA.x, pointA.y, pointB.x, pointB.y, 1.0F, 1.0F, deviation);
        Self::hitEx_static(traces, pointA.x, pointA.y, pointB.x, pointB.y, 1.0, 1.0, deviation)
    }

    // Java: static ArrayList<Integer> hitEx(ArrayList<TraceAdapter> traces, float pointA_x, float pointA_y, float pointB_x, float pointB_y, float scaleX, float scaleY, float deviation)
    pub fn hitEx_static(traces: &Vec<TraceAdapter>, pointA_x: f32, pointA_y: f32, pointB_x: f32, pointB_y: f32, scaleX: f32, scaleY: f32, deviation: f32) -> Vec<i32> {
        // Java: int hit = -1;
        let mut hit: i32 = -1;
        // Java: ArrayList<Integer> traceIdList = new ArrayList();
        let mut traceIdList: Vec<i32> = Vec::new();
        // Java: int traceIndex = 0;
        let mut traceIndex: usize = 0;
        // Java: for(int traceSize = traces.size(); traceIndex < traceSize; ++traceIndex)
        let traceSize = traces.len();
        while traceIndex < traceSize {
            // Java: TraceAdapter trace = (TraceAdapter)traces.get(traceIndex);
            let trace = &traces[traceIndex];
            // Java: float pAx = pointA_x * trace.getScaleX();
            let pAx = pointA_x * trace.getScaleX();
            // Java: float pAy = pointA_y * trace.getScaleY();
            let pAy = pointA_y * trace.getScaleY();
            // Java: float pBx = pointB_x * trace.getScaleX();
            let pBx = pointB_x * trace.getScaleX();
            // Java: float pBy = pointB_y * trace.getScaleY();
            let pBy = pointB_y * trace.getScaleY();
            // Java: TraceDataList list = trace.getTraceData();
            let list = trace.getTraceData();
            // Java: Brush brush = trace.getBrush();
            let brush = trace.getBrush();
            // Java: float width = trace.getScaleX() != 1.0F ? brush.getWidth() : brush.getWidth() / 26.458334F;
            let width = if trace.getScaleX() != 1.0 { brush.getWidth() } else { brush.getWidth() / 26.458334 };
            // Java: float height = trace.getScaleY() != 1.0F ? brush.getHeight() : brush.getHeight() / 26.458334F;
            let height = if trace.getScaleY() != 1.0 { brush.getHeight() } else { brush.getHeight() / 26.458334 };
            // Java: if (width == height)
            if width == height {
                // Java: float radius = (width + height) / 4.0F;
                let radius = (width + height) / 4.0;
                // Java: float dev = deviation * Math.max(trace.getScaleX(), trace.getScaleY()) + radius;
                let dev = deviation * trace.getScaleX().max(trace.getScaleY()) + radius;
                // Java: if (list.size() == 1 && hitPoint(list, pAx, pAy, pBx, pBy, scaleX, scaleY, dev))
                if list.size() == 1 && Self::hitPoint(list, pAx, pAy, pBx, pBy, scaleX, scaleY, dev) {
                    // Java: hit = traceIndex;
                    hit = traceIndex as i32;
                }
                // Java: else if (list.size() > 1 && hitLine(list, pAx, pAy, pBx, pBy, scaleX, scaleY, dev))
                else if list.size() > 1 && Self::hitLine(list, pAx, pAy, pBx, pBy, scaleX, scaleY, dev) {
                    // Java: hit = traceIndex;
                    hit = traceIndex as i32;
                }
            }
            // Java: else
            else {
                // Java: float var23 = deviation * Math.max(trace.getScaleX(), trace.getScaleY());
                let var23 = deviation * trace.getScaleX().max(trace.getScaleY());
                // Java: if (hitRect(list, pAx, pAy, pBx, pBy, width, height, scaleX, scaleY, var23))
                if Self::hitRect(list, pAx, pAy, pBx, pBy, width, height, scaleX, scaleY, var23) {
                    // Java: hit = traceIndex;
                    hit = traceIndex as i32;
                }
            }
            // Java: if (hit >= 0)
            if hit >= 0 {
                // Java: if (traceIdList.isEmpty())
                if traceIdList.is_empty() {
                    // Java: traceIdList.add(hit);
                    traceIdList.push(hit);
                }
                // Java: else if (!traceIdList.contains(hit))
                else if !traceIdList.contains(&hit) {
                    // Java: traceIdList.add(hit);
                    traceIdList.push(hit);
                }
            }
            traceIndex += 1;
        }
        // Java: return traceIdList;
        traceIdList
    }

    // Java: static boolean hitLine(TraceDataList list, float pointA_x, float pointA_y, float pointB_x, float pointB_y, float scaleX, float scaleY, float deviation)
    fn hitLine(list: &TraceDataList, pointA_x: f32, pointA_y: f32, pointB_x: f32, pointB_y: f32, scaleX: f32, scaleY: f32, deviation: f32) -> bool {
        // Java: Channel.ChannelType channelX = list.getXYChannleType();
        let _channelX = list.getXYChannleType();
        // Java: int xIndex = list.getXIndex();
        let xIndex = list.getXIndex() as usize;
        // Java: int yIndex = list.getYIndex();
        let yIndex = list.getYIndex() as usize;
        // Java: Iterator<Object[]> iterator = list.iterator();
        // Java: Object[] traceFragment = iterator.next();
        if list.size() == 0 { return false; }
        let traceFragment = list.get(0);
        // Java: float lastTrace_x = (Float)traceFragment[xIndex] * scaleX;
        let mut lastTrace_x = traceFragment[xIndex] * scaleX;
        // Java: float traceB_y;
        // Java: for(float lastTrace_y = (Float)traceFragment[yIndex] * scaleY; iterator.hasNext(); lastTrace_y = traceB_y)
        let mut lastTrace_y = traceFragment[yIndex] * scaleY;
        for i in 1..list.size() {
            // Java: traceFragment = iterator.next();
            let traceFragment = list.get(i);
            // Java: float traceB_x = (Float)traceFragment[xIndex] * scaleX;
            let traceB_x = traceFragment[xIndex] * scaleX;
            // Java: traceB_y = (Float)traceFragment[yIndex] * scaleY;
            let traceB_y = traceFragment[yIndex] * scaleY;
            // Java: if (isIntersectLine(lastTrace_x, lastTrace_y, traceB_x, traceB_y, pointA_x, pointA_y, deviation) || Geometry.isIntersectLine(lastTrace_x, lastTrace_y, traceB_x, traceB_y, pointA_x, pointA_y, pointB_x, pointB_y))
            if Self::isIntersectLine_dev(lastTrace_x, lastTrace_y, traceB_x, traceB_y, pointA_x, pointA_y, deviation)
                || Geometry::isIntersectLine(lastTrace_x, lastTrace_y, traceB_x, traceB_y, pointA_x, pointA_y, pointB_x, pointB_y) {
                // Java: return true;
                return true;
            }
            // Java: lastTrace_x = traceB_x;
            lastTrace_x = traceB_x;
            lastTrace_y = traceB_y;
        }
        // Java: return false;
        false
    }

    // Java: static boolean isIntersectLine(float x1, float y1, float x2, float y2, float x0, float y0, float deviation)
    fn isIntersectLine_dev(x1: f32, y1: f32, x2: f32, y2: f32, x0: f32, y0: f32, deviation: f32) -> bool {
        // Java: if (x1 == x2 && y1 == y2)
        if x1 == x2 && y1 == y2 {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: float[] dot = Geometry.getDot(x1, y1, x2, y2, x0, y0);
            let dot = Geometry::getDot(x1, y1, x2, y2, x0, y0);
            // Java: float x = dot[0];
            let x = dot[0];
            // Java: float y = dot[1];
            let y = dot[1];
            // Java: float pointDistance = Geometry.getPointDistance(x0, y0, x, y);
            let pointDistance = Geometry::getPointDistance(x0, y0, x, y);
            // Java: return pointDistance <= deviation && Geometry.isInLineAB(x1, y1, x2, y2, x, y);
            pointDistance <= deviation && Geometry::isInLineAB(x1, y1, x2, y2, x, y)
        }
    }

    // Java: static boolean hitPoint(TraceDataList list, float pointA_x, float pointA_y, float pointB_x, float pointB_y, float scaleX, float scaleY, float deviation)
    fn hitPoint(list: &TraceDataList, pointA_x: f32, pointA_y: f32, pointB_x: f32, pointB_y: f32, scaleX: f32, scaleY: f32, deviation: f32) -> bool {
        // Java: int xIndex = list.getXIndex();
        let xIndex = list.getXIndex() as usize;
        // Java: int yIndex = list.getYIndex();
        let yIndex = list.getYIndex() as usize;
        // Java: Channel.ChannelType type = list.getXYChannleType();
        let _type = list.getXYChannleType();
        // Java: Object[] traceFragment = list.get(0);
        let traceFragment = list.get(0);
        // Java: float x = (Float)traceFragment[xIndex] * scaleX;
        let x = traceFragment[xIndex] * scaleX;
        // Java: float y = (Float)traceFragment[yIndex] * scaleY;
        let y = traceFragment[yIndex] * scaleY;
        // Java: return Geometry.isInCircle(pointA_x, pointA_y, x, y, deviation) || Geometry.isInCircle(pointB_x, pointB_y, x, y, deviation) || Geometry.isIntersectCircle(pointA_x, pointA_y, pointB_x, pointB_y, x, y, deviation);
        Geometry::isInCircle(pointA_x, pointA_y, x, y, deviation)
            || Geometry::isInCircle(pointB_x, pointB_y, x, y, deviation)
            || Geometry::isIntersectCircle(pointA_x, pointA_y, pointB_x, pointB_y, x, y, deviation)
    }

    // Java: static boolean hitRect(TraceDataList list, float pAx, float pAy, float pBx, float pBy, float width, float height, float scaleX, float scaleY, float deviation)
    fn hitRect(list: &TraceDataList, pAx: f32, pAy: f32, pBx: f32, pBy: f32, width: f32, height: f32, scaleX: f32, scaleY: f32, deviation: f32) -> bool {
        // Java: Channel.ChannelType channelX = list.getXYChannleType();
        let _channelX = list.getXYChannleType();
        // Java: int xIndex = list.getXIndex();
        let xIndex = list.getXIndex() as usize;
        // Java: int yIndex = list.getYIndex();
        let yIndex = list.getYIndex() as usize;
        // Java: Iterator<Object[]> iterator = list.iterator();
        // Java: Object[] traceFragment = iterator.next();
        if list.size() == 0 { return false; }
        let traceFragment = list.get(0);
        // Java: RectF rect = RectF.obtain();
        let mut rect = RectF::new();
        // Java: float hw = width / 2.0F;
        let hw = width / 2.0;
        // Java: float hh = height / 2.0F;
        let hh = height / 2.0;
        // Java: float lastTrace_x = (Float)traceFragment[xIndex] * scaleX;
        let mut lastTrace_x = traceFragment[xIndex] * scaleX;
        // Java: float lastTrace_y = (Float)traceFragment[yIndex] * scaleY;
        let mut lastTrace_y = traceFragment[yIndex] * scaleY;
        // Java: rect.set(lastTrace_x - hw, lastTrace_y - hh, lastTrace_x + hw, lastTrace_y + hh);
        rect.set(lastTrace_x - hw, lastTrace_y - hh, lastTrace_x + hw, lastTrace_y + hh);
        // Java: if (!rect.contains(pAx, pAy) && !rect.contains(pBx, pBy))
        if !rect.contains(pAx, pAy) && !rect.contains(pBx, pBy) {
            // Java: while(iterator.hasNext())
            for i in 1..list.size() {
                // Java: traceFragment = iterator.next();
                let traceFragment = list.get(i);
                // Java: float traceB_x = (Float)traceFragment[xIndex] * scaleX;
                let traceB_x = traceFragment[xIndex] * scaleX;
                // Java: float traceB_y = (Float)traceFragment[yIndex] * scaleY;
                let traceB_y = traceFragment[yIndex] * scaleY;
                // Java: if (isIntersectLine(lastTrace_x, lastTrace_y, traceB_x, traceB_y, pAx, pAy, hw, hh, deviation) || Geometry.isIntersectLine(lastTrace_x, lastTrace_y, traceB_x, traceB_y, pAx, pAy, pBx, pBy))
                if Self::isIntersectLine_rect(lastTrace_x, lastTrace_y, traceB_x, traceB_y, pAx, pAy, hw, hh, deviation)
                    || Geometry::isIntersectLine(lastTrace_x, lastTrace_y, traceB_x, traceB_y, pAx, pAy, pBx, pBy) {
                    // Java: rect.recycle();
                    // Java: return true;
                    return true;
                }
                // Java: lastTrace_x = traceB_x;
                lastTrace_x = traceB_x;
                // Java: lastTrace_y = traceB_y;
                lastTrace_y = traceB_y;
            }
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: rect.recycle();
            // Java: return true;
            return true;
        }
    }

    // Java: static boolean isIntersectLine(float x1, float y1, float x2, float y2, float x0, float y0, float width, float height, float deviation)
    fn isIntersectLine_rect(x1: f32, y1: f32, x2: f32, y2: f32, x0: f32, y0: f32, width: f32, height: f32, deviation: f32) -> bool {
        // Java: if (x1 == x2 && y1 == y2)
        if x1 == x2 && y1 == y2 {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: float[] dot = Geometry.getDot(x1, y1, x2, y2, x0, y0);
            let dot = Geometry::getDot(x1, y1, x2, y2, x0, y0);
            // Java: float x = dot[0];
            let x = dot[0];
            // Java: float y = dot[1];
            let y = dot[1];
            // Java: float pointDistance = Geometry.getPointDistance(x0, y0, x, y);
            let pointDistance = Geometry::getPointDistance(x0, y0, x, y);
            // Java: if (pointDistance > deviation)
            if pointDistance > deviation {
                // Java: return false;
                return false;
            }
            // Java: else
            {
                // Java: boolean contains = Geometry.isInLineAB(x1, y1, x2, y2, x, y);
                let mut contains = Geometry::isInLineAB(x1, y1, x2, y2, x, y);
                // Java: if (!contains)
                if !contains {
                    // Java: float hw = width + deviation / 2.0F;
                    let hw = width + deviation / 2.0;
                    // Java: float hh = height + deviation / 2.0F;
                    let hh = height + deviation / 2.0;
                    // Java: RectF rect = RectF.obtain();
                    let mut rect = RectF::new();
                    // Java: rect.set(x1 - hw, y1 - hh, x1 + hw, y1 + hh);
                    rect.set(x1 - hw, y1 - hh, x1 + hw, y1 + hh);
                    // Java: contains = rect.contains(x0, y0);
                    contains = rect.contains(x0, y0);
                    // Java: if (!contains)
                    if !contains {
                        // Java: rect.set(x2 - hw, y2 - hh, x2 + hw, y2 + hh);
                        rect.set(x2 - hw, y2 - hh, x2 + hw, y2 + hh);
                        // Java: contains = rect.contains(x0, y0);
                        contains = rect.contains(x0, y0);
                    }
                    // Java: rect.recycle();
                }
                // Java: return contains;
                contains
            }
        }
    }
}
