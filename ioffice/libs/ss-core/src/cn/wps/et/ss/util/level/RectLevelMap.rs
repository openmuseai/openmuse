// java package cn.wps.et.ss.util.level;

// java import cn.wps.et.ss.util.Area;
use crate::cn::wps::et::ss::util::Area::Area;
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

// java private static class LevelRect / DataRect<T>
enum RectNode<T> {
    Level { first_row: i32, last_row: i32, first_col: i32, last_col: i32, pool: Vec<RectNode<T>> },
    Data { first_row: i32, last_row: i32, first_col: i32, last_col: i32, data: T },
}

// java public class RectLevelMap<T>
pub struct RectLevelMap<T> {
    // java private final int[] horizontal_splitter;
    horizontal_splitter: [i32; 6],
    // java private final int[] vertical_splitter;
    vertical_splitter: [i32; 6],
    // java private final List<Object> pool = new ArrayList();
    pool: Vec<RectNode<T>>,
}

impl<T> RectLevelMap<T> {
    // java public RectLevelMap(SpreadsheetVersion sv)
    pub fn RectLevelMap(sv: SpreadsheetVersion) -> Self {
        // java if (sv == SpreadsheetVersion.EXCEL97) ... else if (sv != EXCEL2007) throw IllegalArgumentException();
        let (horizontal_splitter, vertical_splitter) = if sv == SpreadsheetVersion::EXCEL97 {
            // java {128,64,32,16,8,4}; {16384,4096,1024,256,64,16};
            ([128, 64, 32, 16, 8, 4], [16384, 4096, 1024, 256, 64, 16])
        } else {
            assert_eq!(sv, SpreadsheetVersion::EXCEL2007);
            // java {4096,1024,256,64,16,4}; {131072,16384,2048,256,32,8};
            ([4096, 1024, 256, 64, 16, 4], [131072, 16384, 2048, 256, 32, 8])
        };
        Self { horizontal_splitter, vertical_splitter, pool: Vec::new() }
    }

    // java private void validateSplitter(SpreadsheetVersion sv)
    fn validateSplitter(&self, sv: SpreadsheetVersion) {
        // java if (horizontal_splitter.length != vertical_splitter.length) throw RuntimeException();
        assert_eq!(self.horizontal_splitter.len(), self.vertical_splitter.len());
        // java int c = sv.getMaxColumns(); for each splitter verify c % splitter == 0, then c = splitter;
        let mut c = sv.getMaxColumns();
        for split in self.horizontal_splitter { assert_eq!(c % split, 0); c = split; }
        // java int r = sv.getMaxRows(); for each splitter verify r % splitter == 0, then r = splitter;
        let mut r = sv.getMaxRows();
        for split in self.vertical_splitter { assert_eq!(r % split, 0); r = split; }
    }

    // java public void insert(int firstRow, int lastRow, int firstCol, int lastCol, T data)
    pub fn insert(&mut self, firstRow: i32, lastRow: i32, firstCol: i32, lastCol: i32, data: T) {
        // java List<Object> currentPool = this.pool;
        let mut current_pool = &mut self.pool;
        // java for (level = 0; level < horizontal_splitter.length; ++level)
        for level in 0..self.horizontal_splitter.len() {
            // java subFirstRow/subLastRow = firstRow/lastRow / vertical_splitter[level]; likewise columns.
            let key = (firstRow / self.vertical_splitter[level], lastRow / self.vertical_splitter[level], firstCol / self.horizontal_splitter[level], lastCol / self.horizontal_splitter[level]);
            // java find matching LevelRect in currentPool; if absent add new LevelRect in insertion order.
            let index = current_pool.iter().position(|node| matches!(node, RectNode::Level { first_row, last_row, first_col, last_col, .. } if (*first_row, *last_row, *first_col, *last_col) == key)).unwrap_or_else(|| {
                current_pool.push(RectNode::Level { first_row: key.0, last_row: key.1, first_col: key.2, last_col: key.3, pool: Vec::new() });
                current_pool.len() - 1
            });
            let RectNode::Level { pool, .. } = &mut current_pool[index] else { unreachable!() };
            current_pool = pool;
        }
        // java DataRect<T> dr = new DataRect(); assign bounds/data; currentPool.add(dr);
        current_pool.push(RectNode::Data { first_row: firstRow, last_row: lastRow, first_col: firstCol, last_col: lastCol, data });
    }

    // java public T remove(int firstRow, int lastRow, int firstCol, int lastCol, T data)
    pub fn remove(&mut self, firstRow: i32, lastRow: i32, firstCol: i32, lastCol: i32, data: &T) -> Option<T> where T: PartialEq {
        // java List<Object> curpool = this.pool;
        let mut current_pool = &mut self.pool;
        // java traverse exactly the same six LevelRect keys; return null if absent.
        for level in 0..self.horizontal_splitter.len() {
            let key = (firstRow / self.vertical_splitter[level], lastRow / self.vertical_splitter[level], firstCol / self.horizontal_splitter[level], lastCol / self.horizontal_splitter[level]);
            let index = current_pool.iter().position(|node| matches!(node, RectNode::Level { first_row, last_row, first_col, last_col, .. } if (*first_row, *last_row, *first_col, *last_col) == key))?;
            let RectNode::Level { pool, .. } = &mut current_pool[index] else { unreachable!() };
            current_pool = pool;
        }
        // java iterator removes first DataRect with identical bounds and data.equals(data).
        let index = current_pool.iter().position(|node| matches!(node, RectNode::Data { first_row, last_row, first_col, last_col, data: d } if (*first_row, *last_row, *first_col, *last_col) == (firstRow, lastRow, firstCol, lastCol) && d == data))?;
        let RectNode::Data { data, .. } = current_pool.remove(index) else { unreachable!() };
        Some(data)
    }

    // java public void queryAreaReferred(int row, int col, Collection<T> out)
    pub fn queryAreaReferred(&self, row: i32, col: i32, out: &mut Vec<T>) where T: Clone {
        // java queue.addLast(this.pool); for each level dequeue each current pool in FIFO order.
        let mut queue = vec![&self.pool];
        for level in 0..self.horizontal_splitter.len() {
            if queue.is_empty() { break; }
            // java subRow = row / vertical_splitter[level]; subCol = col / horizontal_splitter[level].
            let (sub_row, sub_col) = (row / self.vertical_splitter[level], col / self.horizontal_splitter[level]);
            // java enqueue LevelRect.pool when its row and column ranges contain the subposition.
            queue = queue.into_iter().flat_map(|pool| pool.iter().filter_map(|node| match node {
                RectNode::Level { first_row, last_row, first_col, last_col, pool } if *first_row <= sub_row && sub_row <= *last_row && *first_col <= sub_col && sub_col <= *last_col => Some(pool),
                _ => None,
            })).collect();
        }
        // java for each DataRect, if exact bounds contain row/col, out.add(data).
        for pool in queue { for node in pool { if let RectNode::Data { first_row, last_row, first_col, last_col, data } = node { if *first_row <= row && row <= *last_row && *first_col <= col && col <= *last_col { out.push(data.clone()); } } } }
    }

    // java public void queryAreaReferred(Area area, Collection<T> out)
    pub fn queryAreaReferred__area(&self, area: &Area, out: &mut Vec<T>) where T: Clone {
        // java queue.addLast(this.pool); read area first/last row and column.
        let mut queue = vec![&self.pool];
        let (first_row, last_row, first_col, last_col) = (area.getFirstRow(), area.getLastRow(), area.getFirstCol(), area.getLastCol());
        // java for each level, enqueue overlapping LevelRect pools in FIFO order.
        for level in 0..self.horizontal_splitter.len() {
            if queue.is_empty() { break; }
            let (sr0, sr1, sc0, sc1) = (first_row / self.vertical_splitter[level], last_row / self.vertical_splitter[level], first_col / self.horizontal_splitter[level], last_col / self.horizontal_splitter[level]);
            queue = queue.into_iter().flat_map(|pool| pool.iter().filter_map(|node| match node {
                RectNode::Level { first_row, last_row, first_col, last_col, pool } if *first_row <= sr1 && *last_row >= sr0 && *first_col <= sc1 && *last_col >= sc0 => Some(pool),
                _ => None,
            })).collect();
        }
        // java for each DataRect, if exact bounds intersect area, out.add(data).
        for pool in queue { for node in pool { if let RectNode::Data { first_row: r0, last_row: r1, first_col: c0, last_col: c1, data } = node { if *r0 <= last_row && *r1 >= first_row && *c0 <= last_col && *c1 >= first_col { out.push(data.clone()); } } } }
    }

    // java public void queryAll(Collection<T> out)
    pub fn queryAll(&self, out: &mut Vec<T>) where T: Clone {
        // java queue.addLast(this.pool); breadth-first descend six levels in insertion order.
        let mut queue = vec![&self.pool];
        for _ in 0..self.horizontal_splitter.len() {
            if queue.is_empty() { break; }
            queue = queue.into_iter().flat_map(|pool| pool.iter().filter_map(|node| match node { RectNode::Level { pool, .. } => Some(pool), _ => None })).collect();
        }
        // java append all DataRect.data values in leaf pool order.
        for pool in queue { for node in pool { if let RectNode::Data { data, .. } = node { out.push(data.clone()); } } }
    }
}
