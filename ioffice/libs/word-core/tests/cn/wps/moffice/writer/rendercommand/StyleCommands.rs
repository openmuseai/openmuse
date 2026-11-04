use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::{RenderCommand,STYLE_BOLD};
use word_core::cn::wps::moffice::writer::rendercommand::StyleCommands::{emit_styled_line,StyledLine};
use word_core::cn::wps::moffice::writer::rendercommand::StyleRuns::StyledRun;
fn r(t:&str,f:&str,s:i32,b:bool,a:i32)->StyledRun{StyledRun{text:t.into(),font_id:f.into(),size_twip:s,bold:b,color:7,advance:a}}
#[test]
fn ST1_emits_style_and_baseline(){let c=emit_styled_line(&StyledLine{page:2,x:10,baseline:30,runs:vec![r("H","head",480,true,100)]});let RenderCommand::Text(t)=&c[0] else{panic!()};assert_eq!((t.page,t.x,t.y,t.size,t.font_id.as_str()),(2,10,30,480,"head"));assert_ne!(t.style&STYLE_BOLD,0);}
#[test]
fn ST2_run_x_accumulates_measured_advance(){let c=emit_styled_line(&StyledLine{page:0,x:10,baseline:0,runs:vec![r("a","a",240,false,25),r("中","b",240,false,30)]});let RenderCommand::Text(t)=&c[1] else{panic!()};assert_eq!(t.x,35);}
#[test]
fn ST3_same_style_coalesces_before_emit(){let c=emit_styled_line(&StyledLine{page:0,x:0,baseline:0,runs:vec![r("a","f",240,false,1),r("b","f",240,false,1)]});assert_eq!(c.len(),1);}
