use crate::java_runtime::{Rc,JavaObject::JavaObject};
use crate::org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants;
// java public class ErrorConstant {
pub struct ErrorConstant{
    // java private final int _errorCode;
    _errorCode:i32,
}
impl ErrorConstant{
    // java private static final ErrorConstant NULL=new ErrorConstant(0),DIV_0=new ErrorConstant(7),VALUE=new ErrorConstant(15),REF=new ErrorConstant(23),NAME=new ErrorConstant(29),NUM=new ErrorConstant(36),NA=new ErrorConstant(42);
    // Native immutable references transport the original seven static objects.
    fn instances()->&'static [Rc<ErrorConstant>;7]{static VALUES:std::sync::LazyLock<[Rc<ErrorConstant>;7]>=std::sync::LazyLock::new(||[Rc::new(ErrorConstant::ErrorConstant(0)),Rc::new(ErrorConstant::ErrorConstant(7)),Rc::new(ErrorConstant::ErrorConstant(15)),Rc::new(ErrorConstant::ErrorConstant(23)),Rc::new(ErrorConstant::ErrorConstant(29)),Rc::new(ErrorConstant::ErrorConstant(36)),Rc::new(ErrorConstant::ErrorConstant(42))]);&VALUES}
    // java private ErrorConstant(int errorCode){this._errorCode=errorCode;}
    fn ErrorConstant(errorCode:i32)->Self{Self{_errorCode:errorCode}}
    // java public int getErrorCode(){return this._errorCode;}
    pub fn getErrorCode(&self)->i32{self._errorCode}
    // java public String getText(){return ErrorConstants.isValidCode(this._errorCode)?ErrorConstants.getText(this._errorCode):"unknown error code ("+this._errorCode+")";}
    pub fn getText(&self)->String{if ErrorConstants::isValidCode(self._errorCode){ErrorConstants::getText(self._errorCode).into()}else{format!("unknown error code ({})",self._errorCode)}}
    // java public static ErrorConstant valueOf(int errorCode){switch(errorCode){case 0:return NULL;case 7:return DIV_0;case 15:return VALUE;case 23:return REF;case 29:return NAME;case 36:return NUM;case 42:return NA;default:System.err.println("Warning - unexpected error code ("+errorCode+")");return new ErrorConstant(errorCode);}}
    pub fn valueOf(errorCode:i32)->Rc<Self>{match errorCode{0=>Self::instances()[0].clone(),7=>Self::instances()[1].clone(),15=>Self::instances()[2].clone(),23=>Self::instances()[3].clone(),29=>Self::instances()[4].clone(),36=>Self::instances()[5].clone(),42=>Self::instances()[6].clone(),_=>{eprintln!("Warning - unexpected error code ({errorCode})");Rc::new(ErrorConstant::ErrorConstant(errorCode))}}}
    // java public String toString(){StringBuffer sb=new StringBuffer(64);sb.append(this.getClass().getName()).append(" [");sb.append(this.getText());sb.append("]");return sb.toString();}
    pub fn toString(&self)->String{let mut sb=String::with_capacity(64);sb.push_str("cn.wps.et.ss.formula.constant.ErrorConstant [");sb.push_str(&self.getText());sb.push(']');sb}
}
impl JavaObject for ErrorConstant{fn as_any(&self)->&dyn std::any::Any{self}fn toString(&self)->String{ErrorConstant::toString(self)}}
