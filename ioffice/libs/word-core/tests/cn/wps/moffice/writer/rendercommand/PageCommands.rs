use word_core::cn::wps::moffice::writer::rendercommand::PageCommands::{emit_pages,StyledPage};
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::RenderCommand;
use word_core::cn::wps::moffice::writer::rendercommand::StyleCommands::StyledLine;
use word_core::cn::wps::moffice::writer::rendercommand::StyleRuns::StyledRun;
#[test]
fn PC1_empty_document(){let d=emit_pages(&[]);assert_eq!(d.version,2);assert!(d.commands.is_empty());}
#[test]
fn PC2_page_is_bracketed_with_background(){let d=emit_pages(&[StyledPage{page:0,width:100,height:200,background:0xffffff,lines:vec![]}]);assert!(matches!(d.commands[0],RenderCommand::BeginPage(_)));assert!(matches!(d.commands[1],RenderCommand::Rect(_)));assert!(matches!(d.commands[2],RenderCommand::EndPage(_)));}
#[test]
fn PC3_multiple_pages_keep_order(){let mk=|p|StyledPage{page:p,width:1,height:1,background:0,lines:vec![]};let d=emit_pages(&[mk(0),mk(1)]);let RenderCommand::EndPage(a)=&d.commands[2]else{panic!()};let RenderCommand::BeginPage(b)=&d.commands[3]else{panic!()};assert_eq!((a.page,b.page),(0,1));}
#[test]
fn PC4_lines_are_emitted_between_background_and_end(){let run=StyledRun{text:"x".into(),font_id:"f".into(),size_twip:240,bold:false,color:0,advance:10};let line=StyledLine{page:0,x:0,baseline:20,runs:vec![run]};let d=emit_pages(&[StyledPage{page:0,width:100,height:100,background:0,lines:vec![line]}]);assert!(matches!(d.commands[2],RenderCommand::Text(_)));assert!(matches!(d.commands[3],RenderCommand::EndPage(_)));}
