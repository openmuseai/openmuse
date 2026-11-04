// java public enum SpreadsheetVersion
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpreadsheetVersion {
    EXCEL97,
    EXCEL2007,
}

// java public static SpreadsheetVersion CUR_VERSION = EXCEL97;
// typeoracle: BookReaderHelper 会赋值这个静态字段。Mutex 只表达可写，默认 EXCEL97。
pub static CUR_VERSION: std::sync::Mutex<SpreadsheetVersion> =
    std::sync::Mutex::new(SpreadsheetVersion::EXCEL97);

// java public enum SpreadsheetVersion
impl SpreadsheetVersion {
    // java private SpreadsheetVersion(int maxRows, int maxColumns, int maxFunctionArgs, int maxCondFormats, int maxText)
    fn SpreadsheetVersion(
        maxRows: i32,
        maxColumns: i32,
        maxFunctionArgs: i32,
        maxCondFormats: i32,
        maxText: i32,
    ) -> (i32, i32, i32, i32, i32) {
        // java this._maxRows = maxRows;
        let _maxRows = maxRows;
        // java this._maxColumns = maxColumns;
        let _maxColumns = maxColumns;
        // java this._maxFunctionArgs = maxFunctionArgs;
        let _maxFunctionArgs = maxFunctionArgs;
        // java this._maxCondFormats = maxCondFormats;
        let _maxCondFormats = maxCondFormats;
        // java this._maxTextLength = maxText;
        let _maxTextLength = maxText;
        (
            _maxRows,
            _maxColumns,
            _maxFunctionArgs,
            _maxCondFormats,
            _maxTextLength,
        )
    }

    fn fields(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            SpreadsheetVersion::EXCEL97 => {
                Self::SpreadsheetVersion(65536, 256, 30, 3, 32767)
            }
            SpreadsheetVersion::EXCEL2007 => {
                Self::SpreadsheetVersion(1048576, 16384, 255, 2147483647, 32767)
            }
        }
    }

    // java public int getMaxRows()
    pub fn getMaxRows(&self) -> i32 {
        // java return this._maxRows;
        self.fields().0
    }

    // java public int getLastRowIndex()
    pub fn getLastRowIndex(&self) -> i32 {
        // java return this._maxRows - 1;
        self.getMaxRows() - 1
    }

    // java public int getMaxColumns()
    pub fn getMaxColumns(&self) -> i32 {
        // java return this._maxColumns;
        self.fields().1
    }

    // java public int getLastColumnIndex()
    pub fn getLastColumnIndex(&self) -> i32 {
        // java return this._maxColumns - 1;
        self.getMaxColumns() - 1
    }

    // java public int getMaxFunctionArgs()
    pub fn getMaxFunctionArgs(&self) -> i32 {
        // java return this._maxFunctionArgs;
        self.fields().2
    }

    // java public int getMaxConditionalFormats()
    pub fn getMaxConditionalFormats(&self) -> i32 {
        // java return this._maxCondFormats;
        self.fields().3
    }

    // java public String getLastColumnName()
    pub fn getLastColumnName(&self) -> String {
        // java return CellReference.convertNumToColString(this.getLastColumnIndex());
        crate::org::apache::poi::ss::util::CellReference::CellReference::convertNumToColString(self.getLastColumnIndex())
    }

    // java public int getMaxTextLength()
    pub fn getMaxTextLength(&self) -> i32 {
        // java return this._maxTextLength;
        self.fields().4
    }
}
