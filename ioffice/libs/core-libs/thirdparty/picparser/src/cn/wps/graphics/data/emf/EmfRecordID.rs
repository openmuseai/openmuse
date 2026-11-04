#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java field: int HEADER
pub(crate) const HEADER: i32 = 1;
// Java field: int POLYBEZIER
pub(crate) const POLYBEZIER: i32 = 2;
// Java field: int POLYGON
pub(crate) const POLYGON: i32 = 3;
// Java field: int POLYLINE
pub(crate) const POLYLINE: i32 = 4;
// Java field: int POLYBEZIERTO
pub(crate) const POLYBEZIERTO: i32 = 5;
// Java field: int POLYLINETO
pub(crate) const POLYLINETO: i32 = 6;
// Java field: int POLYPOLYLINE
pub(crate) const POLYPOLYLINE: i32 = 7;
// Java field: int POLYPOLYGON
pub(crate) const POLYPOLYGON: i32 = 8;
// Java field: int SETWINDOWEXTEX
pub(crate) const SETWINDOWEXTEX: i32 = 9;
// Java field: int SETWINDOWORGEX
pub(crate) const SETWINDOWORGEX: i32 = 10;
// Java field: int SETVIEWPORTEXTEX
pub(crate) const SETVIEWPORTEXTEX: i32 = 11;
// Java field: int SETVIEWPORTORGEX
pub(crate) const SETVIEWPORTORGEX: i32 = 12;
// Java field: int SETBRUSHORGEX
pub(crate) const SETBRUSHORGEX: i32 = 13;
// Java field: int EOF
pub(crate) const EOF: i32 = 14;
// Java field: int SETPIXELV
pub(crate) const SETPIXELV: i32 = 15;
// Java field: int SETMAPPERFLAGS
pub(crate) const SETMAPPERFLAGS: i32 = 16;
// Java field: int SETMAPMODE
pub(crate) const SETMAPMODE: i32 = 17;
// Java field: int SETBKMODE
pub(crate) const SETBKMODE: i32 = 18;
// Java field: int SETPOLYFILLMODE
pub(crate) const SETPOLYFILLMODE: i32 = 19;
// Java field: int SETROP2
pub(crate) const SETROP2: i32 = 20;
// Java field: int SETSTRETCHBLTMODE
pub(crate) const SETSTRETCHBLTMODE: i32 = 21;
// Java field: int SETTEXTALIGN
pub(crate) const SETTEXTALIGN: i32 = 22;
// Java field: int SETCOLORADJUSTMENT
pub(crate) const SETCOLORADJUSTMENT: i32 = 23;
// Java field: int SETTEXTCOLOR
pub(crate) const SETTEXTCOLOR: i32 = 24;
// Java field: int SETBKCOLOR
pub(crate) const SETBKCOLOR: i32 = 25;
// Java field: int OFFSETCLIPRGN
pub(crate) const OFFSETCLIPRGN: i32 = 26;
// Java field: int MOVETOEX
pub(crate) const MOVETOEX: i32 = 27;
// Java field: int SETMETARGN
pub(crate) const SETMETARGN: i32 = 28;
// Java field: int EXCLUDECLIPRECT
pub(crate) const EXCLUDECLIPRECT: i32 = 29;
// Java field: int INTERSECTCLIPRECT
pub(crate) const INTERSECTCLIPRECT: i32 = 30;
// Java field: int SCALEVIEWPORTEXTEX
pub(crate) const SCALEVIEWPORTEXTEX: i32 = 31;
// Java field: int SCALEWINDOWEXTEX
pub(crate) const SCALEWINDOWEXTEX: i32 = 32;
// Java field: int SAVEDC
pub(crate) const SAVEDC: i32 = 33;
// Java field: int RESTOREDC
pub(crate) const RESTOREDC: i32 = 34;
// Java field: int SETWORLDTRANSFORM
pub(crate) const SETWORLDTRANSFORM: i32 = 35;
// Java field: int MODIFYWORLDTRANSFORM
pub(crate) const MODIFYWORLDTRANSFORM: i32 = 36;
// Java field: int SELECTOBJECT
pub(crate) const SELECTOBJECT: i32 = 37;
// Java field: int CREATEPEN
pub(crate) const CREATEPEN: i32 = 38;
// Java field: int CREATEBRUSHINDIRECT
pub(crate) const CREATEBRUSHINDIRECT: i32 = 39;
// Java field: int DELETEOBJECT
pub(crate) const DELETEOBJECT: i32 = 40;
// Java field: int ANGLEARC
pub(crate) const ANGLEARC: i32 = 41;
// Java field: int ELLIPSE
pub(crate) const ELLIPSE: i32 = 42;
// Java field: int RECTANGLE
pub(crate) const RECTANGLE: i32 = 43;
// Java field: int ROUNDRECT
pub(crate) const ROUNDRECT: i32 = 44;
// Java field: int ARC
pub(crate) const ARC: i32 = 45;
// Java field: int CHORD
pub(crate) const CHORD: i32 = 46;
// Java field: int PIE
pub(crate) const PIE: i32 = 47;
// Java field: int SELECTPALETTE
pub(crate) const SELECTPALETTE: i32 = 48;
// Java field: int CREATEPALETTE
pub(crate) const CREATEPALETTE: i32 = 49;
// Java field: int SETPALETTEENTRIES
pub(crate) const SETPALETTEENTRIES: i32 = 50;
// Java field: int RESIZEPALETTE
pub(crate) const RESIZEPALETTE: i32 = 51;
// Java field: int REALIZEPALETTE
pub(crate) const REALIZEPALETTE: i32 = 52;
// Java field: int EXTFLOODFILL
pub(crate) const EXTFLOODFILL: i32 = 53;
// Java field: int LINETO
pub(crate) const LINETO: i32 = 54;
// Java field: int ARCTO
pub(crate) const ARCTO: i32 = 55;
// Java field: int POLYDRAW
pub(crate) const POLYDRAW: i32 = 56;
// Java field: int SETARCDIRECTION
pub(crate) const SETARCDIRECTION: i32 = 57;
// Java field: int SETMITERLIMIT
pub(crate) const SETMITERLIMIT: i32 = 58;
// Java field: int BEGINPATH
pub(crate) const BEGINPATH: i32 = 59;
// Java field: int ENDPATH
pub(crate) const ENDPATH: i32 = 60;
// Java field: int CLOSEFIGURE
pub(crate) const CLOSEFIGURE: i32 = 61;
// Java field: int FILLPATH
pub(crate) const FILLPATH: i32 = 62;
// Java field: int STROKEANDFILLPATH
pub(crate) const STROKEANDFILLPATH: i32 = 63;
// Java field: int STROKEPATH
pub(crate) const STROKEPATH: i32 = 64;
// Java field: int FLATTENPATH
pub(crate) const FLATTENPATH: i32 = 65;
// Java field: int WIDENPATH
pub(crate) const WIDENPATH: i32 = 66;
// Java field: int SELECTCLIPPATH
pub(crate) const SELECTCLIPPATH: i32 = 67;
// Java field: int ABORTPATH
pub(crate) const ABORTPATH: i32 = 68;
// Java field: int RESERVED_69
pub(crate) const RESERVED_69: i32 = 69;
// Java field: int GDICOMMENT
pub(crate) const GDICOMMENT: i32 = 70;
// Java field: int FILLRGN
pub(crate) const FILLRGN: i32 = 71;
// Java field: int FRAMERGN
pub(crate) const FRAMERGN: i32 = 72;
// Java field: int INVERTRGN
pub(crate) const INVERTRGN: i32 = 73;
// Java field: int PAINTRGN
pub(crate) const PAINTRGN: i32 = 74;
// Java field: int EXTSELECTCLIPRGN
pub(crate) const EXTSELECTCLIPRGN: i32 = 75;
// Java field: int BITBLT
pub(crate) const BITBLT: i32 = 76;
// Java field: int STRETCHBLT
pub(crate) const STRETCHBLT: i32 = 77;
// Java field: int MASKBLT
pub(crate) const MASKBLT: i32 = 78;
// Java field: int PLGBLT
pub(crate) const PLGBLT: i32 = 79;
// Java field: int SETDIBITSTODEVICE
pub(crate) const SETDIBITSTODEVICE: i32 = 80;
// Java field: int STRETCHDIBITS
pub(crate) const STRETCHDIBITS: i32 = 81;
// Java field: int EXTCREATEFONTINDIRECTW
pub(crate) const EXTCREATEFONTINDIRECTW: i32 = 82;
// Java field: int EXTTEXTOUTA
pub(crate) const EXTTEXTOUTA: i32 = 83;
// Java field: int EXTTEXTOUTW
pub(crate) const EXTTEXTOUTW: i32 = 84;
// Java field: int POLYBEZIER16
pub(crate) const POLYBEZIER16: i32 = 85;
// Java field: int POLYGON16
pub(crate) const POLYGON16: i32 = 86;
// Java field: int POLYLINE16
pub(crate) const POLYLINE16: i32 = 87;
// Java field: int POLYBEZIERTO16
pub(crate) const POLYBEZIERTO16: i32 = 88;
// Java field: int POLYLINETO16
pub(crate) const POLYLINETO16: i32 = 89;
// Java field: int POLYPOLYLINE16
pub(crate) const POLYPOLYLINE16: i32 = 90;
// Java field: int POLYPOLYGON16
pub(crate) const POLYPOLYGON16: i32 = 91;
// Java field: int POLYDRAW16
pub(crate) const POLYDRAW16: i32 = 92;
// Java field: int CREATEMONOBRUSH
pub(crate) const CREATEMONOBRUSH: i32 = 93;
// Java field: int CREATEDIBPATTERNBRUSHPT
pub(crate) const CREATEDIBPATTERNBRUSHPT: i32 = 94;
// Java field: int EXTCREATEPEN
pub(crate) const EXTCREATEPEN: i32 = 95;
// Java field: int POLYTEXTOUTA
pub(crate) const POLYTEXTOUTA: i32 = 96;
// Java field: int POLYTEXTOUTW
pub(crate) const POLYTEXTOUTW: i32 = 97;
// Java field: int SETICMMODE
pub(crate) const SETICMMODE: i32 = 98;
// Java field: int CREATECOLORSPACE
pub(crate) const CREATECOLORSPACE: i32 = 99;
// Java field: int SETCOLORSPACE
pub(crate) const SETCOLORSPACE: i32 = 100;
// Java field: int DELETECOLORSPACE
pub(crate) const DELETECOLORSPACE: i32 = 101;
// Java field: int GLSRECORD
pub(crate) const GLSRECORD: i32 = 102;
// Java field: int GLSBOUNDEDRECORD
pub(crate) const GLSBOUNDEDRECORD: i32 = 103;
// Java field: int PIXELFORMAT
pub(crate) const PIXELFORMAT: i32 = 104;
// Java field: int DRAWESCAPE
pub(crate) const DRAWESCAPE: i32 = 105;
// Java field: int EXTESCAPE
pub(crate) const EXTESCAPE: i32 = 106;
// Java field: int STARTDOC
pub(crate) const STARTDOC: i32 = 107;
// Java field: int SMALLTEXTOUT
pub(crate) const SMALLTEXTOUT: i32 = 108;
// Java field: int FORCEUFIMAPPING
pub(crate) const FORCEUFIMAPPING: i32 = 109;
// Java field: int NAMEDESCAPE
pub(crate) const NAMEDESCAPE: i32 = 110;
// Java field: int COLORCORRECTPALETTE
pub(crate) const COLORCORRECTPALETTE: i32 = 111;
// Java field: int SETICMPROFILEA
pub(crate) const SETICMPROFILEA: i32 = 112;
// Java field: int SETICMPROFILEW
pub(crate) const SETICMPROFILEW: i32 = 113;
// Java field: int ALPHABLEND
pub(crate) const ALPHABLEND: i32 = 114;
// Java field: int SETLAYOUT
pub(crate) const SETLAYOUT: i32 = 115;
// Java field: int TRANSPARENTBLT
pub(crate) const TRANSPARENTBLT: i32 = 116;
// Java field: int RESERVED_117
pub(crate) const RESERVED_117: i32 = 117;
// Java field: int GRADIENTFILL
pub(crate) const GRADIENTFILL: i32 = 118;
// Java field: int SETLINKEDUFIS
pub(crate) const SETLINKEDUFIS: i32 = 119;
// Java field: int SETTEXTJUSTIFICATION
pub(crate) const SETTEXTJUSTIFICATION: i32 = 120;
// Java field: int COLORMATCHTOTARGETW
pub(crate) const COLORMATCHTOTARGETW: i32 = 121;
// Java field: int CREATECOLORSPACEW
pub(crate) const CREATECOLORSPACEW: i32 = 122;

// Java: public class EmfRecordID
pub struct EmfRecordID {
}

// Java: public class EmfRecordID
impl EmfRecordID {
    // Java: EmfRecordID()
    pub fn new() -> Self {
      // Java: super();
      EmfRecordID {}
    }

}
