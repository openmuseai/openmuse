// Java: import cn.wps.moffice.util.LittleEndian
// Java: import java.io.IOException
// Java: import java.util.ArrayList

use super::KOLEObjectBase::KOLEObjectBase;
use super::BufferManager::BufferManager;
use super::Sector::Sector;

// Java: public class MSAT extends KOLEObjectBase implements OLEConstants
pub struct MSAT {
    // Java field: BufferManager mBufferManager
    mBufferManager: Option<Box<dyn std::any::Any>>,
    // Java field: ArrayList<Sector> mMSATSectors
    mMSATSectors: Vec<Sector>,
    pub(crate) base: KOLEObjectBase,
}

// Java: public class MSAT extends KOLEObjectBase implements OLEConstants
impl MSAT {
    // Java: void doOpen()
    pub fn doOpen(&mut self) {
      // Java: int nSectIdPerSector = this.mDocument.getSectorSize() / 4;

      // Java: Sector pSector;

      // Java: for(int sector = this.mDocument.getMSATStartSector(); sector != -2; sector = LittleEndian.getIntByIndex(pSector.content, nSectIdPerSector - 1))

      {
         // Java: pSector = this.mBufferManager.getSector(sector, true);

         // Java: this.mMSATSectors.add(pSector);

         // Java: this.mBufferManager.pinSector(pSector);

      }
      // Java: if (this.mMSATSectors.size() != this.mDocument.getMSATSectorCount())

      {
         // Java: throw new OLEException("File is damaged");

      }
    }

    // Java: int getSAT(int index)
    pub fn getSAT(&self, _index: i32) -> i32 {
      // Java: if (index < 109)

      {
         // Java: return this.mDocument.getHeaderMSAT(index);

      }
      // Java: else

      {
         // Java: index -= 109;

         // Java: int nSAT_Per_MSATSector = this.SECTID_COUNT - 1;

         // Java: Sector pSector = (Sector)this.mMSATSectors.get(index / nSAT_Per_MSATSector);

         // Java: return LittleEndian.getIntByIndex(pSector.content, index % nSAT_Per_MSATSector);

      }
      todo!()
    }

    // Java: void setSAT(int index, int sector)
    pub fn setSAT(&mut self, _index: i32, _sector: i32) {
      // Java: if (index < 109)

      {
         // Java: this.mDocument.setHeaderMSAT(index, sector);

         // Java: this.mDocument.setHeaderDirty(true);

      }
      // Java: else

      {
         // Java: index -= 109;

         // Java: int nSAT_Per_MSATSector = this.SECTID_COUNT - 1;

         // Java: Sector pSector = (Sector)this.mMSATSectors.get(index / nSAT_Per_MSATSector);

         // Java: pSector.setDirty();

         // Java: LittleEndian.putIntByIndex(pSector.content, index % nSAT_Per_MSATSector, sector);

      }
    }

    // Java: int getCapacity()
    pub fn getCapacity(&self) -> i32 {
      // Java: int nSectIdPerSector = this.mDocument.getSectorSize() / 4;

      // Java: return 109 + this.mMSATSectors.size() * (nSectIdPerSector - 1);

      todo!()
    }

    // Java: Sector addMSATSector(Sector pNewSector)
    pub fn addMSATSector(&mut self, _pNewSector: Sector) -> Sector {
      // Java: int nSectIdPerSector = this.mDocument.getSectorSize() >> 2;

      // Java: if (this.mMSATSectors.isEmpty())

      {
         // Java: this.mDocument.setMSATStartSector(pNewSector.index);

      }
      // Java: else

      {
         // Java: Sector pPrevSector = (Sector)this.mMSATSectors.get(this.mMSATSectors.size() - 1);

         // Java: LittleEndian.putIntByIndex(pPrevSector.content, nSectIdPerSector - 1, pNewSector.index);

         // Java: pPrevSector.setDirty();

      }
      // Java: LittleEndian.memset_int(pNewSector.content, 0, nSectIdPerSector, -1);

      // Java: LittleEndian.putIntByIndex(pNewSector.content, nSectIdPerSector - 1, -2);

      // Java: pNewSector.setDirty();

      // Java: this.mMSATSectors.add(pNewSector);

      // Java: this.mBufferManager.pinSector(pNewSector);

      // Java: this.mDocument.setMSATSectorCount(this.mMSATSectors.size());

      // Java: return pNewSector;

      todo!()
    }

}

impl super::OLEConstants::OLEConstants for MSAT {
}
