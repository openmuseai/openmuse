use super::{ConstToken::TokenChildMap, PlaceHolderPiece::*};
use std::collections::HashMap;
// java public class ConstPieceDic
pub struct ConstPieceDic {
    mPieceMap: HashMap<u16, Box<dyn Piece>>,
    piecePlainText: PiecePlainText,
    piecePlainDigit: PiecePlainDigit,
}
impl ConstPieceDic {
    pub fn new() -> Self {
        let mut map: HashMap<u16, Box<dyn Piece>> = HashMap::new();
        // java this.mPieceMap.put('#',new PlaceHolderPiece.PieceSharp('#'));
        map.insert(35, Box::new(PieceSharp::new('#')));
        // java this.mPieceMap.put('0',new PlaceHolderPiece.PieceZero('0'));
        map.insert(48, Box::new(PieceZero::new('0')));
        // java this.mPieceMap.put('?',new PlaceHolderPiece.PieceAsk('?'));
        map.insert(63, Box::new(PieceAsk::new('?')));
        // java this.mPieceMap.put('.',new PlaceHolderPiece.PieceDecPoint('.'));
        map.insert(46, Box::new(PieceDecPoint::new('.')));
        // java this.mPieceMap.put(',',new PlaceHolderPiece.PieceThousSep(','));
        map.insert(44, Box::new(PieceThousSep::new(',')));
        // java this.mPieceMap.put('\\',new PlaceHolderPiece.PieceEscape('\\'));
        map.insert(92, Box::new(PieceEscape::new('\\')));
        // java this.mPieceMap.put('!',new PlaceHolderPiece.PieceEscape('!'));
        map.insert(33, Box::new(PieceEscape::new('!')));
        // java this.mPieceMap.put('"',new PlaceHolderPiece.PieceDoubleQuatation('"'));
        map.insert(34, Box::new(PieceDoubleQuatation::new('"')));
        // java this.mPieceMap.put('\'',new PlaceHolderPiece.PieceSingleQuatation('\''));
        map.insert(39, Box::new(PieceSingleQuatation::new('\'')));
        // java this.mPieceMap.put('*',new PlaceHolderPiece.PieceAsterisk('*'));
        map.insert(42, Box::new(PieceAsterisk::new('*')));
        // java this.mPieceMap.put('_',new PlaceHolderPiece.PieceUnderline('_'));
        map.insert(95, Box::new(PieceUnderline::new('_')));
        // java this.mPieceMap.put('/',new PlaceHolderPiece.PieceFraction('/'));
        map.insert(47, Box::new(PieceFraction::new('/')));
        // java this.mPieceMap.put('%',new PlaceHolderPiece.PiecePercent('%'));
        map.insert(37, Box::new(PiecePercent::new('%')));
        // java this.mPieceMap.put('@',new PlaceHolderPiece.PieceText('@'));
        map.insert(64, Box::new(PieceText::new('@')));
        // java this.mPieceMap.put(':',new PlaceHolderPiece.PieceColon(':'));
        map.insert(58, Box::new(PieceColon::new(':')));
        // java this.mPieceMap.put('d',new PlaceHolderPiece.Piece_d('d'));
        map.insert(100, Box::new(Piece_d::new('d')));
        // java this.mPieceMap.put('h',new PlaceHolderPiece.Piece_h('h'));
        map.insert(104, Box::new(Piece_h::new('h')));
        // java this.mPieceMap.put('m',new PlaceHolderPiece.Piece_m('m'));
        map.insert(109, Box::new(Piece_m::new('m')));
        // java this.mPieceMap.put('s',new PlaceHolderPiece.Piece_s('s'));
        map.insert(115, Box::new(Piece_s::new('s')));
        // java this.mPieceMap.put('y',new PlaceHolderPiece.Piece_y('y'));
        map.insert(121, Box::new(Piece_y::new('y')));
        // java this.mPieceMap.put('ป',new PlaceHolderPiece.PieceThaiYear('ป'));
        map.insert(3611, Box::new(PieceThaiYear::new('ป')));
        // java this.mPieceMap.put('ด',new PlaceHolderPiece.PieceThaiMonth('ด'));
        map.insert(3604, Box::new(PieceThaiMonth::new('ด')));
        // java this.mPieceMap.put('ว',new PlaceHolderPiece.PieceThaiDay('ว'));
        map.insert(3623, Box::new(PieceThaiDay::new('ว')));
        // java this.mPieceMap.put('ม',new PlaceHolderPiece.PieceThaiHour('ม'));
        map.insert(3617, Box::new(PieceThaiHour::new('ม')));
        // java this.mPieceMap.put('น',new PlaceHolderPiece.PieceThaiMinute('น'));
        map.insert(3609, Box::new(PieceThaiMinute::new('น')));
        // java this.mPieceMap.put('ท',new PlaceHolderPiece.PieceThaiSecond('ท'));
        map.insert(3607, Box::new(PieceThaiSecond::new('ท')));
        map.insert(
            19978,
            Box::new(Piece_AMPM::new(
                '上',
                vec!["上午/下午".into()],
                TokenChildMap::excel(),
            )),
        );
        map.insert(
            97,
            Box::new(Piece_a::new(
                'a',
                vec!["a/p".into(), "am/pm".into()],
                TokenChildMap::excel(),
            )),
        );
        map.insert(
            98,
            Box::new(Piece_b::new(
                'b',
                vec!["B1".into(), "B2".into()],
                TokenChildMap::excel(),
            )),
        );
        map.insert(
            101,
            Box::new(Piece_e::new(
                'e',
                vec!["E+".into(), "E-".into(), "E".into()],
                TokenChildMap::excel(),
            )),
        );
        map.insert(
            103,
            Box::new(Piece_g::new(
                'g',
                vec!["General".into()],
                TokenChildMap::excel(),
            )),
        );
        map.insert(
            91,
            Box::new(PieceBracket::new(
                '[',
                vec![
                    "\\$(?:[^-]*)(?:-([0-9A-F]+))?".into(),
                    "Black|Blue|Cyan|Green|Magenta|Red|White|Yellow|Color(\\d+)".into(),
                    "DBNum(\\d+)".into(),
                    "=|<>|>|<|>=|<=".into(),
                    "h+".into(),
                    "m+".into(),
                    "s+".into(),
                ],
                TokenChildMap::excel(),
                true,
            )),
        );
        Self {
            mPieceMap: map,
            piecePlainText: PiecePlainText::new('p'),
            piecePlainDigit: PiecePlainDigit::new('d'),
        }
    }
    // java char lowerCase=Character.toLowerCase(key);PlaceHolderPiece piece=mPieceMap.get(lowerCase);return piece!=null?piece:piecePlainText;
    pub fn searchPiece(&mut self, key: u16) -> &mut dyn Piece {
        if (49..=57).contains(&key) {
            return &mut self.piecePlainDigit;
        }
        let lower = char::from_u32(key as u32)
            .and_then(|c| c.to_lowercase().next())
            .map_or(key, |c| c as u16);
        if let Some(piece) = self.mPieceMap.get_mut(&lower) {
            piece.as_mut()
        } else {
            &mut self.piecePlainText
        }
    }
    // java this.mPieceMap=null;this.piecePlainText=null;
    pub fn destroy(&mut self) {
        self.mPieceMap.clear();
    }
}
