// java package cn.wps.moss.app;

// java import cn.wps.core.runtime.IResourceManager;
// java import cn.wps.core.runtime.Platform;
// java import cn.wps.moss.util.KmoConstraintCheck;
// java import cn.wps.moss.util.TextHelper;
// java import java.util.ArrayList;
// java import java.util.List;
// java import java.util.Locale;
// java import java.util.regex.Matcher;
// java import java.util.regex.Pattern;
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

use crate::cn::wps::moss::util::KmoConstraintCheck::KmoConstraintCheck;

use super::KmoBook::KmoBook;
use super::KmoSheet::KmoSheet;

// java class KmoSheets
pub struct KmoSheets {
    // java private List<KmoSheet> sheetLst = new ArrayList();
    sheetLst: Vec<Rc<RefCell<KmoSheet>>>,
    // java private int latestNewSheetIndex = 1;
    latestNewSheetIndex: i32,
    // java private int lastestNewSheetId = -1;
    lastestNewSheetId: i32,
    // java private KmoSheet emptySheet;
    emptySheet: Option<Rc<RefCell<KmoSheet>>>,
}

fn full2Half(full: &str) -> String {
    String::from_utf16_lossy(
        &full
            .encode_utf16()
            .map(full_char_2_half)
            .collect::<Vec<_>>(),
    )
}

fn full_char_2_half(full_char: u16) -> u16 {
    if full_char >= 0xFF01 && full_char <= 0xFF5E {
        return full_char - 0xFEE0;
    }
    match full_char {
        0x3000 => b' ' as u16,
        0xFFE0 => 0x00A2,
        0xFFE1 => 0x00A3,
        0xFFE2 => 0x00AC,
        0xFFE3 => 0x00AF,
        0xFFE4 => 0x00A6,
        0xFFE5 => 0x00A5,
        0xFFE6 => 0x20A9,
        _ => {
            if (12593..=12644).contains(&full_char) {
                if full_char == 12644 {
                    return 0xFFA0;
                }
                if (12593..=12622).contains(&full_char) {
                    return full_char.wrapping_add(0xCE70);
                }
                if (12623..=12628).contains(&full_char) {
                    return full_char.wrapping_add(0xCE73);
                }
                if (12629..=12634).contains(&full_char) {
                    return full_char.wrapping_add(0xCE75);
                }
                if (12635..=12640).contains(&full_char) {
                    return full_char.wrapping_add(0xCE77);
                }
                if (12641..=12643).contains(&full_char) {
                    return full_char.wrapping_add(0xCE79);
                }
            }
            full_char
        }
    }
}

fn unreplace_arabic_digit(src: &str) -> String {
    let units: Vec<u16> = src.encode_utf16().collect();
    let has_arabic_digit = units.iter().any(|c| *c >= 0x0660 && *c <= 0x0669);
    if !has_arabic_digit {
        return src.to_string();
    }
    src.to_string()
}

fn format_sheet_name(template: &str, index: i32) -> String {
    if let Some(pos) = template.find("%d") {
        format!("{}{}{}", &template[..pos], index, &template[pos + 2..])
    } else {
        template.to_string()
    }
}

fn equals_ignore_case(left: &str, right: &str) -> bool {
    let lower = |s: &str| {
        s.encode_utf16()
            .map(|c| {
                if (b'A' as u16..=b'Z' as u16).contains(&c) {
                    c + 32
                } else {
                    c
                }
            })
            .collect::<Vec<_>>()
    };
    lower(left) == lower(right)
}

// java class KmoSheets
impl KmoSheets {
    // java protected static KmoSheet createSheet(byte sheetType, KmoBook book, String sheetName, int sheetId)
    pub fn createSheet(
        book: &KmoBook,
        sheet_type: i8,
        sheet_name: Option<String>,
        sheet_id: i32,
    ) -> Rc<RefCell<KmoSheet>> {
        // java switch (sheetType)
        match sheet_type {
            // java case 0:
            0 => {
                // java return new KmoSheet(book, sheetName, sheetId);
                KmoSheet::create(book, sheet_name, sheet_id)
            }
            // java case 2:
            2 => {
                // java return new KmoChartSheet(book, sheetName, sheetId);
                // KmoChartSheet 的构造只调用 KmoSheet 的构造。
                KmoSheet::create(book, sheet_name, sheet_id)
            }
            // java default:
            _ => {
                // java return new KmoSheet(book, sheetName, sheetId);
                KmoSheet::create(book, sheet_name, sheet_id)
            }
        }
    }

    // java KmoSheets(KmoBook parent)
    pub fn KmoSheets() -> Self {
        // java this.parent = parent;
        // rust: no-stmt — 表集合放在 KmoBook 里，调用时把 book 传进来。
        Self {
            sheetLst: Vec::new(),
            latestNewSheetIndex: 1,
            lastestNewSheetId: -1,
            emptySheet: None,
        }
    }

    // java public int size()
    pub fn size(&self) -> i32 {
        // java return this.sheetLst.size();
        self.sheetLst.len() as i32
    }

    // java public KmoSheet insertAt(int sheetId, int index, String name)
    pub fn insertAt(
        book: &KmoBook,
        sheet_id: i32,
        index: i32,
        name: Option<String>,
    ) -> Rc<RefCell<KmoSheet>> {
        // java return this.insertAt(sheetId, index, name, (byte)0);
        Self::insertAt__2(book, sheet_id, index, name, 0)
    }

    // java public KmoSheet insertAt(int sheetId, int index, String name, byte sheetType)
    pub fn insertAt__2(
        book: &KmoBook,
        sheet_id: i32,
        mut index: i32,
        name: Option<String>,
        sheet_type: i8,
    ) -> Rc<RefCell<KmoSheet>> {
        // java if (index > this.sheetLst.size() || index < 0)
        {
            let size = book.sheets.borrow().sheetLst.len() as i32;
            if index > size || index < 0 {
                // java index = this.sheetLst.size();
                index = size;
            }
        }
        // java KmoSheet sheet = createSheet(sheetType, this.parent, name, sheetId);
        let sheet = Self::createSheet(book, sheet_type, name, sheet_id);
        // java this.sheetLst.add(index, sheet);
        book.sheets
            .borrow_mut()
            .sheetLst
            .insert(index as usize, Rc::clone(&sheet));
        // java this.parent.transaction().reset();
        book.transaction().reset();
        // java return sheet;
        sheet
    }

    // java public KmoSheet appendSheet(KmoSheet sheet)
    pub fn appendSheet(book: &KmoBook, sheet: Rc<RefCell<KmoSheet>>) -> Rc<RefCell<KmoSheet>> {
        // java this.sheetLst.add(sheet);
        book.sheets.borrow_mut().sheetLst.push(Rc::clone(&sheet));
        // java this.parent.transaction().reset();
        book.transaction().reset();
        // java return sheet;
        sheet
    }

    // java public KmoSheet append(String name)
    pub fn append(book: &KmoBook, name: Option<String>) -> Rc<RefCell<KmoSheet>> {
        // java return this.insertAt(-1, -1, name);
        Self::insertAt(book, -1, -1, name)
    }

    // java public KmoSheet insertAt(int index)
    pub fn insertAt__3(book: &KmoBook, index: i32) -> Rc<RefCell<KmoSheet>> {
        // java return this.insertAt(-1, index, this.helper_generateNewSheetName());
        let name = book.sheets.borrow_mut().helper_generateNewSheetName();
        Self::insertAt(book, -1, index, Some(name))
    }

    // java public int sheetIndex(KmoSheet sheet)
    pub fn sheetIndex(book: &KmoBook, sheet: &KmoSheet) -> i32 {
        // java return this.sheetLst.indexOf(sheet);
        book.sheets
            .borrow()
            .sheetLst
            .iter()
            .position(|item| std::ptr::eq(item.as_ref().as_ptr(), sheet as *const KmoSheet))
            .map(|i| i as i32)
            .unwrap_or(-1)
    }

    // java public void moveSheet(int srcSheetIndex, int newIndex)
    pub fn moveSheet(book: &KmoBook, src_sheet_index: i32, new_index: i32) {
        // java KmoSheet srcSheet = (KmoSheet)this.sheetLst.get(srcSheetIndex);
        let src_sheet = {
            let sheets = book.sheets.borrow();
            let len = sheets.sheetLst.len() as i32;
            if src_sheet_index < 0 || src_sheet_index >= len || new_index < 0 || new_index >= len {
                panic!("IndexOutOfBoundsException");
            }
            Rc::clone(&sheets.sheetLst[src_sheet_index as usize])
        };
        // java this.sheetLst.remove(srcSheetIndex);
        book.sheets
            .borrow_mut()
            .sheetLst
            .remove(src_sheet_index as usize);
        // java this.sheetLst.add(newIndex, srcSheet);
        book.sheets
            .borrow_mut()
            .sheetLst
            .insert(new_index as usize, src_sheet);
        // java this.parent.transaction().reset();
        book.transaction().reset();
    }

    // java public int sheetIndex(String sheet)
    pub fn sheetIndex__2(book: &KmoBook, sheet: &str) -> i32 {
        // java String convertedSheetName = TextHelper.full2Half(sheet);
        let mut converted_sheet_name = full2Half(sheet);
        // java convertedSheetName = TextHelper.unReplaceArabicDigit(convertedSheetName);
        converted_sheet_name = unreplace_arabic_digit(&converted_sheet_name);
        // java int i = 0;
        let mut i = 0;
        // java for(int size = this.sheetLst.size(); i < size; ++i)
        let size = book.sheets.borrow().size();
        while i < size {
            // java String sheetName = TextHelper.full2Half(((KmoSheet)this.sheetLst.get(i)).name());
            let sheet_name = full2Half(&book.sheets.borrow().sheetLst[i as usize].borrow().name());
            // java if (sheetName.equalsIgnoreCase(convertedSheetName))
            if equals_ignore_case(&sheet_name, &converted_sheet_name) {
                // java return i;
                return i;
            }
            i += 1;
        }
        // java return -1;
        -1
    }

    // java public KmoSheet append()
    pub fn append__2(book: &KmoBook) -> Rc<RefCell<KmoSheet>> {
        // java return this.append(this.helper_generateNewSheetName());
        let name = book.sheets.borrow_mut().helper_generateNewSheetName();
        Self::append(book, Some(name))
    }

    // java public void remove(int index)
    pub fn remove(book: &KmoBook, index: i32) {
        // java KmoConstraintCheck.checkArgument(this.helper_isSheetIndexValid(index));
        KmoConstraintCheck::checkArgument(book.sheets.borrow().helper_isSheetIndexValid(index));
        // java if (this.sheetLst.size() > 1)
        if book.sheets.borrow().sheetLst.len() > 1 {
            // java this.sheetLst.remove(index);
            book.sheets.borrow_mut().sheetLst.remove(index as usize);
        }
        // java this.parent.transaction().reset();
        book.transaction().reset();
    }

    // java public void remove(KmoSheet sheet)
    pub fn remove__2(book: &KmoBook, sheet: &KmoSheet) {
        // java this.sheetLst.remove(sheet);
        book.sheets
            .borrow_mut()
            .sheetLst
            .retain(|item| !item.borrow().same_sheet(sheet));
        // java this.parent.transaction().reset();
        book.transaction().reset();
    }

    // java public void activate(int index)
    pub fn activate(book: &KmoBook, index: i32) {
        // java KmoConstraintCheck.checkArgument(this.helper_isSheetIndexValid(index));
        KmoConstraintCheck::checkArgument(book.sheets.borrow().helper_isSheetIndexValid(index));
        // java int i = 0;
        let mut i = 0i32;
        // java for(int size = this.sheetLst.size(); i < size; ++i)
        let size = book.sheets.borrow().size();
        while i < size {
            let sheet = Rc::clone(&book.sheets.borrow().sheetLst[i as usize]);
            // java if (i == index)
            if i == index {
                // java ((KmoSheet)this.sheetLst.get(i)).getWindow2().setSelected(true);
                sheet.borrow().getWindow2().setSelected(true);
            // java } else
            } else {
                // java ((KmoSheet)this.sheetLst.get(i)).getWindow2().setSelected(false);
                sheet.borrow().getWindow2().setSelected(false);
            }
            i += 1;
        }
    }

    // java public KmoSheet sheetAt(int index)
    pub fn sheetAt(book: &KmoBook, index: i32) -> Rc<RefCell<KmoSheet>> {
        // java if (!this.helper_isSheetIndexValid(index))
        if !book.sheets.borrow().helper_isSheetIndexValid(index) {
            // java if (this.emptySheet == null)
            if book.sheets.borrow().emptySheet.is_none() {
                // java this.emptySheet = new KmoSheet(this.parent, "", -1);
                let empty = KmoSheet::create(book, Some(String::new()), -1);
                book.sheets.borrow_mut().emptySheet = Some(Rc::clone(&empty));
                // java return this.emptySheet;
                return empty;
            }
            // java return this.emptySheet;
            return Rc::clone(book.sheets.borrow().emptySheet.as_ref().unwrap());
        // java } else
        } else {
            // java return (KmoSheet)this.sheetLst.get(index);
            Rc::clone(&book.sheets.borrow().sheetLst[index as usize])
        }
    }

    // java private boolean helper_isSheetIndexValid(int index)
    pub fn helper_isSheetIndexValid(&self, index: i32) -> bool {
        // java return index >= 0 && index < this.sheetLst.size();
        index >= 0 && index < self.sheetLst.len() as i32
    }

    // java public boolean isDefaultSheetName(String sheetName)
    pub fn isDefaultSheetName(&self, sheet_name: &str) -> bool {
        // java String name_template = this.getNameTemplate();
        let mut name_template = self.getNameTemplate();
        // java name_template = name_template.replaceAll("%d", "[1-9][\\\\d]*\\$");
        name_template = name_template.replace("%d", "[1-9][\\d]*$");
        // java Pattern r = Pattern.compile("^" + name_template);
        let pattern = regex::Regex::new(&format!("^{name_template}"))
            .unwrap_or_else(|_| panic!("PatternSyntaxException"));
        // java Matcher m = r.matcher(sheetName);
        // java return m.find();
        pattern.is_match(sheet_name)
    }

    // java private String getNameTemplate()
    fn getNameTemplate(&self) -> String {
        // java String name_template = "Sheet%d";
        let mut name_template = "Sheet%d".to_string();
        // java try {
        // java IResourceManager rm = Platform.getResourceManager();
        // java name_template = rm.getString("et_sheetname_template");
        // rust: no-stmt — Platform 不在这一档，资源管理器取不到模板，进入 catch。
        // java } catch (RuntimeException var3) {
        // java name_template = "Sheet%d";
        name_template = "Sheet%d".to_string();
        // java if (name_template == null || name_template.indexOf("%d") < 0)
        if !name_template.contains("%d") {
            // java name_template = "Sheet%d";
            name_template = "Sheet%d".to_string();
        }
        // java return name_template;
        name_template
    }

    // java private String helper_generateNewSheetName()
    pub fn helper_generateNewSheetName(&mut self) -> String {
        // java String name_template = this.getNameTemplate();
        let name_template = self.getNameTemplate();
        // java do {
        loop {
            // java name = String.format(Locale.US, name_template, this.latestNewSheetIndex++);
            let index = self.latestNewSheetIndex;
            self.latestNewSheetIndex = self.latestNewSheetIndex.wrapping_add(1);
            let name = format_sheet_name(&name_template, index);
            // java } while(!this.helper_isSheetNameUnique(name));
            if self.helper_isSheetNameUnique(Some(&name)) {
                // java return name;
                return name;
            }
        }
    }

    // java public int helper_generateNewSheetId()
    pub fn helper_generateNewSheetId(&mut self) -> i32 {
        // java int i = 0;
        let mut i = 0i32;
        // java for(int size = this.sheetLst.size(); i < size; ++i)
        let size = self.sheetLst.len() as i32;
        while i < size {
            // java int curSheetId = this.sheetAt(i).getSheetId();
            let cur_sheet_id = self.sheetLst[i as usize].borrow().getSheetId();
            // java if (curSheetId > this.lastestNewSheetId)
            if cur_sheet_id > self.lastestNewSheetId {
                // java this.lastestNewSheetId = curSheetId;
                self.lastestNewSheetId = cur_sheet_id;
            }
            i += 1;
        }
        // java return ++this.lastestNewSheetId;
        self.lastestNewSheetId = self.lastestNewSheetId.wrapping_add(1);
        self.lastestNewSheetId
    }

    // java private boolean helper_isSheetNameUnique(String name)
    fn helper_isSheetNameUnique(&self, name: Option<&str>) -> bool {
        // java if (name != null && name.length() != 0)
        if let Some(name) = name {
            if !name.is_empty() {
                // java name = TextHelper.full2Half(name.toLowerCase());
                let name = full2Half(&name.to_lowercase());
                // java int i = 0;
                let mut i = 0i32;
                // java for(int size = this.size(); i < size; ++i)
                let size = self.size();
                while i < size {
                    // java String sn = this.sheetAt(i).name();
                    let sn = self.sheetLst[i as usize].borrow().name();
                    // java sn = sn == null ? null : TextHelper.full2Half(sn.toLowerCase());
                    let sn = full2Half(&sn.to_lowercase());
                    // java if (name.equals(sn))
                    if name == sn {
                        // java return false;
                        return false;
                    }
                    i += 1;
                }
                // java return true;
                return true;
            }
        }
        // java return true;
        true
    }

    // java public void destroy()
    pub fn destroy(&mut self) {
        // java for(KmoSheet sheet : this.sheetLst)
        for sheet in &self.sheetLst {
            // java sheet.destroy();
            sheet.borrow().destroy();
        }
        // java this.sheetLst.clear();
        self.sheetLst.clear();
    }
}
