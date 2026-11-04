#![allow(
    non_snake_case,
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_assignments,
    unused_parens
)]
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use crate::kfs::mfs::RandomAccessFile::RandomAccessFile;
use crate::kfs::zip::ZipEntry::ZipEntry;
// Rust-native: IOException/ZipException 谱系错误的 Java 名字还原
use crate::kfs::zip::ZipErrors;

// Java field:    public static final long MAX_ZIP_ENTRY_AND_ARCHIVE_SIZE = 4294967295L;
pub const MAX_ZIP_ENTRY_AND_ARCHIVE_SIZE: i64 = 4294967295;
// Java field:    private static final short ZIP64_EXTENDED_INFO_HEADER_ID = 1;
const ZIP64_EXTENDED_INFO_HEADER_ID: i16 = 1;
// Java field:    private static final int ZIP64_LOCATOR_SIZE = 20;
const ZIP64_LOCATOR_SIZE: i32 = 20;
// Java field:    private static final int ZIP64_LOCATOR_SIGNATURE = 117853008;
const ZIP64_LOCATOR_SIGNATURE: i32 = 117853008;
// Java field:    private static final int ZIP64_EOCD_RECORD_SIGNATURE = 101075792;
const ZIP64_EOCD_RECORD_SIGNATURE: i32 = 101075792;
// Java field:    private static final int ZIP64_EOCD_RECORD_EFFECTIVE_SIZE = 40;
const ZIP64_EOCD_RECORD_EFFECTIVE_SIZE: i32 = 40;

// Java: public class Zip64
pub struct Zip64 {}

// Java: public class Zip64
impl Zip64 {
    // Java: public static long parseZip64EocdRecordLocator(RandomAccessFile raf, long eocdOffset) throws IOException
    pub fn parseZip64EocdRecordLocator(raf: &mut RandomAccessFile, eocdOffset: i64) -> i64 {
        // Java: if (eocdOffset > 20L)
        if eocdOffset > 20 {
            // Java: raf.seek(eocdOffset - 20L);
            raf.seek(eocdOffset - 20);
            // Java: if (Integer.reverseBytes(raf.readInt()) == 117853008)
            if raf.readInt().swap_bytes() == ZIP64_LOCATOR_SIGNATURE {
                // Java: byte[] zip64EocdLocator = new byte[16];
                let mut zip64EocdLocator = vec![0u8; 16];
                // Java: raf.readFully(zip64EocdLocator);
                raf.readFully(&mut zip64EocdLocator);
                // Java: int diskWithCentralDir = buf.getInt();
                let diskWithCentralDir = i32::from_le_bytes([
                    zip64EocdLocator[0],
                    zip64EocdLocator[1],
                    zip64EocdLocator[2],
                    zip64EocdLocator[3],
                ]);
                // Java: long zip64EocdRecordOffset = buf.getLong();
                let zip64EocdRecordOffset = i64::from_le_bytes([
                    zip64EocdLocator[4],
                    zip64EocdLocator[5],
                    zip64EocdLocator[6],
                    zip64EocdLocator[7],
                    zip64EocdLocator[8],
                    zip64EocdLocator[9],
                    zip64EocdLocator[10],
                    zip64EocdLocator[11],
                ]);
                // Java: int numDisks = buf.getInt();
                let numDisks = i32::from_le_bytes([
                    zip64EocdLocator[12],
                    zip64EocdLocator[13],
                    zip64EocdLocator[14],
                    zip64EocdLocator[15],
                ]);
                // Java: if (numDisks == 1 && diskWithCentralDir == 0)
                if numDisks == 1 && diskWithCentralDir == 0 {
                    // Java: return zip64EocdRecordOffset;
                    return zip64EocdRecordOffset;
                }
                // Java: throw new ZipException("Spanned archives not supported");
                panic!("ZipException: Spanned archives not supported");
            }
        }
        // Java: return -1L;
        -1
    }

    // Rust: try_ 核心 —— Java 的 `throws ZipException` 变成 Err
    pub fn try_parseZip64ExtendedInfo(
        ze: &mut ZipEntry,
        fromCentralDirectory: bool,
    ) -> std::io::Result<bool> {
        // Java: if (ze.extra != null && ze.extra.length > 0)
        if let Some(ref extra) = ze.extra {
            if extra.len() > 4 {
                let mut pos: usize = 0;
                while pos + 4 <= extra.len() {
                    let header_id = u16::from_le_bytes([extra[pos], extra[pos + 1]]);
                    let length = u16::from_le_bytes([extra[pos + 2], extra[pos + 3]]) as usize;
                    if header_id == 1 {
                        let data_start = pos + 4;
                        if data_start + length <= extra.len() {
                            let mut off = data_start;
                            if fromCentralDirectory || ze.getMethod() == 0 {
                                if ze.size == 4294967295 && off + 8 <= data_start + length {
                                    ze.size = i64::from_le_bytes([
                                        extra[off],
                                        extra[off + 1],
                                        extra[off + 2],
                                        extra[off + 3],
                                        extra[off + 4],
                                        extra[off + 5],
                                        extra[off + 6],
                                        extra[off + 7],
                                    ]);
                                    off += 8;
                                }
                                if ze.compressedSize == 4294967295 && off + 8 <= data_start + length
                                {
                                    ze.compressedSize = i64::from_le_bytes([
                                        extra[off],
                                        extra[off + 1],
                                        extra[off + 2],
                                        extra[off + 3],
                                        extra[off + 4],
                                        extra[off + 5],
                                        extra[off + 6],
                                        extra[off + 7],
                                    ]);
                                    off += 8;
                                }
                            }
                            if fromCentralDirectory
                                && ze.localHeaderRelOffset == 4294967295
                                && off + 8 <= data_start + length
                            {
                                ze.localHeaderRelOffset = i64::from_le_bytes([
                                    extra[off],
                                    extra[off + 1],
                                    extra[off + 2],
                                    extra[off + 3],
                                    extra[off + 4],
                                    extra[off + 5],
                                    extra[off + 6],
                                    extra[off + 7],
                                ]);
                            }

                            let ext_total = length + 4;
                            let extras_len = extra.len() - ext_total;
                            let mut new_extra = vec![0u8; extras_len];
                            new_extra[..pos].copy_from_slice(&extra[..pos]);
                            if pos + ext_total < extra.len() {
                                new_extra[pos..].copy_from_slice(&extra[pos + ext_total..]);
                            }
                            ze.extra = Some(new_extra);
                            return Ok(true);
                        }
                        return Ok(false);
                    }
                    pos += 4 + length;
                }
            }
        }

        if ze.compressedSize != 4294967295
            && ze.size != 4294967295
            && ze.localHeaderRelOffset != 4294967295
        {
            return Ok(false);
        }
        // Java: throw new ZipException("File contains no zip64 extended information: name=" + ze.name + ...)
        Err(ZipErrors::zipException(format!(
            "File contains no zip64 extended information: name={}",
            ze.getName()
        )))
    }

    // Java: public static boolean parseZip64ExtendedInfo(ZipEntry ze, boolean fromCentralDirectory) throws ZipException
    pub fn parseZip64ExtendedInfo(ze: &mut ZipEntry, fromCentralDirectory: bool) -> bool {
        Self::try_parseZip64ExtendedInfo(ze, fromCentralDirectory)
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e))
    }

    // Rust: try_ 核心 —— Java 的 `throws ZipException`（extras 放不下）变成 Err
    pub fn try_insertZip64ExtendedInfoToExtras(ze: &mut ZipEntry) -> std::io::Result<()> {
        // Java: int extendedInfoSize = 28;
        let extended_info_size: usize = 28;
        let mut output: Vec<u8>;
        // Java: if (ze.extra == null)
        if ze.extra.is_none() {
            output = vec![0u8; extended_info_size];
        } else {
            let extra = ze.extra.as_ref().unwrap();
            // Java: if (ze.extra.length + 28 > 65535) throw new ZipException("No space in extras for zip64 extended entry info");
            if extra.len() + extended_info_size > 65535 {
                return Err(ZipErrors::zipException(
                    "No space in extras for zip64 extended entry info",
                ));
            }
            output = vec![0u8; extra.len() + extended_info_size];
            output[extended_info_size..].copy_from_slice(extra);
        }

        let mut off: usize = 0;
        output[off..off + 2].copy_from_slice(&1i16.to_le_bytes());
        off += 2;
        output[off..off + 2].copy_from_slice(&24i16.to_le_bytes());
        off += 2;

        if ze.getMethod() == 0 {
            output[off..off + 8].copy_from_slice(&ze.size.to_le_bytes());
            off += 8;
            output[off..off + 8].copy_from_slice(&ze.compressedSize.to_le_bytes());
            off += 8;
        } else {
            output[off..off + 8].copy_from_slice(&0i64.to_le_bytes());
            off += 8;
            output[off..off + 8].copy_from_slice(&0i64.to_le_bytes());
            off += 8;
        }
        output[off..off + 8].copy_from_slice(&ze.localHeaderRelOffset.to_le_bytes());

        // Java: ze.extra = output;
        ze.extra = Some(output);
        Ok(())
    }

    // Java: public static void insertZip64ExtendedInfoToExtras(ZipEntry ze) throws ZipException
    pub fn insertZip64ExtendedInfoToExtras(ze: &mut ZipEntry) {
        Self::try_insertZip64ExtendedInfoToExtras(ze)
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Java: private static int getZip64ExtendedInfoSize(ByteBuffer extras)
    fn getZip64ExtendedInfoSize(extras: &[u8]) -> i32 {
        // Java: while (extras.hasRemaining())
        let mut pos: usize = 0;
        while pos + 4 <= extras.len() {
            // Java: int headerId = extras.getShort() & '\uffff';
            let header_id = u16::from_le_bytes([extras[pos], extras[pos + 1]]);
            // Java: int length = extras.getShort() & '\uffff';
            let length = u16::from_le_bytes([extras[pos + 2], extras[pos + 3]]) as usize;
            if header_id == 1 {
                // Java: if (extras.remaining() >= length) return length;  return -1;
                if extras.len() - (pos + 4) >= length {
                    return length as i32;
                }
                return -1;
            }
            // Java: extras.position(extras.position() + length);  ← 越界会抛 IllegalArgumentException 被 catch 成 -1
            pos += 4 + length;
        }
        // Java: catch (BufferUnderflowException | IllegalArgumentException) { return -1; }
        -1
    }

    // Java: public static void refreshZip64ExtendedInfo(ZipEntry ze)
    // Rust: Java 这里抛的是 IllegalStateException（RuntimeException 谱系），
    //       按两层约定属于“编程错误”，保持 panic，不降级成 Result。
    pub fn refreshZip64ExtendedInfo(ze: &mut ZipEntry) {
        // Java: if (ze.extra == null) throw new IllegalStateException("Zip64 entry has no available extras: " + ze);
        if ze.extra.is_none() {
            panic!(
                "IllegalStateException: Zip64 entry has no available extras: {}",
                ze.getName()
            );
        }

        // Java: ByteBuffer buf = ByteBuffer.wrap(ze.extra).order(ByteOrder.LITTLE_ENDIAN);
        //       int extendedInfoSize = getZip64ExtendedInfoSize(buf);
        let extended_info_size = Self::getZip64ExtendedInfoSize(ze.extra.as_ref().unwrap());
        // Java: if (extendedInfoSize == -1) throw new IllegalStateException("Zip64 entry extras has no zip64 extended info record: " + ze);
        if extended_info_size == -1 {
            panic!(
                "IllegalStateException: Zip64 entry extras has no zip64 extended info record: {}",
                ze.getName()
            );
        }

        // Rust: 先把要写的值取出来，避免同时可变借用 ze.extra 与读取 ze.size
        let size = ze.size;
        let compressed_size = ze.compressedSize;
        let local_header_rel_offset = ze.localHeaderRelOffset;

        let extra = ze.extra.as_mut().unwrap();
        let mut pos: usize = 0;
        while pos + 4 <= extra.len() {
            let header_id = u16::from_le_bytes([extra[pos], extra[pos + 1]]);
            let length = u16::from_le_bytes([extra[pos + 2], extra[pos + 3]]) as usize;
            if header_id == 1 {
                let data_start = pos + 4;
                // Java: catch (BufferOverflowException boe) → throw new IllegalStateException("Invalid extended info extra", boe);
                if data_start + 24 > extra.len() {
                    panic!("IllegalStateException: Invalid extended info extra");
                }
                // Java: buf.putLong(ze.size); buf.putLong(ze.compressedSize); buf.putLong(ze.localHeaderRelOffset);
                extra[data_start..data_start + 8].copy_from_slice(&size.to_le_bytes());
                extra[data_start + 8..data_start + 16].copy_from_slice(&compressed_size.to_le_bytes());
                extra[data_start + 16..data_start + 24]
                    .copy_from_slice(&local_header_rel_offset.to_le_bytes());
                return;
            }
            pos += 4 + length;
        }
    }

    // Java: public static EocdRecord parseZip64EocdRecord(RandomAccessFile raf, long offset, int commentLength) throws IOException
    pub fn parseZip64EocdRecord(
        raf: &mut RandomAccessFile,
        offset: i64,
        _commentLength: i32,
    ) -> super::ZipFile::EocdRecord {
        raf.seek(offset);
        let sig = raf.readInt().swap_bytes();
        if sig != ZIP64_EOCD_RECORD_SIGNATURE {
            panic!("ZipException: Zip64 EOCD record signature not found");
        }
        let mut buf = vec![0u8; 40];
        raf.readFully(&mut buf);

        let num_entries = i64::from_le_bytes([
            buf[24], buf[25], buf[26], buf[27], buf[28], buf[29], buf[30], buf[31],
        ]);
        let central_dir_offset = i64::from_le_bytes([
            buf[32], buf[33], buf[34], buf[35], buf[36], buf[37], buf[38], buf[39],
        ]);

        super::ZipFile::EocdRecord {
            numEntries: num_entries,
            centralDirOffset: central_dir_offset,
            commentLength: 0,
        }
    }
}
