use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;
pub fn fold(v:impl IntoIterator<Item=i32>)->i32{v.into_iter().fold(1i32,|h,v|h.wrapping_mul(31).wrapping_add(v))}
pub fn string(v:&str)->i32{v.encode_utf16().fold(0i32,|h,v|h.wrapping_mul(31).wrapping_add(v as i32))}
pub fn bits(v:f64)->u64{if v.is_nan(){0x7ff8000000000000}else{v.to_bits()}}
pub fn double(v:f64)->i32{let v=bits(v);(v^(v>>32))as i32}
pub fn boolean(v:bool)->i32{if v{1231}else{1237}}
pub fn formulas_eq(a:&Option<Vec<Ptg>>,b:&Option<Vec<Ptg>>)->bool{match(a,b){(None,None)=>true,(Some(a),Some(b))=>a.len()==b.len()&&a.iter().zip(b).all(|(a,b)|a.javaEquals(b)),_=>false}}
pub fn formulas_hash(a:&Option<Vec<Ptg>>)->i32{a.as_ref().map_or(0,|v|fold(v.iter().map(|p|p.javaHashCode())))}
