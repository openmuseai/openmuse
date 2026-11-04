#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class RecordMaps
pub struct RecordMaps {
    // Java field: Map<Integer, MetaFileRecord> emfRecordMap
    // Java field: Map<Integer, MetaFileRecord> wmfRecordMap
    _private: (),
}

impl RecordMaps {
    // Java: RecordMaps()
    pub fn new() -> Self {
        // Java: super();
        Self { _private: () }
    }

    // Java: public static synchronized Map<Integer, MetaFileRecord> getEmfRecordMap()
    pub fn getEmfRecordMap() -> i32 {
        // Java: if (null == emfRecordMap)
        {
            // Java: initEmfRecordMap();
        }
        // Java: return emfRecordMap;
        0
    }

    // Java: public static synchronized Map<Integer, MetaFileRecord> getWmfRecordMap()
    pub fn getWmfRecordMap() -> i32 {
        // Java: if (null == wmfRecordMap)
        {
            // Java: initWmfRecordMap();
        }
        // Java: return wmfRecordMap;
        0
    }

    // Java: private static void initEmfRecordMap()
    pub fn initEmfRecordMap() {
        // Java: emfRecordMap = new HashMap(64);
        // Java: emfRecordMap.put(14, new EOF());
        // Java: emfRecordMap.put(2, new PolyBezier());
        // Java: emfRecordMap.put(3, new Polygon());
        // Java: emfRecordMap.put(4, new Polyline());
        // Java: emfRecordMap.put(5, new PolyBezierTo());
        // Java: emfRecordMap.put(6, new PolylineTo());
        // Java: emfRecordMap.put(7, new PolyPolyline());
        // Java: emfRecordMap.put(8, new PolyPolygon());
        // Java: emfRecordMap.put(9, new SetWindowExtEx());
        // Java: emfRecordMap.put(10, new SetWindowOrgEx());
        // Java: emfRecordMap.put(11, new SetViewportExtEx());
        // Java: emfRecordMap.put(12, new SetViewportOrgEx());
        // Java: emfRecordMap.put(17, new SetMapMode());
        // Java: emfRecordMap.put(18, new SetBkMode());
        // Java: emfRecordMap.put(19, new SetPolyFillMode());
        // Java: emfRecordMap.put(20, new SetROP2());
        // Java: emfRecordMap.put(21, new SetStretchBltMode());
        // Java: emfRecordMap.put(22, new SetTextAlign());
        // Java: emfRecordMap.put(24, new SetTextColor());
        // Java: emfRecordMap.put(25, new SetBkColor());
        // Java: emfRecordMap.put(27, new MoveToEx());
        // Java: emfRecordMap.put(30, new IntersectClipRect());
        // Java: emfRecordMap.put(33, new SaveDC());
        // Java: emfRecordMap.put(34, new RestoreDC());
        // Java: emfRecordMap.put(35, new SetWorldTransform());
        // Java: emfRecordMap.put(36, new ModifyWorldTransform());
        // Java: emfRecordMap.put(37, new SelectObject());
        // Java: emfRecordMap.put(38, new CreatePen());
        // Java: emfRecordMap.put(39, new CreateBrushIndirect());
        // Java: emfRecordMap.put(40, new DeleteObject());
        // Java: emfRecordMap.put(42, new Ellipse());
        // Java: emfRecordMap.put(43, new EMF_Rectangle());
        // Java: emfRecordMap.put(44, new RoundRect());
        // Java: emfRecordMap.put(45, new Arc());
        // Java: emfRecordMap.put(46, new Chord());
        // Java: emfRecordMap.put(47, new Pie());
        // Java: emfRecordMap.put(54, new LineTo());
        // Java: emfRecordMap.put(58, new SetMiterLimit());
        // Java: emfRecordMap.put(59, new BeginPath());
        // Java: emfRecordMap.put(60, new EndPath());
        // Java: emfRecordMap.put(61, new CloseFigure());
        // Java: emfRecordMap.put(62, new FillPath());
        // Java: emfRecordMap.put(63, new StrokeAndFillPath());
        // Java: emfRecordMap.put(64, new StrokePath());
        // Java: emfRecordMap.put(67, new SelectClipPath());
        // Java: emfRecordMap.put(75, new ExtSelectClipRgn());
        // Java: emfRecordMap.put(29, new ExcludeClipRect());
        // Java: emfRecordMap.put(76, new BitBlt());
        // Java: emfRecordMap.put(81, new StretchDIBits());
        // Java: emfRecordMap.put(82, new ExtCreateFontIndirectW());
        // Java: emfRecordMap.put(83, new ExtTextOutA());
        // Java: emfRecordMap.put(84, new ExtTextOutW());
        // Java: emfRecordMap.put(85, new PolyBezier16());
        // Java: emfRecordMap.put(86, new Polygon16());
        // Java: emfRecordMap.put(87, new Polyline16());
        // Java: emfRecordMap.put(88, new PolyBezierTo16());
        // Java: emfRecordMap.put(89, new PolylineTo16());
        // Java: emfRecordMap.put(90, new PolyPolyline16());
        // Java: emfRecordMap.put(8, new PolyPolygon());
        // Java: emfRecordMap.put(91, new PolyPolygon16());
        // Java: emfRecordMap.put(93, new CreateMonoBrush());
        // Java: emfRecordMap.put(94, new CreateDIBPatternBrushPt());
        // Java: emfRecordMap.put(95, new ExtCreatePen());
        // Java: emfRecordMap.put(114, new AlphaBlend());
        // Java: emfRecordMap.put(31, new ScaleViewportExtEx());
        // Java: emfRecordMap.put(77, new StretchBlt());
        // Java: emfRecordMap.put(71, new FillRgn());
        // Java: emfRecordMap.put(68, new AbortPath());
    }

    // Java: private static void initWmfRecordMap()
    pub fn initWmfRecordMap() {
        // Java: wmfRecordMap = new HashMap(64);
        // Java: wmfRecordMap.put(805, new WMF_Polyline());
        // Java: wmfRecordMap.put(259, new Wmf_SetMapMode());
        // Java: wmfRecordMap.put(524, new WMF_SetWindowExt());
        // Java: wmfRecordMap.put(523, new WMF_SetWindowOrg());
        // Java: wmfRecordMap.put(0, new WMF_EOF());
        // Java: wmfRecordMap.put(762, new WMF_CreatePenIndirect());
        // Java: wmfRecordMap.put(301, new WMF_SelectObject());
        // Java: wmfRecordMap.put(247, new WMF_CreatePalette());
        // Java: wmfRecordMap.put(564, new WMF_SelectPalette());
        // Java: wmfRecordMap.put(764, new WMF_CreateBrush_Indirect());
        // Java: wmfRecordMap.put(262, new WMF_SetPolyFillMode());
        // Java: wmfRecordMap.put(804, new WMF_Polygon());
        // Java: wmfRecordMap.put(2071, new WMF_Arc());
        // Java: wmfRecordMap.put(2096, new WMF_Chrod());
        // Java: wmfRecordMap.put(763, new WMF_CreateFont_Indirect());
        // Java: wmfRecordMap.put(505, new WMF_CreatePatternBrush());
        // Java: wmfRecordMap.put(1791, new WMF_CreateRegion());
        // Java: wmfRecordMap.put(2881, new WMF_DibStretchBlt());
        // Java: wmfRecordMap.put(2368, new WMF_DibBitBlt());
        // Java: wmfRecordMap.put(1048, new WMF_Ellipse());
        // Java: wmfRecordMap.put(2610, new WMF_ExtTextOut());
        // Java: wmfRecordMap.put(552, new WMF_FillRegion());
        // Java: wmfRecordMap.put(496, new WMF_DeleteObject());
        // Java: wmfRecordMap.put(302, new WMF_SetTextAlign());
        // Java: wmfRecordMap.put(30, new WMF_SaveDC());
        // Java: wmfRecordMap.put(1336, new WMF_PolyPolygon());
        // Java: wmfRecordMap.put(295, new WMF_RestoreDC());
        // Java: wmfRecordMap.put(1046, new WMF_IntersectClipRect());
        // Java: wmfRecordMap.put(1045, new WMF_ExcludeClipRect());
        // Java: wmfRecordMap.put(531, new WMF_LineTo());
        // Java: wmfRecordMap.put(532, new WMF_MoveTo());
        // Java: wmfRecordMap.put(1565, new WMF_PatBlt());
        // Java: wmfRecordMap.put(2074, new WMF_Pie());
        // Java: wmfRecordMap.put(1051, new WMF_Rectangle());
        // Java: wmfRecordMap.put(1564, new WMF_RoundRect());
        // Java: wmfRecordMap.put(1042, new WMF_ScaleViewportExt());
        // Java: wmfRecordMap.put(1040, new WMF_ScaleWindowExt());
        // Java: wmfRecordMap.put(260, new WMF_SetRop2());
        // Java: wmfRecordMap.put(521, new WMF_SetTextColor());
        // Java: wmfRecordMap.put(526, new WMF_SetViewportExt());
        // Java: wmfRecordMap.put(525, new WMF_SetViewportOrg());
        // Java: wmfRecordMap.put(3907, new WMF_StretchDIB());
        // Java: wmfRecordMap.put(1313, new WMF_TextOut());
        // Java: wmfRecordMap.put(513, new WMF_SetBkColor());
        // Java: wmfRecordMap.put(258, new WMF_SetBkMode());
        // Java: wmfRecordMap.put(322, new WMF_DibCreatePatternBrush());
        // Java: wmfRecordMap.put(1574, new WMF_Escape());
    }
}
