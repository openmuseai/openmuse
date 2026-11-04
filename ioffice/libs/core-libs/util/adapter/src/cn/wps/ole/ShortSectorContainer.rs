// Java: import java.io.IOException

use super::KOLEObjectBase::KOLEObjectBase;
use super::ShortSector::ShortSector;
use super::SectorChain::SectorChain;

// Java: public class ShortSectorContainer extends KOLEObjectBase
pub struct ShortSectorContainer {
    // Java field: SAT mSAT
    mSAT: Option<Box<dyn std::any::Any>>,
    // Java field: SSAT mSSAT
    mSSAT: Option<Box<dyn std::any::Any>>,
    // Java field: BufferManager mBufferManager
    mBufferManager: Option<Box<dyn std::any::Any>>,
    // Java field: SectorChain mChain
    mChain: Option<SectorChain>,
    // Java field: ShortSector[] mShortSectorMap
    mShortSectorMap: Option<Vec<Option<ShortSector>>>,
    pub(crate) base: KOLEObjectBase,
}

// Java: public class ShortSectorContainer extends KOLEObjectBase
impl ShortSectorContainer {
    // Java: ShortSector getSector(int index)
    pub fn getSector(&mut self, _index: i32) -> Option<&ShortSector> {
      // Java: int nDesiredCapacity = index + 1 << this.SHORT_SECTOR_SHIFT;

      // Java: if (this.getCapacity() < nDesiredCapacity)

      {
         // Java: this.ensureCapacity(nDesiredCapacity);

      }
      // Java: if (this.mShortSectorMap == null || this.mShortSectorMap.length <= index)

      {
         // Java: this.resizeShortSectorMap(index + 1);

      }
      // Java: ShortSector pShortSector = this.mShortSectorMap[index];

      // Java: if (pShortSector == null)

      {
         // Java: int position = index << this.SHORT_SECTOR_SHIFT;

         // Java: int iNormalSector = this.mChain.get(position >> this.SECTOR_SHIFT);

         // Java: int iNormalOffset = position & this.SECTOR_SIZE - 1;

         // Java: pShortSector = this.mShortSectorMap[index] = new ShortSector(index, iNormalSector, iNormalOffset);

      }
      // Java: return pShortSector;

      todo!()
    }

    // Java: void resizeShortSectorMap(int nLeastSize)
    pub fn resizeShortSectorMap(&mut self, _nLeastSize: i32) {
      // Java: int nLength = 0;

      // Java: if (this.mShortSectorMap != null)

      {
         // Java: nLength = this.mShortSectorMap.length;

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
         // Java: ShortSector[] pNew = new ShortSector[nNewLength];

         // Java: if (this.mShortSectorMap != null)

         {
            // Java: System.arraycopy(this.mShortSectorMap, 0, pNew, 0, this.mShortSectorMap.length);

         }
         // Java: this.mShortSectorMap = pNew;

      }
    }

    // Java: int getCapacity()
    pub fn getCapacity(&self) -> i32 {
      // Java: return this.mChain.size() << this.SECTOR_SHIFT;

      todo!()
    }

    // Java: void ensureCapacity(int nbytes)
    pub fn ensureCapacity(&mut self, _nbytes: i32) {
      // Java: int nSectorShift = this.SECTOR_SHIFT;

      // Java: int nDesiredSector = nbytes + this.SECTOR_SIZE - 1 >> nSectorShift;

      // Java: SectorChain chain;

      // Java: int iNewSector;

      // Java: for(chain = this.mChain; chain.size() < nDesiredSector; chain.push_back(iNewSector))

      {
         // Java: iNewSector = this.mSAT.allocSector();

         // Java: if (chain.empty())

         {
            // Java: this.mDocument.setShortSectorContainerStartSector(iNewSector);

         }
         // Java: else

         {
            // Java: this.mSAT.setEntry(chain.back(), iNewSector);

         }
      }
      // Java: iNewSector = chain.size() << nSectorShift;

      // Java: if (iNewSector > this.mDocument.getShortSectorContainerSize())

      {
         // Java: this.mDocument.setShortSectorContainerSize(iNewSector);

      }
    }

}
