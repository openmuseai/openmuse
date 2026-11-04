// Java: import java.io.DataInput
// Java: import java.io.DataOutput
// Java: import java.io.EOFException
// Java: import java.io.IOException
// Java: import java.io.UTFDataFormatException

use super::KOLEObjectBase::KOLEObjectBase;
use super::KDirEntry::KDirEntry;
use super::SectorChain::SectorChain;

// Java: public class KOLEStream extends KOLEObjectBase implements OLEConstants, IOLEStream, Cloneable
pub struct KOLEStream {
    // Java field: KOLEStorage mParent
    mParent: Option<Box<dyn std::any::Any>>,
    // Java field: BufferManager mBufferManager
    mBufferManager: Option<Box<dyn std::any::Any>>,
    // Java field: SAT mSAT
    mSAT: Option<Box<dyn std::any::Any>>,
    // Java field: SSAT mSSAT
    mSSAT: Option<Box<dyn std::any::Any>>,
    // Java field: ShortSectorContainer mShortSectorContainer
    mShortSectorContainer: Option<Box<dyn std::any::Any>>,
    // Java field: KDirEntry mEntry
    pub(crate) mEntry: Option<KDirEntry>,
    // Java field: int mPosition
    pub(crate) mPosition: i32,
    // Java field: SectorChain mChain
    mChain: Option<SectorChain>,
    pub(crate) base: KOLEObjectBase,
}

// Java: public class KOLEStream extends KOLEObjectBase implements OLEConstants, IOLEStream, Cloneable
impl KOLEStream {
    // Java: KDirEntry getEntry()
    pub fn getEntry(&self) -> Option<&KDirEntry> {
      // Java: return this.mEntry;
      self.mEntry.as_ref()
    }

    // Java: KOLEStream(KOLEStorage storage, KDirEntry entry)
    pub fn new(_storage: Box<dyn std::any::Any>, _entry: KDirEntry) -> Self {
      // Java: super();
      // Java: this.initBase(storage.getDocument());
      // Java: this.mParent = storage;
      // Java: this.mBufferManager = this.mDocument.getBufferManager();
      // Java: this.mSAT = this.mDocument.getSAT();
      // Java: this.mSSAT = this.mDocument.getSSAT();
      // Java: this.mShortSectorContainer = this.mDocument.getShortSectorContainer();
      // Java: this.mEntry = entry;
      // Java: this.mPosition = 0;
      // Java: this.mChain = new SectorChain();
      // Java: if (entry.isShortSector())
      {
         // Java: this.mChain.reserve(this.mEntry.getSize() >> this.SHORT_SECTOR_SHIFT);
         // Java: this.mSSAT.getSectorChain(this.mEntry.getFirstSector(), this.mChain);
      }
      // Java: else
      {
         // Java: this.mChain.reserve(this.mEntry.getSize() >> this.SECTOR_SHIFT);
         // Java: this.mSAT.getSectorChain(this.mEntry.getFirstSector(), this.mChain);
      }
      todo!()
    }

    // Java: Object clone()
    pub fn clone_stream(&self) -> Box<dyn std::any::Any> {
      // Java: try
      {
         // Java: KOLEStream pStream = new KOLEStream(this.mParent, this.mEntry);
         // Java: return pStream;
      }
      // Java: catch (IOException e)
      {
         // Java: throw new RuntimeException(e);
      }
      todo!()
    }

    // Java: void seek(int offset)
    pub fn seek(&mut self, _offset: i32) {
      // Java: if (offset < 0)
      {
         // Java: throw new EOFException("Seek underflows: " + offset);
      }
      // Java: else if (offset > this.mEntry.getSize() && this.mDocument.getMode() == 0)
      {
         // Java: throw new EOFException("Seek past stream end but in read-only mode: " + offset);
      }
      // Java: else
      {
         // Java: this.mPosition = offset;
      }
    }

    // Java: int tell()
    pub fn tell(&self) -> i32 {
      // Java: return this.mPosition;
      self.mPosition
    }

    // Java: void setSize(int nSize)
    pub fn setSize(&mut self, _nSize: i32) {
      // Java: throw new OLEException("Not implemented yet");
    }

    // Java: void stat(OLEEntry entry)
    pub fn stat(&self, _entry: &mut Box<dyn std::any::Any>) {
      // Java: if (entry != null)
      {
         // Java: KDirEntry pEntry = this.getEntry();
         // Java: setEntry(entry, pEntry);
      }
    }

    // Java: int read_impl_normal_sector(byte[] buf, int bufStart, int nbytes)
    pub fn read_impl_normal_sector_buf(&mut self, _buf: &mut [u8], _bufStart: i32, _nbytes: i32) -> i32 {
      // Java: int nSECTOR_SHIFT = this.SECTOR_SHIFT;
      // Java: int nSECTOR_SIZE = this.SECTOR_SIZE;
      // Java: int nTotalRead;
      // Java: int nRead;
      // Java: for(nTotalRead = 0; nbytes > 0; nTotalRead += nRead)
      {
         // Java: int iSector = this.mChain.get(this.mPosition >> nSECTOR_SHIFT);
         // Java: int iSectorOffset = this.mPosition & nSECTOR_SIZE - 1;
         // Java: nRead = Math.min(nSECTOR_SIZE - iSectorOffset, nbytes);
         // Java: Sector pSector = this.mBufferManager.getSector(iSector, true);
         // Java: pSector.unlock();
         // Java: System.arraycopy(pSector.content, iSectorOffset, buf, bufStart + nTotalRead, nRead);
         // Java: nbytes -= nRead;
         // Java: this.mPosition += nRead;
      }
      // Java: return nTotalRead;
      todo!()
    }

    // Java: int read_impl_normal_sector()
    pub fn read_impl_normal_sector(&mut self) -> i32 {
      // Java: int iSector = this.mChain.get(this.mPosition >> this.SECTOR_SHIFT);
      // Java: int iSectorOffset = this.mPosition & this.SECTOR_SIZE - 1;
      // Java: Sector pSector = this.mBufferManager.getSector(iSector, true);
      // Java: pSector.unlock();
      // Java: ++this.mPosition;
      // Java: return pSector.content[iSectorOffset] & 255;
      todo!()
    }

    // Java: int read_impl_short_sector(byte[] buf, int bufStart, int nbytes)
    pub fn read_impl_short_sector_buf(&mut self, _buf: &mut [u8], _bufStart: i32, _nbytes: i32) -> i32 {
      // Java: int nSECTOR_SHIFT = this.SHORT_SECTOR_SHIFT;
      // Java: int nSECTOR_SIZE = this.SHORT_SECTOR_SIZE;
      // Java: int nTotalRead;
      // Java: int nRead;
      // Java: for(nTotalRead = 0; nbytes > 0; nTotalRead += nRead)
      {
         // Java: int iSector = this.mChain.get(this.mPosition >> nSECTOR_SHIFT);
         // Java: int iSectorOffset = this.mPosition & nSECTOR_SIZE - 1;
         // Java: nRead = Math.min(nSECTOR_SIZE - iSectorOffset, nbytes);
         // Java: ShortSector pShortSector = this.mShortSectorContainer.getSector(iSector);
         // Java: Sector pSector = this.mBufferManager.getSector(pShortSector.mNormalSector, true);
         // Java: pSector.unlock();
         // Java: iSectorOffset += pShortSector.mOffset;
         // Java: System.arraycopy(pSector.content, iSectorOffset, buf, bufStart + nTotalRead, nRead);
         // Java: nbytes -= nRead;
         // Java: this.mPosition += nRead;
      }
      // Java: return nTotalRead;
      todo!()
    }

    // Java: int read_impl_short_sector()
    pub fn read_impl_short_sector(&mut self) -> i32 {
      // Java: int iSector = this.mChain.get(this.mPosition >> this.SHORT_SECTOR_SHIFT);
      // Java: int iSectorOffset = this.mPosition & this.SHORT_SECTOR_SIZE - 1;
      // Java: ShortSector pShortSector = this.mShortSectorContainer.getSector(iSector);
      // Java: Sector pSector = this.mBufferManager.getSector(pShortSector.mNormalSector, true);
      // Java: pSector.unlock();
      // Java: iSectorOffset += pShortSector.mOffset;
      // Java: ++this.mPosition;
      // Java: return pSector.content[iSectorOffset] & 255;
      todo!()
    }

    // Java: int read_impl(byte[] buf, int bufStart, int nbytes)
    pub fn read_impl_buf(&mut self, _buf: &mut [u8], _bufStart: i32, _nbytes: i32) -> i32 {
      // Java: int nTotalSize = this.mEntry.getSize();
      // Java: if (this.mPosition + nbytes > nTotalSize)
      {
         // Java: nbytes = nTotalSize - this.mPosition;
      }
      // Java: return this.mEntry.isShortSector() ? this.read_impl_short_sector(buf, bufStart, nbytes) : this.read_impl_normal_sector(buf, bufStart, nbytes);
      todo!()
    }

    // Java: int read_impl()
    pub fn read_impl(&mut self) -> i32 {
      // Java: int nTotalSize = this.mEntry.getSize();
      // Java: if (this.mPosition >= nTotalSize)
      {
         // Java: return -1;
      }
      // Java: else
      {
         // Java: return this.mEntry.isShortSector() ? this.read_impl_short_sector() : this.read_impl_normal_sector();
      }
      todo!()
    }

    // Java: boolean check_need_read(int start, int end)
    pub fn check_need_read(&self, _start: i32, _end: i32) -> bool {
      // Java: if (this.mDocument.getMode() == 1)
      {
         // Java: return false;
      }
      // Java: else
      {
         // Java: return end <= this.mEntry.getSize();
      }
      todo!()
    }

    // Java: void write_impl_short_sector(byte[] buf, int bufStart, int nbytes)
    pub fn write_impl_short_sector_buf(&mut self, _buf: &[u8], _bufStart: i32, _nbytes: i32) {
      // Java: int nSECTOR_SHIFT = this.SHORT_SECTOR_SHIFT;
      // Java: int nSECTOR_SIZE = this.SHORT_SECTOR_SIZE;
      // Java: if (this.mChain.size() << nSECTOR_SHIFT < this.mPosition + nbytes)
      {
         // Java: this.expandCapacity(this.mPosition + nbytes);
      }
      // Java: int nWrite;
      // Java: for(int nTotalWritten = 0; nbytes > 0; nTotalWritten += nWrite)
      {
         // Java: int iSector = this.mChain.get(this.mPosition >> nSECTOR_SHIFT);
         // Java: int iSectorOffset = this.mPosition & nSECTOR_SIZE - 1;
         // Java: nWrite = Math.min(nSECTOR_SIZE - iSectorOffset, nbytes);
         // Java: ShortSector pShortSector = this.mShortSectorContainer.getSector(iSector);
         // Java: Sector pSector = this.mBufferManager.getSector(pShortSector.mNormalSector, this.check_need_read(this.mPosition, this.mPosition + nWrite));
         // Java: iSectorOffset += pShortSector.mOffset;
         // Java: System.arraycopy(buf, bufStart + nTotalWritten, pSector.content, iSectorOffset, nWrite);
         // Java: pSector.setDirty();
         // Java: pSector.unlock();
         // Java: nbytes -= nWrite;
         // Java: this.mPosition += nWrite;
      }
    }

    // Java: void write_impl_normal_sector(byte[] buf, int bufStart, int nbytes)
    pub fn write_impl_normal_sector_buf(&mut self, _buf: &[u8], _bufStart: i32, _nbytes: i32) {
      // Java: int nSECTOR_SHIFT = this.SECTOR_SHIFT;
      // Java: int nSECTOR_SIZE = this.SECTOR_SIZE;
      // Java: if (this.mChain.size() << nSECTOR_SHIFT < this.mPosition + nbytes)
      {
         // Java: this.expandCapacity(this.mPosition + nbytes);
      }
      // Java: int nWrite;
      // Java: for(int nTotalWritten = 0; nbytes > 0; nbytes -= nWrite)
      {
         // Java: int iSector = this.mChain.get(this.mPosition >> nSECTOR_SHIFT);
         // Java: int iSectorOffset = this.mPosition & nSECTOR_SIZE - 1;
         // Java: nWrite = Math.min(nSECTOR_SIZE - iSectorOffset, nbytes);
         // Java: Sector pSector = this.mBufferManager.getSector(iSector, this.check_need_read(this.mPosition, this.mPosition + nWrite));
         // Java: System.arraycopy(buf, bufStart + nTotalWritten, pSector.content, iSectorOffset, nWrite);
         // Java: pSector.setDirty();
         // Java: pSector.unlock();
         // Java: nTotalWritten += nWrite;
         // Java: this.mPosition += nWrite;
      }
      // Java: if (this.mPosition > this.mEntry.getSize())
      {
         // Java: this.mEntry.setSize(this.mPosition);
      }
    }

    // Java: void write_impl(byte[] buf, int bufStart, int nbytes)
    pub fn write_impl_buf(&mut self, _buf: &[u8], _bufStart: i32, _nbytes: i32) {
      // Java: if (this.mEntry.isShortSector())
      {
         // Java: this.write_impl_short_sector(buf, bufStart, nbytes);
      }
      // Java: else
      {
         // Java: this.write_impl_normal_sector(buf, bufStart, nbytes);
      }
    }

    // Java: void write_impl_normal_sector(int b)
    pub fn write_impl_normal_sector_byte(&mut self, _b: i32) {
      // Java: if (this.mChain.size() << this.SECTOR_SHIFT < this.mPosition + 1)
      {
         // Java: this.expandCapacity(this.mPosition + 1);
      }
      // Java: int iSector = this.mChain.get(this.mPosition >> this.SECTOR_SHIFT);
      // Java: int iSectorOffset = this.mPosition & this.SECTOR_SIZE - 1;
      // Java: Sector pSector = this.mBufferManager.getSector(iSector, this.check_need_read(this.mPosition, this.mPosition + 1));
      // Java: pSector.content[iSectorOffset] = (byte)b;
      // Java: pSector.setDirty();
      // Java: pSector.unlock();
      // Java: ++this.mPosition;
      // Java: if (this.mPosition > this.mEntry.getSize())
      {
         // Java: this.mEntry.setSize(this.mPosition);
      }
    }

    // Java: void write_impl_short_sector(int b)
    pub fn write_impl_short_sector_byte(&mut self, _b: i32) {
      // Java: if (this.mChain.size() << this.SHORT_SECTOR_SHIFT < this.mPosition + 1)
      {
         // Java: this.expandCapacity(this.mPosition + 1);
      }
      // Java: int iSector = this.mChain.get(this.mPosition >> this.SHORT_SECTOR_SHIFT);
      // Java: int iSectorOffset = this.mPosition & this.SHORT_SECTOR_SIZE - 1;
      // Java: ShortSector pShortSector = this.mShortSectorContainer.getSector(iSector);
      // Java: Sector pSector = this.mBufferManager.getSector(pShortSector.mNormalSector, this.check_need_read(this.mPosition, this.mPosition + 1));
      // Java: iSectorOffset += pShortSector.mOffset;
      // Java: pSector.content[iSectorOffset] = (byte)b;
      // Java: pSector.setDirty();
      // Java: pSector.unlock();
      // Java: ++this.mPosition;
      // Java: if (this.mPosition > this.mEntry.getSize())
      {
         // Java: this.mEntry.setSize(this.mPosition);
      }
    }

    // Java: void write_impl(int b)
    pub fn write_impl(&mut self, _b: i32) {
      // Java: if (this.mEntry.isShortSector())
      {
         // Java: this.write_impl_short_sector(b);
      }
      // Java: else
      {
         // Java: this.write_impl_normal_sector(b);
      }
    }

    // Java: void expandCapacity(int nDesiredCapacity)
    pub fn expandCapacity(&mut self, _nDesiredCapacity: i32) {
      // Java: KDirEntry pEntry = this.getEntry();
      // Java: if (this.mEntry.isShortSector())
      {
         // Java: SSAT pSSAT = this.mSSAT;
         // Java: int nDesiredSectors = nDesiredCapacity + this.SHORT_SECTOR_SIZE - 1 >> this.SHORT_SECTOR_SHIFT;
         // Java: this.mChain.reserve(nDesiredSectors);
         // Java: int iNewSector;
         // Java: for(; this.mChain.size() < nDesiredSectors; this.mChain.push_back(iNewSector))
         {
            // Java: iNewSector = pSSAT.allocSector();
            // Java: if (this.mChain.empty())
            {
               // Java: pEntry.setFirstSector(iNewSector);
            }
            // Java: else
            {
               // Java: pSSAT.setEntry(this.mChain.back(), iNewSector);
            }
         }
      }
      // Java: else
      {
         // Java: SAT pSAT = this.mSAT;
         // Java: int nDesiredSectors = nDesiredCapacity + this.SECTOR_SIZE - 1 >> this.SECTOR_SHIFT;
         // Java: this.mChain.reserve(nDesiredSectors);
         // Java: int iNewSector;
         // Java: for(; this.mChain.size() < nDesiredSectors; this.mChain.push_back(iNewSector))
         {
            // Java: iNewSector = pSAT.allocSector();
            // Java: pSAT.setEntry(iNewSector, -2);
            // Java: if (this.mChain.empty())
            {
               // Java: pEntry.setFirstSector(iNewSector);
            }
            // Java: else
            {
               // Java: pSAT.setEntry(this.mChain.back(), iNewSector);
            }
         }
      }
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.switchSector();
      // Java: this.checkSectorChain();
    }

    // Java: void checkSectorChain()
    pub fn checkSectorChain(&self) {
    }

    // Java: void switchSector()
    pub fn switchSector(&mut self) {
      // Java: int nShortStreamLimit = this.mDocument.getShortStreamLimit();
      // Java: if (!this.mEntry.isShortSector() && this.mEntry.getSize() < nShortStreamLimit)
      {
         // Java: this._switchToShortSector();
      }
      // Java: if (this.mEntry.isShortSector() && this.mEntry.getSize() >= nShortStreamLimit)
      {
         // Java: this._switchToNormalSector();
      }
    }

    // Java: void _switchToShortSector()
    pub fn _switchToShortSector(&mut self) {
      // Java: int nSize = this.mEntry.getSize();
      // Java: int nShortSector = nSize + this.SHORT_SECTOR_SIZE - 1 >> this.SHORT_SECTOR_SHIFT;
      // Java: int nTotalShortSectorWritten = 0;
      // Java: int nNormalStart = this.mEntry.getFirstSector();
      // Java: int nShortSectorStart = -2;
      // Java: int nShortSectorWrite;
      // Java: for(int iLastShortSector = -2; nNormalStart != -2; nTotalShortSectorWritten += nShortSectorWrite)
      {
         // Java: Sector pNormalSector = this.mBufferManager.getSector(nNormalStart, true);
         // Java: nShortSectorWrite = this.SECTOR_SIZE >> this.SHORT_SECTOR_SHIFT;
         // Java: if (nShortSectorWrite > nShortSector - nTotalShortSectorWritten)
         {
            // Java: nShortSectorWrite = nShortSector - nTotalShortSectorWritten;
         }
         // Java: for(int i = 0; i < nShortSectorWrite; ++i)
         {
            // Java: int offset = i << this.SHORT_SECTOR_SHIFT;
            // Java: int iShortSector = this.mSSAT.allocSector();
            // Java: if (nShortSectorStart == -2)
            {
               // Java: nShortSectorStart = iShortSector;
               // Java: iLastShortSector = iShortSector;
            }
            // Java: else
            {
               // Java: this.mSSAT.setEntry(iLastShortSector, iShortSector);
               // Java: iLastShortSector = iShortSector;
            }
            // Java: ShortSector pShortSector = this.mShortSectorContainer.getSector(iShortSector);
            // Java: Sector pShortSectorHolder = this.mBufferManager.getSector(pShortSector.mNormalSector, false);
            // Java: System.arraycopy(pNormalSector.content, offset, pShortSectorHolder.content, pShortSector.mOffset, this.SHORT_SECTOR_SIZE);
            // Java: pNormalSector.setDirty();
            // Java: pShortSectorHolder.setDirty();
         }
         // Java: pNormalSector.unlock();
         // Java: int iNormalNext = this.mSAT.getEntry(nNormalStart);
         // Java: this.mSAT.freeSector(nNormalStart);
         // Java: nNormalStart = iNormalNext;
      }
      // Java: this.mEntry.setFirstSector(nShortSectorStart);
      // Java: this.mEntry.setShortSector(true);
    }

    // Java: void _switchToNormalSector()
    pub fn _switchToNormalSector(&mut self) {
      // Java: int nSize = this.mEntry.getSize();
      // Java: SectorChain pShortChain = this.mChain;
      // Java: this.mEntry.setFirstSector(-2);
      // Java: this.mEntry.setSize(0);
      // Java: this.mChain = new SectorChain();
      // Java: this.mEntry.setShortSector(false);
      // Java: this.mPosition = 0;
      // Java: int i = 0;
      // Java: for(int pShortChain_size = pShortChain.size(); i < pShortChain_size; ++i)
      {
         // Java: int iShortSector = pShortChain.get(i);
         // Java: ShortSector pShortSector = this.mShortSectorContainer.getSector(iShortSector);
         // Java: Sector pShortSectorHolder = this.mBufferManager.getSector(pShortSector.mNormalSector, true);
         // Java: this.write_impl(pShortSectorHolder.content, pShortSector.mOffset, this.SHORT_SECTOR_SIZE);
         // Java: pShortSectorHolder.unlock();
         // Java: this.mSSAT.freeSector(iShortSector);
      }
      // Java: this.mEntry.setSize(nSize);
      // Java: this.mPosition = nSize;
    }

    // Java: int readBytes(byte[] b, int off, int len)
    pub fn readBytes(&mut self, _b: &mut [u8], _off: i32, _len: i32) -> i32 {
      // Java: return this.read_impl(b, off, len);
      todo!()
    }

    // Java: int read()
    pub fn read(&mut self) -> i32 {
      // Java: return this.read_impl();
      todo!()
    }

    // Java: void writeBytes(byte[] b, int off, int len)
    pub fn writeBytes(&mut self, _b: &[u8], _off: i32, _len: i32) {
      // Java: this.write_impl(b, off, len);
    }

    // Java: void write(int b)
    pub fn write_byte(&mut self, _b: i32) {
      // Java: this.write_impl(b);
    }

    // Java: int read(byte[] buf, int bufStart, int nbytes)
    pub fn read_buf(&mut self, _buf: &mut [u8], _bufStart: i32, _nbytes: i32) -> i32 {
      // Java: return this.readBytes(buf, bufStart, nbytes);
      todo!()
    }

    // Java: void write(byte[] buf, int bufStart, int nbytes)
    pub fn write_buf(&mut self, _buf: &[u8], _bufStart: i32, _nbytes: i32) {
      // Java: this.writeBytes(buf, bufStart, nbytes);
    }

    // Java: void readFully(byte[] b)
    pub fn readFully(&mut self, _b: &mut [u8]) {
      // Java: this.readFully(b, 0, b.length);
    }

    // Java: void readFully(byte[] b, int off, int len)
    pub fn readFully_with_offset(&mut self, _b: &mut [u8], _off: i32, _len: i32) {
      // Java: int n = this.read(b, off, len);
      // Java: if (n < 0)
      {
         // Java: throw new EOFException();
      }
    }

    // Java: int skipBytes(int n)
    pub fn skipBytes(&mut self, _n: i32) -> i32 {
      // Java: if (n <= 0)
      {
         // Java: return 0;
      }
      // Java: else
      {
         // Java: int pos = this.tell();
         // Java: int len = this.mEntry.getSize();
         // Java: int newpos = pos + n;
         // Java: if (newpos > len)
         {
            // Java: newpos = len;
         }
         // Java: this.seek(newpos);
         // Java: return newpos - pos;
      }
      todo!()
    }

    // Java: boolean readBoolean()
    pub fn readBoolean(&mut self) -> bool {
      // Java: int ch = this.read();
      // Java: if (ch < 0)
      {
         // Java: throw new EOFException();
      }
      // Java: else
      {
         // Java: return ch != 0;
      }
      todo!()
    }

    // Java: byte readByte()
    pub fn readByte(&mut self) -> i8 {
      // Java: int ch = this.read();
      // Java: if (ch < 0)
      {
         // Java: throw new EOFException();
      }
      // Java: else
      {
         // Java: return (byte)ch;
      }
      todo!()
    }

    // Java: int readUnsignedByte()
    pub fn readUnsignedByte(&mut self) -> i32 {
      // Java: int ch = this.read();
      // Java: if (ch < 0)
      {
         // Java: throw new EOFException();
      }
      // Java: else
      {
         // Java: return ch;
      }
      todo!()
    }

    // Java: short readShort()
    pub fn readShort(&mut self) -> i16 {
      // Java: int ch1 = this.read();
      // Java: int ch2 = this.read();
      // Java: if ((ch1 | ch2) < 0)
      {
         // Java: throw new EOFException();
      }
      // Java: else
      {
         // Java: return (short)((ch2 << 8) + (ch1 << 0));
      }
      todo!()
    }

    // Java: int readUnsignedShort()
    pub fn readUnsignedShort(&mut self) -> i32 {
      // Java: int ch1 = this.read();
      // Java: int ch2 = this.read();
      // Java: if ((ch1 | ch2) < 0)
      {
         // Java: throw new EOFException();
      }
      // Java: else
      {
         // Java: return (ch2 << 8) + (ch1 << 0);
      }
      todo!()
    }

    // Java: char readChar()
    pub fn readChar(&mut self) -> char {
      // Java: int ch1 = this.read();
      // Java: int ch2 = this.read();
      // Java: if ((ch1 | ch2) < 0)
      {
         // Java: throw new EOFException();
      }
      // Java: else
      {
         // Java: return (char)((ch2 << 8) + (ch1 << 0));
      }
      todo!()
    }

    // Java: int readInt()
    pub fn readInt(&mut self) -> i32 {
      // Java: int ch1 = this.read();
      // Java: int ch2 = this.read();
      // Java: int ch3 = this.read();
      // Java: int ch4 = this.read();
      // Java: if ((ch1 | ch2 | ch3 | ch4) < 0)
      {
         // Java: throw new EOFException();
      }
      // Java: else
      {
         // Java: return (ch4 << 24) + (ch3 << 16) + (ch2 << 8) + (ch1 << 0);
      }
      todo!()
    }

    // Java: long readLong()
    pub fn readLong(&mut self) -> i64 {
      // Java: return ((long)this.readInt() << 32) + ((long)this.readInt() & 4294967295L);
      todo!()
    }

    // Java: float readFloat()
    pub fn readFloat(&mut self) -> f32 {
      // Java: return Float.intBitsToFloat(this.readInt());
      todo!()
    }

    // Java: double readDouble()
    pub fn readDouble(&mut self) -> f64 {
      // Java: return Double.longBitsToDouble(this.readLong());
      todo!()
    }

    // Java: String readLine()
    pub fn readLine(&mut self) -> Option<String> {
      // Java: StringBuffer input = new StringBuffer();
      // Java: int c = -1;
      // Java: boolean eol = false;
      // Java: while(!eol)
      {
         // Java: switch (c = this.read())
         {
            // Java: case -1:
            // Java: case 10:
               // Java: eol = true;
               // Java: break;
            // Java: case 13:
               // Java: eol = true;
               // Java: int cur = this.tell();
               // Java: if (this.read() != 10)
               {
                  // Java: this.seek(cur);
               }
               // Java: break;
            // Java: default:
               // Java: input.append((char)c);
         }
      }
      // Java: if (c == -1 && input.length() == 0)
      {
         // Java: return null;
      }
      // Java: else
      {
         // Java: return input.toString();
      }
      todo!()
    }

    // Java: String readUTF()
    pub fn readUTF(&mut self) -> String {
      // Java: return readUTF(this);
      todo!()
    }

    // Java: void write(byte[] b)
    pub fn write_bytes(&mut self, _b: &[u8]) {
      // Java: this.writeBytes(b, 0, b.length);
    }

    // Java: void writeBoolean(boolean v)
    pub fn writeBoolean(&mut self, _v: bool) {
      // Java: this.write(v ? 1 : 0);
    }

    // Java: void writeByte(int v)
    pub fn writeByte(&mut self, _v: i32) {
      // Java: this.write(v);
    }

    // Java: void writeShort(int v)
    pub fn writeShort(&mut self, _v: i32) {
      // Java: this.write(v >>> 0 & 255);
      // Java: this.write(v >>> 8 & 255);
    }

    // Java: void writeChar(int v)
    pub fn writeChar(&mut self, _v: i32) {
      // Java: this.write(v >>> 0 & 255);
      // Java: this.write(v >>> 8 & 255);
    }

    // Java: void writeInt(int v)
    pub fn writeInt(&mut self, _v: i32) {
      // Java: this.write(v >>> 0 & 255);
      // Java: this.write(v >>> 8 & 255);
      // Java: this.write(v >>> 16 & 255);
      // Java: this.write(v >>> 24 & 255);
    }

    // Java: void writeLong(long v)
    pub fn writeLong(&mut self, _v: i64) {
      // Java: this.write((int)(v >>> 0) & 255);
      // Java: this.write((int)(v >>> 8) & 255);
      // Java: this.write((int)(v >>> 16) & 255);
      // Java: this.write((int)(v >>> 24) & 255);
      // Java: this.write((int)(v >>> 32) & 255);
      // Java: this.write((int)(v >>> 40) & 255);
      // Java: this.write((int)(v >>> 48) & 255);
      // Java: this.write((int)(v >>> 56) & 255);
    }

    // Java: void writeFloat(float v)
    pub fn writeFloat(&mut self, _v: f32) {
      // Java: this.writeInt(Float.floatToIntBits(v));
    }

    // Java: void writeDouble(double v)
    pub fn writeDouble(&mut self, _v: f64) {
      // Java: this.writeLong(Double.doubleToLongBits(v));
    }

    // Java: void writeBytes(String s)
    pub fn writeBytes_str(&mut self, _s: &str) {
      // Java: byte[] b = s.getBytes();
      // Java: this.writeBytes(b, 0, b.length);
    }

    // Java: void writeChars(String s)
    pub fn writeChars(&mut self, _s: &str) {
      // Java: int clen = s.length();
      // Java: int blen = 2 * clen;
      // Java: byte[] b = new byte[blen];
      // Java: char[] c = new char[clen];
      // Java: s.getChars(0, clen, c, 0);
      // Java: int i = 0;
      // Java: for(int j = 0; i < clen; ++i)
      {
         // Java: b[j++] = (byte)(c[i] >>> 0);
         // Java: b[j++] = (byte)(c[i] >>> 8);
      }
      // Java: this.writeBytes(b, 0, blen);
    }

    // Java: void writeUTF(String str)
    pub fn writeUTF(&mut self, _str: &str) {
      // Java: writeUTF(str, this);
    }

    // Java: static String readUTF(DataInput in)
    pub fn readUTF_from_input(_input: &mut dyn std::any::Any) -> String {
      // Java: int utflen = in.readUnsignedShort();
      // Java: byte[] bytearr = null;
      // Java: char[] chararr = null;
      // Java: bytearr = new byte[utflen];
      // Java: chararr = new char[utflen];
      // Java: int count = 0;
      // Java: int chararr_count = 0;
      // Java: in.readFully(bytearr, 0, utflen);
      // Java: while(count < utflen)
      {
         // Java: int c = bytearr[count] & 255;
         // Java: if (c > 127)
         {
            // Java: break;
         }
         // Java: ++count;
         // Java: chararr[chararr_count++] = (char)c;
      }
      // Java: while(count < utflen)
      {
         // Java: int c = bytearr[count] & 255;
         // Java: switch (c >> 4)
         {
            // Java: case 0:
            // Java: case 1:
            // Java: case 2:
            // Java: case 3:
            // Java: case 4:
            // Java: case 5:
            // Java: case 6:
            // Java: case 7:
               // Java: ++count;
               // Java: chararr[chararr_count++] = (char)c;
               // Java: break;
            // Java: case 8:
            // Java: case 9:
            // Java: case 10:
            // Java: case 11:
            // Java: default:
               // Java: throw new UTFDataFormatException("malformed input around byte " + count);
            // Java: case 12:
            // Java: case 13:
               // Java: count += 2;
               // Java: if (count > utflen)
               {
                  // Java: throw new UTFDataFormatException("malformed input: partial character at end");
               }
               // Java: int char2 = bytearr[count - 1];
               // Java: if ((char2 & 192) != 128)
               {
                  // Java: throw new UTFDataFormatException("malformed input around byte " + count);
               }
               // Java: chararr[chararr_count++] = (char)((c & 31) << 6 | char2 & 63);
               // Java: break;
            // Java: case 14:
               // Java: count += 3;
               // Java: if (count > utflen)
               {
                  // Java: throw new UTFDataFormatException("malformed input: partial character at end");
               }
               // Java: char2 = bytearr[count - 2];
               // Java: int char3 = bytearr[count - 1];
               // Java: if ((char2 & 192) != 128 || (char3 & 192) != 128)
               {
                  // Java: throw new UTFDataFormatException("malformed input around byte " + (count - 1));
               }
               // Java: chararr[chararr_count++] = (char)((c & 15) << 12 | (char2 & 63) << 6 | (char3 & 63) << 0);
         }
      }
      // Java: return new String(chararr, 0, chararr_count);
      todo!()
    }

    // Java: static int writeUTF(String str, DataOutput out)
    pub fn writeUTF_to_output(_str: &str, _out: &mut dyn std::any::Any) -> i32 {
      // Java: int strlen = str.length();
      // Java: int utflen = 0;
      // Java: int count = 0;
      // Java: for(int i = 0; i < strlen; ++i)
      {
         // Java: int c = str.charAt(i);
         // Java: if (c >= 1 && c <= 127)
         {
            // Java: ++utflen;
         }
         // Java: else if (c > 2047)
         {
            // Java: utflen += 3;
         }
         // Java: else
         {
            // Java: utflen += 2;
         }
      }
      // Java: if (utflen > 65535)
      {
         // Java: throw new UTFDataFormatException("encoded string too long: " + utflen + " bytes");
      }
      // Java: else
      {
         // Java: byte[] bytearr = null;
         // Java: bytearr = new byte[utflen + 2];
         // Java: bytearr[count++] = (byte)(utflen >>> 0 & 255);
         // Java: bytearr[count++] = (byte)(utflen >>> 8 & 255);
         // Java: int i = 0;
         // Java: for(i = 0; i < strlen; ++i)
         {
            // Java: int c = str.charAt(i);
            // Java: if (c < 1 || c > 127)
            {
               // Java: break;
            }
            // Java: bytearr[count++] = (byte)c;
         }
         // Java: for(; i < strlen; ++i)
         {
            // Java: int c = str.charAt(i);
            // Java: if (c >= 1 && c <= 127)
            {
               // Java: bytearr[count++] = (byte)c;
            }
            // Java: else if (c > 2047)
            {
               // Java: bytearr[count++] = (byte)(224 | c >> 12 & 15);
               // Java: bytearr[count++] = (byte)(128 | c >> 6 & 63);
               // Java: bytearr[count++] = (byte)(128 | c >> 0 & 63);
            }
            // Java: else
            {
               // Java: bytearr[count++] = (byte)(192 | c >> 6 & 31);
               // Java: bytearr[count++] = (byte)(128 | c >> 0 & 63);
            }
         }
         // Java: out.write(bytearr, 0, utflen + 2);
         // Java: return utflen + 2;
      }
      todo!()
    }

}

impl super::OLEConstants::OLEConstants for KOLEStream {
}

impl super::IOLEStream::IOLEStream for KOLEStream {
    fn seek(&mut self, _var1: i32) { todo!() }
    fn tell(&self) -> i32 { self.mPosition }
    fn setSize(&mut self, _var1: i32) { todo!() }
    fn stat(&self, _var1: &dyn std::any::Any) {}
    fn read(&mut self, _var1: &mut [u8], _var2: i32, _var3: i32) -> i32 { todo!() }
}
