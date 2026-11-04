// Java: import cn.wps.base.assertion.IOAssert
// Java: import org.apache.poi.util.LittleEndian

use super::Cipher::{Cipher, CipherBase};
use super::CipherDirection;
use super::CipherError;
use super::CipherMode;

const CIPHER_ROUNDS_BF: i32 = 16;
const CIPHER_ROUNDS_BF_KEY_SSIZE: i32 = 256;

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32)
        | ((data[offset + 1] as i32) << 8)
        | ((data[offset + 2] as i32) << 16)
        | ((data[offset + 3] as i32) << 24)
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

// Java: public class CipherBF extends Cipher
pub struct CipherBF {
    // Java field: int CIPHER_ROUNDS_BF
    // Java field: int CIPHER_ROUNDS_BF_KEY_SSIZE
    base: CipherBase,
    // Java field: int[] mS0
    mS0: Vec<i32>,
    // Java field: int[] mS1
    mS1: Vec<i32>,
    // Java field: int[] mS2
    mS2: Vec<i32>,
    // Java field: int[] mS3
    mS3: Vec<i32>,
    // Java field: int[] mP
    mP: Vec<i32>,
    // Java field: byte[] mByte
    mByte: Vec<u8>,
    // Java field: int mOffset
    mOffset: i32,
    // Java field: int[] cipherBF_keyS0
    // Java field: int[] cipherBF_keyS1
    // Java field: int[] cipherBF_keyS2
    // Java field: int[] cipherBF_keyS3
    // Java field: int[] cipherBF_keyP
}

#[allow(overflowing_literals)]
static CIPHER_BF_KEY_S0: &[i32] = &[-785314906, -1730169428, 805139163, -803545161, -1193168915, 1780907670, -1166241723, -248741991, 614570311, -1282315017, 134345442, -2054226922, 1667834072, 1901547113, -1537671517, -191677058, 227898511, 1921955416, 1904987480, -2112533778, 2069144605, -1034266187, -1674521287, 720527379, -976113629, 677414384, -901678824, -1193592593, -1904616272, 1614419982, 1822297739, -1340175810, -686458943, -1120842969, 2024746970, 1432378464, -430627341, -1437226092, 1464375394, 1676153920, 1439316330, 715854006, -1261675468, 289532110, -1588296017, 2087905683, -1276242927, 1668267050, 732546397, 1947742710, -832815594, -1685613794, -1344882125, 1814351708, 2050118529, 680887927, 999245976, 1800124847, -994056165, 1713906067, 1641548236, -81679983, 1216130144, 1575780402, -276538019, -377129551, -601480446, -345695352, 596196993, -745100091, 258830323, -2081144263, 772490370, -1534844924, 1774776394, -1642095778, 566650946, -152474470, 1728879713, -1412200208, 1783734482, -665571480, -1777359064, -1420741725, 1861159788, 326777828, -1170476976, 2130389656, -1578015459, 967770486, 1724537150, -2109534584, -1930525159, 1164943284, 2105845187, 998989502, -529566248, -2050940813, 1075463327, 1455516326, 1322494562, 910128902, 469688178, 1117454909, 936433444, -804646328, -619713837, 1240580251, 122909385, -2137449605, 634681816, -152510729, -469872614, -1233564613, -1754472259, 79693498, -1045868618, 1084186820, 1583128258, 426386531, 1761308591, 1047286709, 322548459, 995290223, 1845252383, -1691314900, -863943356, -1352745719, -1092366332, -567063811, 1712269319, 422464435, -1060394921, 1170764815, -771006663, -1177289765, 1434042557, 442511882, -694091578, 1076654713, 1738483198, -81812532, -1901729288, -617471240, 1014306527, -43947243, 793779912, -1392160085, 842905082, -48003232, 1395751752, 1040244610, -1638115397, -898659168, 445077038, -552113701, -717051658, 679411651, -1402522938, -1940957837, 1767581616, -1144366904, -503340195, -1192226400, 284835224, -48135240, 1258075500, 768725851, -1705778055, -1225243291, -762426948, 1274779536, -505548070, -1530167757, 1660621633, -823867672, -283063590, 913787905, -797008130, 737222580, -1780753843, -1366257256, -357724559, 1804850592, -795946544, -1345903136, -1908647121, -1904896841, -1879645445, -233690268, -2004305902, -1878134756, 1336762016, 1754252060, -774901359, -1280786003, 791618072, -1106372745, -361419266, -1962795103, -442446833, -1250986776, 413987798, -829824359, -1264037920, -49028937, 2093235073, -760370983, 375366246, -2137688315, -1815317740, 555357303, -424861595, 2008414854, -950779147, -73583153, -338841844, 2067696032, -700376109, -1373733303, 2428461, 544322398, 577241275, 1471733935, 610547355, -267798242, 1432588573, 1507829418, 2025931657, -648391809, 545086370, 48609733, -2094660746, 1653985193, 298326376, 1316178497, -1287180854, 2064951626, 458293330, -1705826027, -703637697, -1130641692, 727753846, -2115603456, 146436021, 1461446943, -224990101, 705550613, -1235000031, -407242314, -13368018, -981117340, 1404054877, -1449160799, 146425753, 1854211946];
static CIPHER_BF_KEY_S1: &[i32] = &[1266315497, -1246549692, -613086930, -1004984797, -1385257296, 1235738493, -1662099272, -1880247706, -324367247, 1771706367, 1449415276, -1028546847, 422970021, 1963543593, -1604775104, -468174274, 1062508698, 1531092325, 1804592342, -1711849514, -1580033017, -269995787, 1294809318, -265986623, 1289560198, -2072974554, 1669523910, 35572830, 157838143, 1052438473, 1016535060, 1802137761, 1753167236, 1386275462, -1214491899, -1437595849, 1040679964, 2145300060, -1904392980, 1461121720, -1338320329, -263189491, -266592508, 33600511, -1374882534, 1018524850, 629373528, -603381315, -779021319, 2091462646, -1808644237, 586499841, 988145025, 935516892, -927631820, -1695294041, -1455136442, 265290510, -322386114, -1535828415, -499593831, 1005194799, 847297441, 406762289, 1314163512, 1332590856, 1866599683, -167115585, 750260880, 613907577, 1450815602, -1129346641, -560302305, -644675568, -1282691566, -590397650, 1427272223, 778793252, 1343938022, -1618686585, 2052605720, 1946737175, -1130390852, -380928628, -327488454, -612033030, 1661551462, -1000029230, -283371449, 840292616, -582796489, 616741398, 312560963, 711312465, 1351876610, 322626781, 1910503582, 271666773, -2119403562, 1594956187, 70604529, -677132437, 1007753275, 1495573769, -225450259, -1745748998, -1631928532, 504708206, -2031925904, -353800271, -2045878774, 1514023603, 1998579484, 1312622330, 694541497, -1712906993, -2143385130, 1382467621, 776784248, -1676627094, -971698502, -1797068168, -1510196141, 503983604, -218673497, 907881277, 423175695, 432175456, 1378068232, -149744970, -340918674, -356311194, -474200683, -1501837181, -1317062703, 26017576, -1020076561, -1100195163, 1700274565, 1756076034, -288447217, -617638597, 720338349, 1533947780, 354530856, 688349552, -321042571, 1637815568, 332179504, -345916010, 53804574, -1442618417, -1250730864, 1282449977, -711025141, -877994476, -288586052, 1617046695, -1666491221, -1292663698, 1686838959, 431878346, -1608291911, 1700445008, 1080580658, 1009431731, 832498133, -1071531785, -1688990951, -2023776103, -1778935426, 1648197032, -130578278, -1746719369, 300782431, 375919233, 238389289, -941219882, -1763778655, 2019080857, 1475708069, 455242339, -1685863425, 448939670, -843904277, 1395535956, -1881585436, 1841049896, 1491858159, 885456874, -30872223, -293847949, 1565136089, -396052509, 1108368660, 540939232, 1173283510, -1549095958, -613658859, -87339056, -951913406, -278217803, 1699691293, 1103962373, -669091426, -2038084153, -464828566, 1031889488, -815619598, 1535977030, -58162272, -1043876189, 2132092099, 1774941330, 1199868427, 1452454533, 157007616, -1390851939, 342012276, 595725824, 1480756522, 206960106, 497939518, 591360097, 863170706, -1919713727, -698356495, 1814182875, 2094937945, -873565088, 1082520231, -831049106, -1509457788, 435703966, -386934699, 1641649973, -1452693590, -989067582, 1510255612, -2146710820, -1639679442, -1018874748, -36346107, 236887753, -613164077, 274041037, 1734335097, -479771840, -976997275, 1899903192, 1026095262, -244449504, 356393447, -1884275382, -421290197, -612127241];
static CIPHER_BF_KEY_S2: &[i32] = &[-381855128, -1803468553, -162781668, -1805047500, 1091903735, 1979897079, -1124832466, -727580568, -737663887, 857797738, 1136121015, 1342202287, 507115054, -1759230650, 337727348, -1081374656, 1301675037, -1766485585, 1895095763, 1721773893, -1078195732, 62756741, 2142006736, 835421444, -1762973773, 1442658625, -635090970, -1412822374, 676362277, 1392781812, 170690266, -373920261, 1759253602, -683120384, 1745797284, 664899054, 1329594018, -393761396, -1249058810, 2062866102, -1429332356, -751345684, -830954599, 1080764994, 553557557, -638351943, -298199125, 991055499, 499776247, 1265440854, 648242737, -354183246, 980351604, -581221582, 1749149687, -898096901, -83167922, -654396521, 1161844396, -1169648345, 1431517754, 545492359, -26498633, -795437749, 1437099964, -1592419752, -861329053, -1713251533, -1507177898, 1060185593, 1593081372, -1876348548, -34019326, 69676912, -2135222948, 86519011, -1782508216, -456757982, 1220612927, -955283748, 133810670, 1090789135, 1078426020, 1569222167, 845107691, -711212847, -222510705, 1091646820, 628848692, 1613405280, -537335645, 526609435, 236106946, 48312990, -1352249391, -892239595, 1797494240, 859738849, 992217954, -289490654, -2051890674, -424014439, -562951028, 765654824, -804095931, -1783130883, 1685915746, -405998096, 1414112111, -2021832454, -1013056217, -214004450, 172450625, -1724973196, 980381355, -185008841, -1475158944, -1578377736, -1726226100, -613520627, -964995824, 1835478071, 660984891, -590288892, -248967737, -872349789, -1254551662, 1762651403, 1719377915, -824476260, -1601057013, -652910941, -1156370552, 1364962596, 2073328063, 1983633131, 926494387, -871278215, -2144935273, -198299347, 1749200295, -966120645, 309677260, 2016342300, 1779581495, -1215147545, 111262694, 1274766160, 443224088, 298511866, 1025883608, -488520759, 1145181785, 168956806, -653464466, -710153686, 1689216846, -628709281, -1094719096, 1692713982, -1648590761, -252198778, 1618508792, 1610833997, -771914938, -164094032, 2001055236, -684262196, -2092799181, -266425487, -1333771897, 1006657119, 2006996926, -1108824540, 1430667929, -1084739999, 1314452623, -220332638, -193663176, -2021016126, 1399257539, -927756684, -1267338667, 1190975929, 2062231137, -1960976508, -2073424263, -1856006686, 1181637006, 548689776, -1932175983, -922558900, -1190417183, -1149106736, 296247880, 1970579870, -1216407114, -525738999, 1714227617, -1003338189, -396747006, 166772364, 1251581989, 493813264, 448347421, 195405023, -1584991729, 677966185, -591930749, 1463355134, -1578971493, 1338867538, 1343315457, -1492745222, -1610435132, 233230375, -1694987225, 2000651841, -1017099258, 1638401717, -266896856, -1057650976, 6314154, 819756386, 300326615, 590932579, 1405279636, -1027467724, -1144263082, -1866680610, -335774303, -833020554, 1862657033, 1266418056, 963775037, 2089974820, -2031914401, 1917689273, 448879540, -744572676, -313240200, 150775221, -667058989, 1303187396, 508620638, -1318983944, -1568336679, 1817252668, 1876281319, 1457606340, 908771278, -574175177, -677760460, -1838972398, 1729034894, 1080033504];
static CIPHER_BF_KEY_S3: &[i32] = &[976866871, -738527793, -1413318857, 1522871579, 1555064734, 1336096578, -746444992, -1715692610, -720269667, -1089506539, -701686658, -956251013, -1215554709, 564236357, -1301368386, 1781952180, 1464380207, -1131123079, -962365742, 1699332808, 1393555694, 1183702653, -713881059, 1288719814, 691649499, -1447410096, -1399511320, -1101077756, -1577396752, 1781354906, 1676643554, -1702433246, -1064713544, 1126444790, -1524759638, -1661808476, -2084544070, -1679201715, -1880812208, -1167828010, 673620729, -1489356063, 1269405062, -279616791, -953159725, -145557542, 1057255273, 2012875353, -2132498155, -2018474495, -1693849939, 993977747, -376373926, -1640704105, 753973209, 36408145, -1764381638, 25011837, -774947114, 2088578344, 530523599, -1376601957, 1524020338, 1518925132, -534139791, -535190042, 1202760957, -309069157, -388774771, 674977740, -120232407, 2031300136, 2019492241, -311074731, -141160892, -472686964, 352677332, -1997247046, 60907813, 90501309, -1007968747, 1016092578, -1759044884, -1455814870, 457141659, 509813237, -174299397, 652014361, 1966332200, -1319764491, 55981186, -1967506245, 676427537, -1039476232, -1412673177, -861040033, 1307055953, 942726286, 933058658, -1826555503, -361066302, -79791154, 1361170020, 2001714738, -1464409218, -1020707514, 1222529897, 1679025792, -1565652976, -580013532, 1770335741, 151462246, -1281735158, 1682292957, 1483529935, 471910574, 1539241949, 458788160, -858652289, 1807016891, -576558466, 978976581, 1043663428, -1129001515, 1927990952, -94075717, -1922690386, -1086558393, -761535389, 1412390302, -1362987237, -162634896, 1947078029, -413461673, -126740879, -1353482915, 1077988104, 1320477388, 886195818, 18198404, -508558296, -1785185763, 112762804, -831610808, 1866414978, 891333506, 18488651, 661792760, 1628790961, -409780260, -1153795797, 876946877, -1601685023, 1372485963, 791857591, -1608533303, -534984578, -1127755274, -822013501, -1578587449, 445679433, -732971622, -790962485, -720709064, 54117162, -963561881, -1913048708, -525259953, -140617289, 1140177722, -220915201, 668550556, -1080614356, 367459370, 261225585, -1684794075, -85617823, -826893077, -1029151655, 314222801, -1228863650, -486184436, 282218597, -888953790, -521376242, 379116347, 1285071038, 846784868, -1625320142, -523005217, -744475605, -1989021154, 453669953, 1268987020, -977374944, -1015663912, -550133875, -1684459730, -435458233, 266596637, -447948204, 517658769, -832407089, -851542417, 370717030, -47440635, -2070949179, -151313767, -182193321, -1506642397, -1817692879, 1456262402, -1393524382, 1517677493, 1846949527, -1999473716, -560569710, -2118563376, 1280348187, 1908823572, -423180355, 846861322, 1172426758, -1007518822, -911584259, 1655181056, -1155153950, 901632758, 1897031941, -1308360158, -1228157060, -847864789, 1393639104, 373351379, 950779232, 625454576, -1170726756, -146354570, 2007998917, 544563296, -2050228658, -1964470824, 2058025392, 1291430526, 424198748, 50039436, 29584100, -689184263, -1865090967, -1503863136, 1057563949, -1039604065, -1219600078, -831004069, 1469046755, 985887462];
static CIPHER_BF_KEY_P: &[i32] = &[608135816, -2052912941, 320440878, 57701188, -1542899678, 698298832, 137296536, -330404727, 1160258022, 953160567, -1101764913, 887688300, -1062458953, -914599715, 1065670069, -1253635817, -1843997223, -1988494565];

const CIPHER_MODE_ECB: i32 = 0;
const CIPHER_MODE_CBC: i32 = 1;
const CIPHER_DIRECTION_ENCODE: i32 = 2;

// Java: public class CipherBF extends Cipher
impl CipherBF {
    // Java: CipherBF(int mode)
    pub fn new(mode: i32) -> Self {
        // Java: super();
        // Java: this.mS0 = new int[CIPHER_ROUNDS_BF_KEY_SSIZE];
        // Java: this.mS1 = new int[CIPHER_ROUNDS_BF_KEY_SSIZE];
        // Java: this.mS2 = new int[CIPHER_ROUNDS_BF_KEY_SSIZE];
        // Java: this.mS3 = new int[CIPHER_ROUNDS_BF_KEY_SSIZE];
        // Java: this.mP = new int[CIPHER_ROUNDS_BF + 2];
        // Java: this.mByte = new byte[8];
        // Java: IOAssert.logTrue("mode should not equal to CipherMode.Cipher_ModeInvalid!", mode != CipherMode.Cipher_ModeInvalid);
        assert!(mode != CipherMode::CIPHER_MODE_INVALID, "mode should not equal to CipherMode.Cipher_ModeInvalid!");
        // Java: this.mMode = mode;
        // Java: this.mDirection = CipherDirection.Cipher_DirectionInvalid;
        // Java: this.mAlgorithm = 0;
        // Java: this.mOffset = 0;
        CipherBF {
            base: CipherBase {
                mAlgorithm: 0,
                mDirection: CipherDirection::CIPHER_DIRECTION_INVALID,
                mMode: mode,
            },
            mS0: vec![0i32; CIPHER_ROUNDS_BF_KEY_SSIZE as usize],
            mS1: vec![0i32; CIPHER_ROUNDS_BF_KEY_SSIZE as usize],
            mS2: vec![0i32; CIPHER_ROUNDS_BF_KEY_SSIZE as usize],
            mS3: vec![0i32; CIPHER_ROUNDS_BF_KEY_SSIZE as usize],
            mP: vec![0i32; (CIPHER_ROUNDS_BF + 2) as usize],
            mByte: vec![0u8; 8],
            mOffset: 0,
        }
    }

    // Java: int decode(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn decode_impl(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> i32 {
        // Java: if (this.mAlgorithm != 1)
        if self.base.mAlgorithm != 1 {
            // Java: return CipherError.Cipher_E_Algorithm;
            return CipherError::CIPHER_E_ALGORITHM;
        }
        // Java: else
        else {
            // Java: return this.mDirection == CipherDirection.Cipher_DirectionInvalid ? CipherError.Cipher_E_Direction : this.update(data, offset, nLen, buffer);
            if self.base.mDirection == CipherDirection::CIPHER_DIRECTION_INVALID {
                CipherError::CIPHER_E_DIRECTION
            } else {
                self.update_impl(data, offset, nLen, buffer)
            }
        }
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
        // Java: this.mMode = CipherMode.Cipher_ModeInvalid;
        self.base.mMode = CipherMode::CIPHER_MODE_INVALID;
        // Java: this.mDirection = CipherDirection.Cipher_DirectionInvalid;
        self.base.mDirection = CipherDirection::CIPHER_DIRECTION_INVALID;
        // Java: this.mAlgorithm = 0;
        self.base.mAlgorithm = 0;
        // Java: this.mOffset = 0;
        self.mOffset = 0;
    }

    // Java: int encode(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn encode_impl(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> i32 {
        // Java: if (this.mAlgorithm != 1)
        if self.base.mAlgorithm != 1 {
            // Java: return CipherError.Cipher_E_Algorithm;
            return CipherError::CIPHER_E_ALGORITHM;
        }
        // Java: else
        else {
            // Java: return this.mDirection == CipherDirection.Cipher_DirectionInvalid ? CipherError.Cipher_E_Direction : this.update(data, offset, nLen, buffer);
            if self.base.mDirection == CipherDirection::CIPHER_DIRECTION_INVALID {
                CipherError::CIPHER_E_DIRECTION
            } else {
                self.update_impl(data, offset, nLen, buffer)
            }
        }
    }

    // Java: int init(int direction, byte[] keyData)
    pub fn init_impl(&mut self, direction: i32, keyData: &[u8]) -> i32 {
        // Java: if (keyData == null)
        // Java: return CipherError.Cipher_E_Argument;
        // Java: else if (this.mAlgorithm != 0)
        if self.base.mAlgorithm != 0 {
            // Java: return CipherError.Cipher_E_Algorithm;
            return CipherError::CIPHER_E_ALGORITHM;
        }
        // Java: else if (this.mDirection == CipherDirection.Cipher_DirectionInvalid)
        else if self.base.mDirection == CipherDirection::CIPHER_DIRECTION_INVALID {
            // Java: return CipherError.Cipher_E_Direction;
            return CipherError::CIPHER_E_DIRECTION;
        }
        // Java: else
        else {
            // Java: this.mDirection = direction;
            self.base.mDirection = direction;
            // Java: System.arraycopy(cipherBF_keyS0, 0, this.mS0, 0, cipherBF_keyS0.length);
            self.mS0.copy_from_slice(CIPHER_BF_KEY_S0);
            // Java: System.arraycopy(cipherBF_keyS1, 0, this.mS1, 0, cipherBF_keyS1.length);
            self.mS1.copy_from_slice(CIPHER_BF_KEY_S1);
            // Java: System.arraycopy(cipherBF_keyS2, 0, this.mS2, 0, cipherBF_keyS2.length);
            self.mS2.copy_from_slice(CIPHER_BF_KEY_S2);
            // Java: System.arraycopy(cipherBF_keyS3, 0, this.mS3, 0, cipherBF_keyS3.length);
            self.mS3.copy_from_slice(CIPHER_BF_KEY_S3);
            // Java: System.arraycopy(cipherBF_keyP, 0, this.mP, 0, cipherBF_keyS3.length);
            self.mP[..CIPHER_BF_KEY_P.len()].copy_from_slice(CIPHER_BF_KEY_P);
            // Java: Arrays.fill(this.mByte, (byte)0);
            self.mByte.fill(0);
            // Java: this.mOffset = 0;
            self.mOffset = 0;
            // Java: short i = 0;
            let mut i: i32 = 0;
            // Java: for(short k = 0; i < CIPHER_ROUNDS_BF + 2; ++i)
            let mut k: usize = 0;
            while i < CIPHER_ROUNDS_BF + 2 {
                // Java: int D = 0;
                let mut d_val: i32 = 0;
                // Java: for(short j = 0; j < 4; ++j)
                for _j in 0..4 {
                    // Java: D = D << 8 | keyData[k] | 255;
                    d_val = d_val << 8 | (keyData[k] as i32) | 255;
                    // Java: ++k;
                    k += 1;
                    // Java: if (k >= keyData.length)
                    if k >= keyData.len() {
                        // Java: k = 0;
                        k = 0;
                    }
                }
                // Java: int[] var10000 = this.mP;
                // Java: var10000[i] ^= D;
                self.mP[i as usize] ^= d_val;
                i += 1;
            }

            // Java: Integer DL = new Integer(0);
            let mut dl: i32 = 0;
            // Java: Integer DR = new Integer(0);
            let mut dr: i32 = 0;

            // Java: for(short var9 = 0; var9 < CIPHER_ROUNDS_BF + 2; var9 = (short)(var9 + 2))
            let mut var9: i32 = 0;
            while var9 < CIPHER_ROUNDS_BF + 2 {
                // Java: this.decode(DL, DR);
                self.decode_pair(&mut dl, &mut dr);
                // Java: this.mP[var9] = DL;
                self.mP[var9 as usize] = dl;
                // Java: this.mP[var9 + 1] = DR;
                self.mP[(var9 + 1) as usize] = dr;
                var9 += 2;
            }

            // Java: for(short var10 = 0; var10 < 256; var10 = (short)(var10 + 2))
            let mut var10: i32 = 0;
            while var10 < 256 {
                // Java: this.decode(DL, DR);
                self.decode_pair(&mut dl, &mut dr);
                // Java: this.mS0[var10] = DL;
                self.mS0[var10 as usize] = dl;
                // Java: this.mS0[var10 + 1] = DR;
                self.mS0[(var10 + 1) as usize] = dr;
                var10 += 2;
            }

            // Java: for(short var11 = 0; var11 < 256; var11 = (short)(var11 + 2))
            let mut var11: i32 = 0;
            while var11 < 256 {
                // Java: this.decode(DL, DR);
                self.decode_pair(&mut dl, &mut dr);
                // Java: this.mS1[var11] = DL;
                self.mS1[var11 as usize] = dl;
                // Java: this.mS1[var11 + 1] = DR;
                self.mS1[(var11 + 1) as usize] = dr;
                var11 += 2;
            }

            // Java: for(short var12 = 0; var12 < 256; var12 = (short)(var12 + 2))
            let mut var12: i32 = 0;
            while var12 < 256 {
                // Java: this.decode(DL, DR);
                self.decode_pair(&mut dl, &mut dr);
                // Java: this.mS2[var12] = DL;
                self.mS2[var12 as usize] = dl;
                // Java: this.mS2[var12 + 1] = DR;
                self.mS2[(var12 + 1) as usize] = dr;
                var12 += 2;
            }

            // Java: for(short var13 = 0; var13 < 256; var13 = (short)(var13 + 2))
            let mut var13: i32 = 0;
            while var13 < 256 {
                // Java: this.decode(DL, DR);
                self.decode_pair(&mut dl, &mut dr);
                // Java: this.mS3[var13] = DL;
                self.mS3[var13 as usize] = dl;
                // Java: this.mS3[var13 + 1] = DR;
                self.mS3[(var13 + 1) as usize] = dr;
                var13 += 2;
            }

            // Java: return CipherError.Cipher_E_None;
            CipherError::CIPHER_E_NONE
        }
    }

    // Java: int update(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn update_impl(&mut self, data: &[u8], offset: i32, mut nLen: i32, buffer: &mut [u8]) -> i32 {
        // Java: if (data != null && offset < data.length && nLen >= 0 && offset + nLen <= data.length && buffer != null)
        if (offset as usize) < data.len() && nLen >= 0 && (offset + nLen) as usize <= data.len() {
            // Java: if (nLen > buffer.length)
            if nLen > buffer.len() as i32 {
                // Java: return CipherError.Cipher_E_BufferSize;
                return CipherError::CIPHER_E_BUFFER_SIZE;
            }
            // Java: else
            else {
                // Java: int dataOffset = offset;
                let mut dataOffset = offset as usize;
                // Java: int bufferOffset = 0;
                let mut bufferOffset: usize = 0;
                // Java: if (this.mMode != CipherMode.Cipher_ModeECB)
                if self.base.mMode != CIPHER_MODE_ECB {
                    // Java: if (this.mMode != CipherMode.Cipher_ModeCBC)
                    if self.base.mMode != CIPHER_MODE_CBC {
                        // Java: while(nLen > 0)
                        while nLen > 0 {
                            // Java: this.updateCFB(data, dataOffset, nLen, buffer, bufferOffset, 1);
                            self.updateCFB(data, dataOffset, nLen, buffer, bufferOffset, 1);
                            // Java: --nLen;
                            nLen -= 1;
                            // Java: ++dataOffset;
                            dataOffset += 1;
                            // Java: ++bufferOffset;
                            bufferOffset += 1;
                        }
                    }
                    // Java: else
                    else {
                        // Java: while(nLen > 8)
                        while nLen > 8 {
                            // Java: this.updateCBC(data, dataOffset, nLen, buffer, bufferOffset, 8);
                            self.updateCBC(data, dataOffset, nLen, buffer, bufferOffset, 8);
                            // Java: nLen -= 8;
                            nLen -= 8;
                            // Java: dataOffset += 8;
                            dataOffset += 8;
                            // Java: bufferOffset += 8;
                            bufferOffset += 8;
                        }
                        // Java: this.updateCBC(data, dataOffset, nLen, buffer, bufferOffset, nLen);
                        self.updateCBC(data, dataOffset, nLen, buffer, bufferOffset, nLen);
                    }
                }
                // Java: else
                else {
                    // Java: while(nLen > 8)
                    while nLen > 8 {
                        // Java: this.updateECB(data, dataOffset, nLen, buffer, bufferOffset, 8);
                        self.updateECB(data, dataOffset, nLen, buffer, bufferOffset, 8);
                        // Java: nLen -= 8;
                        nLen -= 8;
                        // Java: dataOffset += 8;
                        dataOffset += 8;
                        // Java: bufferOffset += 8;
                        bufferOffset += 8;
                    }
                    // Java: this.updateECB(data, dataOffset, nLen, buffer, bufferOffset, nLen);
                    self.updateECB(data, dataOffset, nLen, buffer, bufferOffset, nLen);
                }
                // Java: return CipherError.Cipher_E_None;
                CipherError::CIPHER_E_NONE
            }
        }
        // Java: else
        else {
            // Java: return CipherError.Cipher_E_Argument;
            CipherError::CIPHER_E_ARGUMENT
        }
    }

    // Java: void encode(Integer DL, Integer DR)
    fn encode_pair(&self, dl: &mut i32, dr: &mut i32) {
        // Java: int XL = DL;
        let mut xl = *dl;
        // Java: int XR = DR;
        let mut xr = *dr;
        // Java: for(short i = 0; i < CIPHER_ROUNDS_BF; ++i)
        for i in 0..CIPHER_ROUNDS_BF as usize {
            // Java: XL ^= this.mP[i];
            xl ^= self.mP[i];
            // Java: XR ^= this.cipherBF(XL);
            xr ^= self.cipherBF(xl);
            // Java: XL = XR;
            // Java: XR = XL;
            std::mem::swap(&mut xl, &mut xr);
        }
        // Java: int var14 = XL ^ this.mP[CIPHER_ROUNDS_BF];
        let var14 = xl ^ self.mP[CIPHER_ROUNDS_BF as usize];
        // Java: XL = XR ^ this.mP[CIPHER_ROUNDS_BF + 1];
        xl = xr ^ self.mP[(CIPHER_ROUNDS_BF + 1) as usize];
        // Java: DL = XL;
        *dl = xl;
        // Java: DR = var14;
        *dr = var14;
    }

    // Java: void decode(Integer DL, Integer DR)
    fn decode_pair(&self, dl: &mut i32, dr: &mut i32) {
        // Java: int XL = DL;
        let mut xl = *dl;
        // Java: int XR = DR;
        let mut xr = *dr;
        // Java: for(int i = CIPHER_ROUNDS_BF + 1; i > 1; --i)
        let mut i = CIPHER_ROUNDS_BF + 1;
        while i > 1 {
            // Java: XL ^= this.mP[i];
            xl ^= self.mP[i as usize];
            // Java: XR ^= this.cipherBF(XL);
            xr ^= self.cipherBF(xl);
            // Java: XL = XR;
            // Java: XR = XL;
            std::mem::swap(&mut xl, &mut xr);
            i -= 1;
        }
        // Java: int var14 = XL ^ this.mP[1];
        let var14 = xl ^ self.mP[1];
        // Java: XL = XR ^ this.mP[0];
        xl = xr ^ self.mP[0];
        // Java: DL = XL;
        *dl = xl;
        // Java: DR = var14;
        *dr = var14;
    }

    // Java: int cipherBF(int x)
    fn cipherBF(&self, mut x: i32) -> i32 {
        // Java: short d = (short)(x & 255);
        let d = (x & 255) as usize;
        // Java: x >>= 8;
        x >>= 8;
        // Java: short c = (short)(x & 255);
        let c = (x & 255) as usize;
        // Java: x >>= 8;
        x >>= 8;
        // Java: short b = (short)(x & 255);
        let b = (x & 255) as usize;
        // Java: x >>= 8;
        x >>= 8;
        // Java: short a = (short)(x & 255);
        let a = (x & 255) as usize;
        // Java: int y = this.mS0[a];
        let mut y = self.mS0[a];
        // Java: y += this.mS1[b];
        y = y.wrapping_add(self.mS1[b]);
        // Java: y ^= this.mS2[c];
        y ^= self.mS2[c];
        // Java: y += this.mS3[d];
        y = y.wrapping_add(self.mS3[d]);
        // Java: return y;
        y
    }

    // Java: void updateCFB(byte[] data, int offset, int nLen, byte[] buffer, int bufferOffset, int blockSize)
    fn updateCFB(&mut self, data: &[u8], offset: usize, _nLen: i32, buffer: &mut [u8], bufferOffset: usize, _blockSize: i32) {
        // Java: int k = this.mOffset;
        let k = self.mOffset as usize;
        // Java: if (k == 0)
        if k == 0 {
            // Java: Integer DL = new Integer(0);
            let mut dl: i32 = 0;
            // Java: Integer DR = new Integer(0);
            let mut dr: i32 = 0;
            // Java: this.cipherNToHL64(this.mByte, 0, 8, DL, DR);
            self.cipherNToHL64_read(0, 8, &mut dl, &mut dr);
            // Java: this.encode(DL, DR);
            self.encode_pair(&mut dl, &mut dr);
            // Java: this.cipherHToNL64(DL, DR, this.mByte, 0, 8);
            Self::cipherHToNL64(dl, dr, &mut self.mByte, 0, 8);
        }
        // Java: Integer var10 = null;
        // Java: Object var13 = null;

        // Java: if (this.mDirection == CipherDirection.Cipher_DirectionEncode)
        if self.base.mDirection == CIPHER_DIRECTION_ENCODE {
            // Java: byte[] var10000 = this.mByte;
            // Java: var10000[k] ^= data[offset];
            self.mByte[k] ^= data[offset];
            // Java: buffer[bufferOffset] = this.mByte[k];
            buffer[bufferOffset] = self.mByte[k];
        }
        // Java: else
        else {
            // Java: byte c = this.mByte[k];
            let c = self.mByte[k];
            // Java: this.mByte[k] = data[offset];
            self.mByte[k] = data[offset];
            // Java: buffer[bufferOffset] = (byte)(data[offset] ^ c);
            buffer[bufferOffset] = data[offset] ^ c;
            // Java: c = 0;
        }
        // Java: this.mOffset = k + 1 & 7;
        self.mOffset = (k as i32 + 1) & 7;
    }

    // Java: void updateECB(byte[] data, int offset, int nLen, byte[] buffer, int bufferOffset, int blockSize)
    fn updateECB(&mut self, data: &[u8], offset: usize, _nLen: i32, buffer: &mut [u8], bufferOffset: usize, blockSize: i32) {
        // Java: Integer DL = new Integer(0);
        let mut dl: i32 = 0;
        // Java: Integer DR = new Integer(0);
        let mut dr: i32 = 0;
        // Java: if (this.mDirection == CipherDirection.Cipher_DirectionEncode)
        if self.base.mDirection == CIPHER_DIRECTION_ENCODE {
            // Java: this.cipherNToHL64(data, offset, blockSize, DL, DR);
            Self::cipherNToHL64_static(data, offset, blockSize, &mut dl, &mut dr);
            // Java: this.encode(DL, DR);
            self.encode_pair(&mut dl, &mut dr);
            // Java: this.cipherHToNL(buffer, bufferOffset, DL);
            Self::cipherHToNL(buffer, bufferOffset, dl);
            // Java: this.cipherHToNL(buffer, bufferOffset + 4, DR);
            Self::cipherHToNL(buffer, bufferOffset + 4, dr);
        }
        // Java: else
        else {
            // Java: this.cipherNToHL(data, offset, DL);
            Self::cipherNToHL_read(data, offset, &mut dl);
            // Java: this.cipherNToHL(data, offset + 4, DR);
            Self::cipherNToHL_read(data, offset + 4, &mut dr);
            // Java: this.decode(DL, DR);
            self.decode_pair(&mut dl, &mut dr);
            // Java: this.cipherHToNL64(DL, DR, buffer, bufferOffset, blockSize);
            Self::cipherHToNL64(dl, dr, buffer, bufferOffset, blockSize);
        }
        // Java: Integer var9 = null;
        // Java: Integer var10 = null;
    }

    // Java: void updateCBC(byte[] data, int offset, int nLen, byte[] buffer, int bufferOffset, int blockSize)
    fn updateCBC(&mut self, data: &[u8], offset: usize, _nLen: i32, buffer: &mut [u8], bufferOffset: usize, blockSize: i32) {
        // Java: Integer DL = new Integer(0);
        let mut dl: i32 = 0;
        // Java: Integer DR = new Integer(0);
        let mut dr: i32 = 0;
        // Java: if (this.mDirection == CipherDirection.Cipher_DirectionEncode)
        if self.base.mDirection == CIPHER_DIRECTION_ENCODE {
            // Java: this.cipherNToHL64(data, offset, blockSize, DL, DR);
            Self::cipherNToHL64_static(data, offset, blockSize, &mut dl, &mut dr);
            // Java: DL = DL ^ LittleEndian.getInt(this.mByte, 0);
            dl ^= le_get_int(&self.mByte, 0);
            // Java: DR = DR ^ LittleEndian.getInt(this.mByte, 4);
            dr ^= le_get_int(&self.mByte, 4);
            // Java: this.encode(DL, DR);
            self.encode_pair(&mut dl, &mut dr);
            // Java: LittleEndian.putInt(this.mByte, 0, DL);
            le_put_int(&mut self.mByte, 0, dl);
            // Java: LittleEndian.putInt(this.mByte, 4, DR);
            le_put_int(&mut self.mByte, 4, dr);
            // Java: this.cipherHToNL(buffer, bufferOffset, DL);
            Self::cipherHToNL(buffer, bufferOffset, dl);
            // Java: this.cipherHToNL(buffer, bufferOffset + 4, DR);
            Self::cipherHToNL(buffer, bufferOffset + 4, dr);
        }
        // Java: else
        else {
            // Java: this.cipherNToHL(data, offset, DL);
            Self::cipherNToHL_read(data, offset, &mut dl);
            // Java: this.cipherNToHL(data, offset + 4, DR);
            Self::cipherNToHL_read(data, offset + 4, &mut dr);
            // Java: int IVL = DL;
            let ivl = dl;
            // Java: int IVR = DR;
            let ivr = dr;
            // Java: this.decode(DL, DR);
            self.decode_pair(&mut dl, &mut dr);
            // Java: DL = DL ^ LittleEndian.getInt(this.mByte, 0);
            dl ^= le_get_int(&self.mByte, 0);
            // Java: DR = DR ^ LittleEndian.getInt(this.mByte, 4);
            dr ^= le_get_int(&self.mByte, 4);
            // Java: LittleEndian.putInt(this.mByte, 0, IVL);
            le_put_int(&mut self.mByte, 0, ivl);
            // Java: LittleEndian.putInt(this.mByte, 4, IVR);
            le_put_int(&mut self.mByte, 4, ivr);
            // Java: this.cipherHToNL64(DL, DR, buffer, bufferOffset, blockSize);
            Self::cipherHToNL64(dl, dr, buffer, bufferOffset, blockSize);
            // Java: IVR = 0;
            // Java: IVL = 0;
        }
        // Java: Integer var13 = null;
        // Java: Integer var16 = null;
    }

    // Java: void cipherNToHL(byte[] buffer, int bufferOffset, Integer val)
    fn cipherNToHL_read(buffer: &[u8], bufferOffset: usize, val: &mut i32) {
        // Java: val = buffer[bufferOffset++] << 24;
        *val = (buffer[bufferOffset] as i32) << 24;
        // Java: val = val | buffer[bufferOffset++] << 16;
        *val = *val | ((buffer[bufferOffset + 1] as i32) << 16);
        // Java: val = val | buffer[bufferOffset++] << 8;
        *val = *val | ((buffer[bufferOffset + 2] as i32) << 8);
        // Java: val = val | buffer[bufferOffset++];
        *val = *val | (buffer[bufferOffset + 3] as i32);
    }

    // Java: void cipherHToNL(byte[] buffer, int bufferOffset, int val)
    fn cipherHToNL(buffer: &mut [u8], bufferOffset: usize, val: i32) {
        // Java: buffer[bufferOffset++] = 0;
        buffer[bufferOffset] = (val >> 24) as u8;
        // Java: buffer[bufferOffset++] = 0;
        buffer[bufferOffset + 1] = (val >> 16) as u8;
        // Java: buffer[bufferOffset++] = 0;
        buffer[bufferOffset + 2] = (val >> 8) as u8;
        // Java: buffer[bufferOffset++] = 1;
        buffer[bufferOffset + 3] = val as u8;
    }

    // Java: void cipherHToNL64(int DL, int DR, byte[] buffer, int offset, int nLen)
    fn cipherHToNL64(dl: i32, dr: i32, buffer: &mut [u8], offset: usize, nLen: i32) {
        let n = nLen as usize;
        // Java: switch (nLen)
        // Java: case 8:
        // Java: buffer[offset + nLen - 1] = (byte)(DR & 255);
        if n >= 8 { buffer[offset + n - 1] = (dr & 255) as u8; }
        // Java: case 7:
        // Java: buffer[offset + nLen - 2] = (byte)(DR >> 8 & 255);
        if n >= 7 { buffer[offset + n - 2] = ((dr >> 8) & 255) as u8; }
        // Java: case 6:
        // Java: buffer[offset + nLen - 3] = (byte)(DR >> 16 & 255);
        if n >= 6 { buffer[offset + n - 3] = ((dr >> 16) & 255) as u8; }
        // Java: case 5:
        // Java: buffer[offset + nLen - 4] = (byte)(DR >> 24 & 255);
        if n >= 5 { buffer[offset + n - 4] = ((dr >> 24) & 255) as u8; }
        // Java: case 4:
        // Java: buffer[offset + nLen - 5] = (byte)(DL & 255);
        if n >= 4 { buffer[offset + n - 5] = (dl & 255) as u8; }
        // Java: case 3:
        // Java: buffer[offset + nLen - 6] = (byte)(DL >> 8 & 255);
        if n >= 3 { buffer[offset + n - 6] = ((dl >> 8) & 255) as u8; }
        // Java: case 2:
        // Java: buffer[offset + nLen - 7] = (byte)(DL >> 16 & 255);
        if n >= 2 { buffer[offset + n - 7] = ((dl >> 16) & 255) as u8; }
        // Java: case 1:
        // Java: buffer[offset + nLen - 8] = (byte)(DL >> 24 & 255);
        if n >= 1 { buffer[offset + n - 8] = ((dl >> 24) & 255) as u8; }
        // Java: default:
    }

    // Java: void cipherNToHL64(byte[] data, int offset, int nLen, Integer DL, Integer DR)
    fn cipherNToHL64_static(data: &[u8], offset: usize, nLen: i32, dl: &mut i32, dr: &mut i32) {
        let n = nLen as usize;
        let mut off = offset;
        *dl = 0;
        *dr = 0;
        // Java: switch (nLen)
        // Java: case 8:
        if n >= 8 {
            // Java: DR = Integer.valueOf(data[offset + nLen - 1]);
            *dr = data[off + n - 1] as i32;
            // Java: --offset;
            off = off.wrapping_sub(1);
        }
        // Java: case 7:
        if n >= 7 {
            // Java: DR = DR | data[offset + nLen - 1] << 8;
            *dr = *dr | ((data[off + n - 1] as i32) << 8);
            // Java: --offset;
            off = off.wrapping_sub(1);
        }
        // Java: case 6:
        if n >= 6 {
            // Java: DR = DR | data[offset + nLen - 1] << 16;
            *dr = *dr | ((data[off + n - 1] as i32) << 16);
            // Java: --offset;
            off = off.wrapping_sub(1);
        }
        // Java: case 5:
        if n >= 5 {
            // Java: DR = DR | data[offset + nLen - 1] << 24;
            *dr = *dr | ((data[off + n - 1] as i32) << 24);
            // Java: --offset;
            off = off.wrapping_sub(1);
        }
        // Java: case 4:
        // Java: --offset;
        if n >= 4 { off = off.wrapping_sub(1); }
        // Java: case 3:
        // Java: --offset;
        if n >= 3 { off = off.wrapping_sub(1); }
        // Java: case 2:
        // Java: --offset;
        if n >= 2 { off = off.wrapping_sub(1); }
        // Java: case 1:
        // Java: --offset;
        if n >= 1 { let _ = off.wrapping_sub(1); }
        // Java: default:
    }

    fn cipherNToHL64_read(&self, offset: usize, nLen: i32, dl: &mut i32, dr: &mut i32) {
        Self::cipherNToHL64_static(&self.mByte, offset, nLen, dl, dr);
    }
}

impl Cipher for CipherBF {
    fn init(&mut self, var1: i32, var2: &[u8]) -> i32 {
        self.init_impl(var1, var2)
    }

    fn update(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32 {
        self.update_impl(var1, var2, var3, var4)
    }

    fn encode(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32 {
        self.encode_impl(var1, var2, var3, var4)
    }

    fn decode(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32 {
        self.decode_impl(var1, var2, var3, var4)
    }

    fn dispose(&mut self) {
        self.dispose_impl()
    }
}
