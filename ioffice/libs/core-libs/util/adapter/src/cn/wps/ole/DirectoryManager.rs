// Java: import java.io.IOException

use super::KOLEObjectBase::KOLEObjectBase;
use super::KDirEntry::KDirEntry;
use super::SectorChain::SectorChain;

// Java: public class DirectoryManager extends KOLEObjectBase implements OLEConstants
pub struct DirectoryManager {
    // Java field: BufferManager mBufferManager
    mBufferManager: Option<Box<dyn std::any::Any>>,
    // Java field: SAT mSAT
    mSAT: Option<Box<dyn std::any::Any>>,
    // Java field: KDirEntry[] mEntries
    pub(crate) mEntries: Option<Vec<Option<KDirEntry>>>,
    pub(crate) base: KOLEObjectBase,
}

// Java: public class DirectoryManager extends KOLEObjectBase implements OLEConstants
impl DirectoryManager {
    // Java: DirectoryManager(KOLEDocument document)
    pub fn new(_document: Box<dyn std::any::Any>) -> Self {
      // Java: super();

      // Java: this.initBase(document);

      // Java: this.mBufferManager = document.getBufferManager();

      // Java: this.mSAT = document.getSAT();

      // Java: this.mEntries = null;

      todo!()
    }

    // Java: long getFileEntry(int index)
    fn getFileEntry(&self, _index: i32) -> i64 {
      // Java: int nSectorShift = this.SECTOR_SHIFT;

      // Java: int offset = index * 128;

      // Java: int sector_index = offset >> nSectorShift;

      // Java: int sector_offset = offset & (1 << nSectorShift) - 1;

      // Java: SectorChain chain = new SectorChain();

      // Java: this.mDocument.getSAT().getSectorChain(this.mDocument.getDirectoryStartSector(), chain);

      // Java: int iSector = chain.get(sector_index);

      // Java: return ((long)iSector << 32) + (long)sector_offset;

      todo!()
    }

    // Java: KDirEntry getEntry(int index)
    pub fn getEntry(&mut self, _index: i32) -> Option<&KDirEntry> {
      // Java: if (index < 0)

      {
         // Java: if (index == -1)

         {
            // Java: return null;

         }
         // Java: else

         {
            // Java: throw new OLEException("Invalid directory index: " + index);

         }
      }
      // Java: else if (this.mEntries != null && this.mEntries.length > index && this.mEntries[index] != null)

      {
         // Java: return this.mEntries[index];

      }
      // Java: else

      {
         // Java: SectorChain chain = new SectorChain();

         // Java: this.mDocument.getSAT().getSectorChain(this.mDocument.getDirectoryStartSector(), chain);

         // Java: if (index + 1 << 7 > chain.size() << this.SECTOR_SHIFT)

         {
            // Java: throw new OLEException("Invalid directory index: " + index);

         }
         // Java: else

         {
            // Java: if (this.mEntries == null || this.mEntries.length <= index)

            {
               // Java: this.resizeEntryMap(index + 1);

            }
            // Java: if (this.mEntries[index] == null)

            {
               // Java: long nFileEntry = this.getFileEntry(index);

               // Java: int iSector = (int)(nFileEntry >> 32);

               // Java: int offset = (int)(nFileEntry & 4294967295L);

               // Java: Sector pSector = this.mBufferManager.getSector(iSector, true);

               // Java: this.mBufferManager.pinSector(pSector);

               // Java: KDirEntry pEntry = new KDirEntry(this, index, pSector, offset);

               // Java: pEntry.onOpen();

               // Java: this.mEntries[index] = pEntry;

            }
            // Java: return this.mEntries[index];

         }
      }
      todo!()
    }

    // Java: void resizeEntryMap(int nLeastSize)
    fn resizeEntryMap(&mut self, _nLeastSize: i32) {
      // Java: int nLength = 0;

      // Java: if (this.mEntries != null)

      {
         // Java: nLength = this.mEntries.length;

      }
      // Java: if (nLength < nLeastSize)

      {
         // Java: int nNewLength = nLength + (nLength >> 1);

         // Java: if (nNewLength < nLeastSize)

         {
            // Java: nNewLength = nLeastSize;

         }
         // Java: if (nNewLength < 32)

         {
            // Java: nNewLength = 32;

         }
         // Java: KDirEntry[] pNew = new KDirEntry[nNewLength];

         // Java: for(int i = 0; i < nNewLength; ++i)

         {
            // Java: pNew[i] = null;

         }
         // Java: if (this.mEntries != null)

         {
            // Java: System.arraycopy(this.mEntries, 0, pNew, 0, this.mEntries.length);

         }
         // Java: this.mEntries = pNew;

      }
    }

    // Java: KDirEntry allocEntry()
    pub fn allocEntry(&mut self) -> Option<KDirEntry> {
      // Java: int nEntryPerSector = this.SECTOR_SIZE / 128;

      // Java: SectorChain chain = new SectorChain();

      // Java: this.mDocument.getSAT().getSectorChain(this.mDocument.getDirectoryStartSector(), chain);

      // Java: for(int i = 0; i < chain.size(); ++i)

      {
         // Java: Sector pSector = this.mBufferManager.getSector(chain.get(i), true);

         // Java: for(int iEntry = 0; iEntry < nEntryPerSector; ++iEntry)

         {
            // Java: int entry_offset = iEntry << 7;

            // Java: int nType = FileOLEEntry.getType(pSector.content, entry_offset);

            // Java: if (nType == 0)

            {
               // Java: KDirEntry pEntry = new KDirEntry(this, i * nEntryPerSector + iEntry, pSector, entry_offset);

               // Java: pEntry.onNew();

               // Java: return pEntry;

            }
         }
         // Java: pSector.unlock();

      }
      // Java: int iSector = this.mSAT.allocSector();

      // Java: Sector pSector = this.mBufferManager.getSector(iSector, false);

      // Java: for(int i = 0; i < nEntryPerSector; ++i)

      {
         // Java: FileOLEEntry.setType(pSector.content, i * 128, 0);

         // Java: FileOLEEntry.setName(pSector.content, i * 128, "XX");

      }
      // Java: if (chain.empty())

      {
         // Java: this.mDocument.setDirectoryStartSector(iSector);

      }
      // Java: else

      {
         // Java: this.mSAT.setEntry(chain.back(), iSector);

      }
      // Java: chain.push_back(iSector);

      // Java: this.mSAT.setEntry(iSector, -2);

      // Java: KDirEntry pEntry = new KDirEntry(this, (chain.size() - 1) * nEntryPerSector, pSector, 0);

      // Java: pEntry.onNew();

      // Java: return pEntry;

      todo!()
    }

}

impl super::OLEConstants::OLEConstants for DirectoryManager {
}
