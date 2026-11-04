// java import org.apache.poi.util.IntMapper
use util::org::apache::poi::util::IntMapper::IntMapper;

// java final class FormulaStringStore
pub struct FormulaStringStore {
    // java private final IntMapper<String> fmlStrs = new IntMapper();
    fmlStrs: IntMapper<String>,
}

// java final class FormulaStringStore
impl FormulaStringStore {
    pub fn FormulaStringStore() -> Self {
        FormulaStringStore {
            fmlStrs: IntMapper::new(),
        }
    }

    // java public String get(int id)
    pub fn get(&self, id: i32) -> String {
        // java return (String)this.fmlStrs.get(id);
        match self.fmlStrs.get(id) {
            Some(value) => value.clone(),
            None => panic!("IndexOutOfBoundsException: {id}"),
        }
    }

    // java public int add(String value)
    pub fn add(&mut self, value: &str) -> i32 {
        // java int index = this.fmlStrs.getIndex(value);
        let mut index = self.fmlStrs.get_index(&value.to_string());
        // java if (index == -1)
        if index == -1 {
            // java index = this.fmlStrs.size();
            index = self.fmlStrs.size();
            // java this.fmlStrs.add(value);
            self.fmlStrs.add(value.to_string());
        }
        // java return index;
        index
    }
}
