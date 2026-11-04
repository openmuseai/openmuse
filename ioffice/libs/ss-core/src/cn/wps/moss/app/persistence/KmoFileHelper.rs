use std::sync::Mutex;
use crate::java_runtime::FileChannel::FileChannel;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

// java public class KmoFileHelper
pub struct KmoFileHelper;

// java private static List<String> pathList = new ArrayList();
static pathList: Mutex<Vec<String>> = Mutex::new(Vec::new());

// java public class KmoFileHelper
impl KmoFileHelper {
    // java public static FileChannel getFileChannel()
    pub fn getFileChannel() -> Option<FileChannel> {
        // java String fileName = UUID.randomUUID().toString();
        let fileName = uuid::Uuid::new_v4().to_string();
        // java String path = getFilePath(fileName);
        let path = Self::getFilePath(&fileName);
        // java pathList.add(path);
        pathList.lock().unwrap().push(path.clone());
        // java File temp = new File(path);
        let temp = path;
        // java temp.createNewFile();
        let created=std::fs::OpenOptions::new().write(true).create_new(true).open(&temp).map(|_|()).or_else(|e|if e.kind()==std::io::ErrorKind::AlreadyExists{Ok(())}else{Err(e)});
        match created {
            Ok(()) => {
                // java return (new RandomAccessFile(temp, "rw")).getChannel();
                match FileChannel::open(&temp){Ok(channel)=>Some(channel),Err(e)=>{eprintln!("{e}\n{}",std::backtrace::Backtrace::capture());None}}
            }
            Err(e) => {
                // java catch (IOException e)
                // java e.printStackTrace();
                // java return null;
                eprintln!("{e}\n{}",std::backtrace::Backtrace::capture());None
            }
        }
    }

    // java private static String getFilePath(String fileName)
    fn getFilePath(fileName: &str) -> String {
        // java Pair<File, Boolean> pair = Platform.createFile(fileName);
        let pair=Platform::createFile(fileName);
        // java return pair != null ? ((File)pair.first).getAbsolutePath() : "";
        pair.map(|(path,_)|if std::path::Path::new(&path).is_absolute(){path}else{std::env::current_dir().expect("File.getAbsolutePath").join(path).to_string_lossy().into_owned()}).unwrap_or_default()
    }

    // java public static void deleteAll()
    pub fn deleteAll() {
        // java int i = 0;
        let mut i = 0;
        // java for(int size = pathList.size(); i < size; ++i)
        let size = pathList.lock().unwrap().len();
        while i < size {
            // java (new File((String)pathList.get(i))).delete();
            let path = pathList.lock().unwrap()[i].clone();
            let _ = std::fs::remove_file(path);
            i += 1;
        }
    }
}
