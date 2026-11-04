use std::sync::LazyLock;

use util::org::apache::poi::util::BitField::BitField;
use util::org::apache::poi::util::BitFieldFactory::BitFieldFactory;

use crate::org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants;

use super::FormulaDataStore::FormulaDataStore;
use super::FormulaStringStore::FormulaStringStore;
use super::FormulaTokensStore::FormulaTokensStore;
use super::KmoFormulaData::KmoFormulaData;
use super::LocalFmlBytes::LocalFmlBytes;
use super::MemoryMixSQLDoubleList::MemoryMixSQLDoubleList;
use super::MemoryMixSQLStringList::MemoryMixSQLStringList;
use super::MemoryMixSQLStringList::OoxmlStr;

// java private static final BitField VALUE_TYPE_MASK = BitFieldFactory.getInstance(-536870912);
static VALUE_TYPE_MASK: LazyLock<BitField> =
    LazyLock::new(|| BitFieldFactory::getInstance(-536870912));

fn value_type_mask() -> &'static BitField {
    &VALUE_TYPE_MASK
}

// java public static final int ID_BLANK = 0;
pub const ID_BLANK: i32 = 0;

// java ID_TRUE = VALUE_TYPE_MASK.setValue(1, 5);
static ID_TRUE: LazyLock<i32> = LazyLock::new(|| value_type_mask().setValue(1, 5));
// java ID_FALSE = VALUE_TYPE_MASK.setValue(0, 5);
static ID_FALSE: LazyLock<i32> = LazyLock::new(|| value_type_mask().setValue(0, 5));
// java mLocalFmlBytes = new LocalFmlBytes();
static mLocalFmlBytes: LazyLock<LocalFmlBytes> =
    LazyLock::new(|| LocalFmlBytes::LocalFmlBytes());

// java public final class KmoCellValueStore
pub struct KmoCellValueStore {
    // java private MemoryMixSQLDoubleList dVals = new MemoryMixSQLDoubleList();
    dVals: MemoryMixSQLDoubleList,
    // java private MemoryMixSQLStringList sVals = new MemoryMixSQLStringList();
    sVals: MemoryMixSQLStringList,
    // java private FormulaDataStore formulas = new FormulaDataStore();
    formulas: FormulaDataStore,
    // java private FormulaTokensStore fTokens = new FormulaTokensStore();
    fTokens: FormulaTokensStore,
    // java private FormulaStringStore fStrs = new FormulaStringStore();
    fStrs: FormulaStringStore,
}

// java public final class KmoCellValueStore
impl KmoCellValueStore {
    pub fn KmoCellValueStore() -> Self {
        let _ = *ID_TRUE;
        let _ = *ID_FALSE;
        let _ = &*mLocalFmlBytes;
        KmoCellValueStore {
            dVals: MemoryMixSQLDoubleList::MemoryMixSQLDoubleList(),
            sVals: MemoryMixSQLStringList::MemoryMixSQLStringList(),
            formulas: FormulaDataStore::FormulaDataStore(),
            fTokens: FormulaTokensStore::FormulaTokensStore(),
            fStrs: FormulaStringStore::FormulaStringStore(),
        }
    }

    // java public void addStrsByOOXML(Iterator<UnicodeString> strs, int startPos, int totalSize, boolean bReadAllStrings)
    pub fn addStrsByOOXML(
        &mut self,
        strs: &[OoxmlStr],
        startPos: i32,
        totalSize: i32,
        bReadAllStrings: bool,
    ) {
        // java this.sVals.addStrsByOOXML(strs, startPos, totalSize, bReadAllStrings);
        self.sVals
            .addStrsByOOXML(strs, startPos, totalSize, bReadAllStrings);
    }

    // java public void initStrStore(Iterator<UnicodeString> strs, int size)
    pub fn initStrStore(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sVals.initStrs(strs, size);
        */
    }

    // java public int addUnicodeString(UnicodeString str)
    pub fn addUnicodeString(&mut self, str: Option<crate::java_runtime::Rc<OoxmlStr>>) -> i32 {
        // java return this.sVals.addUnicodeString(str);
        self.sVals.addUnicodeString(str)
    }

    // java public static int identifyTypeByID(int id)
    pub fn identifyTypeByID(id: i32) -> i32 {
        // java int t = VALUE_TYPE_MASK.getValue(id);
        let t = value_type_mask().getValue(id);
        // java return t != 3 && t != 4 ? t : 2;
        if t != 3 && t != 4 { t } else { 2 }
    }

    // java public static boolean isNumericId(int id)
    pub fn isNumericId(id: i32) -> bool {
        // java return 1 == VALUE_TYPE_MASK.getValue(id);
        1 == value_type_mask().getValue(id)
    }

    // java public static boolean isFormulaId(int id)
    pub fn isFormulaId(id: i32) -> bool {
        // java return 7 == VALUE_TYPE_MASK.getValue(id);
        7 == value_type_mask().getValue(id)
    }

    // java public static boolean isStringId(int id)
    pub fn isStringId(id: i32) -> bool {
        // java int t = VALUE_TYPE_MASK.getValue(id);
        let t = value_type_mask().getValue(id);
        // java return 2 == t || 3 == t || 4 == t;
        2 == t || 3 == t || 4 == t
    }

    // java public static boolean isRunsString(int id)
    pub fn isRunsString(id: i32) -> bool {
        // java return stringTypeHasRun(id);
        Self::stringTypeHasRun(id)
    }

    // java public static boolean isBlankId(int id)
    pub fn isBlankId(id: i32) -> bool {
        // java return 0 == id;
        0 == id
    }

    // java public static boolean isBoolId(int id)
    pub fn isBoolId(id: i32) -> bool {
        // java return 5 == VALUE_TYPE_MASK.getValue(id);
        5 == value_type_mask().getValue(id)
    }

    // java public static boolean isErrId(int id)
    pub fn isErrId(id: i32) -> bool {
        // java return 6 == VALUE_TYPE_MASK.getValue(id);
        6 == value_type_mask().getValue(id)
    }

    // java public static boolean stringTypeHasRun(int id)
    pub fn stringTypeHasRun(id: i32) -> bool {
        // java return 3 == VALUE_TYPE_MASK.getValue(id);
        3 == value_type_mask().getValue(id)
    }

    // java public static int id2sstid(int id)
    pub fn id2sstid(id: i32) -> i32 {
        // java return VALUE_TYPE_MASK.clear(id);
        value_type_mask().clear(id)
    }

    // java public boolean isRunsString2(int id)
    pub fn isRunsString2(&self, mut id: i32) -> bool {
        // java int t = VALUE_TYPE_MASK.getValue(id);
        let t = value_type_mask().getValue(id);
        // java boolean hasRun = 3 == t;
        let mut hasRun = 3 == t;
        // java if (!hasRun && 2 == t)
        if !hasRun && 2 == t {
            // java id = VALUE_TYPE_MASK.clear(id);
            id = value_type_mask().clear(id);
            // java hasRun = this.sVals.isRunsString(id);
            hasRun = self.sVals.isRunsString(id);
        }
        // java return hasRun;
        hasRun
    }

    // java public double doubleValue(int id)
    pub fn doubleValue(&mut self, id: i32) -> f64 {
        // java int offset = VALUE_TYPE_MASK.clear(id);
        let offset = value_type_mask().clear(id);
        // java return this.dVals.getDouble(offset);
        self.dVals.getDouble(offset)
    }

    // java public byte errorValue(int id)
    pub fn errorValue(&self, id: i32) -> i8 {
        // java int errorCode = VALUE_TYPE_MASK.clear(id);
        let errorCode = value_type_mask().clear(id);
        // java return (byte)errorCode;
        errorCode as i8
    }

    // java public boolean booleanValue(int id)
    pub fn booleanValue(&self, id: i32) -> bool {
        // java return id == ID_TRUE;
        id == *ID_TRUE
    }

    // java public String stringValue(int id)
    pub fn stringValue(&mut self, id: i32) -> String {
        // java int type = VALUE_TYPE_MASK.getValue(id);
        let r#type = value_type_mask().getValue(id);
        // java if (2 != type && 3 != type)
        if 2 != r#type && 3 != r#type {
            // java if (4 == type)
            if 4 == r#type {
                // java return this.fStrs.get(VALUE_TYPE_MASK.clear(id));
                self.fStrs.get(value_type_mask().clear(id))
            } else {
                // java throw new IllegalArgumentException("获取字符串ID出错，出错ID：" + id);
                panic!("IllegalArgumentException: 获取字符串ID出错，出错ID：{id}")
            }
        } else {
            // java return this.sVals.getString(VALUE_TYPE_MASK.clear(id));
            self.sVals.getString(value_type_mask().clear(id))
        }
    }

    // java public UnicodeString runsStringValue(int id)
    pub fn runsStringValue(&mut self,id:i32)->Option<crate::java_runtime::Rc<super::MemoryMixSQLStringList::OoxmlStr>>{
        // java return this.sVals.getRunsString(VALUE_TYPE_MASK.clear(id));
        self.sVals.getRunsString(value_type_mask().clear(id))
    }

    // java public void formulaValue(int id, KmoFormulaData formula)
    pub fn formulaValue(&self, id: i32, formula: &mut KmoFormulaData) {
        // java int offset = VALUE_TYPE_MASK.clear(id);
        let offset = value_type_mask().clear(id);
        // java int position = this.formulas.get(offset);
        let position = self.formulas.get__int(offset);
        // java if (position != -1)
        if position != -1 {
            // java formula.read(this.formulas.data(), position);
            formula.read(self.formulas.data(), position);
        }
    }

    // java public int formulaCachaId(int id)
    pub fn formulaCachaId(&self, id: i32) -> i32 {
        // java int offset = VALUE_TYPE_MASK.clear(id);
        let mut offset = value_type_mask().clear(id);
        // java byte[] fmlBytes = (byte[])mLocalFmlBytes.get();
        // java this.formulas.get(offset, fmlBytes);
        mLocalFmlBytes.with_bytes(|fmlBytes| {
            self.formulas.get(offset, fmlBytes);
            // java offset = 0;
            offset = 0;
            // java offset += 4;
            offset += 4;
            // java offset += 2;
            offset += 2;
            // java return LittleEndian.getInt(fmlBytes, offset);
            util::cn::wps::moffice::util::LittleEndian::LittleEndian::getInt(
                fmlBytes,
                offset as usize,
            )
        })
    }

    // java public byte[] formulaTokens(int id)
    pub fn formulaTokens(&self, id: i32) -> Vec<u8> {
        // java return this.fTokens.get(id);
        self.fTokens.get(id)
    }

    // java public int newDoubleValue(double value)
    pub fn newDoubleValue(&mut self, value: f64) -> i32 {
        // java return VALUE_TYPE_MASK.setValue(this.dVals.addDouble(value), 1);
        value_type_mask().setValue(self.dVals.addDouble(value), 1)
    }

    // java public int newStringValue(String value)
    pub fn newStringValue(&mut self, value: Option<&str>) -> i32 {
        // java return VALUE_TYPE_MASK.setValue(this.sVals.addString(value), 2);
        value_type_mask().setValue(self.sVals.addString(value), 2)
    }

    // java public int newRunStringValue(UnicodeString unicodeString)
    pub fn newRunStringValue(&mut self, unicodeString: Option<crate::java_runtime::Rc<OoxmlStr>>) -> i32 {
        // java return VALUE_TYPE_MASK.setValue(this.addUnicodeString(unicodeString), 3);
        let index = self.addUnicodeString(unicodeString);
        value_type_mask().setValue(index, 3)
    }

    // java public int newFormulaValue(KmoFormulaData formula)
    pub fn newFormulaValue(&mut self, formula: &KmoFormulaData) -> i32 {
        // java return VALUE_TYPE_MASK.setValue(this.formulas.add(formula.write()), 7);
        let bytes = formula.write__1();
        value_type_mask().setValue(self.formulas.add(Some(&bytes)), 7)
    }

    // java public int newFormulaTokens(byte[] tokens)
    pub fn newFormulaTokens(&mut self, tokens: Option<&[u8]>) -> i32 {
        // java return this.fTokens.add(tokens);
        self.fTokens.add(tokens)
    }

    // java public int newFmlString(String value)
    pub fn newFmlString(&mut self, value: &str) -> i32 {
        // java return VALUE_TYPE_MASK.setValue(this.fStrs.add(value), 4);
        value_type_mask().setValue(self.fStrs.add(value), 4)
    }

    // java public int newBooleanValue(boolean value)
    pub fn newBooleanValue(&self, value: bool) -> i32 {
        // java return value ? ID_TRUE : ID_FALSE;
        if value { *ID_TRUE } else { *ID_FALSE }
    }

    // java public int newErrorValue(int errorCode)
    pub fn newErrorValue(&self, errorCode: i32) -> i32 {
        // java if (!ErrorConstants.isValidCode(errorCode))
        if !ErrorConstants::isValidCode(errorCode) {
            // java throw new IllegalStateException("No such error code");
            panic!("IllegalStateException: No such error code");
        } else {
            // java return VALUE_TYPE_MASK.setValue(errorCode, 6);
            value_type_mask().setValue(errorCode, 6)
        }
    }

    // java public int getEngineStrStoreId(int id)
    pub fn getEngineStrStoreId(&self, id: i32) -> i32 {
        // java if (!this.sVals.isValidId(id))
        if !self.sVals.isValidId(id) {
            // java throw new IllegalArgumentException("");
            panic!("IllegalArgumentException:");
        } else {
            // java return this.sVals.isRunsString(id) ? VALUE_TYPE_MASK.setValue(id, 3) : VALUE_TYPE_MASK.setValue(id, 2);
            if self.sVals.isRunsString(id) {
                value_type_mask().setValue(id, 3)
            } else {
                value_type_mask().setValue(id, 2)
            }
        }
    }

    // java public boolean isValidId(int id)
    pub fn isValidId(&self, id: i32) -> bool {
        // java return this.sVals.isValidId(id);
        self.sVals.isValidId(id)
    }

    // java public String getStringBySSTId(int sstId)
    pub fn getStringBySSTId(&mut self, sstId: i32) -> String {
        // java return this.sVals.getString(sstId);
        self.sVals.getString(sstId)
    }

    // java public Iterator<UnicodeString> getStrStoreIterator()
    pub fn getStrStoreIterator(&mut self)->super::MemoryMixSQLStringList::StrStoreIterator<'_> {
        // java return this.sVals.getStrStoreIterator();
        self.sVals.getStrStoreIterator()
    }

    // java public int getUniqueCount()
    pub fn getUniqueCount(&self) -> i32 {
        // java return this.sVals.allStrs.size() + this.sVals.existedStringCount;
        self.sVals.allStrs.len() as i32 + self.sVals.existedStringCount
    }

    // java public void setFormulaCalcFlag(int fmlId, byte calcFlag)
    pub fn setFormulaCalcFlag(&mut self, fmlId: i32, calcFlag: i8) {
        // java this.formulas.setCalcFlag(VALUE_TYPE_MASK.clear(fmlId), calcFlag);
        self.formulas
            .setCalcFlag(value_type_mask().clear(fmlId), calcFlag);
    }

    // java public byte getFormulaCalcFlag(int fmlId)
    pub fn getFormulaCalcFlag(&self, fmlId: i32) -> i8 {
        // java return this.formulas.getCalcFlag(VALUE_TYPE_MASK.clear(fmlId));
        self.formulas
            .getCalcFlag(value_type_mask().clear(fmlId))
    }

    // java public void releaseStoreData()
    pub fn releaseStoreData(&mut self) {
        // java this.dVals.close();
        self.dVals.close();
        // java this.sVals.close();
        self.sVals.close();
    }
}
