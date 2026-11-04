import android.graphics.RectF;

import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public final class ComplexTextReflowTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static boolean hasCjk(String value) {
        if (value == null) return false;
        for (int i = 0; i < value.length(); i++) {
            char c = value.charAt(i);
            if (c >= '\u3400' && c <= '\u9fff') return true;
        }
        return false;
    }

    static String textOf(String diagnostic) {
        int at = diagnostic.indexOf(",text=");
        return at < 0 ? "" : diagnostic.substring(at + 6);
    }

    static String compactLogicalText(String value) {
        return value == null ? "" : value.replace("\r", "").replace("\n", "");
    }

    public static void main(String[] args) {
        if (args.length != 2) throw new IllegalArgumentException("input and output required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle page = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open");
        check(document.native_getPage(doc.value(), 0, page) == 0, "page");
        document.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize");
        check(api.native_getTextEditorCount(page.value()) == 1, "single semantic editor");
        check(api.native_OnLButtonDown(page.value(), 100, 700, 0), "focus English run");
        String original = api.native_GetEditAllText(page.value());
        check(original != null && original.contains("中文段落用于字体回退"),
                "domain exposes existing CJK");
        RectF before = new RectF();
        check(api.native_getTextEditorRect(page.value(), 0, before, new float[9], false),
                "frame before");
        String inserted = "新增中文内容用于自动换行新增中文内容";
        int caret = original.indexOf("prefix") + "prefix".length();
        check(caret >= 0 && api.native_SetTextSelectRange(page.value(), caret, caret) &&
                api.native_OnChars(page.value(), inserted, false, caret + inserted.length()),
                "insert CJK into Helvetica run");
        String after = api.native_GetEditAllText(page.value());
        check(compactLogicalText(after).contains(inserted), "logical CJK survives reflow");
        String[] diagnostics = RuntimeProbe.nativeTextObjectDiagnostics(page.value());
        boolean cjkFallback = false;
        boolean cjkInHelvetica = false;
        if (diagnostics != null) for (String value : diagnostics) {
            if (!hasCjk(textOf(value))) continue;
            if (value.contains("fontName=NotoSansSC")) cjkFallback = true;
            if (value.contains("fontName=Helvetica")) cjkInHelvetica = true;
        }
        check(cjkFallback, "CJK assigned to encodable domain font");
        check(!cjkInHelvetica, "CJK not assigned to Helvetica");
        RectF changed = new RectF();
        check(api.native_getTextEditorRect(page.value(), 0, changed, new float[9], false) &&
                Math.abs(changed.width() - before.width()) < 0.1f,
                "editor width remains stable");
        long undo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        if (undo != 0) PDFFormFill.native_undoEdit(undo);
        check(undo != 0 && original.equals(api.native_GetEditAllText(page.value())), "undo");
        if (undo != 0) PDFFormFill.native_redoEdit(undo);
        check(undo != 0 && compactLogicalText(api.native_GetEditAllText(page.value())).contains(inserted),
                "redo");
        check(document.native_syncEditPageObj(doc.value(), page.value(), true), "sync");
        api.native_unloadPage2(page.value());
        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        check(session != 0 && saver.start(session, args[1]) && saver.run(session, 500) == 100,
                "save");
        saver.destroy(session);
        document.native_closePDF(doc.value());
        NativeHandle reopened = new NativeHandle();
        NativeHandle reopenedPage = new NativeHandle();
        check(PDFDocument.native_openPDF(args[1], reopened) == 0, "reopen");
        check(document.native_getPage(reopened.value(), 0, reopenedPage) == 0, "reopen page");
        document.native_setEditStatus(reopened.value(), 1);
        PDFPage verify = new PDFPage(reopenedPage.value());
        check(verify.native_SectionRecognize(reopenedPage.value()) &&
                verify.native_OnLButtonDown(reopenedPage.value(), 100, 700, 0), "refocus");
        String saved = verify.native_GetEditAllText(reopenedPage.value());
        check(compactLogicalText(saved).contains(inserted), "saved CJK text");
        verify.native_unloadPage2(reopenedPage.value());
        document.native_closePDF(reopened.value());
        PDFFormFill.native_deleteTransEdit(undo);
        System.out.println("COMPLEX_TEXT_REFLOW PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
