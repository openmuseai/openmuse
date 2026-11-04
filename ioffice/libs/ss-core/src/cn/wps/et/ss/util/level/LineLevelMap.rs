// java package cn.wps.et.ss.util.level;

// java import cn.wps.et.ss.util.Area;
use crate::cn::wps::et::ss::util::Area::Area;
// java import cn.wps.et.ss.util.indexmap.SparseIndexMap;
use crate::cn::wps::et::ss::util::indexmap::SparseIndexMap::SparseIndexMap;
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

// java private static class Segment / SegmentData<T>
#[derive(Clone)]
enum LineNode<T> {
    Segment { first: i32, last: i32, pool: Vec<LineNode<T>> },
    Data { first: i32, last: i32, data: T },
}

// java public class LineLevelMap<T>
pub struct LineLevelMap<T> {
    // java private final int[] horizontal_splitter / vertical_splitter;
    horizontal_splitter: Vec<i32>,
    vertical_splitter: Vec<i32>,
    // java private final SparseIndexMap<List<Object>> horizontal_map / vertical_map;
    horizontal_map: SparseIndexMap<Vec<LineNode<T>>>,
    vertical_map: SparseIndexMap<Vec<LineNode<T>>>,
}

impl<T: Clone> LineLevelMap<T> {
    // java public LineLevelMap(SpreadsheetVersion sv)
    pub fn LineLevelMap(sv: SpreadsheetVersion) -> Self {
        // java EXCEL97 uses horizontal {64,16,4}, vertical {16384,4096,1024,256,64,16}.
        let (horizontal_splitter, vertical_splitter) = if sv == SpreadsheetVersion::EXCEL97 {
            (vec![64, 16, 4], vec![16384, 4096, 1024, 256, 64, 16])
        } else {
            // java if (sv != EXCEL2007) throw IllegalArgumentException();
            assert_eq!(sv, SpreadsheetVersion::EXCEL2007);
            // java EXCEL2007 uses horizontal {4096,1024,256,64,16}, vertical {131072,16384,2048,256,32,8}.
            (vec![4096, 1024, 256, 64, 16], vec![131072, 16384, 2048, 256, 32, 8])
        };
        // java horizontal_map = new SparseIndexMap(sv.getMaxRows()); vertical_map = ...getMaxColumns().
        Self { horizontal_splitter, vertical_splitter, horizontal_map: SparseIndexMap::SparseIndexMap(sv.getMaxRows()), vertical_map: SparseIndexMap::SparseIndexMap(sv.getMaxColumns()) }
    }

    // java private void validateSplitter(SpreadsheetVersion sv)
    fn validateSplitter(&self, sv: SpreadsheetVersion) {
        // java each splitter must divide previous max columns/rows; otherwise RuntimeException.
        let mut c = sv.getMaxColumns();
        for split in &self.horizontal_splitter { assert_eq!(c % split, 0); c = *split; }
        let mut r = sv.getMaxRows();
        for split in &self.vertical_splitter { assert_eq!(r % split, 0); r = *split; }
    }

    // java public void insertHorizontalLine(int row, int firstCol, int lastCol, T data)
    pub fn insertHorizontalLine(&mut self, row: i32, firstCol: i32, lastCol: i32, data: T) {
        // java line = horizontal_map.get(row); if null create ArrayList and put(row,line).
        if self.horizontal_map.get(row).is_none() { self.horizontal_map.put(row, Vec::new()); }
        // java descend horizontal_splitter levels, then append SegmentData(firstCol,lastCol,data).
        let line = self.horizontal_map.get_mut(row).expect("line inserted");
        insert_line(line, &self.horizontal_splitter, firstCol, lastCol, data);
    }

    // java public void insertVerticalLine(int firstRow, int lastRow, int col, T fml)
    pub fn insertVerticalLine(&mut self, firstRow: i32, lastRow: i32, col: i32, fml: T) {
        // java line = vertical_map.get(col); if null create ArrayList and put(col,line).
        if self.vertical_map.get(col).is_none() { self.vertical_map.put(col, Vec::new()); }
        // java descend vertical_splitter levels, then append SegmentData(firstRow,lastRow,fml).
        let line = self.vertical_map.get_mut(col).expect("line inserted");
        insert_line(line, &self.vertical_splitter, firstRow, lastRow, fml);
    }

    // java public Object removeHorizontalLine(int row, int firstCol, int lastCol, T data)
    pub fn removeHorizontalLine(&mut self, row: i32, firstCol: i32, lastCol: i32, data: &T) -> Option<T> where T: PartialEq {
        // java if horizontal_map.get(row)==null return null; descend and remove first equal SegmentData.
        remove_line(self.horizontal_map.get_mut(row)?, &self.horizontal_splitter, firstCol, lastCol, data)
    }

    // java public Object removeVerticalLine(int firstRow, int lastRow, int col, Object data)
    pub fn removeVerticalLine(&mut self, firstRow: i32, lastRow: i32, col: i32, data: &T) -> Option<T> where T: PartialEq {
        // java if vertical_map.get(col)==null return null; descend and remove first equal SegmentData.
        remove_line(self.vertical_map.get_mut(col)?, &self.vertical_splitter, firstRow, lastRow, data)
    }

    // java public void queryReferred(int row, int col, Collection<T> out)
    pub fn queryReferred(&self, row: i32, col: i32, out: &mut Vec<T>) {
        // java query horizontal_map[row] first, then vertical_map[col], preserving each queue order.
        if let Some(line) = self.horizontal_map.get(row) { query_line_point(line, &self.horizontal_splitter, col, out); }
        if let Some(line) = self.vertical_map.get(col) { query_line_point(line, &self.vertical_splitter, row, out); }
    }

    // java public void queryReferred(Area area, Collection<T> out)
    pub fn queryReferred__area(&self, area: &Area, out: &mut Vec<T>) {
        // java horizontal_map.iterator(firstRow,lastRow), then vertical_map.iterator(firstCol,lastCol).
        for (_, line) in self.horizontal_map.iteratorRange(area.getFirstRow(), area.getLastRow()) {
            query_line_range(line, &self.horizontal_splitter, area.getFirstCol(), area.getLastCol(), out);
        }
        for (_, line) in self.vertical_map.iteratorRange(area.getFirstCol(), area.getLastCol()) {
            query_line_range(line, &self.vertical_splitter, area.getFirstRow(), area.getLastRow(), out);
        }
    }

    // java public void queryAll(Collection<T> out)
    pub fn queryAll(&self, out: &mut Vec<T>) {
        // java visit all horizontal map entries in SparseIndexMap order, then all vertical entries.
        for (_, line) in self.horizontal_map.iterator() { query_line_all(line, &self.horizontal_splitter, out); }
        for (_, line) in self.vertical_map.iterator() { query_line_all(line, &self.vertical_splitter, out); }
    }
}

// rust: shared body of Java insertHorizontalLine / insertVerticalLine after choosing the corresponding map and splitter.
fn insert_line<T>(line: &mut Vec<LineNode<T>>, splitter: &[i32], first: i32, last: i32, data: T) {
    // java List<Object> currentPool = line; for each level find Segment(first/split,last/split), else append new Segment.
    let mut current_pool = line;
    for split in splitter {
        let key = (first / split, last / split);
        let index = current_pool.iter().position(|node| matches!(node, LineNode::Segment { first, last, .. } if (*first, *last) == key)).unwrap_or_else(|| {
            current_pool.push(LineNode::Segment { first: key.0, last: key.1, pool: Vec::new() });
            current_pool.len() - 1
        });
        let LineNode::Segment { pool, .. } = &mut current_pool[index] else { unreachable!() };
        current_pool = pool;
    }
    // java SegmentData<T> sd; sd.first=first; sd.last=last; sd.data=data; currentPool.add(sd).
    current_pool.push(LineNode::Data { first, last, data });
}

// rust: shared body of Java removeHorizontalLine / removeVerticalLine after map lookup.
fn remove_line<T: PartialEq>(line: &mut Vec<LineNode<T>>, splitter: &[i32], first: i32, last: i32, data: &T) -> Option<T> {
    // java descend matching Segment at each level; return null if absent.
    let mut current_pool = line;
    for split in splitter {
        let key = (first / split, last / split);
        let index = current_pool.iter().position(|node| matches!(node, LineNode::Segment { first, last, .. } if (*first, *last) == key))?;
        let LineNode::Segment { pool, .. } = &mut current_pool[index] else { unreachable!() };
        current_pool = pool;
    }
    // java iterator removes first SegmentData with exact bounds and data.equals(data).
    let index = current_pool.iter().position(|node| matches!(node, LineNode::Data { first: f, last: l, data: d } if *f == first && *l == last && d == data))?;
    let LineNode::Data { data, .. } = current_pool.remove(index) else { unreachable!() };
    Some(data)
}

// rust: Java queryReferred(row,col) uses the same queue traversal for horizontal and vertical lines.
fn query_line_point<T: Clone>(line: &Vec<LineNode<T>>, splitter: &[i32], point: i32, out: &mut Vec<T>) {
    // java queue.addLast(line); at each splitter keep Segment pools whose range contains point/split.
    let mut queue = vec![line];
    for split in splitter {
        if queue.is_empty() { break; }
        let sub = point / split;
        queue = queue.into_iter().flat_map(|pool| pool.iter().filter_map(|node| match node {
            LineNode::Segment { first, last, pool } if *first <= sub && sub <= *last => Some(pool),
            _ => None,
        })).collect();
    }
    // java for each remaining SegmentData, if first <= point <= last, out.add(data).
    for pool in queue { for node in pool { if let LineNode::Data { first, last, data } = node { if *first <= point && point <= *last { out.push(data.clone()); } } } }
}

// rust: Java queryReferred(Area) uses the same queue traversal for row and column intervals.
fn query_line_range<T: Clone>(line: &Vec<LineNode<T>>, splitter: &[i32], first: i32, last: i32, out: &mut Vec<T>) {
    // java queue.addLast(line); at each splitter keep Segment pools overlapping [first/split,last/split].
    let mut queue = vec![line];
    for split in splitter {
        if queue.is_empty() { break; }
        let (sub_first, sub_last) = (first / split, last / split);
        queue = queue.into_iter().flat_map(|pool| pool.iter().filter_map(|node| match node {
            LineNode::Segment { first, last, pool } if *first <= sub_last && *last >= sub_first => Some(pool),
            _ => None,
        })).collect();
    }
    // java for each remaining SegmentData, if it overlaps exact interval, out.add(data).
    for pool in queue { for node in pool { if let LineNode::Data { first: f, last: l, data } = node { if *f <= last && *l >= first { out.push(data.clone()); } } } }
}

// rust: Java queryAll traverses all horizontal and vertical Segment pools breadth first.
fn query_line_all<T: Clone>(line: &Vec<LineNode<T>>, splitter: &[i32], out: &mut Vec<T>) {
    let mut queue = vec![line];
    for _ in splitter {
        if queue.is_empty() { break; }
        queue = queue.into_iter().flat_map(|pool| pool.iter().filter_map(|node| match node { LineNode::Segment { pool, .. } => Some(pool), _ => None })).collect();
    }
    for pool in queue { for node in pool { if let LineNode::Data { data, .. } = node { out.push(data.clone()); } } }
}
