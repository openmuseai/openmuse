import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

import java.util.ArrayList;

public final class EditDamageTest {
    static int pass, fail;
    static void check(boolean value, String name) {
        if (value) pass++; else { fail++; System.out.println("FAIL: " + name); }
    }
    static boolean contains(float[] damage, RectF frame) {
        float top = Math.max(frame.top, frame.bottom);
        float bottom = Math.min(frame.top, frame.bottom);
        return damage[0] <= frame.left + .05f && damage[1] <= bottom + .05f
                && damage[2] >= frame.right - .05f && damage[3] >= top - .05f;
    }
    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("editable PDF required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle(), page = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open");
        check(document.native_getPage(doc.value(), 0, page) == 0, "load page");
        document.native_setEditStatus(doc.value(), 1);
        PDFPage api = new PDFPage(page.value());
        check(api.native_SectionRecognize(page.value()), "recognize");
        check(api.native_OnLButtonDown(page.value(), 80, 700, 0), "focus");
        RectF before = new RectF();
        check(api.native_GetFocusObjectRect(page.value(), before), "before frame");
        String text = api.native_GetEditAllText(page.value());
        String overflow = " alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu";
        check(api.native_SetTextSelectRange(page.value(), text.length(), text.length())
                && api.native_OnChars(page.value(), overflow, false, text.length() + overflow.length()),
                "overflow mutation");
        long overflowCommand = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        float[] damage = new float[4];
        check(overflowCommand != 0 && RuntimeProbe.nativeGetEditUndoDamage(overflowCommand, damage),
                "overflow command damage");
        RectF after = new RectF();
        check(api.native_GetFocusObjectRect(page.value(), after), "after frame");
        check(contains(damage, before) && contains(damage, after),
                "damage contains old and new editor frames");
        check(damage[2] - damage[0] < 600 && damage[3] - damage[1] < 300,
                "overflow damage remains editor-local");
        PDFFormFill.native_undoEdit(overflowCommand);
        float[] undoDamage = new float[4];
        check(RuntimeProbe.nativeGetEditUndoDamage(overflowCommand, undoDamage)
                && java.util.Arrays.equals(damage, undoDamage), "undo reuses stable damage");
        PDFFormFill.native_redoEdit(overflowCommand);

        ArrayList<Long> commands = new ArrayList<>();
        long started = System.nanoTime();
        boolean incremental = true;
        for (int i = 0; i < 30; ++i) {
            int[] selection = api.native_GetTextSelectRange(page.value());
            incremental &= selection != null && api.native_OnChars(page.value(), "x", false,
                    selection == null ? 0 : selection[1] + 1);
            long command = RuntimeProbe.nativeTakePendingEditUndo(page.value());
            float[] local = new float[4];
            incremental &= command != 0 && RuntimeProbe.nativeGetEditUndoDamage(command, local)
                    && local[2] > local[0] && local[3] > local[1]
                    && local[2] - local[0] < 600 && local[3] - local[1] < 400;
            commands.add(command);
        }
        long elapsedMs = (System.nanoTime() - started) / 1_000_000L;
        check(incremental, "30 incremental edits expose bounded damage");
        check(elapsedMs < 15_000, "30-edit host pressure under 15s (" + elapsedMs + "ms)");
        for (long command : commands) PDFFormFill.native_deleteTransEdit(command);
        PDFFormFill.native_deleteTransEdit(overflowCommand);
        check(!RuntimeProbe.nativeGetEditUndoDamage(overflowCommand, new float[4]),
                "deleted command invalidates damage token");
        api.native_unloadPage2(page.value());
        document.native_closePDF(doc.value());
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "no handles leaked");
        System.out.println("EDIT_DAMAGE PASS=" + pass + " FAIL=" + fail
                + " ELAPSED_MS=" + elapsedMs);
        if (fail != 0) System.exit(1);
    }
}
