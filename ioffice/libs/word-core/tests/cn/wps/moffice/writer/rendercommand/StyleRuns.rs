use word_core::cn::wps::moffice::writer::rendercommand::StyleRuns::{coalesce_style_runs,StyledRun};
fn r(text:&str,font:&str,size:i32,bold:bool,advance:i32)->StyledRun{StyledRun{text:text.into(),font_id:font.into(),size_twip:size,bold,color:0,advance}}
#[test]
fn SR1_same_style_coalesces(){let x=coalesce_style_runs(&[r("a","f",240,false,10),r("b","f",240,false,20)]);assert_eq!(x.len(),1);assert_eq!((x[0].text.as_str(),x[0].advance),("ab",30));}
#[test]
fn SR2_bold_boundary_is_preserved(){assert_eq!(coalesce_style_runs(&[r("a","f",240,false,1),r("b","f",240,true,1)]).len(),2);}
#[test]
fn SR3_heading_size_boundary_is_preserved(){assert_eq!(coalesce_style_runs(&[r("H","f",480,false,1),r("b","f",240,false,1)]).len(),2);}
#[test]
fn SR4_font_boundary_is_preserved(){assert_eq!(coalesce_style_runs(&[r("a","ascii",240,false,1),r("中","east",240,false,1)]).len(),2);}
#[test]
fn SR5_empty_runs_are_dropped(){assert!(coalesce_style_runs(&[r("","f",240,false,0)]).is_empty());}
