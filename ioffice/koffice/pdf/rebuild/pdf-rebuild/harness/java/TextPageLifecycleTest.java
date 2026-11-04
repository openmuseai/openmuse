import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.select.PDFTextPage;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.pdf.lib.select.TextPage;
import cn.wps.rebuild.RuntimeProbe;
import java.util.ArrayList;

public class TextPageLifecycleTest {
    static int pass, fail;
    static void check(boolean value, String name) {
        if (value) pass++; else { fail++; System.out.println("FAIL: " + name); }
    }
    public static void main(String[] args) {
        if (args.length != 1) throw new IllegalArgumentException("fixture path required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open fixture");
        NativeHandle pageHandle = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, pageHandle) == 0, "load page");
        PDFTextPage api = new PDFTextPage();
        long textPage = api.native_new(pageHandle.value());
        check(textPage != 0, "create TextPage");
        check(api.native_new(pageHandle.value()) == textPage, "reuse one token per Page");
        check(api.native_new(doc.value()) == 0, "reject Document as Page");
        check(RuntimeProbe.nativeLastErrorCode() == -1003, "wrong type error retained");
        int count = TextPage.nCharCount(textPage);
        check(count > 0, "fixture has extractable characters");
        String all = api.native_getPageText(textPage);
        check(all != null && !all.isEmpty() && all.indexOf('\0') < 0,
              "full UTF-16 text without NUL");
        if (count > 0) {
            check(api.native_getUnicode(textPage, 0) >= 0, "Unicode at index");
            check(api.native_getText(textPage, 0, 0).length() <= 2, "inclusive one-char range");
            check(TextPage.nFontSize(textPage, 0) >= 0, "font size");
            TextPage.nColor(textPage, 0);
            TextPage.nSysFontName(textPage, 0);
            check(api.native_getUnicode(textPage, count) == -2, "Unicode OOB sentinel");
            check(RuntimeProbe.nativeLastErrorCode() == -1006, "Unicode OOB error");
            int rectCount = api.native_countRects(textPage, 0, count - 1);
            check(rectCount > 0, "range rectangles");
            if (rectCount > 0) {
                RectF rect = new RectF();
                api.native_getRect(textPage, 0, rect);
                check(rect.width() > 0 && Math.abs(rect.height()) > 0, "rectangle geometry");
                int hit = api.native_getCPAtPoint(textPage,
                        (rect.left + rect.right) / 2, (rect.top + rect.bottom) / 2, 1, 1);
                check(hit >= 0 && hit < count, "page-coordinate character hit");
                int[] selected = new int[2];
                int selectedCount = api.native_getCharRangeIndex(textPage,
                        rect.left, rect.top, rect.right, rect.bottom, selected, false);
                check(selectedCount > 0 && selected[0] >= 0 && selected[1] == selectedCount,
                      "rectangle-to-range");
            }
            // Original libkwopdf calls old CPDF_TextPage::GetIndexAtPos directly.
            // Verify the rebuilt JNI keeps its nearest-box behavior even when the
            // Java fuzzy path supplies page-sized tolerances.
            RectF[] charBoxes = new RectF[count];
            int probe = -1;
            for (int i = 0; i < count; i++) {
                if (api.native_countRects(textPage, i, i) <= 0) continue;
                RectF box = new RectF();
                api.native_getRect(textPage, 0, box);
                if (box.width() <= 0 || Math.abs(box.height()) <= 0) continue;
                charBoxes[i] = box;
                if (probe < 0 && i >= count / 3) probe = i;
            }
            if (probe >= 0) {
                RectF box = charBoxes[probe];
                float hitX = box.right + Math.max(2f, box.width() * 0.75f);
                float hitY = (box.top + box.bottom) / 2f;
                float tolerance = 10000f;
                int expected = legacyNearest(charBoxes, hitX, hitY, tolerance, tolerance);
                int actual = api.native_getCPAtPoint(textPage, hitX, hitY,
                        tolerance, tolerance);
                check(expected >= 0 && actual == expected,
                      "old SO fuzzy nearest-character semantics");
            }
            ArrayList<String> paragraphs = api.native_getText2(textPage, 0, count - 1);
            check(paragraphs != null && !paragraphs.isEmpty(), "paragraph text list");
            ArrayList<String> blocks = new ArrayList<>();
            ArrayList<RectF> blockRects = new ArrayList<>();
            api.native_getPageTextAndRects(textPage, blocks, blockRects);
            check(!blocks.isEmpty() && blocks.size() == blockRects.size(), "text/rect block pairs");
        }
        PDFPage page = new PDFPage(pageHandle.value());
        page.native_reloadTextPage(pageHandle.value());
        check(TextPage.nCharCount(textPage) == count, "reload preserves token and count");
        check(page.native_unloadPage2(pageHandle.value()) == 0, "close page subtree");
        check(TextPage.nCharCount(textPage) == 0, "stale TextPage rejected");
        check(RuntimeProbe.nativeLastErrorCode() == -1005, "stale TextPage error");
        check(document.native_closePDF(doc.value()) == 0, "close document");
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "registry empty");
        System.out.println("TEXT_PAGE_LIFECYCLE PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }

    static int legacyNearest(RectF[] boxes, float x, float y, float xt, float yt) {
        int near = -1;
        double bestX = 5000, bestY = 5000;
        for (int i = 0; i < boxes.length; i++) {
            RectF source = boxes[i];
            if (source == null) continue;
            float left = Math.min(source.left, source.right);
            float right = Math.max(source.left, source.right);
            float bottom = Math.min(source.top, source.bottom);
            float top = Math.max(source.top, source.bottom);
            if (x >= left && x <= right && y >= bottom && y <= top) return i;
            if (x < left - xt / 2 || x > right + xt / 2
                    || y < bottom - yt / 2 || y > top + yt / 2) continue;
            double dx = Math.min(Math.abs(x - left), Math.abs(x - right));
            double dy = Math.min(Math.abs(y - bottom), Math.abs(y - top));
            if (dx + dy < bestX + bestY) {
                bestX = dx; bestY = dy; near = i;
            }
        }
        return near;
    }
}
