#![allow(non_snake_case, non_upper_case_globals, dead_code)]

// Java: public class Rotation
pub struct Rotation {
    // Java field: float lat
    pub lat: f32,
    // Java field: float lon
    pub lon: f32,
    // Java field: float rev
    pub rev: f32,
}

// Java: public class Rotation
impl Rotation {
    // Java: Rotation(float lat, float lon, float rev)
    pub fn new_with_lat_lon_rev(lat: f32, lon: f32, rev: f32) -> Self {
      // Java: super();
      Rotation {
          // Java: this.lat = lat;
          lat,
          // Java: this.lon = lon;
          lon,
          // Java: this.rev = rev;
          rev,
      }
    }

    // Java: Rotation()
    pub fn new() -> Self {
      // Java: super();
      Rotation { lat: 0.0, lon: 0.0, rev: 0.0 }
    }

}
