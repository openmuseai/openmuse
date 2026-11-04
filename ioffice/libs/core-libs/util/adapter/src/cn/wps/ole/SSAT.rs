// Java: import cn.wps.moffice.util.LittleEndian
// Java: import java.io.IOException

use super::KOLEObjectBase::KOLEObjectBase;
use super::SectorChain::SectorChain;

// Java: public class SSAT extends KOLEObjectBase implements OLEConstants
pub struct SSAT {
    // Java field: BufferManager mBufferManager
    mBufferManager: Option<Box<dyn std::any::Any>>,
    // Java field: SAT mSAT
    mSAT: Option<Box<dyn std::any::Any>>,
    pub(crate) base: KOLEObjectBase,
}

// Java: public class SSAT extends KOLEObjectBase implements OLEConstants
impl SSAT {
    // Java: int getEntry(int iShortSectorIndex)
    pub fn getEntry(&self, _iShortSectorIndex: i32) -> i32 {
      // Java: SectorChain chain = new SectorChain();

      // Java: this.mSAT.getSectorChain(this.mDocument.getSSATStartSector(), chain);

      // Java: int nPerSector = this.mDocument.getSectorSize() / 4;

      // Java: int iSector = iShortSectorIndex / nPerSector;

      // Java: int offset = iShortSectorIndex % nPerSector;

      // Java: Sector pSector = this.mBufferManager.getSector(chain.get(iSector), true);

      // Java: pSector.unlock();

      // Java: return LittleEndian.getIntByIndex(pSector.content, offset);

      todo!()
    }

    // Java: void setEntry(int iShortSectorIndex, int value)
    pub fn setEntry(&mut self, _iShortSectorIndex: i32, _value: i32) {
      // Java: SectorChain chain = new SectorChain();

      // Java: this.mSAT.getSectorChain(this.mDocument.getSSATStartSector(), chain);

      // Java: int nPerSector = this.mDocument.getSectorSize() / 4;

      // Java: int iSector = iShortSectorIndex / nPerSector;

      // Java: int offset = iShortSectorIndex % nPerSector;

      // Java: Sector pSector = this.mBufferManager.getSector(chain.get(iSector), true);

      // Java: LittleEndian.putIntByIndex(pSector.content, offset, value);

      // Java: pSector.unlock();

      // Java: pSector.setDirty();

    }

    // Java: void getSectorChain(int start, SectorChain chain)
    pub fn getSectorChain(&self, _start: i32, _chain: &mut SectorChain) {
      // Java: while(start != -2)

      {
         // Java: chain.push_back(start);

         // Java: start = this.getEntry(start);

      }
    }

    // Java: int allocSector()
    pub fn allocSector(&mut self) -> i32 {
      // Java: int nSectIdPerSector = this.mDocument.getSectorSize() / 4;

      // Java: SectorChain chain = new SectorChain();

      // Java: this.mSAT.getSectorChain(this.mDocument.getSSATStartSector(), chain);

      // Java: for(int iSectorIndex = 0; iSectorIndex < chain.size(); ++iSectorIndex)

      {
         // Java: int iSector = chain.get(iSectorIndex);

         // Java: Sector pSector = this.mBufferManager.getSector(iSector, true);

         // Java: for(int i = 0; i < nSectIdPerSector; ++i)

         {
            // Java: if (LittleEndian.getIntByIndex(pSector.content, i) == -1)

            {
               // Java: LittleEndian.putIntByIndex(pSector.content, i, -2);

               // Java: pSector.setDirty();

               // Java: pSector.unlock();

               // Java: return iSectorIndex * nSectIdPerSector + i;

            }
         }
         // Java: pSector.unlock();

      }
      // Java: int iNewSector = this.mSAT.allocSector();

      // Java: if (chain.empty())

      {
         // Java: this.mDocument.setSSATStartSector(iNewSector);

      }
      // Java: else

      {
         // Java: this.mSAT.setEntry(chain.back(), iNewSector);

      }
      // Java: chain.push_back(iNewSector);

      // Java: this.mDocument.setSSATSectorCount(chain.size());

      // Java: Sector pSector = this.mBufferManager.getSector(iNewSector, false);

      // Java: LittleEndian.memset_int(pSector.content, 0, nSectIdPerSector, -1);

      // Java: LittleEndian.putIntByIndex(pSector.content, 0, -2);

      // Java: pSector.unlock();

      // Java: return (chain.size() - 1) * nSectIdPerSector;

      todo!()
    }

    // Java: void freeSector(int index)
    pub fn freeSector(&mut self, _index: i32) {
      // Java: this.setEntry(index, -1);

    }

}

impl super::OLEConstants::OLEConstants for SSAT {
}
