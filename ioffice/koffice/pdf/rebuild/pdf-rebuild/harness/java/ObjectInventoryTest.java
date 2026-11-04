import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public class ObjectInventoryTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static boolean finite(float[] values) {
        for (float value : values) if (!Float.isFinite(value)) return false;
        return true;
    }

    static float[] multiply(float[] p, float[] q) {
        return new float[] {
            p[0] * q[0] + p[1] * q[3], p[0] * q[1] + p[1] * q[4],
            p[0] * q[2] + p[1] * q[5] + p[2],
            p[3] * q[0] + p[4] * q[3], p[3] * q[1] + p[4] * q[4],
            p[3] * q[2] + p[4] * q[5] + p[5], 0, 0, 1};
    }

    public static void main(String[] args) {
        if (args.length != 3) throw new IllegalArgumentException(
                "editable, general, and page-object fixtures required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open editable fixture");
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0, "load editable page");
        PDFPage api = new PDFPage(page.value());
        api.native_onAfterLoadPage(page.value());
        check(RuntimeProbe.nativeLastErrorCode() == 0, "analysis succeeds");
        int count = api.native_getTextEditorCount(page.value());
        check(count == 0, "original no-op onAfterLoad does not recognize editors");
        check(api.native_SectionRecognize(page.value()), "section recognition succeeds");
        count = api.native_getTextEditorCount(page.value());
        check(count == 1, "two nearby text objects become one editor domain");
        long[] ids = RuntimeProbe.nativeTextObjectIds(page.value());
        check(ids != null && ids.length == 2 && ids[0] != 0 && ids[0] != ids[1],
              "inventory has distinct deterministic local IDs");

        RectF rect = new RectF();
        float[] forward = new float[9];
        check(api.native_getTextEditorRect(page.value(), 0, rect, forward, false),
              "read forward geometry");
        check(rect.left < rect.right && rect.bottom < rect.top, "local bounds are ordered");
        check(finite(forward) && forward[8] == 1, "forward matrix is finite affine matrix");
        check(api.native_OnLButtonDown(page.value(), 80f, 700f, 0),
              "mouse down focuses existing text object");
        check(api.native_GetFocusType(page.value()) == 1 &&
              api.native_GetFocusStatus(page.value()) == 2 &&
              api.native_GetFocusObjectCount(page.value()) == 1,
              "focused object reports original text focus contract");
        String originalText = api.native_GetText(page.value());
        check("Editable PDF text one".equals(originalText), "focused text readback");
        check(!api.native_SetTextSelectRange(page.value(), -1, originalText.length() + 1) &&
              RuntimeProbe.nativeLastErrorCode() == -1006,
              "out-of-range text selection is rejected");
        check(api.native_SetTextSelectRange(page.value(), originalText.length(), originalText.length()) &&
              api.native_OnChars(page.value(), "++", false, originalText.length() + 2),
              "insert text through OnChars");
        check((originalText + "++").equals(api.native_GetText(page.value())),
              "inserted text reads back from original object");
        check(api.native_SetTextSelectRange(page.value(), 0, 8) &&
              api.native_OnChars(page.value(), "Changed", false, 7),
              "replace selected text through OnChars");
        check(api.native_GetText(page.value()).startsWith("Changed"), "replacement reads back");
        int changedLength = api.native_GetTextLength(page.value());
        check(api.native_SetTextSelectRange(page.value(), changedLength - 2, changedLength) &&
              api.native_OnKeyDown(page.value(), 112, 0), "delete selected text through key path");
        check(!api.native_GetText(page.value()).endsWith("++"), "deleted text reads back");
        check(api.native_IsModified(page.value()) && api.native_CheckFuncModified(page.value(), 1),
              "text mutation sets page dirty function");
        RectF reverseRect = new RectF();
        float[] reverse = new float[9];
        check(api.native_getTextEditorRect(page.value(), 0, reverseRect, reverse, true),
              "read reverse geometry");
        float[] identity = multiply(forward, reverse);
        check(Math.abs(identity[0] - 1) < 0.0001f && Math.abs(identity[4] - 1) < 0.0001f &&
              Math.abs(identity[2]) < 0.001f && Math.abs(identity[5]) < 0.001f,
              "reverse matrix inverts forward matrix");
        check(!api.native_getTextEditorRect(page.value(), -1, new RectF(), new float[9], false),
              "negative index rejected");
        check(RuntimeProbe.nativeLastErrorCode() == -1006, "negative index is out of range");
        check(!api.native_getTextEditorRect(page.value(), count, new RectF(), new float[9], false),
              "past-end index rejected");
        check(!api.native_getTextEditorRect(page.value(), 0, new RectF(), new float[8], false),
              "short matrix array rejected");
        check(RuntimeProbe.nativeLastErrorCode() == -1001, "short matrix is invalid argument");

        check(document.native_syncEditPageObj(doc.value(), page.value(), false),
              "sync invalidates inventory safely");
        check(api.native_GetText(page.value()) == null &&
              RuntimeProbe.nativeLastErrorCode() == -1009,
              "sync clears focus and focused reads return not found");
        check(api.native_getTextEditorCount(page.value()) == 0,
              "sync clears recognized editor domains");
        check(api.native_SectionRecognize(page.value()) &&
              api.native_getTextEditorCount(page.value()) == count,
              "recognition rebuilds editor domains after sync");
        long[] rebuiltIds = RuntimeProbe.nativeTextObjectIds(page.value());
        check(rebuiltIds[0] == ids[0] && rebuiltIds[1] == ids[1],
              "unchanged object IDs survive inventory rebuild");
        check(api.native_unloadPage2(page.value()) == 0, "close editable page");
        check(api.native_getTextEditorCount(page.value()) == 0, "stale page count returns zero");
        check(RuntimeProbe.nativeLastErrorCode() == -1005, "stale page error retained");
        check(document.native_closePDF(doc.value()) == 0, "close editable document");

        NativeHandle generalDoc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[1], generalDoc) == 0, "open general fixture");
        NativeHandle generalPage = new NativeHandle();
        check(document.native_getPage(generalDoc.value(), 0, generalPage) == 0, "load general page");
        PDFPage generalApi = new PDFPage(generalPage.value());
        generalApi.native_onAfterLoadPage(generalPage.value());
        check(generalApi.native_SectionRecognize(generalPage.value()),
              "recognize general PDF sections");
        int generalCount = generalApi.native_getTextEditorCount(generalPage.value());
        check(generalCount > 0, "general PDF exposes text objects");
        check(document.native_closePDF(generalDoc.value()) == 0, "cascade close general document");

        NativeHandle objectDoc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[2], objectDoc) == 0, "open image/path fixture");
        NativeHandle objectPage = new NativeHandle();
        check(document.native_getPage(objectDoc.value(), 0, objectPage) == 0,
              "load image/path page");
        PDFPage objectApi = new PDFPage(objectPage.value());
        long[] images = objectApi.native_getImages(objectPage.value());
        long[] paths = RuntimeProbe.nativePageObjectHandles(objectPage.value(), 2);
        check(images != null && images.length == 1 && images[0] != 0,
              "image enumeration returns typed token");
        check(paths != null && paths.length == 1 && paths[0] != 0,
              "path joins unified object inventory");
        long[] imagesAgain = objectApi.native_getImages(objectPage.value());
        check(imagesAgain.length == 1 && imagesAgain[0] == images[0],
              "unchanged inventory reuses stable token");
        RectF imageRect = new RectF();
        objectApi.native_getImageRect(objectPage.value(), images[0], imageRect);
        check(imageRect.left == 200f && imageRect.right == 250f &&
              imageRect.top == 600f && imageRect.bottom == 640f,
              "image matrix becomes page-space rectangle");
        check(objectApi.native_getImageObjectAtPoint(objectPage.value(), 225f, 620f) == images[0],
              "image point lookup returns inventory token");
        RectF pathRect = new RectF();
        objectApi.native_getPathRect(objectPage.value(), paths[0], pathRect);
        check(pathRect.left == 100f && pathRect.right == 180f &&
              pathRect.top == 500f && pathRect.bottom == 530f,
              "path rectangle uses page coordinates");
        check(objectApi.native_getPathObjectAtPoint(objectPage.value(), 140f, 515f) == 0,
              "ordinary content path stays outside original editable-path selector");
        check(objectApi.native_FindImageObjectWithImageIndex(objectPage.value(), 1) == images[0] &&
              objectApi.native_GetImageIndex(objectPage.value(), images[0]) == 1,
              "original one-based image index contract preserved");
        NativeHandle siblingPage = new NativeHandle();
        check(document.native_getPage(objectDoc.value(), 0, siblingPage) == 0,
              "load independent page wrapper for owner check");
        new PDFPage(siblingPage.value()).native_getImageRect(
                siblingPage.value(), images[0], new RectF());
        check(RuntimeProbe.nativeLastErrorCode() == -1004,
              "page-object token cannot cross page wrappers");
        check(new PDFPage(siblingPage.value()).native_unloadPage2(siblingPage.value()) == 0,
              "close sibling page wrapper");
        objectApi.native_getPathRect(objectPage.value(), images[0], new RectF());
        check(RuntimeProbe.nativeLastErrorCode() == -1003,
              "image token rejected by path API as wrong type");
        check(document.native_syncEditPageObj(objectDoc.value(), objectPage.value(), false),
              "page sync invalidates object tokens");
        objectApi.native_getImageRect(objectPage.value(), images[0], new RectF());
        check(RuntimeProbe.nativeLastErrorCode() == -1005,
              "pre-sync image token is stale");
        long[] rebuiltImages = objectApi.native_getImages(objectPage.value());
        check(rebuiltImages.length == 1 && rebuiltImages[0] != images[0],
              "inventory rebuild advances token generation");
        check(objectApi.native_unloadPage2(objectPage.value()) == 0, "close object page");
        objectApi.native_getImageRect(objectPage.value(), rebuiltImages[0], new RectF());
        check(RuntimeProbe.nativeLastErrorCode() == -1005,
              "page close cascades object token invalidation");
        check(document.native_closePDF(objectDoc.value()) == 0, "close object document");
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "no leaked handles");

        System.out.println("OBJECT_INVENTORY PASS=" + pass + " FAIL=" + fail
                + " GENERAL_TEXT_OBJECTS=" + generalCount);
        if (fail != 0) System.exit(1);
    }
}
