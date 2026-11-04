#![allow(non_snake_case)]

// Java: import java.io.Externalizable
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput
use std::io::{Read, Write, Result};

// Java: public class GeoFormula implements Externalizable
pub struct GeoFormula {
    // Java field: int name
    pub name: i32,
    // Java field: int[] arguments
    pub arguments: Option<Vec<i32>>,
}

pub trait Externalizable {}
impl Externalizable for GeoFormula {}

// Java: public class GeoFormula implements Externalizable
impl GeoFormula {
    // Java: GeoFormula()
    pub fn new() -> Self {
        // Java: super();
        GeoFormula {
            name: 0,
            arguments: None,
        }
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn Read) -> Result<()> {
        let mut buf4 = [0u8; 4];
        // Java: this.name = input.readInt();
        input.read_exact(&mut buf4)?;
        self.name = i32::from_be_bytes(buf4);
        // Java: int size = input.readInt();
        input.read_exact(&mut buf4)?;
        let size = i32::from_be_bytes(buf4);
        // Java: if (size > 0)
        if size > 0 {
            // Java: this.arguments = new int[size];
            self.arguments = Some(vec![0i32; size as usize]);
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size as usize {
                // Java: this.arguments[i] = input.readInt();
                input.read_exact(&mut buf4)?;
                self.arguments.as_mut().unwrap()[i] = i32::from_be_bytes(buf4);
            }
        }
        Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn Write) -> Result<()> {
        // Java: output.writeInt(this.name);
        output.write_all(&self.name.to_be_bytes())?;
        // Java: int size = this.arguments == null ? 0 : this.arguments.length;
        let size = match &self.arguments {
            Some(args) => args.len() as i32,
            None => 0,
        };
        // Java: output.writeInt(size);
        output.write_all(&size.to_be_bytes())?;
        // Java: for(int i = 0; i < size; ++i)
        if let Some(args) = &self.arguments {
            for i in 0..size as usize {
                // Java: output.writeInt(this.arguments[i]);
                output.write_all(&args[i].to_be_bytes())?;
            }
        }
        Ok(())
    }
}
