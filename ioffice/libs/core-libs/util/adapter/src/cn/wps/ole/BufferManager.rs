// Java: import java.io.IOException

use super::KOLEObjectBase::KOLEObjectBase;
use super::OLEConstants::*;
use super::Sector::Sector;
use super::SectorCache::SectorCache;

// Java field: boolean ENABLE_SECTOR_CACHE
const ENABLE_SECTOR_CACHE: bool = true;
// Java field: int DEFAULT_BUFFER_SIZE
const DEFAULT_BUFFER_SIZE: i32 = 262144;
// Java field: int MINIMUM_BUFFER_SIZE
const MINIMUM_BUFFER_SIZE: i32 = 32768;
// Java field: int MAXIMUM_BUFFER_SIZE
const MAXIMUM_BUFFER_SIZE: i32 = 5242880;
// Java field: int PIN_PRIORITY
const PIN_PRIORITY: i32 = 1073741824;

// Java: public class BufferManager extends KOLEObjectBase implements OLEConstants
pub struct BufferManager {
    // Java field: Sector[] mSectors
    pub(crate) mSectors: Vec<Option<Sector>>,
    // Java field: int mBufferedSectorCount
    mBufferedSectorCount: i32,
    // Java field: Sector[] mQueue
    pub(crate) mQueue: Vec<Option<Sector>>,
    // Java field: SectorCache mSectorCache
    mSectorCache: Option<SectorCache>,
    pub(crate) base: KOLEObjectBase,
}

// Java: public class BufferManager extends KOLEObjectBase implements OLEConstants
impl BufferManager {
    // Java: BufferManager(KOLEDocument document)
    pub fn new(_document: Box<dyn std::any::Any>) -> Self {
      // Java: super();

      // Java: this.initBase(document);

      // Java: this.mBufferedSectorCount = 0;

      // Java: this.setBufferSize(262144);

      // Java: this.mSectorCache = new SectorCache(this.SECTOR_SIZE);

      todo!()
    }

    // Java: void ensureMapSize(int size)
    pub fn ensureMapSize(&mut self, _size: i32) {
      // Java: if (this.mSectors.length < size)

      {
         // Java: int nNewSize = this.mSectors.length + (this.mSectors.length >> 1);

         // Java: if (nNewSize < size)

         {
            // Java: nNewSize = size + 32;

         }
         // Java: Sector[] pNew = new Sector[nNewSize];

         // Java: System.arraycopy(this.mSectors, 0, pNew, 0, this.mSectors.length);

         // Java: this.mSectors = pNew;

      }
    }

    // Java: Sector getSector(int index, boolean bReadFromDisk)
    pub fn getSector(&mut self, _index: i32, _bReadFromDisk: bool) -> Option<&Sector> {
      // Java: this.ensureMapSize(index + 1);

      // Java: Sector sector = this.mSectors[index];

      // Java: if (sector == null)

      {
         // Java: if (512 + (index << this.SECTOR_SHIFT) < this.mDocument.getCurrentFileSize())

         {
            // Java: sector = this.readSector(index);

         }
         // Java: else

         {
            // Java: sector = this.allocSector(index);

            // Java: sector.setDirty();

         }
         // Java: this.mSectors[index] = sector;

         // Java: this.addToQueue(sector);

      }
      // Java: else

      {
         // Java: this.siftUp(sector, 1);

      }
      // Java: sector.lock();

      // Java: return sector;

      todo!()
    }

    // Java: Sector getLoadedSector(int index)
    pub fn getLoadedSector(&self, _index: i32) -> Option<&Sector> {
      // Java: return this.mSectors.length > index ? this.mSectors[index] : null;

      todo!()
    }

    // Java: void pinSector(Sector pSector)
    pub fn pinSector(&mut self, _pSector: &mut Sector) {
      // Java: this.siftUp(pSector, pSector.priority >= 1073741824 ? 1 : 1073741824);

    }

    // Java: void discardSector(int index)
    pub fn discardSector(&mut self, _index: i32) {
      // Java: Sector pSector = this.mSectors[index];

      // Java: if (pSector != null)

      {
         // Java: pSector.clearDirty();

      }
    }

    // Java: void flush()
    pub fn flush(&mut self) {
      // Java: int n = this.mSectors.length;

      // Java: for(int i = 0; i < n; ++i)

      {
         // Java: Sector pSector = this.mSectors[i];

         // Java: if (pSector != null && pSector.isDirty())

         {
            // Java: this.flushSector(pSector);

         }
      }
    }

    // Java: Sector readSector(int index)
    pub fn readSector(&mut self, _index: i32) -> Sector {
      // Java: Sector pSector = this.allocSector(index);

      // Java: int nPosition = (index << this.SECTOR_SHIFT) + 512;

      // Java: int nRead = this.mDocument.readFile(pSector.content, 0, this.SECTOR_SIZE, nPosition);

      // Java: if (nRead != this.SECTOR_SIZE)

      {
         // Java: throw new OLEException("Must read a whole sector");

      }
      // Java: else

      {
         // Java: return pSector;

      }
      todo!()
    }

    // Java: void flushSector(Sector pSector)
    pub fn flushSector(&mut self, _pSector: &mut Sector) {
      // Java: if (pSector != null && pSector.isDirty())

      {
         // Java: int nPosition = (pSector.index << this.SECTOR_SHIFT) + 512;

         // Java: this.mDocument.writeFile(pSector.content, 0, this.SECTOR_SIZE, nPosition);

         // Java: pSector.clearDirty();

      }
    }

    // Java: void setBufferSize(int nBufferSize)
    pub fn setBufferSize(&mut self, _nBufferSize: i32) {
      // Java: if (nBufferSize < 32768)

      {
         // Java: nBufferSize = 32768;

      }
      // Java: else if (nBufferSize > 5242880)

      {
         // Java: nBufferSize = 5242880;

      }
      // Java: int nSectors = nBufferSize + this.SECTOR_SIZE - 1 >> this.SECTOR_SHIFT;

      // Java: Sector[] pNewQueue = new Sector[nSectors];

      // Java: if (this.mQueue != null)

      {
         // Java: System.arraycopy(this.mQueue, 0, pNewQueue, 0, this.mQueue.length);

      }
      // Java: this.mQueue = pNewQueue;

    }

    // Java: int getBufferSize()
    pub fn getBufferSize(&self) -> i32 {
      // Java: return this.mQueue == null ? 0 : this.mQueue.length << this.SECTOR_SHIFT;

      todo!()
    }

    // Java: void addToQueue(Sector pSector)
    pub fn addToQueue(&mut self, _pSector: &mut Sector) {
      // Java: int nQueueLength = this.mQueue.length;

      // Java: int iQueuePos;

      // Java: if (this.mBufferedSectorCount >= nQueueLength && !this.all_locked())

      {
         // Java: Sector pRemoved = this.findSwapCandidate();

         // Java: pRemoved.priority = 0;

         // Java: this.siftDown(pRemoved);

         // Java: if (pRemoved.isDirty())

         {
            // Java: this.flushSector(pRemoved);

         }
         // Java: this.mSectors[pRemoved.index] = null;

         // Java: iQueuePos = pRemoved.mQueueIndex;

         // Java: this.releaseSector(pRemoved);

      }
      // Java: else

      {
         // Java: iQueuePos = this.mBufferedSectorCount++;

         // Java: if (this.mBufferedSectorCount >= nQueueLength)

         {
            // Java: this.setBufferSize(this.getBufferSize() + 65536);

         }
      }
      // Java: pSector.priority = 0;

      // Java: pSector.mQueueIndex = iQueuePos;

      // Java: this.mQueue[iQueuePos] = pSector;

    }

    // Java: boolean all_locked()
    pub fn all_locked(&self) -> bool {
      // Java: int nTotal = this.mBufferedSectorCount;

      // Java: for(int i = nTotal - 1; i >= 0; --i)

      {
         // Java: Sector pSector = this.mQueue[i];

         // Java: if (!pSector.isLocked())

         {
            // Java: return false;

         }
      }
      // Java: return true;

      todo!()
    }

    // Java: Sector findSwapCandidate()
    pub fn findSwapCandidate(&self) -> Option<&Sector> {
      // Java: int nTotal = this.mBufferedSectorCount;

      // Java: for(int i = nTotal - 1; i >= 0; --i)

      {
         // Java: Sector pSector = this.mQueue[i];

         // Java: if (!pSector.isLocked())

         {
            // Java: return pSector;

         }
      }
      // Java: throw new OLEException("No buffer available!! You must increase buffer size!!!");

      todo!()
    }

    // Java: void siftUp(Sector pSector, int nAddedPriority)
    pub fn siftUp(&mut self, _pSector: &mut Sector, _nAddedPriority: i32) {
      // Java: if (pSector.mQueueIndex >= this.mQueue.length >> 1 && pSector.priority < 1073741824)

      {
         // Java: pSector.priority += nAddedPriority;

         // Java: int parent;

         // Java: for(int index = pSector.mQueueIndex; index > 0; index = parent)

         {
            // Java: parent = index - 1 >> 1;

            // Java: Sector pParent = this.mQueue[parent];

            // Java: if (pParent.priority >= pSector.priority)

            {
               // Java: break;

            }
            // Java: this.mQueue[parent] = pSector;

            // Java: this.mQueue[index] = pParent;

            // Java: pSector.mQueueIndex = parent;

            // Java: pParent.mQueueIndex = index;

         }
      }
    }

    // Java: void siftDown(Sector pSector)
    pub fn siftDown(&mut self, _pSector: &mut Sector) {
      // Java: int largest;

      // Java: for(int index = pSector.mQueueIndex; index < this.mBufferedSectorCount; index = largest)

      {
         // Java: int left = index * 2 + 1;

         // Java: int right = index * 2 + 2;

         // Java: largest = pSector.mQueueIndex;

         // Java: if (left < this.mBufferedSectorCount && this.mQueue[left].priority > pSector.priority)

         {
            // Java: largest = left;

         }
         // Java: if (right < this.mBufferedSectorCount && this.mQueue[right].priority > this.mQueue[largest].priority)

         {
            // Java: largest = right;

         }
         // Java: if (largest == index)

         {
            // Java: break;

         }
         // Java: Sector pLargest = this.mQueue[largest];

         // Java: pLargest.mQueueIndex = index;

         // Java: pSector.mQueueIndex = largest;

         // Java: this.mQueue[index] = this.mQueue[largest];

         // Java: this.mQueue[largest] = pSector;

      }
    }

    // Java: Sector allocSector(int index)
    pub fn allocSector(&mut self, _index: i32) -> Sector {
      // Java: Sector sector = this.mSectorCache.obtain();

      // Java: sector.index = index;

      // Java: return sector;

      todo!()
    }

    // Java: void releaseSector(Sector sector)
    pub fn releaseSector(&mut self, _sector: Sector) {
      // Java: this.mSectorCache.release(sector);

    }

    // Java: byte[] getBuffers(byte[] header)
    pub fn getBuffers(&self, _header: &[u8]) -> Vec<u8> {
      // Java: byte[] bytes = new byte[header.length + this.getBufferSize()];

      // Java: int n = this.mSectors.length;

      // Java: for(int index = 0; index < header.length; ++index)

      {
         // Java: bytes[index] = header[index];

      }
      // Java: int mainIndex = header.length;

      // Java: for(int i = 0; i < n; ++i)

      {
         // Java: Sector pSector = this.mSectors[i];

         // Java: if (pSector != null)

         {
            // Java: int nPosition = (pSector.index << this.SECTOR_SHIFT) + 512;

            // Java: for(int index = 0; index < pSector.content.length; ++index)

            {
               // Java: bytes[mainIndex] = pSector.content[index];

               // Java: ++mainIndex;

            }
         }
      }
      // Java: return bytes;

      todo!()
    }

}

impl super::OLEConstants::OLEConstants for BufferManager {
}
