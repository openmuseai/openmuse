#![allow(non_snake_case)]

// Java: import java.io.Externalizable
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput
use std::io::{Read, Write, Result};

pub trait Cloneable {}
pub trait Externalizable {}

// Java field: String TAG
const TAG: &str = "";

// Java: public class Arrow implements Cloneable, Externalizable
#[derive(Clone)]
pub struct Arrow {
    // Java field: int head
    pub head: i32,
    // Java field: int width
    pub width: i32,
    // Java field: int len
    pub len: i32,
}

impl Cloneable for Arrow {}
impl Externalizable for Arrow {}

// Java: public class Arrow implements Cloneable, Externalizable
impl Arrow {
    // Java: Arrow()
    pub fn new() -> Self {
        // Java: super();
        Arrow {
            head: 0,
            width: 1,
            len: 1,
        }
    }

    // Java: Arrow(int head, int width, int len)
    pub fn with_params(head: i32, width: i32, len: i32) -> Self {
        // Java: super();
        let mut instance = Arrow { head: 0, width: 1, len: 1 };
        instance._init_params(head, width, len);
        instance
    }

    fn _init_params(&mut self, head: i32, width: i32, len: i32) {
        // Java: this.setHead(head);
        self.setHead(head);
        // Java: this.setWidth(width);
        self.setWidth(width);
        // Java: this.setLength(len);
        self.setLength(len);
    }

    // Java: Arrow clone()
    pub fn clone_arrow(&self) -> Option<Arrow> {
        // Java: try
        let result: std::result::Result<Arrow, String> = (|| -> std::result::Result<Arrow, String> {
            // Java: return (Arrow)super.clone();
            Ok(self.clone())
        })();
        match result {
            Ok(cloned) => Some(cloned),
            // Java: catch (CloneNotSupportedException e)
            Err(e) => {
                // Java: e.printStackTrace();
                eprintln!("{}", e);
                // Java: return null;
                None
            }
        }
    }

    // Java: int getHead()
    pub fn getHead(&self) -> i32 {
        // Java: return this.head;
        self.head
    }

    // Java: void setHead(int head)
    pub fn setHead(&mut self, head: i32) {
        // Java: this.head = head;
        self.head = head;
    }

    // Java: int getWidth()
    pub fn getWidth(&self) -> i32 {
        // Java: return this.width;
        self.width
    }

    // Java: void setWidth(int width)
    pub fn setWidth(&mut self, width: i32) {
        // Java: this.width = width;
        self.width = width;
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
        // Java: return this.len;
        self.len
    }

    // Java: void setLength(int len)
    pub fn setLength(&mut self, len: i32) {
        // Java: this.len = len;
        self.len = len;
    }

    // Java: float getLineWidth(float strokeWidth)
    pub fn getLineWidth(&self, strokeWidth: f32) -> f32 {
        // Java: return strokeWidth * this.getScaleNum(this.width);
        strokeWidth * self.getScaleNum(self.width)
    }

    // Java: float getLineLength(float strokeWidth)
    pub fn getLineLength(&self, strokeWidth: f32) -> f32 {
        // Java: return strokeWidth * this.getScaleNum(this.len);
        strokeWidth * self.getScaleNum(self.len)
    }

    // Java: float getScaleNum(int type)
    fn getScaleNum(&self, r#type: i32) -> f32 {
        // Java: float scale = 1.0F;
        let scale;
        // Java: switch (type)
        'switch_type: {
            match r#type {
                // Java: case 0:
                0 => {
                    // Java: scale = 2.0F;
                    scale = 2.0;
                    // Java: break;
                    break 'switch_type;
                }
                // Java: case 1:
                1 => {
                    // Java: scale = 2.8F;
                    scale = 2.8;
                    // Java: break;
                    break 'switch_type;
                }
                // Java: case 2:
                2 => {
                    // Java: scale = 4.8F;
                    scale = 4.8;
                }
                _ => {
                    scale = 1.0;
                }
            }
        }
        // Java: return scale;
        scale
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn Read) -> Result<()> {
        let mut buf = [0u8; 4];
        // Java: this.head = input.readInt();
        input.read_exact(&mut buf)?;
        self.head = i32::from_be_bytes(buf);
        // Java: this.width = input.readInt();
        input.read_exact(&mut buf)?;
        self.width = i32::from_be_bytes(buf);
        // Java: this.len = input.readInt();
        input.read_exact(&mut buf)?;
        self.len = i32::from_be_bytes(buf);
        Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn Write) -> Result<()> {
        // Java: output.writeInt(this.head);
        output.write_all(&self.head.to_be_bytes())?;
        // Java: output.writeInt(this.width);
        output.write_all(&self.width.to_be_bytes())?;
        // Java: output.writeInt(this.len);
        output.write_all(&self.len.to_be_bytes())?;
        Ok(())
    }

    // Java: boolean needSpecialHanding()
    pub fn needSpecialHanding(&self) -> bool {
        // Java: return this.width == 0 && this.len == 2;
        self.width == 0 && self.len == 2
    }
}
