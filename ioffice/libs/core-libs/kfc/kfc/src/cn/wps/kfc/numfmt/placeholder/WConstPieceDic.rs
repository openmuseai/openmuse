// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Java field: short PIECE_DIC_COMMOM
pub(crate) const PIECE_DIC_COMMOM: i16 = 0;
// Java field: short PIECE_DIC_DIGIT
pub(crate) const PIECE_DIC_DIGIT: i16 = 1;
// Java field: short PIECE_DIC_DTIME
pub(crate) const PIECE_DIC_DTIME: i16 = 2;
// Java field: String CASE_SENSITIVE_STR
pub(crate) const CASE_SENSITIVE_STR: &str = "OnMW";

use super::ConstToken;
use super::PlaceHolderPiece::*;

// Java field: HashMap<Character, PlaceHolderPiece>[] mPieceMaps
static S_PIECE_MAPS: OnceLock<Vec<HashMap<char, PlaceHolderPiece>>> = OnceLock::new();
fn get_piece_maps() -> &'static Vec<HashMap<char, PlaceHolderPiece>> {
    S_PIECE_MAPS.get_or_init(|| {
        let mut maps: Vec<HashMap<char, PlaceHolderPiece>> = Vec::new();
        for _ in 0..3 {
            maps.push(HashMap::new());
        }

        // mPieceMaps[0] - common
        maps[0].insert(
            '\'',
            PlaceHolderPiece::PieceDoubleQuatation(PieceDoubleQuatation::new('\'')),
        );
        maps[0].insert(
            '`',
            PlaceHolderPiece::WPieceApostrophe(WPieceApostrophe::new('`')),
        );

        // mPieceMaps[1] - digit
        maps[1].insert('#', PlaceHolderPiece::PieceAsk(PieceAsk::new('#')));
        maps[1].insert('0', PlaceHolderPiece::PieceZero(PieceZero::new('0')));
        maps[1].insert(
            '.',
            PlaceHolderPiece::PieceDecPoint(PieceDecPoint::new('.')),
        );
        maps[1].insert(
            ',',
            PlaceHolderPiece::PieceThousSep(PieceThousSep::new(',')),
        );
        maps[1].insert('x', PlaceHolderPiece::WPieceX(WPieceX::new('x')));
        maps[1].insert('-', PlaceHolderPiece::WPieceSign(WPieceSign::new('-')));
        maps[1].insert('+', PlaceHolderPiece::WPieceSign(WPieceSign::new('+')));

        // mPieceMaps[2] - datetime
        maps[2].insert(
            'a',
            PlaceHolderPiece::WPiece_a(WPiece_a::new(
                'A',
                &ConstToken::TOKEN_W_a_CHILDREN,
                &ConstToken::W_TOKEN_CHILD_TO_HOLDER,
            )),
        );
        maps[2].insert('b', PlaceHolderPiece::WPiece_b(WPiece_b::new('b')));
        maps[2].insert('e', PlaceHolderPiece::WPiece_e(WPiece_e::new('e')));
        maps[2].insert('g', PlaceHolderPiece::WPiece_g(WPiece_g::new('g')));
        maps[2].insert('h', PlaceHolderPiece::Piece_h(Piece_h::new('h')));
        maps[2].insert('y', PlaceHolderPiece::WPiece_y(WPiece_y::new('y')));
        maps[2].insert('m', PlaceHolderPiece::WPiece_m(WPiece_m::new('m')));
        maps[2].insert('M', PlaceHolderPiece::WPiece_m(WPiece_m::new('M')));
        maps[2].insert('d', PlaceHolderPiece::WPiece_d(WPiece_d::new('d')));
        maps[2].insert('s', PlaceHolderPiece::Piece_s(Piece_s::new('s')));
        maps[2].insert('O', PlaceHolderPiece::WPiece_o(WPiece_o::new('O')));
        maps[2].insert('n', PlaceHolderPiece::WPiece_n(WPiece_n::new('n')));
        maps[2].insert('r', PlaceHolderPiece::WPiece_r(WPiece_r::new('r')));
        maps[2].insert('W', PlaceHolderPiece::WPiece_w(WPiece_w::new('W')));
        maps[2].insert(
            'ป',
            PlaceHolderPiece::PieceThaiYear(PieceThaiYear::new('ป')),
        );
        maps[2].insert(
            'ด',
            PlaceHolderPiece::PieceThaiMonth(PieceThaiMonth::new('ด')),
        );
        maps[2].insert('ว', PlaceHolderPiece::PieceThaiDay(PieceThaiDay::new('ว')));
        maps[2].insert(
            'ม',
            PlaceHolderPiece::PieceThaiHour(PieceThaiHour::new('ม')),
        );
        maps[2].insert(
            'น',
            PlaceHolderPiece::PieceThaiMinute(PieceThaiMinute::new('น')),
        );
        maps[2].insert(
            'ท',
            PlaceHolderPiece::PieceThaiSecond(PieceThaiSecond::new('ท')),
        );

        maps
    })
}

// Java field: PlaceHolderPiece.WPiecePlainText piecePlainText
static S_PIECE_PLAIN_TEXT: OnceLock<Mutex<WPiecePlainText>> = OnceLock::new();
fn get_piece_plain_text() -> &'static Mutex<WPiecePlainText> {
    S_PIECE_PLAIN_TEXT.get_or_init(|| Mutex::new(WPiecePlainText::new()))
}

// Java: public class WConstPieceDic
pub struct WConstPieceDic {}

// Java: public class WConstPieceDic
impl WConstPieceDic {
    // Java: WConstPieceDic()
    pub fn new() -> Self {
        // Java: super();
        WConstPieceDic {}
    }

    // Java: static PlaceHolderPiece searchPiece(char key, short dicType)
    pub fn searchPiece(key: char, dic_type: i16) -> PlaceHolderPiece {
        // Java: PlaceHolderPiece piece = (PlaceHolderPiece)mPieceMaps[0].get(key);
        let maps = get_piece_maps();
        let mut piece = maps[0].get(&key).cloned();
        // Java: if (piece == null)
        if piece.is_none() {
            // Java: char c = key;
            let mut c = key;
            // Java: if (CASE_SENSITIVE_STR.indexOf(key) == -1)
            if !CASE_SENSITIVE_STR.contains(key) {
                // Java: c = Character.toLowerCase(key);
                c = key.to_lowercase().next().unwrap_or(key);
            }
            // Java: piece = (PlaceHolderPiece)mPieceMaps[dicType].get(c);
            piece = maps[dic_type as usize].get(&c).cloned();
        }
        // Java: if (piece == null)
        if piece.is_none() {
            // Java: piecePlainText.setIdentifer(key);
            let mut plain = get_piece_plain_text()
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            plain.setIdentifer(key);
            // Java: piece = piecePlainText;
            return PlaceHolderPiece::WPiecePlainText(plain.clone());
        }
        // Java: return piece;
        piece.unwrap()
    }
}
