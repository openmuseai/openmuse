// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import org.apache.poi.util.LittleEndian

use super::ClassID::ClassID;
use super::Property::Property;

// Java: public class TypeWriter
pub struct TypeWriter;

// Java: public class TypeWriter
impl TypeWriter {
    // Java: TypeWriter()
    pub fn new() -> Self {
        // Java: super();
        TypeWriter
    }

    // Java: int writeToStream(OutputStream out, short n)
    pub fn writeToStream_short(out: &mut dyn std::io::Write, n: i16) -> std::io::Result<i32> {
        // Java: int length = 2;
        // Java: byte[] buffer = new byte[2];
        // Java: LittleEndian.putShort(buffer, 0, n);
        let buffer = n.to_le_bytes();
        // Java: out.write(buffer, 0, 2);
        out.write_all(&buffer)?;
        // Java: return 2;
        Ok(2)
    }

    // Java: int writeToStream(OutputStream out, int n)
    pub fn writeToStream_int(out: &mut dyn std::io::Write, n: i32) -> std::io::Result<i32> {
        // Java: int l = 4;
        // Java: byte[] buffer = new byte[4];
        // Java: LittleEndian.putInt(buffer, 0, n);
        let buffer = n.to_le_bytes();
        // Java: out.write(buffer, 0, 4);
        out.write_all(&buffer)?;
        // Java: return 4;
        Ok(4)
    }

    // Java: int writeToStream(OutputStream out, long n)
    pub fn writeToStream_long(out: &mut dyn std::io::Write, n: i64) -> std::io::Result<i32> {
        // Java: int l = 8;
        // Java: byte[] buffer = new byte[8];
        // Java: LittleEndian.putLong(buffer, 0, n);
        let buffer = n.to_le_bytes();
        // Java: out.write(buffer, 0, 8);
        out.write_all(&buffer)?;
        // Java: return 8;
        Ok(8)
    }

    // Java: void writeUShortToStream(OutputStream out, int n)
    pub fn writeUShortToStream(out: &mut dyn std::io::Write, n: i32) -> std::io::Result<()> {
        // Java: int high = n & -65536;
        let high = n & !0xFFFF;
        // Java: if (high != 0)
        if high != 0
        {
            // Java: throw new IllegalPropertySetDataException("Value " + n + " cannot be represented by 2 bytes.");
            panic!("Value {} cannot be represented by 2 bytes.", n);
        }
        // Java: else
        else
        {
            // Java: writeToStream(out, (short)n);
            Self::writeToStream_short(out, n as i16)?;
        }
        Ok(())
    }

    // Java: int writeUIntToStream(OutputStream out, long n)
    pub fn writeUIntToStream(out: &mut dyn std::io::Write, n: i64) -> std::io::Result<i32> {
        // Java: long high = n & -4294967296L;
        let high = n & !0xFFFFFFFF_i64;
        // Java: if (high != 0L && high != -4294967296L)
        if high != 0 && high != -4294967296i64
        {
            // Java: throw new IllegalPropertySetDataException("Value " + n + " cannot be represented by 4 bytes.");
            panic!("Value {} cannot be represented by 4 bytes.", n);
        }
        // Java: else
        else
        {
            // Java: return writeToStream(out, (int)n);
            return Self::writeToStream_int(out, n as i32);
        }
    }

    // Java: int writeToStream(OutputStream out, ClassID n)
    pub fn writeToStream_classid(out: &mut dyn std::io::Write, n: &ClassID) -> std::io::Result<i32> {
        // Java: byte[] b = new byte[16];
        let mut b = vec![0u8; 16];
        // Java: n.write(b, 0);
        n.write(&mut b, 0);
        // Java: out.write(b, 0, b.length);
        out.write_all(&b)?;
        // Java: return b.length;
        Ok(b.len() as i32)
    }

    // Java: void writeToStream(OutputStream out, Property[] properties, int codepage)
    pub fn writeToStream_properties(out: &mut dyn std::io::Write, properties: Option<&[Property]>, codepage: i32) -> std::io::Result<()> {
        // Java: if (properties != null)
        if let Some(props) = properties
        {
            // Java: for(int i = 0; i < properties.length; ++i)
            for i in 0..props.len()
            {
                // Java: Property p = properties[i];
                let p = &props[i];
                // Java: writeUIntToStream(out, p.getID());
                Self::writeUIntToStream(out, p.getID())?;
                // Java: writeUIntToStream(out, (long)p.getSize());
                Self::writeUIntToStream(out, p.getSize() as i64)?;
            }

            // Java: for(int i = 0; i < properties.length; ++i)
            for i in 0..props.len()
            {
                // Java: Property p = properties[i];
                let p = &props[i];
                // Java: long type = p.getType();
                let type_ = p.getType();
                // Java: writeUIntToStream(out, type);
                Self::writeUIntToStream(out, type_)?;
                // Java: VariantSupport.write(out, (long)((int)type), p.getValue(), codepage);
            }
        }
        Ok(())
    }

    // Java: int writeToStream(OutputStream out, double n)
    pub fn writeToStream_double(out: &mut dyn std::io::Write, n: f64) -> std::io::Result<i32> {
        // Java: int l = 8;
        // Java: byte[] buffer = new byte[8];
        // Java: LittleEndian.putDouble(buffer, 0, n);
        let buffer = n.to_le_bytes();
        // Java: out.write(buffer, 0, 8);
        out.write_all(&buffer)?;
        // Java: return 8;
        Ok(8)
    }
}
