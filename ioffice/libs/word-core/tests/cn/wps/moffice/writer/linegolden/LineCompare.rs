use word_core::cn::wps::moffice::writer::cache::Stage2Rows::Stage2Line;
use word_core::cn::wps::moffice::writer::linegolden::LineCompare::compare_lines;
fn r()->Stage2Line{Stage2Line{page:0,line:0,start_cp:0,end_cp:2,x:10,y:20,width:30,height:40,xpos:vec![]}}
#[test]
fn LC1_equal(){assert!(compare_lines(vec![r()],vec![r()],0).is_ok());}
#[test]
fn LC2_cp_is_exact(){let mut a=r();a.end_cp=3;let d=compare_lines(vec![r()],vec![a],99).unwrap_err();assert_eq!(d.field,"end_cp");}
#[test]
fn LC3_geometry_uses_tolerance(){let mut a=r();a.x+=1;assert!(compare_lines(vec![r()],vec![a.clone()],1).is_ok());a.x+=1;assert_eq!(compare_lines(vec![r()],vec![a],1).unwrap_err().field,"x");}
#[test]
fn LC4_first_difference_reported(){let mut a=r();a.page=1;a.start_cp=9;a.end_cp=10;let d=compare_lines(vec![r()],vec![a],0).unwrap_err();assert_eq!(d.field,"page");}
#[test]
fn LC5_count_difference(){assert_eq!(compare_lines(vec![r()],vec![],0).unwrap_err().field,"line-count");}
#[test]
fn LC6_invalid_expected_is_named(){let mut a=r();a.end_cp=-1;assert_eq!(compare_lines(vec![a],vec![r()],0).unwrap_err().field,"expected-invalid");}
#[test]
fn LC7_invalid_actual_is_named(){let mut a=r();a.height=-1;assert_eq!(compare_lines(vec![r()],vec![a],0).unwrap_err().field,"actual-invalid");}
