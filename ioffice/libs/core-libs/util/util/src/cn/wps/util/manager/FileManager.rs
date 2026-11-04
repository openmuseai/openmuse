#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.io.IOException
// Java: import java.util.concurrent.atomic.AtomicInteger
use std::sync::atomic::{AtomicI32, Ordering};
use std::path::{Path, PathBuf};

// Java field: int MAX_COUNT
const MAX_COUNT: i32 = 50;
// Java field: int MAX_TOTAL
const MAX_TOTAL: i32 = 2500;

// Java: public class FileManager
pub struct FileManager {
    // Java field: String mRootDir
    mRootDir: String,
    // Java field: String mRootPath
    mRootPath: String,
    // Java field: AtomicInteger mCount
    mCount: AtomicI32,
    // Java field: AtomicInteger mIndex
    mIndex: AtomicI32,
    // Java field: AtomicInteger mTotal
    mTotal: AtomicI32,
    // Java field: File mCurrentFile
    mCurrentFile: PathBuf,
    // Java field: int mPrefixIndex
    mPrefixIndex: i32,
}

// Java: public class FileManager
impl FileManager {
    // Java: FileManager(String rootDir)
    pub fn new(root_dir: &str) -> Self {
        // Java: super();
        // Java: this.mRootDir = rootDir;
        let mut mgr = Self {
            mRootDir: root_dir.to_string(),
            mRootPath: String::new(),
            mCount: AtomicI32::new(0),
            mIndex: AtomicI32::new(0),
            mTotal: AtomicI32::new(0),
            mCurrentFile: PathBuf::new(),
            mPrefixIndex: 0,
        };
        // Java: this.initPrefixIndex();
        mgr.initPrefixIndex();
        // Java: this.initData();
        mgr.initData();
        mgr
    }

    // Java: void delete()
    pub fn delete(&self) {
        // Java: synchronized(FileManager.class)
        {
            // Java: String temp = this.createRootPath(this.mPrefixIndex);
            let temp = self.createRootPath(self.mPrefixIndex);
            // Java: FileUtil.delFolder(temp);
            let _ = std::fs::remove_dir_all(&temp);
        }
    }

    // Java: void deleteOldFile()
    pub fn deleteOldFile(&self) {
        // Java: String temp = this.createRootPath(this.mPrefixIndex);
        let temp = self.createRootPath(self.mPrefixIndex);
        // Java: File file = new File(temp);
        let file = Path::new(&temp);
        // Java: File parentFile = file.getParentFile();
        // Java: if (parentFile != null)
        if let Some(parent_file) = file.parent()
        {
            // Java: File[] files = parentFile.listFiles();
            // Java: if (files != null)
            if let Ok(entries) = std::fs::read_dir(parent_file)
            {
                // Java: for(File tempFile : files)
                for entry in entries.flatten()
                {
                    // Java: if (tempFile != null && !tempFile.getName().equals(file.getName()))
                    if entry.file_name() != file.file_name().unwrap_or_default()
                    {
                        // Java: FileUtil.delFolder(tempFile);
                        let _ = std::fs::remove_dir_all(entry.path());
                    }
                }
            }
        }
    }

    // Java: String createRootPath(int prefixIndex)
    fn createRootPath(&self, prefix_index: i32) -> String {
        // Java: return this.mRootDir + "fileManager" + File.separator + prefixIndex + File.separator;
        format!("{}fileManager/{}/", self.mRootDir, prefix_index)
    }

    // Java: File createCurrentFile()
    fn createCurrentFile(&self) -> PathBuf {
        // Java: return new File(this.mRootPath + this.mIndex + "_" + System.currentTimeMillis());
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        PathBuf::from(format!("{}{}_{}", self.mRootPath, self.mIndex.load(Ordering::SeqCst), millis))
    }

    // Java: void initData()
    fn initData(&mut self) {
        // Java: this.mRootPath = this.createRootPath(this.mPrefixIndex) + System.currentTimeMillis() + File.separator;
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        self.mRootPath = format!("{}{}/", self.createRootPath(self.mPrefixIndex), millis);
        // Java: this.mCount.getAndSet(0);
        self.mCount.store(0, Ordering::SeqCst);
        // Java: this.mIndex.getAndSet(0);
        self.mIndex.store(0, Ordering::SeqCst);
        // Java: this.mTotal.getAndSet(0);
        self.mTotal.store(0, Ordering::SeqCst);
        // Java: this.mCurrentFile = this.createCurrentFile();
        self.mCurrentFile = self.createCurrentFile();
        // Java: this.mCurrentFile.mkdirs();
        let _ = std::fs::create_dir_all(&self.mCurrentFile);
    }

    // Java: void initPrefixIndex()
    fn initPrefixIndex(&mut self) {
        // Java: String rootPath = null;
        // Java: do
        loop
        {
            // Java: ++this.mPrefixIndex;
            self.mPrefixIndex += 1;
            // Java: rootPath = this.createRootPath(this.mPrefixIndex);
            let root_path = self.createRootPath(self.mPrefixIndex);
            if !Path::new(&root_path).exists() {
                break;
            }
        }
    }

    // Java: File createTempFile(String suffix)
    pub fn createTempFile(&self, suffix: &str) -> Option<PathBuf> {
        // Java: try
        {
            // Java: if (this.mCount.getAndAdd(0) > 50)
            if self.mCount.load(Ordering::SeqCst) > 50
            {
                // Java: synchronized(FileManager.class)
                {
                    // Java: if (this.mCount.getAndAdd(0) > 50)
                    if self.mCount.load(Ordering::SeqCst) > 50
                    {
                        // Java: this.mCount.getAndSet(0);
                        self.mCount.store(0, Ordering::SeqCst);
                        // Java: this.mIndex.incrementAndGet();
                        self.mIndex.fetch_add(1, Ordering::SeqCst);
                        // Java: this.mCurrentFile = this.createCurrentFile();
                        // Java: this.mCurrentFile.mkdirs();
                    }
                }
            }
            // Java: if (this.mTotal.getAndAdd(0) > 2500)
            if self.mTotal.load(Ordering::SeqCst) > 2500
            {
                // Java: synchronized(FileManager.class)
                {
                    // Java: if (this.mTotal.getAndAdd(0) > 2500)
                    if self.mTotal.load(Ordering::SeqCst) > 2500
                    {
                        // Java: this.initData();
                    }
                }
            }
            // Java: File file = null;
            let file: PathBuf;
            // Java: do
            {
                // Java: String pathSB = this.mCurrentFile.getAbsolutePath() + File.separator + this.mCount + "_" + Thread.currentThread().getName() + "_" + System.currentTimeMillis() + "." + suffix;
                let millis = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis();
                let path_sb = format!("{}/{}_thread_{}.{}", self.mCurrentFile.display(), self.mCount.load(Ordering::SeqCst), millis, suffix);
                // Java: file = new File(pathSB);
                file = PathBuf::from(path_sb);
            }
            // Java: File parentFile = file.getParentFile();
            // Java: if (!parentFile.exists())
            if let Some(parent_file) = file.parent() {
                if !parent_file.exists()
                {
                    // Java: parentFile.mkdirs();
                    let _ = std::fs::create_dir_all(parent_file);
                }
            }
            // Java: if (!file.createNewFile())
            if std::fs::File::create(&file).is_err()
            {
                // Java: return null;
                return None;
            }
            // Java: else
            {
                // Java: this.mCount.incrementAndGet();
                self.mCount.fetch_add(1, Ordering::SeqCst);
                // Java: this.mTotal.incrementAndGet();
                self.mTotal.fetch_add(1, Ordering::SeqCst);
                // Java: return file;
                return Some(file);
            }
        }
        // Java: catch (IOException var7)
        {
            // Java: return null;
            None
        }
    }
}
