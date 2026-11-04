// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.util.LittleEndian
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException

// Java: public class KOLEDocument implements OLEConstants, IOLEDocument
pub struct KOLEDocument {
    // Java field: byte[] mHeader
    mHeader: Option<Vec<u8>>,
    // Java field: boolean mHeaderDirty
    pub(crate) mHeaderDirty: bool,
    // Java field: RandomAccessFile mFile
    mFile: Option<Box<dyn std::any::Any>>,
    // Java field: int mMode
    pub(crate) mMode: i32,
    // Java field: int mFileSize
    pub(crate) mFileSize: i32,
    // Java field: BufferManager mBufferManager
    pub(crate) mBufferManager: Option<Box<dyn std::any::Any>>,
    // Java field: MSAT mMSAT
    pub(crate) mMSAT: Option<Box<dyn std::any::Any>>,
    // Java field: SAT mSAT
    pub(crate) mSAT: Option<Box<dyn std::any::Any>>,
    // Java field: DirectoryManager mDirectoryManager
    pub(crate) mDirectoryManager: Option<Box<dyn std::any::Any>>,
    // Java field: SSAT mSSAT
    pub(crate) mSSAT: Option<Box<dyn std::any::Any>>,
    // Java field: ShortSectorContainer mShortSectorContainer
    pub(crate) mShortSectorContainer: Option<Box<dyn std::any::Any>>,
    // Java field: byte[] _ole_signature
    _ole_signature: Vec<u8>,
}

// Java: public class KOLEDocument implements OLEConstants, IOLEDocument
impl KOLEDocument {
    // Java: KOLEDocument()
    pub fn new() -> Self {
      // Java: super();
      KOLEDocument {
          mHeader: None,
          mHeaderDirty: false,
          mFile: None,
          mMode: 0,
          mFileSize: 0,
          mBufferManager: None,
          mMSAT: None,
          mSAT: None,
          mDirectoryManager: None,
          mSSAT: None,
          mShortSectorContainer: None,
          _ole_signature: vec![0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1],
      }
    }

    // Java: BufferManager getBufferManager()
    pub fn getBufferManager(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mBufferManager;
      self.mBufferManager.as_ref()
    }

    // Java: MSAT getMSAT()
    pub fn getMSAT(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mMSAT;
      self.mMSAT.as_ref()
    }

    // Java: SAT getSAT()
    pub fn getSAT(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mSAT;
      self.mSAT.as_ref()
    }

    // Java: SSAT getSSAT()
    pub fn getSSAT(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mSSAT;
      self.mSSAT.as_ref()
    }

    // Java: ShortSectorContainer getShortSectorContainer()
    pub fn getShortSectorContainer(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mShortSectorContainer;
      self.mShortSectorContainer.as_ref()
    }

    // Java: DirectoryManager getDirectoryManager()
    pub fn getDirectoryManager(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mDirectoryManager;
      self.mDirectoryManager.as_ref()
    }

    // Java: int getSectorShift()
    pub fn getSectorShift(&self) -> i32 {
      // Java: return LittleEndian.getShort(this.mHeader, 30);
      todo!()
    }

    // Java: int getSectorSize()
    pub fn getSectorSize(&self) -> i32 {
      // Java: return 1 << this.getSectorShift();
      todo!()
    }

    // Java: int getShortSectorShift()
    pub fn getShortSectorShift(&self) -> i32 {
      // Java: return LittleEndian.getShort(this.mHeader, 32);
      todo!()
    }

    // Java: int getShortSectorSize()
    pub fn getShortSectorSize(&self) -> i32 {
      // Java: return 1 << this.getShortSectorShift();
      todo!()
    }

    // Java: int getShortStreamLimit()
    pub fn getShortStreamLimit(&self) -> i32 {
      // Java: return LittleEndian.getInt(this.mHeader, 56);
      todo!()
    }

    // Java: int getDirectoryStartSector()
    pub fn getDirectoryStartSector(&self) -> i32 {
      // Java: return LittleEndian.getInt(this.mHeader, 48);
      todo!()
    }

    // Java: void setDirectoryStartSector(int sector)
    pub fn setDirectoryStartSector(&mut self, _sector: i32) {
      // Java: this.setHeaderDirty(true);

      // Java: LittleEndian.putInt(this.mHeader, 48, sector);

    }

    // Java: int getMSATStartSector()
    pub fn getMSATStartSector(&self) -> i32 {
      // Java: return LittleEndian.getInt(this.mHeader, 68);
      todo!()
    }

    // Java: void setMSATStartSector(int sector)
    pub fn setMSATStartSector(&mut self, _sector: i32) {
      // Java: LittleEndian.putInt(this.mHeader, 68, sector);

      // Java: this.setHeaderDirty(true);

    }

    // Java: int getMSATSectorCount()
    pub fn getMSATSectorCount(&self) -> i32 {
      // Java: return LittleEndian.getInt(this.mHeader, 72);
      todo!()
    }

    // Java: void setMSATSectorCount(int count)
    pub fn setMSATSectorCount(&mut self, _count: i32) {
      // Java: LittleEndian.putInt(this.mHeader, 72, count);

      // Java: this.setHeaderDirty(true);

    }

    // Java: int getHeaderMSAT(int index)
    pub fn getHeaderMSAT(&self, _index: i32) -> i32 {
      // Java: return LittleEndian.getInt(this.mHeader, 76 + index * 4);
      todo!()
    }

    // Java: void setHeaderMSAT(int index, int value)
    pub fn setHeaderMSAT(&mut self, _index: i32, _value: i32) {
      // Java: LittleEndian.putInt(this.mHeader, 76 + index * 4, value);

      // Java: this.setHeaderDirty(true);

    }

    // Java: int getSATSectorCount()
    pub fn getSATSectorCount(&self) -> i32 {
      // Java: return LittleEndian.getInt(this.mHeader, 44);
      todo!()
    }

    // Java: void setSATSectorCount(int nSATSector)
    pub fn setSATSectorCount(&mut self, _nSATSector: i32) {
      // Java: LittleEndian.putInt(this.mHeader, 44, nSATSector);

      // Java: this.setHeaderDirty(true);

    }

    // Java: int getSSATStartSector()
    pub fn getSSATStartSector(&self) -> i32 {
      // Java: return LittleEndian.getInt(this.mHeader, 60);
      todo!()
    }

    // Java: void setSSATStartSector(int sector)
    pub fn setSSATStartSector(&mut self, _sector: i32) {
      // Java: LittleEndian.putInt(this.mHeader, 60, sector);

      // Java: this.setHeaderDirty(true);

    }

    // Java: int getSSATSectorCount()
    pub fn getSSATSectorCount(&self) -> i32 {
      // Java: return LittleEndian.getInt(this.mHeader, 64);
      todo!()
    }

    // Java: void setSSATSectorCount(int count)
    pub fn setSSATSectorCount(&mut self, _count: i32) {
      // Java: LittleEndian.putInt(this.mHeader, 64, count);

      // Java: this.setHeaderDirty(true);

    }

    // Java: int getShortSectorContainerStartSector()
    pub fn getShortSectorContainerStartSector(&self) -> i32 {
      // Java: return this.getRootEntry().getFirstSector();
      todo!()
    }

    // Java: void setShortSectorContainerStartSector(int sector)
    pub fn setShortSectorContainerStartSector(&mut self, _sector: i32) {
      // Java: this.getRootEntry().setFirstSector(sector);

    }

    // Java: int getShortSectorContainerSize()
    pub fn getShortSectorContainerSize(&self) -> i32 {
      // Java: return this.getRootEntry().getSize();
      todo!()
    }

    // Java: void setShortSectorContainerSize(int nNewSize)
    pub fn setShortSectorContainerSize(&mut self, _nNewSize: i32) {
      // Java: this.getRootEntry().setSize(nNewSize);

    }

    // Java: KDirEntry getRootEntry()
    pub fn getRootEntry(&self) -> Option<Box<dyn std::any::Any>> {
      // Java: try

      {
         // Java: return this.mDirectoryManager.getEntry(0);

      }
      // Java: catch (IOException var2)

      {
         // Java: return null;

      }
      todo!()
    }

    // Java: boolean getHeaderDirty()
    pub fn getHeaderDirty(&self) -> bool {
      // Java: return this.mHeaderDirty;
      self.mHeaderDirty
    }

    // Java: void setHeaderDirty(boolean bDirty)
    pub fn setHeaderDirty(&mut self, _bDirty: bool) {
      // Java: this.mHeaderDirty = bDirty;
      self.mHeaderDirty = _bDirty;
    }

    // Java: int getCurrentFileSize()
    pub fn getCurrentFileSize(&self) -> i32 {
      // Java: return this.mFileSize;
      self.mFileSize
    }

    // Java: int getMode()
    pub fn getMode(&self) -> i32 {
      // Java: return this.mMode;
      self.mMode
    }

    // Java: int getBufferSize()
    pub fn getBufferSize(&self) -> i32 {
      // Java: return this.mBufferManager.getBufferSize();
      todo!()
    }

    // Java: void setBufferSize(int nBufferSize)
    pub fn setBufferSize(&mut self, _nBufferSize: i32) {
      // Java: this.mBufferManager.setBufferSize(nBufferSize);

    }

    // Java: int readFile(byte[] buf, int bufStart, int nbytes, int pos)
    pub fn readFile(&mut self, _buf: &mut [u8], _bufStart: i32, _nbytes: i32, _pos: i32) -> i32 {
      // Java: this.mFile.seek((long)pos);

      // Java: return this.mFile.read(buf, bufStart, nbytes);

      todo!()
    }

    // Java: void writeFile(byte[] buf, int bufStart, int nbytes, int pos)
    pub fn writeFile(&mut self, _buf: &[u8], _bufStart: i32, _nbytes: i32, _pos: i32) {
      // Java: if (pos + nbytes > this.mFileSize)

      {
         // Java: this.mFileSize = pos + nbytes;

      }
      // Java: this.mFile.seek((long)pos);

      // Java: this.mFile.write(buf, bufStart, nbytes);

    }

    // Java: IOLEStorage getRootStorage()
    pub fn getRootStorage(&self) -> Option<Box<dyn std::any::Any>> {
      // Java: KDirEntry pRootEntry = this.getRootEntry();

      // Java: KOLEStorage pStorage = new KOLEStorage(this, (KOLEStorage)null, pRootEntry);

      // Java: return pStorage;

      todo!()
    }

    // Java: void open(String szPath, int mode)
    pub fn open_from_path(&mut self, _szPath: &str, _mode: i32) {
      // Java: this.open(new File(szPath), mode);

    }

    // Java: void open(File file, int mode)
    pub fn open(&mut self, _file: Box<dyn std::any::Any>, _mode: i32) {
      // Java: if (file != null && file.exists())

      {
         // Java: this.mFileSize = (int)file.length();

         // Java: this.mMode = mode;

         // Java: String strMode = "r";

         // Java: if (mode == 0)

         {
            // Java: strMode = "r";

         }
         // Java: else if (mode == 1)

         {
            // Java: strMode = "w";

         }
         // Java: else

         {
            // Java: if (mode != 2)

            {
               // Java: throw new OLEException("Opening file [" + file.getAbsolutePath() + "] in invalid mode: " + mode);

            }
            // Java: strMode = "rw";

         }
         // Java: this.mFile = new RandomAccessFile(file, strMode);

         // Java: this.mHeader = new byte[512];

         // Java: this.readFile(this.mHeader, 0, 512, 0);

         // Java: if (!this.checkHeader())

         {
            // Java: this.close();

            // Java: throw new OLEException("File corrupted: check header failed");

         }
         // Java: else

         {
            // Java: this.mBufferManager = new BufferManager(this);

            // Java: this.mMSAT = new MSAT(this);

            // Java: this.mMSAT.doOpen();

            // Java: this.mSAT = new SAT(this);

            // Java: this.mDirectoryManager = new DirectoryManager(this);

            // Java: this.mSSAT = new SSAT(this);

            // Java: this.mShortSectorContainer = new ShortSectorContainer(this);

            // Java: KDirEntry pRootEntry = this.getRootEntry();

            // Java: if (pRootEntry == null)

            {
               // Java: throw new OLEException("Invalid root entry, file may be corrupted");

            }
         }
      }
      // Java: else

      {
         // Java: throw new FileNotFoundException("File not exists!");

      }
    }

    // Java: boolean checkHeader()
    pub fn checkHeader(&self) -> bool {
      // Java: for(int i = 0; i < _ole_signature.length; ++i)

      {
         // Java: if (this.mHeader[i] != _ole_signature[i])

         {
            // Java: return false;

         }
      }
      // Java: if (LittleEndian.getUShort(this.mHeader, 28) != 65534)

      {
         // Java: return false;

      }
      // Java: else

      {
         // Java: int nSectorShift = this.getSectorShift();

         // Java: int nShortSectorShift = this.getShortSectorShift();

         // Java: if (nSectorShift < 7)

         {
            // Java: return false;

         }
         // Java: else if (nSectorShift > 20)

         {
            // Java: return false;

         }
         // Java: else if (nShortSectorShift >= 5 && nShortSectorShift < nSectorShift)

         {
            // Java: if (this.getShortStreamLimit() <= this.getShortSectorSize())

            {
               // Java: return false;

            }
            // Java: else

            {
               // Java: return true;

            }
         }
         // Java: else

         {
            // Java: return false;

         }
      }
      todo!()
    }

    // Java: void create(String szPath, int mode)
    pub fn create_from_path(&mut self, _szPath: &str, _mode: i32) {
      // Java: this.create(new File(szPath), mode);

    }

    // Java: void create(File file, int mode)
    pub fn create(&mut self, _file: Box<dyn std::any::Any>, _mode: i32) {
      // Java: if (file == null)

      {
         // Java: throw new FileNotFoundException("Invalid argments");

      }
      // Java: else

      {
         // Java: this.mMode = mode;

         // Java: if (mode != 1 && mode != 2)

         {
            // Java: throw new OLEException("Create file [" + file.getAbsolutePath() + "] in invalid Mode: " + mode);

         }
         // Java: else

         {
            // Java: if (!file.exists())

            {
               // Java: file.createNewFile();

            }
            // Java: String strMode = "rw";

            // Java: this.mFile = new RandomAccessFile(file, strMode);

            // Java: this.mFileSize = 0;

            // Java: this.mHeader = new byte[512];

            // Java: System.arraycopy(_ole_signature, 0, this.mHeader, 0, _ole_signature.length);

            // Java: LittleEndian.putUShort(this.mHeader, 26, 3);

            // Java: LittleEndian.putUShort(this.mHeader, 24, 62);

            // Java: LittleEndian.putUShort(this.mHeader, 28, 65534);

            // Java: LittleEndian.putUShort(this.mHeader, 30, 9);

            // Java: LittleEndian.putUShort(this.mHeader, 32, 6);

            // Java: LittleEndian.putInt(this.mHeader, 56, 4096);

            // Java: this.setMSATStartSector(-2);

            // Java: this.setDirectoryStartSector(-2);

            // Java: this.setSSATStartSector(-2);

            // Java: for(int iSAT = 0; iSAT < 109; ++iSAT)

            {
               // Java: this.setHeaderMSAT(iSAT, -1);

            }
            // Java: this.setHeaderDirty(true);

            // Java: this.mBufferManager = new BufferManager(this);

            // Java: this.mMSAT = new MSAT(this);

            // Java: this.mSAT = new SAT(this);

            // Java: this.mDirectoryManager = new DirectoryManager(this);

            // Java: this.createRoot();

            // Java: this.mSSAT = new SSAT(this);

            // Java: this.mShortSectorContainer = new ShortSectorContainer(this);

         }
      }
    }

    // Java: void createRoot()
    pub fn createRoot(&mut self) {
      // Java: KDirEntry pEntry = this.mDirectoryManager.allocEntry();

      // Java: pEntry.setName("Root Entry");

      // Java: pEntry.setType(5);

      // Java: pEntry.setColor(1);

    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.mDirectoryManager = null;
      self.mDirectoryManager = None;
      // Java: this.mShortSectorContainer = null;
      self.mShortSectorContainer = None;
      // Java: this.mSSAT = null;
      self.mSSAT = None;
      // Java: this.mSAT = null;
      self.mSAT = None;
      // Java: this.mMSAT = null;
      self.mMSAT = None;
      // Java: if (this.mBufferManager != null)

      {
         // Java: this.mBufferManager.flush();

      }
      // Java: this.mBufferManager = null;
      self.mBufferManager = None;
      // Java: if (this.mFile != null && this.getHeaderDirty())

      {
         // Java: this.writeFile(this.mHeader, 0, 512, 0);

         // Java: this.setHeaderDirty(false);

      }
      // Java: if (this.mFile != null)

      {
         // Java: this.mFile.close();

         // Java: this.mFile = null;

      }
    }

    // Java: byte[] getBuffers()
    pub fn getBuffers(&self) -> Option<Vec<u8>> {
      // Java: return null != this.mBufferManager ? this.mBufferManager.getBuffers(this.mHeader) : null;
      todo!()
    }

}

impl super::OLEConstants::OLEConstants for KOLEDocument {
}

impl super::IOLEDocument::IOLEDocument for KOLEDocument {
    fn getRootStorage(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    fn getSectorSize(&self) -> i32 {
        KOLEDocument::getSectorSize(self)
    }

    fn getBufferSize(&self) -> i32 {
        KOLEDocument::getBufferSize(self)
    }

    fn setBufferSize(&mut self, var1: i32) {
        KOLEDocument::setBufferSize(self, var1);
    }

    fn getBuffers(&self) -> Vec<u8> {
        todo!()
    }
}
