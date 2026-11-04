// Java: import java.io.BufferedReader
// Java: import java.io.BufferedWriter
// Java: import java.io.File
// Java: import java.io.FileReader
// Java: import java.io.FileWriter
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.List
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::fs::File;
use std::path::Path;

// Java: public class FontCacheFile
pub struct FontCacheFile {
    // Java field: float version
    pub(crate) version: f32,
    // Java field: List<String> fullNames
    pub(crate) fullNames: Vec<String>,
    // Java field: List<String> paths
    pub(crate) paths: Vec<String>,
}

// Java: public class FontCacheFile
impl FontCacheFile {
    // Java: FontCacheFile()
    pub fn new() -> Self {
        // Java: super();
        FontCacheFile {
            version: 0.0,
            fullNames: Vec::new(),
            paths: Vec::new(),
        }
    }

    // Java: float getVersion()
    pub fn getVersion(&self) -> f32 {
        // Java: return this.version;
        self.version
    }

    // Java: void setVersion(float version)
    pub fn setVersion(&mut self, version: f32) {
        // Java: this.version = version;
        self.version = version;
    }

    // Java: List<String> getFullNames()
    pub fn getFullNames(&self) -> &Vec<String> {
        // Java: return this.fullNames;
        &self.fullNames
    }

    // Java: void setFullNames(List<String> fullNames)
    pub fn setFullNames(&mut self, full_names: Vec<String>) {
        // Java: this.fullNames = fullNames;
        self.fullNames = full_names;
    }

    // Java: List<String> getPaths()
    pub fn getPaths(&self) -> &Vec<String> {
        // Java: return this.paths;
        &self.paths
    }

    // Java: void setPaths(List<String> paths)
    pub fn setPaths(&mut self, paths: Vec<String>) {
        // Java: this.paths = paths;
        self.paths = paths;
    }

    // Java: static FontCacheFile parse(File file)
    pub fn parse(file: &Path) -> Option<FontCacheFile> {
        // Java: FontCacheFile cache = new FontCacheFile();
        let mut cache = FontCacheFile::new();
        // Java: BufferedReader reader = new BufferedReader(new FileReader(file));
        let f = File::open(file).ok()?;
        let reader = BufReader::new(f);
        let mut lines = reader.lines();
        // Java: try
        let result = {
            // Java: String line = reader.readLine();
            let line = lines.next();
            let line = match line {
                Some(Ok(l)) => l,
                _ => return None,
            };
            // Java: if (line != null && line.startsWith("Version"))
            if line.starts_with("Version") {
                // Java: float version = Float.parseFloat(line.substring("Version".length()));
                let version: f32 = line["Version".len()..].parse().unwrap_or(0.0);
                // Java: if (version < 5.0F)
                if version < 5.0 {
                    // Java: Object var14 = null;
                    // Java: return (FontCacheFile)var14;
                    None
                } else {
                    // Java: cache.version = version;
                    cache.version = version;
                    // Java: while((line = reader.readLine()) != null)
                    for line_result in lines {
                        let line = match line_result {
                            Ok(l) => l,
                            Err(_) => break,
                        };
                        // Java: if (line.length() > 0 && line.charAt(0) != '#')
                        if !line.is_empty() && line.as_bytes()[0] != b'#' {
                            // Java: String[] parts = line.split("\t");
                            let parts: Vec<&str> = line.split('\t').collect();
                            // Java: if (parts != null && parts.length >= 2)
                            if parts.len() >= 2 {
                                // Java: String fullNames = parts[0];
                                let full_names = parts[0].to_string();
                                // Java: String fontPath = parts[1];
                                let font_path = parts[1].to_string();
                                // Java: cache.fullNames.add(fullNames);
                                cache.fullNames.push(full_names);
                                // Java: cache.paths.add(fontPath);
                                cache.paths.push(font_path);
                            }
                        }
                    }
                    // Java: FontCacheFile var13 = cache;
                    // Java: return var13;
                    Some(cache)
                }
            } else {
                // Java: return null;
                None
            }
        };
        // Java: finally
        {
            // Java: reader.close();
        }
        result
    }

    // Java: void save(File file)
    pub fn save(&self, file: &Path) -> std::io::Result<()> {
        // Java: FileWriter fwriter = new FileWriter(file);
        let fwriter = File::create(file)?;
        // Java: BufferedWriter writer = new BufferedWriter(fwriter);
        let mut writer = BufWriter::new(fwriter);
        // Java: try
        {
            // Java: writer.append("Version").append(String.format("%.1f", this.version)).append("\n");
            write!(writer, "Version{:.1}\n", self.version)?;
            // Java: for(int i = 0; i < this.fullNames.size(); ++i)
            for i in 0..self.fullNames.len() {
                // Java: writer.append((CharSequence)this.fullNames.get(i)).append("\t");
                write!(writer, "{}\t", self.fullNames[i])?;
                // Java: writer.append((CharSequence)this.paths.get(i)).append("\n");
                write!(writer, "{}\n", self.paths[i])?;
            }
        }
        // Java: finally
        {
            // Java: writer.flush();
            writer.flush()?;
            // Java: fwriter.flush();
            // Java: writer.close();
        }
        Ok(())
    }
}
