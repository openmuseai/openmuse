// java package cn.wps.et.ss.util.level;

use crate::cn::wps::et::ss::util::Area::Area;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

// java private static class LevelPoint / DataPoint<T>
enum PointNode<T> {
    Level { row: i32, col: i32, pool: Vec<PointNode<T>> },
    Data { row: i32, col: i32, data: T },
}

// java public class PointLevelMap<T>
pub struct PointLevelMap<T> {
    // java private final int[] horizontal_splitter;
    horizontal_splitter: [i32; 6],
    // java private final int[] vertical_splitter;
    vertical_splitter: [i32; 6],
    // java private final List<Object> pool = new ArrayList();
    pool: Vec<PointNode<T>>,
}

impl<T> PointLevelMap<T> {
    // java public PointLevelMap(SpreadsheetVersion sv)
    pub fn PointLevelMap(sv: SpreadsheetVersion) -> Self {
        // java if (sv == SpreadsheetVersion.EXCEL97)
        let (horizontal_splitter, vertical_splitter) = if sv == SpreadsheetVersion::EXCEL97 {
            // java horizontal = {128, 64, 32, 16, 8, 4}; vertical = {16384, 4096, 1024, 256, 64, 16};
            ([128, 64, 32, 16, 8, 4], [16384, 4096, 1024, 256, 64, 16])
        } else {
            // java if (sv != SpreadsheetVersion.EXCEL2007) throw new IllegalArgumentException();
            assert_eq!(sv, SpreadsheetVersion::EXCEL2007);
            // java horizontal = {4096, 1024, 256, 64, 16, 4}; vertical = {131072, 16384, 2048, 256, 32, 8};
            ([4096, 1024, 256, 64, 16, 4], [131072, 16384, 2048, 256, 32, 8])
        };
        Self { horizontal_splitter, vertical_splitter, pool: Vec::new() }
    }

    // java private void validateSplitter(SpreadsheetVersion sv)
    fn validateSplitter(&self, sv: SpreadsheetVersion) {
        // java if (this.horizontal_splitter.length != this.vertical_splitter.length) throw new RuntimeException();
        assert_eq!(self.horizontal_splitter.len(), self.vertical_splitter.len());
        // java int c = sv.getMaxColumns();
        let mut c = sv.getMaxColumns();
        // java for (int i = 0; i < this.horizontal_splitter.length; ++i)
        for split in self.horizontal_splitter {
            // java if (c % this.horizontal_splitter[i] != 0) throw new RuntimeException();
            assert_eq!(c % split, 0);
            // java c = this.horizontal_splitter[i];
            c = split;
        }
        // java int r = sv.getMaxRows();
        let mut r = sv.getMaxRows();
        // java for (int i = 0; i < this.vertical_splitter.length; ++i)
        for split in self.vertical_splitter {
            // java if (r % this.vertical_splitter[i] != 0) throw new RuntimeException();
            assert_eq!(r % split, 0);
            // java r = this.vertical_splitter[i];
            r = split;
        }
    }

    // java public void insert(int row, int col, T data)
    pub fn insert(&mut self, row: i32, col: i32, data: T) {
        // java List<Object> currentPool = this.pool;
        let mut current_pool = &mut self.pool;
        // java for (int level = 0; level < this.horizontal_splitter.length; ++level)
        for level in 0..self.horizontal_splitter.len() {
            // java int subRow = row / this.vertical_splitter[level]; int subCol = col / this.horizontal_splitter[level];
            let sub_row = row / self.vertical_splitter[level];
            let sub_col = col / self.horizontal_splitter[level];
            // java for (...) if (lp.row == subRow && lp.col == subCol) currentPool = lp.pool;
            let index = current_pool.iter().position(|node| matches!(node, PointNode::Level { row, col, .. } if *row == sub_row && *col == sub_col));
            // java if (!found) { LevelPoint lp = new LevelPoint(); currentPool.add(lp); currentPool = lp.pool; }
            let index = index.unwrap_or_else(|| {
                current_pool.push(PointNode::Level { row: sub_row, col: sub_col, pool: Vec::new() });
                current_pool.len() - 1
            });
            let PointNode::Level { pool, .. } = &mut current_pool[index] else { unreachable!() };
            current_pool = pool;
        }
        // java DataPoint<T> dp = new DataPoint<T>(); dp.row = row; dp.col = col; dp.data = data; currentPool.add(dp);
        current_pool.push(PointNode::Data { row, col, data });
    }

    // java public T remove(int row, int col, T data)
    pub fn remove(&mut self, row: i32, col: i32, data: &T) -> Option<T> where T: PartialEq {
        // java List<Object> curpool = this.pool;
        let mut current_pool = &mut self.pool;
        // java for (int level = 0; level < this.horizontal_splitter.length; ++level)
        for level in 0..self.horizontal_splitter.len() {
            // java int subRow = row / this.vertical_splitter[level]; int subCol = col / this.horizontal_splitter[level];
            let sub_row = row / self.vertical_splitter[level];
            let sub_col = col / self.horizontal_splitter[level];
            // java if (!found) return null;
            let index = current_pool.iter().position(|node| matches!(node, PointNode::Level { row, col, .. } if *row == sub_row && *col == sub_col))?;
            let PointNode::Level { pool, .. } = &mut current_pool[index] else { unreachable!() };
            current_pool = pool;
        }
        // java if (dp.row == row && dp.col == col && dp.data.equals(data)) { iter.remove(); return dp.data; }
        let index = current_pool.iter().position(|node| matches!(node, PointNode::Data { row: r, col: c, data: d } if *r == row && *c == col && d == data))?;
        let PointNode::Data { data, .. } = current_pool.remove(index) else { unreachable!() };
        Some(data)
    }

    // java public void queryAreaRefered(int row, int col, Collection<T> out)
    pub fn queryAreaRefered(&self, row: i32, col: i32, out: &mut Vec<T>) where T: Clone {
        // java LinkedList<List<Object>> queue = new LinkedList(); queue.addLast(this.pool);
        let mut queue = vec![&self.pool];
        // java for (int level = 0; level < this.horizontal_splitter.length && !queue.isEmpty(); ++level)
        for level in 0..self.horizontal_splitter.len() {
            if queue.is_empty() { break; }
            // java int subRow = row / this.vertical_splitter[level]; int subCol = col / this.horizontal_splitter[level];
            let sub_row = row / self.vertical_splitter[level];
            let sub_col = col / self.horizontal_splitter[level];
            // java for each curPool in queue, enqueue matching lp.pool in insertion order
            queue = queue.into_iter().flat_map(|pool| pool.iter().filter_map(|node| match node {
                PointNode::Level { row, col, pool } if *row == sub_row && *col == sub_col => Some(pool),
                _ => None,
            })).collect();
        }
        // java for each curPool in queue, if (dp.row == row && dp.col == col) out.add(dp.data);
        for pool in queue {
            for node in pool {
                if let PointNode::Data { row: r, col: c, data } = node {
                    if *r == row && *c == col { out.push(data.clone()); }
                }
            }
        }
    }

    // java public void queryAreaReferred(Area area, Collection<T> out)
    pub fn queryAreaReferred(&self, area: &Area, out: &mut Vec<T>) where T: Clone {
        // java firstRow/lastRow/firstCol/lastCol = area getters; queue.addLast(this.pool);
        let mut queue = vec![&self.pool];
        // java for (int level = 0; level < this.horizontal_splitter.length && !queue.isEmpty(); ++level)
        for level in 0..self.horizontal_splitter.len() {
            if queue.is_empty() { break; }
            // java divide first/last row and col by this level's splitter
            let (first_row, last_row) = (area.getFirstRow() / self.vertical_splitter[level], area.getLastRow() / self.vertical_splitter[level]);
            let (first_col, last_col) = (area.getFirstCol() / self.horizontal_splitter[level], area.getLastCol() / self.horizontal_splitter[level]);
            // java if (subFirstRow <= lp.row && lp.row <= subLastRow && subFirstCol <= lp.col && lp.col <= subLastCol) queue.addLast(lp.pool);
            queue = queue.into_iter().flat_map(|pool| pool.iter().filter_map(|node| match node {
                PointNode::Level { row, col, pool } if first_row <= *row && *row <= last_row && first_col <= *col && *col <= last_col => Some(pool),
                _ => None,
            })).collect();
        }
        // java if (firstRow <= dp.row && dp.row <= lastRow && firstCol <= dp.col && dp.col <= lastCol) out.add(dp.data);
        for pool in queue {
            for node in pool {
                if let PointNode::Data { row, col, data } = node {
                    if area.contains(*row, *col) { out.push(data.clone()); }
                }
            }
        }
    }

    // java public void queryAll(Collection<T> out)
    pub fn queryAll(&self, out: &mut Vec<T>) where T: Clone {
        // java queue.addLast(this.pool);
        let mut queue = vec![&self.pool];
        // java for (int level = 0; level < this.horizontal_splitter.length && !queue.isEmpty(); ++level)
        for _ in 0..self.horizontal_splitter.len() {
            if queue.is_empty() { break; }
            // java for each curPool, enqueue every lp.pool in order
            queue = queue.into_iter().flat_map(|pool| pool.iter().filter_map(|node| match node {
                PointNode::Level { pool, .. } => Some(pool),
                _ => None,
            })).collect();
        }
        // java for each curPool, out.add(dp.data)
        for pool in queue {
            for node in pool {
                if let PointNode::Data { data, .. } = node { out.push(data.clone()); }
            }
        }
    }
}
