// Java: public class Verifier
pub struct Verifier;

// Java: public class Verifier
impl Verifier {
    // Java: Verifier()
    pub fn new() -> Self {
      // Java: super();
      Verifier
    }

    // Java: static boolean isXMLNCNameCharacter(char c)
    pub fn isXMLNCNameCharacter(c: char) -> bool {
      // Java: return isXMLLetter(c) || isXMLDigit(c) || c == '.' || c == '-' || c == '_' || isXMLCombiningChar(c) || isXMLExtender(c);
      Self::isXMLLetter(c) || Self::isXMLDigit(c) || c == '.' || c == '-' || c == '_' || Self::isXMLCombiningChar(c) || Self::isXMLExtender(c)
    }

    // Java: static boolean isXMLNCNameStartCharacter(char c)
    pub fn isXMLNCNameStartCharacter(c: char) -> bool {
      // Java: return isXMLLetter(c) || c == '_';
      Self::isXMLLetter(c) || c == '_'
    }

    // Java: static boolean isXMLLetter(char c)
    pub fn isXMLLetter(c: char) -> bool {
      let cv = c as u32;
      // Java: if (c < 'A')
      // Java: return false;
      if cv < 0x41 { return false; }
      // Java: else if (c <= 'Z')
      // Java: return true;
      else if cv <= 0x5A { return true; }
      // Java: else if (c < 'a')
      // Java: return false;
      else if cv < 0x61 { return false; }
      // Java: else if (c <= 'z')
      // Java: return true;
      else if cv <= 0x7A { return true; }
      // Java: else if (c < 192)
      // Java: return false;
      else if cv < 192 { return false; }
      // Java: else if (c <= 214)
      // Java: return true;
      else if cv <= 214 { return true; }
      // Java: else if (c < 216)
      // Java: return false;
      else if cv < 216 { return false; }
      // Java: else if (c <= 246)
      // Java: return true;
      else if cv <= 246 { return true; }
      // Java: else if (c < 248)
      // Java: return false;
      else if cv < 248 { return false; }
      // Java: else if (c <= 255)
      // Java: return true;
      else if cv <= 255 { return true; }
      // Java: else if (c <= 305)
      // Java: return true;
      else if cv <= 305 { return true; }
      // Java: else if (c < 308)
      // Java: return false;
      else if cv < 308 { return false; }
      // Java: else if (c <= 318)
      // Java: return true;
      else if cv <= 318 { return true; }
      // Java: else if (c < 321)
      // Java: return false;
      else if cv < 321 { return false; }
      // Java: else if (c <= 328)
      // Java: return true;
      else if cv <= 328 { return true; }
      // Java: else if (c < 330)
      // Java: return false;
      else if cv < 330 { return false; }
      // Java: else if (c <= 382)
      // Java: return true;
      else if cv <= 382 { return true; }
      // Java: else if (c < 384)
      // Java: return false;
      else if cv < 384 { return false; }
      // Java: else if (c <= 451)
      // Java: return true;
      else if cv <= 451 { return true; }
      // Java: else if (c < 461)
      // Java: return false;
      else if cv < 461 { return false; }
      // Java: else if (c <= 496)
      // Java: return true;
      else if cv <= 496 { return true; }
      // Java: else if (c < 500)
      // Java: return false;
      else if cv < 500 { return false; }
      // Java: else if (c <= 501)
      // Java: return true;
      else if cv <= 501 { return true; }
      // Java: else if (c < 506)
      // Java: return false;
      else if cv < 506 { return false; }
      // Java: else if (c <= 535)
      // Java: return true;
      else if cv <= 535 { return true; }
      // Java: else if (c < 592)
      // Java: return false;
      else if cv < 592 { return false; }
      // Java: else if (c <= 680)
      // Java: return true;
      else if cv <= 680 { return true; }
      // Java: else if (c < 699)
      // Java: return false;
      else if cv < 699 { return false; }
      // Java: else if (c <= 705)
      // Java: return true;
      else if cv <= 705 { return true; }
      // Java: else if (c == 902)
      // Java: return true;
      else if cv == 902 { return true; }
      // Java: else if (c < 904)
      // Java: return false;
      else if cv < 904 { return false; }
      // Java: else if (c <= 906)
      // Java: return true;
      else if cv <= 906 { return true; }
      // Java: else if (c == 908)
      // Java: return true;
      else if cv == 908 { return true; }
      // Java: else if (c < 910)
      // Java: return false;
      else if cv < 910 { return false; }
      // Java: else if (c <= 929)
      // Java: return true;
      else if cv <= 929 { return true; }
      // Java: else if (c < 931)
      // Java: return false;
      else if cv < 931 { return false; }
      // Java: else if (c <= 974)
      // Java: return true;
      else if cv <= 974 { return true; }
      // Java: else if (c < 976)
      // Java: return false;
      else if cv < 976 { return false; }
      // Java: else if (c <= 982)
      // Java: return true;
      else if cv <= 982 { return true; }
      // Java: else if (c == 986)
      // Java: return true;
      else if cv == 986 { return true; }
      // Java: else if (c == 988)
      // Java: return true;
      else if cv == 988 { return true; }
      // Java: else if (c == 990)
      // Java: return true;
      else if cv == 990 { return true; }
      // Java: else if (c == 992)
      // Java: return true;
      else if cv == 992 { return true; }
      // Java: else if (c < 994)
      // Java: return false;
      else if cv < 994 { return false; }
      // Java: else if (c <= 1011)
      // Java: return true;
      else if cv <= 1011 { return true; }
      // Java: else if (c < 1025)
      // Java: return false;
      else if cv < 1025 { return false; }
      // Java: else if (c <= 1036)
      // Java: return true;
      else if cv <= 1036 { return true; }
      // Java: else if (c < 1038)
      // Java: return false;
      else if cv < 1038 { return false; }
      // Java: else if (c <= 1103)
      // Java: return true;
      else if cv <= 1103 { return true; }
      // Java: else if (c < 1105)
      // Java: return false;
      else if cv < 1105 { return false; }
      // Java: else if (c <= 1116)
      // Java: return true;
      else if cv <= 1116 { return true; }
      // Java: else if (c < 1118)
      // Java: return false;
      else if cv < 1118 { return false; }
      // Java: else if (c <= 1153)
      // Java: return true;
      else if cv <= 1153 { return true; }
      // Java: else if (c < 1168)
      // Java: return false;
      else if cv < 1168 { return false; }
      // Java: else if (c <= 1220)
      // Java: return true;
      else if cv <= 1220 { return true; }
      // Java: else if (c < 1223)
      // Java: return false;
      else if cv < 1223 { return false; }
      // Java: else if (c <= 1224)
      // Java: return true;
      else if cv <= 1224 { return true; }
      // Java: else if (c < 1227)
      // Java: return false;
      else if cv < 1227 { return false; }
      // Java: else if (c <= 1228)
      // Java: return true;
      else if cv <= 1228 { return true; }
      // Java: else if (c < 1232)
      // Java: return false;
      else if cv < 1232 { return false; }
      // Java: else if (c <= 1259)
      // Java: return true;
      else if cv <= 1259 { return true; }
      // Java: else if (c < 1262)
      // Java: return false;
      else if cv < 1262 { return false; }
      // Java: else if (c <= 1269)
      // Java: return true;
      else if cv <= 1269 { return true; }
      // Java: else if (c < 1272)
      // Java: return false;
      else if cv < 1272 { return false; }
      // Java: else if (c <= 1273)
      // Java: return true;
      else if cv <= 1273 { return true; }
      // Java: else if (c < 1329)
      // Java: return false;
      else if cv < 1329 { return false; }
      // Java: else if (c <= 1366)
      // Java: return true;
      else if cv <= 1366 { return true; }
      // Java: else if (c == 1369)
      // Java: return true;
      else if cv == 1369 { return true; }
      // Java: else if (c < 1377)
      // Java: return false;
      else if cv < 1377 { return false; }
      // Java: else if (c <= 1414)
      // Java: return true;
      else if cv <= 1414 { return true; }
      // Java: else if (c < 1488)
      // Java: return false;
      else if cv < 1488 { return false; }
      // Java: else if (c <= 1514)
      // Java: return true;
      else if cv <= 1514 { return true; }
      // Java: else if (c < 1520)
      // Java: return false;
      else if cv < 1520 { return false; }
      // Java: else if (c <= 1522)
      // Java: return true;
      else if cv <= 1522 { return true; }
      // Java: else if (c < 1569)
      // Java: return false;
      else if cv < 1569 { return false; }
      // Java: else if (c <= 1594)
      // Java: return true;
      else if cv <= 1594 { return true; }
      // Java: else if (c < 1601)
      // Java: return false;
      else if cv < 1601 { return false; }
      // Java: else if (c <= 1610)
      // Java: return true;
      else if cv <= 1610 { return true; }
      // Java: else if (c < 1649)
      // Java: return false;
      else if cv < 1649 { return false; }
      // Java: else if (c <= 1719)
      // Java: return true;
      else if cv <= 1719 { return true; }
      // Java: else if (c < 1722)
      // Java: return false;
      else if cv < 1722 { return false; }
      // Java: else if (c <= 1726)
      // Java: return true;
      else if cv <= 1726 { return true; }
      // Java: else if (c < 1728)
      // Java: return false;
      else if cv < 1728 { return false; }
      // Java: else if (c <= 1742)
      // Java: return true;
      else if cv <= 1742 { return true; }
      // Java: else if (c < 1744)
      // Java: return false;
      else if cv < 1744 { return false; }
      // Java: else if (c <= 1747)
      // Java: return true;
      else if cv <= 1747 { return true; }
      // Java: else if (c == 1749)
      // Java: return true;
      else if cv == 1749 { return true; }
      // Java: else if (c < 1765)
      // Java: return false;
      else if cv < 1765 { return false; }
      // Java: else if (c <= 1766)
      // Java: return true;
      else if cv <= 1766 { return true; }
      // Java: else if (c < 2309)
      // Java: return false;
      else if cv < 2309 { return false; }
      // Java: else if (c <= 2361)
      // Java: return true;
      else if cv <= 2361 { return true; }
      // Java: else if (c == 2365)
      // Java: return true;
      else if cv == 2365 { return true; }
      // Java: else if (c < 2392)
      // Java: return false;
      else if cv < 2392 { return false; }
      // Java: else if (c <= 2401)
      // Java: return true;
      else if cv <= 2401 { return true; }
      // Java: else if (c < 2437)
      // Java: return false;
      else if cv < 2437 { return false; }
      // Java: else if (c <= 2444)
      // Java: return true;
      else if cv <= 2444 { return true; }
      // Java: else if (c < 2447)
      // Java: return false;
      else if cv < 2447 { return false; }
      // Java: else if (c <= 2448)
      // Java: return true;
      else if cv <= 2448 { return true; }
      // Java: else if (c < 2451)
      // Java: return false;
      else if cv < 2451 { return false; }
      // Java: else if (c <= 2472)
      // Java: return true;
      else if cv <= 2472 { return true; }
      // Java: else if (c < 2474)
      // Java: return false;
      else if cv < 2474 { return false; }
      // Java: else if (c <= 2480)
      // Java: return true;
      else if cv <= 2480 { return true; }
      // Java: else if (c == 2482)
      // Java: return true;
      else if cv == 2482 { return true; }
      // Java: else if (c < 2486)
      // Java: return false;
      else if cv < 2486 { return false; }
      // Java: else if (c <= 2489)
      // Java: return true;
      else if cv <= 2489 { return true; }
      // Java: else if (c < 2524)
      // Java: return false;
      else if cv < 2524 { return false; }
      // Java: else if (c <= 2525)
      // Java: return true;
      else if cv <= 2525 { return true; }
      // Java: else if (c < 2527)
      // Java: return false;
      else if cv < 2527 { return false; }
      // Java: else if (c <= 2529)
      // Java: return true;
      else if cv <= 2529 { return true; }
      // Java: else if (c < 2544)
      // Java: return false;
      else if cv < 2544 { return false; }
      // Java: else if (c <= 2545)
      // Java: return true;
      else if cv <= 2545 { return true; }
      // Java: else if (c < 2565)
      // Java: return false;
      else if cv < 2565 { return false; }
      // Java: else if (c <= 2570)
      // Java: return true;
      else if cv <= 2570 { return true; }
      // Java: else if (c < 2575)
      // Java: return false;
      else if cv < 2575 { return false; }
      // Java: else if (c <= 2576)
      // Java: return true;
      else if cv <= 2576 { return true; }
      // Java: else if (c < 2579)
      // Java: return false;
      else if cv < 2579 { return false; }
      // Java: else if (c <= 2600)
      // Java: return true;
      else if cv <= 2600 { return true; }
      // Java: else if (c < 2602)
      // Java: return false;
      else if cv < 2602 { return false; }
      // Java: else if (c <= 2608)
      // Java: return true;
      else if cv <= 2608 { return true; }
      // Java: else if (c < 2610)
      // Java: return false;
      else if cv < 2610 { return false; }
      // Java: else if (c <= 2611)
      // Java: return true;
      else if cv <= 2611 { return true; }
      // Java: else if (c < 2613)
      // Java: return false;
      else if cv < 2613 { return false; }
      // Java: else if (c <= 2614)
      // Java: return true;
      else if cv <= 2614 { return true; }
      // Java: else if (c < 2616)
      // Java: return false;
      else if cv < 2616 { return false; }
      // Java: else if (c <= 2617)
      // Java: return true;
      else if cv <= 2617 { return true; }
      // Java: else if (c < 2649)
      // Java: return false;
      else if cv < 2649 { return false; }
      // Java: else if (c <= 2652)
      // Java: return true;
      else if cv <= 2652 { return true; }
      // Java: else if (c == 2654)
      // Java: return true;
      else if cv == 2654 { return true; }
      // Java: else if (c < 2674)
      // Java: return false;
      else if cv < 2674 { return false; }
      // Java: else if (c <= 2676)
      // Java: return true;
      else if cv <= 2676 { return true; }
      // Java: else if (c < 2693)
      // Java: return false;
      else if cv < 2693 { return false; }
      // Java: else if (c <= 2699)
      // Java: return true;
      else if cv <= 2699 { return true; }
      // Java: else if (c == 2701)
      // Java: return true;
      else if cv == 2701 { return true; }
      // Java: else if (c < 2703)
      // Java: return false;
      else if cv < 2703 { return false; }
      // Java: else if (c <= 2705)
      // Java: return true;
      else if cv <= 2705 { return true; }
      // Java: else if (c < 2707)
      // Java: return false;
      else if cv < 2707 { return false; }
      // Java: else if (c <= 2728)
      // Java: return true;
      else if cv <= 2728 { return true; }
      // Java: else if (c < 2730)
      // Java: return false;
      else if cv < 2730 { return false; }
      // Java: else if (c <= 2736)
      // Java: return true;
      else if cv <= 2736 { return true; }
      // Java: else if (c < 2738)
      // Java: return false;
      else if cv < 2738 { return false; }
      // Java: else if (c <= 2739)
      // Java: return true;
      else if cv <= 2739 { return true; }
      // Java: else if (c < 2741)
      // Java: return false;
      else if cv < 2741 { return false; }
      // Java: else if (c <= 2745)
      // Java: return true;
      else if cv <= 2745 { return true; }
      // Java: else if (c == 2749)
      // Java: return true;
      else if cv == 2749 { return true; }
      // Java: else if (c == 2784)
      // Java: return true;
      else if cv == 2784 { return true; }
      // Java: else if (c < 2821)
      // Java: return false;
      else if cv < 2821 { return false; }
      // Java: else if (c <= 2828)
      // Java: return true;
      else if cv <= 2828 { return true; }
      // Java: else if (c < 2831)
      // Java: return false;
      else if cv < 2831 { return false; }
      // Java: else if (c <= 2832)
      // Java: return true;
      else if cv <= 2832 { return true; }
      // Java: else if (c < 2835)
      // Java: return false;
      else if cv < 2835 { return false; }
      // Java: else if (c <= 2856)
      // Java: return true;
      else if cv <= 2856 { return true; }
      // Java: else if (c < 2858)
      // Java: return false;
      else if cv < 2858 { return false; }
      // Java: else if (c <= 2864)
      // Java: return true;
      else if cv <= 2864 { return true; }
      // Java: else if (c < 2866)
      // Java: return false;
      else if cv < 2866 { return false; }
      // Java: else if (c <= 2867)
      // Java: return true;
      else if cv <= 2867 { return true; }
      // Java: else if (c < 2870)
      // Java: return false;
      else if cv < 2870 { return false; }
      // Java: else if (c <= 2873)
      // Java: return true;
      else if cv <= 2873 { return true; }
      // Java: else if (c == 2877)
      // Java: return true;
      else if cv == 2877 { return true; }
      // Java: else if (c < 2908)
      // Java: return false;
      else if cv < 2908 { return false; }
      // Java: else if (c <= 2909)
      // Java: return true;
      else if cv <= 2909 { return true; }
      // Java: else if (c < 2911)
      // Java: return false;
      else if cv < 2911 { return false; }
      // Java: else if (c <= 2913)
      // Java: return true;
      else if cv <= 2913 { return true; }
      // Java: else if (c < 2949)
      // Java: return false;
      else if cv < 2949 { return false; }
      // Java: else if (c <= 2954)
      // Java: return true;
      else if cv <= 2954 { return true; }
      // Java: else if (c < 2958)
      // Java: return false;
      else if cv < 2958 { return false; }
      // Java: else if (c <= 2960)
      // Java: return true;
      else if cv <= 2960 { return true; }
      // Java: else if (c < 2962)
      // Java: return false;
      else if cv < 2962 { return false; }
      // Java: else if (c <= 2965)
      // Java: return true;
      else if cv <= 2965 { return true; }
      // Java: else if (c < 2969)
      // Java: return false;
      else if cv < 2969 { return false; }
      // Java: else if (c <= 2970)
      // Java: return true;
      else if cv <= 2970 { return true; }
      // Java: else if (c == 2972)
      // Java: return true;
      else if cv == 2972 { return true; }
      // Java: else if (c < 2974)
      // Java: return false;
      else if cv < 2974 { return false; }
      // Java: else if (c <= 2975)
      // Java: return true;
      else if cv <= 2975 { return true; }
      // Java: else if (c < 2979)
      // Java: return false;
      else if cv < 2979 { return false; }
      // Java: else if (c <= 2980)
      // Java: return true;
      else if cv <= 2980 { return true; }
      // Java: else if (c < 2984)
      // Java: return false;
      else if cv < 2984 { return false; }
      // Java: else if (c <= 2986)
      // Java: return true;
      else if cv <= 2986 { return true; }
      // Java: else if (c < 2990)
      // Java: return false;
      else if cv < 2990 { return false; }
      // Java: else if (c <= 2997)
      // Java: return true;
      else if cv <= 2997 { return true; }
      // Java: else if (c < 2999)
      // Java: return false;
      else if cv < 2999 { return false; }
      // Java: else if (c <= 3001)
      // Java: return true;
      else if cv <= 3001 { return true; }
      // Java: else if (c < 3077)
      // Java: return false;
      else if cv < 3077 { return false; }
      // Java: else if (c <= 3084)
      // Java: return true;
      else if cv <= 3084 { return true; }
      // Java: else if (c < 3086)
      // Java: return false;
      else if cv < 3086 { return false; }
      // Java: else if (c <= 3088)
      // Java: return true;
      else if cv <= 3088 { return true; }
      // Java: else if (c < 3090)
      // Java: return false;
      else if cv < 3090 { return false; }
      // Java: else if (c <= 3112)
      // Java: return true;
      else if cv <= 3112 { return true; }
      // Java: else if (c < 3114)
      // Java: return false;
      else if cv < 3114 { return false; }
      // Java: else if (c <= 3123)
      // Java: return true;
      else if cv <= 3123 { return true; }
      // Java: else if (c < 3125)
      // Java: return false;
      else if cv < 3125 { return false; }
      // Java: else if (c <= 3129)
      // Java: return true;
      else if cv <= 3129 { return true; }
      // Java: else if (c < 3168)
      // Java: return false;
      else if cv < 3168 { return false; }
      // Java: else if (c <= 3169)
      // Java: return true;
      else if cv <= 3169 { return true; }
      // Java: else if (c < 3205)
      // Java: return false;
      else if cv < 3205 { return false; }
      // Java: else if (c <= 3212)
      // Java: return true;
      else if cv <= 3212 { return true; }
      // Java: else if (c < 3214)
      // Java: return false;
      else if cv < 3214 { return false; }
      // Java: else if (c <= 3216)
      // Java: return true;
      else if cv <= 3216 { return true; }
      // Java: else if (c < 3218)
      // Java: return false;
      else if cv < 3218 { return false; }
      // Java: else if (c <= 3240)
      // Java: return true;
      else if cv <= 3240 { return true; }
      // Java: else if (c < 3242)
      // Java: return false;
      else if cv < 3242 { return false; }
      // Java: else if (c <= 3251)
      // Java: return true;
      else if cv <= 3251 { return true; }
      // Java: else if (c < 3253)
      // Java: return false;
      else if cv < 3253 { return false; }
      // Java: else if (c <= 3257)
      // Java: return true;
      else if cv <= 3257 { return true; }
      // Java: else if (c == 3294)
      // Java: return true;
      else if cv == 3294 { return true; }
      // Java: else if (c < 3296)
      // Java: return false;
      else if cv < 3296 { return false; }
      // Java: else if (c <= 3297)
      // Java: return true;
      else if cv <= 3297 { return true; }
      // Java: else if (c < 3333)
      // Java: return false;
      else if cv < 3333 { return false; }
      // Java: else if (c <= 3340)
      // Java: return true;
      else if cv <= 3340 { return true; }
      // Java: else if (c < 3342)
      // Java: return false;
      else if cv < 3342 { return false; }
      // Java: else if (c <= 3344)
      // Java: return true;
      else if cv <= 3344 { return true; }
      // Java: else if (c < 3346)
      // Java: return false;
      else if cv < 3346 { return false; }
      // Java: else if (c <= 3368)
      // Java: return true;
      else if cv <= 3368 { return true; }
      // Java: else if (c < 3370)
      // Java: return false;
      else if cv < 3370 { return false; }
      // Java: else if (c <= 3385)
      // Java: return true;
      else if cv <= 3385 { return true; }
      // Java: else if (c < 3424)
      // Java: return false;
      else if cv < 3424 { return false; }
      // Java: else if (c <= 3425)
      // Java: return true;
      else if cv <= 3425 { return true; }
      // Java: else if (c < 3585)
      // Java: return false;
      else if cv < 3585 { return false; }
      // Java: else if (c <= 3630)
      // Java: return true;
      else if cv <= 3630 { return true; }
      // Java: else if (c == 3632)
      // Java: return true;
      else if cv == 3632 { return true; }
      // Java: else if (c < 3634)
      // Java: return false;
      else if cv < 3634 { return false; }
      // Java: else if (c <= 3635)
      // Java: return true;
      else if cv <= 3635 { return true; }
      // Java: else if (c < 3648)
      // Java: return false;
      else if cv < 3648 { return false; }
      // Java: else if (c <= 3653)
      // Java: return true;
      else if cv <= 3653 { return true; }
      // Java: else if (c < 3713)
      // Java: return false;
      else if cv < 3713 { return false; }
      // Java: else if (c <= 3714)
      // Java: return true;
      else if cv <= 3714 { return true; }
      // Java: else if (c == 3716)
      // Java: return true;
      else if cv == 3716 { return true; }
      // Java: else if (c < 3719)
      // Java: return false;
      else if cv < 3719 { return false; }
      // Java: else if (c <= 3720)
      // Java: return true;
      else if cv <= 3720 { return true; }
      // Java: else if (c == 3722)
      // Java: return true;
      else if cv == 3722 { return true; }
      // Java: else if (c == 3725)
      // Java: return true;
      else if cv == 3725 { return true; }
      // Java: else if (c < 3732)
      // Java: return false;
      else if cv < 3732 { return false; }
      // Java: else if (c <= 3735)
      // Java: return true;
      else if cv <= 3735 { return true; }
      // Java: else if (c < 3737)
      // Java: return false;
      else if cv < 3737 { return false; }
      // Java: else if (c <= 3743)
      // Java: return true;
      else if cv <= 3743 { return true; }
      // Java: else if (c < 3745)
      // Java: return false;
      else if cv < 3745 { return false; }
      // Java: else if (c <= 3747)
      // Java: return true;
      else if cv <= 3747 { return true; }
      // Java: else if (c == 3749)
      // Java: return true;
      else if cv == 3749 { return true; }
      // Java: else if (c == 3751)
      // Java: return true;
      else if cv == 3751 { return true; }
      // Java: else if (c < 3754)
      // Java: return false;
      else if cv < 3754 { return false; }
      // Java: else if (c <= 3755)
      // Java: return true;
      else if cv <= 3755 { return true; }
      // Java: else if (c < 3757)
      // Java: return false;
      else if cv < 3757 { return false; }
      // Java: else if (c <= 3758)
      // Java: return true;
      else if cv <= 3758 { return true; }
      // Java: else if (c == 3760)
      // Java: return true;
      else if cv == 3760 { return true; }
      // Java: else if (c < 3762)
      // Java: return false;
      else if cv < 3762 { return false; }
      // Java: else if (c <= 3763)
      // Java: return true;
      else if cv <= 3763 { return true; }
      // Java: else if (c == 3773)
      // Java: return true;
      else if cv == 3773 { return true; }
      // Java: else if (c < 3776)
      // Java: return false;
      else if cv < 3776 { return false; }
      // Java: else if (c <= 3780)
      // Java: return true;
      else if cv <= 3780 { return true; }
      // Java: else if (c < 3904)
      // Java: return false;
      else if cv < 3904 { return false; }
      // Java: else if (c <= 3911)
      // Java: return true;
      else if cv <= 3911 { return true; }
      // Java: else if (c < 3913)
      // Java: return false;
      else if cv < 3913 { return false; }
      // Java: else if (c <= 3945)
      // Java: return true;
      else if cv <= 3945 { return true; }
      // Java: else if (c < 4256)
      // Java: return false;
      else if cv < 4256 { return false; }
      // Java: else if (c <= 4293)
      // Java: return true;
      else if cv <= 4293 { return true; }
      // Java: else if (c < 4304)
      // Java: return false;
      else if cv < 4304 { return false; }
      // Java: else if (c <= 4342)
      // Java: return true;
      else if cv <= 4342 { return true; }
      // Java: else if (c == 4352)
      // Java: return true;
      else if cv == 4352 { return true; }
      // Java: else if (c < 4354)
      // Java: return false;
      else if cv < 4354 { return false; }
      // Java: else if (c <= 4355)
      // Java: return true;
      else if cv <= 4355 { return true; }
      // Java: else if (c < 4357)
      // Java: return false;
      else if cv < 4357 { return false; }
      // Java: else if (c <= 4359)
      // Java: return true;
      else if cv <= 4359 { return true; }
      // Java: else if (c == 4361)
      // Java: return true;
      else if cv == 4361 { return true; }
      // Java: else if (c < 4363)
      // Java: return false;
      else if cv < 4363 { return false; }
      // Java: else if (c <= 4364)
      // Java: return true;
      else if cv <= 4364 { return true; }
      // Java: else if (c < 4366)
      // Java: return false;
      else if cv < 4366 { return false; }
      // Java: else if (c <= 4370)
      // Java: return true;
      else if cv <= 4370 { return true; }
      // Java: else if (c == 4412)
      // Java: return true;
      else if cv == 4412 { return true; }
      // Java: else if (c == 4414)
      // Java: return true;
      else if cv == 4414 { return true; }
      // Java: else if (c == 4416)
      // Java: return true;
      else if cv == 4416 { return true; }
      // Java: else if (c == 4428)
      // Java: return true;
      else if cv == 4428 { return true; }
      // Java: else if (c == 4430)
      // Java: return true;
      else if cv == 4430 { return true; }
      // Java: else if (c == 4432)
      // Java: return true;
      else if cv == 4432 { return true; }
      // Java: else if (c < 4436)
      // Java: return false;
      else if cv < 4436 { return false; }
      // Java: else if (c <= 4437)
      // Java: return true;
      else if cv <= 4437 { return true; }
      // Java: else if (c == 4441)
      // Java: return true;
      else if cv == 4441 { return true; }
      // Java: else if (c < 4447)
      // Java: return false;
      else if cv < 4447 { return false; }
      // Java: else if (c <= 4449)
      // Java: return true;
      else if cv <= 4449 { return true; }
      // Java: else if (c == 4451)
      // Java: return true;
      else if cv == 4451 { return true; }
      // Java: else if (c == 4453)
      // Java: return true;
      else if cv == 4453 { return true; }
      // Java: else if (c == 4455)
      // Java: return true;
      else if cv == 4455 { return true; }
      // Java: else if (c == 4457)
      // Java: return true;
      else if cv == 4457 { return true; }
      // Java: else if (c < 4461)
      // Java: return false;
      else if cv < 4461 { return false; }
      // Java: else if (c <= 4462)
      // Java: return true;
      else if cv <= 4462 { return true; }
      // Java: else if (c < 4466)
      // Java: return false;
      else if cv < 4466 { return false; }
      // Java: else if (c <= 4467)
      // Java: return true;
      else if cv <= 4467 { return true; }
      // Java: else if (c == 4469)
      // Java: return true;
      else if cv == 4469 { return true; }
      // Java: else if (c == 4510)
      // Java: return true;
      else if cv == 4510 { return true; }
      // Java: else if (c == 4520)
      // Java: return true;
      else if cv == 4520 { return true; }
      // Java: else if (c == 4523)
      // Java: return true;
      else if cv == 4523 { return true; }
      // Java: else if (c < 4526)
      // Java: return false;
      else if cv < 4526 { return false; }
      // Java: else if (c <= 4527)
      // Java: return true;
      else if cv <= 4527 { return true; }
      // Java: else if (c < 4535)
      // Java: return false;
      else if cv < 4535 { return false; }
      // Java: else if (c <= 4536)
      // Java: return true;
      else if cv <= 4536 { return true; }
      // Java: else if (c == 4538)
      // Java: return true;
      else if cv == 4538 { return true; }
      // Java: else if (c < 4540)
      // Java: return false;
      else if cv < 4540 { return false; }
      // Java: else if (c <= 4546)
      // Java: return true;
      else if cv <= 4546 { return true; }
      // Java: else if (c == 4587)
      // Java: return true;
      else if cv == 4587 { return true; }
      // Java: else if (c == 4592)
      // Java: return true;
      else if cv == 4592 { return true; }
      // Java: else if (c == 4601)
      // Java: return true;
      else if cv == 4601 { return true; }
      // Java: else if (c < 7680)
      // Java: return false;
      else if cv < 7680 { return false; }
      // Java: else if (c <= 7835)
      // Java: return true;
      else if cv <= 7835 { return true; }
      // Java: else if (c < 7840)
      // Java: return false;
      else if cv < 7840 { return false; }
      // Java: else if (c <= 7929)
      // Java: return true;
      else if cv <= 7929 { return true; }
      // Java: else if (c < 7936)
      // Java: return false;
      else if cv < 7936 { return false; }
      // Java: else if (c <= 7957)
      // Java: return true;
      else if cv <= 7957 { return true; }
      // Java: else if (c < 7960)
      // Java: return false;
      else if cv < 7960 { return false; }
      // Java: else if (c <= 7965)
      // Java: return true;
      else if cv <= 7965 { return true; }
      // Java: else if (c < 7968)
      // Java: return false;
      else if cv < 7968 { return false; }
      // Java: else if (c <= 8005)
      // Java: return true;
      else if cv <= 8005 { return true; }
      // Java: else if (c < 8008)
      // Java: return false;
      else if cv < 8008 { return false; }
      // Java: else if (c <= 8013)
      // Java: return true;
      else if cv <= 8013 { return true; }
      // Java: else if (c < 8016)
      // Java: return false;
      else if cv < 8016 { return false; }
      // Java: else if (c <= 8023)
      // Java: return true;
      else if cv <= 8023 { return true; }
      // Java: else if (c == 8025)
      // Java: return true;
      else if cv == 8025 { return true; }
      // Java: else if (c == 8027)
      // Java: return true;
      else if cv == 8027 { return true; }
      // Java: else if (c == 8029)
      // Java: return true;
      else if cv == 8029 { return true; }
      // Java: else if (c < 8031)
      // Java: return false;
      else if cv < 8031 { return false; }
      // Java: else if (c <= 8061)
      // Java: return true;
      else if cv <= 8061 { return true; }
      // Java: else if (c < 8064)
      // Java: return false;
      else if cv < 8064 { return false; }
      // Java: else if (c <= 8116)
      // Java: return true;
      else if cv <= 8116 { return true; }
      // Java: else if (c < 8118)
      // Java: return false;
      else if cv < 8118 { return false; }
      // Java: else if (c <= 8124)
      // Java: return true;
      else if cv <= 8124 { return true; }
      // Java: else if (c == 8126)
      // Java: return true;
      else if cv == 8126 { return true; }
      // Java: else if (c < 8130)
      // Java: return false;
      else if cv < 8130 { return false; }
      // Java: else if (c <= 8132)
      // Java: return true;
      else if cv <= 8132 { return true; }
      // Java: else if (c < 8134)
      // Java: return false;
      else if cv < 8134 { return false; }
      // Java: else if (c <= 8140)
      // Java: return true;
      else if cv <= 8140 { return true; }
      // Java: else if (c < 8144)
      // Java: return false;
      else if cv < 8144 { return false; }
      // Java: else if (c <= 8147)
      // Java: return true;
      else if cv <= 8147 { return true; }
      // Java: else if (c < 8150)
      // Java: return false;
      else if cv < 8150 { return false; }
      // Java: else if (c <= 8155)
      // Java: return true;
      else if cv <= 8155 { return true; }
      // Java: else if (c < 8160)
      // Java: return false;
      else if cv < 8160 { return false; }
      // Java: else if (c <= 8172)
      // Java: return true;
      else if cv <= 8172 { return true; }
      // Java: else if (c < 8178)
      // Java: return false;
      else if cv < 8178 { return false; }
      // Java: else if (c <= 8180)
      // Java: return true;
      else if cv <= 8180 { return true; }
      // Java: else if (c < 8182)
      // Java: return false;
      else if cv < 8182 { return false; }
      // Java: else if (c <= 8188)
      // Java: return true;
      else if cv <= 8188 { return true; }
      // Java: else if (c == 8486)
      // Java: return true;
      else if cv == 8486 { return true; }
      // Java: else if (c < 8490)
      // Java: return false;
      else if cv < 8490 { return false; }
      // Java: else if (c <= 8491)
      // Java: return true;
      else if cv <= 8491 { return true; }
      // Java: else if (c == 8494)
      // Java: return true;
      else if cv == 8494 { return true; }
      // Java: else if (c < 8576)
      // Java: return false;
      else if cv < 8576 { return false; }
      // Java: else if (c <= 8578)
      // Java: return true;
      else if cv <= 8578 { return true; }
      // Java: else if (c == 12295)
      // Java: return true;
      else if cv == 12295 { return true; }
      // Java: else if (c < 12321)
      // Java: return false;
      else if cv < 12321 { return false; }
      // Java: else if (c <= 12329)
      // Java: return true;
      else if cv <= 12329 { return true; }
      // Java: else if (c < 12353)
      // Java: return false;
      else if cv < 12353 { return false; }
      // Java: else if (c <= 12436)
      // Java: return true;
      else if cv <= 12436 { return true; }
      // Java: else if (c < 12449)
      // Java: return false;
      else if cv < 12449 { return false; }
      // Java: else if (c <= 12538)
      // Java: return true;
      else if cv <= 12538 { return true; }
      // Java: else if (c < 12549)
      // Java: return false;
      else if cv < 12549 { return false; }
      // Java: else if (c <= 12588)
      // Java: return true;
      else if cv <= 12588 { return true; }
      // Java: else if (c < 19968)
      // Java: return false;
      else if cv < 19968 { return false; }
      // Java: else if (c <= '龥')
      // Java: return true;
      else if cv <= 0x9FA5 { return true; }
      // Java: else if (c < '가')
      // Java: return false;
      else if cv < 0xAC00 { return false; }
      // Java: else
      // Java: return c <= '힣';
      else { cv <= 0xD7A3 }
    }

    // Java: static boolean isXMLCombiningChar(char c)
    pub fn isXMLCombiningChar(c: char) -> bool {
      let cv = c as u32;
      // Java: if (c < 768)
      // Java: return false;
      if cv < 768 { return false; }
      // Java: else if (c <= 837)
      // Java: return true;
      else if cv <= 837 { return true; }
      // Java: else if (c < 864)
      // Java: return false;
      else if cv < 864 { return false; }
      // Java: else if (c <= 865)
      // Java: return true;
      else if cv <= 865 { return true; }
      // Java: else if (c < 1155)
      // Java: return false;
      else if cv < 1155 { return false; }
      // Java: else if (c <= 1158)
      // Java: return true;
      else if cv <= 1158 { return true; }
      // Java: else if (c < 1425)
      // Java: return false;
      else if cv < 1425 { return false; }
      // Java: else if (c <= 1441)
      // Java: return true;
      else if cv <= 1441 { return true; }
      // Java: else if (c < 1443)
      // Java: return false;
      else if cv < 1443 { return false; }
      // Java: else if (c <= 1465)
      // Java: return true;
      else if cv <= 1465 { return true; }
      // Java: else if (c < 1467)
      // Java: return false;
      else if cv < 1467 { return false; }
      // Java: else if (c <= 1469)
      // Java: return true;
      else if cv <= 1469 { return true; }
      // Java: else if (c == 1471)
      // Java: return true;
      else if cv == 1471 { return true; }
      // Java: else if (c < 1473)
      // Java: return false;
      else if cv < 1473 { return false; }
      // Java: else if (c <= 1474)
      // Java: return true;
      else if cv <= 1474 { return true; }
      // Java: else if (c == 1476)
      // Java: return true;
      else if cv == 1476 { return true; }
      // Java: else if (c < 1611)
      // Java: return false;
      else if cv < 1611 { return false; }
      // Java: else if (c <= 1618)
      // Java: return true;
      else if cv <= 1618 { return true; }
      // Java: else if (c == 1648)
      // Java: return true;
      else if cv == 1648 { return true; }
      // Java: else if (c < 1750)
      // Java: return false;
      else if cv < 1750 { return false; }
      // Java: else if (c <= 1756)
      // Java: return true;
      else if cv <= 1756 { return true; }
      // Java: else if (c <= 1759)
      // Java: return true;
      else if cv <= 1759 { return true; }
      // Java: else if (c <= 1764)
      // Java: return true;
      else if cv <= 1764 { return true; }
      // Java: else if (c < 1767)
      // Java: return false;
      else if cv < 1767 { return false; }
      // Java: else if (c <= 1768)
      // Java: return true;
      else if cv <= 1768 { return true; }
      // Java: else if (c < 1770)
      // Java: return false;
      else if cv < 1770 { return false; }
      // Java: else if (c <= 1773)
      // Java: return true;
      else if cv <= 1773 { return true; }
      // Java: else if (c < 2305)
      // Java: return false;
      else if cv < 2305 { return false; }
      // Java: else if (c <= 2307)
      // Java: return true;
      else if cv <= 2307 { return true; }
      // Java: else if (c == 2364)
      // Java: return true;
      else if cv == 2364 { return true; }
      // Java: else if (c < 2366)
      // Java: return false;
      else if cv < 2366 { return false; }
      // Java: else if (c <= 2380)
      // Java: return true;
      else if cv <= 2380 { return true; }
      // Java: else if (c == 2381)
      // Java: return true;
      else if cv == 2381 { return true; }
      // Java: else if (c < 2385)
      // Java: return false;
      else if cv < 2385 { return false; }
      // Java: else if (c <= 2388)
      // Java: return true;
      else if cv <= 2388 { return true; }
      // Java: else if (c < 2402)
      // Java: return false;
      else if cv < 2402 { return false; }
      // Java: else if (c <= 2403)
      // Java: return true;
      else if cv <= 2403 { return true; }
      // Java: else if (c < 2433)
      // Java: return false;
      else if cv < 2433 { return false; }
      // Java: else if (c <= 2435)
      // Java: return true;
      else if cv <= 2435 { return true; }
      // Java: else if (c == 2492)
      // Java: return true;
      else if cv == 2492 { return true; }
      // Java: else if (c == 2494)
      // Java: return true;
      else if cv == 2494 { return true; }
      // Java: else if (c == 2495)
      // Java: return true;
      else if cv == 2495 { return true; }
      // Java: else if (c < 2496)
      // Java: return false;
      else if cv < 2496 { return false; }
      // Java: else if (c <= 2500)
      // Java: return true;
      else if cv <= 2500 { return true; }
      // Java: else if (c < 2503)
      // Java: return false;
      else if cv < 2503 { return false; }
      // Java: else if (c <= 2504)
      // Java: return true;
      else if cv <= 2504 { return true; }
      // Java: else if (c < 2507)
      // Java: return false;
      else if cv < 2507 { return false; }
      // Java: else if (c <= 2509)
      // Java: return true;
      else if cv <= 2509 { return true; }
      // Java: else if (c == 2519)
      // Java: return true;
      else if cv == 2519 { return true; }
      // Java: else if (c < 2530)
      // Java: return false;
      else if cv < 2530 { return false; }
      // Java: else if (c <= 2531)
      // Java: return true;
      else if cv <= 2531 { return true; }
      // Java: else if (c == 2562)
      // Java: return true;
      else if cv == 2562 { return true; }
      // Java: else if (c == 2620)
      // Java: return true;
      else if cv == 2620 { return true; }
      // Java: else if (c == 2622)
      // Java: return true;
      else if cv == 2622 { return true; }
      // Java: else if (c == 2623)
      // Java: return true;
      else if cv == 2623 { return true; }
      // Java: else if (c < 2624)
      // Java: return false;
      else if cv < 2624 { return false; }
      // Java: else if (c <= 2626)
      // Java: return true;
      else if cv <= 2626 { return true; }
      // Java: else if (c < 2631)
      // Java: return false;
      else if cv < 2631 { return false; }
      // Java: else if (c <= 2632)
      // Java: return true;
      else if cv <= 2632 { return true; }
      // Java: else if (c < 2635)
      // Java: return false;
      else if cv < 2635 { return false; }
      // Java: else if (c <= 2637)
      // Java: return true;
      else if cv <= 2637 { return true; }
      // Java: else if (c < 2672)
      // Java: return false;
      else if cv < 2672 { return false; }
      // Java: else if (c <= 2673)
      // Java: return true;
      else if cv <= 2673 { return true; }
      // Java: else if (c < 2689)
      // Java: return false;
      else if cv < 2689 { return false; }
      // Java: else if (c <= 2691)
      // Java: return true;
      else if cv <= 2691 { return true; }
      // Java: else if (c == 2748)
      // Java: return true;
      else if cv == 2748 { return true; }
      // Java: else if (c < 2750)
      // Java: return false;
      else if cv < 2750 { return false; }
      // Java: else if (c <= 2757)
      // Java: return true;
      else if cv <= 2757 { return true; }
      // Java: else if (c < 2759)
      // Java: return false;
      else if cv < 2759 { return false; }
      // Java: else if (c <= 2761)
      // Java: return true;
      else if cv <= 2761 { return true; }
      // Java: else if (c < 2763)
      // Java: return false;
      else if cv < 2763 { return false; }
      // Java: else if (c <= 2765)
      // Java: return true;
      else if cv <= 2765 { return true; }
      // Java: else if (c < 2817)
      // Java: return false;
      else if cv < 2817 { return false; }
      // Java: else if (c <= 2819)
      // Java: return true;
      else if cv <= 2819 { return true; }
      // Java: else if (c == 2876)
      // Java: return true;
      else if cv == 2876 { return true; }
      // Java: else if (c < 2878)
      // Java: return false;
      else if cv < 2878 { return false; }
      // Java: else if (c <= 2883)
      // Java: return true;
      else if cv <= 2883 { return true; }
      // Java: else if (c < 2887)
      // Java: return false;
      else if cv < 2887 { return false; }
      // Java: else if (c <= 2888)
      // Java: return true;
      else if cv <= 2888 { return true; }
      // Java: else if (c < 2891)
      // Java: return false;
      else if cv < 2891 { return false; }
      // Java: else if (c <= 2893)
      // Java: return true;
      else if cv <= 2893 { return true; }
      // Java: else if (c < 2902)
      // Java: return false;
      else if cv < 2902 { return false; }
      // Java: else if (c <= 2903)
      // Java: return true;
      else if cv <= 2903 { return true; }
      // Java: else if (c < 2946)
      // Java: return false;
      else if cv < 2946 { return false; }
      // Java: else if (c <= 2947)
      // Java: return true;
      else if cv <= 2947 { return true; }
      // Java: else if (c < 3006)
      // Java: return false;
      else if cv < 3006 { return false; }
      // Java: else if (c <= 3010)
      // Java: return true;
      else if cv <= 3010 { return true; }
      // Java: else if (c < 3014)
      // Java: return false;
      else if cv < 3014 { return false; }
      // Java: else if (c <= 3016)
      // Java: return true;
      else if cv <= 3016 { return true; }
      // Java: else if (c < 3018)
      // Java: return false;
      else if cv < 3018 { return false; }
      // Java: else if (c <= 3021)
      // Java: return true;
      else if cv <= 3021 { return true; }
      // Java: else if (c == 3031)
      // Java: return true;
      else if cv == 3031 { return true; }
      // Java: else if (c < 3073)
      // Java: return false;
      else if cv < 3073 { return false; }
      // Java: else if (c <= 3075)
      // Java: return true;
      else if cv <= 3075 { return true; }
      // Java: else if (c < 3134)
      // Java: return false;
      else if cv < 3134 { return false; }
      // Java: else if (c <= 3140)
      // Java: return true;
      else if cv <= 3140 { return true; }
      // Java: else if (c < 3142)
      // Java: return false;
      else if cv < 3142 { return false; }
      // Java: else if (c <= 3144)
      // Java: return true;
      else if cv <= 3144 { return true; }
      // Java: else if (c < 3146)
      // Java: return false;
      else if cv < 3146 { return false; }
      // Java: else if (c <= 3149)
      // Java: return true;
      else if cv <= 3149 { return true; }
      // Java: else if (c < 3157)
      // Java: return false;
      else if cv < 3157 { return false; }
      // Java: else if (c <= 3158)
      // Java: return true;
      else if cv <= 3158 { return true; }
      // Java: else if (c < 3202)
      // Java: return false;
      else if cv < 3202 { return false; }
      // Java: else if (c <= 3203)
      // Java: return true;
      else if cv <= 3203 { return true; }
      // Java: else if (c < 3262)
      // Java: return false;
      else if cv < 3262 { return false; }
      // Java: else if (c <= 3268)
      // Java: return true;
      else if cv <= 3268 { return true; }
      // Java: else if (c < 3270)
      // Java: return false;
      else if cv < 3270 { return false; }
      // Java: else if (c <= 3272)
      // Java: return true;
      else if cv <= 3272 { return true; }
      // Java: else if (c < 3274)
      // Java: return false;
      else if cv < 3274 { return false; }
      // Java: else if (c <= 3277)
      // Java: return true;
      else if cv <= 3277 { return true; }
      // Java: else if (c < 3285)
      // Java: return false;
      else if cv < 3285 { return false; }
      // Java: else if (c <= 3286)
      // Java: return true;
      else if cv <= 3286 { return true; }
      // Java: else if (c < 3330)
      // Java: return false;
      else if cv < 3330 { return false; }
      // Java: else if (c <= 3331)
      // Java: return true;
      else if cv <= 3331 { return true; }
      // Java: else if (c < 3390)
      // Java: return false;
      else if cv < 3390 { return false; }
      // Java: else if (c <= 3395)
      // Java: return true;
      else if cv <= 3395 { return true; }
      // Java: else if (c < 3398)
      // Java: return false;
      else if cv < 3398 { return false; }
      // Java: else if (c <= 3400)
      // Java: return true;
      else if cv <= 3400 { return true; }
      // Java: else if (c < 3402)
      // Java: return false;
      else if cv < 3402 { return false; }
      // Java: else if (c <= 3405)
      // Java: return true;
      else if cv <= 3405 { return true; }
      // Java: else if (c == 3415)
      // Java: return true;
      else if cv == 3415 { return true; }
      // Java: else if (c == 3633)
      // Java: return true;
      else if cv == 3633 { return true; }
      // Java: else if (c < 3636)
      // Java: return false;
      else if cv < 3636 { return false; }
      // Java: else if (c <= 3642)
      // Java: return true;
      else if cv <= 3642 { return true; }
      // Java: else if (c < 3655)
      // Java: return false;
      else if cv < 3655 { return false; }
      // Java: else if (c <= 3662)
      // Java: return true;
      else if cv <= 3662 { return true; }
      // Java: else if (c == 3761)
      // Java: return true;
      else if cv == 3761 { return true; }
      // Java: else if (c < 3764)
      // Java: return false;
      else if cv < 3764 { return false; }
      // Java: else if (c <= 3769)
      // Java: return true;
      else if cv <= 3769 { return true; }
      // Java: else if (c < 3771)
      // Java: return false;
      else if cv < 3771 { return false; }
      // Java: else if (c <= 3772)
      // Java: return true;
      else if cv <= 3772 { return true; }
      // Java: else if (c < 3784)
      // Java: return false;
      else if cv < 3784 { return false; }
      // Java: else if (c <= 3789)
      // Java: return true;
      else if cv <= 3789 { return true; }
      // Java: else if (c < 3864)
      // Java: return false;
      else if cv < 3864 { return false; }
      // Java: else if (c <= 3865)
      // Java: return true;
      else if cv <= 3865 { return true; }
      // Java: else if (c == 3893)
      // Java: return true;
      else if cv == 3893 { return true; }
      // Java: else if (c == 3895)
      // Java: return true;
      else if cv == 3895 { return true; }
      // Java: else if (c == 3897)
      // Java: return true;
      else if cv == 3897 { return true; }
      // Java: else if (c == 3902)
      // Java: return true;
      else if cv == 3902 { return true; }
      // Java: else if (c == 3903)
      // Java: return true;
      else if cv == 3903 { return true; }
      // Java: else if (c < 3953)
      // Java: return false;
      else if cv < 3953 { return false; }
      // Java: else if (c <= 3972)
      // Java: return true;
      else if cv <= 3972 { return true; }
      // Java: else if (c < 3974)
      // Java: return false;
      else if cv < 3974 { return false; }
      // Java: else if (c <= 3979)
      // Java: return true;
      else if cv <= 3979 { return true; }
      // Java: else if (c < 3984)
      // Java: return false;
      else if cv < 3984 { return false; }
      // Java: else if (c <= 3989)
      // Java: return true;
      else if cv <= 3989 { return true; }
      // Java: else if (c == 3991)
      // Java: return true;
      else if cv == 3991 { return true; }
      // Java: else if (c < 3993)
      // Java: return false;
      else if cv < 3993 { return false; }
      // Java: else if (c <= 4013)
      // Java: return true;
      else if cv <= 4013 { return true; }
      // Java: else if (c < 4017)
      // Java: return false;
      else if cv < 4017 { return false; }
      // Java: else if (c <= 4023)
      // Java: return true;
      else if cv <= 4023 { return true; }
      // Java: else if (c == 4025)
      // Java: return true;
      else if cv == 4025 { return true; }
      // Java: else if (c < 8400)
      // Java: return false;
      else if cv < 8400 { return false; }
      // Java: else if (c <= 8412)
      // Java: return true;
      else if cv <= 8412 { return true; }
      // Java: else if (c == 8417)
      // Java: return true;
      else if cv == 8417 { return true; }
      // Java: else if (c < 12330)
      // Java: return false;
      else if cv < 12330 { return false; }
      // Java: else if (c <= 12335)
      // Java: return true;
      else if cv <= 12335 { return true; }
      // Java: else if (c == 12441)
      // Java: return true;
      else if cv == 12441 { return true; }
      // Java: else
      // Java: return c == 12442;
      else { cv == 12442 }
    }

    // Java: static boolean isXMLExtender(char c)
    pub fn isXMLExtender(c: char) -> bool {
      let cv = c as u32;
      // Java: if (c < 182)
      // Java: return false;
      if cv < 182 { return false; }
      // Java: else if (c == 183)
      // Java: return true;
      else if cv == 183 { return true; }
      // Java: else if (c == 720)
      // Java: return true;
      else if cv == 720 { return true; }
      // Java: else if (c == 721)
      // Java: return true;
      else if cv == 721 { return true; }
      // Java: else if (c == 903)
      // Java: return true;
      else if cv == 903 { return true; }
      // Java: else if (c == 1600)
      // Java: return true;
      else if cv == 1600 { return true; }
      // Java: else if (c == 3654)
      // Java: return true;
      else if cv == 3654 { return true; }
      // Java: else if (c == 3782)
      // Java: return true;
      else if cv == 3782 { return true; }
      // Java: else if (c == 12293)
      // Java: return true;
      else if cv == 12293 { return true; }
      // Java: else if (c < 12337)
      // Java: return false;
      else if cv < 12337 { return false; }
      // Java: else if (c <= 12341)
      // Java: return true;
      else if cv <= 12341 { return true; }
      // Java: else if (c < 12445)
      // Java: return false;
      else if cv < 12445 { return false; }
      // Java: else if (c <= 12446)
      // Java: return true;
      else if cv <= 12446 { return true; }
      // Java: else if (c < 12540)
      // Java: return false;
      else if cv < 12540 { return false; }
      // Java: else
      // Java: return c <= 12542;
      else { cv <= 12542 }
    }

    // Java: static boolean isXMLDigit(char c)
    pub fn isXMLDigit(c: char) -> bool {
      let cv = c as u32;
      // Java: if (c < '0')
      // Java: return false;
      if cv < 0x30 { return false; }
      // Java: else if (c <= '9')
      // Java: return true;
      else if cv <= 0x39 { return true; }
      // Java: else if (c < 1632)
      // Java: return false;
      else if cv < 1632 { return false; }
      // Java: else if (c <= 1641)
      // Java: return true;
      else if cv <= 1641 { return true; }
      // Java: else if (c < 1776)
      // Java: return false;
      else if cv < 1776 { return false; }
      // Java: else if (c <= 1785)
      // Java: return true;
      else if cv <= 1785 { return true; }
      // Java: else if (c < 2406)
      // Java: return false;
      else if cv < 2406 { return false; }
      // Java: else if (c <= 2415)
      // Java: return true;
      else if cv <= 2415 { return true; }
      // Java: else if (c < 2534)
      // Java: return false;
      else if cv < 2534 { return false; }
      // Java: else if (c <= 2543)
      // Java: return true;
      else if cv <= 2543 { return true; }
      // Java: else if (c < 2662)
      // Java: return false;
      else if cv < 2662 { return false; }
      // Java: else if (c <= 2671)
      // Java: return true;
      else if cv <= 2671 { return true; }
      // Java: else if (c < 2790)
      // Java: return false;
      else if cv < 2790 { return false; }
      // Java: else if (c <= 2799)
      // Java: return true;
      else if cv <= 2799 { return true; }
      // Java: else if (c < 2918)
      // Java: return false;
      else if cv < 2918 { return false; }
      // Java: else if (c <= 2927)
      // Java: return true;
      else if cv <= 2927 { return true; }
      // Java: else if (c < 3047)
      // Java: return false;
      else if cv < 3047 { return false; }
      // Java: else if (c <= 3055)
      // Java: return true;
      else if cv <= 3055 { return true; }
      // Java: else if (c < 3174)
      // Java: return false;
      else if cv < 3174 { return false; }
      // Java: else if (c <= 3183)
      // Java: return true;
      else if cv <= 3183 { return true; }
      // Java: else if (c < 3302)
      // Java: return false;
      else if cv < 3302 { return false; }
      // Java: else if (c <= 3311)
      // Java: return true;
      else if cv <= 3311 { return true; }
      // Java: else if (c < 3430)
      // Java: return false;
      else if cv < 3430 { return false; }
      // Java: else if (c <= 3439)
      // Java: return true;
      else if cv <= 3439 { return true; }
      // Java: else if (c < 3664)
      // Java: return false;
      else if cv < 3664 { return false; }
      // Java: else if (c <= 3673)
      // Java: return true;
      else if cv <= 3673 { return true; }
      // Java: else if (c < 3792)
      // Java: return false;
      else if cv < 3792 { return false; }
      // Java: else if (c <= 3801)
      // Java: return true;
      else if cv <= 3801 { return true; }
      // Java: else if (c < 3872)
      // Java: return false;
      else if cv < 3872 { return false; }
      // Java: else
      // Java: return c <= 3881;
      else { cv <= 3881 }
    }
}
