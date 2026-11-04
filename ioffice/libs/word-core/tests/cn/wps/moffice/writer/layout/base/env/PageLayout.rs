use word_core::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use word_core::cn::wps::moffice::writer::layout::base::env::PageLayout::paginate;
use word_core::cn::wps::moffice::writer::layout::base::env::WidowControl::PageLine;

fn l(cp: i32, h: i32, para: i32, force: bool) -> PageLine { let mut line=TypoTextLine::new(); line.setStartCP(cp); line.setEndCP(cp+1); line.setTextRectHeight(h); PageLine{line,para_id:para,widow_control:false,force_page_after:force,cell:None} }
#[test]
fn PL1_empty_is_zero_pages(){ assert!(paginate(vec![],100).is_empty()); }
#[test]
fn PL2_height_paginates_and_sets_y(){ let p=paginate(vec![l(0,60,1,false),l(1,60,2,false)],100); assert_eq!(p.len(),2); assert_eq!(p[0].lines[0].line.getYPos(),0); }
#[test]
fn PL3_form_feed_finishes_page(){ let p=paginate(vec![l(0,10,1,true),l(1,10,2,false)],100); assert_eq!(p.len(),2); }
#[test]
fn PL4_last_line_flag_only_on_page_end(){ let p=paginate(vec![l(0,40,1,false),l(1,40,2,false)],100); assert!(!p[0].lines[0].line.isLastLineOfPage__self()); assert!(p[0].lines[1].line.isLastLineOfPage__self()); }
#[test]
fn PL5_oversized_line_does_not_create_empty_page(){ let p=paginate(vec![l(0,120,1,false)],100); assert_eq!(p.len(),1); assert_eq!(p[0].lines.len(),1); }
