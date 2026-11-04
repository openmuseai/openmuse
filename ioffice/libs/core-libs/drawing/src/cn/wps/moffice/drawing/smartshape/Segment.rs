#![allow(non_snake_case)]

// Java: import java.io.Externalizable
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput
use std::io::{Read, Write, Result};

pub trait Externalizable {}

#[derive(Clone)]
// Java: public class Segment implements Externalizable
pub struct Segment {
    // Java field: short cmd
    pub cmd: i16,
    // Java field: int start
    pub start: i32,
}

impl Externalizable for Segment {}

// Java: public class Segment implements Externalizable
impl Segment {
    // Java: Segment()
    pub fn new() -> Self {
        // Java: super();
        Segment { cmd: 0, start: 0 }
    }

    // Java: Segment(short cmd, int start)
    pub fn with_params(cmd: i16, start: i32) -> Self {
        let mut instance = Self { cmd: 0, start: 0 };
        instance._init(cmd, start);
        instance
    }

    fn _init(&mut self, cmd: i16, start: i32) {
        // Java: super();
        *self = Self { cmd: 0, start: 0 };
        // Java: this.cmd = cmd;
        self.cmd = cmd;
        // Java: this.start = start;
        self.start = start;
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn Read) -> Result<()> {
        let mut buf2 = [0u8; 2];
        let mut buf4 = [0u8; 4];
        // Java: this.cmd = input.readShort();
        input.read_exact(&mut buf2)?;
        self.cmd = i16::from_be_bytes(buf2);
        // Java: this.start = input.readInt();
        input.read_exact(&mut buf4)?;
        self.start = i32::from_be_bytes(buf4);
        Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn Write) -> Result<()> {
        // Java: output.writeShort(this.cmd);
        output.write_all(&self.cmd.to_be_bytes())?;
        // Java: output.writeInt(this.start);
        output.write_all(&self.start.to_be_bytes())?;
        Ok(())
    }
}
