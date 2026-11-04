use std::{collections::HashMap,rc::Rc,cell::RefCell};
use super::{VariantToken::VariantToken,PlaceHolderPiece::*};
pub type PieceRef=Rc<RefCell<Box<dyn Piece>>>;
fn reference(value:impl Piece+'static)->PieceRef{Rc::new(RefCell::new(Box::new(value)))}
// java public class VariantPieceDic {
pub struct VariantPieceDic{
    // java protected VariantToken mVariantToken = new VariantToken();
    pub(crate) mVariantToken:Option<VariantToken>,
    // java private HashMap<Character, PlaceHolderPiece> mVariantPieceMap = new HashMap();
    mVariantPieceMap:Option<HashMap<u16,PieceRef>>,
    // java private HashMap<Character, PlaceHolderPiece> mVariantPhraseMap = new HashMap();
    mVariantPhraseMap:Option<HashMap<u16,PieceRef>>,
    // java private PlaceHolderPiece.PiecePlainText piecePlainText = new PlaceHolderPiece.PiecePlainText();
    piecePlainText:PieceRef,
    // java private short mLatestLcid;
    mLatestLcid:i16,
}
impl VariantPieceDic{
    // java public VariantPieceDic() { super(); }
    pub fn new()->Self{Self{mVariantToken:Some(VariantToken::new()),mVariantPieceMap:Some(HashMap::new()),mVariantPhraseMap:Some(HashMap::new()),piecePlainText:reference(PiecePlainText::new('p')),mLatestLcid:0}}
    // java public void destroy() { this.mVariantToken=null;this.mVariantPieceMap=null;this.mVariantPhraseMap=null;this.mLatestLcid=0; }
    pub fn destroy(&mut self){self.mVariantToken=None;self.mVariantPieceMap=None;self.mVariantPhraseMap=None;self.mLatestLcid=0;}
    // java public void reset(short lcid) {
    pub fn reset(&mut self,lcid:i16){
        // java if (lcid != this.mLatestLcid) { this.mLatestLcid=lcid;this.mVariantToken.load(lcid);this.mVariantPieceMap.clear();
        if lcid!=self.mLatestLcid{self.mLatestLcid=lcid;let t=self.mVariantToken.as_mut().expect("NullPointerException");t.load(lcid);let map=self.mVariantPieceMap.as_mut().expect("NullPointerException");map.clear();
            // java HashMap var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);Character var10001='#';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceSharp('#'));
            map.insert('#' as u16,reference(PieceSharp::new('#')));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='0';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceZero('0'));
            map.insert('0' as u16,reference(PieceZero::new('0')));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='?';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceAsk('?'));
            map.insert('?' as u16,reference(PieceAsk::new('?')));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='？';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceAsk('？'));
            map.insert('？' as u16,reference(PieceAsk::new('？')));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyTokenDecimal,new PlaceHolderPiece.PieceDecPoint(this.mVariantToken.mKeyTokenDecimal));
            map.insert(t.mKeyTokenDecimal as u16,reference(PieceDecPoint::new(t.mKeyTokenDecimal)));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyTokenThous,new PlaceHolderPiece.PieceThousSep(this.mVariantToken.mKeyTokenThous));
            map.insert(t.mKeyTokenThous as u16,reference(PieceThousSep::new(t.mKeyTokenThous)));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='\\';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceEscape('\\'));
            map.insert('\\' as u16,reference(PieceEscape::new('\\')));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyTokenComment,new PlaceHolderPiece.PieceEscape(this.mVariantToken.mKeyTokenComment));
            map.insert(t.mKeyTokenComment as u16,reference(PieceEscape::new(t.mKeyTokenComment)));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='"';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceDoubleQuatation('"'));
            map.insert('"' as u16,reference(PieceDoubleQuatation::new('"')));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='*';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceAsterisk('*'));
            map.insert('*' as u16,reference(PieceAsterisk::new('*')));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='_';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceUnderline('_'));
            map.insert('_' as u16,reference(PieceUnderline::new('_')));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='/';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceFraction('/'));
            map.insert('/' as u16,reference(PieceFraction::new('/')));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='%';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PiecePercent('%'));
            map.insert('%' as u16,reference(PiecePercent::new('%')));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='@';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceText('@'));
            map.insert('@' as u16,reference(PieceText::new('@')));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001=':';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceColon(':'));
            map.insert(':' as u16,reference(PieceColon::new(':')));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='上';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.Piece_AMPM('上',this.mVariantToken.TOKEN_CH_AP_CHILDREN,this.mVariantToken.mTokenChildMap));
            map.insert('上' as u16,reference(Piece_AMPM::new('上',t.TOKEN_CH_AP_CHILDREN.clone(),t.mTokenChildMap.clone())));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='[';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.PieceBracket('[',this.mVariantToken.mTokenBracketChildren,this.mVariantToken.mTokenChildMap,false));
            map.insert('[' as u16,reference(PieceBracket::new('[',t.mTokenBracketChildren.clone(),t.mTokenChildMap.clone(),false)));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyToken_a,new PlaceHolderPiece.Piece_a(this.mVariantToken.mKeyToken_a,this.mVariantToken.mToken_a_children,this.mVariantToken.mTokenChildMap));
            map.insert(t.mKeyToken_a as u16,reference(Piece_a::new(t.mKeyToken_a,t.mToken_a_children.clone(),t.mTokenChildMap.clone())));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='b';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.Piece_b('b',this.mVariantToken.mToken_b_children,this.mVariantToken.mTokenChildMap));
            map.insert('b' as u16,reference(Piece_b::new('b',t.mToken_b_children.clone(),t.mTokenChildMap.clone())));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='e';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.Piece_e('e',this.mVariantToken.mToken_e_children,this.mVariantToken.mTokenChildMap));
            map.insert('e' as u16,reference(Piece_e::new('e',t.mToken_e_children.clone(),t.mTokenChildMap.clone())));
            // java var10000=this.mVariantPieceMap;Objects.requireNonNull(this.mVariantToken);var10001='g';Objects.requireNonNull(this.mVariantToken);var10000.put(var10001,new PlaceHolderPiece.Piece_g('g',this.mVariantToken.mToken_g_children,this.mVariantToken.mTokenChildMap));
            map.insert('g' as u16,reference(Piece_g::new('g',t.mToken_g_children.clone(),t.mTokenChildMap.clone())));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyTokenHour,new PlaceHolderPiece.Piece_h(this.mVariantToken.mKeyTokenHour));
            map.insert(t.mKeyTokenHour as u16,reference(Piece_h::new(t.mKeyTokenHour)));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyTokenMin,new PlaceHolderPiece.Piece_m(this.mVariantToken.mKeyTokenMin));
            map.insert(t.mKeyTokenMin as u16,reference(Piece_m::new(t.mKeyTokenMin)));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyTokenSec,new PlaceHolderPiece.Piece_s(this.mVariantToken.mKeyTokenSec));
            map.insert(t.mKeyTokenSec as u16,reference(Piece_s::new(t.mKeyTokenSec)));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyTokenDay,new PlaceHolderPiece.Piece_d(this.mVariantToken.mKeyTokenDay));
            map.insert(t.mKeyTokenDay as u16,reference(Piece_d::new(t.mKeyTokenDay)));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyTokenMonth,new PlaceHolderPiece.Piece_m(this.mVariantToken.mKeyTokenMonth));
            map.insert(t.mKeyTokenMonth as u16,reference(Piece_m::new(t.mKeyTokenMonth)));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyTokenYear,new PlaceHolderPiece.Piece_y(this.mVariantToken.mKeyTokenYear));
            map.insert(t.mKeyTokenYear as u16,reference(Piece_y::new(t.mKeyTokenYear)));
            // java PlaceHolderPiece piece=new PlaceHolderPiece.PiecePlainDigit();
            let piece=reference(PiecePlainDigit::new('d'));
            // java this.mVariantPieceMap.put(this.mVariantToken.mKeyToken_1,piece);this.mVariantPieceMap.put(this.mVariantToken.mKeyToken_2,piece);this.mVariantPieceMap.put(this.mVariantToken.mKeyToken_3,piece);this.mVariantPieceMap.put(this.mVariantToken.mKeyToken_4,piece);this.mVariantPieceMap.put(this.mVariantToken.mKeyToken_5,piece);this.mVariantPieceMap.put(this.mVariantToken.mKeyToken_6,piece);this.mVariantPieceMap.put(this.mVariantToken.mKeyToken_7,piece);this.mVariantPieceMap.put(this.mVariantToken.mKeyToken_8,piece);this.mVariantPieceMap.put(this.mVariantToken.mKeyToken_9,piece);
            map.insert(t.mKeyToken_1 as u16,piece.clone());map.insert(t.mKeyToken_2 as u16,piece.clone());map.insert(t.mKeyToken_3 as u16,piece.clone());map.insert(t.mKeyToken_4 as u16,piece.clone());map.insert(t.mKeyToken_5 as u16,piece.clone());map.insert(t.mKeyToken_6 as u16,piece.clone());map.insert(t.mKeyToken_7 as u16,piece.clone());map.insert(t.mKeyToken_8 as u16,piece.clone());map.insert(t.mKeyToken_9 as u16,piece);
            // java this.mVariantPhraseMap.put(this.mVariantToken.mKeyTokenGeneral,new PlaceHolderPiece.PieceGeneral(this.mVariantToken.mKeyTokenGeneral,this.mVariantToken.mTokenGeneralChildren,this.mVariantToken.mTokenChildMap));
            let phrases=self.mVariantPhraseMap.as_mut().expect("NullPointerException");phrases.insert(t.mKeyTokenGeneral as u16,reference(PieceGeneral::new(t.mKeyTokenGeneral,t.mTokenGeneralChildren.clone(),t.mTokenChildMap.clone())));
            // java this.mVariantPhraseMap.put(this.mVariantToken.mKeyToken_a,new PlaceHolderPiece.Piece_AMPM(this.mVariantToken.mKeyToken_a,this.mVariantToken.mToken_a_children,this.mVariantToken.mTokenChildMap,true));
            phrases.insert(t.mKeyToken_a as u16,reference(Piece_AMPM::new__4(t.mKeyToken_a,t.mToken_a_children.clone(),t.mTokenChildMap.clone(),true)));
        }
    }
    // java public PlaceHolderPiece searchPiece(char c,String srcCode,int start) {
    pub fn searchPiece(&mut self,c:u16,srcCode:&[u16],start:usize)->PieceRef{
        // java char c1=Character.toLowerCase(c);PlaceHolderPiece piece=this.mVariantPhraseMap.get(c1);
        let c1=crate::native_runtime::Character::toLowerCase(c);let piece=self.mVariantPhraseMap.as_ref().expect("NullPointerException").get(&c1).cloned();
        // java if(piece!=null&&((PlaceHolderPiece.PieceVariousChar)piece).find(srcCode,start,srcCode.length(),true)){return piece;}else{
        if let Some(piece)=piece{let found=piece.borrow_mut().find_various(srcCode,start,srcCode.len(),true);if found{return piece;}}
        // java piece=this.mVariantPieceMap.get(c1);return piece!=null?piece:this.piecePlainText;
        self.mVariantPieceMap.as_ref().expect("NullPointerException").get(&c1).cloned().unwrap_or_else(||self.piecePlainText.clone())
    }
}
