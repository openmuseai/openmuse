use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// Java: public class SavedByEntry
pub struct SavedByEntry {
    // Java field: String userName
    pub(crate) userName: String,
    // Java field: String saveLocation
    pub(crate) saveLocation: String,
}

// Java: public class SavedByEntry
impl SavedByEntry {
    // Java: SavedByEntry(String userName, String saveLocation)
    pub fn new(userName: String, saveLocation: String) -> Self {
        // Java: super();
        // Java: this.userName = userName;
        // Java: this.saveLocation = saveLocation;
        SavedByEntry { userName, saveLocation }
    }

    // Java: String getUserName()
    pub fn getUserName(&self) -> &str {
        // Java: return this.userName;
        &self.userName
    }

    // Java: String getSaveLocation()
    pub fn getSaveLocation(&self) -> &str {
        // Java: return this.saveLocation;
        &self.saveLocation
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: Option<&SavedByEntry>) -> bool {
        // Java: if (other == this)
        if std::ptr::eq(self, other.unwrap_or(self)) && other.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else if (!(other instanceof SavedByEntry))
        else if other.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: SavedByEntry that = (SavedByEntry)other;
            let that = other.unwrap();
            // Java: return that.userName.equals(this.userName) && that.saveLocation.equals(this.saveLocation);
            that.userName == self.userName && that.saveLocation == self.saveLocation
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int hash = 29;
        let mut hash: i32 = 29;
        // Java: hash = hash * 13 + this.userName.hashCode();
        let mut hasher = DefaultHasher::new();
        self.userName.hash(&mut hasher);
        hash = hash.wrapping_mul(13).wrapping_add(hasher.finish() as i32);
        // Java: hash = hash * 13 + this.saveLocation.hashCode();
        let mut hasher2 = DefaultHasher::new();
        self.saveLocation.hash(&mut hasher2);
        hash = hash.wrapping_mul(13).wrapping_add(hasher2.finish() as i32);
        // Java: return hash;
        hash
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "SavedByEntry[userName=" + this.getUserName() + ",saveLocation=" + this.getSaveLocation() + "]";
        format!("SavedByEntry[userName={},saveLocation={}]", self.getUserName(), self.getSaveLocation())
    }
}
