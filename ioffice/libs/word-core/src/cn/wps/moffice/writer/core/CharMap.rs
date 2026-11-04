// java private static final int CC_SomeShared = -1;
const CC_SomeShared: i32 = -1;
// java private static CharMap msCharMap = null;
static msCharMap: std::sync::LazyLock<std::sync::Arc<Option<Box<CharMap>>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(None));
// java private static final int s = 4;
const s: i32 = 4;
// java private static final int n = 3;
const n: i32 = 3;
// java public static final int UNICODE_OFFSET = 128;
pub(crate) const UNICODE_OFFSET: i32 = 128;
// java public static final byte[] CHAR_CLASS_TABLE = new byte[]{3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 4, 3, 3, 4, 4, 3, 4, 3, 3, 4, 3, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 4, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3};
pub(crate) static CHAR_CLASS_TABLE: std::sync::LazyLock<Vec<u8>> =
                    std::sync::LazyLock::new(|| vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 4, 3, 3, 4, 4, 3, 4, 3, 3, 4, 3, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 4, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);

// Java CharMap.mData, generated verbatim from CharMap.java Item constructors.
const JAVA_ITEM_TABLE: &[(i32, i32, i32, i32)] = &[
    (0, 0, 127, 1),
    (1, 128, 255, -1),
    (2, 256, 383, -1),
    (3, 384, 591, -1),
    (4, 592, 687, -1),
    (5, 688, 767, 4),
    (6, 768, 879, 4),
    (7, 880, 1023, 4),
    (8, 1024, 1279, 4),
    (9, 1280, 1327, 3),
    (10, 1328, 1423, 3),
    (11, 1424, 1535, 5),
    (12, 1536, 1791, 5),
    (13, 1792, 1871, 5),
    (14, 1872, 1919, 5),
    (15, 1920, 1983, 5),
    (16, 1984, 2047, 5),
    (17, 2048, 2111, 5),
    (18, 2112, 2143, 5),
    (20, 2208, 2303, 5),
    (21, 2304, 2431, 5),
    (22, 2432, 2559, 5),
    (23, 2560, 2687, 5),
    (24, 2688, 2815, 5),
    (25, 2816, 2943, 5),
    (26, 2944, 3071, 5),
    (27, 3072, 3199, 5),
    (28, 3200, 3327, 5),
    (29, 3328, 3455, 5),
    (30, 3456, 3583, 5),
    (31, 3584, 3711, 5),
    (32, 3712, 3839, 5),
    (33, 3840, 4095, 5),
    (34, 4096, 4255, 3),
    (35, 4256, 4351, 3),
    (36, 4352, 4607, 3),
    (37, 4608, 4991, 3),
    (38, 4992, 5023, 3),
    (39, 5024, 5119, 3),
    (40, 5120, 5759, 3),
    (41, 5760, 5791, 3),
    (42, 5792, 5887, 3),
    (43, 5888, 5919, 3),
    (44, 5920, 5951, 3),
    (45, 5952, 5983, 5),
    (46, 5984, 6015, 5),
    (47, 6016, 6143, 5),
    (48, 6144, 6319, 5),
    (49, 6320, 6399, 3),
    (50, 6400, 6479, 2),
    (51, 6480, 6527, 2),
    (52, 6528, 6623, 2),
    (53, 6624, 6655, 2),
    (54, 6656, 6687, 2),
    (55, 6688, 6831, 2),
    (57, 6912, 7039, 2),
    (58, 7040, 7103, 2),
    (59, 7104, 7167, 2),
    (60, 7168, 7247, 2),
    (61, 7248, 7295, 2),
    (63, 7360, 7375, 2),
    (64, 7376, 7423, 2),
    (65, 7424, 7551, 4),
    (66, 7552, 7615, 4),
    (67, 7616, 7679, 4),
    (68, 7680, 7935, 4),
    (69, 7936, 8191, 3),
    (70, 8192, 8303, 4),
    (71, 8304, 8351, 4),
    (72, 8352, 8399, 4),
    (73, 8400, 8447, 4),
    (74, 8448, 8527, 4),
    (75, 8528, 8591, 4),
    (76, 8592, 8703, 4),
    (77, 8704, 8959, 4),
    (78, 8960, 9215, 4),
    (79, 9216, 9279, 4),
    (80, 9280, 9311, 4),
    (81, 9312, 9471, 4),
    (82, 9472, 9599, 4),
    (83, 9600, 9631, 4),
    (84, 9632, 9727, 4),
    (85, 9728, 9983, 4),
    (86, 9984, 10175, 4),
    (87, 10176, 10223, 4),
    (88, 10224, 10239, 4),
    (89, 10240, 10495, 4),
    (90, 10496, 10623, 4),
    (91, 10624, 10751, 4),
    (92, 10752, 11007, 4),
    (93, 11008, 11263, 4),
    (94, 11264, 11359, 3),
    (95, 11360, 11391, 4),
    (96, 11392, 11519, 3),
    (97, 11520, 11567, 3),
    (98, 11568, 11647, 3),
    (99, 11648, 11743, 3),
    (100, 11744, 11775, 3),
    (101, 11776, 11903, 4),
    (102, 11904, 12031, 2),
    (103, 12032, 12255, 2),
    (105, 12272, 12287, 2),
    (106, 12288, 12351, 2),
    (107, 12352, 12447, 2),
    (108, 12448, 12543, 2),
    (109, 12544, 12591, 2),
    (110, 12592, 12687, 2),
    (111, 12688, 12703, 2),
    (112, 12704, 12735, 2),
    (113, 12736, 12783, 2),
    (114, 12784, 12799, 2),
    (115, 12800, 13055, 4),
    (116, 13056, 13311, 2),
    (117, 13312, 19903, 2),
    (118, 19904, 19967, 2),
    (119, 19968, 40911, 2),
    (121, 40960, 42127, 2),
    (122, 42128, 42191, 2),
    (123, 42192, 42239, 2),
    (124, 42240, 42559, 3),
    (125, 42560, 42655, 3),
    (126, 42656, 42751, 3),
    (127, 42752, 42783, 3),
    (128, 42784, 43007, 4),
    (129, 43008, 43055, 2),
    (130, 43056, 43071, 2),
    (131, 43072, 43135, 3),
    (132, 43136, 43231, 2),
    (133, 43232, 43263, 2),
    (134, 43264, 43311, 2),
    (135, 43312, 43359, 2),
    (136, 43360, 43391, 2),
    (137, 43392, 43487, 2),
    (139, 43520, 43615, 2),
    (140, 43616, 43647, 2),
    (141, 43648, 43743, 2),
    (142, 43744, 43775, 2),
    (143, 43776, 43823, 3),
    (145, 43968, 44031, 2),
    (146, 44032, 55215, 2),
    (147, 55216, 55295, 2),
    (148, 55296, 56319, 2),
    (149, 56320, 57343, 2),
    (150, 57344, 63743, 4),
    (151, 63744, 64255, 2),
    (152, 64256, 64335, 4),
    (153, 64336, 65023, 4),
    (154, 65024, 65039, 4),
    (155, 65040, 65055, 4),
    (156, 65056, 65071, 2),
    (157, 65072, 65103, 2),
    (158, 65104, 65135, 2),
    (159, 65136, 65279, 4),
    (160, 65280, 65519, 2),
    (161, 65520, 65535, 3),
    (162, 65536, 65663, 3),
    (163, 65664, 65791, 3),
    (164, 65792, 65855, 6),
    (165, 65856, 65935, 6),
    (166, 65936, 65999, 6),
    (167, 66000, 66047, 3),
    (169, 66176, 66207, 3),
    (170, 66208, 66271, 3),
    (172, 66304, 66351, 3),
    (173, 66352, 66383, 3),
    (175, 66432, 66463, 3),
    (176, 66464, 66527, 3),
    (178, 66560, 66639, 6),
    (179, 66640, 66687, 3),
    (180, 66688, 66735, 3),
    (182, 67584, 67647, 3),
    (183, 67648, 67679, 3),
    (185, 67840, 67871, 3),
    (186, 67872, 67903, 3),
    (188, 67968, 67999, 3),
    (189, 68000, 68095, 3),
    (190, 68096, 68191, 2),
    (191, 68192, 68223, 3),
    (193, 68352, 68415, 3),
    (194, 68416, 68447, 3),
    (195, 68448, 68479, 3),
    (197, 68608, 68687, 3),
    (199, 69216, 69247, 6),
    (201, 69632, 69759, 2),
    (202, 69760, 69839, 2),
    (203, 69840, 69887, 2),
    (204, 69888, 69967, 2),
    (206, 70016, 70111, 2),
    (208, 71296, 71375, 2),
    (210, 73728, 74751, 3),
    (211, 74752, 74879, 3),
    (213, 77824, 78895, 3),
    (215, 92160, 92735, 3),
    (217, 93952, 94111, 2),
    (219, 110592, 110847, 2),
    (221, 118784, 119039, 6),
    (222, 119040, 119295, 6),
    (223, 119296, 119375, 6),
    (225, 119552, 119647, 6),
    (226, 119648, 119679, 6),
    (228, 119808, 120831, 6),
    (230, 126976, 127023, 4),
    (232, 127136, 127231, 4),
    (233, 127232, 127487, 4),
    (234, 127488, 127743, 4),
    (235, 127744, 128511, 4),
    (236, 128512, 128591, 4),
    (238, 128640, 128767, 4),
    (239, 128768, 128895, 6),
    (241, 131072, 173791, 2),
    (243, 173824, 177983, 2),
    (244, 177984, 178207, 2),
    (246, 194560, 195103, 2),
    (248, 917504, 917631, 6),
    (250, 917760, 917999, 6),
    (252, 983040, 1048575, 6),
    (253, 1048576, 1114111, 6),
];

// java public final class CharMap
#[derive(Default)]
pub struct CharMap {
    // java final Item[] mData = new Item[]{new Item(0, 0, 127, 1), new Item(1, 128, 255, -1), new Item(2, 256, 383, -1), new Item(3, 384, 591, -1), new Item(4, 592, 687, -1), new Item(5, 688, 767, 4), new Item(6, 768, 879, 4), new Item(7, 880, 1023, 4), new Item(8, 1024, 1279, 4), new Item(9, 1280, 1327, 3), new Item(10, 1328, 1423, 3), new Item(11, 1424, 1535, 5), new Item(12, 1536, 1791, 5), new Item(13, 1792, 1871, 5), new Item(14, 1872, 1919, 5), new Item(15, 1920, 1983, 5), new Item(16, 1984, 2047, 5), new Item(17, 2048, 2111, 5), new Item(18, 2112, 2143, 5), new Item(20, 2208, 2303, 5), new Item(21, 2304, 2431, 5), new Item(22, 2432, 2559, 5), new Item(23, 2560, 2687, 5), new Item(24, 2688, 2815, 5), new Item(25, 2816, 2943, 5), new Item(26, 2944, 3071, 5), new Item(27, 3072, 3199, 5), new Item(28, 3200, 3327, 5), new Item(29, 3328, 3455, 5), new Item(30, 3456, 3583, 5), new Item(31, 3584, 3711, 5), new Item(32, 3712, 3839, 5), new Item(33, 3840, 4095, 5), new Item(34, 4096, 4255, 3), new Item(35, 4256, 4351, 3), new Item(36, 4352, 4607, 3), new Item(37, 4608, 4991, 3), new Item(38, 4992, 5023, 3), new Item(39, 5024, 5119, 3), new Item(40, 5120, 5759, 3), new Item(41, 5760, 5791, 3), new Item(42, 5792, 5887, 3), new Item(43, 5888, 5919, 3), new Item(44, 5920, 5951, 3), new Item(45, 5952, 5983, 5), new Item(46, 5984, 6015, 5), new Item(47, 6016, 6143, 5), new Item(48, 6144, 6319, 5), new Item(49, 6320, 6399, 3), new Item(50, 6400, 6479, 2), new Item(51, 6480, 6527, 2), new Item(52, 6528, 6623, 2), new Item(53, 6624, 6655, 2), new Item(54, 6656, 6687, 2), new Item(55, 6688, 6831, 2), new Item(57, 6912, 7039, 2), new Item(58, 7040, 7103, 2), new Item(59, 7104, 7167, 2), new Item(60, 7168, 7247, 2), new Item(61, 7248, 7295, 2), new Item(63, 7360, 7375, 2), new Item(64, 7376, 7423, 2), new Item(65, 7424, 7551, 4), new Item(66, 7552, 7615, 4), new Item(67, 7616, 7679, 4), new Item(68, 7680, 7935, 4), new Item(69, 7936, 8191, 3), new Item(70, 8192, 8303, 4), new Item(71, 8304, 8351, 4), new Item(72, 8352, 8399, 4), new Item(73, 8400, 8447, 4), new Item(74, 8448, 8527, 4), new Item(75, 8528, 8591, 4), new Item(76, 8592, 8703, 4), new Item(77, 8704, 8959, 4), new Item(78, 8960, 9215, 4), new Item(79, 9216, 9279, 4), new Item(80, 9280, 9311, 4), new Item(81, 9312, 9471, 4), new Item(82, 9472, 9599, 4), new Item(83, 9600, 9631, 4), new Item(84, 9632, 9727, 4), new Item(85, 9728, 9983, 4), new Item(86, 9984, 10175, 4), new Item(87, 10176, 10223, 4), new Item(88, 10224, 10239, 4), new Item(89, 10240, 10495, 4), new Item(90, 10496, 10623, 4), new Item(91, 10624, 10751, 4), new Item(92, 10752, 11007, 4), new Item(93, 11008, 11263, 4), new Item(94, 11264, 11359, 3), new Item(95, 11360, 11391, 4), new Item(96, 11392, 11519, 3), new Item(97, 11520, 11567, 3), new Item(98, 11568, 11647, 3), new Item(99, 11648, 11743, 3), new Item(100, 11744, 11775, 3), new Item(101, 11776, 11903, 4), new Item(102, 11904, 12031, 2), new Item(103, 12032, 12255, 2), new Item(105, 12272, 12287, 2), new Item(106, 12288, 12351, 2), new Item(107, 12352, 12447, 2), new Item(108, 12448, 12543, 2), new Item(109, 12544, 12591, 2), new Item(110, 12592, 12687, 2), new Item(111, 12688, 12703, 2), new Item(112, 12704, 12735, 2), new Item(113, 12736, 12783, 2), new Item(114, 12784, 12799, 2), new Item(115, 12800, 13055, 4), new Item(116, 13056, 13311, 2), new Item(117, 13312, 19903, 2), new Item(118, 19904, 19967, 2), new Item(119, 19968, 40911, 2), new Item(121, 40960, 42127, 2), new Item(122, 42128, 42191, 2), new Item(123, 42192, 42239, 2), new Item(124, 42240, 42559, 3), new Item(125, 42560, 42655, 3), new Item(126, 42656, 42751, 3), new Item(127, 42752, 42783, 3), new Item(128, 42784, 43007, 4), new Item(129, 43008, 43055, 2), new Item(130, 43056, 43071, 2), new Item(131, 43072, 43135, 3), new Item(132, 43136, 43231, 2), new Item(133, 43232, 43263, 2), new Item(134, 43264, 43311, 2), new Item(135, 43312, 43359, 2), new Item(136, 43360, 43391, 2), new Item(137, 43392, 43487, 2), new Item(139, 43520, 43615, 2), new Item(140, 43616, 43647, 2), new Item(141, 43648, 43743, 2), new Item(142, 43744, 43775, 2), new Item(143, 43776, 43823, 3), new Item(145, 43968, 44031, 2), new Item(146, 44032, 55215, 2), new Item(147, 55216, 55295, 2), new Item(148, 55296, 56319, 2), new Item(149, 56320, 57343, 2), new Item(150, 57344, 63743, 4), new Item(151, 63744, 64255, 2), new Item(152, 64256, 64335, 4), new Item(153, 64336, 65023, 4), new Item(154, 65024, 65039, 4), new Item(155, 65040, 65055, 4), new Item(156, 65056, 65071, 2), new Item(157, 65072, 65103, 2), new Item(158, 65104, 65135, 2), new Item(159, 65136, 65279, 4), new Item(160, 65280, 65519, 2), new Item(161, 65520, 65535, 3), new Item(162, 65536, 65663, 3), new Item(163, 65664, 65791, 3), new Item(164, 65792, 65855, 6), new Item(165, 65856, 65935, 6), new Item(166, 65936, 65999, 6), new Item(167, 66000, 66047, 3), new Item(169, 66176, 66207, 3), new Item(170, 66208, 66271, 3), new Item(172, 66304, 66351, 3), new Item(173, 66352, 66383, 3), new Item(175, 66432, 66463, 3), new Item(176, 66464, 66527, 3), new Item(178, 66560, 66639, 6), new Item(179, 66640, 66687, 3), new Item(180, 66688, 66735, 3), new Item(182, 67584, 67647, 3), new Item(183, 67648, 67679, 3), new Item(185, 67840, 67871, 3), new Item(186, 67872, 67903, 3), new Item(188, 67968, 67999, 3), new Item(189, 68000, 68095, 3), new Item(190, 68096, 68191, 2), new Item(191, 68192, 68223, 3), new Item(193, 68352, 68415, 3), new Item(194, 68416, 68447, 3), new Item(195, 68448, 68479, 3), new Item(197, 68608, 68687, 3), new Item(199, 69216, 69247, 6), new Item(201, 69632, 69759, 2), new Item(202, 69760, 69839, 2), new Item(203, 69840, 69887, 2), new Item(204, 69888, 69967, 2), new Item(206, 70016, 70111, 2), new Item(208, 71296, 71375, 2), new Item(210, 73728, 74751, 3), new Item(211, 74752, 74879, 3), new Item(213, 77824, 78895, 3), new Item(215, 92160, 92735, 3), new Item(217, 93952, 94111, 2), new Item(219, 110592, 110847, 2), new Item(221, 118784, 119039, 6), new Item(222, 119040, 119295, 6), new Item(223, 119296, 119375, 6), new Item(225, 119552, 119647, 6), new Item(226, 119648, 119679, 6), new Item(228, 119808, 120831, 6), new Item(230, 126976, 127023, 4), new Item(232, 127136, 127231, 4), new Item(233, 127232, 127487, 4), new Item(234, 127488, 127743, 4), new Item(235, 127744, 128511, 4), new Item(236, 128512, 128591, 4), new Item(238, 128640, 128767, 4), new Item(239, 128768, 128895, 6), new Item(241, 131072, 173791, 2), new Item(243, 173824, 177983, 2), new Item(244, 177984, 178207, 2), new Item(246, 194560, 195103, 2), new Item(248, 917504, 917631, 6), new Item(250, 917760, 917999, 6), new Item(252, 983040, 1048575, 6), new Item(253, 1048576, 1114111, 6)};
    pub(crate) mData: Option<Vec<Item>>,
    // java final int mCodeMaxIndex;
    pub(crate) mCodeMaxIndex: i32,
    // java final int mCharMaxIndex;
    pub(crate) mCharMaxIndex: i32,
    // java final int mMaxCodePoint;
    pub(crate) mMaxCodePoint: i32,
}

// java private static final class SomeSharedCharClassTable
#[derive(Default)]
pub struct SomeSharedCharClassTable {
}

// java static final class Item
#[derive(Default, Clone, Copy)]
pub struct Item {
    // java int start;
    pub(crate) start: i32,
    // java int end;
    pub(crate) end: i32,
    // java public int unicodeId;
    pub(crate) unicodeId: i32,
    // java private int charType;
    charType: i32,
}

// java public final class CharMap
impl CharMap {
    // java private  CharMap()
    // Java mData constructors are copied into JAVA_ITEM_TABLE above in the same order.
    pub fn CharMap() -> Self {
      // java this.mCodeMaxIndex = this.mData.length - 1;
      // java this.mCharMaxIndex = this.getCharMaxIndex();
      // java this.mMaxCodePoint = ...;
      let mData: Vec<Item> = JAVA_ITEM_TABLE.iter()
          .map(|&(id, start, end, char_type)| Item::Item(id, start, end, char_type))
          .collect();
      let mut map = Self {
          mCodeMaxIndex: mData.len() as i32 - 1,
          mCharMaxIndex: -1,
          mMaxCodePoint: mData.last().map(|item| item.end).unwrap_or(0),
          mData: Some(mData),
      };
      map.mCharMaxIndex = map.getCharMaxIndex();
      map
    }



    // java private int getCharMaxIndex()
    pub fn getCharMaxIndex(&self) -> i32 {
      // java int max = this.mCodeMaxIndex;
      let mut max: i32 = self.mCodeMaxIndex;
      let mut i: i32 = 0;
      // java for(int i = 0; i < this.mData.length; ++i)
      while i < (self.mData.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) {
        // java if (this.mData[i].unicodeId == 161)
        if self.mData.as_ref().unwrap()[(i) as usize].unicodeId == 161 {
          // java max = i;
          max = i;
          // java break;
          break;
        }
        // java ++i;
        i += 1;
      }
      // java return max;
      return max;
    }

    // java public static CharMap getInstance()
    // Rust value semantics: the immutable table is copied per CharParse instance.
    pub fn getInstance() -> CharMap {
      // java if (msCharMap == null) { msCharMap = new CharMap(); } return msCharMap;
      CharMap::CharMap()
    }

    // java public Item getCharItem(char c)
    pub fn getCharItem__char(&mut self, c: char) -> Option<Item> {
      // java int codePoint = c & '\uffff';
      let codePoint = (c as u32 & 0xFFFF) as i32;
      // java return this.binarySearch(codePoint, this.mCharMaxIndex);
      self.binarySearch(codePoint, self.mCharMaxIndex)
    }

    // java public Item getCharItem(int codePoint)
    pub fn getCharItem__int(&mut self, codePoint: i32) -> Option<Item> {
      // java return codePoint <= this.mMaxCodePoint && codePoint >= 0 ? this.binarySearch(codePoint, this.mCodeMaxIndex) : null;
      if codePoint <= self.mMaxCodePoint && codePoint >= 0 {
          self.binarySearch(codePoint, self.mCodeMaxIndex)
      } else {
          None
      }
    }

    // java private Item binarySearch(int codePoint, int high)
    pub fn binarySearch(&self, codePoint: i32, mut high: i32) -> Option<Item> {
      // java int low = 0;
      let mut low = 0;
      // java while (low <= high)
      while low <= high {
          // java int mid = low + (high - low) / 2;
          let mid = low + (high - low) / 2;
          let item = self.mData.as_ref()?.get(mid as usize).copied()?;
          // java int midVal = this.mData[mid].start;
          let midVal = item.start;
          // java if (codePoint < midVal) { high = mid - 1; }
          if codePoint < midVal {
              high = mid - 1;
          } else {
              // java if (codePoint <= midVal) { return this.mData[mid]; }
              if codePoint <= midVal { return Some(item); }
              // java if (codePoint <= this.mData[mid].end) { return this.mData[mid]; }
              if codePoint <= item.end { return Some(item); }
              // java low = mid + 1;
              low = mid + 1;
          }
      }
      // java return null;
      None
    }


}

// java private static final class SomeSharedCharClassTable
impl SomeSharedCharClassTable {
}

// java static final class Item
impl Item {
    // java public  Item(int id, int start, int end, int charType)
    pub fn Item(id: i32, start: i32, end: i32, charType: i32) -> Self {
      // java this.unicodeId = id;
      Self { start: start, end: end, unicodeId: id, charType: charType }
      // java this.start = start;
      // java this.end = end;
      // java this.charType = charType;
    }

    // java public int getCharTypeUnchecked()
    pub fn getCharTypeUnchecked(&self) -> i32 {
      // java return this.charType;
      return self.charType;
    }

    // java public int getCharType(int unicode)
    pub fn getCharType(&self, unicode: i32) -> i32 {
      // java int ret = this.charType;
      let mut ret: i32 = self.charType;
      // java if (-1 != ret)
      if -1 != ret {
        // java return ret;
        return ret;
      }
      // java else
      else {
        // java int idx = unicode - 128;
        let idx: i32 = unicode - 128;
        // java return CharMap.SomeSharedCharClassTable.CHAR_CLASS_TABLE[idx];
        // rust: CHAR_CLASS_TABLE 为箱级静态 → 直接索引（clamp 防越界）
        return CHAR_CLASS_TABLE[(idx.clamp(0, 559)) as usize] as i32;
      }
    }

}
