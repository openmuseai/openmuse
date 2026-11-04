// Java: import cn.wps.moffice.util.LittleEndian
// Java: import java.io.IOException

use super::KOLEObjectBase::KOLEObjectBase;
use super::Sector::Sector;
use super::SectorChain::SectorChain;

// Java: public class SAT extends KOLEObjectBase implements OLEConstants
pub struct SAT {
    // Java field: BufferManager mBufferManager
    mBufferManager: Option<Box<dyn std::any::Any>>,
    // Java field: MSAT mMSAT
    mMSAT: Option<Box<dyn std::any::Any>>,
    // Java field: FreeSATSector mFreeSATSector
    mFreeSATSector: Option<FreeSATSector>,
    pub(crate) base: KOLEObjectBase,
}

// Java: public class FreeSATSector
pub struct FreeSATSector {
    // Java field: Sector sector
    pub(crate) sector: Option<Sector>,
    // Java field: int iSAT
    pub(crate) iSAT: i32,
    // Java field: int offset
    pub(crate) offset: i32,
}

// Java: public class SAT extends KOLEObjectBase implements OLEConstants
impl SAT {
    // Java: SAT(KOLEDocument document)
    pub fn new(_document: Box<dyn std::any::Any>) -> Self {
      // Java: super();

      // Java: this.initBase(document);

      // Java: this.mBufferManager = document.getBufferManager();

      // Java: this.mMSAT = document.getMSAT();

      todo!()
    }

    // Java: int getEntry(int sectorIndex)
    pub fn getEntry(&self, _sectorIndex: i32) -> i32 {
      // Java: int iSAT = this.SAT_INDEX(sectorIndex);

      // Java: int nSATSector = this.mMSAT.getSAT(iSAT);

      // Java: int offset = this.SAT_OFFSET(sectorIndex);

      // Java: Sector pSector = this.mBufferManager.getSector(nSATSector, true);

      // Java: int iEntry = LittleEndian.getIntByIndex(pSector.content, offset);

      // Java: pSector.unlock();

      // Java: return iEntry;

      todo!()
    }

    // Java: void setEntry(int sectorIndex, int value)
    pub fn setEntry(&mut self, _sectorIndex: i32, _value: i32) {
      // Java: int iSAT = this.SAT_INDEX(sectorIndex);

      // Java: int nSATSector = this.mMSAT.getSAT(iSAT);

      // Java: int offset = this.SAT_OFFSET(sectorIndex);

      // Java: Sector pSector = this.mBufferManager.getSector(nSATSector, true);

      // Java: LittleEndian.putIntByIndex(pSector.content, offset, value);

      // Java: pSector.setDirty();

      // Java: pSector.unlock();

    }

    // Java: void getSectorChain(int start, SectorChain chain)
    pub fn getSectorChain(&self, _start: i32, _chain: &mut SectorChain) {
      // Java: int orgStart = start;

      // Java: while(start != -2)

      {
         // Java: chain.push_back(start);

         // Java: start = this.getEntry(start);

         // Java: if (start == orgStart)

         {
            // Java: throw new OLEException("File is diagramed");

         }
      }
    }

    // Java: int allocSector()
    pub fn allocSector(&mut self) -> i32 {
      // Java: int nSectIdPerSector = this.SECTID_COUNT;

      // Java: while(this.mFreeSATSector.sector != null)

      {
         // Java: while(this.mFreeSATSector.offset < nSectIdPerSector)

         {
            // Java: if (LittleEndian.getIntByIndex(this.mFreeSATSector.sector.content, this.mFreeSATSector.offset) == -1)

            {
               // Java: int iFreeSAT = (this.mFreeSATSector.iSAT << this.SECTID_COUNT_SHIFT) + this.mFreeSATSector.offset;

               // Java: LittleEndian.putIntByIndex(this.mFreeSATSector.sector.content, this.mFreeSATSector.offset, -2);

               // Java: this.mFreeSATSector.sector.setDirty();

               // Java: ++this.mFreeSATSector.offset;

               // Java: return iFreeSAT;

            }
            // Java: ++this.mFreeSATSector.offset;

         }
         // Java: if (this.mFreeSATSector.sector != null)

         {
            // Java: this.mFreeSATSector.sector.unlock();

         }
         // Java: this.mFreeSATSector.sector = null;

         // Java: int iSAT = this.mFreeSATSector.iSAT + 1;

         // Java: int nSAT = this.mMSAT.getCapacity();

         // Java: if (iSAT < nSAT)

         {
            // Java: int iSATSector = this.mMSAT.getSAT(iSAT);

            // Java: if (iSATSector != -1)

            {
               // Java: this.mFreeSATSector.iSAT = iSAT;

               // Java: this.mFreeSATSector.sector = this.mBufferManager.getSector(iSATSector, true);

               // Java: this.mFreeSATSector.offset = 0;

            }
         }
      }
      // Java: int nMSATCap = this.mMSAT.getCapacity();

      // Java: int nTotalSATSector = this.mDocument.getSATSectorCount();

      // Java: if (nTotalSATSector == nMSATCap)

      {
         // Java: int iNewMSATSector = nTotalSATSector * nSectIdPerSector;

         // Java: int iNewSATSector = iNewMSATSector + 1;

         // Java: Sector pMSATSector = this.mBufferManager.getSector(iNewMSATSector, false);

         // Java: this.mBufferManager.pinSector(pMSATSector);

         // Java: Sector pSATSector = this.mBufferManager.getSector(iNewSATSector, false);

         // Java: this.mMSAT.addMSATSector(pMSATSector);

         // Java: LittleEndian.putInt(pMSATSector.content, 0, iNewSATSector);

         // Java: byte[] content_p = pSATSector.content;

         // Java: LittleEndian.memset_int(content_p, 0, nSectIdPerSector, -1);

         // Java: LittleEndian.putIntByIndex(content_p, 0, -4);

         // Java: LittleEndian.putIntByIndex(content_p, 1, -3);

         // Java: ++nTotalSATSector;

         // Java: this.mDocument.setSATSectorCount(nTotalSATSector);

         // Java: this.mFreeSATSector.markFree(this.mMSAT.getCapacity() - (this.SECTID_COUNT - 1), pSATSector, 3);

         // Java: LittleEndian.putIntByIndex(content_p, 2, -2);

         // Java: return iNewSATSector + 1;

      }
      // Java: else

      {
         // Java: int iSAT;

         // Java: for(iSAT = 0; iSAT < nMSATCap; ++iSAT)

         {
            // Java: int iSATSector = this.mMSAT.getSAT(iSAT);

            // Java: if (iSATSector == -1)

            {
               // Java: break;

            }
         }
         // Java: int iNewSAT = nTotalSATSector << this.SECTID_COUNT_SHIFT;

         // Java: Sector pNewSATSector = this.mBufferManager.getSector(iNewSAT, false);

         // Java: this.mBufferManager.pinSector(pNewSATSector);

         // Java: LittleEndian.memset_int(pNewSATSector.content, 0, nSectIdPerSector, -1);

         // Java: this.mMSAT.setSAT(iSAT, iNewSAT);

         // Java: ++nTotalSATSector;

         // Java: this.mDocument.setSATSectorCount(nTotalSATSector);

         // Java: this.setEntry(iNewSAT, -3);

         // Java: this.mFreeSATSector.markFree(iSAT, pNewSATSector, 2);

         // Java: this.setEntry(iNewSAT + 1, -2);

         // Java: return iNewSAT + 1;

      }
      todo!()
    }

    // Java: void freeSector(int sector)
    pub fn freeSector(&mut self, _sector: i32) {
      // Java: this.setEntry(sector, -1);

      // Java: this.mBufferManager.discardSector(sector);

    }

    // Java: void freeSectorChain(int start)
    pub fn freeSectorChain(&mut self, _start: i32) {
      // Java: while(start != -2)

      {
         // Java: int next = this.getEntry(start);

         // Java: this.freeSector(start);

         // Java: start = next;

      }
    }

}

// Java: public class FreeSATSector
impl FreeSATSector {
    // Java: void markFree(int iSAT, Sector sector, int offset)
    pub fn markFree(&mut self, _iSAT: i32, _sector: Sector, _offset: i32) {
         // Java: if (this.sector != null)

         {
            // Java: this.sector.unlock();

         }
         // Java: this.iSAT = iSAT;

         // Java: this.sector = sector;

         // Java: this.offset = offset;

    }

}

impl super::OLEConstants::OLEConstants for SAT {
}
