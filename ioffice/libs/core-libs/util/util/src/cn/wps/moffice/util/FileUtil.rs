use std::path::{Path, PathBuf};
use std::io::{self, Read, Write};
use std::fs;

// Java field: String TAG
pub const TAG: &str = "FileUtil";
// Java field: int IO_BUFFER_SIZE
pub const IO_BUFFER_SIZE: i32 = 8192;
// Java field: int MAX_TRY_COUNT
pub const MAX_TRY_COUNT: i32 = 5;
// Java field: int MIN_WAIT_TIME
pub const MIN_WAIT_TIME: i32 = 5;
// Java field: String FILE_PREFIX
pub const FILE_PREFIX: &str = "file://";
// Java field: String tmpSuffix
pub const tmpSuffix: &str = ".tmp";
// Java field: char rightparenthesis
pub const rightparenthesis: char = ')';
// Java field: char leftparenthesis
pub const leftparenthesis: char = '(';
// Java field: int MAX_AUTO_INDEX
pub const MAX_AUTO_INDEX: i32 = 9999999;
// Java field: int FILE_NAME_MAX_LENGTH
pub const FILE_NAME_MAX_LENGTH: i32 = 80;
// Java field: int FILE_NAME_MAX_SIZE
pub const FILE_NAME_MAX_SIZE: i32 = 240;
// Java field: int BLUETOOTH_NAME_MAX_SIZE
pub const BLUETOOTH_NAME_MAX_SIZE: i32 = 90;
// Java field: String INVALID_REGEX
pub const INVALID_REGEX: &str = "[/\\:\"<>|?*\r\n\t]";
// Java field: _INVALID_NAMES: &[String]
pub static INVALID_NAMES: &[&str] = &[];
// Java field: String invalid_filename_chars
pub const invalid_filename_chars: &str = "/\\:\"<>|?*\r\n\t";

pub trait DataTransferListener {
    fn started(&self);
    fn transferred(&self, progress: i32);
    fn completed(&self, path: &str);
    fn aborted(&self);
}

// Java: public interface IDeleteCallback
pub trait IDeleteCallback {
    // Java: abstract boolean shouldDelete(java.io.File var1)
    fn shouldDelete(&self, var1: &Path) -> bool;
}

// Java: public interface IChecker
pub trait IChecker {
    // Java: abstract String exist(String var1)
    fn exist(&self, var1: &str) -> Option<String>;
}

// Java: public class FileUtil
pub struct FileUtil;

// Java: public class FileUtil
impl FileUtil {
    // Java: FileUtil()
    pub fn _method_1() {
        // Java: super();
    }

    // Java: static String getSHA1(String text)
    pub fn _method_2() {
        // Java: MessageDigest md = null;
        // Java: try
        // Java: md = MessageDigest.getInstance("SHA-1");
        // Java: md.update(text.getBytes("utf-8"));
        // Java: _sha1hash: &[u8] = md.digest();
        // Java: return byteArrayToHexString(sha1hash);
        // Java: catch (UnsupportedEncodingException var3)
        // Java: catch (NoSuchAlgorithmException var4)
        // Java: return null;
    }

    // Java: static String getSHA1(java.io.File file)
    pub fn _method_3() {
        // Java: FileInputStream fis = null;
        // Java: String sha1 = null;
        // Java: try
        // Java: MessageDigest md = MessageDigest.getInstance("SHA-1");
        // Java: fis = new FileInputStream(file);
        // Java: _buffer: &[u8] = new byte[4096];
        // Java: int length = -1;
        // Java: while((length = fis.read(buffer)) != -1)
        // Java: md.update(buffer, 0, length);
        // Java: sha1 = byteArrayToHexString(md.digest());
        // Java: catch (IOException var10)
        // Java: catch (NoSuchAlgorithmException var11)
        // Java: finally
        // Java: closeQuietlyOrg(fis);
        // Java: return sha1;
    }

    // Java: static String byteArrayToHexString(_array: &[u8])
    pub fn _method_4() {
        // Java: if (null == array)
        // Java: return null;
        // Java: else
        // Java: StringBuilder sb = new StringBuilder(array.length * 2);
        // Java: for(int i = 0; i < array.length; ++i)
        // Java: int v = array[i] & 255;
        // Java: if (v < 16)
        // Java: sb.append('0');
        // Java: sb.append(Integer.toHexString(v));
        // Java: return sb.toString();
    }

    // Java: static void closeQuietly(FileOutputStream out)
    pub fn _method_5() {
        // Java: FileCloseUtil.closeQuietly(out);
    }

    // Java: static void closeQuietly(RandomAccessFile out)
    pub fn _method_6() {
        // Java: FileCloseUtil.closeQuietly(out);
    }

    // Java: static void closeQuietlyOrg(Closeable closable)
    pub fn _method_7() {
        // Java: if (null != closable)
        // Java: try
        // Java: if (closable instanceof java.io.FileOutputStream)
        // Java: java.io.FileOutputStream _out = (java.io.FileOutputStream)closable;
        // Java: _out.flush();
        // Java: _out.getFD().sync();
        // Java: else if (closable instanceof java.io.RandomAccessFile)
        // Java: java.io.RandomAccessFile _out = (java.io.RandomAccessFile)closable;
        // Java: _out.getFD().sync();
        // Java: closable.close();
        // Java: catch (IOException e)
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "closeQuietlyOrg", e);
        // Java: KmoLog.e("FileUtil", "closeQuietlyOrg", e);
    }

    // Java: static void closeQuietly(Closeable closable)
    pub fn _method_8() {
        // Java: FileCloseUtil.closeQuietly(closable);
    }

    // Java: static boolean safeCopyFile(String srcPath, String tarPath)
    pub fn _method_9() {
        // Java: return null != srcPath && null != tarPath ? safeCopyFile(new File(srcPath), new File(tarPath)) : false;
    }

    // Java: static boolean safeCopyFileEx(String srcPath, String tarPath)
    pub fn _method_10() {
        // Java: if (null != srcPath && null != tarPath)
        // Java: KmoLog.logPoint("FileUtil.safeCopyFileEx:" + tarPath);
        // Java: return safeCopyFileEx(new File(srcPath), new File(tarPath));
        // Java: else
        // Java: return false;
    }

    // Java: static boolean safeCopyFile(File srcFile, File tarFile)
    pub fn _method_11() {
        // Java: return safeCopyFile(srcFile, tarFile, (byte[])null);
    }

    // Java: static boolean safeCopyFileEx(File srcFile, File tarFile)
    pub fn _method_12() {
        // Java: return safeCopyFileEx(srcFile, tarFile, (byte[])null);
    }

    // Java: static boolean safeCopyFile(File srcFile, File tarFile, _buffer: &[u8])
    pub fn _method_13() {
        // Java: try
        // Java: return safeCopyFileEx(srcFile, tarFile, buffer);
        // Java: catch (Exception e)
        // Java: e.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "safeCopyFile", e);
        // Java: KmoLog.e("FileUtil", "safeCopyFile", e);
        // Java: return false;
    }

    // Java: static boolean safeCopyFileEx(File srcFile, File tarFile, _buffer: &[u8])
    pub fn _method_14() {
        // Java: if (null != srcFile && null != tarFile)
        // Java: if (!srcFile.exists())
        // Java: return false;
        // Java: else
        // Java: boolean result = false;
        // Java: File tmpFile = null;
        // Java: try
        // Java: File parentDir = tarFile.getParentFile();
        // Java: parentDir.mkdirs();
        // Java: if (parentDir != null)
        // Java: KmoLog.logPoint("FileUtil.safeCopyFileEx:" + parentDir.getAbsolutePath());
        // Java: else
        // Java: KmoLog.logPoint("FileUtil.safeCopyFileEx: parentDir is null");
        // Java: try
        // Java: tmpFile = File.createTempFile("moffice", (String)null, parentDir);
        // Java: catch (IOException e)
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // Java: throw e;
        // Java: throw new IOException("path:" + parentDir, e);
        // Java: result = copyFile(srcFile, tmpFile, buffer) && tmpFile.renameTo(tarFile);
        // Java: if (result)
        // Java: tmpFile = null;
        // Java: catch (IOException e)
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // Java: throw new NoSpaceLeftException(e);
        // Java: throw e;
        // Java: finally
        // Java: if (null != tmpFile)
        // Java: tmpFile.delete();
        // Java: return result;
        // Java: else
        // Java: return false;
    }

    // Java: static boolean copyFile(String oldPath, String newPath)
    pub fn _method_15() {
        // Java: return null != oldPath && null != newPath ? copyFile(new File(oldPath), new File(newPath)) : false;
    }

    // Java: static boolean copyFileOrg(String oldPath, String newPath)
    pub fn _method_16() {
        // Java: return null != oldPath && null != newPath ? copyFileOrg(new java.io.File(oldPath), new java.io.File(newPath)) : false;
    }

    // Java: static boolean checkIsPathSimply(String filePath)
    pub fn _method_17() {
        // Java: return filePath != null && filePath.contains("/");
    }

    // Java: static boolean copyFileByChannel(String oldPath, String newPath)
    pub fn _method_18() {
        // Java: File oldfile = new File(oldPath);
        // Java: if (!oldfile.exists())
        // Java: return false;
        // Java: else
        // Java: boolean result = true;
        // Java: File newFile = new File(newPath);
        // Java: if (oldfile.getAbsolutePath().equalsIgnoreCase(newFile.getAbsolutePath()))
        // Java: return result;
        // Java: else
        // Java: cn.wps.moffice.kfs.FileInputStream fileInputStream = null;
        // Java: FileOutputStream fileOutputStream = null;
        // Java: FileChannel fileChannelIn = null;
        // Java: FileChannel fileChannelOut = null;
        // Java: try
        // Java: fileInputStream = new cn.wps.moffice.kfs.FileInputStream(oldPath);
        // Java: fileOutputStream = new FileOutputStream(newPath);
        // Java: fileChannelIn = fileInputStream.getChannel();
        // Java: fileChannelOut = fileOutputStream.getChannel();
        // Java: fileChannelIn.transferTo(0L, fileChannelIn.size(), fileChannelOut);
        // Java: catch (IOException e)
        // Java: KSLog.e("FileUtil", "Exception", e);
        // Java: finally
        // Java: try
        // Java: fileInputStream.close();
        // Java: catch (IOException e)
        // Java: KSLog.e("FileUtil", "Exception", e);
        // Java: result = false;
        // Java: try
        // Java: fileChannelIn.close();
        // Java: catch (IOException e)
        // Java: KSLog.e("FileUtil", "Exception", e);
        // Java: result = false;
        // Java: try
        // Java: fileOutputStream.close();
        // Java: catch (IOException e)
        // Java: KSLog.e("FileUtil", "Exception", e);
        // Java: result = false;
        // Java: try
        // Java: fileChannelOut.close();
        // Java: catch (IOException e)
        // Java: KSLog.e("FileUtil", "Exception", e);
        // Java: result = false;
        // Java: return result;
    }

    // Java: static boolean copyFile(File oldfile, File newFile)
    pub fn _method_19() {
        // Java: return copyFile(oldfile, newFile, (byte[])null);
    }

    // Java: static boolean copyFileOrg(java.io.File oldfile, java.io.File newFile)
    pub fn _method_20() {
        // Java: return copyFileOrg(oldfile, newFile, (byte[])null);
    }

    // Java: static boolean copyFileEx2(File oldfile, File newFile)
    pub fn _method_21() {
        // Java: return copyFileEx(oldfile, newFile, (byte[])null);
    }

    // Java: static boolean copyFile(File srcFile, File tarFile, _buffer: &[u8])
    pub fn _method_22() {
        // Java: try
        // Java: return copyFileEx(srcFile, tarFile, buffer);
        // Java: catch (Exception e)
        // Java: e.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "copyFile", e);
        // Java: KmoLog.e("FileUtil", "copyFile", e);
        // Java: return false;
    }

    // Java: static boolean copyFileEx(File srcFile, File tarFile, _buffer: &[u8])
    pub fn _method_23() {
        // Java: if (null != srcFile && null != tarFile)
        // Java: if (!srcFile.exists())
        // Java: return false;
        // Java: else if (srcFile.getAbsolutePath().equalsIgnoreCase(tarFile.getAbsolutePath()))
        // Java: return true;
        // Java: else if (!prepareFilePath(tarFile))
        // Java: return false;
        // Java: else
        // Java: cn.wps.moffice.kfs.FileInputStream is = null;
        // Java: FileOutputStream os = null;
        // Java: boolean var5;
        // Java: try
        // Java: is = new cn.wps.moffice.kfs.FileInputStream(srcFile);
        // Java: os = new FileOutputStream(tarFile);
        // Java: copy(is, os, buffer);
        // Java: var5 = true;
        // Java: finally
        // Java: closeQuietly(os);
        // Java: closeQuietly((Closeable)is);
        // Java: return var5;
        // Java: else
        // Java: return false;
    }

    // Java: static boolean copyFileOrg(java.io.File srcFile, java.io.File tarFile, _buffer: &[u8])
    pub fn _method_24() {
        // Java: try
        // Java: return copyFileExOrg(srcFile, tarFile, buffer);
        // Java: catch (Exception e)
        // Java: e.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "copyFile", e);
        // Java: return false;
    }

    // Java: static boolean copyFileExOrg(java.io.File srcFile, java.io.File tarFile, _buffer: &[u8])
    pub fn _method_25() {
        // Java: if (null != srcFile && null != tarFile)
        // Java: if (!srcFile.exists())
        // Java: return false;
        // Java: else if (srcFile.getAbsolutePath().equalsIgnoreCase(tarFile.getAbsolutePath()))
        // Java: return true;
        // Java: else if (!prepareFilePathOrg(tarFile))
        // Java: return false;
        // Java: else
        // Java: FileInputStream is = null;
        // Java: java.io.FileOutputStream os = null;
        // Java: boolean var5;
        // Java: try
        // Java: is = new FileInputStream(srcFile);
        // Java: os = new java.io.FileOutputStream(tarFile);
        // Java: copy(is, os, buffer);
        // Java: var5 = true;
        // Java: finally
        // Java: closeQuietlyOrg(os);
        // Java: closeQuietlyOrg(is);
        // Java: return var5;
        // Java: else
        // Java: return false;
    }

    // Java: static boolean copyFile(InputStream in, String destPath)
    pub fn _method_26() {
        // Java: return copyFile(in, destPath, true);
    }

    // Java: static boolean copyFile(InputStream in, String destPath, boolean closeInput)
    pub fn _method_27() {
        // Java: if (in != null && destPath != null)
        // Java: File outFile = new File(destPath);
        // Java: if (!prepareFilePath(outFile))
        // Java: return false;
        // Java: else
        // Java: FileOutputStream os = null;
        // Java: try
        // Java: os = new FileOutputStream(outFile);
        // Java: copy(in, os);
        // Java: boolean e = true;
        // Java: return e;
        // Java: catch (Exception e)
        // Java: e.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "copyFile", e);
        // Java: KmoLog.e("FileUtil", "copyFile", e);
        // Java: finally
        // Java: closeQuietly(os);
        // Java: if (closeInput)
        // Java: closeQuietly((Closeable)in);
        // Java: return false;
        // Java: else
        // Java: return false;
    }

    // Java: static boolean copyFileOrg(InputStream in, String destPath)
    pub fn _method_28() {
        // Java: return copyFileOrg(in, destPath, true);
    }

    // Java: static boolean copyFileOrg(InputStream in, String destPath, boolean closeInput)
    pub fn _method_29() {
        // Java: if (in != null && destPath != null)
        // Java: java.io.File outFile = new java.io.File(destPath);
        // Java: if (!prepareFilePathOrg(outFile))
        // Java: return false;
        // Java: else
        // Java: java.io.FileOutputStream os = null;
        // Java: try
        // Java: os = new java.io.FileOutputStream(outFile);
        // Java: copy(in, os);
        // Java: boolean e = true;
        // Java: return e;
        // Java: catch (Exception e)
        // Java: e.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "copyFile", e);
        // Java: finally
        // Java: closeQuietlyOrg(os);
        // Java: if (closeInput)
        // Java: closeQuietlyOrg(in);
        // Java: return false;
        // Java: else
        // Java: return false;
    }

    // Java: static void copy(InputStream in, OutputStream out)
    pub fn _method_30() {
        // Java: copy(in, out, (byte[])null);
    }

    // Java: static void copy(InputStream in, OutputStream out, _buffer: &[u8])
    pub fn _method_31() {
        // Java: if (null == buffer)
        // Java: buffer = new byte[8192];
        // Java: int read;
        // Java: while((read = in.read(buffer)) != -1)
        // Java: out.write(buffer, 0, read);
    }

    // Java: static int dirFileCount(File[] dirs)
    pub fn _method_32() {
        // Java: if (null == dirs)
        // Java: return 0;
        // Java: else
        // Java: int result = 0;
        // Java: int i = 0;
        // Java: for(int length = dirs.length; i < length; ++i)
        // Java: result += dirFileCount(dirs[i]);
        // Java: return result;
    }

    // Java: static int dirFileCount(File dir)
    pub fn _method_33() {
        // Java: if (null == dir)
        // Java: return 0;
        // Java: else if (!dir.isDirectory())
        // Java: return 0;
        // Java: else
        // Java: int result = 0;
        // Java: Stack<File> dirlist = new Stack();
        // Java: dirlist.push(dir);
        // Java: while(!dirlist.isEmpty())
        // Java: File dirCurrent = (File)dirlist.pop();
        // Java: File[] fileList = dirCurrent.listFiles();
        // Java: if (null != fileList)
        // Java: int i = 0;
        // Java: for(int length = fileList.length; i < length; ++i)
        // Java: File f = fileList[i];
        // Java: if (f.isDirectory())
        // Java: dirlist.push(f);
        // Java: else
        // Java: ++result;
        // Java: return result;
    }

    // Java: static long dirSize(File[] dirs)
    pub fn _method_34() {
        // Java: if (null == dirs)
        // Java: return 0L;
        // Java: else
        // Java: long result = 0L;
        // Java: int i = 0;
        // Java: for(int length = dirs.length; i < length; ++i)
        // Java: result += dirSize(dirs[i]);
        // Java: return result;
    }

    // Java: static long dirSize(File dir)
    pub fn _method_35() {
        // Java: if (null == dir)
        // Java: return 0L;
        // Java: else
        // Java: long result = 0L;
        // Java: Stack<File> dirlist = new Stack();
        // Java: dirlist.push(dir);
        // Java: while(!dirlist.isEmpty())
        // Java: File dirCurrent = (File)dirlist.pop();
        // Java: File[] fileList = dirCurrent.listFiles();
        // Java: if (null != fileList)
        // Java: int i = 0;
        // Java: for(int length = fileList.length; i < length; ++i)
        // Java: File f = fileList[i];
        // Java: if (f.isDirectory())
        // Java: dirlist.push(f);
        // Java: else
        // Java: result += f.length();
        // Java: return result;
    }

    // Java: static boolean deleteDirFiles(File[] dirs)
    pub fn _method_36() {
        // Java: writeLog("deleteDirFiles delete dirs = " + dirs);
        // Java: if (null == dirs)
        // Java: return false;
        // Java: else
        // Java: boolean result = true;
        // Java: int i = 0;
        // Java: for(int length = dirs.length; i < length; ++i)
        // Java: result &= deleteDirFiles(dirs[i]);
        // Java: return result;
    }

    // Java: static boolean deleteDirFiles(String dirPath)
    pub fn _method_37() {
        // Java: File dir = new File(dirPath);
        // Java: return deleteDirFiles(dir);
    }

    // Java: static boolean deleteDirFiles(File dir)
    pub fn _method_38() {
        // Java: if (null == dir)
        // Java: return false;
        // Java: else
        // Java: writeLog("deleteDirFiles delete dir = " + dir);
        // Java: Stack<File> dirlist = new Stack();
        // Java: Stack<File> dirlist2 = new Stack();
        // Java: dirlist.push(dir);
        // Java: dirlist2.push(dir);
        // Java: while(!dirlist.isEmpty())
        // Java: File dirCurrent = (File)dirlist.pop();
        // Java: File[] fileList = dirCurrent.listFiles();
        // Java: if (null != fileList)
        // Java: int i = 0;
        // Java: for(int length = fileList.length; i < length; ++i)
        // Java: File f = fileList[i];
        // Java: if (f.isDirectory())
        // Java: dirlist.push(f);
        // Java: dirlist2.push(f);
        // Java: else
        // Java: f.delete();
        // Java: writeLog("deleteDirFiles delete f = " + f, false);
        // Java: while(!dirlist2.isEmpty())
        // Java: File dirCurrent = (File)dirlist2.pop();
        // Java: dirCurrent.delete();
        // Java: writeLog("deleteDirFiles delete dirCurrent = " + dirCurrent, false);
        // Java: return true;
    }

    // Java: static File getOldestFile(File[] dirs)
    pub fn _method_39() {
        // Java: if (null == dirs)
        // Java: return null;
        // Java: else
        // Java: File oldestFile = null;
        // Java: int i = 0;
        // Java: for(int length = dirs.length; i < length; ++i)
        // Java: File f = getOldestFile(dirs[i]);
        // Java: if (null != f && (null == oldestFile || oldestFile.lastModified() > f.lastModified()))
        // Java: oldestFile = f;
        // Java: return oldestFile;
    }

    // Java: static File getOldestFile(File dir)
    pub fn _method_40() {
        // Java: if (null != dir && dir.isDirectory())
        // Java: Stack<File> dirlist = new Stack();
        // Java: dirlist.push(dir);
        // Java: long oldestModifiedTime = Long.MAX_VALUE;
        // Java: File oldestFile = null;
        // Java: while(!dirlist.isEmpty())
        // Java: File[] fileList = ((File)dirlist.pop()).listFiles();
        // Java: if (null != fileList)
        // Java: int i = 0;
        // Java: for(int length = fileList.length; i < length; ++i)
        // Java: File f = fileList[i];
        // Java: if (f.isDirectory())
        // Java: dirlist.push(f);
        // Java: else
        // Java: long time = f.lastModified();
        // Java: if (time < oldestModifiedTime)
        // Java: oldestModifiedTime = time;
        // Java: oldestFile = f;
        // Java: return oldestFile;
        // Java: else
        // Java: return null;
    }

    // Java: static boolean prepareFilePath(File file)
    pub fn _method_41() {
        // Java: File parent = file.getParentFile();
        // Java: if (parent == null)
        // Java: return false;
        // Java: else
        // Java: parent.mkdirs();
        // Java: return true;
    }

    // Java: static boolean prepareFilePathOrg(java.io.File file)
    pub fn _method_42() {
        // Java: java.io.File parent = file.getParentFile();
        // Java: if (parent == null)
        // Java: return false;
        // Java: else
        // Java: parent.mkdirs();
        // Java: return true;
    }

    // Java: static boolean moveFile(File srcFile, File destFile)
    pub fn _method_43() {
        // Java: try
        // Java: return moveFileEx(srcFile, destFile);
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "moveFile", e);
        // Java: KmoLog.e("FileUtil", "copyFile", e);
        // Java: return false;
    }

    // Java: static boolean moveFileEx(File srcFile, File destFile)
    pub fn _method_44() {
        // Java: writeLog("moveFile src = " + srcFile + " dest = " + destFile, true);
        // Java: if (null != srcFile && null != destFile)
        // Java: if (!srcFile.exists())
        // Java: return false;
        // Java: else
        // Java: KmoLog.logPoint("moveFileEx:srcFile exists");
        // Java: if (!prepareFilePath(destFile))
        // Java: return false;
        // Java: else
        // Java: KmoLog.logPoint("moveFileEx:prepareFilePath");
        // Java: try
        // Java: if (srcFile.renameTo(destFile))
        // Java: return true;
        // Java: catch (Throwable e)
        // Java: KmoLog.logPoint(e.getStackTrace());
        // Java: KmoLog.logPoint("FiltUtil.moveFileEx:" + destFile.getAbsolutePath());
        // Java: if (safeCopyFileEx(srcFile, destFile))
        // Java: srcFile.delete();
        // Java: return true;
        // Java: else
        // Java: KmoLog.logPoint("moveFileEx:fail");
        // Java: return false;
        // Java: else
        // Java: return false;
    }

    // Java: static boolean moveFile(String oldPath, String newPath)
    pub fn _method_45() {
        // Java: return null != oldPath && null != newPath ? moveFile(new File(oldPath), new File(newPath)) : false;
    }

    // Java: static String join(String prefix, String suffix)
    pub fn _method_46() {
        // Java: int prefixLength = prefix.length();
        // Java: boolean haveSlash = prefixLength > 0 && prefix.charAt(prefixLength - 1) == File.separatorChar;
        // Java: if (!haveSlash)
        // Java: haveSlash = suffix.length() > 0 && suffix.charAt(0) == File.separatorChar;
        // Java: return haveSlash ? prefix + suffix : prefix + File.separatorChar + suffix;
    }

    // Java: static boolean addToFile(String filePath, InputStream in, long size, long startIndex, DataTransferListener listener)
    pub fn _method_47() {
        // Java: return addToFile(filePath, in, size, startIndex, listener, true);
    }

    // Java: static boolean addToFile(String filePath, InputStream in, long size, long startIndex, DataTransferListener listener, boolean closeInput)
    pub fn _method_48() {
        // Java: if (null != filePath && null != in)
        // Java: RandomAccessFile out = new RandomAccessFile(filePath, "rw");
        // Java: try
        // Java: if (listener != null)
        // Java: listener.started();
        // Java: out.seek(startIndex);
        // Java: long downloadSize = startIndex;
        // Java: int nRead = 0;
        // Java: _b: &[u8] = new byte[8192];
        // Java: while((nRead = in.read(b, 0, b.length)) != -1)
        // Java: if (listener == null)
        // Java: out.write(b, 0, nRead);
        // Java: downloadSize += (long)nRead;
        // Java: else
        // Java: if (listener.isCanceled(filePath))
        // Java: listener.aborted();
        // Java: break;
        // Java: out.write(b, 0, nRead);
        // Java: downloadSize += (long)nRead;
        // Java: listener.transferred((int)(downloadSize * 100L / size));
        // Java: finally
        // Java: closeQuietly(out);
        // Java: if (closeInput)
        // Java: closeQuietly((Closeable)in);
        // Java: return true;
        // Java: else
        // Java: return false;
    }

    // Java: static boolean writeToFile(String filePath, InputStream io, long size, long startIndex, boolean isUserTmpFile, DataTransferListener listener)
    pub fn _method_49() {
        // Java: boolean result = addToFile(filePath + ".tmp", io, size, startIndex, listener);
        // Java: if (result && listener != null && !listener.isCanceled(filePath))
        // Java: if (isUserTmpFile)
        // Java: moveFile(filePath + ".tmp", filePath);
        // Java: listener.transferred(100);
        // Java: listener.completed(filePath);
        // Java: return result;
    }

    // Java: static boolean writeToFile(File file, InputStream io, long size, DataTransferListener listener)
    pub fn _method_50() {
        // Java: return null == file ? false : addToFile(file.getAbsolutePath(), io, size, 0L, listener);
    }

    // Java: static boolean writeToFile(String strFileName, InputStream ins)
    pub fn _method_51() {
        // Java: return writeToFile(strFileName, ins, false);
    }

    // Java: static boolean writeToFile(String strFileName, InputStream ins, boolean append)
    pub fn _method_52() {
        // Java: return writeToFile(strFileName, ins, append, true);
    }

    // Java: static boolean writeToFile(String strFileName, InputStream ins, boolean append, boolean closeInput)
    pub fn _method_53() {
        // Java: if (null != strFileName && null != ins)
        // Java: FileOutputStream fouts = null;
        // Java: try
        // Java: fouts = new FileOutputStream(strFileName, append);
        // Java: copy(ins, fouts);
        // Java: boolean e = true;
        // Java: return e;
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "writeToFile", e);
        // Java: KmoLog.e("FileUtil", "writeToFile", e);
        // Java: finally
        // Java: closeQuietly(fouts);
        // Java: if (closeInput)
        // Java: closeQuietly((Closeable)ins);
        // Java: return false;
        // Java: else
        // Java: return false;
    }

    // Java: static boolean writeToFile(String strFileName, _bytes: &[u8])
    pub fn _method_54() {
        // Java: if (null != strFileName && null != bytes)
        // Java: FileOutputStream fouts = null;
        // Java: try
        // Java: File file = new File(strFileName);
        // Java: fouts = new FileOutputStream(file);
        // Java: fouts.write(bytes, 0, bytes.length);
        // Java: boolean var4 = true;
        // Java: return var4;
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "writeToFile", e);
        // Java: KmoLog.e("FileUtil", "writeToFile", e);
        // Java: finally
        // Java: closeQuietly(fouts);
        // Java: return false;
        // Java: else
        // Java: return false;
    }

    // Java: static boolean writeToFile(String filename, String data)
    pub fn _method_55() {
        // Java: return writeToFile(filename, data, false);
    }

    // Java: static boolean writeToFile(String filename, String data, boolean append)
    pub fn _method_56() {
        // Java: if (null != filename && null != data)
        // Java: java.io.FileOutputStream out = null;
        // Java: try
        // Java: out = new java.io.FileOutputStream(filename, append);
        // Java: out.write(data.getBytes());
        // Java: boolean ex = true;
        // Java: return ex;
        // Java: catch (IOException ex)
        // Java: ex.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "writeToFile", ex);
        // Java: KmoLog.e("FileUtil", "writeToFile", ex);
        // Java: finally
        // Java: closeQuietlyOrg(out);
        // Java: return false;
        // Java: else
        // Java: return false;
    }

    // Java: static String readFile(String filePath)
    pub fn _method_57() {
        // Java: if (null == filePath)
        // Java: return "";
        // Java: else
        // Java: cn.wps.moffice.kfs.FileInputStream inputstream = null;
        // Java: InputStreamReader reader = null;
        // Java: String result = "";
        // Java: try
        // Java: inputstream = new cn.wps.moffice.kfs.FileInputStream(filePath);
        // Java: StringBuilder builder = new StringBuilder();
        // Java: reader = new InputStreamReader(inputstream);
        // Java: _buffer: &[char] = new char[8192];
        // Java: int count;
        // Java: while((count = reader.read(buffer, 0, buffer.length)) > 0)
        // Java: builder.append(buffer, 0, count);
        // Java: result = builder.toString();
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "readFile", e);
        // Java: KmoLog.e("FileUtil", "readFile", e);
        // Java: finally
        // Java: closeQuietly((Closeable)inputstream);
        // Java: closeQuietly((Closeable)reader);
        // Java: return result;
    }

    // Java: static boolean newFolder(String folderPath)
    pub fn _method_58() {
        // Java: if (null == folderPath)
        // Java: return false;
        // Java: else
        // Java: File file = new File(folderPath);
        // Java: return file.exists() ? true : file.mkdirs();
    }

    // Java: static void newFile(String filePath, String content)
    pub fn _method_59() {
        // Java: if (null != filePath && null != content)
        // Java: FileOutputStream outStream = null;
        // Java: PrintWriter writer = null;
        // Java: try
        // Java: File file = new File(filePath);
        // Java: if (!file.exists())
        // Java: file.createNewFile();
        // Java: outStream = new FileOutputStream(file);
        // Java: writer = new PrintWriter(outStream);
        // Java: writer.println(content);
        // Java: catch (Exception e)
        // Java: e.printStackTrace();
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "newFile", e);
        // Java: KmoLog.e("FileUtil", "newFile", e);
        // Java: finally
        // Java: if (null != writer)
        // Java: writer.flush();
        // Java: closeQuietly(outStream);
    }

    // Java: static java.io.File newFileOrg(String path)
    pub fn _method_60() {
        // Java: if (null == path)
        // Java: return null;
        // Java: else
        // Java: java.io.File file = new java.io.File(path);
        // Java: java.io.File parent = file.getParentFile();
        // Java: if (!parent.exists())
        // Java: parent.mkdirs();
        // Java: if (file.exists())
        // Java: file.delete();
        // Java: file.createNewFile();
        // Java: return file;
    }

    // Java: static File newFile(String path)
    pub fn _method_61() {
        // Java: if (null == path)
        // Java: return null;
        // Java: else
        // Java: File file = new File(path);
        // Java: File parent = file.getParentFile();
        // Java: if (!parent.exists())
        // Java: parent.mkdirs();
        // Java: if (file.exists())
        // Java: file.delete();
        // Java: file.createNewFile();
        // Java: return file;
    }

    // Java: static boolean delFile(String path)
    pub fn _method_62() {
        // Java: if (null == path)
        // Java: return false;
        // Java: else
        // Java: writeLog("delFile delete path = " + path);
        // Java: File file = new File(path);
        // Java: return file.delete();
    }

    // Java: static boolean delFolder(String path)
    pub fn _method_63() {
        // Java: if (path == null)
        // Java: return false;
        // Java: else
        // Java: writeLog("delFolder delete path = " + path);
        // Java: delAllFile(path);
        // Java: File file = new File(path);
        // Java: return file.delete();
    }

    // Java: static boolean delFolder(File dir)
    pub fn _method_64() {
        // Java: if (dir == null)
        // Java: return false;
        // Java: else
        // Java: delAllFile(dir);
        // Java: return dir.delete();
    }

    // Java: static void delAllFile(String path)
    pub fn _method_65() {
        // Java: if (null != path)
        // Java: delAllFile(new File(path));
    }

    // Java: static void delAllFile(File file)
    pub fn _method_66() {
        // Java: writeLog("delAllFile delete file = " + file);
        // Java: if (null != file)
        // Java: if (file.exists())
        // Java: if (file.isDirectory())
        // Java: _list: &[String] = file.list();
        // Java: if (null != list)
        // Java: File temp = null;
        // Java: for(int i = 0; i < list.length; ++i)
        // Java: temp = new File(file, list[i]);
        // Java: if (temp.isFile())
        // Java: temp.delete();
        // Java: writeLog("delAllFile delete file = " + temp, false);
        // Java: else if (temp.isDirectory())
        // Java: delAllFile(temp);
        // Java: temp.delete();
        // Java: writeLog("delAllFile delete directory = " + temp, false);
    }

    // Java: static boolean isExpired(long modify, long expiredTime, long now)
    pub fn _method_67() {
        // Java: return now - modify >= expiredTime;
    }

    // Java: static void delAllFileNow(java.io.File file, IDeleteCallback cb)
    pub fn _method_68() {
        // Java: if (null != file)
        // Java: long expiredTime = 0L;
        // Java: delAllExpiredFile(file, expiredTime, false, cb);
    }

    // Java: static void delAllExpiredFile(java.io.File file, long expiredTime, boolean forceDeleteEmptyDir, IDeleteCallback cb)
    pub fn _method_69() {
        // Java: writeLog("delAllExpiredFile delete file = " + file + " expiredTime " + expiredTime + " forceDeleteEmptyDir " + forceDeleteEmptyDir);
        // Java: if (file.exists())
        // Java: if (file.isDirectory())
        // Java: java.io.File[] list = file.listFiles();
        // Java: java.io.File temp = null;
        // Java: long now = (new Date()).getTime();
        // Java: for(int i = 0; i < list.length; ++i)
        // Java: temp = list[i];
        // Java: if (temp.isFile())
        // Java: if (isExpired(temp.lastModified(), expiredTime, now))
        // Java: boolean shouldDelete = cb == null || cb != null && cb.shouldDelete(temp);
        // Java: if (shouldDelete)
        // Java: writeLog("delAllExpiredFile delete file = " + temp, false);
        // Java: temp.delete();
        // Java: else if (temp.isDirectory())
        // Java: boolean shouldDelete = cb == null || cb != null && cb.shouldDelete(temp);
        // Java: if (shouldDelete)
        // Java: delAllExpiredFile(temp, expiredTime, forceDeleteEmptyDir, cb);
        // Java: deleteExpiredEmptyDir(temp, expiredTime, forceDeleteEmptyDir);
    }

    // Java: static void delAllOutDayFile(java.io.File file)
    pub fn _method_70() {
        // Java: if (null != file)
        // Java: long expiredTime = 86400000L;
        // Java: delAllExpiredFile(file, expiredTime, true, (IDeleteCallback)null);
    }

    // Java: static void delAllExpiredFile(java.io.File file, IDeleteCallback cb)
    pub fn _method_71() {
        // Java: if (null != file)
        // Java: long expiredTime = 604800000L;
        // Java: delAllExpiredFile(file, expiredTime, false, cb);
    }

    // Java: static void delAllExpiredFile(java.io.File file, long expiredTime, IDeleteCallback cb)
    pub fn _method_72() {
        // Java: if (null != file)
        // Java: delAllExpiredFile(file, expiredTime, false, cb);
    }

    // Java: static void delAllExpiredFile(java.io.File file)
    pub fn _method_73() {
        // Java: writeLog("delAllExpiredFile file = " + file);
        // Java: if (null != file)
        // Java: long expiredTime = 604800000L;
        // Java: delAllExpiredFile(file, expiredTime, false, (IDeleteCallback)null);
    }

    // Java: static void writeLog(String s)
    pub fn _method_74() {
        // Java: writeLog(s, true);
    }

    // Java: static void writeLog(String s, boolean neesStack)
    pub fn _method_75() {
    }

    // Java: static void deleteExpiredEmptyDir(java.io.File dir, long expiredTime, boolean isForce)
    pub fn _method_76() {
        // Java: writeLog("deleteExpiredEmptyDir delete dir = " + dir + " expiredTime = " + expiredTime + " isForce = " + isForce);
        // Java: if (dir.isDirectory())
        // Java: long now = (new Date()).getTime();
        // Java: java.io.File[] fileLists = dir.listFiles();
        // Java: if (fileLists != null)
        // Java: for(int i = 0; i < fileLists.length; ++i)
        // Java: java.io.File tmpFile = fileLists[i];
        // Java: if (tmpFile.isDirectory())
        // Java: deleteExpiredEmptyDir(tmpFile, expiredTime, isForce);
        // Java: if (isForce || isExpired(tmpFile.lastModified(), expiredTime, now))
        // Java: writeLog("deleteExpiredEmptyDir delete file = " + tmpFile, false);
        // Java: tmpFile.delete();
        // Java: if (isForce || isExpired(dir.lastModified(), expiredTime, now))
        // Java: dir.delete();
        // Java: writeLog("deleteExpiredEmptyDir delete dir = " + dir, false);
    }

    // Java: static void copyFolder(String oldPath, String newPath)
    pub fn _method_77() {
        // Java: if (null != oldPath && null != newPath)
        // Java: copyFolder(new File(oldPath), new File(newPath));
    }

    // Java: static void copyFolder(File lhs, File rhs)
    pub fn _method_78() {
        // Java: if (null != lhs && null != rhs)
        // Java: rhs.mkdirs();
        // Java: _list: &[String] = lhs.list();
        // Java: File temp = null;
        // Java: for(int i = 0; i < list.length; ++i)
        // Java: temp = new File(lhs, list[i]);
        // Java: if (temp.isFile())
        // Java: copyFile(lhs, rhs);
        // Java: else if (temp.isDirectory())
        // Java: copyFolder(new File(lhs, list[i]), new File(rhs, list[i]));
    }

    // Java: static String copyFileEx(File src, File dest)
    pub fn _method_79() {
        // Java: if (null != src && null != dest)
        // Java: if (!src.exists())
        // Java: throw new FileNotFoundException("source file does not exist.");
        // Java: else
        // Java: InputStream in = new cn.wps.moffice.kfs.FileInputStream(src);
        // Java: return copyFileEx(in, dest, true);
        // Java: else
        // Java: return null;
    }

    // Java: static String copyFileEx(InputStream in, File dest)
    pub fn _method_80() {
        // Java: return copyFileEx(in, dest, true);
    }

    // Java: static String copyFileEx(InputStream in, File dest, boolean closeInput)
    pub fn _method_81() {
        // Java: if (null != in && null != dest)
        // Java: String sha1 = null;
        // Java: FileOutputStream out = null;
        // Java: MessageDigest digest = null;
        // Java: try
        // Java: digest = MessageDigest.getInstance("SHA-1");
        // Java: catch (NoSuchAlgorithmException e)
        // Java: KFSFileLogAdapter.eForSave("FileUtil", "copyFileEx", e);
        // Java: KmoLog.e("FileUtil", "copyFileEx", e);
        // Java: try
        // Java: out = new FileOutputStream(dest);
        // Java: _buf: &[u8] = new byte[8192];
        // Java: int read = 0;
        // Java: while((read = in.read(buf)) != -1)
        // Java: out.write(buf, 0, read);
        // Java: if (digest != null)
        // Java: digest.update(buf, 0, read);
        // Java: if (digest != null)
        // Java: sha1 = SHA1Util.bytes2HexString(digest.digest());
        // Java: finally
        // Java: closeQuietly(out);
        // Java: if (closeInput)
        // Java: closeQuietly((Closeable)in);
        // Java: return sha1;
        // Java: else
        // Java: return null;
    }

    // Java: static void moveFolder(String oldPath, String newPath)
    pub fn _method_82() {
        // Java: if (null != oldPath && null != newPath)
        // Java: File oldfile = new File(oldPath);
        // Java: if (oldfile.exists())
        // Java: copyFolder(oldPath, newPath);
        // Java: delFolder(oldPath);
    }

    // Java: static String getFileName(String filePath)
    pub fn _method_83() {
        // Java: return null == filePath ? null : (new File(filePath)).getName();
    }

    // Java: static String trimType(String filename)
    pub fn _method_84() {
        // Java: if (null == filename)
        // Java: return null;
        // Java: else
        // Java: int index = filename.lastIndexOf(".");
        // Java: return index != -1 ? filename.substring(0, index) : filename;
    }

    // Java: static String getFileTypeFromMimeType(String mineType)
    pub fn _method_85() {
        // Java: if ("text/plain".equals(mineType))
        // Java: return ".txt";
        // Java: else if ("application/log".equals(mineType))
        // Java: return ".log";
        // Java: else if (!"text/html".equals(mineType) && !"application/xhtml+xml".equals(mineType))
        // Java: if ("text/x-mht".equals(mineType))
        // Java: return ".mht";
        // Java: else if ("application/rtf".equals(mineType))
        // Java: return ".rtf";
        // Java: else if ("application/vnd.uof.text".equals(mineType))
        // Java: return ".uot3";
        // Java: else if ("application/pdf".equals(mineType))
        // Java: return ".pdf";
        // Java: else if (!"application/vnd.ms-works".equals(mineType) && !"application/kswps".equals(mineType) && !"application/wps".equals(mineType))
        // Java: if (!"application/kset".equals(mineType) && !"application/et".equals(mineType))
        // Java: if (!"application/ksdps".equals(mineType) && !"application/dps".equals(mineType))
        // Java: if (!"application/msexcel".equals(mineType) && !"application/vnd.ms-excel".equals(mineType))
        // Java: if ("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".equals(mineType))
        // Java: return ".xlsx";
        // Java: else if ("application/vnd.openxmlformats-officedocument.spreadsheetml.template".equals(mineType))
        // Java: return ".xltx";
        // Java: else if (!"application/mspowerpoint".equals(mineType) && !"application/vnd.ms-powerpoint".equals(mineType))
        // Java: if ("application/vnd.openxmlformats-officedocument.presentationml.presentation".equals(mineType))
        // Java: return ".pptx";
        // Java: else if ("application/vnd.openxmlformats-officedocument.presentationml.template".equals(mineType))
        // Java: return ".potx";
        // Java: else if ("application/vnd.openxmlformats-officedocument.presentationml.slideshow".equals(mineType))
        // Java: return ".ppsx";
        // Java: else if (!"application/msword".equals(mineType) && !"application/vnd.ms-word".equals(mineType))
        // Java: if ("application/vnd.openxmlformats-officedocument.wordprocessingml.document".equals(mineType))
        // Java: return ".docx";
        // Java: else if ("application/vnd.openxmlformats-officedocument.wordprocessingml.template".equals(mineType))
        // Java: return ".dotx";
        // Java: else if (!"text/comma-separated-values".equals(mineType) && !"text/csv".equals(mineType))
        // Java: if (!"application/zip".equals(mineType) && !"application/zipx".equals(mineType) && !"application/x-zip".equals(mineType))
        // Java: return !"application/x-rar-compressed".equals(mineType) && !"application/rar".equals(mineType) ? ".file" : ".rar";
        // Java: else
        // Java: return ".zip";
        // Java: else
        // Java: return ".csv";
        // Java: else
        // Java: return ".doc";
        // Java: else
        // Java: return ".ppt";
        // Java: else
        // Java: return ".xls";
        // Java: else
        // Java: return ".dps";
        // Java: else
        // Java: return ".et";
        // Java: else
        // Java: return ".wps";
        // Java: else
        // Java: return ".html";
    }

    // Java: static String getAutoNewFileName(String fileName)
    pub fn _method_86() {
        // Java: return getAutoNewFileName(fileName, true);
    }

    // Java: static _getAutoNewFileNames: &[String](String fileName)
    pub fn _method_87() {
        // Java: if (null == fileName)
        // Java: return null;
        // Java: else if (!(new File(fileName)).exists())
        // Java: return new String[]{fileName, fileName};
        // Java: else
        // Java: String oldFileName = fileName;
        // Java: String newName = null;
        // Java: String preName = null;
        // Java: _num: &[i32] = new int[1];
        // Java: num[0] = 1;
        // Java: String ext = StringUtil.pathExtension(fileName);
        // Java: fileName = StringUtil.stringByDeletingPathExtension(fileName);
        // Java: String originName = getOriginFileName(fileName, num);
        // Java: if (9999999 <= num[0] || 0 >= num[0])
        // Java: originName = fileName;
        // Java: num[0] = 1;
        // Java: while(true)
        // Java: newName = String.format("%s(%d).%s", originName, num[0], ext);
        // Java: if (!(new File(newName)).exists())
        // Java: if (preName == null)
        // Java: preName = oldFileName;
        // Java: return new String[]{preName, newName};
        // Java: preName = newName;
        // Java: int var10002 = num[0]++;
    }

    // Java: static String getAutoNewFileName(String fileName, boolean hasExtension)
    pub fn _method_88() {
        // Java: if (null == fileName)
        // Java: return null;
        // Java: else if (!(new File(fileName)).exists())
        // Java: return fileName;
        // Java: else
        // Java: String newName = null;
        // Java: _num: &[i32] = new int[1];
        // Java: num[0] = 1;
        // Java: if (!hasExtension)
        // Java: String originName = getOriginFileName(fileName, num);
        // Java: if (9999999 <= num[0] || 0 >= num[0])
        // Java: originName = fileName;
        // Java: num[0] = 1;
        // Java: while(true)
        // Java: newName = String.format("%s(%d)", originName, num[0]);
        // Java: if (!(new File(newName)).exists())
        // Java: break;
        // Java: int var10002 = num[0]++;
        // Java: else
        // Java: String ext = StringUtil.pathExtension(fileName);
        // Java: fileName = StringUtil.stringByDeletingPathExtension(fileName);
        // Java: String originName = getOriginFileName(fileName, num);
        // Java: if (9999999 <= num[0] || 0 >= num[0])
        // Java: originName = fileName;
        // Java: num[0] = 1;
        // Java: while(true)
        // Java: newName = String.format("%s(%d).%s", originName, num[0], ext);
        // Java: if (!(new File(newName)).exists())
        // Java: break;
        // Java: int var9 = num[0]++;
        // Java: return newName;
    }

    // Java: static String getAutoNewFileName(String fileName, List<String> existNames)
    pub fn _method_89() {
        // Java: if (null == fileName)
        // Java: return null;
        // Java: else if (existNames != null && !existNames.isEmpty())
        // Java: String newName = null;
        // Java: _num: &[i32] = new int[1];
        // Java: num[0] = 1;
        // Java: String ext = StringUtil.pathExtension(fileName);
        // Java: fileName = StringUtil.stringByDeletingPathExtension(fileName);
        // Java: String originName = getOriginFileName(fileName, num);
        // Java: if (9999999 <= num[0] || 0 >= num[0])
        // Java: originName = fileName;
        // Java: num[0] = 1;
        // Java: while(true)
        // Java: newName = String.format("%s(%d).%s", originName, num[0], ext);
        // Java: if (!existNames.contains(newName))
        // Java: return newName;
        // Java: int var10002 = num[0]++;
        // Java: else
        // Java: return fileName;
    }

    // Java: static String getOriginFileName(String fileName, _num: &[i32])
    pub fn _method_90() {
        // Java: int len = fileName.length();
        // Java: char endChar = fileName.charAt(len - 1);
        // Java: if (endChar != ')')
        // Java: return fileName;
        // Java: else
        // Java: int i = 0;
        // Java: for(i = len - 2; i >= 0; --i)
        // Java: char c = fileName.charAt(i);
        // Java: if (c == '(')
        // Java: break;
        // Java: if (!Character.isDigit(c))
        // Java: return fileName;
        // Java: if (i == len - 2)
        // Java: return fileName;
        // Java: else
        // Java: String numString = fileName.substring(i + 1, fileName.length() - 1);
        // Java: try
        // Java: num[0] = Integer.valueOf(numString);
        // Java: catch (Exception e)
        // Java: e.printStackTrace();
        // Java: num[0] = -1;
        // Java: return fileName.substring(0, i);
    }

    // Java: static void findFiles(String baseDirName, String targetFileName, List<String> fileList)
    pub fn _method_91() {
        // Java: if (null != baseDirName && null != targetFileName && null != fileList)
        // Java: String tempName = null;
        // Java: File baseDir = new File(baseDirName);
        // Java: if (baseDir.exists() && baseDir.isDirectory())
        // Java: _filelist: &[String] = baseDir.list();
        // Java: for(int i = 0; i < filelist.length; ++i)
        // Java: File readfile = new File(baseDirName, filelist[i]);
        // Java: if (!readfile.isDirectory())
        // Java: tempName = readfile.getName();
        // Java: if (wildcardMatch(targetFileName, tempName))
        // Java: fileList.add(readfile.getAbsolutePath());
        // Java: else if (readfile.isDirectory())
        // Java: findFiles(join(baseDirName, filelist[i]), targetFileName, fileList);
    }

    // Java: static boolean wildcardMatch(String pattern, String str)
    pub fn _method_92() {
        // Java: int patternLength = pattern.length();
        // Java: int strLength = str.length();
        // Java: int strIndex = 0;
        // Java: for(int patternIndex = 0; patternIndex < patternLength; ++patternIndex)
        // Java: char ch = pattern.charAt(patternIndex);
        // Java: if (ch == '*')
        // Java: while(strIndex < strLength)
        // Java: if (wildcardMatch(pattern.substring(patternIndex + 1), str.substring(strIndex)))
        // Java: return true;
        // Java: ++strIndex;
        // Java: else if (ch == '?')
        // Java: ++strIndex;
        // Java: if (strIndex > strLength)
        // Java: return false;
        // Java: else
        // Java: if (strIndex >= strLength || ch != str.charAt(strIndex))
        // Java: return false;
        // Java: ++strIndex;
        // Java: return strIndex == strLength;
    }

    // Java: static boolean isFileCanRead(String filePath)
    pub fn _method_93() {
        // Java: if (null != filePath && filePath.length() > 0)
        // Java: File f = new File(filePath);
        // Java: return f.exists() && f.canRead();
        // Java: else
        // Java: return false;
    }

    // Java: static boolean isFileCanReadAndWrite(String filePath)
    pub fn _method_94() {
        // Java: if (null != filePath && filePath.length() > 0)
        // Java: File f = new File(filePath);
        // Java: return f.exists() && f.canRead() && f.canWrite();
        // Java: else
        // Java: return false;
    }

    // Java: static boolean isFileNameValid(String fileName)
    pub fn _method_95() {
        // Java: return isFileNameValid(fileName, true);
    }

    // Java: static boolean isBlueNameValid(String bluetoothName)
    pub fn _method_96() {
        // Java: boolean result = false;
        // Java: if (null != bluetoothName && bluetoothName.length() > 0)
        // Java: if (bluetoothName.trim().length() == 0)
        // Java: return false;
        // Java: if (bluetoothName.getBytes().length > 90)
        // Java: return false;
        // Java: if (isInvalidEnd(bluetoothName))
        // Java: return false;
        // Java: if (isInvalidChar(bluetoothName))
        // Java: return false;
        // Java: Pattern p = Pattern.compile("[/\\\\:\"<>|?*\r\n\t]");
        // Java: Matcher m = p.matcher(bluetoothName);
        // Java: result = !m.find();
        // Java: return result;
    }

    // Java: static boolean isFileNameValid(String fileName, boolean checkSpecialChar)
    pub fn _method_97() {
        // Java: boolean result = false;
        // Java: if (null != fileName && fileName.length() > 0)
        // Java: if (fileName.trim().length() == 0)
        // Java: return false;
        // Java: if (fileName.getBytes().length > 240)
        // Java: return false;
        // Java: if (isInvalidEnd(fileName) && checkSpecialChar)
        // Java: return false;
        // Java: if (isInvalidChar(fileName) && checkSpecialChar)
        // Java: return false;
        // Java: Pattern p = Pattern.compile("[/\\\\:\"<>|?*\r\n\t]");
        // Java: Matcher m = p.matcher(fileName);
        // Java: result = !m.find();
        // Java: return result;
    }

    // Java: static boolean isInvalidEnd(String fileName)
    pub fn _method_98() {
        // Java: return matchEnd(fileName, ".") || matchEnd(fileName, " ");
    }

    // Java: static boolean matchEnd(String fileName, String match)
    pub fn _method_99() {
        // Java: String lastChar = fileName.substring(fileName.length() - 1);
        // Java: return lastChar != null && lastChar.equals(match);
    }

    // Java: static boolean isInvalidChar(String fileName)
    pub fn _method_100() {
        // Java: String trimFileName = fileName.trim();
        // Java: for(int i = 0; i < INVALID_NAMES.length; ++i)
        // Java: if (INVALID_NAMES[i].equalsIgnoreCase(trimFileName))
        // Java: return true;
        // Java: return false;
    }

    // Java: static boolean isFileNameValid(char ch)
    pub fn _method_101() {
        // Java: return "/\\:\"<>|?*\r\n\t".indexOf(ch) == -1;
    }

    // Java: static String getMIMEType(File f)
    pub fn _method_102() {
        // Java: String type = "";
        // Java: String fName = f.getName();
        // Java: String end = StringUtil.pathExtension(fName).toLowerCase();
        // Java: if (end.equals("pdf"))
        // Java: type = "application/pdf";
        // Java: else if (MimeType.Audio.contains(end))
        // Java: type = "audio/*";
        // Java: else if (MimeType.Video.contains(end))
        // Java: type = "video/*";
        // Java: else if (MimeType.Image.contains(end))
        // Java: type = "image/*";
        // Java: else if (MimeType.Archive.contains(end))
        // Java: type = "application/" + end;
        // Java: else
        // Java: type = null;
        // Java: return type;
    }

    // Java: static boolean exist(String filePath)
    pub fn _method_103() {
        // Java: if (null != filePath && filePath.length() > 0)
        // Java: File f = new File(filePath);
        // Java: return f.exists();
        // Java: else
        // Java: return false;
    }

    // Java: static boolean ensureFileExist(String filePath)
    pub fn _method_104() {
        // Java: if (null == filePath)
        // Java: return false;
        // Java: else
        // Java: java.io.File f = new java.io.File(filePath);
        // Java: if (!f.exists())
        // Java: try
        // Java: newFileOrg(filePath);
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
        // Java: return f.isFile();
    }

    // Java: static String encodePath(String path)
    pub fn _method_105() {
        // Java: if (path != null)
        // Java: path = path.replaceAll("[|]", "%7C");
        // Java: return path;
    }

    // Java: static String decodePath(String path)
    pub fn _method_106() {
        // Java: if (path != null)
        // Java: path = path.replaceAll("%7C", "|");
        // Java: return path;
    }

    // Java: static FileOutputStream getFileOutput(File file)
    pub fn _method_107() {
        // Java: return getFileOutput((File)file, 0);
    }

    // Java: static File getSaveOrTempFile(String path)
    pub fn _method_108() {
        // Java: String filePath = null;
        // Java: filePath = "/storage/emulated/0/Android/data/com.kingsoft.moffice_pro/.cache/KingsoftOffice/.temp/" + path;
        // Java: try
        // Java: newFile(filePath);
        // Java: catch (IOException var3)
        // Java: filePath = PlatformAdapter.createTempFile(path, "save").getAbsolutePath();
        // Java: return new File(filePath);
    }

    // Java: static FileOutputStream getFileOutput(File file, int tryCount)
    pub fn _method_109() {
        // Java: try
        // Java: FileOutputStream output = new FileOutputStream(file);
        // Java: return output;
        // Java: catch (FileNotFoundException e)
        // Java: ++tryCount;
        // Java: if (tryCount < 5)
        // Java: try
        // Java: Thread.sleep((long)(5 * tryCount));
        // Java: catch (InterruptedException e1)
        // Java: e1.printStackTrace();
        // Java: return getFileOutput(file, tryCount);
        // Java: else
        // Java: throw e;
    }

    // Java: static FileOutputStream getFileOutput(String filePath)
    pub fn _method_110() {
        // Java: return getFileOutput((String)filePath, 0);
    }

    // Java: static FileOutputStream getFileOutput(String filePath, int tryCount)
    pub fn _method_111() {
        // Java: try
        // Java: FileOutputStream output = new FileOutputStream(filePath);
        // Java: return output;
        // Java: catch (FileNotFoundException e)
        // Java: ++tryCount;
        // Java: if (tryCount < 5)
        // Java: try
        // Java: Thread.sleep((long)(5 * tryCount));
        // Java: catch (InterruptedException e1)
        // Java: e1.printStackTrace();
        // Java: return getFileOutput(filePath, tryCount);
        // Java: else
        // Java: throw e;
    }

    // Java: static String getAutoNewFileName(String filePath, IChecker checker)
    pub fn _method_112() {
        // Java: if (null == filePath)
        // Java: return null;
        // Java: else
        // Java: _num: &[i32] = new int[1];
        // Java: num[0] = 1;
        // Java: String ext = StringUtil.pathExtension(filePath);
        // Java: String originName = getOriginFileName(StringUtil.stringByDeletingPathExtension(filePath), num);
        // Java: if (9999999 <= num[0] || 0 >= num[0])
        // Java: originName = filePath;
        // Java: num[0] = 1;
        // Java: while(true)
        // Java: String newName = String.format("%s(%d).%s", originName, num[0], ext);
        // Java: if (null == checker.exist(newName))
        // Java: return newName;
        // Java: int var10002 = num[0]++;
    }

    // Java: static String getFilePathWithTime(String parentPath, String fileName, String ext)
    pub fn _method_113() {
        // Java: File parentFile = new File(parentPath);
        // Java: if (!parentFile.exists())
        // Java: parentFile.mkdirs();
        // Java: Date currentTime = new Date();
        // Java: SimpleDateFormat formatter = new SimpleDateFormat("yyyyMMddHHmmss");
        // Java: String dateString = formatter.format(currentTime);
        // Java: String realFileName = fileName + "_" + dateString;
        // Java: return parentFile.getAbsolutePath() + File.separator + realFileName + "." + ext;
    }

    // Java: static String openStringFile(String path)
    pub fn _method_114() {
        // Java: return openStringFile(new File(path));
    }

    // Java: static String openStringFile(File file)
    pub fn _method_115() {
        // Java: cn.wps.moffice.kfs.FileInputStream iStream = new cn.wps.moffice.kfs.FileInputStream(file);
        // Java: InputStreamReader reader = new InputStreamReader(iStream, "utf-8");
        // Java: StringBuilder sb = new StringBuilder();
        // Java: _buf: &[char] = new char[1024];
        // Java: for(int amt = reader.read(buf); amt > 0; amt = reader.read(buf))
        // Java: sb.append(buf, 0, amt);
        // Java: reader.close();
        // Java: return sb.toString();
    }

    // Java: static void saveStringFile(String path, String data)
    pub fn _method_116() {
        // Java: saveStringFile(new File(path), data);
    }

    // Java: static void saveStringFile(File file, String data)
    pub fn _method_117() {
        // Java: FileOutputStream oStream = new FileOutputStream(file);
        // Java: OutputStreamWriter writer = new OutputStreamWriter(oStream, "utf-8");
        // Java: writer.write(data);
        // Java: writer.close();
    }

    // Java: static ArrayList<String> getListFiles(File dir, ArrayList<String> paths)
    pub fn _method_118() {
        // Java: if (dir == null)
        // Java: KSLog.d("FileUtil", "dir in init_dir_files of PathsManager is null.");
        // Java: return paths;
        // Java: else
        // Java: File[] files = dir.listFiles();
        // Java: if (files != null && files.length >= 1)
        // Java: for(File file : files)
        // Java: if (file.isFile())
        // Java: paths.add(file.getAbsolutePath());
        // Java: else if (file.isDirectory())
        // Java: getListFiles(file, paths);
        // Java: KSLog.d("FileUtil", "getFiles end :" + paths.toString());
        // Java: return paths;
        // Java: else
        // Java: KSLog.d("FileUtil", "dir files is null.");
        // Java: return paths;
    }

    // Java: static boolean readByte(_bytes: &[u8], String path)
    pub fn _method_119() {
        // Java: if (path == null)
        // Java: return false;
        // Java: else
        // Java: cn.wps.moffice.kfs.FileInputStream fileInputStream = null;
        // Java: BufferedInputStream bufferedInputStream = null;
        // Java: boolean var11;
        // Java: try
        // Java: fileInputStream = new cn.wps.moffice.kfs.FileInputStream(path);
        // Java: bufferedInputStream = new BufferedInputStream(fileInputStream);
        // Java: int read = bufferedInputStream.read(bytes, 0, bytes.length);
        // Java: if (read > 0)
        // Java: return true;
        // Java: var11 = false;
        // Java: catch (Exception var91)
        // Java: var11 = false;
        // Java: return var11;
        // Java: finally
        // Java: closeQuietly((Closeable)bufferedInputStream);
        // Java: closeQuietly((Closeable)fileInputStream);
        // Java: return var11;
    }

    // Java: static _getBytesFromFile: &[u8](String filePath)
    pub fn _method_120() {
        // Java: try
        // Java: File file = new File(filePath);
        // Java: cn.wps.moffice.kfs.FileInputStream fis = new cn.wps.moffice.kfs.FileInputStream(file);
        // Java: ByteArrayOutputStream bos = new ByteArrayOutputStream(1024);
        // Java: _buffer: &[u8] = new byte[1024];
        // Java: int end;
        // Java: while((end = fis.read(buffer)) != -1)
        // Java: bos.write(buffer, 0, end);
        // Java: fis.close();
        // Java: _data: &[u8] = bos.toByteArray();
        // Java: bos.close();
        // Java: return data;
        // Java: catch (Exception e)
        // Java: e.printStackTrace();
        // Java: return new byte[0];
    }

}

// Java: public class FileType
pub struct FileType;

// Java field: int FILE_FMT_UNKNOW
// Java field: int FILE_FMT_DOC
// Java field: int FILE_FMT_XLS
// Java field: int FILE_FMT_PPT
// Java field: int FILE_FMT_DOCX
// Java field: int FILE_FMT_XLSX
// Java field: int FILE_FMT_PPTX
// Java field: int FILE_FMT_HTML
// Java field: int FILE_FMT_PDF
// Java field: int FILE_FMT_TXT
// Java field: int FILE_FMT_IMG
// Java field: int FILE_FMT_RTF

// Java: public class FileType
impl FileType {
    // Java: FileType()
    pub fn new() -> Self {
        // Java: super();
        FileType
    }
}
