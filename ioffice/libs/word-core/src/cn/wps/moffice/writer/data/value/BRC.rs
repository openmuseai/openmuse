// Java: import cn.wps.collections.IShareObjIndexdMap
use util::collections::IShareObjIndexdMap::*;
// Java: import java.util.HashMap
// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::collections::{HashMap};

// java private static HashMap<BRC, BRC> sPool = new HashMap();
// rust: no-stmt
// java private static BRC sTemp = new BRC();
// rust: no-stmt
// java public static final int brcNone = 0;
pub(crate) const brcNone: i32 = 0;
// java public static final int brcSingle = 1;
pub(crate) const brcSingle: i32 = 1;
// java public static final int brcThick = 2;
pub(crate) const brcThick: i32 = 2;
// java public static final int brcDouble = 3;
pub(crate) const brcDouble: i32 = 3;
// java public static final int brcHairline = 5;
pub(crate) const brcHairline: i32 = 5;
// java public static final int brcDot = 6;
pub(crate) const brcDot: i32 = 6;
// java public static final int brcDashLarge = 7;
pub(crate) const brcDashLarge: i32 = 7;
// java public static final int brcDotDash = 8;
pub(crate) const brcDotDash: i32 = 8;
// java public static final int brcDotDotDash = 9;
pub(crate) const brcDotDotDash: i32 = 9;
// java public static final int brcTriple = 10;
pub(crate) const brcTriple: i32 = 10;
// java public static final int brcThinThickSmall = 11;
pub(crate) const brcThinThickSmall: i32 = 11;
// java public static final int brcThickThinSmall = 12;
pub(crate) const brcThickThinSmall: i32 = 12;
// java public static final int brcThinThickThinSmall = 13;
pub(crate) const brcThinThickThinSmall: i32 = 13;
// java public static final int brcThinThickMedium = 14;
pub(crate) const brcThinThickMedium: i32 = 14;
// java public static final int brcThickThinMedium = 15;
pub(crate) const brcThickThinMedium: i32 = 15;
// java public static final int brcThinThickThinMedium = 16;
pub(crate) const brcThinThickThinMedium: i32 = 16;
// java public static final int brcThinThickLarge = 17;
pub(crate) const brcThinThickLarge: i32 = 17;
// java public static final int brcThickThinLarge = 18;
pub(crate) const brcThickThinLarge: i32 = 18;
// java public static final int brcThinThickThinLarge = 19;
pub(crate) const brcThinThickThinLarge: i32 = 19;
// java public static final int brcWave = 20;
pub(crate) const brcWave: i32 = 20;
// java public static final int brcDoubleWave = 21;
pub(crate) const brcDoubleWave: i32 = 21;
// java public static final int brcDashSmall = 22;
pub(crate) const brcDashSmall: i32 = 22;
// java public static final int brcDashDotStroked = 23;
pub(crate) const brcDashDotStroked: i32 = 23;
// java public static final int brcEmboss = 24;
pub(crate) const brcEmboss: i32 = 24;
// java public static final int brcEngrave = 25;
pub(crate) const brcEngrave: i32 = 25;
// java public static final int brcOutset = 26;
pub(crate) const brcOutset: i32 = 26;
// java public static final int brcInset = 27;
pub(crate) const brcInset: i32 = 27;
// java public static final int apples = 64;
pub(crate) const apples: i32 = 64;
// java public static final int archedScallops = 65;
pub(crate) const archedScallops: i32 = 65;
// java public static final int babyPacifier = 66;
pub(crate) const babyPacifier: i32 = 66;
// java public static final int babyRattle = 67;
pub(crate) const babyRattle: i32 = 67;
// java public static final int balloons3Colors = 68;
pub(crate) const balloons3Colors: i32 = 68;
// java public static final int balloonsHotAir = 69;
pub(crate) const balloonsHotAir: i32 = 69;
// java public static final int basicBlackDashes = 70;
pub(crate) const basicBlackDashes: i32 = 70;
// java public static final int basicBlackDots = 71;
pub(crate) const basicBlackDots: i32 = 71;
// java public static final int basicBlackSquares = 72;
pub(crate) const basicBlackSquares: i32 = 72;
// java public static final int basicThinLines = 73;
pub(crate) const basicThinLines: i32 = 73;
// java public static final int basicWhiteDashes = 74;
pub(crate) const basicWhiteDashes: i32 = 74;
// java public static final int basicWhiteDots = 75;
pub(crate) const basicWhiteDots: i32 = 75;
// java public static final int basicWhiteSquares = 76;
pub(crate) const basicWhiteSquares: i32 = 76;
// java public static final int basicWideInline = 77;
pub(crate) const basicWideInline: i32 = 77;
// java public static final int basicWideMidline = 78;
pub(crate) const basicWideMidline: i32 = 78;
// java public static final int basicWideOutline = 79;
pub(crate) const basicWideOutline: i32 = 79;
// java public static final int bats = 80;
pub(crate) const bats: i32 = 80;
// java public static final int birds = 81;
pub(crate) const birds: i32 = 81;
// java public static final int birdsFlight = 82;
pub(crate) const birdsFlight: i32 = 82;
// java public static final int cabins = 83;
pub(crate) const cabins: i32 = 83;
// java public static final int cakeSlice = 84;
pub(crate) const cakeSlice: i32 = 84;
// java public static final int candyCorn = 85;
pub(crate) const candyCorn: i32 = 85;
// java public static final int celticKnotwork = 86;
pub(crate) const celticKnotwork: i32 = 86;
// java public static final int certificateBanner = 87;
pub(crate) const certificateBanner: i32 = 87;
// java public static final int chainLink = 88;
pub(crate) const chainLink: i32 = 88;
// java public static final int champagneBottle = 89;
pub(crate) const champagneBottle: i32 = 89;
// java public static final int checkedBarBlack = 90;
pub(crate) const checkedBarBlack: i32 = 90;
// java public static final int checkedBarColor = 91;
pub(crate) const checkedBarColor: i32 = 91;
// java public static final int checkered = 92;
pub(crate) const checkered: i32 = 92;
// java public static final int christmasTree = 93;
pub(crate) const christmasTree: i32 = 93;
// java public static final int circlesLines = 94;
pub(crate) const circlesLines: i32 = 94;
// java public static final int circlesRectangles = 95;
pub(crate) const circlesRectangles: i32 = 95;
// java public static final int classicalWave = 96;
pub(crate) const classicalWave: i32 = 96;
// java public static final int clocks = 97;
pub(crate) const clocks: i32 = 97;
// java public static final int compass = 98;
pub(crate) const compass: i32 = 98;
// java public static final int confetti = 99;
pub(crate) const confetti: i32 = 99;
// java public static final int confettiGrays = 100;
pub(crate) const confettiGrays: i32 = 100;
// java public static final int confettiOutline = 101;
pub(crate) const confettiOutline: i32 = 101;
// java public static final int confettiStreamers = 102;
pub(crate) const confettiStreamers: i32 = 102;
// java public static final int confettiWhite = 103;
pub(crate) const confettiWhite: i32 = 103;
// java public static final int cornerTriangles = 104;
pub(crate) const cornerTriangles: i32 = 104;
// java public static final int couponCutoutDashes = 105;
pub(crate) const couponCutoutDashes: i32 = 105;
// java public static final int couponCutoutDots = 106;
pub(crate) const couponCutoutDots: i32 = 106;
// java public static final int crazyMaze = 107;
pub(crate) const crazyMaze: i32 = 107;
// java public static final int creaturesButterfly = 108;
pub(crate) const creaturesButterfly: i32 = 108;
// java public static final int creaturesFish = 109;
pub(crate) const creaturesFish: i32 = 109;
// java public static final int creaturesInsects = 110;
pub(crate) const creaturesInsects: i32 = 110;
// java public static final int creaturesLadyBug = 111;
pub(crate) const creaturesLadyBug: i32 = 111;
// java public static final int crossStitch = 112;
pub(crate) const crossStitch: i32 = 112;
// java public static final int cup = 113;
pub(crate) const cup: i32 = 113;
// java public static final int decoArch = 114;
pub(crate) const decoArch: i32 = 114;
// java public static final int decoArchColor = 115;
pub(crate) const decoArchColor: i32 = 115;
// java public static final int decoBlocks = 116;
pub(crate) const decoBlocks: i32 = 116;
// java public static final int diamondsGray = 117;
pub(crate) const diamondsGray: i32 = 117;
// java public static final int doubleD = 118;
pub(crate) const doubleD: i32 = 118;
// java public static final int doubleDiamonds = 119;
pub(crate) const doubleDiamonds: i32 = 119;
// java public static final int earth1 = 120;
pub(crate) const earth1: i32 = 120;
// java public static final int earth2 = 121;
pub(crate) const earth2: i32 = 121;
// java public static final int eclipsingSquares1 = 122;
pub(crate) const eclipsingSquares1: i32 = 122;
// java public static final int eclipsingSquares2 = 123;
pub(crate) const eclipsingSquares2: i32 = 123;
// java public static final int eggsBlack = 124;
pub(crate) const eggsBlack: i32 = 124;
// java public static final int fans = 125;
pub(crate) const fans: i32 = 125;
// java public static final int film = 126;
pub(crate) const film: i32 = 126;
// java public static final int firecrackers = 127;
pub(crate) const firecrackers: i32 = 127;
// java public static final int flowersBlockPrint = 128;
pub(crate) const flowersBlockPrint: i32 = 128;
// java public static final int flowersDaisies = 129;
pub(crate) const flowersDaisies: i32 = 129;
// java public static final int flowersModern1 = 130;
pub(crate) const flowersModern1: i32 = 130;
// java public static final int flowersModern2 = 131;
pub(crate) const flowersModern2: i32 = 131;
// java public static final int flowersPansy = 132;
pub(crate) const flowersPansy: i32 = 132;
// java public static final int flowersRedRose = 133;
pub(crate) const flowersRedRose: i32 = 133;
// java public static final int flowersRoses = 134;
pub(crate) const flowersRoses: i32 = 134;
// java public static final int flowersTeacup = 135;
pub(crate) const flowersTeacup: i32 = 135;
// java public static final int flowersTiny = 136;
pub(crate) const flowersTiny: i32 = 136;
// java public static final int gems = 137;
pub(crate) const gems: i32 = 137;
// java public static final int gingerbreadMan = 138;
pub(crate) const gingerbreadMan: i32 = 138;
// java public static final int gradient = 139;
pub(crate) const gradient: i32 = 139;
// java public static final int handmade1 = 140;
pub(crate) const handmade1: i32 = 140;
// java public static final int handmade2 = 141;
pub(crate) const handmade2: i32 = 141;
// java public static final int heartBalloon = 142;
pub(crate) const heartBalloon: i32 = 142;
// java public static final int heartGray = 143;
pub(crate) const heartGray: i32 = 143;
// java public static final int hearts = 144;
pub(crate) const hearts: i32 = 144;
// java public static final int heebieJeebies = 145;
pub(crate) const heebieJeebies: i32 = 145;
// java public static final int holly = 146;
pub(crate) const holly: i32 = 146;
// java public static final int houseFunky = 147;
pub(crate) const houseFunky: i32 = 147;
// java public static final int hypnotic = 148;
pub(crate) const hypnotic: i32 = 148;
// java public static final int iceCreamCones = 149;
pub(crate) const iceCreamCones: i32 = 149;
// java public static final int lightBulb = 150;
pub(crate) const lightBulb: i32 = 150;
// java public static final int lightning1 = 151;
pub(crate) const lightning1: i32 = 151;
// java public static final int lightning2 = 152;
pub(crate) const lightning2: i32 = 152;
// java public static final int mapPins = 153;
pub(crate) const mapPins: i32 = 153;
// java public static final int mapleLeaf = 154;
pub(crate) const mapleLeaf: i32 = 154;
// java public static final int mapleMuffins = 155;
pub(crate) const mapleMuffins: i32 = 155;
// java public static final int marquee = 156;
pub(crate) const marquee: i32 = 156;
// java public static final int marqueeToothed = 157;
pub(crate) const marqueeToothed: i32 = 157;
// java public static final int moons = 158;
pub(crate) const moons: i32 = 158;
// java public static final int mosaic = 159;
pub(crate) const mosaic: i32 = 159;
// java public static final int musicNotes = 160;
pub(crate) const musicNotes: i32 = 160;
// java public static final int northwest = 161;
pub(crate) const northwest: i32 = 161;
// java public static final int ovals = 162;
pub(crate) const ovals: i32 = 162;
// java public static final int packages = 163;
pub(crate) const packages: i32 = 163;
// java public static final int palmsBlack = 164;
pub(crate) const palmsBlack: i32 = 164;
// java public static final int palmsColor = 165;
pub(crate) const palmsColor: i32 = 165;
// java public static final int paperClips = 166;
pub(crate) const paperClips: i32 = 166;
// java public static final int papyrus = 167;
pub(crate) const papyrus: i32 = 167;
// java public static final int partyFavor = 168;
pub(crate) const partyFavor: i32 = 168;
// java public static final int partyGlass = 169;
pub(crate) const partyGlass: i32 = 169;
// java public static final int pencils = 170;
pub(crate) const pencils: i32 = 170;
// java public static final int people = 171;
pub(crate) const people: i32 = 171;
// java public static final int peopleWaving = 172;
pub(crate) const peopleWaving: i32 = 172;
// java public static final int peopleHats = 173;
pub(crate) const peopleHats: i32 = 173;
// java public static final int poinsettias = 174;
pub(crate) const poinsettias: i32 = 174;
// java public static final int postageStamp = 175;
pub(crate) const postageStamp: i32 = 175;
// java public static final int pumpkin1 = 176;
pub(crate) const pumpkin1: i32 = 176;
// java public static final int pushPinNote2 = 177;
pub(crate) const pushPinNote2: i32 = 177;
// java public static final int pushPinNote1 = 178;
pub(crate) const pushPinNote1: i32 = 178;
// java public static final int pyramids = 179;
pub(crate) const pyramids: i32 = 179;
// java public static final int pyramidsAbove = 180;
pub(crate) const pyramidsAbove: i32 = 180;
// java public static final int quadrants = 181;
pub(crate) const quadrants: i32 = 181;
// java public static final int rings = 182;
pub(crate) const rings: i32 = 182;
// java public static final int safari = 183;
pub(crate) const safari: i32 = 183;
// java public static final int sawtooth = 184;
pub(crate) const sawtooth: i32 = 184;
// java public static final int sawtoothGray = 185;
pub(crate) const sawtoothGray: i32 = 185;
// java public static final int scaredCat = 186;
pub(crate) const scaredCat: i32 = 186;
// java public static final int seattle = 187;
pub(crate) const seattle: i32 = 187;
// java public static final int shadowedSquares = 188;
pub(crate) const shadowedSquares: i32 = 188;
// java public static final int sharksTeeth = 189;
pub(crate) const sharksTeeth: i32 = 189;
// java public static final int shorebirdTracks = 190;
pub(crate) const shorebirdTracks: i32 = 190;
// java public static final int skyrocket = 191;
pub(crate) const skyrocket: i32 = 191;
// java public static final int snowflakeFancy = 192;
pub(crate) const snowflakeFancy: i32 = 192;
// java public static final int snowflakes = 193;
pub(crate) const snowflakes: i32 = 193;
// java public static final int sombrero = 194;
pub(crate) const sombrero: i32 = 194;
// java public static final int southwest = 195;
pub(crate) const southwest: i32 = 195;
// java public static final int stars = 196;
pub(crate) const stars: i32 = 196;
// java public static final int starsTop = 197;
pub(crate) const starsTop: i32 = 197;
// java public static final int stars3d = 198;
pub(crate) const stars3d: i32 = 198;
// java public static final int starsBlack = 199;
pub(crate) const starsBlack: i32 = 199;
// java public static final int starsShadowed = 200;
pub(crate) const starsShadowed: i32 = 200;
// java public static final int sun = 201;
pub(crate) const sun: i32 = 201;
// java public static final int swirligig = 202;
pub(crate) const swirligig: i32 = 202;
// java public static final int tornPaper = 203;
pub(crate) const tornPaper: i32 = 203;
// java public static final int tornPaperBlack = 204;
pub(crate) const tornPaperBlack: i32 = 204;
// java public static final int trees = 205;
pub(crate) const trees: i32 = 205;
// java public static final int triangleParty = 206;
pub(crate) const triangleParty: i32 = 206;
// java public static final int triangles = 207;
pub(crate) const triangles: i32 = 207;
// java public static final int tribal1 = 208;
pub(crate) const tribal1: i32 = 208;
// java public static final int tribal2 = 209;
pub(crate) const tribal2: i32 = 209;
// java public static final int tribal3 = 210;
pub(crate) const tribal3: i32 = 210;
// java public static final int tribal4 = 211;
pub(crate) const tribal4: i32 = 211;
// java public static final int tribal5 = 212;
pub(crate) const tribal5: i32 = 212;
// java public static final int tribal6 = 213;
pub(crate) const tribal6: i32 = 213;
// java public static final int twistedLines1 = 214;
pub(crate) const twistedLines1: i32 = 214;
// java public static final int twistedLines2 = 215;
pub(crate) const twistedLines2: i32 = 215;
// java public static final int vine = 216;
pub(crate) const vine: i32 = 216;
// java public static final int waveline = 217;
pub(crate) const waveline: i32 = 217;
// java public static final int weavingAngles = 218;
pub(crate) const weavingAngles: i32 = 218;
// java public static final int weavingBraid = 219;
pub(crate) const weavingBraid: i32 = 219;
// java public static final int weavingRibbon = 220;
pub(crate) const weavingRibbon: i32 = 220;
// java public static final int weavingStrips = 221;
pub(crate) const weavingStrips: i32 = 221;
// java public static final int whiteFlowers = 222;
pub(crate) const whiteFlowers: i32 = 222;
// java public static final int woodwork = 223;
pub(crate) const woodwork: i32 = 223;
// java public static final int xIllusions = 224;
pub(crate) const xIllusions: i32 = 224;
// java public static final int zanyTriangles = 225;
pub(crate) const zanyTriangles: i32 = 225;
// java public static final int zigZag = 226;
pub(crate) const zigZag: i32 = 226;
// java public static final int zigZagStitch = 227;
pub(crate) const zigZagStitch: i32 = 227;
// java public static final int brcIgnore = 255;
pub(crate) const brcIgnore: i32 = 255;
// java public static final int brcMax = 256;
pub(crate) const brcMax: i32 = 256;

// java public class BRC implements Cloneable, IShareObjIndexdMap.IShareObjIndexed
#[derive(Default, Clone)]
pub struct BRC {
    // java float dptLineWidth;
    pub(crate) dptLineWidth: f32,
    // java int brcType;
    pub(crate) brcType: i32,
    // java int color;
    pub(crate) color: i32,
    // java float dptSpace;
    pub(crate) dptSpace: f32,
    // java boolean fShadow;
    pub(crate) fShadow: bool,
    // java boolean fFrame;
    pub(crate) fFrame: bool,
    // java private int mIndex;
    mIndex: i32,
    // java int hash;
    pub(crate) hash: i32,
}

// java public class BRC implements Cloneable, IShareObjIndexdMap.IShareObjIndexed
impl BRC {
    // java public void setIndex(int index)
    pub fn setIndex(&mut self, index: i32) {
      // java this.mIndex = index;
      self.mIndex = index;
    }

    // java public int getIndex()
    pub fn getIndex(&self) -> i32 {
      // java return this.mIndex;
      return self.mIndex;
    }

    // java public Object getHostObj()
    pub fn getHostObj(&self) -> Box<dyn Object> {
      /** DEFERRED_NON_FUNCTION
      // java return this;
      return self;
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public  BRC()
    pub fn new() -> Self {
      // java this(0.0F, 0, 0, 0.0F, false, false);
      Self::new__float_int_int_float_boolean_boolean(0.0, 0, 0, 0.0, false, false)
    }

    // java public  BRC(int color)
    pub fn new__int(color: i32) -> Self {
      // java this(0.0F, 0, color, 0.0F, false, false);
      Self::new__float_int_int_float_boolean_boolean(0.0, 0, color, 0.0, false, false)
    }

    // java public  BRC(float dptLineWidth, int brcType, int color, float dptSpace, boolean fShadow, boolean fFrame)
    pub fn new__float_int_int_float_boolean_boolean(dptLineWidth: f32, brcType: i32, color: i32, dptSpace: f32, fShadow: bool, fFrame: bool) -> Self {
      // java this.mIndex = 0;
      // java this.dptLineWidth = dptLineWidth;
      // java this.brcType = brcType;
      // java this.color = color;
      // java this.dptSpace = dptSpace;
      // java this.fShadow = fShadow;
      // java this.fFrame = fFrame;
      Self { dptLineWidth, brcType, color, dptSpace, fShadow, fFrame, mIndex: 0, hash: 0 }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      /** DEFERRED_NON_FUNCTION
      // java StringBuilder sb = new StringBuilder();
      let mut sb: String = StringBuilder::new();
      // java sb.append("dptLineWidth = " + this.dptLineWidth + ", ");
      sb.append("dptLineWidth = " + self.dptLineWidth + ", ");
      // java sb.append("brcType = " + this.brcType + ", ");
      sb.append("brcType = " + self.brcType + ", ");
      // java sb.append("color = " + this.color + ", ");
      sb.append("color = " + self.color + ", ");
      // java sb.append("dptSpace = " + this.dptSpace + ", ");
      sb.append("dptSpace = " + self.dptSpace + ", ");
      // java sb.append("fShadow = " + this.fShadow + ", ");
      sb.append("fShadow = " + self.fShadow + ", ");
      // java sb.append("fFrame = " + this.fFrame);
      sb.append("fFrame = " + self.fFrame);
      // java return sb.toString();
      return sb.toString();
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public  BRC(float lineWidth, int type)
    pub fn new__float_int(lineWidth: f32, r#type: i32) -> Self {
      // java this();
      let mut this = Self::new();
      // java this.dptLineWidth = lineWidth;
      this.dptLineWidth = lineWidth;
      // java this.brcType = type;
      this.brcType = r#type;
      this
    }

    // java public int getBorderType()
    pub fn getBorderType(&self) -> i32 {
      // java return this.brcType;
      return self.brcType;
    }

    // java public float getLineWidth()
    pub fn getLineWidth(&self) -> f32 {
      // java return this.dptLineWidth;
      return self.dptLineWidth;
    }

    // java public int getColor()
    pub fn getColor(&self) -> i32 {
      // java return this.color;
      return self.color;
    }

    // java public float getSpace()
    pub fn getSpace(&self) -> f32 {
      // java return this.dptSpace;
      return self.dptSpace;
    }

    // java public boolean isShadow()
    pub fn isShadow(&self) -> bool {
      // java return this.fShadow;
      return self.fShadow;
    }

    // java public boolean isFrame()
    pub fn isFrame(&self) -> bool {
      // java return this.fFrame;
      return self.fFrame;
    }

    // java public boolean equals(Object rhs)
    pub fn equals(&self, rhs: &dyn Object) -> bool {
      /** DEFERRED_NON_FUNCTION
      // java if (rhs == this)
      if rhs == self {
        // java return true;
        return true;
      }
      // java else if (!(rhs instanceof BRC))
      else if !rhs.downcast_ref::<BRC>().is_some() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java BRC r = (BRC)rhs;
        let mut r: BRC = (rhs).downcast_ref::<BRC>().unwrap();
        // java return (int)(this.dptLineWidth * 8.0F) == (int)(r.dptLineWidth * 8.0F) && this.brcType == r.brcType && this.color == r.color && (int)(this.dptSpace * 8.0F) == (int)(r.dptSpace * 8.0F) && this.fShadow == r.fShadow && this.fFrame == r.fFrame;
        return ((self.dptLineWidth * 8) as i32) == ((r.dptLineWidth * 8) as i32) && self.brcType == r.brcType && self.color == r.color && ((self.dptSpace * 8) as i32) == ((r.dptSpace * 8) as i32) && self.fShadow == r.fShadow && self.fFrame == r.fFrame;
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      /** DEFERRED_NON_FUNCTION
      // java if (this.hash == 0 || sTemp == this)
      if self.hash == 0 || sTemp == self {
        // java this.hash = (int)(this.dptLineWidth * 8.0F) + this.brcType + this.color + (int)(this.dptSpace * 8.0F) + (this.fShadow ? 1 : 0) + (this.fFrame ? 1 : 0);
        self.hash = ((self.dptLineWidth * 8) as i32) + self.brcType + self.color + ((self.dptSpace * 8) as i32) + (if self.fShadow { 1 } else { 0 }) + (if self.fFrame { 1 } else { 0 });
      }
      // java return this.hash;
      return self.hash;
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public boolean equalsIgnoreDptSpace(Object rhs)
    pub fn equalsIgnoreDptSpace(&self, rhs: Option<&dyn Object>) -> bool {
      /** DEFERRED_NON_FUNCTION
      // java if (rhs != null && rhs instanceof BRC)
      if let Some(rhs) = rhs {
        if rhs.downcast_ref::<BRC>().is_some() {
          // java BRC r = (BRC)rhs;
          let mut r: BRC = (rhs).downcast_ref::<BRC>().unwrap();
          // java return (int)(this.dptLineWidth * 8.0F) == (int)(r.dptLineWidth * 8.0F) && this.brcType == r.brcType && this.color == r.color && this.fShadow == r.fShadow && this.fFrame == r.fFrame;
          return ((self.dptLineWidth * 8) as i32) == ((r.dptLineWidth * 8) as i32) && self.brcType == r.brcType && self.color == r.color && self.fShadow == r.fShadow && self.fFrame == r.fFrame;
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public boolean isValidTableBrc()
    pub fn isValidTableBrc(&self) -> bool {
      // java return this.brcType != 0 && this.brcType != 255;
      return self.brcType != 0 && self.brcType != 255;
    }

    // java public static synchronized BRC obtain(float dptLineWidth, int brcType, int color, float dptSpace, boolean fShadow, boolean fFrame)
    pub fn obtain(dptLineWidth: f32, brcType: i32, color: i32, dptSpace: f32, fShadow: bool, fFrame: bool) -> BRC {
      /** DEFERRED_NON_FUNCTION
      // java sTemp.dptLineWidth = dptLineWidth;
      // java sTemp.brcType = brcType;
      // java sTemp.color = color;
      // java sTemp.dptSpace = dptSpace;
      // java sTemp.fShadow = fShadow;
      // java sTemp.fFrame = fFrame;
      // java BRC cached = (BRC)sPool.get(sTemp);
      // java if (cached == null)
        // java cached = new BRC(dptLineWidth, brcType, color, dptSpace, fShadow, fFrame);
        // java sPool.put(cached, cached);
      // java return cached;
      **/
      let cached = BRC { dptLineWidth, brcType, color, dptSpace, fShadow, fFrame, mIndex: 0, hash: 0 };
      cached
    }

    // java public static BRC obtain(float dptLineWidth, int brcType, int color)
    pub fn obtain__float_int_int(dptLineWidth: f32, brcType: i32, color: i32) -> BRC {
      /** DEFERRED_NON_FUNCTION
      // java return obtain(dptLineWidth, brcType, color, 0.0F, false, false);
      return Self::obtain__float_int_int_float_boolean_boolean(dptLineWidth, brcType, color, 0, false, false);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public static BRC obtain(int brcType)
    pub fn obtain__int(brcType: i32) -> BRC {
      /** DEFERRED_NON_FUNCTION
      // java return obtain(0.0F, brcType, 0, 0.0F, false, false);
      return Self::obtain__float_int_int_float_boolean_boolean(0, brcType, 0, 0, false, false);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public static BRC obtain(BRC old, float space)
    pub fn obtain__brc_float(old: &BRC, space: f32) -> BRC {
      /** DEFERRED_NON_FUNCTION
      // java return obtain(old.getLineWidth(), old.getBorderType(), old.getColor(), space, old.isShadow(), old.isFrame());
      return Self::obtain__float_int_int_float_boolean_boolean(old.getLineWidth(), old.getBorderType(), old.getColor(), space, old.isShadow(), old.isFrame());
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public static BRC obtain(BRC old, int color)
    pub fn obtain__brc_int(old: &BRC, color: i32) -> BRC {
      /** DEFERRED_NON_FUNCTION
      // java return obtain(old.getLineWidth(), old.getBorderType(), color, (float)old.getColor(), old.isShadow(), old.isFrame());
      return Self::obtain__float_int_int_float_boolean_boolean(old.getLineWidth(), old.getBorderType(), color, ((old.getColor()) as f32), old.isShadow(), old.isFrame());
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public static BRC obtain(BRC old, float lineWidth, int type)
    pub fn obtain__brc_float_int(old: &BRC, lineWidth: f32, r#type: i32) -> BRC {
      /** DEFERRED_NON_FUNCTION
      // java return obtain(lineWidth, type, old.getColor(), old.getSpace(), old.isShadow(), old.isFrame());
      return Self::obtain__float_int_int_float_boolean_boolean(lineWidth, r#type, old.getColor(), old.getSpace(), old.isShadow(), old.isFrame());
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public static synchronized void clear()
    pub fn clear() {
      /** DEFERRED_NON_FUNCTION
      // java sPool.clear();
      sPool.clear();
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

}
