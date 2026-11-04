use super::SMModel::SMModel;

// Java: public class CodingStateMachine
pub struct CodingStateMachine {
    // Java field: SMModel model
    pub(crate) model: SMModel,
    // Java field: int currentState
    pub(crate) current_state: i32,
    // Java field: int currentCharLen
    pub(crate) current_char_len: i32,
    // Java field: int currentBytePos
    pub(crate) current_byte_pos: i32,
}

// Java: public class CodingStateMachine
impl CodingStateMachine {
    // Java: CodingStateMachine(SMModel model)
    pub fn new(model: SMModel) -> Self {
        // Java: super();
        // Java: this.model = model;
        // Java: this.currentState = 0;
        Self {
            model,
            current_state: 0,
            current_char_len: 0,
            current_byte_pos: 0,
        }
    }

    // Java: int nextState(byte c)
    pub fn next_state(&mut self, c: u8) -> i32 {
        // Java: int byteCls = this.model.getClass(c);
        let byte_cls = self.model.get_class(c);
        // Java: if (this.currentState == 0)
        if self.current_state == 0 {
            // Java: this.currentBytePos = 0;
            self.current_byte_pos = 0;
            // Java: this.currentCharLen = this.model.getCharLen(byteCls);
            self.current_char_len = self.model.get_char_len(byte_cls);
        }
        // Java: this.currentState = this.model.getNextState(byteCls, this.currentState);
        self.current_state = self.model.get_next_state(byte_cls, self.current_state);
        // Java: ++this.currentBytePos;
        self.current_byte_pos += 1;
        // Java: return this.currentState;
        self.current_state
    }

    // Java: int getCurrentCharLen()
    pub fn get_current_char_len(&self) -> i32 {
        // Java: return this.currentCharLen;
        self.current_char_len
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.currentState = 0;
        self.current_state = 0;
    }

    // Java: String getCodingStateMachine()
    pub fn get_coding_state_machine(&self) -> &str {
        // Java: return this.model.getName();
        self.model.get_name()
    }
}
