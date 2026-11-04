use std::sync::atomic::{AtomicI32,Ordering};
use crate::{java_runtime::FileChannel::FileChannel,cn::wps::moss::util::objpool::SharedObjects::byteBuffer8};
use util::cn::wps::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
use super::KmoFileHelper::KmoFileHelper;
// java public class KmoDataPersistence {
pub struct KmoDataPersistence;
// java static int mapCount = 0;
static mapCount:AtomicI32=AtomicI32::new(0);
#[cfg(test)]
#[path="KmoDataPersistence.review.rs"]
mod review;
impl KmoDataPersistence{
    // java private static void gc() {
    pub fn gc(){
        // java ++mapCount; if (mapCount >= 10000) { mapCount = 0; System.gc(); System.runFinalization(); }
        let count=mapCount.fetch_add(1,Ordering::Relaxed).wrapping_add(1);if count>=10000{mapCount.store(0,Ordering::Relaxed);/* native ownership replaces Java GC hints */}
    }
    // java public static void clear() { KmoFileHelper.deleteAll(); mapCount = 0; }
    pub fn clear(){KmoFileHelper::deleteAll();mapCount.store(0,Ordering::Relaxed);}
    // java private static ByteBuffer getValue(int id, int blockBeginPosition, FileChannel channel, int maxCount) throws IOException {
    fn getValue(id:i32,blockBeginPosition:i32,channel:&mut FileChannel,maxCount:i32)->std::io::Result<Vec<u8>>{
        // java int INT_BYTE_NUM = 4; ByteBuffer stringBuffer = null;
        let INT_BYTE_NUM=4i32;
        // java int offsetBeIndex = blockBeginPosition + id * 4;
        let offsetBeIndex=blockBeginPosition.wrapping_add(id.wrapping_mul(INT_BYTE_NUM));
        // java int beforePo = 0; int curPo = 0;
        let mut beforePo=0;let curPo;
        // java if (id != 0) {
        if id!=0{
            // java IntBuffer sizeBuffer = channel.map(MapMode.READ_ONLY, (long)(offsetBeIndex - 4), 8L).asIntBuffer();
            let sizeBuffer=channel.map(false,offsetBeIndex.wrapping_sub(4)as i64,8)?;
            // java beforePo = sizeBuffer.get(); curPo = sizeBuffer.get();
            beforePo=i32::from_be_bytes(sizeBuffer[0..4].try_into().unwrap());curPo=i32::from_be_bytes(sizeBuffer[4..8].try_into().unwrap());
        // java } else {
        }else{
            // java IntBuffer sizeBuffer = channel.map(MapMode.READ_ONLY, (long)offsetBeIndex, 4L).asIntBuffer(); curPo = sizeBuffer.get();
            let sizeBuffer=channel.map(false,offsetBeIndex as i64,4)?;curPo=i32::from_be_bytes(sizeBuffer[0..4].try_into().unwrap());
        }
        // java stringBuffer = ByteBuffer.allocate(curPo - beforePo);
        let size=curPo.wrapping_sub(beforePo);assert!(size>=0,"IllegalArgumentException");let mut stringBuffer=vec![0;size as usize];
        // java int stringBegin = beforePo + blockBeginPosition + maxCount * 4;
        let stringBegin=beforePo.wrapping_add(blockBeginPosition).wrapping_add(maxCount.wrapping_mul(4));
        // java channel.read(stringBuffer, (long)stringBegin); stringBuffer.rewind(); return stringBuffer;
        channel.read(&mut stringBuffer,stringBegin as i64)?;Ok(stringBuffer)
    }
}
// java public static class DoubleArrayPersistence {
pub struct DoubleArrayPersistence{
    // java private int existedDoubleCount = 0;
    existedDoubleCount:i32,
    // java private FileChannel channel = KmoFileHelper.getFileChannel();
    channel:Option<FileChannel>,
}
// java private static final int DOUBLE_BYTE_NUM = 8;
const DOUBLE_BYTE_NUM:i32=8;
impl DoubleArrayPersistence{
    pub fn DoubleArrayPersistence()->Self{Self{existedDoubleCount:0,channel:KmoFileHelper::getFileChannel()}}
    // java public void closeChannel() { if (this.channel != null) { try { this.channel.close(); } catch (IOException e) { e.printStackTrace(); } } }
    pub fn closeChannel(&mut self){if let Some(channel)=&mut self.channel{if let Err(e)=channel.close(){printStackTrace(&e);}}}
    // java public boolean insertData(WDoubleArrayList dVals) {
    pub fn insertData(&mut self,dVals:&[f64])->bool{
        // java KmoDataPersistence.gc(); int dataCount = dVals.size();
        KmoDataPersistence::gc();let dataCount=dVals.len()as i32;
        // java int bePostion = this.existedDoubleCount * 8; int sizeOfRegion = dataCount * 8;
        let bePostion=self.existedDoubleCount.wrapping_mul(DOUBLE_BYTE_NUM);let sizeOfRegion=dataCount.wrapping_mul(DOUBLE_BYTE_NUM);
        // java try { if (this.channel == null) { return false; }
        let Some(channel)=&self.channel else{return false;};
        let result=(||->std::io::Result<()>{
            // java DoubleBuffer buffer = this.channel.map(MapMode.READ_WRITE, (long)bePostion, (long)sizeOfRegion).asDoubleBuffer();
            let mut buffer=channel.map(true,bePostion as i64,sizeOfRegion as i64)?;let mut position=0usize;
            // java int step = 1024; double[] bufferValue = new double[step]; int blockNum = dataCount / step;
            let step=1024i32;let mut bufferValue=vec![0.0;step as usize];let blockNum=dataCount/step;
            // java for(int i = 0; i < blockNum; ++i) { int offset = i * step;
            for i in 0..blockNum{let offset=i*step;
                // java for(int j = 0; j < step; ++j) { bufferValue[j] = dVals.getQuick(offset + j); }
                for j in 0..step{bufferValue[j as usize]=dVals[(offset+j)as usize];}
                // java buffer.put(bufferValue);
                for value in &bufferValue{buffer[position..position+8].copy_from_slice(&value.to_be_bytes());position+=8;}
            }
            // java int bufferOffset = -1;
            let mut bufferOffset=-1i32;
            // java for(int i = blockNum * step; i < dataCount; ++i) { ++bufferOffset; bufferValue[bufferOffset] = dVals.getQuick(i); }
            for i in blockNum*step..dataCount{bufferOffset+=1;bufferValue[bufferOffset as usize]=dVals[i as usize];}
            // java if (bufferOffset != -1) { double[] remainValue = new double[bufferOffset + 1]; System.arraycopy(bufferValue, 0, remainValue, 0, remainValue.length); buffer.put(remainValue); }
            if bufferOffset!=-1{let remainValue=bufferValue[..bufferOffset as usize+1].to_vec();for value in remainValue{buffer[position..position+8].copy_from_slice(&value.to_be_bytes());position+=8;}}
            Ok(())
        })();
        // java } catch (IOException e) { e.printStackTrace(); return false; }
        if let Err(e)=result{printStackTrace(&e);return false;}
        // java this.existedDoubleCount += dataCount; return true;
        self.existedDoubleCount=self.existedDoubleCount.wrapping_add(dataCount);true
    }
    // java public double queryData(int id) {
    pub fn queryData(&mut self,id:i32)->f64{
        // java KmoDataPersistence.gc(); ByteBuffer buffer = SharedObjects.byteBuffer8.borrowObject(); double d = (double)0.0F;
        KmoDataPersistence::gc();let buffer=byteBuffer8.borrowObject();let d=0.0;
        // java try { if (this.channel != null) { this.channel.read(buffer, (long)(id * 8)); buffer.clear(); d = buffer.getDouble(); buffer.clear(); return d; } return d;
        let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||->std::io::Result<f64>{if let Some(channel)=&mut self.channel{let mut buffer=buffer.lock().unwrap_or_else(|e|e.into_inner());let begin=buffer.position;let n=channel.read(&mut buffer.bytes[begin..],id.wrapping_mul(8)as i64)?;buffer.position+=n;buffer.position=0;let d=f64::from_be_bytes(buffer.bytes);buffer.position=0;Ok(d)}else{Ok(d)}}));
        // java } catch (Throwable ex) { ex.printStackTrace(); return d; } finally { SharedObjects.byteBuffer8.returnObject(buffer); }
        let value=match result{Ok(Ok(v))=>v,Ok(Err(e))=>{printStackTrace(&e);d},Err(_)=>d};byteBuffer8.returnObject(Some(buffer));value
    }
}
// java public static class ByteArrayPersistence {
pub struct ByteArrayPersistence{
    // java private WIntArrayList blocksSize = new WIntArrayList();
    blocksSize:WIntArrayList,
    // java private WIntArrayList blockByteNums = new WIntArrayList();
    blockByteNums:WIntArrayList,
    // java private int curSizeAll = 0;
    curSizeAll:i32,
    // java private int existedSQLCount = 0;
    existedSQLCount:i32,
    // java private FileChannel channel = KmoFileHelper.getFileChannel();
    channel:Option<FileChannel>,
}
impl ByteArrayPersistence{
    pub fn ByteArrayPersistence()->Self{Self{blocksSize:WIntArrayList::new(),blockByteNums:WIntArrayList::new(),curSizeAll:0,existedSQLCount:0,channel:KmoFileHelper::getFileChannel()}}
    // java public void closeChannel() { if (this.channel != null) { try { this.channel.close(); } catch (IOException e) { e.printStackTrace(); } } }
    pub fn closeChannel(&mut self){if let Some(channel)=&mut self.channel{if let Err(e)=channel.close(){printStackTrace(&e);}}}
    // java public boolean insertData(List<byte[]> metadataList) {
    pub fn insertData(&mut self,metadataList:&[Vec<u8>])->bool{
        // java KmoDataPersistence.gc(); int INT_BYTE_NUM = 4; int sizeAll = 0; int metadataSize = metadataList.size();
        KmoDataPersistence::gc();let INT_BYTE_NUM=4i32;let metadataSize=metadataList.len()as i32;
        // java int[] indexArray = this.getHeadIndexArray(metadataList); sizeAll = indexArray[metadataSize - 1];
        let indexArray=self.getHeadIndexArray(metadataList);let sizeAll=indexArray[metadataSize as usize-1];
        // java int indexBeginPostion = this.curSizeAll; int sizeOfHeadIndex = metadataSize * 4; int contentBeginIndex = indexBeginPostion + sizeOfHeadIndex;
        let indexBeginPostion=self.curSizeAll;let sizeOfHeadIndex=metadataSize.wrapping_mul(INT_BYTE_NUM);let contentBeginIndex=indexBeginPostion.wrapping_add(sizeOfHeadIndex);
        // java try { if (this.channel == null) { return false; }
        let Some(channel)=&self.channel else{return false;};
        let result=(||->std::io::Result<()>{
            // java IntBuffer intSizeBuffer = this.channel.map(MapMode.READ_WRITE, (long)indexBeginPostion, (long)sizeOfHeadIndex).asIntBuffer(); intSizeBuffer.put(indexArray);
            let mut intSizeBuffer=channel.map(true,indexBeginPostion as i64,sizeOfHeadIndex as i64)?;for(i,value)in indexArray.iter().enumerate(){intSizeBuffer[i*4..i*4+4].copy_from_slice(&value.to_be_bytes());}
            // java int[] var14 = null;
            drop(intSizeBuffer);
            // java MappedByteBuffer byteBuffer = this.channel.map(MapMode.READ_WRITE, (long)contentBeginIndex, (long)sizeAll);
            let mut byteBuffer=channel.map(true,contentBeginIndex as i64,sizeAll as i64)?;let mut position=0usize;
            // java for(int i = 0; i < metadataSize; ++i) { byteBuffer.put((byte[])metadataList.get(i)); }
            for i in 0..metadataSize{let bytes=&metadataList[i as usize];byteBuffer[position..position+bytes.len()].copy_from_slice(bytes);position+=bytes.len();}Ok(())
        })();
        // java } catch (IOException e) { e.printStackTrace(); return false; }
        if let Err(e)=result{printStackTrace(&e);return false;}
        // java int curBlockNum = sizeAll + metadataSize * 4; this.curSizeAll += curBlockNum;
        let curBlockNum=sizeAll.wrapping_add(metadataSize.wrapping_mul(4));self.curSizeAll=self.curSizeAll.wrapping_add(curBlockNum);
        // java this.blocksSize.add(this.existedSQLCount + metadataSize); this.existedSQLCount += metadataSize; this.blockByteNums.add(this.curSizeAll); return true;
        self.blocksSize.add(self.existedSQLCount.wrapping_add(metadataSize));self.existedSQLCount=self.existedSQLCount.wrapping_add(metadataSize);self.blockByteNums.add(self.curSizeAll);true
    }
    // java private int[] getHeadIndexArray(List<byte[]> metadataList) {
    fn getHeadIndexArray(&self,metadataList:&[Vec<u8>])->Vec<i32>{
        // java int metadataSize = metadataList.size(); int[] indexArray = new int[metadataSize]; indexArray[0] = ((byte[])metadataList.get(0)).length;
        let metadataSize=metadataList.len();let mut indexArray=vec![0i32;metadataSize];let first=metadataList.first().expect("IndexOutOfBoundsException");indexArray[0]=first.len()as i32;
        // java for(int i = 1; i < metadataSize; ++i) { indexArray[i] = ((byte[])metadataList.get(i)).length + indexArray[i - 1]; } return indexArray;
        for i in 1..metadataSize{indexArray[i]=(metadataList[i].len()as i32).wrapping_add(indexArray[i-1]);}indexArray
    }
    // java public byte[] queryData(int id) {
    pub fn queryData(&mut self,id:i32)->Option<Vec<u8>>{
        // java KmoDataPersistence.gc(); int blockIndex = this.getBlockIndex(id); int blockBeginPosition = this.getBlockOffset(blockIndex); ByteBuffer stringBuffer = null;
        KmoDataPersistence::gc();let blockIndex=self.getBlockIndex(id);let blockBeginPosition=self.getBlockOffset(blockIndex);
        // java int maxCount = this.blocksSize.get(blockIndex); int offsetId = id;
        let mut maxCount=self.blocksSize.get(blockIndex);let mut offsetId=id;
        // java if (blockIndex > 0) { int prMaxCount = this.blocksSize.get(blockIndex - 1); maxCount -= prMaxCount; offsetId = id - prMaxCount; }
        if blockIndex>0{let prMaxCount=self.blocksSize.get(blockIndex-1);maxCount=maxCount.wrapping_sub(prMaxCount);offsetId=id.wrapping_sub(prMaxCount);}
        // java try { if (this.channel == null) { return null; } else { stringBuffer = KmoDataPersistence.getValue(offsetId, blockBeginPosition, this.channel, maxCount); return stringBuffer.array(); } } catch (IOException e) { e.printStackTrace(); return null; }
        let channel=self.channel.as_mut()?;match KmoDataPersistence::getValue(offsetId,blockBeginPosition,channel,maxCount){Ok(v)=>Some(v),Err(e)=>{printStackTrace(&e);None}}
    }
    // java private int getBlockIndex(int id) { int index = 0; int i = 0; for(int size = this.blocksSize.size(); i < size; ++i) { if (id < this.blocksSize.get(i)) { index = i; break; } } return index; }
    fn getBlockIndex(&self,id:i32)->i32{let mut index=0;for i in 0..self.blocksSize.size(){if id<self.blocksSize.get(i){index=i;break;}}index}
    // java private int getBlockOffset(int blockIndex) { int offset = 0; if (blockIndex > 0) { offset = this.blockByteNums.get(blockIndex - 1); } return offset; }
    fn getBlockOffset(&self,blockIndex:i32)->i32{let mut offset=0;if blockIndex>0{offset=self.blockByteNums.get(blockIndex-1);}offset}
}
// Native Throwable.printStackTrace leaf; host stack frames differ from Android.
fn printStackTrace(error:&std::io::Error){eprintln!("{error}\n{}",std::backtrace::Backtrace::capture());}
