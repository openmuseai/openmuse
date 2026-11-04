#![allow(non_snake_case)]

// Java: import java.io.Externalizable
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput
use std::io::{Read, Write};

// Java: public class GRF implements Cloneable, Externalizable
#[derive(Clone)]
pub struct GRF {
    // Java field: boolean fGroup
    pub(crate) fGroup: bool,
    // Java field: boolean fChild
    pub(crate) fChild: bool,
    // Java field: boolean fPatriarch
    pub(crate) fPatriarch: bool,
    // Java field: boolean fDeleted
    pub(crate) fDeleted: bool,
    // Java field: boolean fOleShape
    pub(crate) fOleShape: bool,
    // Java field: boolean fHaveMaster
    pub(crate) fHaveMaster: bool,
    // Java field: boolean fFlipH
    pub(crate) fFlipH: bool,
    // Java field: boolean fFlipV
    pub(crate) fFlipV: bool,
    // Java field: boolean fConnector
    pub(crate) fConnector: bool,
    // Java field: boolean fHaveAnchor
    pub(crate) fHaveAnchor: bool,
    // Java field: boolean fBackground
    pub(crate) fBackground: bool,
    // Java field: boolean fHaveSpt
    pub(crate) fHaveSpt: bool,
}

// Java: public class GRF implements Cloneable, Externalizable
impl GRF {
    // Java: GRF()
    pub fn new() -> Self {
        // Java: super();
        Self {
            fGroup: false,
            fChild: false,
            fPatriarch: false,
            fDeleted: false,
            fOleShape: false,
            fHaveMaster: false,
            fFlipH: false,
            fFlipV: false,
            fConnector: false,
            fHaveAnchor: true,
            fBackground: false,
            fHaveSpt: true,
        }
    }

    // Java: GRF clone()
    pub fn clone(&self) -> Result<GRF, String> {
        // Java: return (GRF)super.clone();
        Ok(GRF {
            fGroup: self.fGroup,
            fChild: self.fChild,
            fPatriarch: self.fPatriarch,
            fDeleted: self.fDeleted,
            fOleShape: self.fOleShape,
            fHaveMaster: self.fHaveMaster,
            fFlipH: self.fFlipH,
            fFlipV: self.fFlipV,
            fConnector: self.fConnector,
            fHaveAnchor: self.fHaveAnchor,
            fBackground: self.fBackground,
            fHaveSpt: self.fHaveSpt,
        })
    }

    // Java: void setGroup(boolean bGroup)
    pub fn setGroup(&mut self, bGroup: bool) {
        // Java: this.fGroup = bGroup;
        self.fGroup = bGroup;
    }

    // Java: boolean isGroup()
    pub fn isGroup(&self) -> bool {
        // Java: return this.fGroup;
        self.fGroup
    }

    // Java: void setChild(boolean bChild)
    pub fn setChild(&mut self, bChild: bool) {
        // Java: this.fChild = bChild;
        self.fChild = bChild;
    }

    // Java: boolean isChild()
    pub fn isChild(&self) -> bool {
        // Java: return this.fChild;
        self.fChild
    }

    // Java: void setPatriarch(boolean bPatriarch)
    pub fn setPatriarch(&mut self, bPatriarch: bool) {
        // Java: this.fPatriarch = bPatriarch;
        self.fPatriarch = bPatriarch;
    }

    // Java: boolean isPatriarch()
    pub fn isPatriarch(&self) -> bool {
        // Java: return this.fPatriarch;
        self.fPatriarch
    }

    // Java: void setDeleted(boolean bDeleted)
    pub fn setDeleted(&mut self, bDeleted: bool) {
        // Java: this.fDeleted = bDeleted;
        self.fDeleted = bDeleted;
    }

    // Java: boolean isDeleted()
    pub fn isDeleted(&self) -> bool {
        // Java: return this.fDeleted;
        self.fDeleted
    }

    // Java: void setOleShape(boolean bOleShape)
    pub fn setOleShape(&mut self, bOleShape: bool) {
        // Java: this.fOleShape = bOleShape;
        self.fOleShape = bOleShape;
    }

    // Java: boolean isOleShape()
    pub fn isOleShape(&self) -> bool {
        // Java: return this.fOleShape;
        self.fOleShape
    }

    // Java: void setHaveMaster(boolean bHaveMaster)
    pub fn setHaveMaster(&mut self, bHaveMaster: bool) {
        // Java: this.fHaveMaster = bHaveMaster;
        self.fHaveMaster = bHaveMaster;
    }

    // Java: boolean isHaveMaster()
    pub fn isHaveMaster(&self) -> bool {
        // Java: return this.fHaveMaster;
        self.fHaveMaster
    }

    // Java: void setFlipH(boolean bFlipH)
    pub fn setFlipH(&mut self, bFlipH: bool) {
        // Java: this.fFlipH = bFlipH;
        self.fFlipH = bFlipH;
    }

    // Java: boolean isFlipH()
    pub fn isFlipH(&self) -> bool {
        // Java: return this.fFlipH;
        self.fFlipH
    }

    // Java: void setFlipV(boolean bFlipV)
    pub fn setFlipV(&mut self, bFlipV: bool) {
        // Java: this.fFlipV = bFlipV;
        self.fFlipV = bFlipV;
    }

    // Java: boolean isFlipV()
    pub fn isFlipV(&self) -> bool {
        // Java: return this.fFlipV;
        self.fFlipV
    }

    // Java: void setConnector(boolean bConnector)
    pub fn setConnector(&mut self, bConnector: bool) {
        // Java: this.fConnector = bConnector;
        self.fConnector = bConnector;
    }

    // Java: boolean isConnector()
    pub fn isConnector(&self) -> bool {
        // Java: return this.fConnector;
        self.fConnector
    }

    // Java: void setHaveAnchor(boolean bHaveAnchor)
    pub fn setHaveAnchor(&mut self, bHaveAnchor: bool) {
        // Java: this.fHaveAnchor = bHaveAnchor;
        self.fHaveAnchor = bHaveAnchor;
    }

    // Java: boolean isHaveAnchor()
    pub fn isHaveAnchor(&self) -> bool {
        // Java: return this.fHaveAnchor;
        self.fHaveAnchor
    }

    // Java: void setBackground(boolean bBackground)
    pub fn setBackground(&mut self, bBackground: bool) {
        // Java: this.fBackground = bBackground;
        self.fBackground = bBackground;
    }

    // Java: boolean isBackground()
    pub fn isBackground(&self) -> bool {
        // Java: return this.fBackground;
        self.fBackground
    }

    // Java: void setHaveSpt(boolean bHaveSpt)
    pub fn setHaveSpt(&mut self, bHaveSpt: bool) {
        // Java: this.fHaveSpt = bHaveSpt;
        self.fHaveSpt = bHaveSpt;
    }

    // Java: boolean isHaveSpt()
    pub fn isHaveSpt(&self) -> bool {
        // Java: return this.fHaveSpt;
        self.fHaveSpt
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn Read) -> std::io::Result<()> {
        let mut buf = [0u8; 1];
        // Java: this.setGroup(input.readBoolean());
        input.read_exact(&mut buf)?; self.setGroup(buf[0] != 0);
        // Java: this.setChild(input.readBoolean());
        input.read_exact(&mut buf)?; self.setChild(buf[0] != 0);
        // Java: this.setPatriarch(input.readBoolean());
        input.read_exact(&mut buf)?; self.setPatriarch(buf[0] != 0);
        // Java: this.setDeleted(input.readBoolean());
        input.read_exact(&mut buf)?; self.setDeleted(buf[0] != 0);
        // Java: this.setOleShape(input.readBoolean());
        input.read_exact(&mut buf)?; self.setOleShape(buf[0] != 0);
        // Java: this.setHaveMaster(input.readBoolean());
        input.read_exact(&mut buf)?; self.setHaveMaster(buf[0] != 0);
        // Java: this.setFlipH(input.readBoolean());
        input.read_exact(&mut buf)?; self.setFlipH(buf[0] != 0);
        // Java: this.setFlipV(input.readBoolean());
        input.read_exact(&mut buf)?; self.setFlipV(buf[0] != 0);
        // Java: this.setConnector(input.readBoolean());
        input.read_exact(&mut buf)?; self.setConnector(buf[0] != 0);
        // Java: this.setHaveAnchor(input.readBoolean());
        input.read_exact(&mut buf)?; self.setHaveAnchor(buf[0] != 0);
        // Java: this.setBackground(input.readBoolean());
        input.read_exact(&mut buf)?; self.setBackground(buf[0] != 0);
        // Java: this.setHaveSpt(input.readBoolean());
        input.read_exact(&mut buf)?; self.setHaveSpt(buf[0] != 0);
        Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn Write) -> std::io::Result<()> {
        // Java: output.writeBoolean(this.isGroup());
        output.write_all(&[self.fGroup as u8])?;
        // Java: output.writeBoolean(this.isChild());
        output.write_all(&[self.fChild as u8])?;
        // Java: output.writeBoolean(this.isPatriarch());
        output.write_all(&[self.fPatriarch as u8])?;
        // Java: output.writeBoolean(this.isDeleted());
        output.write_all(&[self.fDeleted as u8])?;
        // Java: output.writeBoolean(this.isOleShape());
        output.write_all(&[self.fOleShape as u8])?;
        // Java: output.writeBoolean(this.isHaveMaster());
        output.write_all(&[self.fHaveMaster as u8])?;
        // Java: output.writeBoolean(this.isFlipH());
        output.write_all(&[self.fFlipH as u8])?;
        // Java: output.writeBoolean(this.isFlipV());
        output.write_all(&[self.fFlipV as u8])?;
        // Java: output.writeBoolean(this.isConnector());
        output.write_all(&[self.fConnector as u8])?;
        // Java: output.writeBoolean(this.isHaveAnchor());
        output.write_all(&[self.fHaveAnchor as u8])?;
        // Java: output.writeBoolean(this.isBackground());
        output.write_all(&[self.fBackground as u8])?;
        // Java: output.writeBoolean(this.isHaveSpt());
        output.write_all(&[self.fHaveSpt as u8])?;
        Ok(())
    }
}

pub trait Cloneable {}
impl Cloneable for GRF {}

pub trait Externalizable {}
impl Externalizable for GRF {}
