// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import cn.wps.kfc.api.set.IIntSet
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap

use std::collections::HashMap;
use std::sync::OnceLock;
use super::EscherPropertyMetaData::EscherPropertyMetaData;

// Java field: short TRANSFORM__ROTATION
pub const TRANSFORM__ROTATION: i16 = 4;
// Java field: short PROTECTION__LOCKROTATION
pub const PROTECTION__LOCKROTATION: i16 = 119;
// Java field: short PROTECTION__LOCKASPECTRATIO
pub const PROTECTION__LOCKASPECTRATIO: i16 = 120;
// Java field: short PROTECTION__LOCKPOSITION
pub const PROTECTION__LOCKPOSITION: i16 = 121;
// Java field: short PROTECTION__LOCKAGAINSTSELECT
pub const PROTECTION__LOCKAGAINSTSELECT: i16 = 122;
// Java field: short PROTECTION__LOCKCROPPING
pub const PROTECTION__LOCKCROPPING: i16 = 123;
// Java field: short PROTECTION__LOCKVERTICES
pub const PROTECTION__LOCKVERTICES: i16 = 124;
// Java field: short PROTECTION__LOCKTEXT
pub const PROTECTION__LOCKTEXT: i16 = 125;
// Java field: short PROTECTION__LOCKADJUSTHANDLES
pub const PROTECTION__LOCKADJUSTHANDLES: i16 = 126;
// Java field: short PROTECTION__LOCKAGAINSTGROUPING
pub const PROTECTION__LOCKAGAINSTGROUPING: i16 = 127;
// Java field: short TEXT__TEXTID
pub const TEXT__TEXTID: i16 = 128;
// Java field: short TEXT__TEXTLEFT
pub const TEXT__TEXTLEFT: i16 = 129;
// Java field: short TEXT__TEXTTOP
pub const TEXT__TEXTTOP: i16 = 130;
// Java field: short TEXT__TEXTRIGHT
pub const TEXT__TEXTRIGHT: i16 = 131;
// Java field: short TEXT__TEXTBOTTOM
pub const TEXT__TEXTBOTTOM: i16 = 132;
// Java field: short TEXT__WRAPTEXT
pub const TEXT__WRAPTEXT: i16 = 133;
// Java field: short TEXT__SCALETEXT
pub const TEXT__SCALETEXT: i16 = 134;
// Java field: short TEXT__ANCHORTEXT
pub const TEXT__ANCHORTEXT: i16 = 135;
// Java field: short TEXT__TEXTFLOW
pub const TEXT__TEXTFLOW: i16 = 136;
// Java field: short TEXT__FONTROTATION
pub const TEXT__FONTROTATION: i16 = 137;
// Java field: short TEXT__IDOFNEXTSHAPE
pub const TEXT__IDOFNEXTSHAPE: i16 = 138;
// Java field: short TEXT__BIDIR
pub const TEXT__BIDIR: i16 = 139;
// Java field: short TEXT__SINGLECLICKSELECTS
pub const TEXT__SINGLECLICKSELECTS: i16 = 187;
// Java field: short TEXT__USEHOSTMARGINS
pub const TEXT__USEHOSTMARGINS: i16 = 188;
// Java field: short TEXT__ROTATETEXTWITHSHAPE
pub const TEXT__ROTATETEXTWITHSHAPE: i16 = 189;
// Java field: short TEXT__SIZESHAPETOFITTEXT
pub const TEXT__SIZESHAPETOFITTEXT: i16 = 190;
// Java field: short TEXT__SIZE_TEXT_TO_FIT_SHAPE
pub const TEXT__SIZE_TEXT_TO_FIT_SHAPE: i16 = 191;
// Java field: short GEOTEXT__UNICODE
pub const GEOTEXT__UNICODE: i16 = 192;
// Java field: short GEOTEXT__RTFTEXT
pub const GEOTEXT__RTFTEXT: i16 = 193;
// Java field: short GEOTEXT__ALIGNMENTONCURVE
pub const GEOTEXT__ALIGNMENTONCURVE: i16 = 194;
// Java field: short GEOTEXT__DEFAULTPOINTSIZE
pub const GEOTEXT__DEFAULTPOINTSIZE: i16 = 195;
// Java field: short GEOTEXT__TEXTSPACING
pub const GEOTEXT__TEXTSPACING: i16 = 196;
// Java field: short GEOTEXT__FONTFAMILYNAME
pub const GEOTEXT__FONTFAMILYNAME: i16 = 197;
// Java field: short GEOTEXT__REVERSEROWORDER
pub const GEOTEXT__REVERSEROWORDER: i16 = 240;
// Java field: short GEOTEXT__HASTEXTEFFECT
pub const GEOTEXT__HASTEXTEFFECT: i16 = 241;
// Java field: short GEOTEXT__ROTATECHARACTERS
pub const GEOTEXT__ROTATECHARACTERS: i16 = 242;
// Java field: short GEOTEXT__KERNCHARACTERS
pub const GEOTEXT__KERNCHARACTERS: i16 = 243;
// Java field: short GEOTEXT__TIGHTORTRACK
pub const GEOTEXT__TIGHTORTRACK: i16 = 244;
// Java field: short GEOTEXT__STRETCHTOFITSHAPE
pub const GEOTEXT__STRETCHTOFITSHAPE: i16 = 245;
// Java field: short GEOTEXT__CHARBOUNDINGBOX
pub const GEOTEXT__CHARBOUNDINGBOX: i16 = 246;
// Java field: short GEOTEXT__SCALETEXTONPATH
pub const GEOTEXT__SCALETEXTONPATH: i16 = 247;
// Java field: short GEOTEXT__STRETCHCHARHEIGHT
pub const GEOTEXT__STRETCHCHARHEIGHT: i16 = 248;
// Java field: short GEOTEXT__NOMEASUREALONGPATH
pub const GEOTEXT__NOMEASUREALONGPATH: i16 = 249;
// Java field: short GEOTEXT__BOLDFONT
pub const GEOTEXT__BOLDFONT: i16 = 250;
// Java field: short GEOTEXT__ITALICFONT
pub const GEOTEXT__ITALICFONT: i16 = 251;
// Java field: short GEOTEXT__UNDERLINEFONT
pub const GEOTEXT__UNDERLINEFONT: i16 = 252;
// Java field: short GEOTEXT__SHADOWFONT
pub const GEOTEXT__SHADOWFONT: i16 = 253;
// Java field: short GEOTEXT__SMALLCAPSFONT
pub const GEOTEXT__SMALLCAPSFONT: i16 = 254;
// Java field: short GEOTEXT__STRIKETHROUGHFONT
pub const GEOTEXT__STRIKETHROUGHFONT: i16 = 255;
// Java field: short BLIP__CROPFROMTOP
pub const BLIP__CROPFROMTOP: i16 = 256;
// Java field: short BLIP__CROPFROMBOTTOM
pub const BLIP__CROPFROMBOTTOM: i16 = 257;
// Java field: short BLIP__CROPFROMLEFT
pub const BLIP__CROPFROMLEFT: i16 = 258;
// Java field: short BLIP__CROPFROMRIGHT
pub const BLIP__CROPFROMRIGHT: i16 = 259;
// Java field: short BLIP__BLIPTODISPLAY
pub const BLIP__BLIPTODISPLAY: i16 = 260;
// Java field: short BLIP__BLIPFILENAME
pub const BLIP__BLIPFILENAME: i16 = 261;
// Java field: short BLIP__BLIPFLAGS
pub const BLIP__BLIPFLAGS: i16 = 262;
// Java field: short BLIP__TRANSPARENTCOLOR
pub const BLIP__TRANSPARENTCOLOR: i16 = 263;
// Java field: short BLIP__CONTRASTSETTING
pub const BLIP__CONTRASTSETTING: i16 = 264;
// Java field: short BLIP__BRIGHTNESSSETTING
pub const BLIP__BRIGHTNESSSETTING: i16 = 265;
// Java field: short BLIP__GAMMA
pub const BLIP__GAMMA: i16 = 266;
// Java field: short BLIP__PICTUREID
pub const BLIP__PICTUREID: i16 = 267;
// Java field: short BLIP__DOUBLEMOD
pub const BLIP__DOUBLEMOD: i16 = 268;
// Java field: short BLIP__PICTUREFILLMOD
pub const BLIP__PICTUREFILLMOD: i16 = 269;
// Java field: short BLIP__PICTURELINE
pub const BLIP__PICTURELINE: i16 = 270;
// Java field: short BLIP__PRINTBLIP
pub const BLIP__PRINTBLIP: i16 = 271;
// Java field: short BLIP__PRINTBLIPFILENAME
pub const BLIP__PRINTBLIPFILENAME: i16 = 272;
// Java field: short BLIP__PRINTFLAGS
pub const BLIP__PRINTFLAGS: i16 = 273;
// Java field: short BLIP__NOHITTESTPICTURE
pub const BLIP__NOHITTESTPICTURE: i16 = 316;
// Java field: short BLIP__PICTUREGRAY
pub const BLIP__PICTUREGRAY: i16 = 317;
// Java field: short BLIP__PICTUREBILEVEL
pub const BLIP__PICTUREBILEVEL: i16 = 318;
// Java field: short BLIP__PICTUREACTIVE
pub const BLIP__PICTUREACTIVE: i16 = 319;
// Java field: short GEOMETRY__LEFT
pub const GEOMETRY__LEFT: i16 = 320;
// Java field: short GEOMETRY__TOP
pub const GEOMETRY__TOP: i16 = 321;
// Java field: short GEOMETRY__RIGHT
pub const GEOMETRY__RIGHT: i16 = 322;
// Java field: short GEOMETRY__BOTTOM
pub const GEOMETRY__BOTTOM: i16 = 323;
// Java field: short GEOMETRY__SHAPEPATH
pub const GEOMETRY__SHAPEPATH: i16 = 324;
// Java field: short GEOMETRY__VERTICES
pub const GEOMETRY__VERTICES: i16 = 325;
// Java field: short GEOMETRY__SEGMENTINFO
pub const GEOMETRY__SEGMENTINFO: i16 = 326;
// Java field: short GEOMETRY__ADJUSTVALUE
pub const GEOMETRY__ADJUSTVALUE: i16 = 327;
// Java field: short GEOMETRY__ADJUST2VALUE
pub const GEOMETRY__ADJUST2VALUE: i16 = 328;
// Java field: short GEOMETRY__ADJUST3VALUE
pub const GEOMETRY__ADJUST3VALUE: i16 = 329;
// Java field: short GEOMETRY__ADJUST4VALUE
pub const GEOMETRY__ADJUST4VALUE: i16 = 330;
// Java field: short GEOMETRY__ADJUST5VALUE
pub const GEOMETRY__ADJUST5VALUE: i16 = 331;
// Java field: short GEOMETRY__ADJUST6VALUE
pub const GEOMETRY__ADJUST6VALUE: i16 = 332;
// Java field: short GEOMETRY__ADJUST7VALUE
pub const GEOMETRY__ADJUST7VALUE: i16 = 333;
// Java field: short GEOMETRY__ADJUST8VALUE
pub const GEOMETRY__ADJUST8VALUE: i16 = 334;
// Java field: short GEOMETRY__ADJUST9VALUE
pub const GEOMETRY__ADJUST9VALUE: i16 = 335;
// Java field: short GEOMETRY__ADJUST10VALUE
pub const GEOMETRY__ADJUST10VALUE: i16 = 336;
// Java field: short GEOMETRY__CONNECTIONSITES
pub const GEOMETRY__CONNECTIONSITES: i16 = 337;
// Java field: short GEOMETRY__CONNECTIONSITES_DIR
pub const GEOMETRY__CONNECTIONSITES_DIR: i16 = 338;
// Java field: short GEOMETRY__ADJUST_HANDLERS
pub const GEOMETRY__ADJUST_HANDLERS: i16 = 341;
// Java field: short GEOMETRY__GUIDES
pub const GEOMETRY__GUIDES: i16 = 342;
// Java field: short GEOMETRY__INSCRIBE
pub const GEOMETRY__INSCRIBE: i16 = 343;
// Java field: short GEOMETRY__SHADOWok
pub const GEOMETRY__SHADOWok: i16 = 378;
// Java field: short GEOMETRY__3DOK
pub const GEOMETRY__3DOK: i16 = 379;
// Java field: short GEOMETRY__LINEOK
pub const GEOMETRY__LINEOK: i16 = 380;
// Java field: short GEOMETRY__GEOTEXTOK
pub const GEOMETRY__GEOTEXTOK: i16 = 381;
// Java field: short GEOMETRY__FILLSHADESHAPEOK
pub const GEOMETRY__FILLSHADESHAPEOK: i16 = 382;
// Java field: short GEOMETRY__FILLOK
pub const GEOMETRY__FILLOK: i16 = 383;
// Java field: short FILL__FILLTYPE
pub const FILL__FILLTYPE: i16 = 384;
// Java field: short FILL__FILLCOLOR
pub const FILL__FILLCOLOR: i16 = 385;
// Java field: short FILL__FILLOPACITY
pub const FILL__FILLOPACITY: i16 = 386;
// Java field: short FILL__FILLBACKCOLOR
pub const FILL__FILLBACKCOLOR: i16 = 387;
// Java field: short FILL__BACKOPACITY
pub const FILL__BACKOPACITY: i16 = 388;
// Java field: short FILL__CRMOD
pub const FILL__CRMOD: i16 = 389;
// Java field: short FILL__BLIP
pub const FILL__BLIP: i16 = 390;
// Java field: short FILL__BLIPFILENAME
pub const FILL__BLIPFILENAME: i16 = 391;
// Java field: short FILL__BLIPFLAGS
pub const FILL__BLIPFLAGS: i16 = 392;
// Java field: short FILL__WIDTH
pub const FILL__WIDTH: i16 = 393;
// Java field: short FILL__HEIGHT
pub const FILL__HEIGHT: i16 = 394;
// Java field: short FILL__ANGLE
pub const FILL__ANGLE: i16 = 395;
// Java field: short FILL__FOCUS
pub const FILL__FOCUS: i16 = 396;
// Java field: short FILL__TOLEFT
pub const FILL__TOLEFT: i16 = 397;
// Java field: short FILL__TOTOP
pub const FILL__TOTOP: i16 = 398;
// Java field: short FILL__TORIGHT
pub const FILL__TORIGHT: i16 = 399;
// Java field: short FILL__TOBOTTOM
pub const FILL__TOBOTTOM: i16 = 400;
// Java field: short FILL__RECTLEFT
pub const FILL__RECTLEFT: i16 = 401;
// Java field: short FILL__RECTTOP
pub const FILL__RECTTOP: i16 = 402;
// Java field: short FILL__RECTRIGHT
pub const FILL__RECTRIGHT: i16 = 403;
// Java field: short FILL__RECTBOTTOM
pub const FILL__RECTBOTTOM: i16 = 404;
// Java field: short FILL__DZTYPE
pub const FILL__DZTYPE: i16 = 405;
// Java field: short FILL__SHADEPRESET
pub const FILL__SHADEPRESET: i16 = 406;
// Java field: short FILL__SHADECOLORS
pub const FILL__SHADECOLORS: i16 = 407;
// Java field: short FILL__ORIGINX
pub const FILL__ORIGINX: i16 = 408;
// Java field: short FILL__ORIGINY
pub const FILL__ORIGINY: i16 = 409;
// Java field: short FILL__SHAPEORIGINX
pub const FILL__SHAPEORIGINX: i16 = 410;
// Java field: short FILL__SHAPEORIGINY
pub const FILL__SHAPEORIGINY: i16 = 411;
// Java field: short FILL__SHADETYPE
pub const FILL__SHADETYPE: i16 = 412;
// Java field: short FILL__FILLED
pub const FILL__FILLED: i16 = 443;
// Java field: short FILL__HITTESTFILL
pub const FILL__HITTESTFILL: i16 = 444;
// Java field: short FILL__SHAPE
pub const FILL__SHAPE: i16 = 445;
// Java field: short FILL__USERECT
pub const FILL__USERECT: i16 = 446;
// Java field: short FILL__NOFILLHITTEST
pub const FILL__NOFILLHITTEST: i16 = 447;
// Java field: short LINESTYLE__COLOR
pub const LINESTYLE__COLOR: i16 = 448;
// Java field: short LINESTYLE__OPACITY
pub const LINESTYLE__OPACITY: i16 = 449;
// Java field: short LINESTYLE__BACKCOLOR
pub const LINESTYLE__BACKCOLOR: i16 = 450;
// Java field: short LINESTYLE__CRMOD
pub const LINESTYLE__CRMOD: i16 = 451;
// Java field: short LINESTYLE__LINETYPE
pub const LINESTYLE__LINETYPE: i16 = 452;
// Java field: short LINESTYLE__FILLBLIP
pub const LINESTYLE__FILLBLIP: i16 = 453;
// Java field: short LINESTYLE__FILLBLIPNAME
pub const LINESTYLE__FILLBLIPNAME: i16 = 454;
// Java field: short LINESTYLE__FILLBLIPFLAGS
pub const LINESTYLE__FILLBLIPFLAGS: i16 = 455;
// Java field: short LINESTYLE__FILLWIDTH
pub const LINESTYLE__FILLWIDTH: i16 = 456;
// Java field: short LINESTYLE__FILLHEIGHT
pub const LINESTYLE__FILLHEIGHT: i16 = 457;
// Java field: short LINESTYLE__FILLDZTYPE
pub const LINESTYLE__FILLDZTYPE: i16 = 458;
// Java field: short LINESTYLE__LINEWIDTH
pub const LINESTYLE__LINEWIDTH: i16 = 459;
// Java field: short LINESTYLE__LINEMITERLIMIT
pub const LINESTYLE__LINEMITERLIMIT: i16 = 460;
// Java field: short LINESTYLE__LINESTYLE
pub const LINESTYLE__LINESTYLE: i16 = 461;
// Java field: short LINESTYLE__LINEDASHING
pub const LINESTYLE__LINEDASHING: i16 = 462;
// Java field: short LINESTYLE__LINEDASHSTYLE
pub const LINESTYLE__LINEDASHSTYLE: i16 = 463;
// Java field: short LINESTYLE__LINESTARTARROWHEAD
pub const LINESTYLE__LINESTARTARROWHEAD: i16 = 464;
// Java field: short LINESTYLE__LINEENDARROWHEAD
pub const LINESTYLE__LINEENDARROWHEAD: i16 = 465;
// Java field: short LINESTYLE__LINESTARTARROWWIDTH
pub const LINESTYLE__LINESTARTARROWWIDTH: i16 = 466;
// Java field: short LINESTYLE__LINEESTARTARROWLENGTH
pub const LINESTYLE__LINEESTARTARROWLENGTH: i16 = 467;
// Java field: short LINESTYLE__LINEENDARROWWIDTH
pub const LINESTYLE__LINEENDARROWWIDTH: i16 = 468;
// Java field: short LINESTYLE__LINEENDARROWLENGTH
pub const LINESTYLE__LINEENDARROWLENGTH: i16 = 469;
// Java field: short LINESTYLE__LINEJOINSTYLE
pub const LINESTYLE__LINEJOINSTYLE: i16 = 470;
// Java field: short LINESTYLE__LINEENDCAPSTYLE
pub const LINESTYLE__LINEENDCAPSTYLE: i16 = 471;
// Java field: short LINESTYLE__ARROWHEADSOK
pub const LINESTYLE__ARROWHEADSOK: i16 = 507;
// Java field: short LINESTYLE__ANYLINE
pub const LINESTYLE__ANYLINE: i16 = 508;
// Java field: short LINESTYLE__HITLINETEST
pub const LINESTYLE__HITLINETEST: i16 = 509;
// Java field: short LINESTYLE__LINEFILLSHAPE
pub const LINESTYLE__LINEFILLSHAPE: i16 = 510;
// Java field: short LINESTYLE__NOLINEDRAWDASH
pub const LINESTYLE__NOLINEDRAWDASH: i16 = 511;
// Java field: short SHADOWSTYLE__TYPE
pub const SHADOWSTYLE__TYPE: i16 = 512;
// Java field: short SHADOWSTYLE__COLOR
pub const SHADOWSTYLE__COLOR: i16 = 513;
// Java field: short SHADOWSTYLE__HIGHLIGHT
pub const SHADOWSTYLE__HIGHLIGHT: i16 = 514;
// Java field: short SHADOWSTYLE__CRMOD
pub const SHADOWSTYLE__CRMOD: i16 = 515;
// Java field: short SHADOWSTYLE__OPACITY
pub const SHADOWSTYLE__OPACITY: i16 = 516;
// Java field: short SHADOWSTYLE__OFFSETX
pub const SHADOWSTYLE__OFFSETX: i16 = 517;
// Java field: short SHADOWSTYLE__OFFSETY
pub const SHADOWSTYLE__OFFSETY: i16 = 518;
// Java field: short SHADOWSTYLE__SECONDOFFSETX
pub const SHADOWSTYLE__SECONDOFFSETX: i16 = 519;
// Java field: short SHADOWSTYLE__SECONDOFFSETY
pub const SHADOWSTYLE__SECONDOFFSETY: i16 = 520;
// Java field: short SHADOWSTYLE__SCALEXTOX
pub const SHADOWSTYLE__SCALEXTOX: i16 = 521;
// Java field: short SHADOWSTYLE__SCALEYTOX
pub const SHADOWSTYLE__SCALEYTOX: i16 = 522;
// Java field: short SHADOWSTYLE__SCALEXTOY
pub const SHADOWSTYLE__SCALEXTOY: i16 = 523;
// Java field: short SHADOWSTYLE__SCALEYTOY
pub const SHADOWSTYLE__SCALEYTOY: i16 = 524;
// Java field: short SHADOWSTYLE__PERSPECTIVEX
pub const SHADOWSTYLE__PERSPECTIVEX: i16 = 525;
// Java field: short SHADOWSTYLE__PERSPECTIVEY
pub const SHADOWSTYLE__PERSPECTIVEY: i16 = 526;
// Java field: short SHADOWSTYLE__WEIGHT
pub const SHADOWSTYLE__WEIGHT: i16 = 527;
// Java field: short SHADOWSTYLE__ORIGINX
pub const SHADOWSTYLE__ORIGINX: i16 = 528;
// Java field: short SHADOWSTYLE__ORIGINY
pub const SHADOWSTYLE__ORIGINY: i16 = 529;
// Java field: short SHADOWSTYLE__SHADOW
pub const SHADOWSTYLE__SHADOW: i16 = 574;
// Java field: short SHADOWSTYLE__SHADOWOBSURED
pub const SHADOWSTYLE__SHADOWOBSURED: i16 = 575;
// Java field: short PERSPECTIVE__TYPE
pub const PERSPECTIVE__TYPE: i16 = 576;
// Java field: short PERSPECTIVE__OFFSETX
pub const PERSPECTIVE__OFFSETX: i16 = 577;
// Java field: short PERSPECTIVE__OFFSETY
pub const PERSPECTIVE__OFFSETY: i16 = 578;
// Java field: short PERSPECTIVE__SCALEXTOX
pub const PERSPECTIVE__SCALEXTOX: i16 = 579;
// Java field: short PERSPECTIVE__SCALEYTOX
pub const PERSPECTIVE__SCALEYTOX: i16 = 580;
// Java field: short PERSPECTIVE__SCALEXTOY
pub const PERSPECTIVE__SCALEXTOY: i16 = 581;
// Java field: short PERSPECTIVE__SCALEYTOY
pub const PERSPECTIVE__SCALEYTOY: i16 = 582;
// Java field: short PERSPECTIVE__PERSPECTIVEX
pub const PERSPECTIVE__PERSPECTIVEX: i16 = 583;
// Java field: short PERSPECTIVE__PERSPECTIVEY
pub const PERSPECTIVE__PERSPECTIVEY: i16 = 584;
// Java field: short PERSPECTIVE__WEIGHT
pub const PERSPECTIVE__WEIGHT: i16 = 585;
// Java field: short PERSPECTIVE__ORIGINX
pub const PERSPECTIVE__ORIGINX: i16 = 586;
// Java field: short PERSPECTIVE__ORIGINY
pub const PERSPECTIVE__ORIGINY: i16 = 587;
// Java field: short PERSPECTIVE__PERSPECTIVEON
pub const PERSPECTIVE__PERSPECTIVEON: i16 = 639;
// Java field: short THREED__SPECULARAMOUNT
pub const THREED__SPECULARAMOUNT: i16 = 640;
// Java field: short THREED__DIFFUSEAMOUNT
pub const THREED__DIFFUSEAMOUNT: i16 = 641;
// Java field: short THREED__SHININESS
pub const THREED__SHININESS: i16 = 642;
// Java field: short THREED__EDGETHICKNESS
pub const THREED__EDGETHICKNESS: i16 = 643;
// Java field: short THREED__EXTRUDEFORWARD
pub const THREED__EXTRUDEFORWARD: i16 = 644;
// Java field: short THREED__EXTRUDEBACKWARD
pub const THREED__EXTRUDEBACKWARD: i16 = 645;
// Java field: short THREED__EXTRUDEPLANE
pub const THREED__EXTRUDEPLANE: i16 = 646;
// Java field: short THREED__EXTRUSIONCOLOR
pub const THREED__EXTRUSIONCOLOR: i16 = 647;
// Java field: short THREED__CRMOD
pub const THREED__CRMOD: i16 = 648;
// Java field: short THREED__3DEFFECT
pub const THREED__3DEFFECT: i16 = 700;
// Java field: short THREED__METALLIC
pub const THREED__METALLIC: i16 = 701;
// Java field: short THREED__USEEXTRUSIONCOLOR
pub const THREED__USEEXTRUSIONCOLOR: i16 = 702;
// Java field: short THREED__LIGHTFACE
pub const THREED__LIGHTFACE: i16 = 703;
// Java field: short THREEDSTYLE__YROTATIONANGLE
pub const THREEDSTYLE__YROTATIONANGLE: i16 = 704;
// Java field: short THREEDSTYLE__XROTATIONANGLE
pub const THREEDSTYLE__XROTATIONANGLE: i16 = 705;
// Java field: short THREEDSTYLE__ROTATIONAXISX
pub const THREEDSTYLE__ROTATIONAXISX: i16 = 706;
// Java field: short THREEDSTYLE__ROTATIONAXISY
pub const THREEDSTYLE__ROTATIONAXISY: i16 = 707;
// Java field: short THREEDSTYLE__ROTATIONAXISZ
pub const THREEDSTYLE__ROTATIONAXISZ: i16 = 708;
// Java field: short THREEDSTYLE__ROTATIONANGLE
pub const THREEDSTYLE__ROTATIONANGLE: i16 = 709;
// Java field: short THREEDSTYLE__ROTATIONCENTERX
pub const THREEDSTYLE__ROTATIONCENTERX: i16 = 710;
// Java field: short THREEDSTYLE__ROTATIONCENTERY
pub const THREEDSTYLE__ROTATIONCENTERY: i16 = 711;
// Java field: short THREEDSTYLE__ROTATIONCENTERZ
pub const THREEDSTYLE__ROTATIONCENTERZ: i16 = 712;
// Java field: short THREEDSTYLE__RENDERMODE
pub const THREEDSTYLE__RENDERMODE: i16 = 713;
// Java field: short THREEDSTYLE__TOLERANCE
pub const THREEDSTYLE__TOLERANCE: i16 = 714;
// Java field: short THREEDSTYLE__XVIEWPOINT
pub const THREEDSTYLE__XVIEWPOINT: i16 = 715;
// Java field: short THREEDSTYLE__YVIEWPOINT
pub const THREEDSTYLE__YVIEWPOINT: i16 = 716;
// Java field: short THREEDSTYLE__ZVIEWPOINT
pub const THREEDSTYLE__ZVIEWPOINT: i16 = 717;
// Java field: short THREEDSTYLE__ORIGINX
pub const THREEDSTYLE__ORIGINX: i16 = 718;
// Java field: short THREEDSTYLE__ORIGINY
pub const THREEDSTYLE__ORIGINY: i16 = 719;
// Java field: short THREEDSTYLE__SKEWANGLE
pub const THREEDSTYLE__SKEWANGLE: i16 = 720;
// Java field: short THREEDSTYLE__SKEWAMOUNT
pub const THREEDSTYLE__SKEWAMOUNT: i16 = 721;
// Java field: short THREEDSTYLE__AMBIENTINTENSITY
pub const THREEDSTYLE__AMBIENTINTENSITY: i16 = 722;
// Java field: short THREEDSTYLE__KEYX
pub const THREEDSTYLE__KEYX: i16 = 723;
// Java field: short THREEDSTYLE__KEYY
pub const THREEDSTYLE__KEYY: i16 = 724;
// Java field: short THREEDSTYLE__KEYZ
pub const THREEDSTYLE__KEYZ: i16 = 725;
// Java field: short THREEDSTYLE__KEYINTENSITY
pub const THREEDSTYLE__KEYINTENSITY: i16 = 726;
// Java field: short THREEDSTYLE__FILLX
pub const THREEDSTYLE__FILLX: i16 = 727;
// Java field: short THREEDSTYLE__FILLY
pub const THREEDSTYLE__FILLY: i16 = 728;
// Java field: short THREEDSTYLE__FILLZ
pub const THREEDSTYLE__FILLZ: i16 = 729;
// Java field: short THREEDSTYLE__FILLINTENSITY
pub const THREEDSTYLE__FILLINTENSITY: i16 = 730;
// Java field: short THREEDSTYLE__CONSTRAINROTATION
pub const THREEDSTYLE__CONSTRAINROTATION: i16 = 763;
// Java field: short THREEDSTYLE__ROTATIONCENTERAUTO
pub const THREEDSTYLE__ROTATIONCENTERAUTO: i16 = 764;
// Java field: short THREEDSTYLE__PARALLEL
pub const THREEDSTYLE__PARALLEL: i16 = 765;
// Java field: short THREEDSTYLE__KEYHARSH
pub const THREEDSTYLE__KEYHARSH: i16 = 766;
// Java field: short THREEDSTYLE__FILLHARSH
pub const THREEDSTYLE__FILLHARSH: i16 = 767;
// Java field: short SHAPE__MASTER
pub const SHAPE__MASTER: i16 = 769;
// Java field: short SHAPE__CONNECTORSTYLE
pub const SHAPE__CONNECTORSTYLE: i16 = 771;
// Java field: short SHAPE__BLACKANDWHITESETTINGS
pub const SHAPE__BLACKANDWHITESETTINGS: i16 = 772;
// Java field: short SHAPE__WMODEPUREBW
pub const SHAPE__WMODEPUREBW: i16 = 773;
// Java field: short SHAPE__WMODEBW
pub const SHAPE__WMODEBW: i16 = 774;
// Java field: short SHAPE__OLEICON
pub const SHAPE__OLEICON: i16 = 826;
// Java field: short SHAPE__PREFERRELATIVERESIZE
pub const SHAPE__PREFERRELATIVERESIZE: i16 = 827;
// Java field: short SHAPE__LOCKSHAPETYPE
pub const SHAPE__LOCKSHAPETYPE: i16 = 828;
// Java field: short SHAPE__DELETEATTACHEDOBJECT
pub const SHAPE__DELETEATTACHEDOBJECT: i16 = 830;
// Java field: short SHAPE__BACKGROUNDSHAPE
pub const SHAPE__BACKGROUNDSHAPE: i16 = 831;
// Java field: short CALLOUT__CALLOUTTYPE
pub const CALLOUT__CALLOUTTYPE: i16 = 832;
// Java field: short CALLOUT__XYCALLOUTGAP
pub const CALLOUT__XYCALLOUTGAP: i16 = 833;
// Java field: short CALLOUT__CALLOUTANGLE
pub const CALLOUT__CALLOUTANGLE: i16 = 834;
// Java field: short CALLOUT__CALLOUTDROPTYPE
pub const CALLOUT__CALLOUTDROPTYPE: i16 = 835;
// Java field: short CALLOUT__CALLOUTDROPSPECIFIED
pub const CALLOUT__CALLOUTDROPSPECIFIED: i16 = 836;
// Java field: short CALLOUT__CALLOUTLENGTHSPECIFIED
pub const CALLOUT__CALLOUTLENGTHSPECIFIED: i16 = 837;
// Java field: short CALLOUT__ISCALLOUT
pub const CALLOUT__ISCALLOUT: i16 = 889;
// Java field: short CALLOUT__CALLOUTACCENTBAR
pub const CALLOUT__CALLOUTACCENTBAR: i16 = 890;
// Java field: short CALLOUT__CALLOUTTEXTBORDER
pub const CALLOUT__CALLOUTTEXTBORDER: i16 = 891;
// Java field: short CALLOUT__CALLOUTMINUSX
pub const CALLOUT__CALLOUTMINUSX: i16 = 892;
// Java field: short CALLOUT__CALLOUTMINUSY
pub const CALLOUT__CALLOUTMINUSY: i16 = 893;
// Java field: short CALLOUT__DROPAUTO
pub const CALLOUT__DROPAUTO: i16 = 894;
// Java field: short CALLOUT__LENGTHSPECIFIED
pub const CALLOUT__LENGTHSPECIFIED: i16 = 895;
// Java field: short GROUPSHAPE__SHAPENAME
pub const GROUPSHAPE__SHAPENAME: i16 = 896;
// Java field: short GROUPSHAPE__DESCRIPTION
pub const GROUPSHAPE__DESCRIPTION: i16 = 897;
// Java field: short GROUPSHAPE__HYPERLINK
pub const GROUPSHAPE__HYPERLINK: i16 = 898;
// Java field: short GROUPSHAPE__WRAPPOLYGONVERTICES
pub const GROUPSHAPE__WRAPPOLYGONVERTICES: i16 = 899;
// Java field: short GROUPSHAPE__WRAPDISTLEFT
pub const GROUPSHAPE__WRAPDISTLEFT: i16 = 900;
// Java field: short GROUPSHAPE__WRAPDISTTOP
pub const GROUPSHAPE__WRAPDISTTOP: i16 = 901;
// Java field: short GROUPSHAPE__WRAPDISTRIGHT
pub const GROUPSHAPE__WRAPDISTRIGHT: i16 = 902;
// Java field: short GROUPSHAPE__WRAPDISTBOTTOM
pub const GROUPSHAPE__WRAPDISTBOTTOM: i16 = 903;
// Java field: short GROUPSHAPE__REGROUPID
pub const GROUPSHAPE__REGROUPID: i16 = 904;
// Java field: short GROUPSHAPE__EDITEDWRAP
pub const GROUPSHAPE__EDITEDWRAP: i16 = 953;
// Java field: short GROUPSHAPE__BEHINDDOCUMENT
pub const GROUPSHAPE__BEHINDDOCUMENT: i16 = 954;
// Java field: short GROUPSHAPE__ONDBLCLICKNOTIFY
pub const GROUPSHAPE__ONDBLCLICKNOTIFY: i16 = 955;
// Java field: short GROUPSHAPE__ISBUTTON
pub const GROUPSHAPE__ISBUTTON: i16 = 956;
// Java field: short GROUPSHAPE__1DADJUSTMENT
pub const GROUPSHAPE__1DADJUSTMENT: i16 = 957;
// Java field: short GROUPSHAPE__HIDDEN
pub const GROUPSHAPE__HIDDEN: i16 = 958;
// Java field: short GROUPSHAPE__PRINT
pub const GROUPSHAPE__PRINT: i16 = 959;

// Java field: IIntObjectMap<EscherPropertyMetaData> properties

// Java: public class EscherProperties
pub struct EscherProperties;

// Java: public class EscherProperties
impl EscherProperties {
    // Java: EscherProperties()
    pub fn new() -> Self {
        // Java: super();
        EscherProperties
    }

    // Java: static IIntObjectMap<EscherPropertyMetaData> initProps()
    fn initProps() -> HashMap<i16, EscherPropertyMetaData> {
        // Java: IIntObjectMap<EscherPropertyMetaData> m = new WIntObjectHashMap(276);
        let mut m: HashMap<i16, EscherPropertyMetaData> = HashMap::with_capacity(276);
        // Java: m.put(4, new EscherPropertyMetaData("transform.rotation"));
        m.insert(4, EscherPropertyMetaData::new("transform.rotation"));
        // Java: m.put(119, new EscherPropertyMetaData("protection.lockrotation"));
        m.insert(119, EscherPropertyMetaData::new("protection.lockrotation"));
        // Java: m.put(120, new EscherPropertyMetaData("protection.lockaspectratio"));
        m.insert(120, EscherPropertyMetaData::new("protection.lockaspectratio"));
        // Java: m.put(121, new EscherPropertyMetaData("protection.lockposition"));
        m.insert(121, EscherPropertyMetaData::new("protection.lockposition"));
        // Java: m.put(122, new EscherPropertyMetaData("protection.lockagainstselect"));
        m.insert(122, EscherPropertyMetaData::new("protection.lockagainstselect"));
        // Java: m.put(123, new EscherPropertyMetaData("protection.lockcropping"));
        m.insert(123, EscherPropertyMetaData::new("protection.lockcropping"));
        // Java: m.put(124, new EscherPropertyMetaData("protection.lockvertices"));
        m.insert(124, EscherPropertyMetaData::new("protection.lockvertices"));
        // Java: m.put(125, new EscherPropertyMetaData("protection.locktext"));
        m.insert(125, EscherPropertyMetaData::new("protection.locktext"));
        // Java: m.put(126, new EscherPropertyMetaData("protection.lockadjusthandles"));
        m.insert(126, EscherPropertyMetaData::new("protection.lockadjusthandles"));
        // Java: m.put(127, new EscherPropertyMetaData("protection.lockagainstgrouping", (byte)1));
        m.insert(127, EscherPropertyMetaData::new_with_type("protection.lockagainstgrouping", 1));
        // Java: m.put(128, new EscherPropertyMetaData("text.textid"));
        m.insert(128, EscherPropertyMetaData::new("text.textid"));
        // Java: m.put(129, new EscherPropertyMetaData("text.textleft"));
        m.insert(129, EscherPropertyMetaData::new("text.textleft"));
        // Java: m.put(130, new EscherPropertyMetaData("text.texttop"));
        m.insert(130, EscherPropertyMetaData::new("text.texttop"));
        // Java: m.put(131, new EscherPropertyMetaData("text.textright"));
        m.insert(131, EscherPropertyMetaData::new("text.textright"));
        // Java: m.put(132, new EscherPropertyMetaData("text.textbottom"));
        m.insert(132, EscherPropertyMetaData::new("text.textbottom"));
        // Java: m.put(133, new EscherPropertyMetaData("text.wraptext"));
        m.insert(133, EscherPropertyMetaData::new("text.wraptext"));
        // Java: m.put(134, new EscherPropertyMetaData("text.scaletext"));
        m.insert(134, EscherPropertyMetaData::new("text.scaletext"));
        // Java: m.put(135, new EscherPropertyMetaData("text.anchortext"));
        m.insert(135, EscherPropertyMetaData::new("text.anchortext"));
        // Java: m.put(136, new EscherPropertyMetaData("text.textflow"));
        m.insert(136, EscherPropertyMetaData::new("text.textflow"));
        // Java: m.put(137, new EscherPropertyMetaData("text.fontrotation"));
        m.insert(137, EscherPropertyMetaData::new("text.fontrotation"));
        // Java: m.put(138, new EscherPropertyMetaData("text.idofnextshape"));
        m.insert(138, EscherPropertyMetaData::new("text.idofnextshape"));
        // Java: m.put(139, new EscherPropertyMetaData("text.bidir"));
        m.insert(139, EscherPropertyMetaData::new("text.bidir"));
        // Java: m.put(187, new EscherPropertyMetaData("text.singleclickselects"));
        m.insert(187, EscherPropertyMetaData::new("text.singleclickselects"));
        // Java: m.put(188, new EscherPropertyMetaData("text.usehostmargins"));
        m.insert(188, EscherPropertyMetaData::new("text.usehostmargins"));
        // Java: m.put(189, new EscherPropertyMetaData("text.rotatetextwithshape"));
        m.insert(189, EscherPropertyMetaData::new("text.rotatetextwithshape"));
        // Java: m.put(190, new EscherPropertyMetaData("text.sizeshapetofittext"));
        m.insert(190, EscherPropertyMetaData::new("text.sizeshapetofittext"));
        // Java: m.put(191, new EscherPropertyMetaData("text.sizetexttofitshape", (byte)1));
        m.insert(191, EscherPropertyMetaData::new_with_type("text.sizetexttofitshape", 1));
        // Java: m.put(192, new EscherPropertyMetaData("geotext.unicode"));
        m.insert(192, EscherPropertyMetaData::new("geotext.unicode"));
        // Java: m.put(193, new EscherPropertyMetaData("geotext.rtftext"));
        m.insert(193, EscherPropertyMetaData::new("geotext.rtftext"));
        // Java: m.put(194, new EscherPropertyMetaData("geotext.alignmentoncurve"));
        m.insert(194, EscherPropertyMetaData::new("geotext.alignmentoncurve"));
        // Java: m.put(195, new EscherPropertyMetaData("geotext.defaultpointsize"));
        m.insert(195, EscherPropertyMetaData::new("geotext.defaultpointsize"));
        // Java: m.put(196, new EscherPropertyMetaData("geotext.textspacing"));
        m.insert(196, EscherPropertyMetaData::new("geotext.textspacing"));
        // Java: m.put(197, new EscherPropertyMetaData("geotext.fontfamilyname"));
        m.insert(197, EscherPropertyMetaData::new("geotext.fontfamilyname"));
        // Java: m.put(240, new EscherPropertyMetaData("geotext.reverseroworder"));
        m.insert(240, EscherPropertyMetaData::new("geotext.reverseroworder"));
        // Java: m.put(241, new EscherPropertyMetaData("geotext.hastexteffect"));
        m.insert(241, EscherPropertyMetaData::new("geotext.hastexteffect"));
        // Java: m.put(242, new EscherPropertyMetaData("geotext.rotatecharacters"));
        m.insert(242, EscherPropertyMetaData::new("geotext.rotatecharacters"));
        // Java: m.put(243, new EscherPropertyMetaData("geotext.kerncharacters"));
        m.insert(243, EscherPropertyMetaData::new("geotext.kerncharacters"));
        // Java: m.put(244, new EscherPropertyMetaData("geotext.tightortrack"));
        m.insert(244, EscherPropertyMetaData::new("geotext.tightortrack"));
        // Java: m.put(245, new EscherPropertyMetaData("geotext.stretchtofitshape"));
        m.insert(245, EscherPropertyMetaData::new("geotext.stretchtofitshape"));
        // Java: m.put(246, new EscherPropertyMetaData("geotext.charboundingbox"));
        m.insert(246, EscherPropertyMetaData::new("geotext.charboundingbox"));
        // Java: m.put(247, new EscherPropertyMetaData("geotext.scaletextonpath"));
        m.insert(247, EscherPropertyMetaData::new("geotext.scaletextonpath"));
        // Java: m.put(248, new EscherPropertyMetaData("geotext.stretchcharheight"));
        m.insert(248, EscherPropertyMetaData::new("geotext.stretchcharheight"));
        // Java: m.put(249, new EscherPropertyMetaData("geotext.nomeasurealongpath"));
        m.insert(249, EscherPropertyMetaData::new("geotext.nomeasurealongpath"));
        // Java: m.put(250, new EscherPropertyMetaData("geotext.boldfont"));
        m.insert(250, EscherPropertyMetaData::new("geotext.boldfont"));
        // Java: m.put(251, new EscherPropertyMetaData("geotext.italicfont"));
        m.insert(251, EscherPropertyMetaData::new("geotext.italicfont"));
        // Java: m.put(252, new EscherPropertyMetaData("geotext.underlinefont"));
        m.insert(252, EscherPropertyMetaData::new("geotext.underlinefont"));
        // Java: m.put(253, new EscherPropertyMetaData("geotext.shadowfont"));
        m.insert(253, EscherPropertyMetaData::new("geotext.shadowfont"));
        // Java: m.put(254, new EscherPropertyMetaData("geotext.smallcapsfont"));
        m.insert(254, EscherPropertyMetaData::new("geotext.smallcapsfont"));
        // Java: m.put(255, new EscherPropertyMetaData("geotext.strikethroughfont"));
        m.insert(255, EscherPropertyMetaData::new("geotext.strikethroughfont"));
        // Java: m.put(256, new EscherPropertyMetaData("blip.cropfromtop"));
        m.insert(256, EscherPropertyMetaData::new("blip.cropfromtop"));
        // Java: m.put(257, new EscherPropertyMetaData("blip.cropfrombottom"));
        m.insert(257, EscherPropertyMetaData::new("blip.cropfrombottom"));
        // Java: m.put(258, new EscherPropertyMetaData("blip.cropfromleft"));
        m.insert(258, EscherPropertyMetaData::new("blip.cropfromleft"));
        // Java: m.put(259, new EscherPropertyMetaData("blip.cropfromright"));
        m.insert(259, EscherPropertyMetaData::new("blip.cropfromright"));
        // Java: m.put(260, new EscherPropertyMetaData("blip.bliptodisplay"));
        m.insert(260, EscherPropertyMetaData::new("blip.bliptodisplay"));
        // Java: m.put(261, new EscherPropertyMetaData("blip.blipfilename"));
        m.insert(261, EscherPropertyMetaData::new("blip.blipfilename"));
        // Java: m.put(262, new EscherPropertyMetaData("blip.blipflags"));
        m.insert(262, EscherPropertyMetaData::new("blip.blipflags"));
        // Java: m.put(263, new EscherPropertyMetaData("blip.transparentcolor"));
        m.insert(263, EscherPropertyMetaData::new("blip.transparentcolor"));
        // Java: m.put(264, new EscherPropertyMetaData("blip.contrastsetting"));
        m.insert(264, EscherPropertyMetaData::new("blip.contrastsetting"));
        // Java: m.put(265, new EscherPropertyMetaData("blip.brightnesssetting"));
        m.insert(265, EscherPropertyMetaData::new("blip.brightnesssetting"));
        // Java: m.put(266, new EscherPropertyMetaData("blip.gamma"));
        m.insert(266, EscherPropertyMetaData::new("blip.gamma"));
        // Java: m.put(267, new EscherPropertyMetaData("blip.pictureid"));
        m.insert(267, EscherPropertyMetaData::new("blip.pictureid"));
        // Java: m.put(268, new EscherPropertyMetaData("blip.doublemod"));
        m.insert(268, EscherPropertyMetaData::new("blip.doublemod"));
        // Java: m.put(269, new EscherPropertyMetaData("blip.picturefillmod"));
        m.insert(269, EscherPropertyMetaData::new("blip.picturefillmod"));
        // Java: m.put(270, new EscherPropertyMetaData("blip.pictureline"));
        m.insert(270, EscherPropertyMetaData::new("blip.pictureline"));
        // Java: m.put(271, new EscherPropertyMetaData("blip.printblip"));
        m.insert(271, EscherPropertyMetaData::new("blip.printblip"));
        // Java: m.put(272, new EscherPropertyMetaData("blip.printblipfilename"));
        m.insert(272, EscherPropertyMetaData::new("blip.printblipfilename"));
        // Java: m.put(273, new EscherPropertyMetaData("blip.printflags"));
        m.insert(273, EscherPropertyMetaData::new("blip.printflags"));
        // Java: m.put(316, new EscherPropertyMetaData("blip.nohittestpicture"));
        m.insert(316, EscherPropertyMetaData::new("blip.nohittestpicture"));
        // Java: m.put(317, new EscherPropertyMetaData("blip.picturegray"));
        m.insert(317, EscherPropertyMetaData::new("blip.picturegray"));
        // Java: m.put(318, new EscherPropertyMetaData("blip.picturebilevel"));
        m.insert(318, EscherPropertyMetaData::new("blip.picturebilevel"));
        // Java: m.put(319, new EscherPropertyMetaData("blip.pictureactive"));
        m.insert(319, EscherPropertyMetaData::new("blip.pictureactive"));
        // Java: m.put(320, new EscherPropertyMetaData("geometry.left"));
        m.insert(320, EscherPropertyMetaData::new("geometry.left"));
        // Java: m.put(321, new EscherPropertyMetaData("geometry.top"));
        m.insert(321, EscherPropertyMetaData::new("geometry.top"));
        // Java: m.put(322, new EscherPropertyMetaData("geometry.right"));
        m.insert(322, EscherPropertyMetaData::new("geometry.right"));
        // Java: m.put(323, new EscherPropertyMetaData("geometry.bottom"));
        m.insert(323, EscherPropertyMetaData::new("geometry.bottom"));
        // Java: m.put(324, new EscherPropertyMetaData("geometry.shapepath", (byte)3));
        m.insert(324, EscherPropertyMetaData::new_with_type("geometry.shapepath", 3));
        // Java: m.put(325, new EscherPropertyMetaData("geometry.vertices", (byte)5));
        m.insert(325, EscherPropertyMetaData::new_with_type("geometry.vertices", 5));
        // Java: m.put(326, new EscherPropertyMetaData("geometry.segmentinfo", (byte)5));
        m.insert(326, EscherPropertyMetaData::new_with_type("geometry.segmentinfo", 5));
        // Java: m.put(337, new EscherPropertyMetaData("geometry.connectionsites"));
        m.insert(337, EscherPropertyMetaData::new("geometry.connectionsites"));
        // Java: m.put(338, new EscherPropertyMetaData("geometry.connectionsites_dir"));
        m.insert(338, EscherPropertyMetaData::new("geometry.connectionsites_dir"));
        // Java: m.put(341, new EscherPropertyMetaData("geometry.adjusthandles", (byte)5));
        m.insert(341, EscherPropertyMetaData::new_with_type("geometry.adjusthandles", 5));
        // Java: m.put(342, new EscherPropertyMetaData("geometry.guides", (byte)5));
        m.insert(342, EscherPropertyMetaData::new_with_type("geometry.guides", 5));
        // Java: m.put(327, new EscherPropertyMetaData("geometry.adjustvalue"));
        m.insert(327, EscherPropertyMetaData::new("geometry.adjustvalue"));
        // Java: m.put(328, new EscherPropertyMetaData("geometry.adjust2value"));
        m.insert(328, EscherPropertyMetaData::new("geometry.adjust2value"));
        // Java: m.put(329, new EscherPropertyMetaData("geometry.adjust3value"));
        m.insert(329, EscherPropertyMetaData::new("geometry.adjust3value"));
        // Java: m.put(330, new EscherPropertyMetaData("geometry.adjust4value"));
        m.insert(330, EscherPropertyMetaData::new("geometry.adjust4value"));
        // Java: m.put(331, new EscherPropertyMetaData("geometry.adjust5value"));
        m.insert(331, EscherPropertyMetaData::new("geometry.adjust5value"));
        // Java: m.put(332, new EscherPropertyMetaData("geometry.adjust6value"));
        m.insert(332, EscherPropertyMetaData::new("geometry.adjust6value"));
        // Java: m.put(333, new EscherPropertyMetaData("geometry.adjust7value"));
        m.insert(333, EscherPropertyMetaData::new("geometry.adjust7value"));
        // Java: m.put(334, new EscherPropertyMetaData("geometry.adjust8value"));
        m.insert(334, EscherPropertyMetaData::new("geometry.adjust8value"));
        // Java: m.put(335, new EscherPropertyMetaData("geometry.adjust9value"));
        m.insert(335, EscherPropertyMetaData::new("geometry.adjust9value"));
        // Java: m.put(336, new EscherPropertyMetaData("geometry.adjust10value"));
        m.insert(336, EscherPropertyMetaData::new("geometry.adjust10value"));
        // Java: m.put(378, new EscherPropertyMetaData("geometry.shadowOK"));
        m.insert(378, EscherPropertyMetaData::new("geometry.shadowOK"));
        // Java: m.put(379, new EscherPropertyMetaData("geometry.3dok"));
        m.insert(379, EscherPropertyMetaData::new("geometry.3dok"));
        // Java: m.put(380, new EscherPropertyMetaData("geometry.lineok"));
        m.insert(380, EscherPropertyMetaData::new("geometry.lineok"));
        // Java: m.put(381, new EscherPropertyMetaData("geometry.geotextok"));
        m.insert(381, EscherPropertyMetaData::new("geometry.geotextok"));
        // Java: m.put(382, new EscherPropertyMetaData("geometry.fillshadeshapeok"));
        m.insert(382, EscherPropertyMetaData::new("geometry.fillshadeshapeok"));
        // Java: m.put(383, new EscherPropertyMetaData("geometry.fillok", (byte)1));
        m.insert(383, EscherPropertyMetaData::new_with_type("geometry.fillok", 1));
        // Java: m.put(343, new EscherPropertyMetaData("geometry.inscribe", (byte)5));
        m.insert(343, EscherPropertyMetaData::new_with_type("geometry.inscribe", 5));
        // Java: m.put(384, new EscherPropertyMetaData("fill.filltype"));
        m.insert(384, EscherPropertyMetaData::new("fill.filltype"));
        // Java: m.put(385, new EscherPropertyMetaData("fill.fillcolor", (byte)2));
        m.insert(385, EscherPropertyMetaData::new_with_type("fill.fillcolor", 2));
        // Java: m.put(386, new EscherPropertyMetaData("fill.fillopacity"));
        m.insert(386, EscherPropertyMetaData::new("fill.fillopacity"));
        // Java: m.put(387, new EscherPropertyMetaData("fill.fillbackcolor", (byte)2));
        m.insert(387, EscherPropertyMetaData::new_with_type("fill.fillbackcolor", 2));
        // Java: m.put(388, new EscherPropertyMetaData("fill.backopacity"));
        m.insert(388, EscherPropertyMetaData::new("fill.backopacity"));
        // Java: m.put(389, new EscherPropertyMetaData("fill.crmod"));
        m.insert(389, EscherPropertyMetaData::new("fill.crmod"));
        // Java: m.put(390, new EscherPropertyMetaData("fill.blip"));
        m.insert(390, EscherPropertyMetaData::new("fill.blip"));
        // Java: m.put(391, new EscherPropertyMetaData("fill.blipfilename"));
        m.insert(391, EscherPropertyMetaData::new("fill.blipfilename"));
        // Java: m.put(392, new EscherPropertyMetaData("fill.blipflags"));
        m.insert(392, EscherPropertyMetaData::new("fill.blipflags"));
        // Java: m.put(393, new EscherPropertyMetaData("fill.width"));
        m.insert(393, EscherPropertyMetaData::new("fill.width"));
        // Java: m.put(394, new EscherPropertyMetaData("fill.height"));
        m.insert(394, EscherPropertyMetaData::new("fill.height"));
        // Java: m.put(395, new EscherPropertyMetaData("fill.angle"));
        m.insert(395, EscherPropertyMetaData::new("fill.angle"));
        // Java: m.put(396, new EscherPropertyMetaData("fill.focus"));
        m.insert(396, EscherPropertyMetaData::new("fill.focus"));
        // Java: m.put(397, new EscherPropertyMetaData("fill.toleft"));
        m.insert(397, EscherPropertyMetaData::new("fill.toleft"));
        // Java: m.put(398, new EscherPropertyMetaData("fill.totop"));
        m.insert(398, EscherPropertyMetaData::new("fill.totop"));
        // Java: m.put(399, new EscherPropertyMetaData("fill.toright"));
        m.insert(399, EscherPropertyMetaData::new("fill.toright"));
        // Java: m.put(400, new EscherPropertyMetaData("fill.tobottom"));
        m.insert(400, EscherPropertyMetaData::new("fill.tobottom"));
        // Java: m.put(401, new EscherPropertyMetaData("fill.rectleft"));
        m.insert(401, EscherPropertyMetaData::new("fill.rectleft"));
        // Java: m.put(402, new EscherPropertyMetaData("fill.recttop"));
        m.insert(402, EscherPropertyMetaData::new("fill.recttop"));
        // Java: m.put(403, new EscherPropertyMetaData("fill.rectright"));
        m.insert(403, EscherPropertyMetaData::new("fill.rectright"));
        // Java: m.put(404, new EscherPropertyMetaData("fill.rectbottom"));
        m.insert(404, EscherPropertyMetaData::new("fill.rectbottom"));
        // Java: m.put(405, new EscherPropertyMetaData("fill.dztype"));
        m.insert(405, EscherPropertyMetaData::new("fill.dztype"));
        // Java: m.put(406, new EscherPropertyMetaData("fill.shadepreset"));
        m.insert(406, EscherPropertyMetaData::new("fill.shadepreset"));
        // Java: m.put(407, new EscherPropertyMetaData("fill.shadecolors", (byte)5));
        m.insert(407, EscherPropertyMetaData::new_with_type("fill.shadecolors", 5));
        // Java: m.put(408, new EscherPropertyMetaData("fill.originx"));
        m.insert(408, EscherPropertyMetaData::new("fill.originx"));
        // Java: m.put(409, new EscherPropertyMetaData("fill.originy"));
        m.insert(409, EscherPropertyMetaData::new("fill.originy"));
        // Java: m.put(410, new EscherPropertyMetaData("fill.shapeoriginx"));
        m.insert(410, EscherPropertyMetaData::new("fill.shapeoriginx"));
        // Java: m.put(411, new EscherPropertyMetaData("fill.shapeoriginy"));
        m.insert(411, EscherPropertyMetaData::new("fill.shapeoriginy"));
        // Java: m.put(412, new EscherPropertyMetaData("fill.shadetype"));
        m.insert(412, EscherPropertyMetaData::new("fill.shadetype"));
        // Java: m.put(443, new EscherPropertyMetaData("fill.filled"));
        m.insert(443, EscherPropertyMetaData::new("fill.filled"));
        // Java: m.put(444, new EscherPropertyMetaData("fill.hittestfill"));
        m.insert(444, EscherPropertyMetaData::new("fill.hittestfill"));
        // Java: m.put(445, new EscherPropertyMetaData("fill.shape"));
        m.insert(445, EscherPropertyMetaData::new("fill.shape"));
        // Java: m.put(446, new EscherPropertyMetaData("fill.userect"));
        m.insert(446, EscherPropertyMetaData::new("fill.userect"));
        // Java: m.put(447, new EscherPropertyMetaData("fill.nofillhittest", (byte)1));
        m.insert(447, EscherPropertyMetaData::new_with_type("fill.nofillhittest", 1));
        // Java: m.put(448, new EscherPropertyMetaData("linestyle.color", (byte)2));
        m.insert(448, EscherPropertyMetaData::new_with_type("linestyle.color", 2));
        // Java: m.put(449, new EscherPropertyMetaData("linestyle.opacity"));
        m.insert(449, EscherPropertyMetaData::new("linestyle.opacity"));
        // Java: m.put(450, new EscherPropertyMetaData("linestyle.backcolor", (byte)2));
        m.insert(450, EscherPropertyMetaData::new_with_type("linestyle.backcolor", 2));
        // Java: m.put(451, new EscherPropertyMetaData("linestyle.crmod"));
        m.insert(451, EscherPropertyMetaData::new("linestyle.crmod"));
        // Java: m.put(452, new EscherPropertyMetaData("linestyle.linetype"));
        m.insert(452, EscherPropertyMetaData::new("linestyle.linetype"));
        // Java: m.put(453, new EscherPropertyMetaData("linestyle.fillblip"));
        m.insert(453, EscherPropertyMetaData::new("linestyle.fillblip"));
        // Java: m.put(454, new EscherPropertyMetaData("linestyle.fillblipname"));
        m.insert(454, EscherPropertyMetaData::new("linestyle.fillblipname"));
        // Java: m.put(455, new EscherPropertyMetaData("linestyle.fillblipflags"));
        m.insert(455, EscherPropertyMetaData::new("linestyle.fillblipflags"));
        // Java: m.put(456, new EscherPropertyMetaData("linestyle.fillwidth"));
        m.insert(456, EscherPropertyMetaData::new("linestyle.fillwidth"));
        // Java: m.put(457, new EscherPropertyMetaData("linestyle.fillheight"));
        m.insert(457, EscherPropertyMetaData::new("linestyle.fillheight"));
        // Java: m.put(458, new EscherPropertyMetaData("linestyle.filldztype"));
        m.insert(458, EscherPropertyMetaData::new("linestyle.filldztype"));
        // Java: m.put(459, new EscherPropertyMetaData("linestyle.linewidth"));
        m.insert(459, EscherPropertyMetaData::new("linestyle.linewidth"));
        // Java: m.put(460, new EscherPropertyMetaData("linestyle.linemiterlimit"));
        m.insert(460, EscherPropertyMetaData::new("linestyle.linemiterlimit"));
        // Java: m.put(461, new EscherPropertyMetaData("linestyle.linestyle"));
        m.insert(461, EscherPropertyMetaData::new("linestyle.linestyle"));
        // Java: m.put(462, new EscherPropertyMetaData("linestyle.linedashing"));
        m.insert(462, EscherPropertyMetaData::new("linestyle.linedashing"));
        // Java: m.put(463, new EscherPropertyMetaData("linestyle.linedashstyle", (byte)5));
        m.insert(463, EscherPropertyMetaData::new_with_type("linestyle.linedashstyle", 5));
        // Java: m.put(464, new EscherPropertyMetaData("linestyle.linestartarrowhead"));
        m.insert(464, EscherPropertyMetaData::new("linestyle.linestartarrowhead"));
        // Java: m.put(465, new EscherPropertyMetaData("linestyle.lineendarrowhead"));
        m.insert(465, EscherPropertyMetaData::new("linestyle.lineendarrowhead"));
        // Java: m.put(466, new EscherPropertyMetaData("linestyle.linestartarrowwidth"));
        m.insert(466, EscherPropertyMetaData::new("linestyle.linestartarrowwidth"));
        // Java: m.put(467, new EscherPropertyMetaData("linestyle.lineestartarrowlength"));
        m.insert(467, EscherPropertyMetaData::new("linestyle.lineestartarrowlength"));
        // Java: m.put(468, new EscherPropertyMetaData("linestyle.lineendarrowwidth"));
        m.insert(468, EscherPropertyMetaData::new("linestyle.lineendarrowwidth"));
        // Java: m.put(469, new EscherPropertyMetaData("linestyle.lineendarrowlength"));
        m.insert(469, EscherPropertyMetaData::new("linestyle.lineendarrowlength"));
        // Java: m.put(470, new EscherPropertyMetaData("linestyle.linejoinstyle"));
        m.insert(470, EscherPropertyMetaData::new("linestyle.linejoinstyle"));
        // Java: m.put(471, new EscherPropertyMetaData("linestyle.lineendcapstyle"));
        m.insert(471, EscherPropertyMetaData::new("linestyle.lineendcapstyle"));
        // Java: m.put(507, new EscherPropertyMetaData("linestyle.arrowheadsok"));
        m.insert(507, EscherPropertyMetaData::new("linestyle.arrowheadsok"));
        // Java: m.put(508, new EscherPropertyMetaData("linestyle.anyline"));
        m.insert(508, EscherPropertyMetaData::new("linestyle.anyline"));
        // Java: m.put(509, new EscherPropertyMetaData("linestyle.hitlinetest"));
        m.insert(509, EscherPropertyMetaData::new("linestyle.hitlinetest"));
        // Java: m.put(510, new EscherPropertyMetaData("linestyle.linefillshape"));
        m.insert(510, EscherPropertyMetaData::new("linestyle.linefillshape"));
        // Java: m.put(511, new EscherPropertyMetaData("linestyle.nolinedrawdash", (byte)1));
        m.insert(511, EscherPropertyMetaData::new_with_type("linestyle.nolinedrawdash", 1));
        // Java: m.put(512, new EscherPropertyMetaData("shadowstyle.type"));
        m.insert(512, EscherPropertyMetaData::new("shadowstyle.type"));
        // Java: m.put(513, new EscherPropertyMetaData("shadowstyle.color", (byte)2));
        m.insert(513, EscherPropertyMetaData::new_with_type("shadowstyle.color", 2));
        // Java: m.put(514, new EscherPropertyMetaData("shadowstyle.highlight"));
        m.insert(514, EscherPropertyMetaData::new("shadowstyle.highlight"));
        // Java: m.put(515, new EscherPropertyMetaData("shadowstyle.crmod"));
        m.insert(515, EscherPropertyMetaData::new("shadowstyle.crmod"));
        // Java: m.put(516, new EscherPropertyMetaData("shadowstyle.opacity"));
        m.insert(516, EscherPropertyMetaData::new("shadowstyle.opacity"));
        // Java: m.put(517, new EscherPropertyMetaData("shadowstyle.offsetx"));
        m.insert(517, EscherPropertyMetaData::new("shadowstyle.offsetx"));
        // Java: m.put(518, new EscherPropertyMetaData("shadowstyle.offsety"));
        m.insert(518, EscherPropertyMetaData::new("shadowstyle.offsety"));
        // Java: m.put(519, new EscherPropertyMetaData("shadowstyle.secondoffsetx"));
        m.insert(519, EscherPropertyMetaData::new("shadowstyle.secondoffsetx"));
        // Java: m.put(520, new EscherPropertyMetaData("shadowstyle.secondoffsety"));
        m.insert(520, EscherPropertyMetaData::new("shadowstyle.secondoffsety"));
        // Java: m.put(521, new EscherPropertyMetaData("shadowstyle.scalextox"));
        m.insert(521, EscherPropertyMetaData::new("shadowstyle.scalextox"));
        // Java: m.put(522, new EscherPropertyMetaData("shadowstyle.scaleytox"));
        m.insert(522, EscherPropertyMetaData::new("shadowstyle.scaleytox"));
        // Java: m.put(523, new EscherPropertyMetaData("shadowstyle.scalextoy"));
        m.insert(523, EscherPropertyMetaData::new("shadowstyle.scalextoy"));
        // Java: m.put(524, new EscherPropertyMetaData("shadowstyle.scaleytoy"));
        m.insert(524, EscherPropertyMetaData::new("shadowstyle.scaleytoy"));
        // Java: m.put(525, new EscherPropertyMetaData("shadowstyle.perspectivex"));
        m.insert(525, EscherPropertyMetaData::new("shadowstyle.perspectivex"));
        // Java: m.put(526, new EscherPropertyMetaData("shadowstyle.perspectivey"));
        m.insert(526, EscherPropertyMetaData::new("shadowstyle.perspectivey"));
        // Java: m.put(527, new EscherPropertyMetaData("shadowstyle.weight"));
        m.insert(527, EscherPropertyMetaData::new("shadowstyle.weight"));
        // Java: m.put(528, new EscherPropertyMetaData("shadowstyle.originx"));
        m.insert(528, EscherPropertyMetaData::new("shadowstyle.originx"));
        // Java: m.put(529, new EscherPropertyMetaData("shadowstyle.originy"));
        m.insert(529, EscherPropertyMetaData::new("shadowstyle.originy"));
        // Java: m.put(574, new EscherPropertyMetaData("shadowstyle.shadow"));
        m.insert(574, EscherPropertyMetaData::new("shadowstyle.shadow"));
        // Java: m.put(575, new EscherPropertyMetaData("shadowstyle.shadowobsured"));
        m.insert(575, EscherPropertyMetaData::new("shadowstyle.shadowobsured"));
        // Java: m.put(576, new EscherPropertyMetaData("perspective.type"));
        m.insert(576, EscherPropertyMetaData::new("perspective.type"));
        // Java: m.put(577, new EscherPropertyMetaData("perspective.offsetx"));
        m.insert(577, EscherPropertyMetaData::new("perspective.offsetx"));
        // Java: m.put(578, new EscherPropertyMetaData("perspective.offsety"));
        m.insert(578, EscherPropertyMetaData::new("perspective.offsety"));
        // Java: m.put(579, new EscherPropertyMetaData("perspective.scalextox"));
        m.insert(579, EscherPropertyMetaData::new("perspective.scalextox"));
        // Java: m.put(580, new EscherPropertyMetaData("perspective.scaleytox"));
        m.insert(580, EscherPropertyMetaData::new("perspective.scaleytox"));
        // Java: m.put(581, new EscherPropertyMetaData("perspective.scalextoy"));
        m.insert(581, EscherPropertyMetaData::new("perspective.scalextoy"));
        // Java: m.put(582, new EscherPropertyMetaData("perspective.scaleytoy"));
        m.insert(582, EscherPropertyMetaData::new("perspective.scaleytoy"));
        // Java: m.put(583, new EscherPropertyMetaData("perspective.perspectivex"));
        m.insert(583, EscherPropertyMetaData::new("perspective.perspectivex"));
        // Java: m.put(584, new EscherPropertyMetaData("perspective.perspectivey"));
        m.insert(584, EscherPropertyMetaData::new("perspective.perspectivey"));
        // Java: m.put(585, new EscherPropertyMetaData("perspective.weight"));
        m.insert(585, EscherPropertyMetaData::new("perspective.weight"));
        // Java: m.put(586, new EscherPropertyMetaData("perspective.originx"));
        m.insert(586, EscherPropertyMetaData::new("perspective.originx"));
        // Java: m.put(587, new EscherPropertyMetaData("perspective.originy"));
        m.insert(587, EscherPropertyMetaData::new("perspective.originy"));
        // Java: m.put(639, new EscherPropertyMetaData("perspective.perspectiveon"));
        m.insert(639, EscherPropertyMetaData::new("perspective.perspectiveon"));
        // Java: m.put(640, new EscherPropertyMetaData("3d.specularamount"));
        m.insert(640, EscherPropertyMetaData::new("3d.specularamount"));
        // Java: m.put(641, new EscherPropertyMetaData("3d.diffuseamount"));
        m.insert(641, EscherPropertyMetaData::new("3d.diffuseamount"));
        // Java: m.put(642, new EscherPropertyMetaData("3d.shininess"));
        m.insert(642, EscherPropertyMetaData::new("3d.shininess"));
        // Java: m.put(643, new EscherPropertyMetaData("3d.edgethickness"));
        m.insert(643, EscherPropertyMetaData::new("3d.edgethickness"));
        // Java: m.put(644, new EscherPropertyMetaData("3d.extrudeforward"));
        m.insert(644, EscherPropertyMetaData::new("3d.extrudeforward"));
        // Java: m.put(645, new EscherPropertyMetaData("3d.extrudebackward"));
        m.insert(645, EscherPropertyMetaData::new("3d.extrudebackward"));
        // Java: m.put(646, new EscherPropertyMetaData("3d.extrudeplane"));
        m.insert(646, EscherPropertyMetaData::new("3d.extrudeplane"));
        // Java: m.put(647, new EscherPropertyMetaData("3d.extrusioncolor", (byte)2));
        m.insert(647, EscherPropertyMetaData::new_with_type("3d.extrusioncolor", 2));
        // Java: m.put(648, new EscherPropertyMetaData("3d.crmod"));
        m.insert(648, EscherPropertyMetaData::new("3d.crmod"));
        // Java: m.put(700, new EscherPropertyMetaData("3d.3deffect"));
        m.insert(700, EscherPropertyMetaData::new("3d.3deffect"));
        // Java: m.put(701, new EscherPropertyMetaData("3d.metallic"));
        m.insert(701, EscherPropertyMetaData::new("3d.metallic"));
        // Java: m.put(702, new EscherPropertyMetaData("3d.useextrusioncolor", (byte)2));
        m.insert(702, EscherPropertyMetaData::new_with_type("3d.useextrusioncolor", 2));
        // Java: m.put(703, new EscherPropertyMetaData("3d.lightface"));
        m.insert(703, EscherPropertyMetaData::new("3d.lightface"));
        // Java: m.put(704, new EscherPropertyMetaData("3dstyle.yrotationangle"));
        m.insert(704, EscherPropertyMetaData::new("3dstyle.yrotationangle"));
        // Java: m.put(705, new EscherPropertyMetaData("3dstyle.xrotationangle"));
        m.insert(705, EscherPropertyMetaData::new("3dstyle.xrotationangle"));
        // Java: m.put(706, new EscherPropertyMetaData("3dstyle.rotationaxisx"));
        m.insert(706, EscherPropertyMetaData::new("3dstyle.rotationaxisx"));
        // Java: m.put(707, new EscherPropertyMetaData("3dstyle.rotationaxisy"));
        m.insert(707, EscherPropertyMetaData::new("3dstyle.rotationaxisy"));
        // Java: m.put(708, new EscherPropertyMetaData("3dstyle.rotationaxisz"));
        m.insert(708, EscherPropertyMetaData::new("3dstyle.rotationaxisz"));
        // Java: m.put(709, new EscherPropertyMetaData("3dstyle.rotationangle"));
        m.insert(709, EscherPropertyMetaData::new("3dstyle.rotationangle"));
        // Java: m.put(710, new EscherPropertyMetaData("3dstyle.rotationcenterx"));
        m.insert(710, EscherPropertyMetaData::new("3dstyle.rotationcenterx"));
        // Java: m.put(711, new EscherPropertyMetaData("3dstyle.rotationcentery"));
        m.insert(711, EscherPropertyMetaData::new("3dstyle.rotationcentery"));
        // Java: m.put(712, new EscherPropertyMetaData("3dstyle.rotationcenterz"));
        m.insert(712, EscherPropertyMetaData::new("3dstyle.rotationcenterz"));
        // Java: m.put(713, new EscherPropertyMetaData("3dstyle.rendermode"));
        m.insert(713, EscherPropertyMetaData::new("3dstyle.rendermode"));
        // Java: m.put(714, new EscherPropertyMetaData("3dstyle.tolerance"));
        m.insert(714, EscherPropertyMetaData::new("3dstyle.tolerance"));
        // Java: m.put(715, new EscherPropertyMetaData("3dstyle.xviewpoint"));
        m.insert(715, EscherPropertyMetaData::new("3dstyle.xviewpoint"));
        // Java: m.put(716, new EscherPropertyMetaData("3dstyle.yviewpoint"));
        m.insert(716, EscherPropertyMetaData::new("3dstyle.yviewpoint"));
        // Java: m.put(717, new EscherPropertyMetaData("3dstyle.zviewpoint"));
        m.insert(717, EscherPropertyMetaData::new("3dstyle.zviewpoint"));
        // Java: m.put(718, new EscherPropertyMetaData("3dstyle.originx"));
        m.insert(718, EscherPropertyMetaData::new("3dstyle.originx"));
        // Java: m.put(719, new EscherPropertyMetaData("3dstyle.originy"));
        m.insert(719, EscherPropertyMetaData::new("3dstyle.originy"));
        // Java: m.put(720, new EscherPropertyMetaData("3dstyle.skewangle"));
        m.insert(720, EscherPropertyMetaData::new("3dstyle.skewangle"));
        // Java: m.put(721, new EscherPropertyMetaData("3dstyle.skewamount"));
        m.insert(721, EscherPropertyMetaData::new("3dstyle.skewamount"));
        // Java: m.put(722, new EscherPropertyMetaData("3dstyle.ambientintensity"));
        m.insert(722, EscherPropertyMetaData::new("3dstyle.ambientintensity"));
        // Java: m.put(723, new EscherPropertyMetaData("3dstyle.keyx"));
        m.insert(723, EscherPropertyMetaData::new("3dstyle.keyx"));
        // Java: m.put(724, new EscherPropertyMetaData("3dstyle.keyy"));
        m.insert(724, EscherPropertyMetaData::new("3dstyle.keyy"));
        // Java: m.put(725, new EscherPropertyMetaData("3dstyle.keyz"));
        m.insert(725, EscherPropertyMetaData::new("3dstyle.keyz"));
        // Java: m.put(726, new EscherPropertyMetaData("3dstyle.keyintensity"));
        m.insert(726, EscherPropertyMetaData::new("3dstyle.keyintensity"));
        // Java: m.put(727, new EscherPropertyMetaData("3dstyle.fillx"));
        m.insert(727, EscherPropertyMetaData::new("3dstyle.fillx"));
        // Java: m.put(728, new EscherPropertyMetaData("3dstyle.filly"));
        m.insert(728, EscherPropertyMetaData::new("3dstyle.filly"));
        // Java: m.put(729, new EscherPropertyMetaData("3dstyle.fillz"));
        m.insert(729, EscherPropertyMetaData::new("3dstyle.fillz"));
        // Java: m.put(730, new EscherPropertyMetaData("3dstyle.fillintensity"));
        m.insert(730, EscherPropertyMetaData::new("3dstyle.fillintensity"));
        // Java: m.put(763, new EscherPropertyMetaData("3dstyle.constrainrotation"));
        m.insert(763, EscherPropertyMetaData::new("3dstyle.constrainrotation"));
        // Java: m.put(764, new EscherPropertyMetaData("3dstyle.rotationcenterauto"));
        m.insert(764, EscherPropertyMetaData::new("3dstyle.rotationcenterauto"));
        // Java: m.put(765, new EscherPropertyMetaData("3dstyle.parallel"));
        m.insert(765, EscherPropertyMetaData::new("3dstyle.parallel"));
        // Java: m.put(766, new EscherPropertyMetaData("3dstyle.keyharsh"));
        m.insert(766, EscherPropertyMetaData::new("3dstyle.keyharsh"));
        // Java: m.put(767, new EscherPropertyMetaData("3dstyle.fillharsh"));
        m.insert(767, EscherPropertyMetaData::new("3dstyle.fillharsh"));
        // Java: m.put(769, new EscherPropertyMetaData("shape.master"));
        m.insert(769, EscherPropertyMetaData::new("shape.master"));
        // Java: m.put(771, new EscherPropertyMetaData("shape.connectorstyle"));
        m.insert(771, EscherPropertyMetaData::new("shape.connectorstyle"));
        // Java: m.put(772, new EscherPropertyMetaData("shape.blackandwhitesettings"));
        m.insert(772, EscherPropertyMetaData::new("shape.blackandwhitesettings"));
        // Java: m.put(773, new EscherPropertyMetaData("shape.wmodepurebw"));
        m.insert(773, EscherPropertyMetaData::new("shape.wmodepurebw"));
        // Java: m.put(774, new EscherPropertyMetaData("shape.wmodebw"));
        m.insert(774, EscherPropertyMetaData::new("shape.wmodebw"));
        // Java: m.put(826, new EscherPropertyMetaData("shape.oleicon"));
        m.insert(826, EscherPropertyMetaData::new("shape.oleicon"));
        // Java: m.put(827, new EscherPropertyMetaData("shape.preferrelativeresize"));
        m.insert(827, EscherPropertyMetaData::new("shape.preferrelativeresize"));
        // Java: m.put(828, new EscherPropertyMetaData("shape.lockshapetype"));
        m.insert(828, EscherPropertyMetaData::new("shape.lockshapetype"));
        // Java: m.put(830, new EscherPropertyMetaData("shape.deleteattachedobject"));
        m.insert(830, EscherPropertyMetaData::new("shape.deleteattachedobject"));
        // Java: m.put(831, new EscherPropertyMetaData("shape.backgroundshape"));
        m.insert(831, EscherPropertyMetaData::new("shape.backgroundshape"));
        // Java: m.put(832, new EscherPropertyMetaData("callout.callouttype"));
        m.insert(832, EscherPropertyMetaData::new("callout.callouttype"));
        // Java: m.put(833, new EscherPropertyMetaData("callout.xycalloutgap"));
        m.insert(833, EscherPropertyMetaData::new("callout.xycalloutgap"));
        // Java: m.put(834, new EscherPropertyMetaData("callout.calloutangle"));
        m.insert(834, EscherPropertyMetaData::new("callout.calloutangle"));
        // Java: m.put(835, new EscherPropertyMetaData("callout.calloutdroptype"));
        m.insert(835, EscherPropertyMetaData::new("callout.calloutdroptype"));
        // Java: m.put(836, new EscherPropertyMetaData("callout.calloutdropspecified"));
        m.insert(836, EscherPropertyMetaData::new("callout.calloutdropspecified"));
        // Java: m.put(837, new EscherPropertyMetaData("callout.calloutlengthspecified"));
        m.insert(837, EscherPropertyMetaData::new("callout.calloutlengthspecified"));
        // Java: m.put(889, new EscherPropertyMetaData("callout.iscallout"));
        m.insert(889, EscherPropertyMetaData::new("callout.iscallout"));
        // Java: m.put(890, new EscherPropertyMetaData("callout.calloutaccentbar"));
        m.insert(890, EscherPropertyMetaData::new("callout.calloutaccentbar"));
        // Java: m.put(891, new EscherPropertyMetaData("callout.callouttextborder"));
        m.insert(891, EscherPropertyMetaData::new("callout.callouttextborder"));
        // Java: m.put(892, new EscherPropertyMetaData("callout.calloutminusx"));
        m.insert(892, EscherPropertyMetaData::new("callout.calloutminusx"));
        // Java: m.put(893, new EscherPropertyMetaData("callout.calloutminusy"));
        m.insert(893, EscherPropertyMetaData::new("callout.calloutminusy"));
        // Java: m.put(894, new EscherPropertyMetaData("callout.dropauto"));
        m.insert(894, EscherPropertyMetaData::new("callout.dropauto"));
        // Java: m.put(895, new EscherPropertyMetaData("callout.lengthspecified"));
        m.insert(895, EscherPropertyMetaData::new("callout.lengthspecified"));
        // Java: m.put(896, new EscherPropertyMetaData("groupshape.shapename"));
        m.insert(896, EscherPropertyMetaData::new("groupshape.shapename"));
        // Java: m.put(897, new EscherPropertyMetaData("groupshape.description"));
        m.insert(897, EscherPropertyMetaData::new("groupshape.description"));
        // Java: m.put(898, new EscherPropertyMetaData("groupshape.hyperlink"));
        m.insert(898, EscherPropertyMetaData::new("groupshape.hyperlink"));
        // Java: m.put(899, new EscherPropertyMetaData("groupshape.wrappolygonvertices", (byte)5));
        m.insert(899, EscherPropertyMetaData::new_with_type("groupshape.wrappolygonvertices", 5));
        // Java: m.put(900, new EscherPropertyMetaData("groupshape.wrapdistleft"));
        m.insert(900, EscherPropertyMetaData::new("groupshape.wrapdistleft"));
        // Java: m.put(901, new EscherPropertyMetaData("groupshape.wrapdisttop"));
        m.insert(901, EscherPropertyMetaData::new("groupshape.wrapdisttop"));
        // Java: m.put(902, new EscherPropertyMetaData("groupshape.wrapdistright"));
        m.insert(902, EscherPropertyMetaData::new("groupshape.wrapdistright"));
        // Java: m.put(903, new EscherPropertyMetaData("groupshape.wrapdistbottom"));
        m.insert(903, EscherPropertyMetaData::new("groupshape.wrapdistbottom"));
        // Java: m.put(904, new EscherPropertyMetaData("groupshape.regroupid"));
        m.insert(904, EscherPropertyMetaData::new("groupshape.regroupid"));
        // Java: m.put(953, new EscherPropertyMetaData("groupshape.editedwrap"));
        m.insert(953, EscherPropertyMetaData::new("groupshape.editedwrap"));
        // Java: m.put(954, new EscherPropertyMetaData("groupshape.behinddocument"));
        m.insert(954, EscherPropertyMetaData::new("groupshape.behinddocument"));
        // Java: m.put(955, new EscherPropertyMetaData("groupshape.ondblclicknotify"));
        m.insert(955, EscherPropertyMetaData::new("groupshape.ondblclicknotify"));
        // Java: m.put(956, new EscherPropertyMetaData("groupshape.isbutton"));
        m.insert(956, EscherPropertyMetaData::new("groupshape.isbutton"));
        // Java: m.put(957, new EscherPropertyMetaData("groupshape.1dadjustment"));
        m.insert(957, EscherPropertyMetaData::new("groupshape.1dadjustment"));
        // Java: m.put(958, new EscherPropertyMetaData("groupshape.hidden"));
        m.insert(958, EscherPropertyMetaData::new("groupshape.hidden"));
        // Java: m.put(959, new EscherPropertyMetaData("groupshape.print", (byte)1));
        m.insert(959, EscherPropertyMetaData::new_with_type("groupshape.print", 1));
        // Java: return m;
        m
    }

    // Java: static String getPropertyName(short propertyId)
    pub fn getPropertyName(_property_id: i16) -> String {
        // Java: EscherPropertyMetaData o = (EscherPropertyMetaData)properties.get(propertyId);
        // Java: return o == null ? "unknown" : o.getDescription();
        String::from("unknown")
    }

    // Java: static byte getPropertyType(short propertyId)
    pub fn getPropertyType(_property_id: i16) -> i8 {
        // Java: EscherPropertyMetaData escherPropertyMetaData = (EscherPropertyMetaData)properties.get(propertyId);
        // Java: IIntSet keys = properties.keySet();
        // Java: if (!keys.contains(propertyId))
        {
            // Java: return -1;
            return -1;
        }
        // Java: else
        {
            // Java: return escherPropertyMetaData == null ? 0 : escherPropertyMetaData.getType();
            return 0;
        }
    }
}
