// java package cn.wps.moss.filefmt.biff8.record;

use util::org::apache::poi::util::BitFieldFactory::BitFieldFactory;

// java public final class WSBoolRecord extends StandardRecord
#[derive(Clone, Debug)]
pub struct WSBoolRecord {
    // java private byte field_1_wsbool;
    field_1_wsbool: i8,
    // java private byte field_2_wsbool;
    field_2_wsbool: i8,
}

impl Default for WSBoolRecord {
    fn default() -> Self {
        Self::WSBoolRecord()
    }
}

impl WSBoolRecord {
    // java public WSBoolRecord()
    pub fn WSBoolRecord() -> Self {
        Self {
            field_1_wsbool: 0,
            field_2_wsbool: 0,
        }
    }

    // java public void setRowSumsBelow(boolean below)
    pub fn setRowSumsBelow(&mut self, below: bool) {
        // java this.field_1_wsbool = rowsumsbelow.setByteBoolean(this.field_1_wsbool, below);
        let rowsumsbelow = BitFieldFactory::getInstance(64);
        self.field_1_wsbool = rowsumsbelow.setByteBoolean(self.field_1_wsbool, below);
    }

    // java public void setRowSumsRight(boolean right)
    pub fn setRowSumsRight(&mut self, right: bool) {
        // java this.field_1_wsbool = rowsumsright.setByteBoolean(this.field_1_wsbool, right);
        let rowsumsright = BitFieldFactory::getInstance(128);
        self.field_1_wsbool = rowsumsright.setByteBoolean(self.field_1_wsbool, right);
    }

    // java public boolean getRowSumsBelow()
    pub fn getRowSumsBelow(&self) -> bool {
        // java return rowsumsbelow.isSet(this.field_1_wsbool);
        let rowsumsbelow = BitFieldFactory::getInstance(64);
        rowsumsbelow.isSet(i32::from(self.field_1_wsbool))
    }

    // java public boolean getRowSumsRight()
    pub fn getRowSumsRight(&self) -> bool {
        // java return rowsumsright.isSet(this.field_1_wsbool);
        let rowsumsright = BitFieldFactory::getInstance(128);
        rowsumsright.isSet(i32::from(self.field_1_wsbool))
    }
}
