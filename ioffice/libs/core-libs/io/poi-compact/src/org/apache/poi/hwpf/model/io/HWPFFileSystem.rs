// Java: import cn.wps.ole.IOLEDocument
// Java: import cn.wps.ole.IOLEStorage
// Java: import cn.wps.ole.IOLEStream
// Java: import cn.wps.ole.OLEDocumentFactory
use std::collections::HashMap;

pub trait IOLEStream {
    fn write(&mut self, buf: &[u8]);
    fn close(&mut self);
}

pub trait IOLEStorage {
    fn setStorageClsid(&mut self, clsid: &[u8]);
    fn createStream(&mut self, name: &str) -> Box<dyn IOLEStream>;
    fn createStorage(&mut self, name: &str) -> Box<dyn IOLEStorage>;
    fn close(&mut self);
}

pub trait IOLEDocument {
    fn getRootStorage(&mut self) -> Box<dyn IOLEStorage>;
    fn close(&mut self);
    fn getBuffers(&mut self) -> Vec<u8>;
}

pub struct HWPFOutputStream {
    stream: Option<Box<dyn IOLEStream>>,
}

impl HWPFOutputStream {
    pub fn new(stream: Box<dyn IOLEStream>) -> Self {
        HWPFOutputStream { stream: Some(stream) }
    }

    pub fn write(&mut self, buf: &[u8]) {
        if let Some(s) = self.stream.as_mut() {
            s.write(buf);
        }
    }

    pub fn close(&mut self) {
        if let Some(s) = self.stream.as_mut() {
            s.close();
        }
    }
}

static DOC_ROOT_ENTRY_CLSID: &[u8] = &[6, 9, 2, 0, 0, 0, 0, 0, 0xC0, 0, 0, 0, 0, 0, 0, 70];

// Java: public class HWPFFileSystem
pub struct HWPFFileSystem {
    // Java field: IOLEDocument mDocument
    pub(crate) mDocument: Box<dyn IOLEDocument>,
    // Java field: String mPath
    pub(crate) mPath: String,
    // Java field: IOLEStorage mRootStorage
    pub(crate) mRootStorage: Box<dyn IOLEStorage>,
    // Java field: HashMap<String, HWPFOutputStream> _streams
    _streams: HashMap<String, HWPFOutputStream>,
    // Java field: byte[] DOC_ROOT_ENTRY_CLSID
}

// Java: public class HWPFFileSystem
impl HWPFFileSystem {
    // Java: HWPFFileSystem(String path)
    pub fn new(path: String, doc_factory: fn(&str, i32) -> Box<dyn IOLEDocument>) -> Self {
        // Java: super();
        // Java: this.mPath = path;
        // Java: this.mDocument = OLEDocumentFactory.create(path, 2);
        let mut document = doc_factory(&path, 2);
        // Java: this.mRootStorage = this.mDocument.getRootStorage();
        let mut root_storage = document.getRootStorage();
        // Java: this.mRootStorage.setStorageClsid(DOC_ROOT_ENTRY_CLSID);
        root_storage.setStorageClsid(DOC_ROOT_ENTRY_CLSID);
        // Java: String[] streams = new String[]{"WordDocument", "0Table", "Data"};
        let stream_names = ["WordDocument", "0Table", "Data"];
        let mut _streams = HashMap::new();
        // Java: for(int i = 0; i < streams.length; ++i)
        for i in 0..stream_names.len() {
            // Java: String name = streams[i];
            let name = stream_names[i];
            // Java: IOLEStream pStream = this.mRootStorage.createStream(name);
            let pStream = root_storage.createStream(name);
            // Java: this._streams.put(name, new HWPFOutputStream(pStream));
            _streams.insert(name.to_string(), HWPFOutputStream::new(pStream));
        }
        HWPFFileSystem {
            mDocument: document,
            mPath: path,
            mRootStorage: root_storage,
            _streams,
        }
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: for(HWPFOutputStream stream : this._streams.values())
        for (_name, stream) in self._streams.iter_mut() {
            // Java: stream.close();
            stream.close();
        }
        // Java: this.mRootStorage.close();
        self.mRootStorage.close();
        // Java: this.mDocument.close();
        self.mDocument.close();
    }

    // Java: HWPFOutputStream getStream(String name)
    pub fn getStream(&mut self, name: &str) -> Option<&mut HWPFOutputStream> {
        // Java: return (HWPFOutputStream)this._streams.get(name);
        self._streams.get_mut(name)
    }

    // Java: String getPath()
    pub fn getPath(&self) -> &str {
        // Java: return this.mPath;
        &self.mPath
    }

    // Java: HWPFOutputStream createStream(String name)
    pub fn createStream_root(&mut self, name: &str) -> &mut HWPFOutputStream {
        // Java: return this.createStream(name, this.mRootStorage);
        if !self._streams.contains_key(name) {
            let pStream = self.mRootStorage.createStream(name);
            self._streams.insert(name.to_string(), HWPFOutputStream::new(pStream));
        }
        self._streams.get_mut(name).unwrap()
    }

    // Java: IOLEStorage createStorage(String name)
    pub fn createStorage(&mut self, name: &str) -> Box<dyn IOLEStorage> {
        // Java: return this.mRootStorage.createStorage(name);
        self.mRootStorage.createStorage(name)
    }

    // Java: HWPFOutputStream createStream(String name, IOLEStorage storage)
    pub fn createStream_with_storage(&mut self, name: &str, storage: &mut dyn IOLEStorage) -> &mut HWPFOutputStream {
        // Java: HWPFOutputStream stream = this.getStream(name);
        // Java: if (stream == null)
        if !self._streams.contains_key(name) {
            // Java: IOLEStream pStream = storage.createStream(name);
            let pStream = storage.createStream(name);
            // Java: stream = new HWPFOutputStream(pStream);
            // Java: this._streams.put(name, stream);
            self._streams.insert(name.to_string(), HWPFOutputStream::new(pStream));
        }
        // Java: return stream;
        self._streams.get_mut(name).unwrap()
    }

    // Java: byte[] getBuffers()
    pub fn getBuffers(&mut self) -> Vec<u8> {
        // Java: for(HWPFOutputStream stream : this._streams.values())
        for (_name, stream) in self._streams.iter_mut() {
            // Java: stream.close();
            stream.close();
        }
        // Java: this.mRootStorage.close();
        self.mRootStorage.close();
        // Java: return this.mDocument.getBuffers();
        self.mDocument.getBuffers()
    }
}
