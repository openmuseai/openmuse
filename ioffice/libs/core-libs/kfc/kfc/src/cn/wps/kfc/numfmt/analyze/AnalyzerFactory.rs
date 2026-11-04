use std::collections::HashMap;
use super::r#type::{IAnalyzer::IAnalyzer,AnalyzerGeneral::AnalyzerGeneral,AnalyzerNumeric::AnalyzerNumeric,AnalyzerFraction::AnalyzerFraction,AnalyzerScience::AnalyzerScience,AnalyzerText::AnalyzerText,AnalyzerDateTime::AnalyzerDateTime};
use crate::cn::wps::kfc::numfmt::placeholder::{PieceContext::PieceContext,PlaceHolderToken::TokenRef};
// java public class AnalyzerFactory {
pub struct AnalyzerFactory{
    // java private HashMap<Short,IAnalyzer> mAnalyzer=new HashMap();
    mAnalyzer:HashMap<i16,Box<dyn IAnalyzer>>,
}
impl AnalyzerFactory{
    // java public AnalyzerFactory(){super();}
    pub fn new()->Self{Self{mAnalyzer:HashMap::new()}}
    // java public IAnalyzer getAnalyzer(short type,ArrayList<PlaceHolderToken> tokens,PieceContext context) {
    pub fn getAnalyzer(&mut self,r#type:i16,tokens:&[TokenRef],context:&PieceContext)->&mut dyn IAnalyzer{
        // java IAnalyzer analyzer=this.mAnalyzer.get(type);if(analyzer==null){switch(type){
        if !self.mAnalyzer.contains_key(&r#type){let analyzer:Box<dyn IAnalyzer>=match r#type{
            // java case 1:analyzer=new AnalyzerGeneral();break;
            1=>Box::new(AnalyzerGeneral::new()),
            // java case 4:case 512:analyzer=new AnalyzerNumeric();break;
            4|512=>Box::new(AnalyzerNumeric::new()),
            // java case 8:analyzer=new AnalyzerFraction();break;
            8=>Box::new(AnalyzerFraction::new()),
            // java case 16:analyzer=new AnalyzerScience();break;
            16=>Box::new(AnalyzerScience::new()),
            // java case 8192:analyzer=new AnalyzerDateTime();
            8192=>Box::new(AnalyzerDateTime::new()),
            // java case 64:default:analyzer=new AnalyzerText();break;
            _=>Box::new(AnalyzerText::new()),
        };
        // java this.mAnalyzer.put(type,analyzer);
        self.mAnalyzer.insert(r#type,analyzer);}
        // java analyzer.reset(tokens,context,type);return analyzer;
        let analyzer=self.mAnalyzer.get_mut(&r#type).unwrap();analyzer.reset(tokens,context,r#type);analyzer.as_mut()
    }
}
