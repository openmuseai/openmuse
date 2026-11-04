// TextPage differential probe: same ops against whichever lib the process loaded.
package cn.wps.rebuild;

import cn.wps.moffice.pdf.core.select.PDFTextPage;
import cn.wps.pdf.lib.select.TextPage;
import android.graphics.RectF;
import java.util.ArrayList;

public class TextPageProbe {
    static final String TAG = "KSPDIFF";

    static void out(String s) { android.util.Log.i(TAG, s); }

    public static void probe(String label, long page) {
        try {
            PDFTextPage tp = new PDFTextPage();
            long handle = tp.native_new(page);
            out("textpage[" + label + "] handle=" + handle);
            if (handle == 0) { out("textpage[" + label + "] FAIL new=0"); return; }
            int count = TextPage.nCharCount(handle);
            out("textpage[" + label + "] charCount=" + count);
            if (count <= 0) { out("textpage[" + label + "] FAIL charCount"); return; }
            int u0 = tp.native_getUnicode(handle, 0);
            int umid = tp.native_getUnicode(handle, count / 2);
            int ulast = tp.native_getUnicode(handle, count - 1);
            out("textpage[" + label + "] unicode[0]=" + u0 + " mid=" + umid + " last=" + ulast);
            String head = tp.native_getText(handle, 0, Math.min(59, count - 1));
            String pageText = tp.native_getPageText(handle);
            out("textpage[" + label + "] head='" + head.replace("\n", "\\n") + "' len=" + (head == null ? -1 : head.length()));
            out("textpage[" + label + "] pageTextLen=" + (pageText == null ? -1 : pageText.length()));
            float size0 = TextPage.nFontSize(handle, 0);
            int color0 = TextPage.nColor(handle, 0);
            String font0 = TextPage.nSysFontName(handle, 0);
            out("textpage[" + label + "] size0=" + size0 + " color0=" + Integer.toHexString(color0) + " font0='" + font0 + "'");
            int rectCount = tp.native_countRects(handle, 0, Math.min(count - 1, 59));
            out("textpage[" + label + "] rectCount=" + rectCount);
            if (rectCount > 0) {
                RectF rect = new RectF();
                int direction = tp.native_getRect(handle, 0, rect);
                int hit = tp.native_getCPAtPoint(handle, (rect.left + rect.right) / 2,
                        (rect.top + rect.bottom) / 2, 1, 1);
                int[] range = new int[2];
                int selected = tp.native_getCharRangeIndex(handle, rect.left, rect.top,
                        rect.right, rect.bottom, range, false);
                out("textpage[" + label + "] rect0=" + rect + " direction=" + direction
                        + " hit=" + hit + " bounded=" + selected + ":" + range[0] + "," + range[1]);
            }
            ArrayList<String> paragraphs = tp.native_getText2(handle, 0, Math.min(count - 1, 59));
            ArrayList<String> blocks = new ArrayList<>();
            ArrayList<RectF> blockRects = new ArrayList<>();
            tp.native_getPageTextAndRects(handle, blocks, blockRects);
            out("textpage[" + label + "] paragraphs=" + paragraphs.size()
                    + " blocks=" + blocks.size() + " blockRects=" + blockRects.size());
            boolean ok = count > 0 && u0 != 0 && head != null && head.length() > 0
                    && pageText != null && pageText.length() >= count - 1 && rectCount > 0
                    && !paragraphs.isEmpty() && blocks.size() == blockRects.size();
            out("textpage[" + label + "] " + (ok ? "PASS" : "FAIL"));
        } catch (Throwable t) {
            out("textpage[" + label + "] EXC " + t);
        }
    }
}
