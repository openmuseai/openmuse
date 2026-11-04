use crate::cn::wps::kfc::numfmt::{global::{DBNumAccesser::DBNumAccesser,GlobalEnv::GlobalEnv,NCLAccesser::NCLAccesser},placeholder::PieceContext::PieceContext};
use super::NumFmtStyle::NumFmtStyle;
// java public class FormatLocale
pub struct FormatLocale {pub mNumeralShape:i8,mCalendarType:i8,mLocaleCodeID:i16,mDBNum:Option<String>,pub isRMB:bool}
impl FormatLocale {
    pub fn new()->Self{Self{mNumeralShape:0,mCalendarType:0,mLocaleCodeID:0,mDBNum:None,isRMB:false}}
    // java public void reset(PieceContext pieceContext,NumFmtStyle numFmtStyle)
    pub fn reset(&mut self,p:&PieceContext,s:&mut NumFmtStyle){
        // java this.mNumeralShape=0;this.mCalendarType=0;this.mLocaleCodeID=0;this.mDBNum=null;this.isRMB=false;
        self.mNumeralShape=0;self.mCalendarType=0;self.mLocaleCodeID=0;self.mDBNum=None;self.isRMB=false;
        // java if(pieceContext.mNumAreaCode!=null){NCLAccesser formatNCL=NCLAccesser.shareFormatCodeArea(...);...;numFmtStyle.mHasNCL=true;}
        if let Some(code)=&p.mNumAreaCode{let ncl=NCLAccesser::shareFormatCodeArea(code);self.mNumeralShape=ncl.getNumberalShape();self.mCalendarType=ncl.getCalendarType();self.mLocaleCodeID=ncl.getLCID();s.mHasNCL=true;}
        // java this.mDBNum=pieceContext.mDBNumCode;numFmtStyle.mDBNumber=this.processDBNum();numFmtStyle.mIsRMBScale=this.isRMB&&numFmtStyle.mDBNumber==2&&this.mNumeralShape==31;
        self.mDBNum=p.mDBNumCode.clone();s.mDBNumber=self.processDBNum();s.mIsRMBScale=self.isRMB&&s.mDBNumber==2&&self.mNumeralShape==31;
        // java if(pieceContext.mForceArbicShape&&this.mNumeralShape==0)this.mNumeralShape=2;
        if p.mForceArbicShape&&self.mNumeralShape==0{self.mNumeralShape=2;}
        // java numFmtStyle.mNumeralShape=this.mNumeralShape;numFmtStyle.mCalendarType=this.mCalendarType;numFmtStyle.mLocaleCodeID=this.mLocaleCodeID;
        s.mNumeralShape=self.mNumeralShape;s.mCalendarType=self.mCalendarType;s.mLocaleCodeID=self.mLocaleCodeID;
    }
    // java private short processDBNum()
    pub fn processDBNum(&mut self)->i16{
        let mut dbnum=0;
        if let Some(code)=self.mDBNum.as_ref().filter(|s|!s.is_empty()){
            if self.mNumeralShape==0{let lcid=if self.mLocaleCodeID==0{GlobalEnv::getDefaultLCID()}else{self.mLocaleCodeID};self.mNumeralShape=DBNumAccesser::transDBNumLocale(lcid,code);}
            dbnum=code.parse().expect("NumberFormatException");
        }dbnum
    }
    // java processCalendar(): case 2 lcid=1033; case 7 lcid=1054;case 9 lcid=1036;case 10,11 lcid=1025;default unchanged.
    pub fn processCalendar(&self)->i16{match self.mCalendarType{2=>1033,7=>1054,9=>1036,10|11=>1025,_=>self.mLocaleCodeID}}
}
