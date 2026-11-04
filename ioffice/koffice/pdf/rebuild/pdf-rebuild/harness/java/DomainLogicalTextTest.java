import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public final class DomainLogicalTextTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static int lineCount(String value) {
        int count = 1;
        for (int i = 0; i < value.length(); i++) if (value.charAt(i) == '\n') count++;
        return count;
    }

    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("real PDF path required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle page = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open paper");
        check(document.native_getPage(doc.value(), 1, page) == 0, "load page 2");
        document.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize");

        check(api.native_OnLButtonDown(page.value(), 90, 690, 0), "focus chapter 1");
        android.graphics.RectF focusRect = new android.graphics.RectF();
        check(api.native_GetFocusObjectRect(page.value(), focusRect) &&
              focusRect.left < 72 && focusRect.right > 523 &&
              focusRect.top > 697 && focusRect.bottom < 579,
                "focus border covers semantic editor domain");
        String atom = api.native_GetText(page.value());
        String domain = api.native_GetEditAllText(page.value());
        check(atom != null && atom.contains("Introduction"), "clicked atom retained for style");
        check(domain != null && domain.startsWith("1. Introduction "),
                "domain starts at chapter heading");
        check(domain != null && domain.contains("\n1.1. Dimensions of Composability "),
                "domain joins next layout line");
        check(domain != null && domain.contains("\n1.2.3. The Coarse-Grained Workaround "),
                "domain joins nested line and leader atoms");
        check(domain != null && lineCount(domain) == 7, "chapter 1 has seven logical lines");
        check(domain != null && api.native_GetEditAllTextLength(page.value()) == domain.length() &&
              domain.length() > atom.length(), "domain UTF-16 length contract");
        int secondLine = api.native_GetRangeIndexAtPos(page.value(), 90, 671);
        check(secondLine > domain.indexOf('\n'), "point hit maps to domain offset on next line");
        check(api.native_SetCaret(page.value(), secondLine), "set domain caret on next line");
        android.graphics.PointF[] caret = api.native_GetCaretPoints(page.value());
        check(caret != null && caret.length == 2 && Math.abs(caret[0].y - 671) < 15,
                "domain caret resolves through run glyph geometry");
        check(api.native_OnLButtonDown(page.value(), 90, 690, 0) &&
              api.native_OnMouseMove(page.value(), 90, 671, 0) &&
              api.native_OnLButtonUp(page.value(), 90, 671),
                "drag selection crosses atom and line boundary");
        int[] selection = api.native_GetTextSelectRange(page.value());
        check(selection != null && selection[1] > domain.indexOf('\n'),
                "cross-line selection uses domain offsets");

        check(api.native_OnLButtonDown(page.value(), 90, 550, 0), "focus chapter 2");
        String chapter2 = api.native_GetEditAllText(page.value());
        check(chapter2 != null && chapter2.startsWith("2. Preliminaries ") &&
              lineCount(chapter2) == 4, "chapter 2 domain has four lines");

        check(api.native_OnLButtonDown(page.value(), 90, 690, 0),
                "refocus chapter 1 for cross-line edit");
        String beforeCrossLine = api.native_GetEditAllText(page.value());
        int newline = beforeCrossLine.indexOf('\n');
        check(newline > 4 && api.native_SetTextSelectRange(page.value(), newline - 3,
                                                           newline + 5) &&
              api.native_OnChars(page.value(), " / ", false, newline),
                "cross-line replacement is accepted atomically");
        long crossLine = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(crossLine != 0, "cross-line edit has one undo command");
        PDFFormFill.native_undoEdit(crossLine);
        check(beforeCrossLine.equals(api.native_GetEditAllText(page.value())),
                "cross-line undo restores logical domain");
        PDFFormFill.native_redoEdit(crossLine);
        check(!beforeCrossLine.equals(api.native_GetEditAllText(page.value())),
                "cross-line redo reapplies logical mutation");
        PDFFormFill.native_undoEdit(crossLine);
        PDFFormFill.native_deleteTransEdit(crossLine);

        check(api.native_unloadPage2(page.value()) == 0, "close page");

        NativeHandle referencePage = new NativeHandle();
        check(document.native_getPage(doc.value(), 82, referencePage) == 0,
                "load reference page 83");
        PDFPage referenceApi = new PDFPage(referencePage.value());
        check(referenceApi.native_SectionRecognize(referencePage.value()),
                "recognize reference domains");
        check(referenceApi.native_OnLButtonDown(referencePage.value(), 80, 750, 0),
                "focus reference label");
        String labelBefore = referenceApi.native_GetEditAllText(referencePage.value());
        check(labelBefore != null && labelBefore.trim().equals("[45]"),
                "reference label is an isolated logical domain");
        check(referenceApi.native_OnLButtonDown(referencePage.value(), 120, 750, 0),
                "focus reference body");
        String bodyBefore = referenceApi.native_GetEditAllText(referencePage.value());
        check(bodyBefore != null && bodyBefore.startsWith("U. A. Acar") &&
              bodyBefore.contains("\nTransactions on Programming Languages"),
                "reference body owns its hanging continuation");
        check(referenceApi.native_SetTextSelectRange(referencePage.value(), 0, 2) &&
              referenceApi.native_OnChars(referencePage.value(), "X.", false, 2),
                "edit reference body boundary");
        String bodyAfter = referenceApi.native_GetEditAllText(referencePage.value());
        check(bodyAfter != null && bodyAfter.startsWith("X. A. Acar"),
                "reference body edit is visible");
        long referenceCommand = RuntimeProbe.nativeTakePendingEditUndo(referencePage.value());
        check(referenceCommand != 0, "reference body edit has one undo command");
        check(referenceApi.native_OnLButtonDown(referencePage.value(), 80, 750, 0) &&
              labelBefore.equals(referenceApi.native_GetEditAllText(referencePage.value())),
                "body edit preserves sibling label");
        PDFFormFill.native_undoEdit(referenceCommand);
        check(referenceApi.native_OnLButtonDown(referencePage.value(), 120, 750, 0) &&
              bodyBefore.equals(referenceApi.native_GetEditAllText(referencePage.value())),
                "reference body undo restores exact text");
        PDFFormFill.native_deleteTransEdit(referenceCommand);
        check(referenceApi.native_unloadPage2(referencePage.value()) == 0,
                "close reference page");
        check(document.native_closePDF(doc.value()) == 0, "close document");
        System.out.println("DOMAIN_LOGICAL_TEXT PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
