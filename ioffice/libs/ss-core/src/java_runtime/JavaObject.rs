use super::Rc;
use std::any::Any;
// Native references for Java Object-valued external caches. These are Java
// wrapper methods; the ET classes keep their original instanceof/cast branches.
pub type ObjectRef=Rc<dyn JavaObject>;
pub type NullableObject=Option<ObjectRef>;
pub trait JavaObject:Any+Send+Sync{
    fn as_any(&self)->&dyn Any;
    fn toString(&self)->String;
    fn hashCode(&self)->i32{self.as_any()as *const dyn Any as *const()as usize as i32}
    fn equals(&self,other:&dyn JavaObject)->bool{std::ptr::eq(self.as_any(),other.as_any())}
}
fn double_bits(v:f64)->u64{if v.is_nan(){0x7ff8000000000000}else{v.to_bits()}}
impl JavaObject for f64{
    fn as_any(&self)->&dyn Any{self}
    fn toString(&self)->String{kfc::java_runtime::FloatingDecimal::FloatingDecimal::toJavaFormatString(*self)}
    // java Double.hashCode: long bits=Double.doubleToLongBits(value);return (int)(bits^(bits>>>32));
    fn hashCode(&self)->i32{let b=double_bits(*self);(b^(b>>32))as i32}
    // java Double.equals uses doubleToLongBits for both values.
    fn equals(&self,other:&dyn JavaObject)->bool{other.as_any().downcast_ref::<f64>().is_some_and(|o|double_bits(*self)==double_bits(*o))}
}
impl JavaObject for i32{
    fn as_any(&self)->&dyn Any{self}
    fn toString(&self)->String{self.to_string()}
    // java Integer.hashCode returns value; Integer.equals checks Integer type and value.
    fn hashCode(&self)->i32{*self}
    fn equals(&self,other:&dyn JavaObject)->bool{other.as_any().downcast_ref::<i32>()==Some(self)}
}
impl JavaObject for bool{
    fn as_any(&self)->&dyn Any{self}
    fn toString(&self)->String{self.to_string()}
    // java Boolean.hashCode returns value?1231:1237.
    fn hashCode(&self)->i32{if *self{1231}else{1237}}
    fn equals(&self,other:&dyn JavaObject)->bool{other.as_any().downcast_ref::<bool>()==Some(self)}
}
impl JavaObject for String{
    fn as_any(&self)->&dyn Any{self}
    fn toString(&self)->String{self.clone()}
    // java String.hashCode: h=31*h+charAt(i), over UTF16 code units.
    fn hashCode(&self)->i32{self.encode_utf16().fold(0i32,|h,c|h.wrapping_mul(31).wrapping_add(c as i32))}
    fn equals(&self,other:&dyn JavaObject)->bool{other.as_any().downcast_ref::<String>()==Some(self)}
}
