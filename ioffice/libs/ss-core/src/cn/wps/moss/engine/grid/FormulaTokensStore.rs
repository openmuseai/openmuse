// java final class FormulaTokensStore
pub struct FormulaTokensStore {
    // java private final ArrayList<byte[]> tokensLst = new ArrayList();
    tokensLst: Vec<Vec<u8>>,
}

// java final class FormulaTokensStore
impl FormulaTokensStore {
    pub fn FormulaTokensStore() -> Self {
        FormulaTokensStore {
            tokensLst: Vec::new(),
        }
    }

    // java public byte[] get(int id)
    pub fn get(&self, id: i32) -> Vec<u8> {
        // java return id == 0 ? Formula.EMPTY_TOKENS : (byte[])this.tokensLst.get(id - 1);
        if id == 0 {
            Vec::new()
        } else {
            self.tokensLst[(id - 1) as usize].clone()
        }
    }

    // java public int add(byte[] tokens)
    pub fn add(&mut self, tokens: Option<&[u8]>) -> i32 {
        // java if (tokens != null && tokens.length != 0)
        if tokens.is_some() && !tokens.unwrap().is_empty() {
            // java this.tokensLst.add(tokens);
            self.tokensLst.push(tokens.unwrap().to_vec());
            // java return this.tokensLst.size();
            self.tokensLst.len() as i32
        } else {
            // java return 0;
            0
        }
    }
}
