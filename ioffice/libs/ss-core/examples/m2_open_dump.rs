use ss_core::api::SpreadsheetSession::{SpreadsheetSession,SpreadsheetLoadConfig};
fn main(){let path=std::env::args().nth(1).unwrap();let s=SpreadsheetSession::open_path(&path,SpreadsheetLoadConfig::default());let state=s.snapshot(0,0,0,0);println!("{}",serde_json::json!({"inputSha256":s.input_sha256(),"openCode":s.open_code(),"sheetCount":state.sheet_count,"xfCount":state.xf_count}));}
