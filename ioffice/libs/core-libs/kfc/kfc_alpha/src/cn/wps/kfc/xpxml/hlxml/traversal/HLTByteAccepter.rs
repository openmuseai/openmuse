// java public interface HLTByteAccepter { void element(byte[] chars,int start,int end); void characters(byte[] chars,int start,int end); }
pub trait HLTByteAccepter:Send+Sync {fn element(&self,chars:&[u8],start:usize,end:usize);fn characters(&self,chars:&[u8],start:usize,end:usize);}
