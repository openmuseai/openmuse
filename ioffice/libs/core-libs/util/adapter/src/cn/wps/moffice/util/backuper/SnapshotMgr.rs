// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.core.runtime.SaveHelper
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.KfsContext
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import java.io.IOException
// Java: import java.nio.channels.FileLock
// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.HashSet
// Java: import java.util.Iterator
// Java: import java.util.List

use std::collections::HashSet;

// Java field: String SNAPSHOT_LIST_FILE_ENCODING
pub(crate) const SNAPSHOT_LIST_FILE_ENCODING: &str = "UTF8";
// Java field: long MAX_LIST_FILE_SIZE
pub(crate) const MAX_LIST_FILE_SIZE: i64 = 10000i64;
// Java field: int MAX_LIST_COUNT
pub(crate) const MAX_LIST_COUNT: i32 = 100;
// Java field: int MAX_PATH
pub(crate) const MAX_PATH: i32 = 1000;

// Java: public class SnapshotMgr
pub struct SnapshotMgr {
}

// Java: public class SnapshotMgr
impl SnapshotMgr {
    // Java: SnapshotMgr()
    pub fn new() -> Self {
      // Java: super();
      SnapshotMgr {}
    }

    // Java: static String getSnapshot(String srcfileStr)
    pub fn getSnapshot(_srcfileStr: &str) -> Option<String> {
      // Java: return getSnapshot(srcfileStr, SaveHelper.AppType.SpreadSheet);
      todo!()
    }

    // Java: static String getSnapshot(String srcfileStr, SaveHelper.AppType appType)
    pub fn getSnapshot_with_appType(_srcfileStr: &str, _appType: i32) -> Option<String> {
      // Java: if (!hasSnapshot(srcfileStr, appType))
      // Java: return null;
      // Java: else
      // Java: File snapshot = getSnapshotPath(srcfileStr, appType);
      // Java: if (!snapshot.exists())
      // Java: return null;
      // Java: else
      // Java: File srcfile = new File(srcfileStr);
      // Java: return srcfile.exists() && snapshot.lastModified() <= srcfile.lastModified() ? null : snapshot.getAbsolutePath();
      todo!()
    }

    // Java: static File getSnapshotDir(SaveHelper.AppType appType)
    pub fn getSnapshotDir(_appType: i32) -> Option<Box<dyn std::any::Any>> {
      // Java: File parentDir = null;
      // Java: File ret = null;
      // Java: if (appType == SaveHelper.AppType.Presentation)
      // Java: parentDir = new File(Platform.getPresBackupDirectory());
      // Java: ret = new File(parentDir, "io");
      // Java: else if (appType == SaveHelper.AppType.SpreadSheet)
      // Java: parentDir = new File(Platform.getETBackupDirectory());
      // Java: ret = new File(parentDir, "states");
      // Java: if (ret != null && !ret.exists())
      // Java: ret.mkdirs();
      // Java: return ret;
      todo!()
    }

    // Java: static File getSnapshotListFile(SaveHelper.AppType appType)
    pub fn getSnapshotListFile(_appType: i32) -> Option<Box<dyn std::any::Any>> {
      // Java: String globalBackup = Platform.getBackupDirectory();
      // Java: assert globalBackup != null;
      // Java: File parentDir = null;
      // Java: if (appType == SaveHelper.AppType.Presentation)
      // Java: parentDir = new File(Platform.getPresBackupDirectory());
      // Java: else if (appType == SaveHelper.AppType.SpreadSheet)
      // Java: parentDir = new File(Platform.getETBackupDirectory());
      // Java: if (!parentDir.exists())
      // Java: parentDir.mkdirs();
      // Java: return new File(parentDir, "crashmark.txt");
      todo!()
    }

    // Java: static File getSnapshotPath(String filePath)
    pub fn getSnapshotPath(_filePath: &str) -> Option<Box<dyn std::any::Any>> {
      // Java: return getSnapshotPath(filePath, SaveHelper.AppType.SpreadSheet);
      todo!()
    }

    // Java: static File getSnapshotPath(String filePath, SaveHelper.AppType appType)
    pub fn getSnapshotPath_with_appType(_filePath: &str, _appType: i32) -> Option<Box<dyn std::any::Any>> {
      // Java: assert filePath != null;
      // Java: if (filePath == null)
      // Java: return null;
      // Java: else
      // Java: File parentDir = getSnapshotDir(appType);
      // Java: if (parentDir == null)
      // Java: return null;
      // Java: else
      // Java: String hash = SaveHelper.md5digest(filePath);
      // Java: String destfilename = null;
      // Java: if (appType == SaveHelper.AppType.Presentation)
      // Java: destfilename = hash + ".ppt";
      // Java: if (filePath.toLowerCase().endsWith(".pptx"))
      // Java: destfilename = hash + ".pptx";
      // Java: else if (appType == SaveHelper.AppType.SpreadSheet)
      // Java: destfilename = hash + ".xls";
      // Java: if (filePath.toLowerCase().endsWith(".xlsx"))
      // Java: destfilename = hash + ".xlsx";
      // Java: return new File(parentDir, destfilename);
      todo!()
    }

    // Java: static void addToSnapshotList(String filePath)
    pub fn addToSnapshotList(_filePath: &str) {
      // Java: addToSnapshotList(filePath, SaveHelper.AppType.SpreadSheet);
      todo!()
    }

    // Java: static void addToSnapshotList(String filePath, SaveHelper.AppType appType)
    pub fn addToSnapshotList_with_appType(_filePath: &str, _appType: i32) {
      // Java: assert filePath != null;
      // Java: if (filePath != null)
      // Java: File markFile = getSnapshotListFile(appType);
      // Java: if (markFile != null)
      // Java: RandomAccessFile raf = null;
      // Java: FileLock fl = null;
      // Java: try
      // Java: try
      // Java: raf = new RandomAccessFile(markFile, "rw");
      // Java: if (KfsContext.instance().mode() == 0)
      // Java: fl = raf.getChannel().lock();
      // Java: String all = null;
      // Java: raf.seek(0L);
      // Java: long len = raf.length();
      // Java: if (len < 10000L)
      // Java: byte[] data = new byte[(int)len];
      // Java: raf.readFully(data);
      // Java: all = new String(data, "UTF8");
      // Java: byte[] var23 = null;
      // Java: int pos = all.indexOf(filePath);
      // Java: if (pos >= 0 && (pos == 0 || all.charAt(pos - 1) == '\n') && (pos + filePath.length() >= all.length() || all.charAt(pos + filePath.length()) == '\n'))
      // Java: return;
      // Java: StringBuilder sb = new StringBuilder(all == null ? "" : all);
      // Java: if (all != null && all.length() > 0 && all.charAt(all.length() - 1) != '\n')
      // Java: sb.append('\n');
      // Java: String var22 = null;
      // Java: sb.append(filePath);
      // Java: byte[] data = sb.toString().getBytes("UTF8");
      // Java: StringBuilder var25 = null;
      // Java: raf.setLength(0L);
      // Java: raf.write(data);
      // Java: catch (IOException var20)
      // Java: finally
      // Java: try
      // Java: if (fl != null)
      // Java: fl.release();
      // Java: if (raf != null)
      // Java: raf.close();
      // Java: catch (IOException var19)
      todo!()
    }

    // Java: static void removeFromSnapshotList(String filePath, SaveHelper.AppType appType)
    pub fn removeFromSnapshotList(_filePath: &str, _appType: i32) {
      // Java: assert filePath != null;
      // Java: if (filePath != null)
      // Java: File markFile = getSnapshotListFile(appType);
      // Java: if (markFile != null)
      // Java: RandomAccessFile raf = null;
      // Java: FileLock fl = null;
      // Java: try
      // Java: raf = new RandomAccessFile(markFile, "rw");
      // Java: if (KfsContext.instance().mode() == 0)
      // Java: fl = raf.getChannel().lock();
      // Java: raf.seek(0L);
      // Java: long len = raf.length();
      // Java: if (len < 10000L)
      // Java: byte[] data = new byte[(int)len];
      // Java: raf.readFully(data);
      // Java: String all = new String(data, "UTF8");
      // Java: byte[] var21 = null;
      // Java: int pos = all.indexOf(filePath);
      // Java: if (pos >= 0 && (pos == 0 || all.charAt(pos - 1) == '\n') && (pos + filePath.length() >= all.length() || all.charAt(pos + filePath.length()) == '\n'))
      // Java: StringBuilder sb = new StringBuilder();
      // Java: if (pos > 0)
      // Java: sb.append(all.substring(0, pos));
      // Java: if (pos + filePath.length() + 1 < all.length())
      // Java: sb.append(all.substring(pos + filePath.length() + 1));
      // Java: byte[] var22 = sb.toString().getBytes("UTF8");
      // Java: raf.setLength(0L);
      // Java: raf.write(var22);
      // Java: else
      // Java: raf.setLength(0L);
      // Java: catch (Throwable var19)
      // Java: finally
      // Java: try
      // Java: if (fl != null)
      // Java: fl.release();
      // Java: if (raf != null)
      // Java: raf.close();
      // Java: catch (IOException var18)
      todo!()
    }

    // Java: static void removeSnapshot(String filePath)
    pub fn removeSnapshot(_filePath: &str) {
      // Java: removeSnapshot(filePath, SaveHelper.AppType.SpreadSheet);
      todo!()
    }

    // Java: static void removeSnapshot(String filePath, SaveHelper.AppType appType)
    pub fn removeSnapshot_with_appType(_filePath: &str, _appType: i32) {
      // Java: assert filePath != null;
      // Java: if (filePath != null)
      // Java: File snapshot = getSnapshotPath(filePath, appType);
      // Java: if (snapshot.exists())
      // Java: snapshot.delete();
      // Java: removeFromSnapshotList(filePath, appType);
      todo!()
    }

    // Java: static boolean hasSnapshot(String filePath)
    pub fn hasSnapshot(_filePath: &str) -> bool {
      // Java: return hasSnapshot(filePath, SaveHelper.AppType.SpreadSheet);
      todo!()
    }

    // Java: static boolean hasSnapshot(String filePath, SaveHelper.AppType appType)
    pub fn hasSnapshot_with_appType(_filePath: &str, _appType: i32) -> bool {
      // Java: if (filePath == null)
      // Java: return false;
      // Java: else if (KfsContext.instance().mode() == 2)
      // Java: return false;
      // Java: else
      // Java: File markFile = getSnapshotListFile(appType);
      // Java: if (markFile == null)
      // Java: return false;
      // Java: else
      // Java: RandomAccessFile raf = null;
      // Java: FileLock fl = null;
      // Java: boolean var10;
      // Java: try
      // Java: raf = new RandomAccessFile(markFile, "rw");
      // Java: if (KfsContext.instance().mode() == 0)
      // Java: fl = raf.getChannel().tryLock();
      // Java: raf.seek(0L);
      // Java: long len = raf.length();
      // Java: if (len >= 10000L)
      // Java: raf.setLength(0L);
      // Java: return false;
      // Java: byte[] data = new byte[(int)len];
      // Java: raf.readFully(data);
      // Java: String all = new String(data, "UTF8");
      // Java: byte[] var23 = null;
      // Java: int pos = all.indexOf(filePath);
      // Java: if (pos < 0 || pos != 0 && all.charAt(pos - 1) != '\n' || pos + filePath.length() < all.length() && all.charAt(pos + filePath.length()) != '\n')
      // Java: return false;
      // Java: var10 = true;
      // Java: catch (Exception var21)
      // Java: return false;
      // Java: finally
      // Java: try
      // Java: if (fl != null)
      // Java: fl.release();
      // Java: if (raf != null)
      // Java: raf.close();
      // Java: catch (IOException var20)
      // Java: return var10;
      todo!()
    }

    // Java: static void syncSnapshotList(HashSet<String> documents)
    pub fn syncSnapshotList(_documents: &HashSet<String>) {
      // Java: syncSnapshotList(documents, SaveHelper.AppType.SpreadSheet);
      todo!()
    }

    // Java: static void syncSnapshotList(HashSet<String> documents, SaveHelper.AppType appType)
    pub fn syncSnapshotList_with_appType(_documents: &HashSet<String>, _appType: i32) {
      // Java: assert documents != null;
      // Java: if (documents != null)
      // Java: File markFile = getSnapshotListFile(appType);
      // Java: if (markFile != null && markFile.exists())
      // Java: RandomAccessFile raf = null;
      // Java: FileLock fl = null;
      // Java: try
      // Java: raf = new RandomAccessFile(markFile, "rw");
      // Java: if (KfsContext.instance().mode() == 0)
      // Java: fl = raf.getChannel().lock();
      // Java: if (fl != null)
      // Java: List<String> filePaths = new ArrayList();
      // Java: readSnapshotList(raf, filePaths);
      // Java: Iterator<String> iter = filePaths.iterator();
      // Java: while(iter.hasNext())
      // Java: String s = (String)iter.next();
      // Java: if (!documents.contains(s))
      // Java: File snapshot = getSnapshotPath(s, appType);
      // Java: if (snapshot.exists())
      // Java: snapshot.delete();
      // Java: iter.remove();
      // Java: if (Math.random() < 0.02)
      // Java: HashSet<String> markedSnapshots = new HashSet();
      // Java: for(String s : filePaths)
      // Java: markedSnapshots.add(getSnapshotPath(s, appType).getAbsolutePath());
      // Java: File f = getSnapshotDir(appType);
      // Java: File[] snapshotFiles = f.listFiles();
      // Java: if (snapshotFiles != null)
      // Java: for(int i = 0; i < snapshotFiles.length; ++i)
      // Java: File snapshotFile = snapshotFiles[i];
      // Java: if (snapshotFile.isFile() && !markedSnapshots.contains(snapshotFile.getAbsolutePath()))
      // Java: snapshotFile.delete();
      // Java: writeSnapshotList(raf, filePaths);
      // Java: return;
      // Java: catch (IOException var21)
      // Java: return;
      // Java: finally
      // Java: try
      // Java: if (fl != null)
      // Java: fl.release();
      // Java: if (raf != null)
      // Java: raf.close();
      // Java: catch (IOException var20)
      todo!()
    }

    // Java: static void readSnapshotList(RandomAccessFile raf, Collection<String> out)
    pub fn readSnapshotList(_raf: &dyn std::any::Any, _out: &mut Vec<String>) {
      // Java: out.clear();
      // Java: raf.seek(0L);
      // Java: long len = raf.length();
      // Java: if (len < 10000L)
      // Java: byte[] data = new byte[(int)len];
      // Java: raf.readFully(data);
      // Java: String all = new String(data, "UTF8");
      // Java: byte[] var9 = null;
      // Java: StringBuilder sb = new StringBuilder();
      // Java: int i = 0;
      // Java: for(int s = all.length(); i <= s; ++i)
      // Java: if (i != all.length() && all.charAt(i) != '\n')
      // Java: sb.append(all.charAt(i));
      // Java: else
      // Java: if (sb.length() > 0)
      // Java: out.add(sb.toString());
      // Java: sb.setLength(0);
      todo!()
    }

    // Java: static void writeSnapshotList(RandomAccessFile raf, Collection<String> filePaths)
    pub fn writeSnapshotList(_raf: &dyn std::any::Any, _filePaths: &[String]) {
      // Java: StringBuilder sb = new StringBuilder();
      // Java: int count = 0;
      // Java: for(String s : filePaths)
      // Java: if (count >= 100)
      // Java: break;
      // Java: if (s.length() < 1000)
      // Java: sb.append(s).append('\n');
      // Java: ++count;
      // Java: byte[] data = sb.toString().getBytes("UTF8");
      // Java: StringBuilder var6 = null;
      // Java: raf.setLength(0L);
      // Java: raf.write(data);
      todo!()
    }

}
