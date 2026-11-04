// java public class TableJCData
#[derive(Clone, Copy, Default)]
pub struct TableJCData {
    // java public byte jc = 0;
    pub jc: u8,
    // java public int jcLeft = 0;
    pub jcLeft: i32,
    // java public int jcRight = 0;
    pub jcRight: i32,
    // java public int tableIndent = 0;
    pub tableIndent: i32,
}

// java public class TableJCData
impl TableJCData {
    // java public  TableJCData(int tableJC, int left, int right, int indent)
    pub fn TableJCData(tableJC: i32, left: i32, right: i32, indent: i32) -> Self {
      // java this.jc = (byte)tableJC;
      Self { jc: ((tableJC) as u8), jcLeft: left, jcRight: right, tableIndent: indent }
      // java this.jcLeft = left;
      // java this.jcRight = right;
      // java this.tableIndent = indent;
    }

}
