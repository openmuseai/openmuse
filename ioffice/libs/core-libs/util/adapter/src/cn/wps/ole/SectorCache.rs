// Java: import java.util.ArrayList
// Java: import java.util.List

use super::Sector::Sector;

// Java field: int MAX_CACHE
const MAX_CACHE: i32 = 4;

// Java: public class SectorCache
pub struct SectorCache {
    // Java field: int sectorSize
    sectorSize: i32,
    // Java field: List<Sector> sectors
    sectors: Vec<Sector>,
}

// Java: public class SectorCache
impl SectorCache {
    // Java: SectorCache(int sectorSize)
    pub fn new(_sectorSize: i32) -> Self {
      // Java: super();

      // Java: this.sectorSize = sectorSize;

      SectorCache {
          sectorSize: _sectorSize,
          sectors: Vec::with_capacity(8),
      }
    }

    // Java: Sector obtain()
    pub fn obtain(&mut self) -> Sector {
      // Java: if (!this.sectors.isEmpty())

      {
         // Java: Sector sector = (Sector)this.sectors.remove(this.sectors.size() - 1);

         // Java: sector.reset();

         // Java: return sector;

      }
      // Java: else

      {
         // Java: return new Sector(this.sectorSize);

      }
      todo!()
    }

    // Java: void release(Sector sector)
    pub fn release(&mut self, _sector: Sector) {
      // Java: if (this.sectors.size() < 4)

      {
         // Java: this.sectors.add(sector);

      }
    }

}
