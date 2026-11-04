use crate::cn::wps::moss::app::persistence::KmoDataPersistence::ByteArrayPersistence;
use util::cn::wps::kfc::r#impl::map::WIntObjectHashMap::WIntObjectHashMap;

pub use crate::cn::wps::moss::filefmt::biff8::record::common::UnicodeString::UnicodeString as OoxmlStr;

// java class MemoryMixSQLStringList
pub struct MemoryMixSQLStringList {
    // java private final int MAX_MEMORY_STRING_COUNT = 25000;
    MAX_MEMORY_STRING_COUNT: i32,
    // java int existedStringCount = 0;
    pub(crate) existedStringCount: i32,
    // java List<String> allStrs = new ArrayList();
    pub(crate) allStrs: Vec<Option<String>>,
    // java private List<byte[]> nonParsedStrs;
    nonParsedStrs: Option<Vec<Option<Vec<u8>>>>,
    // java private int nonParsedStrCount = 0;
    nonParsedStrCount: i32,
    // java private WIntObjectHashMap<UnicodeString> runsStrs = new WIntObjectHashMap();
    runsStrs: WIntObjectHashMap,
    // java private boolean bReleaseNonParsedStrs;
    bReleaseNonParsedStrs: bool,
    // java private KmoDataPersistence.ByteArrayPersistence kmoPersistence = new KmoDataPersistence.ByteArrayPersistence();
    kmoPersistence: ByteArrayPersistence,
}

// java class MemoryMixSQLStringList
impl MemoryMixSQLStringList {
    // java public MemoryMixSQLStringList()
    pub fn MemoryMixSQLStringList() -> Self {
        MemoryMixSQLStringList {
            MAX_MEMORY_STRING_COUNT: 25000,
            existedStringCount: 0,
            allStrs: Vec::new(),
            nonParsedStrs: None,
            nonParsedStrCount: 0,
            runsStrs: WIntObjectHashMap::new(),
            bReleaseNonParsedStrs: false,
            kmoPersistence: ByteArrayPersistence::ByteArrayPersistence(),
        }
    }

    // java public void close()
    pub fn close(&mut self) {
        // java this.kmoPersistence.closeChannel();
        self.kmoPersistence.closeChannel();
    }

    // java private boolean isStrParsed(int id)
    fn isStrParsed(&self, id: i32) -> bool {
        // java if (id >= this.nonParsedStrs.size())
        let non = self
            .nonParsedStrs
            .as_ref()
            .unwrap_or_else(|| panic!("NullPointerException"));
        if id >= non.len() as i32 {
            // java return true;
            true
        } else {
            // java return this.nonParsedStrs.get(id) == null;
            non[id as usize].is_none()
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
        /* DEFERRED_NON_FUNCTION
        // java this.bReleaseNonParsedStrs = bReadAllStrings;
        // java if (this.allStrs.size() == 0)
        // java this.allStrs = new ArrayList(totalSize);
        // java this.nonParsedStrs = totalSize == 0 ? null : new ArrayList(totalSize);
        // java for(int i = 0; i < totalSize; ++i)
        // java this.allStrs.add(null);
        // java this.nonParsedStrs.add(null);
        // java for(int i = 0; strs.hasNext(); ++i)
        // java UnicodeString str = (UnicodeString)strs.next();
        // java if (null != str && null != this.nonParsedStrs)
        // java if (str.getFormatRunCount() > 0)
        // java this.runsStrs.put(i + startPos, str);
        // java else if (!str.isStrParsed())
        // java this.nonParsedStrs.set(i + startPos, str.getNonParsedBytes());
        // java ++this.nonParsedStrCount;
        // java this.allStrs.set(i + startPos, str.getString());
        */
        self.bReleaseNonParsedStrs = bReadAllStrings;
        if self.allStrs.is_empty() {
            self.allStrs = Vec::with_capacity(totalSize.max(0) as usize);
            self.nonParsedStrs = if totalSize == 0 {
                None
            } else {
                Some(Vec::with_capacity(totalSize as usize))
            };
            let mut i = 0;
            while i < totalSize {
                self.allStrs.push(None);
                self.nonParsedStrs.as_mut().unwrap().push(None);
                i += 1;
            }
        }
        for (i, str) in strs.iter().enumerate() {
            if self.nonParsedStrs.is_some() {
                let index = i as i32 + startPos;
                if str.format_run_count > 0 {
                    self.runsStrs.put(index,Some(crate::java_runtime::Rc::new(str.clone())));
                } else if !str.parsed {
                    self.nonParsedStrs.as_mut().unwrap()[index as usize] = None;
                    self.nonParsedStrCount += 1;
                } else {
                    self.allStrs[index as usize] = Some(str.text.clone());
                }
            }
        }
    }

    // java public void initStrs(Iterator<UnicodeString> strs, int size)
    pub fn initStrs(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.allStrs = new ArrayList(size);
        // java this.nonParsedStrs = size == 0 ? null : new ArrayList(size);
        // java for(int i = 0; i < size; ++i)
        // java UnicodeString str = (UnicodeString)strs.next();
        // java if (str.getFormatRunCount() > 0)
        // java if (!str.isStrParsed())
        // java str.parsedBytesToStr();
        // java this.runsStrs.put(i, str);
        // java this.nonParsedStrs.add(null);
        // java this.allStrs.add(null);
        // java else if (!str.isStrParsed())
        // java this.nonParsedStrs.add(str.getNonParsedBytes());
        // java ++this.nonParsedStrCount;
        // java this.allStrs.add(null);
        // java this.nonParsedStrs.add(null);
        // java this.allStrs.add(str.getString());
        */
    }

    // java public boolean isValidId(int id)
    pub fn isValidId(&self, id: i32) -> bool {
        // java return id < this.allStrs.size() + this.existedStringCount;
        id < self.allStrs.len() as i32 + self.existedStringCount
    }

    // java private boolean pushDataToSQL()
    fn pushDataToSQL(&mut self) -> bool {
        // java int count = this.allStrs.size();
        let count = self.allStrs.len() as i32;
        // java List<byte[]> bytesList = new ArrayList(count);
        let mut bytesList = Vec::with_capacity(count as usize);
        // java this.prepareToPersitsence(bytesList);
        self.prepareToPersitsence(&mut bytesList);
        // java boolean isSuc = this.kmoPersistence.insertData(bytesList);
        let isSuc = self.kmoPersistence.insertData(&bytesList);
        // java if (!isSuc)
        if !isSuc {
            // java this.reverse(bytesList);
            self.reverse(&mut bytesList);
            // java return false;
            false
        } else {
            // java this.existedStringCount += count;
            self.existedStringCount += count;
            // java this.allStrs.clear();
            self.allStrs.clear();
            // java bytesList.clear();
            bytesList.clear();
            // java return true;
            true
        }
    }

    // java private void reverse(List<byte[]> bytesList)
    fn reverse(&mut self, bytesList: &mut Vec<Vec<u8>>) {
        // java int count = bytesList.size();
        let count = bytesList.len() as i32;
        // java for(int i = 0; i < count; ++i)
        for _i in 0..count {
            // java this.allStrs.add(null);
            self.allStrs.push(None);
        }
        // java for(int i = count - 1; i >= 0; --i)
        for i in (0..count).rev() {
            // java byte[] bytes = (byte[])bytesList.remove(i);
            let bytes = bytesList.remove(i as usize);
            // java this.allStrs.set(i, new String(bytes, StandardCharsets.UTF_8));
            self.allStrs[i as usize] = Some(String::from_utf8_lossy(&bytes).into_owned());
        }
    }

    // java private void prepareToPersitsence(List<byte[]> bytesList)
    fn prepareToPersitsence(&mut self, bytesList: &mut Vec<Vec<u8>>) {
        // java int count = this.allStrs.size();
        let count = self.allStrs.len() as i32;
        // java for(int i = 0; i < count; ++i)
        for _i in 0..count {
            // java bytesList.add(null);
            bytesList.push(Vec::new());
        }
        // java for(int i = count - 1; i >= 0; --i)
        for i in (0..count).rev() {
            // java String s = (String)this.allStrs.get(i);
            let s = self.allStrs[i as usize].clone();
            // java if (s == null)
            let s = if s.is_none() {
                // java s = this.getSValsStr(i);
                self.getSValsStr(i)
            } else {
                // java this.allStrs.remove(i);
                self.allStrs.remove(i as usize);
                s.unwrap()
            };
            // java bytesList.set(i, s.getBytes(StandardCharsets.UTF_8));
            bytesList[i as usize] = s.into_bytes();
        }
        // java this.allStrs.clear();
        self.allStrs.clear();
    }

    // java public int addString(String value)
    pub fn addString(&mut self, value: Option<&str>) -> i32 {
        // java int index = this.allStrs.indexOf(value);
        let mut index = index_of(&self.allStrs, value);
        // java if (-1 != index && value != null)
        if index != -1 && value.is_some() {
            // java return index + this.existedStringCount;
            index + self.existedStringCount
        } else {
            // java this.allStrs.add(value);
            self.allStrs.push(value.map(|s| s.to_string()));
            // java index = this.allStrs.size();
            index = self.allStrs.len() as i32;
            // java return index >= 25000 && this.bReleaseNonParsedStrs && this.pushDataToSQL() ? -1 + this.existedStringCount : index - 1 + this.existedStringCount;
            if index >= 25000 && self.bReleaseNonParsedStrs && self.pushDataToSQL() {
                -1 + self.existedStringCount
            } else {
                index - 1 + self.existedStringCount
            }
        }
    }

    // java public int addUnicodeString(UnicodeString str)
    pub fn addUnicodeString(&mut self, str: Option<crate::java_runtime::Rc<OoxmlStr>>) -> i32 {
        // java if (this.nonParsedStrs == null)
        // java this.nonParsedStrs = new ArrayList();
        if self.nonParsedStrs.is_none() { self.nonParsedStrs = Some(Vec::new()); }
        let str = str.expect("NullPointerException UnicodeString");
        // java if (str.getFormatRunCount() > 0)
        if str.getFormatRunCount() > 0 {
        // java if (!str.isStrParsed())
        // java str.parsedBytesToStr();
            // rust: XLSX's UnicodeString is already parsed; BIFF lazy byte decoding remains outside this closure.
            assert!(str.parsed, "UnicodeString.parsedBytesToStr outside XLSX parsed-string closure");
        // java this.runsStrs.put(this.allStrs.size(), str);
            self.runsStrs.put(self.allStrs.len() as i32, Some(str));
        // java this.nonParsedStrs.add(null);
            self.nonParsedStrs.as_mut().unwrap().push(None);
        // java this.allStrs.add(null);
            self.allStrs.push(None);
        } else {
        // java else if (!str.isStrParsed())
        // java this.nonParsedStrs.add(str.getNonParsedBytes());
        // java ++this.nonParsedStrCount;
        // java this.allStrs.add(null);
            assert!(str.parsed, "UnicodeString.getNonParsedBytes outside XLSX parsed-string closure");
        }
        // java return this.allStrs.size() - 1 + this.existedStringCount;
        // Java deliberately does not append a parsed string without runs.
        (self.allStrs.len() as i32).wrapping_sub(1).wrapping_add(self.existedStringCount)
    }

    // java private String getSValsStr(int id)
    fn getSValsStr(&mut self, id: i32) -> String {
        // java if (this.nonParsedStrs != null)
        if self.nonParsedStrs.is_some() {
            // java int size = this.nonParsedStrs.size();
            let size = self.nonParsedStrs.as_ref().unwrap().len() as i32;
            // java if (id < size)
            if id < size {
                // java byte[] tmpByte = (byte[])this.nonParsedStrs.get(id);
                let tmpByte = self.nonParsedStrs.as_ref().unwrap()[id as usize].clone();
                // java if (tmpByte != null)
                if tmpByte.is_some() {
                    // java String str = UnicodeString.parsedByteToStr(tmpByte);
                    // java this.allStrs.set(id, str);
                    // java this.nonParsedStrs.set(id, null);
                    // java --this.nonParsedStrCount;
                    // rust: no-stmt — UnicodeString.parsedByteToStr is 11_function_shared_strings
                    let _ = tmpByte;
                    panic!("UnicodeString.parsedByteToStr");
                }
            }
        }
        // java String s = (String)this.allStrs.get(id);
        let s = self.allStrs[id as usize].clone();
        // java if (s == null)
        if s.is_none() {
            // java UnicodeString ustr = (UnicodeString)this.runsStrs.get(id);
            let has = self.runsStrs.containsKey(id);
            // java return ustr == null ? "" : ustr.getString();
            if !has {
                String::new()
            } else {
                self.runsStrs.get(id).as_ref().unwrap().downcast_ref::<OoxmlStr>().expect("UnicodeString").getString().to_string()
            }
        } else {
            // java return s;
            s.unwrap()
        }
    }

    // java public String getString(int id)
    pub fn getString(&mut self, mut id: i32) -> String {
        // java int stringCount = this.existedStringCount;
        let stringCount = self.existedStringCount;
        // java if (id >= this.allStrs.size() + stringCount)
        if id >= self.allStrs.len() as i32 + stringCount {
            // java return "";
            String::new()
        // java else if (id < stringCount)
        } else if id < stringCount {
            // java byte[] value = this.kmoPersistence.queryData(id);
            let value = self.kmoPersistence.queryData(id);
            // java return value != null ? new String(value, StandardCharsets.UTF_8) : "";
            match value {
                Some(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                None => String::new(),
            }
        } else {
            // java id -= stringCount;
            id -= stringCount;
            // java return this.getSValsStr(id);
            self.getSValsStr(id)
        }
    }

    // java public UnicodeString getRunsString(int id)
    pub fn getRunsString(&mut self,id:i32)->Option<crate::java_runtime::Rc<OoxmlStr>>{
        // java UnicodeString us=(UnicodeString)this.runsStrs.get(id);if(us!=null&&!us.isStrParsed())us.parsedBytesToStr();return us;
        self.runsStrs.get(id).as_ref().and_then(|v|v.clone().downcast::<OoxmlStr>().ok())
    }

    // java public Iterator<UnicodeString> getStrStoreIterator()
    pub fn getStrStoreIterator(&mut self)->StrStoreIterator<'_>{
        // java return new Iterator<UnicodeString>() { private int curId = 0; ... };
        StrStoreIterator{outer:self,curId:0}
    }

    // java public boolean isRunsString(int id)
    pub fn isRunsString(&self, id: i32) -> bool {
        // java return this.runsStrs == null ? false : this.runsStrs.containsKey(id);
        self.runsStrs.containsKey(id)
    }
}

// Java anonymous Iterator ownership adapter. The owning store remains borrowed;
// mutation through a second Java alias during iteration is outside this slice.
pub struct StrStoreIterator<'a>{outer:&'a mut MemoryMixSQLStringList,curId:i32}
impl StrStoreIterator<'_>{
    // java public boolean hasNext() { return this.curId < MemoryMixSQLStringList.this.allStrs.size() + MemoryMixSQLStringList.this.existedStringCount; }
    pub fn hasNext(&self)->bool{self.curId<(self.outer.allStrs.len()as i32).wrapping_add(self.outer.existedStringCount)}
    // java public UnicodeString next() {
    pub fn next(&mut self)->crate::java_runtime::Rc<OoxmlStr>{
        // java UnicodeString str = MemoryMixSQLStringList.this.getRunsString(this.curId);
        let value=self.outer.getRunsString(self.curId);
        // java if (str != null) { ++this.curId; return str; }
        if let Some(value)=value{self.curId=self.curId.wrapping_add(1);value}
        // java else { return new UnicodeString(MemoryMixSQLStringList.this.getString(this.curId++)); }
        else{let id=self.curId;self.curId=self.curId.wrapping_add(1);crate::java_runtime::Rc::new(OoxmlStr::new(self.outer.getString(id)))}
    }
    // java public void remove() { }
    pub fn remove(&mut self){}
}

fn index_of(all: &[Option<String>], value: Option<&str>) -> i32 {
    for (i, item) in all.iter().enumerate() {
        match (item, value) {
            (None, None) => return i as i32,
            (Some(s), Some(v)) if s == v => return i as i32,
            _ => {}
        }
    }
    -1
}
