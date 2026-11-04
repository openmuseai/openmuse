import android.graphics.RectF;

import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.edit.PDFTextEditor;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFFontMgr;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

import java.io.File;

public final class ExternalFontFallbackTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static String compact(String value) {
        return value == null ? "" : value.replace("\r", "").replace("\n", "");
    }

    static String normalizeWhitespace(String value) {
        return value == null ? "" : value.trim().replaceAll("\\s+", " ");
    }

    public static void main(String[] args) {
        if (args.length != 3) throw new IllegalArgumentException("input font output required");
        System.loadLibrary("kspfreetext");
        PDFTextEditor fontEditor = new PDFTextEditor();
        check(!fontEditor.native_setReplaceDisplayFont("Helvetica", "NotoSansSC-Thin"),
                "mapping requires support declaration");
        fontEditor.native_addSupportReplaceFont("Helvetica");
        fontEditor.native_loadFont(args[1]);
        check(PDFFontMgr.native_loadPrivateFont(args[1]), "load private font");
        check(PDFFontMgr.native_loadPrivateFonts(new File(args[1]).getParent()),
                "load private font directory");
        check(fontEditor.native_setReplaceDisplayFont("Helvetica", "NotoSansSC-Thin"),
                "set explicit replacement mapping");

        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle page = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open");
        check(document.native_getPage(doc.value(), 0, page) == 0, "page");
        document.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize");
        check(api.native_OnLButtonDown(page.value(), 100, 700, 0), "focus");
        String original = api.native_GetEditAllText(page.value());
        String inserted = "新增中文内容用于自动换行新增中文内容";
        int caret = original.indexOf("Regular") + "Regular".length();
        check(caret >= 0 && api.native_SetTextSelectRange(page.value(), caret, caret)
                        && api.native_OnChars(page.value(), inserted, false,
                                             caret + inserted.length()),
                "insert through external fallback");
        String after = api.native_GetEditAllText(page.value());
        check(compact(after).contains(inserted), "external fallback preserves characters");
        boolean fallback = false;
        String[] diagnostics = RuntimeProbe.nativeTextObjectDiagnostics(page.value());
        if (diagnostics != null) for (String value : diagnostics) {
            if (value.contains("fontName=NotoSansSC") && value.contains("text=新增")) {
                fallback = true;
            }
        }
        check(fallback, "external font creates CJK TextObject");
        RectF rect = new RectF();
        check(api.native_getTextEditorRect(page.value(), 0, rect, new float[9], false)
                        && rect.width() > 100,
                "editor remains measurable");
        long undo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(undo != 0, "undo command");
        if (undo != 0) PDFFormFill.native_undoEdit(undo);
        check(normalizeWhitespace(original).equals(
                normalizeWhitespace(api.native_GetEditAllText(page.value()))), "undo");
        if (undo != 0) PDFFormFill.native_redoEdit(undo);
        check(compact(api.native_GetEditAllText(page.value())).contains(inserted), "redo");
        check(document.native_syncEditPageObj(doc.value(), page.value(), true), "sync");
        api.native_unloadPage2(page.value());
        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        check(session != 0 && saver.start(session, args[2]) && saver.run(session, 500) == 100,
                "save");
        saver.destroy(session);
        document.native_closePDF(doc.value());

        NativeHandle reopened = new NativeHandle();
        NativeHandle reopenedPage = new NativeHandle();
        check(PDFDocument.native_openPDF(args[2], reopened) == 0, "reopen");
        check(document.native_getPage(reopened.value(), 0, reopenedPage) == 0, "reopen page");
        document.native_setEditStatus(reopened.value(), 1);
        PDFPage verify = new PDFPage(reopenedPage.value());
        check(verify.native_SectionRecognize(reopenedPage.value())
                        && verify.native_OnLButtonDown(reopenedPage.value(), 100, 700, 0),
                "refocus");
        check(compact(verify.native_GetEditAllText(reopenedPage.value())).contains(inserted),
                "saved external-font text");
        verify.native_unloadPage2(reopenedPage.value());
        document.native_closePDF(reopened.value());
        PDFFormFill.native_deleteTransEdit(undo);
        System.out.println("EXTERNAL_FONT_FALLBACK PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
