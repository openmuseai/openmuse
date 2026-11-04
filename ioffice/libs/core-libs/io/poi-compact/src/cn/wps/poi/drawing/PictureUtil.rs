// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.util.FileDataStorage
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.moffice.util.MD4Util
// Java: import cn.wps.moffice.util.StreamUtil
// Java: import cn.wps.moffice.util.zlib.ZOutputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import org.apache.poi.ddf.BlipType
// Java: import org.apache.poi.hwpf.model.HexUtils

use std::io::{self, Read, Write};
use std::any::Any;

pub struct FileDataStorage {
    name: String,
    size: i32,
    user_data: std::collections::HashMap<String, Box<dyn Any>>,
    data: Vec<u8>,
}

// Java field: int DIB_HEADER_SIZE
pub const DIB_HEADER_SIZE: i32 = 14;
// Java field: int WMF_HEADER_SIZE
pub const WMF_HEADER_SIZE: i32 = 22;
// Java field: int WMF_TAIL_SIZE
pub const WMF_TAIL_SIZE: i32 = 0;

// Java: public class PictureUtil
pub struct PictureUtil {
}

// Java: public class PictureUtil
impl PictureUtil {
    // Java: PictureUtil()
    pub fn new() -> Self {
        // Java: super();
        PictureUtil {}
    }

    // Java: static byte[] getMD4(FileDataStorage pPictureData)
    pub fn getMD4_storage(_pPictureData: &mut FileDataStorage) -> io::Result<Vec<u8>> {
        // Java: Object pUserData = pPictureData.getUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_DATA_MD4");
        // Java: if (pUserData != null)
        {
            // Java: return (byte[])pUserData;
        }
        // Java: else
        {
            // Java: pUserData = pPictureData.getUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_DATA");
            // Java: if (pUserData != null)
            {
                // Java: FileDataStorage pRawData = (FileDataStorage)pUserData;
                // Java: pUserData = pRawData.getUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_DATA_MD4");
                // Java: if (pUserData != null)
                {
                    // Java: return (byte[])pUserData;
                }
            }
            // Java: String szName = pPictureData.getName();
            // Java: byte blipType = BlipType.getBlipType(szName.substring(szName.lastIndexOf(".") + 1));
            // Java: byte[] md4 = null;
            // Java: InputStream reader = pPictureData.getReader();
            // Java: switch (blipType)
            {
                // Java: case 2:
                // Java: md4 = MD4Util.getMD4(reader);
                // Java: break;
                // Java: case 3:
                // Java: md4 = MD4Util.getMD4(reader, pPictureData.getSize() - 0);
                // Java: break;
                // Java: case 4:
                // Java: default:
                // Java: md4 = MD4Util.getMD4(reader);
                // Java: break;
                // Java: case 5:
                // Java: case 6:
                // Java: md4 = MD4Util.getMD4(reader);
                // Java: break;
                // Java: case 7:
                // Java: reader.skip(14L);
                // Java: md4 = MD4Util.getMD4(reader);
            }
            // Java: reader.close();
            // Java: pPictureData.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_DATA_MD4", md4);
            // Java: return md4;
            Ok(Vec::new())
        }
    }

    // Java: static byte[] getMD4(String path)
    pub fn getMD4_path(_path: &str) -> io::Result<Vec<u8>> {
        // Java: FileDataStorage pPictureData = KPictureRegistry.instance().register(path);
        // Java: return getMD4(pPictureData);
        Ok(Vec::new())
    }

    // Java: static void serialize(FileDataStorage pCorePicture, OutputStream output, boolean bCloseOutput)
    pub fn serialize_with_close(_pCorePicture: &mut FileDataStorage, _output: &mut dyn Write, _bCloseOutput: bool) -> io::Result<()> {
        // Java: String szName = pCorePicture.getName();
        // Java: byte blipType = BlipType.getBlipType(szName.substring(szName.lastIndexOf(".") + 1));
        // Java: InputStream reader = pCorePicture.getReader();
        // Java: switch (blipType)
        {
            // Java: case 2:
            // Java: output = new ZOutputStream(output, 1);
            // Java: StreamUtil.copyStream(reader, output);
            // Java: break;
            // Java: case 3:
            // Java: reader.skip(22L);
            // Java: output = new ZOutputStream(output, 1);
            // Java: StreamUtil.copyStream(reader, pCorePicture.getSize() - 22 - 0, output);
            // Java: break;
            // Java: case 4:
            // Java: default:
            // Java: StreamUtil.copyStream(reader, output);
            // Java: break;
            // Java: case 5:
            // Java: case 6:
            // Java: StreamUtil.copyStream(reader, output);
            // Java: break;
            // Java: case 7:
            // Java: reader.skip(14L);
            // Java: StreamUtil.copyStream(reader, output);
        }
        // Java: reader.close();
        // Java: if (bCloseOutput)
        if _bCloseOutput {
            // Java: FileUtil.closeQuietly(output);
        }
        Ok(())
    }

    // Java: static void serialize(FileDataStorage pCorePicture, OutputStream output)
    pub fn serialize(pCorePicture: &mut FileDataStorage, output: &mut dyn Write) -> io::Result<()> {
        // Java: serialize(pCorePicture, output, false);
        Self::serialize_with_close(pCorePicture, output, false)
    }

    // Java: static FileDataStorage compressPicture(FileDataStorage pPictureData)
    pub fn compressPicture(_pPictureData: &mut FileDataStorage) -> io::Result<FileDataStorage> {
        // Java: Object pUser = pPictureData.getUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_COMPRESSED");
        // Java: if (pUser != null)
        {
            // Java: return (FileDataStorage)pUser;
        }
        // Java: else
        {
            // Java: File file = Platform.createTempFile("pict-", ".tmp");
            // Java: OutputStream output = new FileOutputStream(file);
            // Java: serialize(pPictureData, output, true);
            // Java: FileDataStorage pCompressed = new FileDataStorage(file);
            // Java: pCompressed.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_COMPRESSION_METHOD", 0);
            // Java: pCompressed.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_CORE_PICTURE", pPictureData);
            // Java: pPictureData.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_COMPRESSED", pCompressed);
            // Java: return pCompressed;
            Ok(FileDataStorage {
                name: String::new(),
                size: 0,
                user_data: std::collections::HashMap::new(),
                data: Vec::new(),
            })
        }
    }

    // Java: static int getCompressionMethod(FileDataStorage pCompressed)
    pub fn getCompressionMethod(_pCompressed: &FileDataStorage) -> i32 {
        // Java: Object pUser = pCompressed.getUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_COMPRESSION_METHOD");
        // Java: return pUser == null ? 0 : (Integer)pUser;
        0
    }

    // Java: static String generateName(byte[] md4)
    pub fn generateName(md4: &[u8]) -> String {
        // Java: int seed = 131;
        let seed: i32 = 131;
        // Java: int hash = 0;
        let mut hash: i32 = 0;
        // Java: int n = md4.length;
        let n = md4.len();
        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n {
            // Java: hash = hash * seed + md4[i];
            hash = hash.wrapping_mul(seed).wrapping_add(md4[i] as i8 as i32);
        }
        // Java: hash &= Integer.MAX_VALUE;
        hash &= i32::MAX;
        // Java: char[] charArray = new char[10];
        let mut char_array = vec!['0'; 10];
        // Java: charArray[0] = '0';
        char_array[0] = '0';
        // Java: charArray[1] = 'x';
        char_array[1] = 'x';
        // Java: HexUtils.int2CharArray(charArray, 2, hash, false);
        let hex = format!("{:08X}", hash);
        for (i, c) in hex.chars().enumerate() {
            if 2 + i < char_array.len() {
                char_array[2 + i] = c;
            }
        }
        // Java: return new String(charArray);
        char_array.into_iter().collect()
    }
}
