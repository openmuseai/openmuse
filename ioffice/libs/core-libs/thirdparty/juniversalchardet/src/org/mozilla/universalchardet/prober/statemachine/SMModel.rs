use super::PkgInt::PkgInt;

// Java field: int START
pub(crate) const START: i32 = 0;
// Java field: int ERROR
pub(crate) const ERROR: i32 = 1;
// Java field: int ITSME
pub(crate) const ITSME: i32 = 2;

// Java: public class SMModel
pub struct SMModel {
    // Java field: PkgInt classTable
    pub(crate) class_table: PkgInt,
    // Java field: int classFactor
    pub(crate) class_factor: i32,
    // Java field: PkgInt stateTable
    pub(crate) state_table: PkgInt,
    // Java field: int[] charLenTable
    pub(crate) char_len_table: &'static [i32],
    // Java field: String name
    pub(crate) name: &'static str,
}

// Java: public class SMModel
impl SMModel {
    // Java: SMModel(PkgInt classTable, int classFactor, PkgInt stateTable, int[] charLenTable, String name)
    pub fn new(
        class_table: PkgInt,
        class_factor: i32,
        state_table: PkgInt,
        char_len_table: &'static [i32],
        name: &'static str,
    ) -> Self {
        // Java: super();
        // Java: this.classTable = classTable;
        // Java: this.classFactor = classFactor;
        // Java: this.stateTable = stateTable;
        // Java: this.charLenTable = charLenTable;
        // Java: this.name = name;
        Self {
            class_table,
            class_factor,
            state_table,
            char_len_table,
            name,
        }
    }

    // Java: int getClass(byte b)
    pub fn get_class(&self, b: u8) -> i32 {
        // Java: int c = b & 255;
        let c = (b & 0xFF) as i32;
        // Java: return this.classTable.unpack(c);
        self.class_table.unpack(c)
    }

    // Java: int getNextState(int cls, int currentState)
    pub fn get_next_state(&self, cls: i32, current_state: i32) -> i32 {
        // Java: return this.stateTable.unpack(currentState * this.classFactor + cls);
        self.state_table.unpack(current_state * self.class_factor + cls)
    }

    // Java: int getCharLen(int cls)
    pub fn get_char_len(&self, cls: i32) -> i32 {
        // Java: return this.charLenTable[cls];
        self.char_len_table[cls as usize]
    }

    // Java: String getName()
    pub fn get_name(&self) -> &str {
        // Java: return this.name;
        self.name
    }
}
