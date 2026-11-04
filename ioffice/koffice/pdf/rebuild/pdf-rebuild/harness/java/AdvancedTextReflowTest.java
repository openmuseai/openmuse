import android.graphics.RectF;

import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public final class AdvancedTextReflowTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static int countFormText(long page) {
        String[] values = RuntimeProbe.nativeTextObjectDiagnostics(page);
        int count = 0;
        if (values != null) for (String value : values)
            if (value.contains("formDepth=1")) count++;
        return count;
    }

    static String formText(long page) {
        String[] values = RuntimeProbe.nativeTextObjectDiagnostics(page);
        StringBuilder result = new StringBuilder();
        if (values != null) for (String value : values)
            if (value.contains("formDepth=1")) result.append(value.substring(
                    value.indexOf(",text=") + 6)).append(' ');
        return result.toString();
    }

    static String topLevelText(long page) {
        String[] values = RuntimeProbe.nativeTextObjectDiagnostics(page);
        StringBuilder result = new StringBuilder();
        if (values != null) for (String value : values)
            if (value.contains("formDepth=0")) result.append(value.substring(
                    value.indexOf(",text=") + 6)).append(' ');
        return result.toString();
    }

    static void rotated(String path) {
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle page = new NativeHandle();
        check(PDFDocument.native_openPDF(path, doc) == 0, "rotated open");
        check(document.native_getPage(doc.value(), 0, page) == 0, "rotated page");
        document.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "rotated recognize");
        RectF before = new RectF();
        float[] beforeMatrix = new float[9];
        check(api.native_getTextEditorRect(page.value(), 1, before, beforeMatrix, false),
                "rotated frame");
        check(api.native_OnLButtonDown(page.value(), 300, 420, 0), "rotated focus");
        String original = api.native_GetEditAllText(page.value());
        int end = original == null ? 0 : original.length();
        String added = " one two three four five six seven eight nine ten eleven twelve";
        check(api.native_SetTextSelectRange(page.value(), end, end) &&
                api.native_OnChars(page.value(), added, false, end + added.length()),
                "rotated append");
        long undo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        RectF after = new RectF();
        float[] afterMatrix = new float[9];
        check(api.native_getTextEditorRect(page.value(), 1, after, afterMatrix, false) &&
                api.native_getTextEditorCount(page.value()) == 3, "rotated topology");
        check(Math.abs(beforeMatrix[1] - afterMatrix[1]) < 0.001f &&
                Math.abs(beforeMatrix[3] - afterMatrix[3]) < 0.001f &&
                Math.abs(afterMatrix[1]) > 0.1f, "rotated orientation retained");
        check(after.width() > before.width() * 2f, "rotated frame grows along baseline");
        if (undo != 0) PDFFormFill.native_undoEdit(undo);
        check(undo != 0 && original.equals(api.native_GetEditAllText(page.value())),
                "rotated undo");
        if (undo != 0) PDFFormFill.native_redoEdit(undo);
        check(undo != 0 &&
                api.native_GetEditAllText(page.value()).endsWith("eleven twelve"), "rotated redo");
        PDFFormFill.native_deleteTransEdit(undo);
        api.native_unloadPage2(page.value());
        document.native_closePDF(doc.value());
    }

    static void form(String input, String output) {
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle page = new NativeHandle();
        check(PDFDocument.native_openPDF(input, doc) == 0, "form open");
        check(document.native_separateSharedDataInEditPage(doc.value(), 0), "form detach");
        check(document.native_getPage(doc.value(), 0, page) == 0, "form page");
        document.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "form recognize");
        int objectsBefore = countFormText(page.value());
        RectF before = new RectF();
        check(api.native_getTextEditorRect(page.value(), 0, before, new float[9], false),
                "form frame");
        check(api.native_OnLButtonDown(page.value(), 100, 600, 0),
                "form editor-envelope focus");
        String original = api.native_GetEditAllText(page.value());
        int insertion = original.indexOf("FORM TEXT ALPHA") + "FORM TEXT ALPHA".length();
        String added = " one two three four five six seven eight nine ten eleven twelve";
        check(insertion >= 0 && api.native_SetTextSelectRange(page.value(), insertion, insertion) &&
                api.native_OnChars(page.value(), added, false, insertion + added.length()),
                "form append and reflow");
        long undo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        int objectsAfter = countFormText(page.value());
        RectF after = new RectF();
        check(undo != 0 && objectsAfter > objectsBefore, "new lines remain Form children");
        check(api.native_getTextEditorRect(page.value(), 0, after, new float[9], false) &&
                Math.abs(after.width() - before.width()) < 0.1f && after.bottom < before.bottom,
                "form fixed width and downward growth");
        String neighborText = topLevelText(page.value());
        check(neighborText.contains("TOP") && neighborText.contains("LEVEL") &&
                neighborText.contains("TEXT"), "following editor content survives collision reflow");
        if (undo != 0) PDFFormFill.native_undoEdit(undo);
        RectF undoneFrame = new RectF();
        check(undo != 0 && countFormText(page.value()) == objectsBefore &&
                original.equals(api.native_GetEditAllText(page.value())), "form atomic undo");
        check(api.native_getTextEditorRect(page.value(), 0, undoneFrame, new float[9], false) &&
                Math.abs(undoneFrame.bottom - before.bottom) < 0.1f,
                "form undo restores editor frame");
        if (undo != 0) PDFFormFill.native_redoEdit(undo);
        RectF redoneFrame = new RectF();
        check(undo != 0 && countFormText(page.value()) == objectsAfter,
                "form atomic redo");
        check(api.native_getTextEditorRect(page.value(), 0, redoneFrame, new float[9], false) &&
                Math.abs(redoneFrame.bottom - after.bottom) < 0.1f,
                "form redo restores editor frame");
        check(document.native_syncEditPageObj(doc.value(), page.value(), true), "form sync");
        api.native_unloadPage2(page.value());
        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        check(session != 0 && saver.start(session, output) && saver.run(session, 500) == 100,
                "form save");
        saver.destroy(session);
        document.native_closePDF(doc.value());

        NativeHandle reopened = new NativeHandle();
        NativeHandle reopenedPage = new NativeHandle();
        check(PDFDocument.native_openPDF(output, reopened) == 0, "form reopen");
        check(document.native_getPage(reopened.value(), 0, reopenedPage) == 0, "form reopen page");
        document.native_setEditStatus(reopened.value(), 1);
        PDFPage verify = new PDFPage(reopenedPage.value());
        check(verify.native_SectionRecognize(reopenedPage.value()), "form rerecognize");
        String savedFormText = formText(reopenedPage.value());
        check(countFormText(reopenedPage.value()) == objectsAfter &&
                savedFormText.contains("eleven") && savedFormText.contains("twelve"),
                "created Form lines persist in Form stream");
        verify.native_unloadPage2(reopenedPage.value());
        document.native_closePDF(reopened.value());
        PDFFormFill.native_deleteTransEdit(undo);
    }

    public static void main(String[] args) {
        if (args.length != 3) throw new IllegalArgumentException("rotated, form and output required");
        System.loadLibrary("kspfreetext");
        rotated(args[0]);
        form(args[1], args[2]);
        System.out.println("ADVANCED_TEXT_REFLOW PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
