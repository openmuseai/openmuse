// Java field: int TYPE_WRITER
pub const TYPE_WRITER: i32 = 1;
// Java field: int TYPE_SPREADSHEET
pub const TYPE_SPREADSHEET: i32 = 2;
// Java field: int TYPE_PRESENTATION
pub const TYPE_PRESENTATION: i32 = 3;
// Java field: int TYPE_PDF
pub const TYPE_PDF: i32 = 4;
// Java field: int OPEN_RESULT_OK
pub const OPEN_RESULT_OK: i32 = 1;
// Java field: int OPEN_RESULT_FAIL
pub const OPEN_RESULT_FAIL: i32 = 2;
// Java field: int OPEN_RESULT_ENCRYPT
pub const OPEN_RESULT_ENCRYPT: i32 = 3;
// Java field: int OPEN_RESULT_FORCE_QUIT
pub const OPEN_RESULT_FORCE_QUIT: i32 = 4;
// Java field: int OPEN_RESULT_ONLINE_SECURITY
pub const OPEN_RESULT_ONLINE_SECURITY: i32 = 5;
// Java field: int OPEN_RESULT_FIRST_PAGE_FORCE_QUIT
pub const OPEN_RESULT_FIRST_PAGE_FORCE_QUIT: i32 = 6;
// Java field: int OPEN_RESULT_READING_LIMIT_QUIT
pub const OPEN_RESULT_READING_LIMIT_QUIT: i32 = 7;

use super::IDLProvider::IDLProvider;
use super::IDocProtection::IDocProtection;
use super::IKmoBookProtect::IKmoBookProtect;

// Java: public interface IDocument
pub trait IDocument {
    // Java: abstract int open(String var1, String var2, Object var3, IDLProvider var4, boolean var5)
    fn open(
        &mut self,
        var1: &str,
        var2: &str,
        var3: Option<&dyn std::any::Any>,
        var4: Option<&dyn IDLProvider>,
        var5: bool,
    ) -> i32;

    // Java: abstract int type()
    fn doc_type(&self) -> i32;

    // Java: abstract boolean isDirty()
    fn isDirty(&self) -> bool;

    // Java: abstract void setSlimDirty()
    fn setSlimDirty(&mut self);

    // Java: abstract void setWritePassword(String var1)
    fn setWritePassword(&mut self, var1: &str);

    // Java: abstract boolean hasWritePassword()
    fn hasWritePassword(&self) -> bool;

    // Java: abstract boolean save(String var1)
    fn save(&mut self, var1: &str) -> bool;

    // Java: abstract IDocProtection getDocProtection()
    fn getDocProtection(&self) -> Option<&dyn IDocProtection>;

    // Java: abstract IKmoBookProtect getBookProtection()
    fn getBookProtection(&self) -> Option<&dyn IKmoBookProtect>;

    // Java: abstract boolean isEmptySlides()
    fn isEmptySlides(&self) -> bool;

    // Java: abstract boolean isSecurityFile()
    fn isSecurityFile(&self) -> bool;
}
