import android.graphics.RectF;
import cn.wps.moffice.pdf.core.NativeHandle;
import cn.wps.moffice.pdf.core.formfill.PDFFormFill;
import cn.wps.moffice.pdf.core.io.PDFNativeSaver;
import cn.wps.moffice.pdf.core.std.PDFDocument;
import cn.wps.moffice.pdf.core.std.PDFPage;
import cn.wps.rebuild.RuntimeProbe;

public class ImageEditTest {
    static int pass;
    static int fail;

    static void check(boolean value, String name) {
        if (value) pass++;
        else { fail++; System.out.println("FAIL: " + name); }
    }

    static boolean sameRect(RectF lhs, RectF rhs) {
        return Math.abs(lhs.left - rhs.left) < 0.01f
                && Math.abs(lhs.top - rhs.top) < 0.01f
                && Math.abs(lhs.right - rhs.right) < 0.01f
                && Math.abs(lhs.bottom - rhs.bottom) < 0.01f;
    }

    static int count(long[] values) { return values == null ? 0 : values.length; }

    static long image(PDFPage api, long page) {
        long[] values = api.native_getImages(page);
        return values != null && values.length == 1 ? values[0] : 0;
    }

    public static void main(String[] args) {
        if (args.length != 2) throw new IllegalArgumentException("input and output PDFs required");
        System.loadLibrary("kspfreetext");
        PDFDocument document = new PDFDocument();
        NativeHandle doc = new NativeHandle();
        check(PDFDocument.native_openPDF(args[0], doc) == 0, "open image fixture");
        document.native_setEditStatus(doc.value(), 6);
        NativeHandle page = new NativeHandle();
        check(document.native_getPage(doc.value(), 0, page) == 0, "load image page");
        PDFPage api = new PDFPage(page.value());

        long[] images = api.native_getImages(page.value());
        check(images != null && images.length == 1, "fixture has one image");
        long originalToken = images[0];
        RectF originalRect = new RectF();
        api.native_getImageRect(page.value(), originalToken, originalRect);
        check(api.native_GetImageIndex(page.value(), originalToken) == 1,
                "image starts at original one-based index");

        RectF resizedRect = new RectF(210f, 590f, 310f, 650f);
        check(api.native_resizeImageRect(page.value(), originalToken, resizedRect) == 1,
                "resize image to original page-coordinate RectF contract");
        long resizeCommand = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        long resizedToken = image(api, page.value());
        RectF resizedActual = new RectF();
        api.native_getImageRect(page.value(), resizedToken, resizedActual);
        check(resizeCommand != 0 && sameRect(resizedRect, resizedActual),
                "resize changes exact image bounds");
        PDFFormFill.native_undoEdit(resizeCommand);
        long restoredAfterResize = image(api, page.value());
        RectF restoredResizeRect = new RectF();
        api.native_getImageRect(page.value(), restoredAfterResize, restoredResizeRect);
        check(sameRect(originalRect, restoredResizeRect), "undo resize restores bounds");
        PDFFormFill.native_redoEdit(resizeCommand);
        long redoneAfterResize = image(api, page.value());
        RectF redoneResizeRect = new RectF();
        api.native_getImageRect(page.value(), redoneAfterResize, redoneResizeRect);
        check(sameRect(resizedRect, redoneResizeRect), "redo resize restores target bounds");
        PDFFormFill.native_undoEdit(resizeCommand);

        long rotateToken = image(api, page.value());
        check(api.native_rotateImageClockwise(page.value(), rotateToken) == 1,
                "rotate image clockwise");
        long rotateCommand = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        long rotatedToken = image(api, page.value());
        RectF rotatedRect = new RectF();
        api.native_getImageRect(page.value(), rotatedToken, rotatedRect);
        check(rotateCommand != 0 && sameRect(new RectF(205f, 595f, 245f, 645f), rotatedRect),
                "clockwise rotation keeps center and swaps bounds");
        check(api.native_reverseImageHorizontal(page.value(), rotatedToken) == 1,
                "reverse rotated image horizontally");
        long reverseCommand = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        long reversedToken = image(api, page.value());
        RectF reversedRect = new RectF();
        api.native_getImageRect(page.value(), reversedToken, reversedRect);
        check(reverseCommand != 0
                        && sameRect(new RectF(205f, 595f, 245f, 645f), reversedRect),
                "horizontal reverse preserves page bounds");
        PDFFormFill.native_undoEdit(reverseCommand);
        PDFFormFill.native_undoEdit(rotateCommand);
        long restoredAfterRotate = image(api, page.value());
        RectF restoredRotateRect = new RectF();
        api.native_getImageRect(page.value(), restoredAfterRotate, restoredRotateRect);
        check(sameRect(originalRect, restoredRotateRect), "undo rotate restores matrix");

        long deleteToken = image(api, page.value());
        check(api.native_removeImageFromPage(page.value(), deleteToken), "remove image");
        long command = RuntimeProbe.nativeTakePendingEditUndo(page.value());
        check(command != 0, "remove produces opaque undo command");
        check(api.native_getImages(page.value()) == null,
                "removed final image uses original null-empty contract");
        check(api.native_GetImageIndex(page.value(), deleteToken) == 0
                        && RuntimeProbe.nativeLastErrorCode() == -1005,
                "removed image token becomes stale");
        check(api.native_IsModified(page.value()) && api.native_CheckFuncModified(page.value(), 6),
                "image remove sets original IMAGE dirty bit");

        PDFFormFill.native_undoEdit(command);
        long[] restored = api.native_getImages(page.value());
        check(restored != null && restored.length == 1 && restored[0] != originalToken,
                "undo restores image under a new token generation");
        RectF restoredRect = new RectF();
        api.native_getImageRect(page.value(), restored[0], restoredRect);
        check(sameRect(originalRect, restoredRect), "undo restores exact image geometry");
        check(api.native_GetImageIndex(page.value(), restored[0]) == 1,
                "undo restores original z-order index");

        PDFFormFill.native_redoEdit(command);
        check(count(api.native_getImages(page.value())) == 0, "redo removes image again");
        check(document.native_syncEditPageObj(doc.value(), page.value(), true), "sync image page");
        check(api.native_unloadPage2(page.value()) == 0, "close image page");

        PDFNativeSaver saver = new PDFNativeSaver();
        long session = saver.newSession(doc.value(), false);
        check(session != 0 && saver.start(session, args[1]) && saver.run(session, 500) == 100,
                "save image deletion");
        saver.destroy(session);
        PDFFormFill.native_deleteTransEdit(command);
        PDFFormFill.native_deleteTransEdit(reverseCommand);
        PDFFormFill.native_deleteTransEdit(rotateCommand);
        PDFFormFill.native_deleteTransEdit(resizeCommand);
        check(document.native_closePDF(doc.value()) == 0, "close source document");

        NativeHandle reopened = new NativeHandle();
        check(PDFDocument.native_openPDF(args[1], reopened) == 0, "reopen saved PDF");
        NativeHandle reopenedPage = new NativeHandle();
        check(document.native_getPage(reopened.value(), 0, reopenedPage) == 0,
                "load saved image page");
        PDFPage verify = new PDFPage(reopenedPage.value());
        check(count(verify.native_getImages(reopenedPage.value())) == 0,
                "saved PDF retains image deletion");
        long[] savedPaths = RuntimeProbe.nativePageObjectHandles(reopenedPage.value(), 2);
        check(savedPaths != null && savedPaths.length == 1,
                "image edits preserve unrelated page path");
        check(verify.native_unloadPage2(reopenedPage.value()) == 0, "close saved page");
        check(document.native_closePDF(reopened.value()) == 0, "close saved document");
        check(RuntimeProbe.nativeLiveHandleCount() == 0, "no leaked handles");

        System.out.println("IMAGE_EDIT PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
