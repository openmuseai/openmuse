// java package cn.wps.moss.filefmt.biff8.record;

// java public final class GutsRecord extends StandardRecord
#[derive(Clone, Debug, Default)]
pub struct GutsRecord {
    // java private short field_1_left_row_gutter;
    field_1_left_row_gutter: i16,
    // java private short field_2_top_col_gutter;
    field_2_top_col_gutter: i16,
    // java private short field_3_row_level_max;
    field_3_row_level_max: i16,
    // java private short field_4_col_level_max;
    field_4_col_level_max: i16,
}

impl GutsRecord {
    // java public GutsRecord()
    pub fn GutsRecord() -> Self {
        Self::default()
    }

    // java public void setRowLevelMax(short max)
    pub fn setRowLevelMax(&mut self, max: i16) {
        // java this.field_3_row_level_max = max;
        self.field_3_row_level_max = max;
    }

    // java public void setColLevelMax(short max)
    pub fn setColLevelMax(&mut self, max: i16) {
        // java this.field_4_col_level_max = max;
        self.field_4_col_level_max = max;
    }

    // java public short getRowLevelMax()
    pub fn getRowLevelMax(&self) -> i16 {
        // java return this.field_3_row_level_max;
        self.field_3_row_level_max
    }

    // java public short getColLevelMax()
    pub fn getColLevelMax(&self) -> i16 {
        // java return this.field_4_col_level_max;
        self.field_4_col_level_max
    }
}
