import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public class EditTransactionTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static void focus(PDFPage api, long page, float x, float y, String expected) {
        check(api.native_OnLButtonDown(page, x, y, 0), "focus " + expected);
        check(expected.equals(api.native_GetText(page)), "focused text " + expected);
    }

    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("editable PDF required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open");
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0, "page");
        PDFPage api = new PDFPage(page.value());
        document.native_setEditStatus(doc.value(), 1);
        check(api.native_SectionRecognize(page.value()), "recognize");
        check(document.native_EditBackup(doc.value(), page.value(), 1), "backup boundary");

        String first = "Editable PDF text one";
        focus(api, page.value(), 80, 700, first);
        check(api.native_SetTextSelectRange(page.value(), first.length(), first.length()) &&
              api.native_OnChars(page.value(), "+", false, first.length() + 1), "append mutation");
        long append = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(append != 0, "append opaque undo handle");
        PDFFormFill.native_redoEdit(append);
        check((first + "+").equals(api.native_GetText(page.value())), "initial redo idempotent");
        PDFFormFill.native_undoEdit(append);
        check(first.equals(api.native_GetText(page.value())), "undo append");
        PDFFormFill.native_redoEdit(append);
        check((first + "+").equals(api.native_GetText(page.value())), "redo append");

        check(api.native_SetTextSelectRange(page.value(), 0, 8) &&
              api.native_OnChars(page.value(), "Changed", false, 7), "replace mutation");
        long replace = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(replace != 0 && replace != append, "replace independent command");
        PDFFormFill.native_undoEdit(replace);
        check((first + "+").equals(api.native_GetText(page.value())), "undo replace");
        PDFFormFill.native_undoEdit(append);
        check(first.equals(api.native_GetText(page.value())), "undo chain");
        PDFFormFill.native_redoEdit(append);
        PDFFormFill.native_redoEdit(replace);
        check(api.native_GetText(page.value()).startsWith("Changed"), "redo chain");

        PDFFormFill.native_undoEdit(replace);
        PDFFormFill.native_undoEdit(append);
        focus(api, page.value(), 80, 672, "Second line for replacement");
        int secondLength = api.native_GetTextLength(page.value());
        int secondStart = api.native_GetEditAllText(page.value())
                .indexOf("Second line for replacement");
        check(secondStart >= 0 &&
              api.native_SetTextSelectRange(page.value(), secondStart,
                                            secondStart + secondLength) &&
              api.native_OnKeyDown(page.value(), 112, 0), "delete object mutation");
        long deletion = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(deletion != 0 && api.native_GetFocusType(page.value()) == 0,
              "delete detaches object");
        PDFFormFill.native_undoEdit(deletion);
        check("Second line for replacement".equals(api.native_GetText(page.value())),
              "undo restores detached object");
        PDFFormFill.native_redoEdit(deletion);
        check(api.native_GetFocusType(page.value()) == 0, "redo detaches object");
        PDFFormFill.native_undoEdit(deletion);

        check(document.native_EditBackup(doc.value(), page.value(), 1), "new cancel boundary");
        focus(api, page.value(), 80, 700, first);
        check(api.native_SetTextSelectRange(page.value(), 0, first.length()) &&
              api.native_OnChars(page.value(), "Temporary", false, 9), "temporary mutation");
        long temporary = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(temporary != 0, "temporary command");
        check(document.native_ClearEditPageData(doc.value(), page.value()), "cancel edit page");
        focus(api, page.value(), 80, 700, first);
        PDFFormFill.native_redoEdit(temporary);
        check(RuntimeProbe.nativeLastErrorCode() == -1005, "cancel invalidates command");

        String beforeReflow = api.native_GetEditAllText(page.value());
        String overflow = " alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu";
        int linesBefore = beforeReflow.split("\\n", -1).length;
        check(api.native_SetTextSelectRange(page.value(), first.length(), first.length()) &&
              api.native_OnChars(page.value(), overflow, false,
                                 first.length() + overflow.length()),
                "uniform paragraph overflow mutation");
        long reflow = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        String afterReflow = api.native_GetEditAllText(page.value());
        check(reflow != 0 && afterReflow != null &&
              afterReflow.split("\\n", -1).length > linesBefore,
                "overflow creates additional layout lines");
        PDFFormFill.native_undoEdit(reflow);
        check(beforeReflow.equals(api.native_GetEditAllText(page.value())),
                "undo removes created reflow objects");
        PDFFormFill.native_redoEdit(reflow);
        check(api.native_GetEditAllText(page.value()).contains("lambda mu"),
                "redo restores reflow text");
        PDFFormFill.native_undoEdit(reflow);
        PDFFormFill.native_deleteTransEdit(reflow);

        PDFFormFill.native_undoEdit(page.value());
        check(RuntimeProbe.nativeLastErrorCode() == -1003, "wrong command type rejected");
        PDFFormFill.native_deleteTransEdit(append);
        PDFFormFill.native_deleteTransEdit(replace);
        PDFFormFill.native_deleteTransEdit(deletion);
        check(api.native_unloadPage2(page.value()) == 0, "close page");
        check(document.native_closePDF(doc.value()) == 0, "close document");
        System.out.println("EDIT_TRANSACTION PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
