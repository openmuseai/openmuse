use word_core::cn::wps::moffice::writer::cache::Stage2Rows::Stage2Line;
use word_core::cn::wps::moffice::writer::linegolden::LineNormalize::normalize_lines;
fn r(p:i32,l:i32,s:i32,e:i32)->Stage2Line{Stage2Line{page:p,line:l,start_cp:s,end_cp:e,x:0,y:0,width:1,height:1,xpos:vec![]}}
#[test]
fn LN1_sorts_page_line(){let v=normalize_lines(vec![r(1,0,1,2),r(0,0,0,1)]).unwrap();assert_eq!(v[0].page,0);}
#[test]
fn LN2_rejects_bad_cp(){assert!(normalize_lines(vec![r(0,0,2,1)]).is_err());}
#[test]
fn LN3_rejects_duplicate_slot(){assert!(normalize_lines(vec![r(0,0,0,1),r(0,0,1,2)]).is_err());}
#[test]
fn LN4_empty_is_valid(){assert!(normalize_lines(vec![]).unwrap().is_empty());}
#[test]
fn LN5_rejects_negative_geometry_and_indices(){let mut a=r(0,0,0,1);a.width=-1;assert!(normalize_lines(vec![a]).is_err());assert!(normalize_lines(vec![r(-1,0,0,1)]).is_err());}
