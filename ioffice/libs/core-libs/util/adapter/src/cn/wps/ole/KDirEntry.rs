// Java: import java.io.IOException

use super::Sector::Sector;

// Java: public class KDirEntry
pub struct KDirEntry {
    // Java field: DirectoryManager mDirectoryManager
    pub(crate) mDirectoryManager: Option<Box<dyn std::any::Any>>,
    // Java field: int mIndex
    pub(crate) mIndex: i32,
    // Java field: Sector mSector
    pub(crate) mSector: Option<Sector>,
    // Java field: int mByteOffset
    pub(crate) mByteOffset: i32,
    // Java field: int mType
    pub(crate) mType: i32,
    // Java field: int mFirstSector
    pub(crate) mFirstSector: i32,
    // Java field: int mSize
    pub(crate) mSize: i32,
    // Java field: char[] mCompareKey
    pub(crate) mCompareKey: Option<Vec<char>>,
    // Java field: byte[] mClsid
    pub(crate) mClsid: Option<Vec<u8>>,
    // Java field: boolean mShortSector
    pub(crate) mShortSector: bool,
}

// Java: public class KDirEntry
impl KDirEntry {
    // Java: KDirEntry(DirectoryManager m, int index, Sector sector, int byteOffset)
    pub fn new(_m: Box<dyn std::any::Any>, _index: i32, _sector: Sector, _byteOffset: i32) -> Self {
        // Java: super();
        // Java: this.mDirectoryManager = m;
        // Java: this.mIndex = index;
        // Java: this.mSector = sector;
        // Java: this.mByteOffset = byteOffset;
        // Java: this.mCompareKey = null;
        // Java: this.mClsid = null;
        KDirEntry {
            mDirectoryManager: Some(_m),
            mIndex: _index,
            mSector: Some(_sector),
            mByteOffset: _byteOffset,
            mType: 0,
            mFirstSector: 0,
            mSize: 0,
            mCompareKey: None,
            mClsid: None,
            mShortSector: false,
        }
    }

    // Java: public void dispose()
    pub fn dispose(&mut self) {
        // Java: this.mDirectoryManager = null;
        self.mDirectoryManager = None;
        // Java: this.mSector = null;
        self.mSector = None;
        // Java: this.mCompareKey = null;
        self.mCompareKey = None;
        // Java: this.mClsid = null;
        self.mClsid = None;
    }

    // Java: public final boolean isShortSector()
    pub fn isShortSector(&self) -> bool {
        // Java: return this.mShortSector;
        self.mShortSector
    }

    // Java: public final void setShortSector(boolean bShortSector)
    pub fn setShortSector(&mut self, _bShortSector: bool) {
        // Java: this.mShortSector = bShortSector;
        self.mShortSector = _bShortSector;
    }

    // Java: final void onOpen()
    pub fn onOpen(&mut self) {
        // Java: this.mType = FileOLEEntry.getType(this.mSector.content, this.mByteOffset);
        // Java: this.mSize = FileOLEEntry.getSize(this.mSector.content, this.mByteOffset);
        // Java: this.mClsid = FileOLEEntry.getClsid(this.mSector.content, this.mByteOffset);
        // Java: this.mFirstSector = FileOLEEntry.getFirstSector(this.mSector.content, this.mByteOffset);
        // Java: this.resetCompareKey();
        // Java: this.mShortSector = this.getSize() < this.mDirectoryManager.getDocument().getShortStreamLimit();
        todo!()
    }

    // Java: final void onNew()
    pub fn onNew(&mut self) {
        // Java: this.setLeft(-1);
        // Java: this.setRight(-1);
        // Java: this.setChildren(-1);
        // Java: this.setFirstSector(-2);
        // Java: this.setSize(0);
        // Java: this.setColor(1);
        // Java: this.mShortSector = false;
        todo!()
    }

    // Java: private void resetCompareKey()
    fn resetCompareKey(&mut self) {
        // Java: int nchar = FileOLEEntry.getNameLength(this.mSector.content, this.mByteOffset);
        // Java: this.mCompareKey = new char[nchar];
        // Java: FileOLEEntry.getName(this.mSector.content, this.mByteOffset, this.mCompareKey);
        // Java: for(int i = 0; i < nchar; ++i)
        {
            // Java: char c = this.mCompareKey[i];
            // Java: if ('a' <= c && c <= 'z')
            {
                // Java: c = (char)(c + 65 - 97);
            }
        }
    }

    // Java: public final int getIndex()
    pub fn getIndex(&self) -> i32 {
        // Java: return this.mIndex;
        self.mIndex
    }

    // Java: public final int getFirstSector()
    pub fn getFirstSector(&self) -> i32 {
        // Java: return this.mFirstSector;
        self.mFirstSector
    }

    // Java: public final void setFirstSector(int sector)
    pub fn setFirstSector(&mut self, _sector: i32) {
        // Java: this.mFirstSector = sector;
        // Java: FileOLEEntry.setFirstSector(this.mSector.content, this.mByteOffset, sector);
        // Java: this.setDirty();
        todo!()
    }

    // Java: public final int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this.mSize;
        self.mSize
    }

    // Java: public final void setSize(int nSize)
    pub fn setSize(&mut self, _nSize: i32) {
        // Java: this.mSize = nSize;
        // Java: FileOLEEntry.setSize(this.mSector.content, this.mByteOffset, nSize);
        // Java: this.setDirty();
        todo!()
    }

    // Java: public final void setName(String szName)
    pub fn setName(&mut self, _szName: &str) {
        // Java: FileOLEEntry.setName(this.mSector.content, this.mByteOffset, szName);
        // Java: this.setDirty();
        // Java: this.resetCompareKey();
        todo!()
    }

    // Java: public final String getName()
    pub fn getName(&self) -> String {
        // Java: char[] chars = FileOLEEntry.getName(this.mSector.content, this.mByteOffset);
        // Java: return new String(chars);
        todo!()
    }

    // Java: public final int getNameLength()
    pub fn getNameLength(&self) -> i32 {
        // Java: return FileOLEEntry.getNameLength(this.mSector.content, this.mByteOffset);
        todo!()
    }

    // Java: public final char[] getCompareKey()
    pub fn getCompareKey(&self) -> Option<&Vec<char>> {
        // Java: return this.mCompareKey;
        self.mCompareKey.as_ref()
    }

    // Java: public final int getType()
    pub fn getType(&self) -> i32 {
        // Java: return this.mType;
        self.mType
    }

    // Java: public final void setType(int type)
    pub fn setType(&mut self, _type: i32) {
        // Java: this.mType = type;
        // Java: FileOLEEntry.setType(this.mSector.content, this.mByteOffset, type);
        // Java: this.setDirty();
        todo!()
    }

    // Java: public final void setChildren(int index)
    pub fn setChildren(&mut self, _index: i32) {
        // Java: FileOLEEntry.setChildren(this.mSector.content, this.mByteOffset, index);
        // Java: this.setDirty();
        todo!()
    }

    // Java: public final byte[] getStorageClsid()
    pub fn getStorageClsid(&self) -> Option<&Vec<u8>> {
        // Java: return this.mClsid;
        self.mClsid.as_ref()
    }

    // Java: public final void setStorageClsid(byte[] clsid)
    pub fn setStorageClsid(&mut self, _clsid: Vec<u8>) {
        // Java: this.mClsid = clsid;
        // Java: FileOLEEntry.setClsid(this.mSector.content, this.mByteOffset, clsid);
        // Java: this.setDirty();
        todo!()
    }

    // Java: public final void setChildren(KDirEntry entry)
    pub fn setChildren_from_entry(&mut self, _entry: &KDirEntry) {
        // Java: this.setChildren(entry.mIndex);
        todo!()
    }

    // Java: public final int getColor()
    pub fn getColor(&self) -> i32 {
        // Java: return FileOLEEntry.getColor(this.mSector.content, this.mByteOffset);
        todo!()
    }

    // Java: public final void setColor(int color)
    pub fn setColor(&mut self, _color: i32) {
        // Java: FileOLEEntry.setColor(this.mSector.content, this.mByteOffset, color);
        // Java: this.setDirty();
        todo!()
    }

    // Java: public final void setLeft(int left)
    pub fn setLeft(&mut self, _left: i32) {
        // Java: FileOLEEntry.setLeft(this.mSector.content, this.mByteOffset, left);
        // Java: this.setDirty();
        todo!()
    }

    // Java: public final void setLeft(KDirEntry left)
    pub fn setLeft_from_entry(&mut self, _left: &KDirEntry) {
        // Java: this.setLeft(left.mIndex);
        todo!()
    }

    // Java: public final void setRight(int right)
    pub fn setRight(&mut self, _right: i32) {
        // Java: FileOLEEntry.setRight(this.mSector.content, this.mByteOffset, right);
        // Java: this.setDirty();
        todo!()
    }

    // Java: public final void setRight(KDirEntry right)
    pub fn setRight_from_entry(&mut self, _right: &KDirEntry) {
        // Java: this.setRight(right.mIndex);
        todo!()
    }

    // Java: public final KDirEntry getChildren()
    pub fn getChildren(&self) -> Option<Box<dyn std::any::Any>> {
        // Java: return this.mDirectoryManager.getEntry(FileOLEEntry.getChildren(this.mSector.content, this.mByteOffset));
        todo!()
    }

    // Java: public final KDirEntry getLeft()
    pub fn getLeft(&self) -> Option<Box<dyn std::any::Any>> {
        // Java: return this.mDirectoryManager.getEntry(FileOLEEntry.getLeft(this.mSector.content, this.mByteOffset));
        todo!()
    }

    // Java: public final KDirEntry getRight()
    pub fn getRight(&self) -> Option<Box<dyn std::any::Any>> {
        // Java: return this.mDirectoryManager.getEntry(FileOLEEntry.getRight(this.mSector.content, this.mByteOffset));
        todo!()
    }

    // Java: private final void setDirty()
    fn setDirty(&mut self) {
        // Java: this.mSector.setDirty();
        if let Some(ref mut sector) = self.mSector {
            sector.setDirty();
        }
    }
}
