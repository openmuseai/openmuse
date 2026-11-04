import android.graphics.RectF;

import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

import java.util.ArrayList;

public final class MixedTextReflowTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static String normalize(String value) {
        return value == null ? "" : value.replace('\n', ' ').replaceAll(" +", " ").trim();
    }

    static boolean diagnosticContains(long page, String font, String text) {
        String[] diagnostics = RuntimeProbe.nativeTextObjectDiagnostics(page);
        if (diagnostics == null) return false;
        for (String value : diagnostics) {
            if (value.contains("fontName=" + font + ",") && value.contains("text=") &&
                value.contains(text)) return true;
        }
        return false;
    }

    static void save(PDFDocument document, NativeHandle doc, NativeHandle page,
                     PDFPage api, String output) {
        check(document.native_syncEditPageObj(doc.value(), page.value(), true), "sync");
        check(api.native_unloadPage2(page.value()) == 0, "unload before save");
        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        check(session != 0 && saver.start(session, output) && saver.run(session, 500) == 100,
              "save");
        saver.destroy(session);
        check(document.native_closePDF(doc.value()) == 0, "close");
    }

    public static void main(String[] args) {
        if (args.length != 2) throw new IllegalArgumentException("input and output PDF required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        NativeHandle page = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open");
        check(document.native_getPage(doc.value(), 0, page) == 0, "page");
        document.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize");
        check(api.native_getTextEditorCount(page.value()) == 1, "one semantic domain");
        check(api.native_OnLButtonDown(page.value(), 80, 700, 0), "focus paragraph");
        String before = api.native_GetEditAllText(page.value());
        check(before != null && before.contains("BOLD BETA") && before.contains("italic epsilon"),
              "logical mixed-style text");
        check(diagnosticContains(page.value(), "Helvetica-Bold", "BOLD BETA") &&
              diagnosticContains(page.value(), "Helvetica-Oblique", "italic epsilon"),
              "source style diagnostics");

        int insertion = before.indexOf("BOLD BETA");
        RectF frameBefore = new RectF();
        check(api.native_getTextEditorRect(page.value(), 0, frameBefore, new float[9], false),
              "read initial editor frame");
        String incremental = "one two three four five six seven eight nine ten eleven twelve ";
        ArrayList<Long> incrementalUndo = new ArrayList<>();
        check(insertion > 0 && api.native_SetTextSelectRange(page.value(), insertion, insertion),
              "incremental insertion caret");
        boolean incrementalOk = true;
        for (int i = 0; i < incremental.length(); i++) {
            int[] selection = api.native_GetTextSelectRange(page.value());
            String unit = incremental.substring(i, i + 1);
            incrementalOk &= selection != null && selection.length == 2 &&
                    api.native_OnChars(page.value(), unit, false, selection[1] + 1);
            long command = RuntimeProbe.nativeTakePendingEditUndo(page.value());
            incrementalOk &= command != 0;
            incrementalUndo.add(command);
        }
        check(incrementalOk, "incremental IME commits");
        RectF frameAfter = new RectF();
        check(api.native_getTextEditorRect(page.value(), 0, frameAfter, new float[9], false) &&
              Math.abs(frameAfter.width() - frameBefore.width()) < 0.1f,
              "incremental commits keep editor width");
        check(api.native_GetEditAllText(page.value()).split("\\n", -1).length >
              before.split("\\n", -1).length, "incremental commits reflow to new lines");
        check(normalize(api.native_GetEditAllText(page.value())).contains(incremental.trim()),
              "incremental commits preserve typed sequence");
        for (int i = incrementalUndo.size() - 1; i >= 0; i--) {
            PDFFormFill.native_undoEdit(incrementalUndo.get(i));
            PDFFormFill.native_deleteTransEdit(incrementalUndo.get(i));
        }
        check(normalize(before).equals(normalize(api.native_GetEditAllText(page.value()))),
              "incremental undo chain restores paragraph");

        String overflow = "one two three four five six seven eight nine ten eleven twelve ";
        check(insertion > 0 && api.native_SetTextSelectRange(page.value(), insertion, insertion) &&
              api.native_OnChars(page.value(), overflow, false, insertion + overflow.length()),
              "mixed-style overflow edit");
        long undo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        String after = api.native_GetEditAllText(page.value());
        check(undo != 0 && after != null &&
              after.split("\\n", -1).length > before.split("\\n", -1).length,
              "overflow creates lines");
        check(normalize(after).contains("one two three four five six") &&
              normalize(after).contains("BOLD BETA") &&
              normalize(after).contains("italic epsilon"), "logical text survives reflow");
        check(diagnosticContains(page.value(), "Helvetica-Bold", "BOLD BETA") &&
              diagnosticContains(page.value(), "Helvetica-Oblique", "italic epsilon"),
              "bold and italic runs survive reflow");

        PDFFormFill.native_undoEdit(undo);
        check(normalize(before).equals(normalize(api.native_GetEditAllText(page.value()))) &&
              before.split("\\n", -1).length ==
                  api.native_GetEditAllText(page.value()).split("\\n", -1).length,
              "atomic undo restores logical text and lines");
        check(diagnosticContains(page.value(), "Helvetica-Bold", "BOLD BETA") &&
              diagnosticContains(page.value(), "Helvetica-Oblique", "italic epsilon"),
              "undo restores styles");
        PDFFormFill.native_redoEdit(undo);
        check(normalize(api.native_GetEditAllText(page.value())).contains("eleven twelve"),
              "atomic redo restores reflow");
        save(document, doc, page, api, args[1]);

        NativeHandle reopened = new NativeHandle();
        NativeHandle reopenedPage = new NativeHandle();
        check(PDFDocument.native_openPDF(args[1], reopened) == 0, "reopen");
        check(document.native_getPage(reopened.value(), 0, reopenedPage) == 0, "reopen page");
        document.native_setEditStatus(reopened.value(), 1);
        PDFPage verify = new PDFPage(reopenedPage.value());
        check(verify.native_SectionRecognize(reopenedPage.value()) &&
              verify.native_OnLButtonDown(reopenedPage.value(), 80, 700, 0), "refocus saved paragraph");
        String saved = verify.native_GetEditAllText(reopenedPage.value());
        check(normalize(saved).contains("eleven twelve") &&
              normalize(saved).contains("BOLD BETA") &&
              normalize(saved).contains("italic epsilon"), "saved logical text");
        check(diagnosticContains(reopenedPage.value(), "Helvetica-Bold", "BOLD BETA") &&
              diagnosticContains(reopenedPage.value(), "Helvetica-Oblique", "italic epsilon"),
              "saved style boundaries");
        verify.native_unloadPage2(reopenedPage.value());
        document.native_closePDF(reopened.value());
        PDFFormFill.native_deleteTransEdit(undo);
        System.out.println("MIXED_TEXT_REFLOW PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
