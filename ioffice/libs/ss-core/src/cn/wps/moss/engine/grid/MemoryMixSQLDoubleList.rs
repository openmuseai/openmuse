use crate::cn::wps::moss::app::persistence::KmoDataPersistence::DoubleArrayPersistence;

// java class MemoryMixSQLDoubleList
pub struct MemoryMixSQLDoubleList {
    // java private final int MAX_MEMORY_DOUBLE_COUNT = 655360;
    MAX_MEMORY_DOUBLE_COUNT: i32,
    // java private WDoubleArrayList dVals = new WDoubleArrayList();
    dVals: Vec<f64>, // WDoubleArrayList
    // java private int existedDoubleCount = 0;
    existedDoubleCount: i32,
    // java private KmoDataPersistence.DoubleArrayPersistence kmoPersistence = new KmoDataPersistence.DoubleArrayPersistence();
    kmoPersistence: DoubleArrayPersistence,
}

// java private static final int MAX_CACHE_VALUE = 128;
const MAX_CACHE_VALUE: i32 = 128;

// java class MemoryMixSQLDoubleList
impl MemoryMixSQLDoubleList {
    // java public MemoryMixSQLDoubleList()
    pub fn MemoryMixSQLDoubleList() -> Self {
        let mut dVals = Vec::new();
        // java for(int i = 0; i <= 128; ++i)
        for i in 0..=MAX_CACHE_VALUE {
            // java this.dVals.add((double)i);
            dVals.push(i as f64);
        }
        MemoryMixSQLDoubleList {
            MAX_MEMORY_DOUBLE_COUNT: 655360,
            dVals,
            existedDoubleCount: 0,
            kmoPersistence: DoubleArrayPersistence::DoubleArrayPersistence(),
        }
    }

    // java public void close()
    pub fn close(&mut self) {
        // java this.kmoPersistence.closeChannel();
        self.kmoPersistence.closeChannel();
    }

    // java private void pushDataToSQL()
    fn pushDataToSQL(&mut self) {
        // java boolean isSuc = this.kmoPersistence.insertData(this.dVals);
        let isSuc = self.kmoPersistence.insertData(&self.dVals);
        // java if (isSuc)
        if isSuc {
            // java this.existedDoubleCount += this.dVals.size();
            self.existedDoubleCount += self.dVals.len() as i32;
            // java this.dVals.clear();
            self.dVals.clear();
        }
    }

    // java public int addDouble(double value)
    pub fn addDouble(&mut self, value: f64) -> i32 {
        // java if (value >= (double)0.0F && value <= (double)128.0F && Math.floor(value) == value)
        if value >= 0.0 && value <= 128.0 && value.floor() == value {
            // java return (int)value;
            return value as i32;
        } else {
            // java this.dVals.add(value);
            self.dVals.push(value);
            // java int size = this.dVals.size();
            let size = self.dVals.len() as i32;
            // java if (size >= this.MAX_MEMORY_DOUBLE_COUNT)
            if size >= self.MAX_MEMORY_DOUBLE_COUNT {
                // java this.pushDataToSQL();
                self.pushDataToSQL();
                // java return -1 + this.existedDoubleCount;
                -1 + self.existedDoubleCount
            } else {
                // java return size - 1 + this.existedDoubleCount;
                size - 1 + self.existedDoubleCount
            }
        }
    }

    // java public double getDouble(int id)
    pub fn getDouble(&mut self, mut id: i32) -> f64 {
        // java int exDoubleCount = this.existedDoubleCount;
        let exDoubleCount = self.existedDoubleCount;
        // java if (id >= this.dVals.size() + exDoubleCount)
        if id >= self.dVals.len() as i32 + exDoubleCount {
            // java throw new IllegalArgumentException("获取Double数值出错");
            panic!("IllegalArgumentException: 获取Double数值出错");
        // java else if (id < exDoubleCount)
        } else if id < exDoubleCount {
            // java return this.kmoPersistence.queryData(id);
            return self.kmoPersistence.queryData(id);
        } else {
            // java id -= exDoubleCount;
            id -= exDoubleCount;
            // java return this.dVals.get(id);
            self.dVals[id as usize]
        }
    }
}
