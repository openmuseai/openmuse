#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

// Java: public class LogDumper
pub struct LogDumper {
    // Java field: ArrayList<String> mLogs
    mLogs: Mutex<Vec<String>>,
    // Java field: String mLogPath
    mLogPath: String,
}

// Java: public class LogDumper
impl LogDumper {
    // Java: LogDumper(String logPath)
    pub fn new(log_path: &str) -> Self {
        // Java: super();
        // Java: this.mLogPath = logPath;
        Self {
            mLogs: Mutex::new(Vec::new()),
            mLogPath: log_path.to_string(),
        }
    }

    // Java: void addLog(String log)
    pub fn addLog(&self, log: &str) {
        // Java: this.mLogs.add(log);
        let mut logs = self.mLogs.lock().unwrap();
        logs.push(log.to_string());
    }

    // Java: void dump()
    pub fn dump(&self) -> Result<(), std::io::Error> {
        let mut logs = self.mLogs.lock().unwrap();
        // Java: if (!this.mLogs.isEmpty())
        if !logs.is_empty()
        {
            // Java: FileWriter writer = null;
            // Java: try
            let result = (|| -> Result<(), std::io::Error>
            {
                // Java: writer = new FileWriter(this.mLogPath, true);
                let mut writer = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.mLogPath)?;
                // Java: for(String log : this.mLogs)
                for log in logs.iter()
                {
                    // Java: writer.write(log);
                    writer.write_all(log.as_bytes())?;
                    // Java: writer.write("\r\n");
                    writer.write_all(b"\r\n")?;
                }
                Ok(())
            })();
            // Java: finally
            {
                // Java: this.mLogs.clear();
                logs.clear();
                // Java: IOUtils.closeStream(writer);
            }
            return result;
        }
        Ok(())
    }
}
