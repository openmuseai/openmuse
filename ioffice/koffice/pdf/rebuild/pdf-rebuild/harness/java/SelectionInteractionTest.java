import cn.wps.rebuild.OriginalSelectionPolicy;
import cn.wps.rebuild.SelectionTransform;

public final class SelectionInteractionTest {
    private static int pass, fail;
    private static void check(boolean ok, String name) {
        if (ok) pass++; else { fail++; System.err.println("FAIL " + name); }
    }
    private static boolean near(float a, float b) { return Math.abs(a - b) < 0.01f; }

    public static void main(String[] args) {
        SelectionTransform t = new SelectionTransform();
        t.setViewport(1000, 600, 1000, 1200, 500, 600, true);
        check(near(t.getZoom(), 1f), "fit-width initial zoom");
        float[] point = new float[2];
        check(t.screenToPage(500, 200, point) && near(point[0], 250) && near(point[1], 100),
                "screen to page at fit width");
        check(!t.screenToPage(-1, 10, point), "outside page rejected");

        t.scaleAt(2f, 500, 300);
        check(near(t.getZoom(), 2f), "pinch scale");
        check(t.screenToPage(500, 300, point) && near(point[0], 250) && near(point[1], 150),
                "pinch focus invariant");
        float[] rect = new float[4];
        check(t.pageRectToScreen(200, 100, 300, 120, rect)
                        && near(rect[0], 300) && near(rect[1], 100)
                        && near(rect[2], 700) && near(rect[3], 180),
                "page rect mapping after zoom");
        t.scroll(100000, 100000);
        check(t.screenToPage(0, 0, point) && near(point[0], 0) && near(point[1], 0),
                "scroll clamp top left");

        String sample = "hello,world 中文";
        OriginalSelectionPolicy.UnicodeSource source = i -> sample.charAt(i);
        int[] word = OriginalSelectionPolicy.expand(source, 2, sample.length());
        check(word[0] == 0 && word[1] == 4, "original Latin expansion");
        int[] comma = OriginalSelectionPolicy.expand(source, 5, sample.length());
        check(comma[0] == 5 && comma[1] == 5, "split char selects itself");
        int[] chinese = OriginalSelectionPolicy.expand(source, sample.length() - 1, sample.length());
        check(chinese[0] == sample.length() - 2 && chinese[1] == sample.length() - 1,
                "original CJK expansion");
        int[] crossed = OriginalSelectionPolicy.order(20, 7);
        check(crossed[0] == 7 && crossed[1] == 20, "handle crossing swaps range");

        System.out.println("SELECTION_INTERACTION PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
