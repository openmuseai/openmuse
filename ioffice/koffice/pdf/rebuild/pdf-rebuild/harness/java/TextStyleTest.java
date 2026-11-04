import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public class TextStyleTest {
    static int pass;
    static int fail;
    static void check(boolean value, String name) {
        if (value) pass++; else { fail++; System.out.println("FAIL: " + name); }
    }

    static PDFPage loadAndFocus(PDFDocument docApi, long doc, NativeHandle page) {
        check(docApi.native_getPage(doc, 0, page) == 0, "load page");
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize");
        check(api.native_OnLButtonDown(page.value(), 150, 700, 0), "focus text");
        return api;
    }

    public static void main(String[] args) {
        if (args.length != 2) throw new IllegalArgumentException("input and output PDFs required");
        System.loadLibrary("kspfreetext");
        PDFDocument docApi = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open");
        docApi.native_setEditStatus(doc.value(), 1);
        NativeHandle page = new NativeHandle();
        PDFPage api = loadAndFocus(docApi, doc.value(), page);

        float originalSize = api.native_GetFontSize(page.value());
        int originalColor = api.native_GetTextColor(page.value());
        String fontName = api.native_GetFontName(page.value());
        check(originalSize > 0, "read font size");
        check(fontName != null && !fontName.isEmpty(), "read font name");
        check(api.native_SetFontName(page.value(), fontName), "same font is idempotent");
        check(RuntimeProbe.nativeTakePendingEditUndo(page.value()) == 0,
              "same font creates no command");

        float changedSize = originalSize + 4;
        check(api.native_SetFontSize(page.value(), changedSize), "set font size");
        long sizeUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(sizeUndo != 0 && Math.abs(api.native_GetFontSize(page.value()) - changedSize) < 0.01f,
              "font size command");
        PDFFormFill.native_undoEdit(sizeUndo);
        check(Math.abs(api.native_GetFontSize(page.value()) - originalSize) < 0.01f,
              "undo font size");
        PDFFormFill.native_redoEdit(sizeUndo);
        check(Math.abs(api.native_GetFontSize(page.value()) - changedSize) < 0.01f,
              "redo font size");

        int changedColor = 0xff3366cc;
        check(api.native_SetTextColor(page.value(), changedColor), "set text color");
        long colorUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(colorUndo != 0 && api.native_GetTextColor(page.value()) == changedColor,
              "text color command");
        PDFFormFill.native_undoEdit(colorUndo);
        check(api.native_GetTextColor(page.value()) == originalColor, "undo text color");
        PDFFormFill.native_redoEdit(colorUndo);
        check(api.native_GetTextColor(page.value()) == changedColor, "redo text color");

        check(api.native_SetFontName(page.value(), "Courier"), "set standard font");
        long fontUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(fontUndo != 0 && "Courier".equals(api.native_GetFontName(page.value())),
              "font command");
        PDFFormFill.native_undoEdit(fontUndo);
        check(fontName.equals(api.native_GetFontName(page.value())), "undo font");
        PDFFormFill.native_redoEdit(fontUndo);
        check("Courier".equals(api.native_GetFontName(page.value())), "redo font");

        check(!api.native_isTextBold(page.value()), "read regular font weight");
        check(api.native_setTextBold(page.value(), true), "set text bold");
        long boldUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(boldUndo != 0 && api.native_isTextBold(page.value()) &&
              "Courier-Bold".equals(api.native_GetFontName(page.value())),
              "bold font command");
        PDFFormFill.native_undoEdit(boldUndo);
        check(!api.native_isTextBold(page.value()) &&
              "Courier".equals(api.native_GetFontName(page.value())), "undo text bold");

        check(api.native_getAlignment(page.value()) == 0, "read left alignment");
        check(api.native_setAlignment(page.value(), 1), "set center alignment");
        long alignmentUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(alignmentUndo != 0 && api.native_getAlignment(page.value()) == 1,
              "alignment command");
        PDFFormFill.native_undoEdit(alignmentUndo);
        check(api.native_getAlignment(page.value()) == 0, "undo alignment");
        PDFFormFill.native_redoEdit(alignmentUndo);
        check(api.native_getAlignment(page.value()) == 1, "redo alignment");
        check(api.native_setAlignment(page.value(), 2), "set right alignment");
        long rightAlignmentUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(rightAlignmentUndo != 0 && api.native_getAlignment(page.value()) == 2,
              "right alignment command");
        PDFFormFill.native_undoEdit(rightAlignmentUndo);
        check(api.native_getAlignment(page.value()) == 1, "undo right alignment");

        float originalLeading = api.native_getLeading(page.value());
        check(originalLeading > 0, "read leading");
        check(api.native_setLeading(page.value(), 2.0f), "set leading");
        long leadingUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(leadingUndo != 0 && Math.abs(api.native_getLeading(page.value()) - 2.0f) < 0.02f,
              "leading command");
        PDFFormFill.native_undoEdit(leadingUndo);
        check(Math.abs(api.native_getLeading(page.value()) - originalLeading) < 0.02f,
              "undo leading");
        PDFFormFill.native_redoEdit(leadingUndo);
        check(Math.abs(api.native_getLeading(page.value()) - 2.0f) < 0.02f,
              "redo leading");

        check(docApi.native_syncEditPageObj(doc.value(), page.value(), true), "sync style page");
        api.native_unloadPage2(page.value());
        PDFNativeSaver saver = new PDFNativeSaver();
        long save = saver.newSession(doc.value(), false);
        check(save != 0 && saver.start(save, args[1]) && saver.run(save, 500) == 100,
              "save styles");
        saver.destroy(save);
        PDFFormFill.native_deleteTransEdit(leadingUndo);
        PDFFormFill.native_deleteTransEdit(rightAlignmentUndo);
        PDFFormFill.native_deleteTransEdit(alignmentUndo);
        PDFFormFill.native_deleteTransEdit(boldUndo);
        PDFFormFill.native_deleteTransEdit(fontUndo);
        PDFFormFill.native_deleteTransEdit(colorUndo);
        PDFFormFill.native_deleteTransEdit(sizeUndo);
        docApi.native_closePDF(doc.value());

        NativeHandle reopened = new NativeHandle();
        check(PDFDocument.native_openPDF(args[1], reopened) == 0, "reopen styles");
        docApi.native_setEditStatus(reopened.value(), 1);
        NativeHandle verifyPage = new NativeHandle();
        PDFPage verify = loadAndFocus(docApi, reopened.value(), verifyPage);
        check(Math.abs(verify.native_GetFontSize(verifyPage.value()) - changedSize) < 0.01f,
              "saved font size");
        check(verify.native_GetTextColor(verifyPage.value()) == changedColor, "saved text color");
        check("Courier".equals(verify.native_GetFontName(verifyPage.value())), "saved font identity");
        check(verify.native_getAlignment(verifyPage.value()) == 1, "saved alignment");
        check(Math.abs(verify.native_getLeading(verifyPage.value()) - 2.0f) < 0.02f,
              "saved leading");
        verify.native_unloadPage2(verifyPage.value());
        docApi.native_closePDF(reopened.value());

        System.out.println("TEXT_STYLE PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
