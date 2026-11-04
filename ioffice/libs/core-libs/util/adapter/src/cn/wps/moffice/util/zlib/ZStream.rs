// Java field: int MAX_WBITS
const MAX_WBITS: i32 = 15;
// Java field: int DEF_WBITS
const DEF_WBITS: i32 = 15;

// Java: public class ZStream
pub struct ZStream {
    // Java field: byte[] next_in
    pub next_in: Option<Vec<u8>>,
    // Java field: int next_in_index
    pub next_in_index: i32,
    // Java field: int avail_in
    pub avail_in: i32,
    // Java field: long total_in
    pub total_in: i64,
    // Java field: byte[] next_out
    pub next_out: Option<Vec<u8>>,
    // Java field: int next_out_index
    pub next_out_index: i32,
    // Java field: int avail_out
    pub avail_out: i32,
    // Java field: long total_out
    pub total_out: i64,
    // Java field: String msg
    pub msg: Option<String>,
    // Java field: Deflate dstate
    pub(crate) dstate: Option<Box<dyn std::any::Any>>,
    // Java field: Inflate istate
    pub(crate) istate: Option<Box<dyn std::any::Any>>,
    // Java field: int data_type
    pub(crate) data_type: i32,
    // Java field: long adler
    pub adler: i64,
    // Java field: Adler32 _adler
    pub(crate) _adler: Option<Box<dyn std::any::Any>>,
}

// Java: public class ZStream
impl ZStream {
    // Java: ZStream()
    pub fn new() -> Self {
        // Java: super();
        ZStream {
            next_in: None,
            next_in_index: 0,
            avail_in: 0,
            total_in: 0,
            next_out: None,
            next_out_index: 0,
            avail_out: 0,
            total_out: 0,
            msg: None,
            dstate: None,
            istate: None,
            data_type: 0,
            adler: 0,
            _adler: None,
        }
    }

    // Java: int inflateInit()
    pub fn inflateInit(&mut self) -> i32 {
        // Java: return this.inflateInit(15);
        self.inflateInit_w(15)
    }

    // Java: int inflateInit(boolean nowrap)
    pub fn inflateInit_nowrap(&mut self, _nowrap: bool) -> i32 {
        // Java: return this.inflateInit(15, nowrap);
        self.inflateInit_w_nowrap(15, _nowrap)
    }

    // Java: int inflateInit(int w)
    pub fn inflateInit_w(&mut self, _w: i32) -> i32 {
        // Java: return this.inflateInit(w, false);
        self.inflateInit_w_nowrap(_w, false)
    }

    // Java: int inflateInit(int w, boolean nowrap)
    pub fn inflateInit_w_nowrap(&mut self, _w: i32, _nowrap: bool) -> i32 {
        // Java: this.istate = new Inflate();
        // Java: return this.istate.inflateInit(this, nowrap ? -w : w);
        todo!()
    }

    // Java: int inflate(int f)
    pub fn inflate(&mut self, _f: i32) -> i32 {
        // Java: return this.istate == null ? -2 : this.istate.inflate(this, f);
        todo!()
    }

    // Java: int inflateEnd()
    pub fn inflateEnd(&mut self) -> i32 {
        // Java: if (this.istate == null)
        {
            // Java: return -2;
        }
        // Java: else
        {
            // Java: int ret = this.istate.inflateEnd(this);
            // Java: this.istate = null;
            // Java: return ret;
        }
        todo!()
    }

    // Java: int inflateSync()
    pub fn inflateSync(&mut self) -> i32 {
        // Java: return this.istate == null ? -2 : this.istate.inflateSync(this);
        todo!()
    }

    // Java: int inflateSetDictionary(byte[] dictionary, int dictLength)
    pub fn inflateSetDictionary(&mut self, _dictionary: &[u8], _dictLength: i32) -> i32 {
        // Java: return this.istate == null ? -2 : this.istate.inflateSetDictionary(this, dictionary, dictLength);
        todo!()
    }

    // Java: int deflateInit(int level)
    pub fn deflateInit(&mut self, _level: i32) -> i32 {
        // Java: return this.deflateInit(level, 15);
        self.deflateInit_bits(_level, 15)
    }

    // Java: int deflateInit(int level, boolean nowrap)
    pub fn deflateInit_nowrap(&mut self, _level: i32, _nowrap: bool) -> i32 {
        // Java: return this.deflateInit(level, 15, nowrap);
        self.deflateInit_bits_nowrap(_level, 15, _nowrap)
    }

    // Java: int deflateInit(int level, int bits)
    pub fn deflateInit_bits(&mut self, _level: i32, _bits: i32) -> i32 {
        // Java: return this.deflateInit(level, bits, false);
        self.deflateInit_bits_nowrap(_level, _bits, false)
    }

    // Java: int deflateInit(int level, int bits, boolean nowrap)
    pub fn deflateInit_bits_nowrap(&mut self, _level: i32, _bits: i32, _nowrap: bool) -> i32 {
        // Java: this.dstate = new Deflate();
        // Java: return this.dstate.deflateInit(this, level, nowrap ? -bits : bits);
        todo!()
    }

    // Java: int deflate(int flush)
    pub fn deflate(&mut self, _flush: i32) -> i32 {
        // Java: return this.dstate == null ? -2 : this.dstate.deflate(this, flush);
        todo!()
    }

    // Java: int deflateEnd()
    pub fn deflateEnd(&mut self) -> i32 {
        // Java: if (this.dstate == null)
        {
            // Java: return -2;
        }
        // Java: else
        {
            // Java: int ret = this.dstate.deflateEnd();
            // Java: this.dstate = null;
            // Java: return ret;
        }
        todo!()
    }

    // Java: int deflateParams(int level, int strategy)
    pub fn deflateParams(&mut self, _level: i32, _strategy: i32) -> i32 {
        // Java: return this.dstate == null ? -2 : this.dstate.deflateParams(this, level, strategy);
        todo!()
    }

    // Java: int deflateSetDictionary(byte[] dictionary, int dictLength)
    pub fn deflateSetDictionary(&mut self, _dictionary: &[u8], _dictLength: i32) -> i32 {
        // Java: return this.dstate == null ? -2 : this.dstate.deflateSetDictionary(this, dictionary, dictLength);
        todo!()
    }

    // Java: void flush_pending()
    pub fn flush_pending(&mut self) {
        // Java: int len = this.dstate.pending;
        // Java: if (len > this.avail_out)
        {
            // Java: len = this.avail_out;
        }
        // Java: if (len != 0)
        {
            // Java: if (this.dstate.pending_buf.length <= this.dstate.pending_out || this.next_out.length <= this.next_out_index || this.dstate.pending_buf.length < this.dstate.pending_out + len || this.next_out.length < this.next_out_index + len)
            {
                // Java: System.out.println(this.dstate.pending_buf.length + ", " + this.dstate.pending_out + ", " + this.next_out.length + ", " + this.next_out_index + ", " + len);
                // Java: System.out.println("avail_out=" + this.avail_out);
            }
            // Java: System.arraycopy(this.dstate.pending_buf, this.dstate.pending_out, this.next_out, this.next_out_index, len);
            // Java: this.next_out_index += len;
            // Java: Deflate var10000 = this.dstate;
            // Java: var10000.pending_out += len;
            // Java: this.total_out += (long)len;
            // Java: this.avail_out -= len;
            // Java: var10000 = this.dstate;
            // Java: var10000.pending -= len;
            // Java: if (this.dstate.pending == 0)
            {
                // Java: this.dstate.pending_out = 0;
            }
        }
        todo!()
    }

    // Java: int read_buf(byte[] buf, int start, int size)
    pub fn read_buf(&mut self, _buf: &mut [u8], _start: i32, _size: i32) -> i32 {
        // Java: int len = this.avail_in;
        // Java: if (len > size)
        {
            // Java: len = size;
        }
        // Java: if (len == 0)
        {
            // Java: return 0;
        }
        // Java: else
        {
            // Java: this.avail_in -= len;
            // Java: if (this.dstate.noheader == 0)
            {
                // Java: this.adler = this._adler.adler32(this.adler, this.next_in, this.next_in_index, len);
            }
            // Java: System.arraycopy(this.next_in, this.next_in_index, buf, start, len);
            // Java: this.next_in_index += len;
            // Java: this.total_in += (long)len;
            // Java: return len;
        }
        todo!()
    }

    // Java: void free()
    pub fn free(&mut self) {
        // Java: this.next_in = null;
        self.next_in = None;
        // Java: this.next_out = null;
        self.next_out = None;
        // Java: this.msg = null;
        self.msg = None;
        // Java: this._adler = null;
        self._adler = None;
    }

}
