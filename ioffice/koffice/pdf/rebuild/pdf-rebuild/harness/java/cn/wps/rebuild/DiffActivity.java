// DiffActivity - on-device differential test.
//   mode=rebuilt : loads libkspfreetext.so (rebuilt layer + pdfium), drives it,
//                  creates seed.pdf and freetext_rebuilt.pdf
//   mode=original: loads libkwopdf.so (WPS original), drives the SAME ops via
//                  WPS-named stubs, logs getter values for comparison
// Results: Logcat tag KSPDIFF + files/diff_<mode>.txt (run-as readable).
package cn.wps.rebuild;

import android.app.Activity;
import android.content.Context;
import android.os.Bundle;
import android.util.Log;

import android.graphics.Bitmap;
import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.annot.FreeTextAnnotation;
import cn.wps.moffice.pdf.core.annot.PDFAnnotationEditor;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.shared.PDFModuleMgr;
import cn.wps.moffice.pdf.core.edit.PDFTextEditor;
import cn.wps.moffice.pdf.core.edit.PDFPageEditor;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFFontMgr;
import cn.wps.moffice.pdf.core.select.PDFTextPage;
import cn.wps.pdf.lib.select.TextPage;

import java.io.File;
import java.io.InputStream;
import java.io.StringWriter;
import java.io.FileOutputStream;

public class DiffActivity extends Activity {
    static final String TAG = "KSPDIFF";
    static StringBuilder sb = new StringBuilder();

    static void out(String s) {
        Log.i(TAG, s);
        sb.append(s).append('\n');
    }
    static void check(boolean c, String what) { out((c ? "PASS " : "FAIL ") + what); }

    @Override
    protected void onCreate(Bundle b) {
        super.onCreate(b);
        String mode = getIntent().getStringExtra("mode");
        if (mode == null) mode = "rebuilt";
        sb = new StringBuilder();
        out("RUN_ID=" + getIntent().getStringExtra("runId"));
        try {
            if (mode.equals("paper-original") || mode.equals("paper-rebuilt")) runPaper(mode);
            else if (mode.equals("shape-original")) runShapeOriginal();
            else if (mode.equals("shape-rebuilt")) runShapeRebuilt();
            else if (mode.equals("rtl-original")) runRtlOriginal();
            else if (mode.equals("rtl-rebuilt")) runRtlRebuilt();
            else if (mode.equals("unicode-rebuilt")) runUnicodeRebuilt();
            else if (mode.equals("external-font-original")) runExternalFontOriginal();
            else if (mode.equals("external-font-rebuilt")) runExternalFontRebuilt();
            else if (mode.equals("complex-original")) runComplexOriginal();
            else if (mode.equals("complex-rebuilt")) runComplexRebuilt();
            else if (mode.equals("advanced-original")) runAdvancedOriginal();
            else if (mode.equals("advanced-rebuilt")) runAdvancedRebuilt();
            else if (mode.equals("original")) runOriginal();
            else if (mode.equals("renderdiff")) runRenderDiff();
            else runRebuilt();
        } catch (Throwable t) {
            out("EXC " + t);
            for (StackTraceElement e : t.getStackTrace()) {
                if (sb.length() < 4000) out("  at " + e);
            }
        }
        out("DONE_" + mode);
        try {
            File dir = new File(getExternalFilesDir(null), "");
            dir.mkdirs();
            File f = new File(dir, "diff_" + mode + ".txt");
            try (FileOutputStream fos = new FileOutputStream(f)) {
                fos.write(sb.toString().getBytes("UTF-8"));
            }
            out("WROTE " + f.getAbsolutePath());
        } catch (Exception e) {
            out("write fail " + e);
        }
        finish();
    }

    // -------------------------------------------------------------------
    // renderdiff: render the SAME content PDF via BOTH libs, compare pixels
    // -------------------------------------------------------------------
    void runRenderDiff() throws Exception {
        out("== MODE renderdiff ==");
        System.loadLibrary("kspfreetext");
        final String dir = getExternalFilesDir(null).getAbsolutePath();
        final String pdf = dir + "/diffcontent.pdf";

        // 1) generate content PDF via the rebuilt layer (pdfium draw)
        long doc = TestPdf.nativeNewDoc();
        long page = TestPdf.nativeAddPage(doc, 612, 792);
        check(TestPdf.nativeDrawRect(page, 40, 40, 200, 120, 255, 0, 0), "drawRect");
        check(TestPdf.nativeDrawText(doc, page, 60, 70, "KSP Render Diff 123456", 36, 0, 0, 255), "drawText");
        check(TestPdf.nativeSaveDoc(doc, pdf), "content save");
        float pw = TestPdf.nativeGetPageWidth(page);
        float ph = TestPdf.nativeGetPageHeight(page);
        int w = (int) pw, h = (int) ph;
        out("page size=" + pw + "x" + ph);

        // 2) REBUILT render (pdfium)
        long d2 = TestPdf.nativeOpenPdf(pdf);
        long p2 = TestPdf.nativeGetPage(d2, 0);
        int[] px = new int[w * h];
        check(TestPdf.nativeRenderPage(p2, w, h, px), "rebuilt render");
        Bitmap bmpA = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888);
        bmpA.setPixels(px, 0, w, 0, 0, w, h);
        savePng(bmpA, dir + "/render_rebuilt.png");
        TestPdf.nativeClosePage(p2);
        TestPdf.nativeCloseDoc(d2);

        // 3) ORIGINAL render (libkwopdf via PDFPageRender natives)
        System.loadLibrary("kwopdf");
        int init = new PDFModuleMgr().initialize();
        out("moduleInit=" + init);
        NativeHandle docH = new NativeHandle();
        check(PDFDocument.native_openPDF(pdf, docH) == 0, "orig openPDF");
        long od = docH.value();
        NativeHandle pageH = new NativeHandle();
        PDFDocument pdfDoc = new PDFDocument();
        check(pdfDoc.native_getPage(od, 0, pageH) == 0, "orig getPage");
        long op = pageH.value();
        Bitmap bmpB = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888);
        bmpB.eraseColor(0xFFFFFFFF);
        cn.wps.moffice.pdf.core.std.PDFPage opage = new cn.wps.moffice.pdf.core.std.PDFPage(op);
        try {
            android.graphics.RectF osz = opage.getPageSize();
            out("orig pageSize=" + osz);
            opage.parsePageBlocked();
            out("parse done, loadState=" + cn.wps.moffice.pdf.core.std.PDFPage.native_getLoadState(op));
            cn.wps.rebuild.TextPageProbe.probe("original", pageH.value());
        } catch (Throwable t) {
            out("parse EXC " + t);
        }
        cn.wps.pdf.lib.render.PageRender r = new cn.wps.pdf.lib.render.PageRender(op);
        RectF wr = new RectF(0, 0, w, h);
        int sr = r.start(bmpB, wr, wr);
        out("orig nStart=" + sr + " state=" + r.state());
        int cr = r.continueRender(Integer.MAX_VALUE, 0, bmpB);
        out("orig nContinue=" + cr + " state=" + r.state());
        out("orig probe(100,100)=" + Integer.toHexString(bmpB.getPixel(100, 100))
                + " (300,300)=" + Integer.toHexString(bmpB.getPixel(300, 300))
                + " (400,700)=" + Integer.toHexString(bmpB.getPixel(400, 700)));
        r.dispose();

        // ---- isolation: hand-written minimal PDF (black rect, no fonts) ----
        String manual = dir + "/manual.pdf";
        StringWriter mw = new StringWriter();
        mw.write("%PDF-1.4\n");
        mw.write("1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n");
        mw.write("2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n");
        mw.write("3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R >> endobj\n");
        mw.write("4 0 obj << /Length 45 >> stream\n");
        mw.write("100 500 300 150 re f\n");
        mw.write("endstream endobj\n");
        mw.write("trailer << /Root 1 0 R >>\n");
        mw.write("%%EOF\n");
        java.io.FileOutputStream mf = new java.io.FileOutputStream(manual);
        mf.write(mw.toString().getBytes("UTF-8"));
        mf.close();
        NativeHandle mdocH = new NativeHandle();
        int mo = PDFDocument.native_openPDF(manual, mdocH);
        out("manual openPDF=" + mo);
        if (mo == 0) {
            NativeHandle mpageH = new NativeHandle();
            int mp = pdfDoc.native_getPage(mdocH.value(), 0, mpageH);
            out("manual getPage=" + mp);
            if (mp == 0) {
                cn.wps.moffice.pdf.core.std.PDFPage mpage =
                        new cn.wps.moffice.pdf.core.std.PDFPage(mpageH.value());
                mpage.parsePageBlocked();
                Bitmap mbmp = Bitmap.createBitmap(612, 792, Bitmap.Config.ARGB_8888);
                mbmp.eraseColor(0xFFFFFFFF);
                cn.wps.pdf.lib.render.PageRender mr = new cn.wps.pdf.lib.render.PageRender(mpageH.value());
                RectF mrct = new RectF(0, 0, 612, 792);
                int msr = mr.start(mbmp, mrct, mrct);
                int mcr = mr.continueRender(Integer.MAX_VALUE, 0, mbmp);
                out("manual nStart=" + msr + " nContinue=" + mcr + " state=" + mr.state());
                out("manual probe(150,560)=" + Integer.toHexString(mbmp.getPixel(150, 560))
                        + " (300,300)=" + Integer.toHexString(mbmp.getPixel(300, 300)));
                savePng(mbmp, dir + "/render_manual_original.png");
                mr.dispose();
            }
            pdfDoc.native_closePDF(mdocH.value());
        }
        savePng(bmpB, dir + "/render_original.png");
        pdfDoc.native_closePDF(od);

        // 4) pixel diff
        int same = 0, diff = 0;
        int[] pxB = new int[w * h];
        bmpB.getPixels(pxB, 0, w, 0, 0, w, h);
        Bitmap diffBmp = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888);
        int[] diffPx = new int[w * h];
        for (int i = 0; i < px.length; i++) {
            if (px[i] == pxB[i]) { same++; diffPx[i] = 0xFF808080; }
            else { diff++; diffPx[i] = 0xFFFF0000; }
        }
        diffBmp.setPixels(diffPx, 0, w, 0, 0, w, h);
        savePng(diffBmp, dir + "/render_diff.png");
        out("DIFF same=" + same + " diff=" + diff
                + " rate=" + (100.0 * diff / (same + diff)) + "%");
        out("PNG dir=" + dir);
    }

    void savePng(Bitmap b, String path) throws Exception {
        java.io.FileOutputStream fos = new java.io.FileOutputStream(path);
        b.compress(Bitmap.CompressFormat.PNG, 100, fos);
        fos.close();
        out("saved " + path);
    }

    // -------------------------------------------------------------------
    // rebuilt variant
    // -------------------------------------------------------------------
    void runAdvancedRebuilt() throws Exception {
        out("== MODE advanced-rebuilt ==");
        System.loadLibrary("kspfreetext");
        probeAdvancedTextReflow("rebuilt", new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath());
    }

    void runAdvancedOriginal() throws Exception {
        out("== MODE advanced-original ==");
        System.loadLibrary("kwopdf");
        out("moduleInit=" + new PDFModuleMgr().initialize());
        probeAdvancedTextReflow("original", new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath());
    }

    void runComplexRebuilt() throws Exception {
        out("== MODE complex-rebuilt ==");
        System.loadLibrary("kspfreetext");
        probeComplexTextReflow("rebuilt", new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath());
    }

    void runComplexOriginal() throws Exception {
        out("== MODE complex-original ==");
        System.loadLibrary("kwopdf");
        out("moduleInit=" + new PDFModuleMgr().initialize());
        probeComplexTextReflow("original", new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath());
    }

    void runExternalFontRebuilt() throws Exception {
        out("== MODE external-font-rebuilt ==");
        System.loadLibrary("kspfreetext");
        probeExternalFontFallback("rebuilt", new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath());
    }

    void runExternalFontOriginal() throws Exception {
        out("== MODE external-font-original ==");
        System.loadLibrary("kwopdf");
        out("moduleInit=" + new PDFModuleMgr().initialize());
        probeExternalFontFallback("original", new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath());
    }

    void runUnicodeRebuilt() throws Exception {
        out("== MODE unicode-rebuilt ==");
        System.loadLibrary("kspfreetext");
        probeUnicodeEditBoundary(new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath());
    }

    void runRtlRebuilt() throws Exception {
        out("== MODE rtl-rebuilt ==");
        System.loadLibrary("kspfreetext");
        probeRtlText("rebuilt", new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath(), true);
    }

    void runRtlOriginal() throws Exception {
        out("== MODE rtl-original ==");
        System.loadLibrary("kwopdf");
        out("moduleInit=" + new PDFModuleMgr().initialize());
        probeRtlText("original", new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath(), false);
    }

    void runShapeRebuilt() throws Exception {
        out("== MODE shape-rebuilt ==");
        System.loadLibrary("kspfreetext");
        probeArabicShape("rebuilt", new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath());
    }

    void runShapeOriginal() throws Exception {
        out("== MODE shape-original ==");
        System.loadLibrary("kwopdf");
        out("moduleInit=" + new PDFModuleMgr().initialize());
        probeArabicShape("original", new PDFDocument(),
                getExternalFilesDir(null).getAbsolutePath());
    }

    /** WT6c2b2 oracle: logical Arabic, joining/ligature clusters and save round-trip. */
    void probeArabicShape(String prefix, PDFDocument document, String dir) throws Exception {
        String fontPath = copyAsset("fonts/NotoSansArabic-WT6-subset.ttf",
                dir + "/NotoSansArabic-WT6-subset.ttf");
        boolean fontLoaded = PDFFontMgr.native_loadPrivateFont(fontPath);
        boolean directoryLoaded = PDFFontMgr.native_loadPrivateFonts(dir);
        PDFTextEditor fontEditor = new PDFTextEditor();
        fontEditor.native_addSupportReplaceFont("NotoSansArabic-Regular");
        fontEditor.native_addSupportReplaceFont("Helvetica");
        boolean fontMapped = fontEditor.native_setReplaceDisplayFont(
                "NotoSansArabic-Regular", "Noto Sans Arabic");
        boolean fallbackMapped = fontEditor.native_setReplaceDisplayFont(
                "Helvetica", "NotoSansArabic-Regular");
        out(prefix + "Shape.fontLoaded=" + fontLoaded + ",directory="
                + directoryLoaded + ",mapped=" + fontMapped
                + ",fallback=" + fallbackMapped);
        String path = copyAsset("arabic-shape-corpus.pdf",
                dir + "/arabic-shape-" + prefix + ".pdf");
        String saved = dir + "/arabic-shape-saved-" + prefix + ".pdf";
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(path, doc) == 0, prefix + " shape open");
        if (prefix.equals("rebuilt")) {
            int[] glyphs = RuntimeProbe.nativeShapeReplacement(
                    doc.value(), "Helvetica", "سلام لا", 1);
            out("rebuiltShape.glyphs=" + intArray(glyphs));
            check(glyphs != null && glyphs.length >= 16 && (glyphs.length - 4) % 6 == 0 &&
                            glyphs[0] == 1 && glyphs[5] > glyphs[glyphs.length - 5],
                    "rebuilt HarfBuzz RTL glyph clusters and CID encoding");
            int[] marks = RuntimeProbe.nativeShapeReplacement(
                    doc.value(), "Helvetica", "سَّ", 1);
            out("rebuiltShape.marks=" + intArray(marks));
            boolean duplicateCluster = marks != null && marks.length >= 22 &&
                    marks[5] == marks[11];
            boolean markYOffset = marks != null && marks.length >= 22 &&
                    (marks[9] != 0 || marks[15] != 0 || marks[21] != 0);
            check(marks != null && marks[0] == 1 && marks[1] == 3 &&
                            duplicateCluster && markYOffset,
                    "rebuilt multi-glyph mark cluster encoding oracle");
        }
        document.native_setEditStatus(doc.value(), 1);
        NativeHandle pageHandle = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, pageHandle) == 0,
                prefix + " shape page");
        cn.wps.moffice.pdf.core.std.PDFPage page =
                new cn.wps.moffice.pdf.core.std.PDFPage(pageHandle.value());
        page.parsePageBlocked();
        String extractedBefore = extractPageText(pageHandle.value());
        page.native_onAfterLoadPage(pageHandle.value());
        boolean recognized = page.native_SectionRecognize(pageHandle.value());
        int editorCount = page.native_getTextEditorCount(pageHandle.value());
        out(prefix + "Shape.extractedBefore=" + escaped(extractedBefore)
                + ",recognized=" + recognized + ",editors=" + editorCount);
        for (int i = 0; i < editorCount; i++) {
            RectF rect = new RectF();
            float[] matrix = new float[9];
            boolean ok = page.native_getTextEditorRect(
                    pageHandle.value(), i, rect, matrix, false);
            out(prefix + "Shape.editor[" + i + "]=" + ok + "," + rect
                    + ",m=" + matrix[0] + "," + matrix[1] + "," + matrix[3]
                    + "," + matrix[4]);
        }

        // The source LAM_ALEF object is at (520, 630), with negative character
        // spacing. Probe glyph origins because the old organizer may merge
        // adjacent Arabic objects into one multi-line editor whose bounding
        // box contains large non-hit-testable gaps.
        float[][] candidates = {{520f, 630f}, {490f, 630f}, {520f, 690f},
                {490f, 690f}, {455f, 690f}, {520f, 570f}, {485f, 570f}};
        boolean focused = false;
        for (float[] candidate : candidates) {
            boolean hit = page.native_OnLButtonDown(
                    pageHandle.value(), candidate[0], candidate[1], 0);
            if (hit) page.native_OnLButtonUp(
                    pageHandle.value(), candidate[0], candidate[1]);
            String hitText = hit ? page.native_GetText(pageHandle.value()) : null;
            out(prefix + "Shape.hit[" + candidate[0] + "," + candidate[1]
                    + "]=" + hit + ",text=" + escaped(hitText));
            if (hit && page.native_GetTextLength(pageHandle.value()) > 0) {
                focused = true;
                break;
            }
        }
        String text = focused ? page.native_GetText(pageHandle.value()) : null;
        String all = focused ? page.native_GetEditAllText(pageHandle.value()) : null;
        out(prefix + "Shape.focus=" + focused + ",text=" + escaped(text)
                + ",all=" + escaped(all) + ",length="
                + (focused ? page.native_GetTextLength(pageHandle.value()) : -1));

        if (focused) {
            int length = page.native_GetTextLength(pageHandle.value());
            for (int caret = 0; caret <= length; caret++) {
                boolean set = page.native_SetCaret(pageHandle.value(), caret);
                android.graphics.PointF[] points = page.native_GetCaretPoints(pageHandle.value());
                out(prefix + "Shape.caret[" + caret + "]=" + set + ","
                        + pointArray(points));
            }
            boolean selected = length >= 2 &&
                    page.native_SetTextSelectRange(pageHandle.value(), 1, 1);
            boolean changed = selected &&
                    page.native_OnChars(pageHandle.value(), "ب", false, 2);
            out(prefix + "Shape.insert=" + changed + ",after="
                    + escaped(page.native_GetText(pageHandle.value())) + ",allAfter="
                    + escaped(page.native_GetEditAllText(pageHandle.value())));
            check(!changed || document.native_syncEditPageObj(
                    doc.value(), pageHandle.value(), true), prefix + " shape sync");
        }
        page.native_unloadPage2(pageHandle.value());

        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        boolean savedOk = session != 0 && saver.start(session, saved)
                && saver.run(session, 500) == 100;
        out(prefix + "Shape.saved=" + savedOk + ",path=" + saved);
        if (session != 0) saver.destroy(session);
        document.native_closePDF(doc.value());

        NativeHandle reopened = new NativeHandle();
        int reopenedResult = PDFDocument.native_openPDF(saved, reopened);
        out(prefix + "Shape.reopen=" + reopenedResult);
        if (reopenedResult == 0) {
            NativeHandle reopenedPage = new NativeHandle();
            if (document.native_getPage(reopened.value(), 0, reopenedPage) == 0) {
                cn.wps.moffice.pdf.core.std.PDFPage verify =
                        new cn.wps.moffice.pdf.core.std.PDFPage(reopenedPage.value());
                verify.parsePageBlocked();
                out(prefix + "Shape.extractedAfter="
                        + escaped(extractPageText(reopenedPage.value())));
                verify.native_unloadPage2(reopenedPage.value());
            }
            document.native_closePDF(reopened.value());
        }
        probeArabicInsertion(prefix, document, dir);
    }

    void probeArabicInsertion(String prefix, PDFDocument document, String dir) throws Exception {
        String path = copyAsset("editable-helvetica.pdf",
                dir + "/arabic-insert-" + prefix + ".pdf");
        String saved = dir + "/arabic-insert-saved-" + prefix + ".pdf";
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(path, doc) == 0, prefix + " Arabic insert open");
        document.native_setEditStatus(doc.value(), 1);
        NativeHandle pageHandle = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, pageHandle) == 0,
                prefix + " Arabic insert page");
        cn.wps.moffice.pdf.core.std.PDFPage page =
                new cn.wps.moffice.pdf.core.std.PDFPage(pageHandle.value());
        page.parsePageBlocked();
        page.native_onAfterLoadPage(pageHandle.value());
        boolean recognized = page.native_SectionRecognize(pageHandle.value());
        String added = " سلام لا سَّ";
        boolean changed;
        int insertion;
        long rebuiltBridgeEditor = 0;
        if (prefix.equals("rebuilt")) {
            long formHandle = document.native_getFormfill(doc.value());
            PDFFormFill fill = new PDFFormFill(formHandle, pageHandle.value());
            long editorHandle = fill.editorHandle();
            rebuiltBridgeEditor = editorHandle;
            check(formHandle != 0 && formHandle == document.native_getFormfill(doc.value()),
                    "rebuilt FormFill document singleton");
            check(editorHandle != 0 && editorHandle == fill.editorHandle(),
                    "rebuilt TextEditor FormFill singleton");
            boolean focused = formHandle != 0 && editorHandle != 0 && fill.click(80f, 700f);
            PDFTextEditor editor = new PDFTextEditor();
            int beforeLength = focused ? editor.native_length(editorHandle) : -1;
            String before = beforeLength > 0
                    ? editor.native_text(editorHandle, 0, beforeLength) : "";
            insertion = beforeLength;
            boolean selected = insertion >= 0 && editor.native_setCursor(editorHandle, insertion);
            boolean fontSet = selected && editor.native_setFontName(
                    editorHandle, pageHandle.value(), "NotoSansArabic-Regular");
            changed = selected && fill.onChars(added);
            int afterLength = focused ? editor.native_length(editorHandle) : -1;
            String after = afterLength > 0
                    ? editor.native_text(editorHandle, 0, afterLength) : "";
            out(prefix + "ArabicInsert.recognized=" + recognized + ",focus=" + focused
                    + ",form=" + formHandle + ",editor=" + editorHandle
                    + ",before=" + escaped(before) + ",insert=" + changed
                    + ",fontSet=" + fontSet + ",after=" + escaped(after));
            check(!changed || (after != null && after.endsWith(added)),
                    "rebuilt Arabic logical insert");
            boolean markInteriorRejected = true;
            if (changed) for (int caret = insertion; caret <= insertion + added.length(); caret++) {
                boolean set = editor.native_setCursor(editorHandle, caret);
                RectF caretRect = new RectF();
                boolean got = editor.native_getCaretInfo(editorHandle, caretRect, true);
                out(prefix + "ArabicInsert.caret[" + caret + "]=" + set
                        + ",get=" + editor.native_cursor(editorHandle) + ",rect="
                        + got + ":" + caretRect);
                if (caret == insertion + 10 || caret == insertion + 11) {
                    markInteriorRejected &= !set;
                }
            }
            if (changed) {
                check(markInteriorRejected,
                        "rebuilt mark cluster rejects interior caret");
                int clusterStart = insertion + 9;
                boolean clusterSelected = page.native_SetTextSelectRange(
                        pageHandle.value(), clusterStart, insertion + added.length());
                RectF[] clusterRects = new PDFPageEditor().native_selectionRects(
                        pageHandle.value());
                check(clusterSelected && clusterRects != null && clusterRects.length == 1
                                && clusterRects[0].width() > 0 && clusterRects[0].height() > 0,
                        "rebuilt mark cluster has one visual selection rectangle");
            }
            if (changed) {
                editor.native_setCursor(editorHandle, insertion + 2);
                out("rebuiltArabicInsert.direction=" +
                        editor.native_getSelDirection(editorHandle));
                check(editor.native_getSelDirection(editorHandle) == 1,
                        "rebuilt FormFill editor RTL direction");
            }
            if (changed) {
                long undo = RuntimeProbe.nativeTakePendingEditUndo(pageHandle.value());
                check(undo != 0, "rebuilt Arabic undo handle");
                PDFFormFill.native_undoEdit(undo);
                check(before.equals(editor.native_text(editorHandle, 0,
                                editor.native_length(editorHandle))),
                        "rebuilt Arabic undo logical text");
                PDFFormFill.native_redoEdit(undo);
                check(after.equals(editor.native_text(editorHandle, 0,
                                editor.native_length(editorHandle))),
                        "rebuilt Arabic redo logical text");
                PDFFormFill.native_deleteTransEdit(undo);
            }
        } else {
            long formHandle = document.native_getFormfill(doc.value());
            PDFFormFill fill = new PDFFormFill(formHandle, pageHandle.value());
            long editorHandle = fill.editorHandle();
            boolean focused = formHandle != 0 && editorHandle != 0 && fill.click(80f, 700f);
            PDFTextEditor editor = new PDFTextEditor();
            int beforeLength = focused ? editor.native_length(editorHandle) : -1;
            String before = beforeLength > 0
                    ? editor.native_text(editorHandle, 0, beforeLength) : "";
            insertion = beforeLength;
            boolean selected = insertion >= 0 && editor.native_setCursor(editorHandle, insertion);
            boolean fontSet = selected && editor.native_setFontName(
                    editorHandle, pageHandle.value(), "NotoSansArabic-Regular");
            changed = selected && fill.onChars(added);
            int afterLength = focused ? editor.native_length(editorHandle) : -1;
            String after = afterLength > 0
                    ? editor.native_text(editorHandle, 0, afterLength) : "";
            out(prefix + "ArabicInsert.recognized=" + recognized + ",focus=" + focused
                    + ",form=" + formHandle + ",editor=" + editorHandle
                    + ",before=" + escaped(before) + ",insert=" + changed
                    + ",fontSet=" + fontSet + ",after=" + escaped(after));
            if (changed) for (int caret = insertion; caret <= insertion + added.length(); caret++) {
                boolean set = editor.native_setCursor(editorHandle, caret);
                RectF caretRect = new RectF();
                boolean got = editor.native_getCaretInfo(editorHandle, caretRect, true);
                out(prefix + "ArabicInsert.caret[" + caret + "]=" + set
                        + ",get=" + editor.native_cursor(editorHandle) + ",rect="
                        + got + ":" + caretRect);
            }
        }
        if (changed) {
            check(document.native_syncEditPageObj(doc.value(), pageHandle.value(), true),
                    prefix + " Arabic insert sync");
        }
        page.native_unloadPage2(pageHandle.value());
        if (prefix.equals("rebuilt")) {
            check(new PDFTextEditor().native_length(rebuiltBridgeEditor) == -1,
                    "rebuilt TextEditor rejects stale page");
        }
        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        boolean savedOk = session != 0 && saver.start(session, saved)
                && saver.run(session, 500) == 100;
        out(prefix + "ArabicInsert.saved=" + savedOk + ",path=" + saved);
        if (session != 0) saver.destroy(session);
        document.native_closePDF(doc.value());
        if (prefix.equals("rebuilt")) {
            check(new PDFTextEditor().native_length(rebuiltBridgeEditor) == -1,
                    "rebuilt TextEditor rejects closed document");
        }

        NativeHandle reopened = new NativeHandle();
        if (PDFDocument.native_openPDF(saved, reopened) == 0) {
            NativeHandle reopenedPage = new NativeHandle();
            if (document.native_getPage(reopened.value(), 0, reopenedPage) == 0) {
                cn.wps.moffice.pdf.core.std.PDFPage verify =
                        new cn.wps.moffice.pdf.core.std.PDFPage(reopenedPage.value());
                verify.parsePageBlocked();
                String extractedAfter = extractPageText(reopenedPage.value());
                out(prefix + "ArabicInsert.extractedAfter=" + escaped(extractedAfter));
                if (prefix.equals("rebuilt")) {
                    check(extractedAfter != null && extractedAfter.endsWith(added),
                            "rebuilt Arabic saved logical text");
                    document.native_setEditStatus(reopened.value(), 1);
                    boolean recognizedAgain = verify.native_SectionRecognize(
                            reopenedPage.value());
                    boolean focusedAgain = recognizedAgain && verify.native_OnLButtonDown(
                            reopenedPage.value(), 80f, 700f, 0);
                    String reeditText = focusedAgain
                            ? verify.native_GetEditAllText(reopenedPage.value()) : null;
                    out("rebuiltArabicInsert.reedit=" + escaped(reeditText));
                    check(reeditText != null && reeditText.endsWith(added),
                            "rebuilt saved shaped run is editable as logical text");
                    if (reeditText != null && reeditText.endsWith(added)) {
                        int clusterStart = reeditText.length() - 3;
                        boolean selectedAgain = verify.native_SetTextSelectRange(
                                reopenedPage.value(), clusterStart, reeditText.length());
                        int[] selectedRangeAgain = verify.native_GetTextSelectRange(
                                reopenedPage.value());
                        boolean replacedAgain = selectedAgain && verify.native_OnChars(
                                reopenedPage.value(), "د", false, 2);
                        String afterReedit = replacedAgain
                                ? verify.native_GetEditAllText(reopenedPage.value()) : null;
                        out("rebuiltArabicInsert.afterReedit=selected=" + selectedAgain
                                + ",range=" + intArray(selectedRangeAgain)
                                + ",replaced=" + replacedAgain + ",text="
                                + escaped(afterReedit));
                        check(afterReedit != null && afterReedit.endsWith(" د"),
                                "rebuilt saved shaped cluster replaces without orphan glyphs");
                    }
                }
                verify.native_unloadPage2(reopenedPage.value());
            }
            document.native_closePDF(reopened.value());
        }
    }

    static String pointArray(android.graphics.PointF[] points) {
        if (points == null) return "<null>";
        StringBuilder value = new StringBuilder();
        for (android.graphics.PointF point : points) {
            if (value.length() > 0) value.append(';');
            value.append(point == null ? "<null>" : point.x + "," + point.y);
        }
        return value.toString();
    }

    static String intArray(int[] values) {
        if (values == null) return "<null>";
        StringBuilder result = new StringBuilder();
        for (int value : values) {
            if (result.length() > 0) result.append(',');
            result.append(value);
        }
        return result.toString();
    }

    void probeRtlText(String prefix, PDFDocument document, String dir,
                      boolean editProbe) throws Exception {
        String path = copyAsset("rtl-edit-corpus.pdf", dir + "/rtl-edit.pdf");
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(path, doc) == 0, prefix + " RTL open");
        NativeHandle pageHandle = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, pageHandle) == 0,
              prefix + " RTL page");
        cn.wps.moffice.pdf.core.std.PDFPage page =
                new cn.wps.moffice.pdf.core.std.PDFPage(pageHandle.value());
        page.parsePageBlocked();
        PDFTextPage textApi = new PDFTextPage();
        long textPage = textApi.native_new(pageHandle.value());
        String extracted = textApi.native_getPageText(textPage);
        String expectedRun = "םולש";
        int start = extracted == null ? -1 : extracted.indexOf(expectedRun);
        out(prefix + "Rtl.text=" + escaped(extracted) + ",start=" + start);
        check(start >= 0, prefix + " RTL extraction contract");
        if (start >= 0) {
            int count = textApi.native_countRects(textPage, start, start + 3);
            RectF rect = new RectF();
            int direction = count > 0 ? textApi.native_getRect(textPage, 0, rect) : -1;
            out(prefix + "Rtl.rect=" + rect + ",direction=" + direction);
            check(count > 0 && direction == 1, prefix + " RTL rect direction");
        }
        if (editProbe) {
            int[] bidi = RuntimeProbe.nativeResolveBidiRuns("עולם-12", 1);
            check(bidi != null && bidi.length == 6 && bidi[0] == 5 &&
                  bidi[1] == 2 && bidi[2] == 0 && bidi[3] == 0 &&
                  bidi[4] == 5 && bidi[5] == 1,
                  "rebuilt RTL ICU visual-run map");
            document.native_setEditStatus(doc.value(), 1);
            check(page.native_SectionRecognize(pageHandle.value()), "rebuilt RTL recognize");
            check(page.native_OnLButtonDown(pageHandle.value(), 500, 680, 0),
                  "rebuilt RTL focus");
            check("שלום".equals(page.native_GetText(pageHandle.value())),
                  "rebuilt RTL editor logical text");
            page.native_SetCaret(pageHandle.value(), 0);
            android.graphics.PointF[] first = page.native_GetCaretPoints(pageHandle.value());
            page.native_SetCaret(pageHandle.value(), 1);
            android.graphics.PointF[] second = page.native_GetCaretPoints(pageHandle.value());
            check(first != null && second != null && second[0].x < first[0].x,
                  "rebuilt RTL caret advances left");
            page.native_SetTextSelectRange(pageHandle.value(), 0, 4);
            Object[] extended = new PDFPageEditor().native_selectionRectsEx(pageHandle.value());
            check(extended != null && ((int[]) extended[1])[0] == 1,
                  "rebuilt RTL edit selection direction");
            check(page.native_OnLButtonDown(pageHandle.value(), 500, 630, 0),
                  "rebuilt mixed bidi focus");
            String domainText = page.native_GetEditAllText(pageHandle.value());
            int mixedStart = domainText == null ? -1 : domainText.indexOf("עולם-12");
            check(mixedStart >= 0 && page.native_SetTextSelectRange(
                      pageHandle.value(), mixedStart, mixedStart + 7),
                  "rebuilt mixed bidi selection");
            Object[] mixed = new PDFPageEditor().native_selectionRectsEx(pageHandle.value());
            boolean ltr = false, rtl = false;
            if (mixed != null && mixed.length == 2 && mixed[1] instanceof int[]) {
                for (int direction : (int[]) mixed[1]) {
                    ltr |= direction == 0;
                    rtl |= direction == 1;
                }
            }
            check(ltr && rtl, "rebuilt mixed selection run directions");
            check(mixedStart >= 0 && page.native_SetTextSelectRange(
                      pageHandle.value(), mixedStart + 5, mixedStart + 7) &&
                  RuntimeProbe.nativeTextSelectionDirection(pageHandle.value()) == 0,
                  "rebuilt numeric run selection direction");
        } else {
            document.native_setEditStatus(doc.value(), 1);
            boolean recognized = page.native_SectionRecognize(pageHandle.value());
            int domains = page.native_getTextEditorCount(pageHandle.value());
            boolean focused = page.native_OnLButtonDown(pageHandle.value(), 500, 680, 0);
            out("originalRtl.editorRecognized=" + recognized + ",domains=" + domains +
                    ",focused=" + focused + ",text=" + escaped(
                    focused ? page.native_GetText(pageHandle.value()) : null) +
                    ",all=" + escaped(focused ? page.native_GetEditAllText(pageHandle.value()) : null));
        }
        page.native_unloadPage2(pageHandle.value());
        document.native_closePDF(doc.value());
    }

    void runRebuilt() {
        out("== MODE rebuilt ==");
        System.loadLibrary("kspfreetext");
        long doc = TestPdf.nativeNewDoc();
        long page = TestPdf.nativeAddPage(doc, 612, 792);
        String dir = getExternalFilesDir(null).getAbsolutePath();
        String seed = dir + "/seed.pdf";
        check(TestPdf.nativeSaveDoc(doc, seed), "seed save");

        check(FreeTextStub.native_addFreeText(doc, page, 100, 100), "addFreeText");
        out("annotCount=" + TestPdf.nativeAnnotCount(page)
                + " type=" + TestPdf.nativeAnnotType(page, 0));

        FreeTextStub.native_setFreeTextColor(page, 0x00FF0000);
        out("colorAfterSetRed=" + Integer.toHexString(FreeTextStub.native_getFreeTextColor(page)));
        FreeTextStub.native_setFreeTextColor(page, 0x8000FF00);
        out("colorAfterSetGreenAlpha=" + Integer.toHexString(FreeTextStub.native_getFreeTextColor(page)));
        FreeTextStub.native_setSelectFontSize(page, 24.0f);
        out("fontSizeAfterSet24=" + FreeTextStub.native_getSelectFontSize(page));
        RectF r = new RectF(10, 20, 110, 40);
        FreeTextStub.native_setFreeTextRect(page, r);
        RectF r2 = new RectF();
        FreeTextStub.native_getFreeTextRect(page, 0, r2);
        out("rect=" + r2.left + "," + r2.top + "," + r2.right + "," + r2.bottom);
        check(TestPdf.nativeSetContents(page, "Hello"), "setContents");
        check(TestPdf.nativeSaveDoc(doc, dir + "/freetext_rebuilt.pdf"), "rebuilt save");
        TestPdf.nativeClosePage(page);
        TestPdf.nativeCloseDoc(doc);

        try {
            runRebuiltDocumentContract(dir);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    void runRebuiltDocumentContract(String dir) throws Exception {
        String fixture = copyFixture(dir, "contract.pdf");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(fixture, doc) == 0, "registry openPDF");
        probeEditableInventory("rebuilt", document, dir);
        probeEditUndo("rebuilt", document, dir);
        probeTextStyle("rebuilt", document, dir);
        probeFormEditCorpus("rebuilt", document, dir);
        probeGlyphCaretCorpus("rebuilt", document, dir);
        probeEditDomainCorpus("rebuilt", document, dir);
        probeAdvancedTextReflow("rebuilt", document, dir);
        probeExternalPaper("rebuilt", document, dir);
        probePageObjectInventory("rebuilt", document, dir);
        probeImageDelete("rebuilt", document, dir);
        probeRebuiltImageGeometry(document, dir);
        out("registryDocHandle=" + doc.value());
        check(doc.value() != 0, "registry document writeback");
        check(document.native_isValid(doc.value()), "registry document valid");
        out("registryPageCount=" + document.native_getPageCount(doc.value()));

        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0, "registry getPage");
        cn.wps.rebuild.TextPageProbe.probe("rebuilt", page.value());
        check(page.value() != 0, "registry page writeback");
        cn.wps.moffice.pdf.core.std.PDFPage pageStub =
                new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
        probeEditSession("rebuilt", document, doc.value(), pageStub, page.value());
        RectF pageSize = pageStub.getPageSize();
        out("registryPageSize=" + pageSize);
        int wrongType = document.native_getPageCount(page.value());
        out("registryWrongType=" + wrongType);
        check(wrongType == -1003, "registry wrong type rejected");
        check(new PDFAnnotationEditor().native_addFreeText(doc.value(), page.value(), 20, 20),
                "registry FreeText handles");
        check(pageStub.native_unloadPage2(page.value()) == 0, "registry closePage");
        check(!pageStub.native_isValid(page.value()), "registry stale page rejected");
        check(document.native_closePDF(doc.value()) == 0, "registry closePDF");

        String passwordFixture = copyAsset(
                "password-user123.pdf", dir + "/password-user123.pdf");
        NativeHandle locked = new NativeHandle();
        int passwordOpen = PDFDocument.native_openPDF(passwordFixture, locked);
        out("registryPasswordOpen=" + passwordOpen + " handle=" + locked.value());
        check(passwordOpen == -3 && locked.value() != 0, "registry password pending handle");
        check(document.native_reopenInPassword(locked.value(), "wrong") == -3,
                "registry wrong password rejected");
        check(document.native_reopenInPassword(locked.value(), "user123") == 0,
                "registry password reopen");
        check(document.native_getPageCount(locked.value()) == 5,
                "registry password page count");
        check(document.native_closePDF(locked.value()) == 0,
                "registry password close");

        String saved = dir + "/session_rebuilt.pdf";
        new File(saved).delete();
        // doc was closed above; use a fresh handle for the save oracle.
        NativeHandle saveDoc = new NativeHandle();
        check(PDFDocument.native_openPDF(fixture, saveDoc) == 0, "save oracle source open");
        PDFNativeSaver nativeSaver = new PDFNativeSaver();
        long saveSession = nativeSaver.newSession(saveDoc.value(), false);
        out("rebuiltSaveSession=" + saveSession);
        check(saveSession != 0, "rebuilt saver new");
        check(nativeSaver.start(saveSession, saved), "rebuilt saver start");
        int saveProgress = nativeSaver.run(saveSession, 200);
        out("rebuiltSaveProgress=" + saveProgress);
        check(saveProgress == 100, "rebuilt saver continue");
        nativeSaver.destroy(saveSession);
        NativeHandle verify = new NativeHandle();
        check(PDFDocument.native_openPDF(saved, verify) == 0, "rebuilt saved reopen");
        out("rebuiltSavedPageCount=" + document.native_getPageCount(verify.value()));
        document.native_closePDF(verify.value());

        String optimized = dir + "/optimize_rebuilt.pdf";
        new File(optimized).delete();
        long optimizeSession = document.native_openOptimize(saveDoc.value(), optimized, 0);
        out("rebuiltOptimizeSession=" + optimizeSession);
        check(optimizeSession != 0, "rebuilt optimize open");
        int optimizeProgress = document.native_continueOptimize(
                saveDoc.value(), optimizeSession, 200);
        out("rebuiltOptimizeSteps=" + optimizeProgress);
        out("rebuiltOptimizeSize=" + new File(optimized).length()
                + " sourceSize=" + new File(fixture).length());
        check(optimizeProgress == 100, "rebuilt optimize continue");
        document.native_closeOptimize(saveDoc.value(), optimizeSession);
        NativeHandle optimizeVerify = new NativeHandle();
        check(PDFDocument.native_openPDF(optimized, optimizeVerify) == 0,
                "rebuilt optimized reopen");
        out("rebuiltOptimizedPageCount="
                + document.native_getPageCount(optimizeVerify.value()));
        document.native_closePDF(optimizeVerify.value());
        document.native_closePDF(saveDoc.value());
        out("registryLiveHandles=" + RuntimeProbe.nativeLiveHandleCount());
    }

    String copyFixture(String dir, String name) throws Exception {
        return copyAsset("testfile_pdf_clip.pdf", dir + "/" + name);
    }

    String copyAsset(String asset, String path) throws Exception {
        try (InputStream in = getAssets().open(asset);
             FileOutputStream target = new FileOutputStream(path)) {
            byte[] buf = new byte[8192];
            for (int n; (n = in.read(buf)) >= 0;) target.write(buf, 0, n);
        }
        return path;
    }

    // -------------------------------------------------------------------
    // original variant (drives libkwopdf.so via WPS-named stubs)
    // -------------------------------------------------------------------
    void runOriginal() throws Exception {
        out("== MODE original ==");
        System.loadLibrary("kwopdf");
        String dir = getExternalFilesDir(null).getAbsolutePath();
        String seed;
        try {
            seed = copyFixture(dir, "seed.pdf");
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
        out("seedExists=" + new File(seed).exists());

        int init = new PDFModuleMgr().initialize();
        out("moduleInit=" + init);

        NativeHandle docH = new NativeHandle();
        int open = PDFDocument.native_openPDF(seed, docH);
        out("openPDF=" + open + " docHandle=" + docH.value());
        long d = docH.value();
        PDFDocument docStub = new PDFDocument();
        int pc = docStub.native_getPageCount(d);
        out("pageCount=" + pc);
        probeEditableInventory("original", docStub, dir);
        probeOriginalLayout(docStub, dir);
        probeFormEditCorpus("original", docStub, dir);
        probeGlyphCaretCorpus("original", docStub, dir);
        probeEditDomainCorpus("original", docStub, dir);
        probeAdvancedTextReflow("original", docStub, dir);
        probeExternalPaper("original", docStub, dir);
        probePageObjectInventory("original", docStub, dir);
        probeImageDelete("original", docStub, dir);
        probeOriginalImageGeometry(docStub, dir);

        // native_getPage is 0-based internally; success == 0
        NativeHandle pageH = new NativeHandle();
        int pr = docStub.native_getPage(d, 0, pageH);
        out("getPage(index0)=" + pr + " pageHandle=" + pageH.value());
        long p = pageH.value();

        if (p != 0) {
            cn.wps.moffice.pdf.core.std.PDFPage originalPage =
                    new cn.wps.moffice.pdf.core.std.PDFPage(p);
            originalPage.parsePageBlocked();
            probeEditSession("original", docStub, d, originalPage, p);
            probeTextPage("original", p);
            PDFAnnotationEditor ed = new PDFAnnotationEditor();
            boolean added = ed.native_addFreeText(d, p, 100, 100);
            out("addFreeText=" + added);

            FreeTextAnnotation ft = new FreeTextAnnotation();
            ft.native_setFreeTextColor(p, 0x00FF0000);
            out("colorAfterSetRed=" + Integer.toHexString(ft.native_getFreeTextColor(p)));
            ft.native_setFreeTextColor(p, 0x8000FF00);
            out("colorAfterSetGreenAlpha=" + Integer.toHexString(ft.native_getFreeTextColor(p)));
            ft.native_setSelectFontSize(p, 24.0f);
            out("fontSizeAfterSet24=" + ft.native_getSelectFontSize(p));
            out("rectSKIP=needsRealAnnotHandle(full editor flow)");
        } else {
            out("SKIP ops (page==0)");
        }

        String optimized = dir + "/optimize_original.pdf";
        new File(optimized).delete();
        long optimizeSession = docStub.native_openOptimize(d, optimized, 0);
        out("originalOptimizeSession=" + optimizeSession);
        check(optimizeSession != 0, "original optimize open");
        int optimizeProgress = 0;
        StringBuilder optimizeSteps = new StringBuilder();
        for (int i = 0; i < 100 && optimizeProgress >= 0 && optimizeProgress < 100; i++) {
            optimizeProgress = docStub.native_continueOptimize(d, optimizeSession, 200);
            if (i != 0) optimizeSteps.append(',');
            optimizeSteps.append(optimizeProgress);
        }
        out("originalOptimizeSteps=" + optimizeSteps);
        out("originalOptimizeSize=" + new File(optimized).length()
                + " sourceSize=" + new File(seed).length());
        check(optimizeProgress == 100, "original optimize continue");
        docStub.native_closeOptimize(d, optimizeSession);
        NativeHandle optimizedVerify = new NativeHandle();
        int optimizedOpen = PDFDocument.native_openPDF(optimized, optimizedVerify);
        check(optimizedOpen == 0, "original optimized reopen");
        if (optimizedOpen == 0) {
            out("originalOptimizedPageCount="
                    + docStub.native_getPageCount(optimizedVerify.value()));
            docStub.native_closePDF(optimizedVerify.value());
        }

        String encrypted = dir + "/password-user123.pdf";
        boolean passwordSaved = docStub.native_setPasswords(
                d, encrypted, "owner123", "user123", 0x0F3C, true);
        out("passwordFixtureSaved=" + passwordSaved + " path=" + encrypted);

        String saved = dir + "/session_original.pdf";
        new File(saved).delete();
        PDFNativeSaver nativeSaver = new PDFNativeSaver();
        long saveSession = nativeSaver.newSession(d, false);
        out("originalSaveSession=" + saveSession);
        check(saveSession != 0, "original saver new");
        check(nativeSaver.start(saveSession, saved), "original saver start");
        int saveProgress = 0;
        for (int i = 0; i < 50 && saveProgress >= 0 && saveProgress < 100; i++)
            saveProgress = nativeSaver.run(saveSession, 200);
        out("originalSaveProgress=" + saveProgress);
        check(saveProgress == 100, "original saver continue");
        nativeSaver.destroy(saveSession);
        NativeHandle verify = new NativeHandle();
        int verifyOpen = PDFDocument.native_openPDF(saved, verify);
        out("originalSavedOpen=" + verifyOpen);
        if (verifyOpen == -3 && verify.value() != 0) {
            verifyOpen = docStub.native_reopenInPassword(verify.value(), "user123");
            out("originalSavedPasswordReopen=" + verifyOpen);
        }
        check(verifyOpen == 0, "original saved reopen");
        if (verifyOpen == 0) {
            out("originalSavedPageCount=" + docStub.native_getPageCount(verify.value()));
            docStub.native_closePDF(verify.value());
        }

        int c = docStub.native_closePDF(d);
        out("closePDF=" + c);
    }

    void probeTextPage(String prefix, long page) {
        PDFTextPage api = new PDFTextPage();
        long textPage = api.native_new(page);
        out(prefix + "TextHandle=" + textPage);
        int count = TextPage.nCharCount(textPage);
        out(prefix + "TextCount=" + count);
        String all = api.native_getPageText(textPage);
        out(prefix + "TextAll=" + escaped(all));
        if (count > 0) {
            int last = count - 1;
            int sampleEnd = Math.min(last, 7);
            out(prefix + "TextRange0_0=" + escaped(api.native_getText(textPage, 0, 0)));
            out(prefix + "TextRange0_" + sampleEnd + "="
                    + escaped(api.native_getText(textPage, 0, sampleEnd)));
            int[] samples = sampleEnd == last ? new int[] {0, last} : new int[] {0, sampleEnd, last};
            for (int index : samples) {
                out(prefix + "TextChar[" + index + "]=" + api.native_getUnicode(textPage, index)
                        + ",size=" + TextPage.nFontSize(textPage, index)
                        + ",color=" + Integer.toHexString(TextPage.nColor(textPage, index))
                        + ",font=" + escaped(TextPage.nSysFontName(textPage, index)));
            }
            out(prefix + "TextUnicodeOob=" + api.native_getUnicode(textPage, count));
            int rectCount = api.native_countRects(textPage, 0, Math.min(last, 59));
            out(prefix + "TextRectCount=" + rectCount);
            if (rectCount > 0) {
                RectF rect = new RectF();
                int direction = api.native_getRect(textPage, 0, rect);
                int hit = api.native_getCPAtPoint(textPage, (rect.left + rect.right) / 2,
                        (rect.top + rect.bottom) / 2, 1, 1);
                int[] range = new int[2];
                int selected = api.native_getCharRangeIndex(textPage, rect.left, rect.top,
                        rect.right, rect.bottom, range, false);
                out(prefix + "TextRect0=" + rect + ",direction=" + direction + ",hit=" + hit
                        + ",bounded=" + selected + ":" + range[0] + "," + range[1]);
            }
            java.util.ArrayList<String> paragraphs = api.native_getText2(
                    textPage, 0, Math.min(last, 59));
            java.util.ArrayList<String> blocks = new java.util.ArrayList<>();
            java.util.ArrayList<RectF> blockRects = new java.util.ArrayList<>();
            api.native_getPageTextAndRects(textPage, blocks, blockRects);
            out(prefix + "TextParagraphs=" + paragraphs.size() + ",blocks=" + blocks.size()
                    + ",blockRects=" + blockRects.size());
        }
    }

    void probeEditSession(String prefix, PDFDocument document,
                          long doc, cn.wps.moffice.pdf.core.std.PDFPage pageApi, long page) {
        out(prefix + "EditDefault=" + document.nGetEditStatus(doc)
                + ",insert=" + document.nGetInsertStatus(doc));
        document.native_setEditStatus(doc, 7);
        document.native_setInsertStatus(doc, 4);
        out(prefix + "EditRoundTrip=" + document.nGetEditStatus(doc)
                + ",insert=" + document.nGetInsertStatus(doc));
        out(prefix + "EditInitialModified=" + pageApi.native_IsModified(page)
                + ",text=" + pageApi.native_CheckFuncModified(page, 1));
        out(prefix + "EditBackup=" + document.native_EditBackup(doc, page, 7));
        out(prefix + "EditClearFocus=" + document.native_ClearEditFocus(doc, page)
                + ",hover=" + document.native_ClearHoverHandle(doc, page));
        out(prefix + "EditSeparate="
                + document.native_separateSharedDataInEditPage(doc, 0));
        out(prefix + "EditSync=" + document.native_syncEditPageObj(doc, page, false));
        pageApi.native_regenerateContent(page);
        out(prefix + "EditAfterSyncModified=" + pageApi.native_IsModified(page));
        pageApi.native_onAfterLoadPage(page);
        int editorCount = pageApi.native_getTextEditorCount(page);
        out(prefix + "TextEditorCount=" + editorCount);
        if (editorCount == 0) {
            document.native_setEditStatus(doc, 1);
            pageApi.native_onAfterLoadPage(page);
            editorCount = pageApi.native_getTextEditorCount(page);
            out(prefix + "TextModeEditorCount=" + editorCount);
        }
        for (int i = 0; i < Math.min(editorCount, 12); i++) {
            RectF rect = new RectF();
            float[] matrix = new float[9];
            boolean ok = pageApi.native_getTextEditorRect(page, i, rect, matrix, false);
            out(prefix + "TextEditor[" + i + "]=" + ok + "," + rect
                    + ",m=" + matrix[0] + "," + matrix[1] + "," + matrix[2]
                    + "," + matrix[3] + "," + matrix[4] + "," + matrix[5]
                    + "," + matrix[6] + "," + matrix[7] + "," + matrix[8]);
        }
    }

    void probeEditableInventory(String prefix, PDFDocument document, String dir) throws Exception {
        String path = copyAsset("editable-helvetica.pdf", dir + "/editable-helvetica.pdf");
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(path, doc);
        out(prefix + "EditableOpen=" + opened);
        if (opened != 0) return;
        NativeHandle page = new NativeHandle();
        if (document.native_getPage(doc.value(), 0, page) != 0) return;
        cn.wps.moffice.pdf.core.std.PDFPage pageApi =
                new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
        pageApi.parsePageBlocked();
        document.native_setEditStatus(doc.value(), 1);
        pageApi.native_onAfterLoadPage(page.value());
        int count = pageApi.native_getTextEditorCount(page.value());
        out(prefix + "EditableTextEditorCount=" + count);
        boolean recognized = pageApi.native_SectionRecognize(page.value());
        int recognizedCount = pageApi.native_getTextEditorCount(page.value());
        out(prefix + "SectionRecognize=" + recognized
                + ",editorCount=" + recognizedCount);
        count = recognizedCount;
        for (int i = 0; i < count; i++) {
            RectF rect = new RectF();
            float[] matrix = new float[9];
            boolean ok = pageApi.native_getTextEditorRect(page.value(), i, rect, matrix, false);
            out(prefix + "EditableTextEditor[" + i + "]=" + ok + "," + rect
                    + ",m=" + matrix[0] + "," + matrix[1] + "," + matrix[2]
                    + "," + matrix[3] + "," + matrix[4] + "," + matrix[5]
                    + "," + matrix[6] + "," + matrix[7] + "," + matrix[8]);
        }
        boolean focused = pageApi.native_OnLButtonDown(page.value(), 80f, 700f, 0);
        pageApi.native_OnLButtonUp(page.value(), 80f, 700f);
        out(prefix + "GlyphFocus=" + focused + ",type="
                + pageApi.native_GetFocusType(page.value()) + ",text="
                + escaped(pageApi.native_GetText(page.value())) + ",all="
                + escaped(pageApi.native_GetEditAllText(page.value())) + ",length="
                + pageApi.native_GetEditAllTextLength(page.value()));
        if (pageApi.native_GetFocusType(page.value()) == 1) {
            float[] xs = {70f, 72f, 76f, 81f, 86f, 94f, 106f, 124f, 150f, 190f, 240f};
            for (float x : xs) {
                out(prefix + "GlyphHit[" + x + ",700]="
                        + pageApi.native_GetRangeIndexAtPos(page.value(), x, 700f));
            }
            int textLength = pageApi.native_GetEditAllTextLength(page.value());
            int[] carets = {0, 1, 2, 8, textLength - 1, textLength};
            for (int caret : carets) {
                boolean set = pageApi.native_SetCaret(page.value(), caret);
                RectF caretRect = new RectF();
                pageApi.native_GetCaretInfo(page.value(), caretRect);
                android.graphics.PointF[] points = pageApi.native_GetCaretPoints(page.value());
                RectF glyphRect = new RectF();
                pageApi.native_GetEditSelectionRect(page.value(), caret, glyphRect);
                float[] quad = pageApi.native_GetEditSelectionQuadRect(page.value(), caret);
                out(prefix + "GlyphCaret[" + caret + "]=" + set + ",get="
                        + pageApi.native_GetCaret(page.value()) + ",rect=" + caretRect
                        + ",points=" + pointsString(points) + ",glyph=" + glyphRect
                        + ",quad=" + floatsString(quad));
            }
            out(prefix + "GlyphCaretOob="
                    + pageApi.native_SetCaret(page.value(), textLength + 1));
        }
        pageApi.native_unloadPage2(page.value());
        document.native_closePDF(doc.value());
    }

    static String pointsString(android.graphics.PointF[] points) {
        if (points == null) return "<null>";
        StringBuilder value = new StringBuilder();
        for (int i = 0; i < points.length; i++) {
            if (i > 0) value.append(';');
            value.append(points[i].x).append(',').append(points[i].y);
        }
        return value.toString();
    }

    static String floatsString(float[] values) {
        if (values == null) return "<null>";
        StringBuilder value = new StringBuilder();
        for (int i = 0; i < values.length; i++) {
            if (i > 0) value.append(',');
            value.append(values[i]);
        }
        return value.toString();
    }

    void probeGlyphCaretCorpus(String prefix, PDFDocument document, String dir) throws Exception {
        String path = copyAsset("glyph-caret-corpus.pdf", dir + "/glyph-caret-corpus.pdf");
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(path, doc);
        out(prefix + "GlyphCorpusOpen=" + opened);
        if (opened != 0) return;
        NativeHandle page = new NativeHandle();
        if (document.native_getPage(doc.value(), 0, page) != 0) return;
        cn.wps.moffice.pdf.core.std.PDFPage api =
                new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
        api.parsePageBlocked();
        document.native_setEditStatus(doc.value(), 1);
        api.native_SectionRecognize(page.value());
        probeGlyphObject(prefix + "GlyphNormal", api, page.value(), 80f, 704f,
                new float[][] {{72, 700}, {77, 700}, {84, 700}, {94, 700},
                        {108, 700}, {125, 700}, {150, 700}, {190, 700}}, 10);
        if (prefix.equals("rebuilt")) {
            boolean textHover = api.native_GetHoverType(page.value(), 80f, 704f) == 1;
            boolean blankHover = api.native_GetHoverType(page.value(), 20f, 20f) == 0;
            boolean down = api.native_OnLButtonDown(page.value(), 80f, 704f, 0);
            boolean drag = api.native_OnMouseMove(page.value(), 145f, 704f, 0);
            int[] dragged = api.native_GetTextSelectRange(page.value());
            boolean up = api.native_OnLButtonUp(page.value(), 145f, 704f);
            boolean word = api.native_OnLButtonDoubleClick(page.value(), 100f, 704f);
            int[] wordRange = api.native_GetTextSelectRange(page.value());
            boolean object = api.native_OnLButtonTripleClick(page.value(), 100f, 704f);
            int[] objectRange = api.native_GetTextSelectRange(page.value());
            check(textHover && blankHover, "rebuilt pointer hover");
            check(down && drag && up && dragged != null && dragged[0] == 0 && dragged[1] > 0,
                    "rebuilt pointer drag selection");
            check(word && wordRange != null && wordRange[0] == 0 && wordRange[1] == 3,
                    "rebuilt pointer word selection");
            check(object && objectRange != null && objectRange[0] == 0 && objectRange[1] == 10,
                    "rebuilt pointer object selection");
        }
        probeGlyphObject(prefix + "GlyphRotated", api, page.value(), 309f, 404f,
                new float[][] {{300, 400}, {305, 402.33f}, {310, 404.66f},
                        {320, 409.33f}, {335, 416.31f}, {355, 425.64f}, {385, 439.63f},
                        {420, 455.95f}}, 12);
        api.native_unloadPage2(page.value());
        document.native_closePDF(doc.value());
    }

    void probeEditUndo(String prefix, PDFDocument document, String dir) throws Exception {
        String path = copyAsset("editable-helvetica.pdf", dir + "/edit-undo.pdf");
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(path, doc);
        out(prefix + "UndoOpen=" + opened);
        if (opened != 0) return;
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0, prefix + " undo page");
        cn.wps.moffice.pdf.core.std.PDFPage api =
                new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
        document.native_setEditStatus(doc.value(), 1);
        api.native_SectionRecognize(page.value());
        check(document.native_EditBackup(doc.value(), page.value(), 1), prefix + " undo backup");
        check(api.native_OnLButtonDown(page.value(), 80, 700, 0), prefix + " undo focus");
        String before = api.native_GetText(page.value());
        check(api.native_SetTextSelectRange(page.value(), before.length(), before.length()) &&
                api.native_OnChars(page.value(), "+", false, before.length() + 1),
                prefix + " undo mutate");
        long command = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        out(prefix + "UndoHandle=" + command);
        check(command != 0, prefix + " opaque undo handle");
        PDFFormFill.native_undoEdit(command);
        check(before.equals(api.native_GetText(page.value())), prefix + " undo apply");
        PDFFormFill.native_redoEdit(command);
        check((before + "+").equals(api.native_GetText(page.value())), prefix + " redo apply");
        check(document.native_ClearEditPageData(doc.value(), page.value()), prefix + " cancel page");
        check(api.native_OnLButtonDown(page.value(), 80, 700, 0) &&
                before.equals(api.native_GetText(page.value())), prefix + " cancel restored");
        PDFFormFill.native_deleteTransEdit(command);
        api.native_unloadPage2(page.value());
        document.native_closePDF(doc.value());
    }

    void probeTextStyle(String prefix, PDFDocument document, String dir) throws Exception {
        String path = copyAsset("editable-helvetica.pdf", dir + "/text-style-source.pdf");
        String savedPath = dir + "/text-style-roundtrip-android.pdf";
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(path, doc) == 0, prefix + " style open");
        document.native_setEditStatus(doc.value(), 1);
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0, prefix + " style page");
        cn.wps.moffice.pdf.core.std.PDFPage api =
                new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
        api.native_SectionRecognize(page.value());
        check(api.native_OnLButtonDown(page.value(), 150, 700, 0), prefix + " style focus");
        float originalSize = api.native_GetFontSize(page.value());
        int originalColor = api.native_GetTextColor(page.value());
        String fontName = api.native_GetFontName(page.value());
        float changedSize = originalSize + 4;
        int changedColor = 0xff3366cc;
        check(originalSize > 0 && fontName != null && !fontName.isEmpty(),
                prefix + " style read");
        check(api.native_SetFontSize(page.value(), changedSize), prefix + " style size");
        long sizeUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        PDFFormFill.native_undoEdit(sizeUndo);
        check(Math.abs(api.native_GetFontSize(page.value()) - originalSize) < 0.01f,
                prefix + " style size undo");
        PDFFormFill.native_redoEdit(sizeUndo);
        check(Math.abs(api.native_GetFontSize(page.value()) - changedSize) < 0.01f,
                prefix + " style size redo");
        check(api.native_SetTextColor(page.value(), changedColor), prefix + " style color");
        long colorUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        PDFFormFill.native_undoEdit(colorUndo);
        check(api.native_GetTextColor(page.value()) == originalColor,
                prefix + " style color undo");
        PDFFormFill.native_redoEdit(colorUndo);
        check(api.native_GetTextColor(page.value()) == changedColor,
                prefix + " style color redo");
        check(api.native_SetFontName(page.value(), "Courier"), prefix + " style font");
        long fontUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        PDFFormFill.native_undoEdit(fontUndo);
        check(fontName.equals(api.native_GetFontName(page.value())), prefix + " style font undo");
        PDFFormFill.native_redoEdit(fontUndo);
        check("Courier".equals(api.native_GetFontName(page.value())), prefix + " style font redo");
        check(api.native_getAlignment(page.value()) == 0, prefix + " style alignment read");
        check(api.native_setAlignment(page.value(), 1), prefix + " style alignment");
        long alignmentUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        PDFFormFill.native_undoEdit(alignmentUndo);
        check(api.native_getAlignment(page.value()) == 0, prefix + " style alignment undo");
        PDFFormFill.native_redoEdit(alignmentUndo);
        check(api.native_getAlignment(page.value()) == 1, prefix + " style alignment redo");
        check(api.native_setAlignment(page.value(), 2), prefix + " style right alignment");
        long rightAlignmentUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(api.native_getAlignment(page.value()) == 2, prefix + " style right alignment read");
        PDFFormFill.native_undoEdit(rightAlignmentUndo);
        check(api.native_getAlignment(page.value()) == 1, prefix + " style right alignment undo");
        float originalLeading = api.native_getLeading(page.value());
        check(originalLeading > 0, prefix + " style leading read");
        check(api.native_setLeading(page.value(), 2.0f), prefix + " style leading");
        long leadingUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        PDFFormFill.native_undoEdit(leadingUndo);
        check(Math.abs(api.native_getLeading(page.value()) - originalLeading) < 0.02f,
                prefix + " style leading undo");
        PDFFormFill.native_redoEdit(leadingUndo);
        check(Math.abs(api.native_getLeading(page.value()) - 2.0f) < 0.02f,
                prefix + " style leading redo");
        check(document.native_syncEditPageObj(doc.value(), page.value(), true),
                prefix + " style sync");
        api.native_unloadPage2(page.value());
        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        check(session != 0 && saver.start(session, savedPath) && saver.run(session, 500) == 100,
                prefix + " style save");
        saver.destroy(session);
        PDFFormFill.native_deleteTransEdit(leadingUndo);
        PDFFormFill.native_deleteTransEdit(rightAlignmentUndo);
        PDFFormFill.native_deleteTransEdit(alignmentUndo);
        PDFFormFill.native_deleteTransEdit(fontUndo);
        PDFFormFill.native_deleteTransEdit(colorUndo);
        PDFFormFill.native_deleteTransEdit(sizeUndo);
        document.native_closePDF(doc.value());

        NativeHandle reopened = new NativeHandle();
        check(PDFDocument.native_openPDF(savedPath, reopened) == 0, prefix + " style reopen");
        document.native_setEditStatus(reopened.value(), 1);
        NativeHandle verifyPage = new NativeHandle();
        check(document.native_getPage(reopened.value(), 0, verifyPage) == 0,
                prefix + " style reopen page");
        cn.wps.moffice.pdf.core.std.PDFPage verify =
                new cn.wps.moffice.pdf.core.std.PDFPage(verifyPage.value());
        verify.native_SectionRecognize(verifyPage.value());
        check(verify.native_OnLButtonDown(verifyPage.value(), 150, 700, 0),
                prefix + " style reopen focus");
        check(Math.abs(verify.native_GetFontSize(verifyPage.value()) - changedSize) < 0.01f,
                prefix + " style saved size");
        check(verify.native_GetTextColor(verifyPage.value()) == changedColor,
                prefix + " style saved color");
        check("Courier".equals(verify.native_GetFontName(verifyPage.value())),
                prefix + " style saved font");
        check(verify.native_getAlignment(verifyPage.value()) == 1,
                prefix + " style saved alignment");
        check(Math.abs(verify.native_getLeading(verifyPage.value()) - 2.0f) < 0.02f,
                prefix + " style saved leading");
        verify.native_unloadPage2(verifyPage.value());
        document.native_closePDF(reopened.value());
    }

    void probeOriginalLayout(PDFDocument document, String dir) throws Exception {
        String path = copyAsset("editable-helvetica.pdf", dir + "/original-layout-source.pdf");
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(path, doc);
        out("originalLayout.open=" + opened);
        if (opened != 0) return;
        document.native_setEditStatus(doc.value(), 1);
        NativeHandle page = new NativeHandle();
        int loaded = document.native_getPage(doc.value(), 0, page);
        out("originalLayout.page=" + loaded);
        if (loaded == 0) {
            cn.wps.moffice.pdf.core.std.PDFPage api =
                    new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
            api.parsePageBlocked();
            boolean recognized = api.native_SectionRecognize(page.value());
            boolean focused = api.native_OnLButtonDown(page.value(), 150f, 700f, 0);
            float before = api.native_getLeading(page.value());
            boolean setOneHalf = api.native_setLeading(page.value(), 1.5f);
            float oneHalf = api.native_getLeading(page.value());
            boolean setTwo = api.native_setLeading(page.value(), 2.0f);
            float two = api.native_getLeading(page.value());
            RectF domain = new RectF();
            float[] matrix = new float[9];
            boolean rect = api.native_getTextEditorRect(page.value(), 0, domain, matrix, false);
            out("originalLayout.recognized=" + recognized + ",focus=" + focused
                    + ",leading=" + before + ",set1.5=" + setOneHalf + ",read1.5=" + oneHalf
                    + ",set2=" + setTwo + ",read2=" + two + ",rect=" + rect + ":" + domain);
            int textLength = api.native_GetEditAllTextLength(page.value());
            int caretBefore = api.native_GetCaret(page.value());
            String overflow = " alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu";
            boolean inserted = api.native_OnChars(page.value(), overflow, false,
                    Math.max(0, caretBefore) + overflow.length());
            RectF afterDomain = new RectF();
            boolean afterRect = api.native_getTextEditorRect(page.value(), 0,
                    afterDomain, new float[9], false);
            out("originalLayout.reflow.len=" + textLength + ",caret=" + caretBefore
                    + ",insert=" + inserted + ",domains="
                    + api.native_getTextEditorCount(page.value()) + ",rect="
                    + afterRect + ":" + afterDomain);
            api.native_unloadPage2(page.value());
        }
        document.native_closePDF(doc.value());
    }

    void probeFormEditCorpus(String prefix, PDFDocument document, String dir) throws Exception {
        String path = copyAsset("form-edit-corpus.pdf", dir + "/form-edit-corpus-" + prefix + ".pdf");
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(path, doc);
        out(prefix + "FormEdit.open=" + opened);
        if (opened != 0) return;
        document.native_setEditStatus(doc.value(), 1);
        out(prefix + "FormEdit.separate="
                + document.native_separateSharedDataInEditPage(doc.value(), 0));

        NativeHandle page0 = new NativeHandle();
        if (document.native_getPage(doc.value(), 0, page0) == 0) {
            cn.wps.moffice.pdf.core.std.PDFPage api =
                    new cn.wps.moffice.pdf.core.std.PDFPage(page0.value());
            api.parsePageBlocked();
            out(prefix + "FormEdit.page0.recognize=" + api.native_SectionRecognize(page0.value())
                    + ",domains=" + api.native_getTextEditorCount(page0.value()));
            boolean firstFocus = api.native_OnLButtonDown(page0.value(), 90f, 674f, 0);
            String before = api.native_GetText(page0.value());
            int editorLength = api.native_GetEditAllTextLength(page0.value());
            String extractedBefore = extractPageText(page0.value());
            boolean changed = false;
            if (firstFocus && editorLength > 0) {
                changed = api.native_SetTextSelectRange(page0.value(), 0, editorLength)
                        && api.native_OnChars(page0.value(), "EDITED FORM", false, 11);
            }
            api.native_reloadTextPage(page0.value());
            out(prefix + "FormEdit.first=" + firstFocus + ",length=" + editorLength
                    + ",before=" + escaped(before) + ",pageBefore=" + escaped(extractedBefore)
                    + ",changed=" + changed + ",after=" + escaped(api.native_GetText(page0.value()))
                    + ",pageAfter=" + escaped(extractPageText(page0.value())));
            if (prefix.equals("rebuilt")) {
                check(changed && "EDITED FORM".equals(api.native_GetText(page0.value()).trim()),
                        "rebuilt Form child replace");
            }
            boolean secondFocus = api.native_OnLButtonDown(page0.value(), 318f, 674f, 0);
            out(prefix + "FormEdit.samePage=" + secondFocus + ",text="
                    + escaped(api.native_GetText(page0.value())));
            if (prefix.equals("rebuilt")) {
                check(secondFocus && "SHARED FORM TEXT".equals(
                                api.native_GetText(page0.value()).trim()),
                        "rebuilt same-page Form isolation");
                check(document.native_syncEditPageObj(doc.value(), page0.value(), true),
                        "rebuilt Form sync before save");
            }
        }

        NativeHandle page1 = new NativeHandle();
        if (document.native_getPage(doc.value(), 1, page1) == 0) {
            cn.wps.moffice.pdf.core.std.PDFPage api =
                    new cn.wps.moffice.pdf.core.std.PDFPage(page1.value());
            api.parsePageBlocked();
            api.native_SectionRecognize(page1.value());
            boolean otherFocus = api.native_OnLButtonDown(page1.value(), 90f, 674f, 0);
            out(prefix + "FormEdit.otherPage=" + otherFocus + ",text="
                    + escaped(api.native_GetText(page1.value())) + ",pageText="
                    + escaped(extractPageText(page1.value())));
            if (prefix.equals("rebuilt")) {
                check(otherFocus && "SHARED FORM TEXT".equals(
                                api.native_GetText(page1.value()).trim()),
                        "rebuilt cross-page Form isolation");
            }
            api.native_unloadPage2(page1.value());
        }
        if (prefix.equals("rebuilt")) {
            String saved = dir + "/form-edit-roundtrip-android.pdf";
            new File(saved).delete();
            PDFNativeSaver saver = new PDFNativeSaver();
            long session = saver.newSession(doc.value(), false);
            check(session != 0 && saver.start(session, saved)
                            && saver.run(session, 500) == 100,
                    "rebuilt Form save");
            saver.destroy(session);
            if (page0.value() != 0) {
                new cn.wps.moffice.pdf.core.std.PDFPage(page0.value())
                        .native_unloadPage2(page0.value());
            }
            document.native_closePDF(doc.value());

            NativeHandle reopened = new NativeHandle();
            check(PDFDocument.native_openPDF(saved, reopened) == 0,
                    "rebuilt Form saved reopen");
            document.native_setEditStatus(reopened.value(), 1);
            NativeHandle verify = new NativeHandle();
            check(document.native_getPage(reopened.value(), 0, verify) == 0,
                    "rebuilt Form saved page");
            cn.wps.moffice.pdf.core.std.PDFPage api =
                    new cn.wps.moffice.pdf.core.std.PDFPage(verify.value());
            api.parsePageBlocked();
            api.native_SectionRecognize(verify.value());
            check(api.native_OnLButtonDown(verify.value(), 90f, 674f, 0)
                            && "EDITED FORM".equals(api.native_GetText(verify.value()).trim()),
                    "rebuilt Form saved first instance");
            check(api.native_OnLButtonDown(verify.value(), 318f, 674f, 0)
                            && "SHARED FORM TEXT".equals(api.native_GetText(verify.value()).trim()),
                    "rebuilt Form saved same-page isolation");
            api.native_unloadPage2(verify.value());
            NativeHandle verifyOther = new NativeHandle();
            check(document.native_getPage(reopened.value(), 1, verifyOther) == 0,
                    "rebuilt Form saved other page");
            cn.wps.moffice.pdf.core.std.PDFPage otherApi =
                    new cn.wps.moffice.pdf.core.std.PDFPage(verifyOther.value());
            otherApi.parsePageBlocked();
            otherApi.native_SectionRecognize(verifyOther.value());
            check(otherApi.native_OnLButtonDown(verifyOther.value(), 90f, 674f, 0)
                            && "SHARED FORM TEXT".equals(
                                    otherApi.native_GetText(verifyOther.value()).trim()),
                    "rebuilt Form saved cross-page isolation");
            otherApi.native_unloadPage2(verifyOther.value());
            document.native_closePDF(reopened.value());
            return;
        }
        if (page0.value() != 0) {
            new cn.wps.moffice.pdf.core.std.PDFPage(page0.value())
                    .native_unloadPage2(page0.value());
        }
        document.native_closePDF(doc.value());
    }

    String extractPageText(long page) {
        PDFTextPage api = new PDFTextPage();
        long textPage = api.native_new(page);
        return textPage == 0 ? null : api.native_getPageText(textPage);
    }

    void probeGlyphObject(String label, cn.wps.moffice.pdf.core.std.PDFPage api,
                          long page, float focusX, float focusY, float[][] points,
                          int expectedLength) {
        boolean focused = api.native_OnLButtonDown(page, focusX, focusY, 0);
        api.native_OnLButtonUp(page, focusX, focusY);
        int length = api.native_GetEditAllTextLength(page);
        out(label + "Focus=" + focused + ",type=" + api.native_GetFocusType(page)
                + ",text=" + escaped(api.native_GetEditAllText(page)) + ",length=" + length);
        for (float[] point : points) {
            out(label + "Hit[" + point[0] + "," + point[1] + "]="
                    + api.native_GetRangeIndexAtPos(page, point[0], point[1]));
        }
        int[] carets = length > 0 ? new int[] {0, 1, 2, length / 2, length}
                : new int[] {0, 1, 2, expectedLength / 2, expectedLength};
        for (int position : carets) {
            boolean set = api.native_SetCaret(page, position);
            RectF rect = new RectF();
            api.native_GetCaretInfo(page, rect);
            out(label + "Caret[" + position + "]=" + set + ",get="
                    + api.native_GetCaret(page) + ",rect=" + rect + ",points="
                    + pointsString(api.native_GetCaretPoints(page)));
        }
    }

    void probePageObjectInventory(String prefix, PDFDocument document, String dir) throws Exception {
        String path = copyAsset("editable-objects.pdf", dir + "/editable-objects.pdf");
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(path, doc);
        out(prefix + "PageObjectOpen=" + opened);
        if (opened != 0) return;
        NativeHandle page = new NativeHandle();
        if (document.native_getPage(doc.value(), 0, page) != 0) return;
        cn.wps.moffice.pdf.core.std.PDFPage pageApi =
                new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
        pageApi.parsePageBlocked();
        long[] images = pageApi.native_getImages(page.value());
        out(prefix + "ImageCount=" + (images == null ? -1 : images.length));
        if (images != null && images.length > 0) {
            RectF rect = new RectF();
            pageApi.native_getImageRect(page.value(), images[0], rect);
            long hit = pageApi.native_getImageObjectAtPoint(page.value(), 225, 620);
            out(prefix + "Image0=" + images[0] + ",rect=" + rect + ",hit=" + hit);
        }
        long pathHandle = pageApi.native_getPathObjectAtPoint(page.value(), 140, 515);
        RectF pathRect = new RectF();
        if (pathHandle != 0) pageApi.native_getPathRect(page.value(), pathHandle, pathRect);
        out(prefix + "PathHit=" + pathHandle + ",rect=" + pathRect);
        if (prefix.equals("rebuilt")) {
            long[] paths = RuntimeProbe.nativePageObjectHandles(page.value(), 2);
            out(prefix + "PathInventoryCount=" + (paths == null ? -1 : paths.length));
        }
        pageApi.native_unloadPage2(page.value());
        document.native_closePDF(doc.value());
    }

    void probeImageDelete(String prefix, PDFDocument document, String dir) throws Exception {
        String path = copyAsset("editable-objects.pdf",
                dir + "/image-delete-source-" + prefix + ".pdf");
        String savedPath = dir + "/image-delete-roundtrip-" + prefix + ".pdf";
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(path, doc);
        out(prefix + "ImageDelete.open=" + opened);
        if (opened != 0) return;
        document.native_setEditStatus(doc.value(), 6);
        NativeHandle page = new NativeHandle();
        int loaded = document.native_getPage(doc.value(), 0, page);
        out(prefix + "ImageDelete.page=" + loaded);
        if (loaded != 0) {
            document.native_closePDF(doc.value());
            return;
        }
        cn.wps.moffice.pdf.core.std.PDFPage api =
                new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
        api.parsePageBlocked();
        long[] before = api.native_getImages(page.value());
        boolean removed = before != null && before.length == 1
                && api.native_removeImageFromPage(page.value(), before[0]);
        long[] after = api.native_getImages(page.value());
        out(prefix + "ImageDelete.removed=" + removed + ",before="
                + (before == null ? -1 : before.length) + ",after="
                + (after == null ? -1 : after.length) + ",dirty="
                + api.native_CheckFuncModified(page.value(), 6));
        check(removed && (after == null || after.length == 0),
                prefix + " image delete removes object");

        long command = 0;
        if (prefix.equals("rebuilt")) {
            command = RuntimeProbe.nativeTakePendingEditUndo(page.value());
            check(command != 0, prefix + " image delete undo command");
            PDFFormFill.native_undoEdit(command);
            long[] restored = api.native_getImages(page.value());
            check(restored != null && restored.length == 1,
                    prefix + " image delete undo restore");
            PDFFormFill.native_redoEdit(command);
            long[] redone = api.native_getImages(page.value());
            check(redone == null || redone.length == 0, prefix + " image delete redo remove");
        }

        check(document.native_syncEditPageObj(doc.value(), page.value(), true),
                prefix + " image delete sync");
        api.native_unloadPage2(page.value());
        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        boolean saved = session != 0 && saver.start(session, savedPath)
                && saver.run(session, 500) == 100;
        check(saved, prefix + " image delete save");
        if (session != 0) saver.destroy(session);
        if (command != 0) PDFFormFill.native_deleteTransEdit(command);
        document.native_closePDF(doc.value());

        if (saved) {
            NativeHandle reopened = new NativeHandle();
            check(PDFDocument.native_openPDF(savedPath, reopened) == 0,
                    prefix + " image delete reopen");
            NativeHandle reopenedPage = new NativeHandle();
            check(document.native_getPage(reopened.value(), 0, reopenedPage) == 0,
                    prefix + " image delete reopen page");
            cn.wps.moffice.pdf.core.std.PDFPage verify =
                    new cn.wps.moffice.pdf.core.std.PDFPage(reopenedPage.value());
            long[] persisted = verify.native_getImages(reopenedPage.value());
            check(persisted == null || persisted.length == 0,
                    prefix + " image delete persisted");
            verify.native_unloadPage2(reopenedPage.value());
            document.native_closePDF(reopened.value());
        }
    }

    void probeOriginalImageGeometry(PDFDocument document, String dir) throws Exception {
        String path = copyAsset("editable-objects.pdf", dir + "/original-image-geometry.pdf");
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(path, doc);
        out("originalImageGeometry.open=" + opened);
        if (opened != 0) return;
        document.native_setEditStatus(doc.value(), 6);
        NativeHandle page = new NativeHandle();
        int loaded = document.native_getPage(doc.value(), 0, page);
        out("originalImageGeometry.page=" + loaded);
        if (loaded == 0) {
            cn.wps.moffice.pdf.core.std.PDFPage api =
                    new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
            api.parsePageBlocked();
            long[] images = api.native_getImages(page.value());
            if (images != null && images.length == 1) {
                long image = images[0];
                RectF before = new RectF();
                api.native_getImageRect(page.value(), image, before);
                int degree0 = api.native_getImageDegree(page.value(), image);
                float opacity0 = api.native_getImageOpacity(page.value(), image);
                RectF target = new RectF(210f, 590f, 310f, 650f);
                int resized = api.native_resizeImageRect(page.value(), image, target);
                RectF afterResize = new RectF();
                api.native_getImageRect(page.value(), image, afterResize);
                int rotated = api.native_rotateImageClockwise(page.value(), image);
                RectF afterRotate = new RectF();
                api.native_getImageRect(page.value(), image, afterRotate);
                int degree1 = api.native_getImageDegree(page.value(), image);
                int reversed = api.native_reverseImageHorizontal(page.value(), image);
                RectF afterReverse = new RectF();
                api.native_getImageRect(page.value(), image, afterReverse);
                int degree2 = api.native_getImageDegree(page.value(), image);
                boolean opacitySet = api.native_setImageOpacity(page.value(), image, 0.4f);
                float opacity1 = api.native_getImageOpacity(page.value(), image);
                boolean degreeSet = api.native_setImageDegree(page.value(), image, 180);
                int degree3 = api.native_getImageDegree(page.value(), image);
                RectF afterDegree = new RectF();
                api.native_getImageRect(page.value(), image, afterDegree);
                out("originalImageGeometry.before=" + before + ",degree=" + degree0
                        + ",opacity=" + opacity0 + ",resize=" + resized + ":" + afterResize
                        + ",rotate=" + rotated + ":" + afterRotate + ",degree=" + degree1
                        + ",reverse=" + reversed + ":" + afterReverse + ",degree=" + degree2
                        + ",opacitySet=" + opacitySet + ",opacity=" + opacity1
                        + ",degreeSet=" + degreeSet + ",degree=" + degree3
                        + ",rect=" + afterDegree);
            }
            api.native_unloadPage2(page.value());
        }
        document.native_closePDF(doc.value());
    }

    void probeRebuiltImageGeometry(PDFDocument document, String dir) throws Exception {
        String path = copyAsset("editable-objects.pdf", dir + "/rebuilt-image-geometry.pdf");
        String savedPath = dir + "/rebuilt-image-geometry-roundtrip.pdf";
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(path, doc) == 0, "rebuilt image geometry open");
        document.native_setEditStatus(doc.value(), 6);
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0,
                "rebuilt image geometry page");
        cn.wps.moffice.pdf.core.std.PDFPage api =
                new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
        api.parsePageBlocked();
        long image = api.native_getImages(page.value())[0];
        RectF target = new RectF(210f, 590f, 310f, 650f);
        check(api.native_resizeImageRect(page.value(), image, target) == 1,
                "rebuilt image resize");
        long resizeUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        image = api.native_getImages(page.value())[0];
        RectF resized = new RectF();
        api.native_getImageRect(page.value(), image, resized);
        check(resized.equals(target), "rebuilt image resize bounds");

        check(api.native_rotateImageClockwise(page.value(), image) == 1,
                "rebuilt image rotate clockwise");
        long rotateUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        image = api.native_getImages(page.value())[0];
        RectF rotated = new RectF();
        api.native_getImageRect(page.value(), image, rotated);
        RectF rotatedExpected = new RectF(230f, 570f, 290f, 670f);
        check(rotated.equals(rotatedExpected), "rebuilt image rotated bounds");
        check(api.native_reverseImageHorizontal(page.value(), image) == 1,
                "rebuilt image reverse horizontal");
        long reverseUndo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        image = api.native_getImages(page.value())[0];
        RectF reversed = new RectF();
        api.native_getImageRect(page.value(), image, reversed);
        check(reversed.equals(rotatedExpected), "rebuilt image reverse preserves bounds");
        PDFFormFill.native_undoEdit(reverseUndo);
        PDFFormFill.native_undoEdit(rotateUndo);
        image = api.native_getImages(page.value())[0];
        RectF undone = new RectF();
        api.native_getImageRect(page.value(), image, undone);
        check(undone.equals(target), "rebuilt image rotate undo");
        PDFFormFill.native_redoEdit(rotateUndo);
        image = api.native_getImages(page.value())[0];
        RectF redone = new RectF();
        api.native_getImageRect(page.value(), image, redone);
        check(redone.equals(rotatedExpected), "rebuilt image rotate redo");

        check(document.native_syncEditPageObj(doc.value(), page.value(), true),
                "rebuilt image geometry sync");
        api.native_unloadPage2(page.value());
        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        check(session != 0 && saver.start(session, savedPath) && saver.run(session, 500) == 100,
                "rebuilt image geometry save");
        saver.destroy(session);
        PDFFormFill.native_deleteTransEdit(reverseUndo);
        PDFFormFill.native_deleteTransEdit(rotateUndo);
        PDFFormFill.native_deleteTransEdit(resizeUndo);
        document.native_closePDF(doc.value());

        NativeHandle reopened = new NativeHandle();
        check(PDFDocument.native_openPDF(savedPath, reopened) == 0,
                "rebuilt image geometry reopen");
        NativeHandle reopenedPage = new NativeHandle();
        check(document.native_getPage(reopened.value(), 0, reopenedPage) == 0,
                "rebuilt image geometry reopen page");
        cn.wps.moffice.pdf.core.std.PDFPage verify =
                new cn.wps.moffice.pdf.core.std.PDFPage(reopenedPage.value());
        long savedImage = verify.native_getImages(reopenedPage.value())[0];
        RectF saved = new RectF();
        verify.native_getImageRect(reopenedPage.value(), savedImage, saved);
        check(saved.equals(rotatedExpected), "rebuilt image geometry persisted");
        long[] savedPaths = RuntimeProbe.nativePageObjectHandles(reopenedPage.value(), 2);
        check(savedPaths != null && savedPaths.length == 1,
                "rebuilt image geometry preserves path");
        verify.native_unloadPage2(reopenedPage.value());
        document.native_closePDF(reopened.value());
    }

    static String escaped(String value) {
        if (value == null) return "<null>";
        StringBuilder result = new StringBuilder();
        for (int i = 0; i < value.length(); i++) {
            char c = value.charAt(i);
            if (c == '\n') result.append("\\n");
            else if (c == '\r') result.append("\\r");
            else if (c == '\t') result.append("\\t");
            else if (c < 0x20 || c > 0x7e) result.append(String.format("\\u%04x", (int)c));
            else result.append(c);
        }
        return result.toString();
    }

    void probeEditDomainCorpus(String prefix, PDFDocument document, String dir) throws Exception {
        String[] names = {"multicolumn", "rotated", "cropbox", "form-xobject"};
        for (String name : names) {
            String path = copyAsset("edit-domain-corpus/" + name + ".pdf",
                    dir + "/edit-domain-" + name + ".pdf");
            NativeHandle doc = new NativeHandle();
            int opened = PDFDocument.native_openPDF(path, doc);
            out(prefix + "DomainCorpus[" + name + "].open=" + opened);
            if (opened != 0) continue;
            NativeHandle page = new NativeHandle();
            int loaded = document.native_getPage(doc.value(), 0, page);
            out(prefix + "DomainCorpus[" + name + "].page=" + loaded);
            if (loaded == 0) {
                cn.wps.moffice.pdf.core.std.PDFPage api =
                        new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
                api.parsePageBlocked();
                RectF size = api.getPageSize();
                float[] display = new float[9];
                int displayResult = api.native_getDisplayMatirx(
                        page.value(), display, new RectF(size), 0);
                out(prefix + "DomainCorpus[" + name + "].display=" + displayResult
                        + ",m=" + display[0] + "," + display[1] + "," + display[2]
                        + "," + display[3] + "," + display[4] + "," + display[5]
                        + "," + display[6] + "," + display[7] + "," + display[8]);
                document.native_setEditStatus(doc.value(), 1);
                boolean recognized = api.native_SectionRecognize(page.value());
                int count = api.native_getTextEditorCount(page.value());
                out(prefix + "DomainCorpus[" + name + "].recognized=" + recognized
                        + ",count=" + count + ",size=" + size);
                for (int i = 0; i < Math.min(count, 20); i++) {
                    RectF rect = new RectF();
                    float[] matrix = new float[9];
                    boolean ok = api.native_getTextEditorRect(
                            page.value(), i, rect, matrix, false);
                    out(prefix + "DomainCorpus[" + name + "][" + i + "]=" + ok
                            + "," + rect + ",m=" + matrix[0] + "," + matrix[1]
                            + "," + matrix[2] + "," + matrix[3] + "," + matrix[4]
                            + "," + matrix[5] + "," + matrix[6] + "," + matrix[7]
                            + "," + matrix[8]);
                }
                api.native_unloadPage2(page.value());
            }
            document.native_closePDF(doc.value());
        }
    }

    /**
     * WT5d oracle: probe the original editor at source-document coordinates.
     * The fixtures are deterministic and the points deliberately avoid a
     * Java-side reconstruction of the editor matrix, so both SOs receive the
     * same page-space gesture and character stream.
     */
    void probeAdvancedTextReflow(String prefix, PDFDocument document, String dir)
            throws Exception {
        String[] names = {"rotated", "form-xobject"};
        float[] xs = {300f, 100f};
        float[] ys = {420f, 600f};
        String overflow = " one two three four five six seven eight nine ten eleven twelve";
        for (int fixture = 0; fixture < names.length; fixture++) {
            String name = names[fixture];
            String path = copyAsset("edit-domain-corpus/" + name + ".pdf",
                    dir + "/advanced-reflow-" + prefix + "-" + name + ".pdf");
            NativeHandle doc = new NativeHandle();
            int opened = PDFDocument.native_openPDF(path, doc);
            out(prefix + "AdvancedReflow[" + name + "].open=" + opened);
            if (opened != 0) continue;
            document.native_setEditStatus(doc.value(), 1);
            NativeHandle page = new NativeHandle();
            if (document.native_getPage(doc.value(), 0, page) == 0) {
                cn.wps.moffice.pdf.core.std.PDFPage api =
                        new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
                api.parsePageBlocked();
                boolean recognized = api.native_SectionRecognize(page.value());
                int countBefore = api.native_getTextEditorCount(page.value());
                boolean focused = api.native_OnLButtonDown(
                        page.value(), xs[fixture], ys[fixture], 0);
                int focusType = api.native_GetFocusType(page.value());
                String beforeText = api.native_GetEditAllText(page.value());
                int beforeLength = api.native_GetEditAllTextLength(page.value());
                // native_SetCaret changes the active rotated editor in the
                // original SO. Preserve the caret chosen by its hit-test for
                // that fixture; use a deterministic insertion point only for
                // the Form paragraph.
                int requestedCaret = fixture == 0 ? api.native_GetCaret(page.value()) : 15;
                boolean caretSet = focused && (fixture == 0 ||
                        api.native_SetCaret(page.value(), requestedCaret));
                int caret = api.native_GetCaret(page.value());
                RectF beforeRect = new RectF();
                boolean beforeRectOk = false;
                // Original and rebuilt recognition enumerate rotated editors
                // differently. Resolve the 25° editor by its native matrix;
                // index-based comparison would accidentally select the 90°
                // fixture in the original SO.
                int focusedIndex = fixture == 0
                        ? closestRotatedEditor(api, page.value(), countBefore, 25f) : 0;
                beforeRectOk = focusedIndex < countBefore &&
                        api.native_getTextEditorRect(page.value(), focusedIndex, beforeRect,
                                new float[9], false);
                RectF beforeFocusRect = new RectF();
                boolean beforeFocusRectOk = api.native_GetFocusObjectRect(
                        page.value(), beforeFocusRect);
                boolean inserted = focused && focusType == 1 &&
                        api.native_OnChars(page.value(), overflow, false,
                                Math.max(0, caret) + overflow.length());
                int countAfter = api.native_getTextEditorCount(page.value());
                RectF afterRect = new RectF();
                boolean afterRectOk = focusedIndex >= 0 &&
                        api.native_getTextEditorRect(page.value(), focusedIndex, afterRect,
                                new float[9], false);
                RectF afterFocusRect = new RectF();
                boolean afterFocusRectOk = api.native_GetFocusObjectRect(
                        page.value(), afterFocusRect);
                out(prefix + "AdvancedReflow[" + name + "].recognized=" + recognized +
                        ",count=" + countBefore + "->" + countAfter + ",focus=" + focused +
                        ",focusType=" + focusType + ",len=" + beforeLength +
                        ",caretSet=" + caretSet + ",caret=" + caret +
                        ",text=" + escaped(beforeText) +
                        ",insert=" + inserted + ",rect=" + beforeRectOk + ":" + beforeRect +
                        "->" + afterRectOk + ":" + afterRect +
                        ",focusRect=" + beforeFocusRectOk + ":" + beforeFocusRect +
                        "->" + afterFocusRectOk + ":" + afterFocusRect);
                if (prefix.equals("rebuilt") && name.equals("form-xobject")) {
                    String[] diagnostics = RuntimeProbe.nativeTextObjectDiagnostics(page.value());
                    if (diagnostics != null) for (String value : diagnostics)
                        out("rebuiltAdvancedReflow.formObject=" + value);
                }
                api.native_unloadPage2(page.value());
            }
            document.native_closePDF(doc.value());
        }
    }

    /** WT6a device oracle: CJK insertion falls back to an encodable page font. */
    void probeComplexTextReflow(String prefix, PDFDocument document, String dir)
            throws Exception {
        String path = copyAsset("complex-text-corpus.pdf",
                dir + "/complex-text-" + prefix + ".pdf");
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(path, doc);
        out(prefix + "ComplexText.open=" + opened);
        if (opened != 0) return;
        document.native_setEditStatus(doc.value(), 1);
        NativeHandle page = new NativeHandle();
        int loaded = document.native_getPage(doc.value(), 0, page);
        out(prefix + "ComplexText.page=" + loaded);
        if (loaded == 0) {
            cn.wps.moffice.pdf.core.std.PDFPage api =
                    new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
            api.parsePageBlocked();
            boolean recognized = api.native_SectionRecognize(page.value());
            boolean focused = api.native_OnLButtonDown(page.value(), 100f, 700f, 0);
            String before = api.native_GetEditAllText(page.value());
            String inserted = "新增中文内容用于自动换行新增中文内容";
            int caret = before == null ? -1 : before.indexOf("prefix");
            caret = caret < 0 ? 14 : caret + "prefix".length();
            boolean changed = focused && api.native_SetTextSelectRange(
                    page.value(), caret, caret) && api.native_OnChars(
                    page.value(), inserted, false, caret + inserted.length());
            String after = api.native_GetEditAllText(page.value());
            String compact = after == null ? "" : after.replace("\r", "").replace("\n", "");
            out(prefix + "ComplexText.recognized=" + recognized + ",focus=" + focused
                    + ",caret=" + caret + ",insert=" + changed + ",before="
                    + escaped(before) + ",after=" + escaped(after));
            if (prefix.equals("rebuilt")) {
                check(recognized && focused && changed && compact.contains(inserted),
                        "rebuilt CJK fallback insertion");
                String[] diagnostics = RuntimeProbe.nativeTextObjectDiagnostics(page.value());
                if (diagnostics != null) for (String value : diagnostics)
                    out("rebuiltComplexText.object=" + value);
                long undo = RuntimeProbe.nativeTakePendingEditUndo(page.value());
                check(undo != 0, "rebuilt CJK undo command");
                if (undo != 0) {
                    PDFFormFill.native_undoEdit(undo);
                    check(before.equals(api.native_GetEditAllText(page.value())),
                            "rebuilt CJK undo");
                    PDFFormFill.native_redoEdit(undo);
                    check(api.native_GetEditAllText(page.value()).replace("\r", "")
                                    .replace("\n", "").contains(inserted),
                            "rebuilt CJK redo");
                    PDFFormFill.native_deleteTransEdit(undo);
                }
            }
            api.native_unloadPage2(page.value());
        }
        document.native_closePDF(doc.value());
    }

    /** WT6b oracle for the original load/support/map font-host sequence. */
    void probeExternalFontFallback(String prefix, PDFDocument document, String dir)
            throws Exception {
        String fontPath = copyAsset("fonts/NotoSansSC-WT6-subset.ttf",
                dir + "/NotoSansSC-WT6-subset.ttf");
        PDFTextEditor editor = new PDFTextEditor();
        boolean loadedFont = PDFFontMgr.native_loadPrivateFont(fontPath);
        boolean loadedDirectory = PDFFontMgr.native_loadPrivateFonts(dir);
        editor.native_addSupportReplaceFont("Helvetica");
        boolean mappedFont = editor.native_setReplaceDisplayFont(
                "Helvetica", "NotoSansSC-Thin");
        out(prefix + "ExternalFont.loaded=" + loadedFont + ",directory="
                + loadedDirectory + ",mapped=" + mappedFont);
        String path = copyAsset("mixed-reflow-corpus.pdf",
                dir + "/external-font-" + prefix + ".pdf");
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(path, doc);
        out(prefix + "ExternalFont.open=" + opened);
        if (opened != 0) return;
        document.native_setEditStatus(doc.value(), 1);
        NativeHandle page = new NativeHandle();
        int pageResult = document.native_getPage(doc.value(), 0, page);
        out(prefix + "ExternalFont.page=" + pageResult);
        if (pageResult == 0) {
            cn.wps.moffice.pdf.core.std.PDFPage api =
                    new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
            api.parsePageBlocked();
            boolean recognized = api.native_SectionRecognize(page.value());
            boolean focused = api.native_OnLButtonDown(page.value(), 100f, 700f, 0);
            String before = api.native_GetEditAllText(page.value());
            String inserted = "新增中文内容用于自动换行新增中文内容";
            int caret = before == null ? -1 : before.indexOf("Regular");
            caret = caret < 0 ? 7 : caret + "Regular".length();
            boolean insertedOk = focused && api.native_SetTextSelectRange(
                    page.value(), caret, caret) && api.native_OnChars(
                    page.value(), inserted, false, caret + inserted.length());
            String after = api.native_GetEditAllText(page.value());
            String compact = after == null ? "" : after.replace("\r", "").replace("\n", "");
            out(prefix + "ExternalFont.recognized=" + recognized + ",focus=" + focused
                    + ",insert=" + insertedOk + ",before=" + escaped(before)
                    + ",after=" + escaped(after));
            if (prefix.equals("rebuilt")) {
                check(loadedFont && loadedDirectory && mappedFont && recognized && focused && insertedOk
                                && compact.contains(inserted),
                        "rebuilt external font fallback");
                String[] diagnostics = RuntimeProbe.nativeTextObjectDiagnostics(page.value());
                boolean cjkObject = false;
                if (diagnostics != null) for (String value : diagnostics) {
                    out("rebuiltExternalFont.object=" + value);
                    if (value.contains("fontName=NotoSansSC") && value.contains("text=新增"))
                        cjkObject = true;
                }
                check(cjkObject, "rebuilt external font TextObject");
            }
            api.native_unloadPage2(page.value());
        }
        document.native_closePDF(doc.value());
    }

    /** WT6c1 device check for original-compatible emoji rejection and domain deletion. */
    void probeUnicodeEditBoundary(PDFDocument document, String dir) throws Exception {
        String path = copyAsset("mixed-reflow-corpus.pdf", dir + "/unicode-boundary.pdf");
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(path, doc) == 0, "unicode open");
        document.native_setEditStatus(doc.value(), 1);
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0, "unicode page");
        cn.wps.moffice.pdf.core.std.PDFPage api =
                new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
        api.parsePageBlocked();
        check(api.native_SectionRecognize(page.value()), "unicode recognize");
        check(api.native_OnLButtonDown(page.value(), 100f, 700f, 0), "unicode focus");
        String original = api.native_GetEditAllText(page.value());
        int caret = original.indexOf("Regular") + "Regular".length();
        check(api.native_SetTextSelectRange(page.value(), caret, caret), "unicode caret");
        check(!api.native_OnChars(page.value(), "\ud83d\ude00", false, caret + 2),
                "unicode reject emoji");
        check(original.equals(api.native_GetEditAllText(page.value())),
                "unicode rejected text unchanged");
        int later = original.indexOf("Second") + "Second".length();
        check(api.native_SetTextSelectRange(page.value(), later, later),
                "unicode later-run caret");
        check(api.native_OnKeyDown(page.value(), 67, 0), "unicode later-run backspace");
        check(api.native_GetEditAllText(page.value()).contains("Secon delta"),
                "unicode later-run result");
        api.native_unloadPage2(page.value());
        document.native_closePDF(doc.value());
    }

    int closestRotatedEditor(cn.wps.moffice.pdf.core.std.PDFPage api,
                             long page, int count, float targetDegrees) {
        int best = -1;
        float bestDistance = Float.MAX_VALUE;
        for (int index = 0; index < count; index++) {
            RectF rect = new RectF();
            float[] matrix = new float[9];
            if (!api.native_getTextEditorRect(page, index, rect, matrix, false)) continue;
            float degrees = (float) Math.toDegrees(Math.atan2(matrix[3], matrix[0]));
            float distance = Math.abs(Math.abs(degrees) - Math.abs(targetDegrees));
            if (distance < bestDistance) {
                bestDistance = distance;
                best = index;
            }
        }
        return best;
    }

    /**
     * Optional real-document oracle.  The file is deliberately not bundled in
     * the APK: push it to getExternalFilesDir as real-edit-paper.pdf before a
     * run.  This keeps the contract reproducible by SHA-256 without making a
     * third-party/user document part of the source tree.
     */
    // Isolate recognition from the mutation/font probes in the full contract.
    void runPaper(String mode) throws Exception {
        boolean original = mode.equals("paper-original");
        System.loadLibrary(original ? "kwopdf" : "kspfreetext");
        if (original) check(new PDFModuleMgr().initialize() == 0, "paper module initialize");
        PDFDocument document = new PDFDocument();
        out("paper.algGen.default=" + document.native_GetSectionRecognizeAlgGen());
        int generation = getIntent().getIntExtra("algGen", 0);
        if (generation != 0) {
            check(document.native_SetSectionRecognizeAlgGen(generation),
                    "set algorithm generation");
            out("paper.algGen.actual=" + document.native_GetSectionRecognizeAlgGen());
        }
        probeExternalPaper(original ? "original" : "rebuilt", document,
                getExternalFilesDir(null).getAbsolutePath());
    }

    void probeExternalPaper(String prefix, PDFDocument document, String dir) throws Exception {
        File file = new File(dir, "real-edit-paper.pdf");
        if (!file.isFile()) {
            out(prefix + "RealPaper=SKIP");
            return;
        }
        NativeHandle doc = new NativeHandle();
        int opened = PDFDocument.native_openPDF(file.getAbsolutePath(), doc);
        out(prefix + "RealPaper.open=" + opened + ",bytes=" + file.length());
        if (opened != 0) return;
        int paperPageCount = document.native_getPageCount(doc.value());
        out(prefix + "RealPaper.pageCount=" + paperPageCount);
        int selectedPage = getIntent().getIntExtra("paperPage", 0);
        int warmupFrom = getIntent().getIntExtra("paperWarmupFrom", 1);
        int warmupThrough = getIntent().getIntExtra("paperWarmupThrough", 0);
        boolean recognizeWarmup = getIntent().getBooleanExtra(
                "paperRecognizeWarmup", true);
        int paperEditStatus = getIntent().getIntExtra("paperEditStatus", 1);
        int paperFocusEditStatus = getIntent().getIntExtra(
                "paperFocusEditStatus", paperEditStatus);
        check(selectedPage >= 0 && selectedPage <= paperPageCount, "paper page range");
        check(warmupFrom >= 1 && warmupFrom <= Math.max(warmupThrough, 1),
                "paper warmup start range");
        check(warmupThrough >= 0 && warmupThrough < Math.max(selectedPage, 1),
                "paper warmup range");
        for (int pageIndex = 0; pageIndex < paperPageCount; pageIndex++) {
            boolean selected = selectedPage == 0 || pageIndex == selectedPage - 1;
            boolean warmup = selectedPage > 0 && pageIndex >= warmupFrom - 1
                    && pageIndex < warmupThrough;
            if (!selected && !warmup) continue;
            NativeHandle page = new NativeHandle();
            int loaded = document.native_getPage(doc.value(), pageIndex, page);
            String pageKey = "RealPaper.page" + (pageIndex + 1);
            out(prefix + pageKey + "=" + loaded);
            if (loaded != 0) continue;
            cn.wps.moffice.pdf.core.std.PDFPage api =
                    new cn.wps.moffice.pdf.core.std.PDFPage(page.value());
            api.parsePageBlocked();
            document.native_setEditStatus(doc.value(), paperEditStatus);
            boolean recognized = !warmup || recognizeWarmup
                    ? api.native_SectionRecognize(page.value()) : false;
            int count = api.native_getTextEditorCount(page.value());
            out(prefix + pageKey + ".recognized=" + recognized + ",count=" + count
                    + (warmup ? ",warmupMode="
                            + (recognizeWarmup ? "recognize" : "parse-only") : ""));
            // Pages 2-4 are the user-visible geometry oracle.  All 88 page
            // counts still run so broad over/under-segmentation is visible
            // without flooding logcat with thousands of rectangles.
            if (selected || (selectedPage == 0 && pageIndex >= 1 && pageIndex <= 3))
                    for (int i = 0; i < count; i++) {
                RectF rect = new RectF();
                float[] matrix = new float[9];
                boolean ok = api.native_getTextEditorRect(
                        page.value(), i, rect, matrix, false);
                out(prefix + pageKey + ".domain[" + i + "]=" + ok + "," + rect
                        + ",m=" + matrix[0] + "," + matrix[1] + "," + matrix[2]
                        + "," + matrix[3] + "," + matrix[4] + "," + matrix[5]);
            }
            float hitX = getIntent().getFloatExtra("paperHitX", -100000.0f);
            float hitY = getIntent().getFloatExtra("paperHitY", -100000.0f);
            if (selected && paperFocusEditStatus != paperEditStatus)
                document.native_setEditStatus(doc.value(), paperFocusEditStatus);
            if (selected && hitX > -99999.0f && hitY > -99999.0f) {
                boolean focused = api.native_OnLButtonDown(page.value(), hitX, hitY, 0);
                RectF focusRect = new RectF();
                boolean focusRectOk = focused &&
                        api.native_GetFocusObjectRect(page.value(), focusRect);
                String focusText = focused ? api.native_GetEditAllText(page.value()) : null;
                out(prefix + pageKey + ".hit=" + hitX + "," + hitY
                        + ",focused=" + focused + ",rect=" + focusRectOk + ":" + focusRect
                        + ",textLength=" + (focusText == null ? -1 : focusText.length()));
            }
            boolean probeEditorHits = getIntent().getBooleanExtra(
                    "paperProbeEditorHits", false);
            if (selected && probeEditorHits) {
                for (int editorIndex = 0; editorIndex < count; editorIndex++) {
                    RectF requestedRect = new RectF();
                    float[] requestedMatrix = new float[9];
                    boolean requestedOk = api.native_getTextEditorRect(
                            page.value(), editorIndex, requestedRect, requestedMatrix, false);
                    if (!requestedOk) {
                        out(prefix + pageKey + ".editorHit[" + editorIndex
                                + "]=rect=false");
                        continue;
                    }
                    float centerX = (requestedRect.left + requestedRect.right) * 0.5f;
                    float centerY = (requestedRect.top + requestedRect.bottom) * 0.5f;
                    boolean focused = api.native_OnLButtonDown(
                            page.value(), centerX, centerY, 0);
                    int downFocusType = api.native_GetFocusType(page.value());
                    int downFocusStatus = api.native_GetFocusStatus(page.value());
                    boolean released = focused && api.native_OnLButtonUp(
                            page.value(), centerX, centerY);
                    RectF focusRect = new RectF();
                    boolean focusRectOk = focused &&
                            api.native_GetFocusObjectRect(page.value(), focusRect);
                    String focusText = focused ? api.native_GetEditAllText(page.value()) : null;
                    String selectedText = focused ? api.native_GetText(page.value()) : null;
                    String focusPrefix = focusText == null ? "<null>" :
                            escaped(focusText.substring(0, Math.min(80, focusText.length())));
                    out(prefix + pageKey + ".editorHit[" + editorIndex + "]="
                            + "at:" + centerX + "," + centerY
                            + ",requested:" + requestedRect
                            + ",focused:" + focused
                            + ",released:" + released
                            + ",focusType:" + downFocusType
                            + ",focusStatus:" + downFocusStatus
                            + ",focusRect:" + focusRectOk + ":" + focusRect
                            + ",textLength:" + (focusText == null ? -1 : focusText.length())
                            + ",textHash:" + (focusText == null ? 0 : focusText.hashCode())
                            + ",selectedLength:"
                            + (selectedText == null ? -1 : selectedText.length())
                            + ",selectedHash:"
                            + (selectedText == null ? 0 : selectedText.hashCode())
                            + ",textPrefix:" + focusPrefix);
                }
            }
            if (prefix.equals("rebuilt") && selectedPage > 0 && selected) {
                boolean recognizedAgain = api.native_SectionRecognize(page.value());
                int repeatedCount = api.native_getTextEditorCount(page.value());
                out(prefix + pageKey + ".repeatRecognized=" + recognizedAgain
                        + ",count=" + repeatedCount);
                check(recognizedAgain && repeatedCount == count,
                        "rebuilt repeated recognition replaces page projection");
                int[] structure = RuntimeProbe.nativeRecognitionPageDiagnostics(
                        doc.value(), pageIndex);
                check(structure != null && structure.length == 10 &&
                                structure[0] == count && structure[2] == 1 &&
                                structure[5] > 0 &&
                                structure[1] == 1 + structure[0] +
                                        structure[4] + structure[5],
                        "rebuilt typed recognition hierarchy");
                if (structure != null && structure.length == 10) {
                    out(prefix + pageKey + ".structure=domains:" + structure[0]
                            + ",nodes:" + structure[1] + ",sections:" + structure[2]
                            + ",paragraphs:" + structure[3] + ",lines:" + structure[4]
                            + ",spans:" + structure[5] + ",formulas:" + structure[6]
                            + ",tables:" + structure[7] + ",rows:" + structure[8]
                            + ",cells:" + structure[9]);
                }
                int[] domains = RuntimeProbe.nativeRecognitionDomainDiagnostics(
                        doc.value(), pageIndex);
                check(domains != null && domains.length == count * 4,
                        "rebuilt recognition domain diagnostics");
                if (domains != null && domains.length == count * 4) {
                    StringBuilder formula = new StringBuilder();
                    for (int domainIndex = 0; domainIndex < count; domainIndex++) {
                        int offset = domainIndex * 4;
                        if (domains[offset] != 4) continue;
                        if (formula.length() > 0) formula.append(';');
                        formula.append(domainIndex).append(':')
                                .append(domains[offset + 1]).append('/')
                                .append(domains[offset + 2]).append('/')
                                .append(domains[offset + 3]);
                    }
                    out(prefix + pageKey + ".formulaDomains=" + formula);
                }
            }
            api.native_unloadPage2(page.value());
        }
        if (prefix.equals("rebuilt")) {
            long[] context = RuntimeProbe.nativeRecognitionContextDiagnostics(doc.value());
            check(context != null && context.length == 8,
                    "rebuilt recognition context diagnostics");
            if (context != null && context.length == 8) {
                out(prefix + "RecognitionContext=invocations:" + context[0]
                        + ",uniquePages:" + context[1] + ",lastPage:" + context[2]
                        + ",lastDomains:" + context[3] + ",generation:" + context[4]
                        + ",cachedPages:" + context[5] + ",cachedDomains:" + context[6]
                        + ",replacements:" + context[7]);
            }
            long[] processor = RuntimeProbe.nativeRecognitionProcessorDiagnostics(doc.value());
            check(processor != null && processor.length == 9,
                    "rebuilt recognition processor diagnostics");
            if (processor != null && processor.length == 9) {
                check(context != null && context.length == 8 &&
                                processor[0] == context[0] &&
                                processor[1] == context[5] &&
                                processor[7] == context[2] &&
                                processor[8] == context[4] &&
                                processor[2] > 0 && processor[3] > 0 &&
                                processor[4] >= processor[3] &&
                                processor[6] > 0,
                        "rebuilt processor commit consistency");
                out(prefix + "RecognitionProcessor=commits:" + processor[0]
                        + ",pages:" + processor[1] + ",formulaPages:" + processor[2]
                        + ",formulaDomains:" + processor[3] + ",formulaSpans:"
                        + processor[4] + ",compoundFormulas:" + processor[5]
                        + ",maxFormulaSpans:" + processor[6] + ",lastPage:"
                        + processor[7] + ",generation:" + processor[8]);
            }
        }
        document.native_closePDF(doc.value());
    }
}
