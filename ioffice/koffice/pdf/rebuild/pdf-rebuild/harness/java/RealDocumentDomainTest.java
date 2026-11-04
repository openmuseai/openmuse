import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public final class RealDocumentDomainTest {
    static int pass, fail;
    static void check(boolean value, String name) {
        if (value) pass++; else { fail++; System.out.println("FAIL: " + name); }
    }
    static boolean close(float actual, float expected) {
        return Math.abs(actual - expected) <= 3.6f;
    }

    static final float[][][] ORIGINAL = {{
        {70.866f,741.877f,124.296f,729.163f}, {70.866f,698.017f,524.409f,574.902f},
        {70.866f,557.504f,524.409f,490.375f}, {70.866f,472.977f,524.409f,237.890f},
        {70.866f,220.492f,524.409f,97.377f}, {294.888f,60.045f,300.388f,48.814f}
    }, {
        {87.226f,757.368f,524.409f,634.164f}, {70.866f,616.767f,524.409f,437.665f},
        {70.866f,420.267f,524.409f,278.490f}, {70.866f,261.092f,524.409f,175.301f},
        {70.866f,157.903f,524.409f,146.947f}, {70.866f,129.362f,524.409f,118.406f},
        {294.888f,60.045f,300.388f,48.814f}
    }, {
        {70.866f,742.020f,165.467f,729.072f}, {70.866f,712.984f,529.360f,578.796f},
        {70.866f,553.707f,246.877f,542.751f}, {70.866f,529.375f,526.424f,504.240f},
        {87.366f,495.251f,529.360f,429.130f}, {87.366f,426.941f,527.160f,374.482f},
        {70.866f,365.493f,529.359f,285.710f}, {70.866f,260.622f,202.360f,249.666f},
        {70.866f,218.237f,178.347f,207.149f}, {70.866f,193.839f,527.160f,168.704f},
        {70.866f,159.715f,527.160f,93.594f}, {294.888f,60.287f,300.388f,48.814f}
    }};

    // Original libkwopdf, physical page 9.  The alternating narrow spans are
    // the main/subscript baseline layers of the displayed formula.
    static final float[][] ORIGINAL_FORMULA_PAGE = {
        {70.866135f,757.5005f,332.0491f,744.6435f},
        {70.866135f,728.4645f,527.15955f,607.93744f},
        {70.866135f,582.8494f,184.25415f,571.9704f},
        {70.866135f,558.3303f,529.35944f,451.41043f},
        {70.866135f,428.96838f,171.84615f,417.8804f},
        {70.866135f,405.0654f,210.25089f,393.0974f},
        {210.25089f,399.6886f,233.80518f,391.5805f},
        {237.47675f,405.0654f,524.4094f,393.0974f},
        {70.866135f,391.40338f,527.1595f,379.4354f},
        {70.866135f,377.7414f,343.30527f,365.7734f},
        {343.30527f,372.3646f,347.68655f,364.2565f},
        {348.30258f,377.7414f,523.41943f,365.7734f},
        {70.866135f,364.0794f,527.1594f,324.78738f},
        {87.366135f,322.4114f,421.55713f,311.12543f},
        {87.366135f,308.74942f,433.97614f,297.46344f},
        {87.366135f,295.76938f,149.14214f,283.8014f},
        {149.14215f,290.3926f,154.57835f,282.28452f},
        {155.19435f,295.76938f,440.76532f,283.8014f},
        {70.866135f,275.3074f,529.35944f,222.35338f},
        {70.866135f,209.17741f,462.18008f,197.8914f},
        {213.41959f,188.78139f,223.0886f,177.1984f},
        {223.0886f,183.40463f,227.4699f,175.29652f},
        {228.08589f,188.78139f,238.22423f,177.1984f},
        {238.22423f,183.40463f,242.60553f,175.29652f},
        {243.22154f,188.78139f,266.59042f,177.1984f},
        {266.59042f,183.40463f,270.9717f,175.29652f},
        {271.58768f,188.78139f,281.72607f,177.1984f},
        {281.72604f,183.40463f,286.10733f,175.29652f},
        {286.72336f,188.78139f,316.74844f,177.1984f},
        {316.74847f,183.40463f,321.12976f,175.29652f},
        {324.19025f,188.78139f,336.5567f,177.1984f},
        {336.55664f,183.40463f,340.93793f,175.29652f},
        {341.55392f,188.78139f,352.9145f,177.1984f},
        {352.91452f,183.40463f,357.2958f,175.29652f},
        {360.35626f,188.78139f,372.5797f,177.1984f},
        {372.5797f,183.40463f,376.961f,175.29652f},
        {377.577f,188.78139f,381.856f,177.1984f},
        {510.3514f,188.78139f,524.4094f,177.1984f},
        {70.866135f,165.5914f,527.15955f,153.6234f},
        {70.866135f,151.92941f,349.31897f,139.96141f},
        {349.3189f,146.55258f,354.7551f,138.44447f},
        {355.37115f,151.92941f,369.43652f,140.34642f},
        {369.4365f,146.55258f,374.87268f,138.44447f},
        {375.48868f,151.92941f,527.1594f,139.96141f},
        {70.866135f,138.26743f,474.54416f,126.29943f},
        {474.54413f,132.8906f,479.98032f,124.782486f},
        {480.59634f,138.26743f,516.84143f,126.29943f},
        {87.366135f,113.06839f,466.2392f,101.837395f},
        {294.88776f,60.044594f,300.38776f,48.81359f},
    };

    // Original libkwopdf, physical page 10.  These samples cover the two-row
    // aligned formula, equation number, commuting diagram, proof derivation,
    // dense subscript sequence and footer.  Public editor order differs for
    // a few equation numbers, so the contract compares the geometry set.
    static final float[][] ORIGINAL_2D_FORMULA_SAMPLES = {
        {70.866135f,758.1045f,347.36215f,746.14746f},
        {70.866135f,737.0264f,342.25137f,701.87946f},
        {87.366135f,700.17444f,254.40726f,688.2174f},
        {510.3514f,737.0264f,524.4094f,725.4434f},
        {154.18388f,555.77747f,178.96687f,525.6815f},
        {178.96684f,550.40063f,184.40305f,523.7795f},
        {196.01906f,555.77747f,321.6122f,544.19446f},
        {510.3514f,547.5274f,524.4094f,535.9444f},
        {243.54395f,537.2645f,268.8953f,525.6815f},
        {310.7387f,537.2645f,441.0917f,525.6815f},
        {70.866135f,485.47842f,511.94165f,449.0484f},
        {289.85513f,439.93842f,356.10452f,404.1944f},
        {262.4178f,405.2594f,268.6678f,394.7294f},
        {247.51979f,378.38736f,256.9998f,367.85736f},
        {301.6378f,377.8144f,324.1678f,367.2844f},
        {259.44778f,350.2594f,271.63776f,339.7294f},
        {293.0333f,342.8444f,297.9333f,332.3144f},
        {298.83328f,344.0564f,301.68228f,336.6854f},
        {323.6378f,350.2594f,335.8278f,339.7294f},
        {315.50107f,277.7544f,357.62253f,249.1654f},
        {515.85144f,260.7484f,524.4094f,249.1654f},
        {87.366135f,212.5964f,125.89913f,200.6284f},
        {177.31369f,184.65639f,202.0967f,173.0734f},
        {510.3514f,184.65639f,524.4094f,173.0734f},
        {292.1378f,60.044594f,303.1378f,48.81359f},
    };

    // Original libkwopdf, physical page 83. Each bracketed bibliography
    // number is a narrow sibling editor beside the hanging paragraph body.
    static final float[][] ORIGINAL_REFERENCE_PAGE_SAMPLES = {
        {70.866135f,757.60944f,91.282135f,746.14746f},
        {103.932144f,757.60944f,529.3595f,718.8125f},
        {70.866135f,675.69934f,91.282135f,664.2264f},
        {103.932144f,675.69934f,447.9351f,664.2264f},
        {70.866135f,177.37741f,91.282135f,165.9044f},
        {103.932144f,177.37741f,527.15955f,124.9184f},
        {70.866135f,115.92941f,91.282135f,104.45641f},
        {103.932144f,115.92941f,527.98444f,90.79442f},
        {292.1378f,60.28659f,303.1378f,48.81359f},
    };

    static void testPage(PDFDocument document, long doc, int pageIndex) {
        NativeHandle page = new NativeHandle();
        int pageNumber = pageIndex + 1;
        check(document.native_getPage(doc, pageIndex, page) == 0, "load page " + pageNumber);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize page " + pageNumber);
        float[][] expected = ORIGINAL[pageIndex - 1];
        check(api.native_getTextEditorCount(page.value()) == expected.length,
                "page " + pageNumber + " original topology count=" + expected.length);
        RectF[] actual = new RectF[expected.length];
        for (int i = 0; i < expected.length; i++) {
            RectF rect = new RectF();
            float[] matrix = new float[9];
            check(api.native_getTextEditorRect(page.value(), i, rect, matrix, false),
                    "page " + pageNumber + " domain geometry " + i);
            check(close(rect.left, expected[i][0]) && close(rect.top, expected[i][1]) &&
                  close(rect.right, expected[i][2]) && close(rect.bottom, expected[i][3]),
                    "page " + pageNumber + " oracle domain " + i + " actual=" + rect);
            actual[i] = rect;
        }
        for (int i = 0; i < actual.length; i++) for (int j = i + 1; j < actual.length; j++) {
            float overlapWidth = Math.min(actual[i].right, actual[j].right)
                    - Math.max(actual[i].left, actual[j].left);
            float overlapHeight = Math.min(actual[i].top, actual[j].top)
                    - Math.max(actual[i].bottom, actual[j].bottom);
            check(overlapWidth <= 0.5f || overlapHeight <= 0.5f,
                    "page " + pageNumber + " domains do not overlap " + i + "/" + j);
        }
        if (pageIndex == 1) {
            String[] objects = RuntimeProbe.nativeTextObjectDiagnostics(page.value());
            check(objects != null && objects.length > 2000,
                    "real page retains atom-heavy content stream");
        }
        api.native_unloadPage2(page.value());
    }

    static void testFormulaPage(PDFDocument document, long doc) {
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc, 8, page) == 0, "load formula page 9");
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize formula page 9");
        check(api.native_getTextEditorCount(page.value()) == ORIGINAL_FORMULA_PAGE.length,
                "formula page 9 topology count=" + ORIGINAL_FORMULA_PAGE.length);
        RectF[] actual = new RectF[ORIGINAL_FORMULA_PAGE.length];
        for (int i = 0; i < ORIGINAL_FORMULA_PAGE.length; i++) {
            RectF rect = new RectF();
            check(api.native_getTextEditorRect(page.value(), i, rect, new float[9], false),
                    "formula page 9 geometry " + i);
            float[] expected = ORIGINAL_FORMULA_PAGE[i];
            check(close(rect.left, expected[0]) && close(rect.top, expected[1]) &&
                  close(rect.right, expected[2]) && close(rect.bottom, expected[3]),
                    "formula page 9 oracle domain " + i + " actual=" + rect);
            actual[i] = rect;
        }
        for (int i = 0; i < actual.length; i++) for (int j = i + 1; j < actual.length; j++) {
            float overlapWidth = Math.min(actual[i].right, actual[j].right)
                    - Math.max(actual[i].left, actual[j].left);
            float overlapHeight = Math.min(actual[i].top, actual[j].top)
                    - Math.max(actual[i].bottom, actual[j].bottom);
            check(overlapWidth <= 0.5f || overlapHeight <= 0.5f,
                    "formula page 9 domains do not overlap " + i + "/" + j);
        }
        api.native_unloadPage2(page.value());
    }

    static void testTwoDimensionalFormulaPage(PDFDocument document, long doc) {
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc, 9, page) == 0, "load formula page 10");
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize formula page 10");
        int count = api.native_getTextEditorCount(page.value());
        check(count == 123, "formula page 10 topology count=123 actual=" + count);
        RectF[] actual = new RectF[count];
        for (int i = 0; i < count; i++) {
            actual[i] = new RectF();
            check(api.native_getTextEditorRect(page.value(), i, actual[i], new float[9], false),
                    "formula page 10 geometry " + i);
        }
        for (int sample = 0; sample < ORIGINAL_2D_FORMULA_SAMPLES.length; sample++) {
            float[] expected = ORIGINAL_2D_FORMULA_SAMPLES[sample];
            boolean found = false;
            for (RectF rect : actual) {
                if (close(rect.left, expected[0]) && close(rect.top, expected[1]) &&
                    close(rect.right, expected[2]) && close(rect.bottom, expected[3])) {
                    found = true;
                    break;
                }
            }
            check(found, "formula page 10 geometry-set sample " + sample);
        }
        for (int i = 0; i < actual.length; i++) for (int j = i + 1; j < actual.length; j++) {
            float overlapWidth = Math.min(actual[i].right, actual[j].right)
                    - Math.max(actual[i].left, actual[j].left);
            float overlapHeight = Math.min(actual[i].top, actual[j].top)
                    - Math.max(actual[i].bottom, actual[j].bottom);
            check(overlapWidth <= 0.5f || overlapHeight <= 0.5f,
                    "formula page 10 domains do not overlap " + i + "/" + j);
        }
        api.native_unloadPage2(page.value());
    }

    static void testReferencePage(PDFDocument document, long doc) {
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc, 82, page) == 0, "load reference page 83");
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize reference page 83");
        int count = api.native_getTextEditorCount(page.value());
        check(count == 31, "reference page 83 topology count=31 actual=" + count);
        if (count != 31) {
            api.native_unloadPage2(page.value());
            return;
        }
        RectF[] actual = new RectF[count];
        for (int i = 0; i < count; i++) {
            actual[i] = new RectF();
            check(api.native_getTextEditorRect(page.value(), i, actual[i], new float[9], false),
                    "reference page 83 geometry " + i);
        }
        for (int sample = 0; sample < ORIGINAL_REFERENCE_PAGE_SAMPLES.length; sample++) {
            float[] expected = ORIGINAL_REFERENCE_PAGE_SAMPLES[sample];
            boolean found = false;
            for (RectF rect : actual) {
                if (close(rect.left, expected[0]) && close(rect.top, expected[1]) &&
                    close(rect.right, expected[2]) && close(rect.bottom, expected[3])) {
                    found = true;
                    break;
                }
            }
            check(found, "reference page 83 geometry-set sample " + sample);
        }
        for (int i = 0; i < 30; i += 2) {
            check(actual[i].right < actual[i + 1].left,
                    "reference marker/body are disjoint " + i + "/" + (i + 1));
            check(close(actual[i].top, actual[i + 1].top),
                    "reference marker/body share first baseline " + i + "/" + (i + 1));
        }
        api.native_unloadPage2(page.value());
    }

    static void testReferencePageFamily(PDFDocument document, long doc) {
        int[] expected = {29, 29, 31, 31, 31, 29, 27, 14};
        for (int offset = 0; offset < expected.length; offset++) {
            int pageIndex = 80 + offset;
            NativeHandle page = new NativeHandle();
            check(document.native_getPage(doc, pageIndex, page) == 0,
                    "load reference family page " + (pageIndex + 1));
            PDFPage api = new PDFPage(page.value());
            check(api.native_SectionRecognize(page.value()),
                    "recognize reference family page " + (pageIndex + 1));
            int actual = api.native_getTextEditorCount(page.value());
            check(actual == expected[offset], "reference family page " + (pageIndex + 1)
                    + " topology=" + expected[offset] + " actual=" + actual);
            api.native_unloadPage2(page.value());
        }
    }

    static void testEditorBorderHit(PDFDocument document, long doc) {
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc, 28, page) == 0, "load boundary-hit page 29");
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize boundary-hit page 29");
        // The point is about 0.92pt below the rebuild glyph-union rectangle.
        // Original libkwopdf focuses an editor here through its 5.5pt content
        // border; this guards the same pointer contract without changing the
        // editor geometry returned to Java.
        check(api.native_OnLButtonDown(page.value(), 300f, 500f, 0),
                "page 29 original-compatible 5.5pt editor border hit");
        RectF focus = new RectF();
        check(api.native_GetFocusObjectRect(page.value(), focus) &&
                focus.right > focus.left && focus.top > focus.bottom,
                "page 29 boundary hit has a valid focus frame");
        String text = api.native_GetEditAllText(page.value());
        check(text != null && !text.isEmpty(), "page 29 boundary hit owns editable text");
        check(!api.native_OnLButtonDown(page.value(), 300f, 519f, 0),
                "page 29 point beyond adjacent 5.5pt borders stays unfocused");
        api.native_unloadPage2(page.value());
    }

    static void testRecognitionContextLifecycle(PDFDocument document, long doc) {
        check(document.native_GetSectionRecognizeAlgGen() == 1,
                "recognition generation defaults to one");
        check(document.native_SetSectionRecognizeAlgGen(3),
                "set module recognition generation");
        check(document.native_GetSectionRecognizeAlgGen() == 3,
                "read module recognition generation");
        check(document.native_SetSectionRecognizeAlgGen(1),
                "restore generation one oracle");

        long[] initial = RuntimeProbe.nativeRecognitionContextDiagnostics(doc);
        check(initial != null && initial.length == 8 && initial[0] == 0 &&
                initial[1] == 0 && initial[2] == -1 && initial[5] == 0 &&
                initial[6] == 0 && initial[7] == 0,
                "new document recognition context is empty");

        NativeHandle first = new NativeHandle();
        check(document.native_getPage(doc, 1, first) == 0,
                "load first context page");
        PDFPage firstApi = new PDFPage(first.value());
        check(firstApi.native_SectionRecognize(first.value()),
                "recognize first context page");
        int firstCount = firstApi.native_getTextEditorCount(first.value());
        firstApi.native_unloadPage2(first.value());
        long[] afterFirst = RuntimeProbe.nativeRecognitionContextDiagnostics(doc);
        check(afterFirst != null && afterFirst[0] == 1 && afterFirst[1] == 1 &&
                afterFirst[2] == 1 && afterFirst[3] == firstCount &&
                afterFirst[4] == 1 && afterFirst[5] == 1 &&
                afterFirst[6] == firstCount && afterFirst[7] == 0,
                "context survives first page wrapper recycle");

        NativeHandle second = new NativeHandle();
        check(document.native_getPage(doc, 2, second) == 0,
                "load second context page");
        PDFPage secondApi = new PDFPage(second.value());
        check(secondApi.native_SectionRecognize(second.value()),
                "recognize second context page");
        int secondCount = secondApi.native_getTextEditorCount(second.value());
        secondApi.native_unloadPage2(second.value());
        long[] afterSecond = RuntimeProbe.nativeRecognitionContextDiagnostics(doc);
        check(afterSecond != null && afterSecond[0] == 2 && afterSecond[1] == 2 &&
                afterSecond[2] == 2 && afterSecond[3] == secondCount &&
                afterSecond[5] == 2 &&
                afterSecond[6] == firstCount + secondCount && afterSecond[7] == 0,
                "document context records recognition order without page pointers");
    }

    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("real PDF path required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open real document");
        check(document.native_getPageCount(doc.value()) == 88, "paper page count");
        document.native_setEditStatus(doc.value(), 1);
        testRecognitionContextLifecycle(document, doc.value());
        for (int pageIndex = 1; pageIndex <= 3; pageIndex++) testPage(document, doc.value(), pageIndex);
        testFormulaPage(document, doc.value());
        testTwoDimensionalFormulaPage(document, doc.value());
        testReferencePage(document, doc.value());
        testReferencePageFamily(document, doc.value());
        testEditorBorderHit(document, doc.value());
        check(document.native_closePDF(doc.value()) == 0, "close real document");
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "no handle leak");
        System.out.println("REAL_DOCUMENT_DOMAIN PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
