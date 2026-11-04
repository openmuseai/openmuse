// java public final class CurrencyRes {
#[derive(Clone,serde::Serialize,serde::Deserialize)]
pub struct CurrencyRes{
    // java public final String CURRENCY_CHAR;
    pub CURRENCY_CHAR:String,
    // java public final PositiveCurrencyFormat POSITIVE_CURRENCY_FORMAT;
    pub POSITIVE_CURRENCY_FORMAT:PositiveCurrencyFormat,
    // java public final NegativeCurrencyFormat NEGATIVE_CURRENCY_FORMAT;
    pub NEGATIVE_CURRENCY_FORMAT:NegativeCurrencyFormat,
    // java public final String CURRENCY_NUMFMT;
    pub CURRENCY_NUMFMT:String,
    // java public final String CURRENCY_NUMFMT_DECIMAL;
    pub CURRENCY_NUMFMT_DECIMAL:String,
}
impl CurrencyRes{
    // java public CurrencyRes(String currencyChar, PositiveCurrencyFormat positiveCurrencyType, NegativeCurrencyFormat negativeCurrencyType, String numfmt, String decimalNumfmt) {
    pub fn CurrencyRes(currencyChar:String,positiveCurrencyType:PositiveCurrencyFormat,negativeCurrencyType:NegativeCurrencyFormat,numfmt:String,decimalNumfmt:String)->Self{
        // java this.CURRENCY_CHAR = currencyChar;
        // java this.POSITIVE_CURRENCY_FORMAT = positiveCurrencyType;
        // java this.NEGATIVE_CURRENCY_FORMAT = negativeCurrencyType;
        // java this.CURRENCY_NUMFMT = numfmt;
        // java this.CURRENCY_NUMFMT_DECIMAL = decimalNumfmt;
        Self{CURRENCY_CHAR:currencyChar,POSITIVE_CURRENCY_FORMAT:positiveCurrencyType,NEGATIVE_CURRENCY_FORMAT:negativeCurrencyType,CURRENCY_NUMFMT:numfmt,CURRENCY_NUMFMT_DECIMAL:decimalNumfmt}
    }
}
// java public static enum PositiveCurrencyFormat { HEAD, HEAD_SPACE, SPACE_END, END; }
#[derive(Clone,Copy,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
pub enum PositiveCurrencyFormat{HEAD,HEAD_SPACE,SPACE_END,END}
// java public static enum NegativeCurrencyFormat { BRACKET_HEAD, SCN, CSN, CNS, BRACKET_END, SNC, NSC, NCS, SNPC, SCPN, NPCS, CPNS, CPSN, NSPC, BRACKET_HEAD_SPACE, BRACKET_SPACE_END; }
#[derive(Clone,Copy,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
pub enum NegativeCurrencyFormat{BRACKET_HEAD,SCN,CSN,CNS,BRACKET_END,SNC,NSC,NCS,SNPC,SCPN,NPCS,CPNS,CPSN,NSPC,BRACKET_HEAD_SPACE,BRACKET_SPACE_END}
