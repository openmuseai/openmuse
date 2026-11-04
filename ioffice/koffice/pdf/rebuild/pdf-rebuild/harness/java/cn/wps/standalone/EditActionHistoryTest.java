package cn.wps.standalone;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public final class EditActionHistoryTest {
    private static int pass;

    private static void check(boolean condition, String name) {
        if (!condition) throw new AssertionError(name);
        pass++;
    }

    private static List<Long> values(EditActionHistory.Action action) {
        return action == null ? null : action.commands();
    }

    public static void main(String[] args) {
        ArrayList<Long> disposed = new ArrayList<>();
        EditActionHistory history = new EditActionHistory();
        history.recordInput(1, 1, 0, "a", false, 1_000, disposed::add);
        history.recordInput(2, 1, 0, "b", false, 2_000, disposed::add);
        history.recordInput(3, 1, 0, "c", false, 3_000, disposed::add);
        check(history.undoCount() == 1, "continuous input merged");
        EditActionHistory.Action action = history.takeUndo();
        check(values(action).equals(Arrays.asList(1L, 2L, 3L)), "command order retained");
        check(values(history.takeRedo()).equals(Arrays.asList(1L, 2L, 3L)),
                "redo returns forward action");

        history.recordInput(4, 1, 0, "d", false, 7_001, disposed::add);
        check(history.undoCount() == 2, "three second window enforced");
        history.recordInput(5, 1, 0, "12345678901234567890123456789012345678901234567890",
                false, 7_100, disposed::add);
        check(history.undoCount() == 3, "fifty character cap enforced");

        history.clear(disposed::add);
        disposed.clear();
        history.recordInput(10, 1, 0, "n", true, 10_000, disposed::add);
        history.recordInput(11, 1, 1, "ni", true, 20_000, disposed::add);
        history.recordInput(12, 1, 2, "你", false, 30_000, disposed::add);
        check(history.undoCount() == 1, "one composition stays one action");
        check(values(history.takeUndo()).equals(Arrays.asList(10L, 11L, 12L)),
                "composition retains every native handle");

        history.clear(disposed::add);
        history.recordInput(20, 1, 0, "a", false, 1_000, disposed::add);
        history.recordInput(21, 1, 0, "\n", false, 1_100, disposed::add);
        history.recordInput(22, 1, 0, "b", false, 1_200, disposed::add);
        check(history.undoCount() == 3, "paragraph interrupts both sides");

        history.clear(disposed::add);
        history.recordInput(30, 1, 0, "a", false, 1_000, disposed::add);
        history.interrupt();
        history.recordInput(31, 1, 0, "b", false, 1_100, disposed::add);
        history.recordInput(32, 2, 0, "c", false, 1_200, disposed::add);
        check(history.undoCount() == 3, "selection and page changes interrupt");

        history.takeUndo();
        history.recordStandalone(40, 1, disposed::add);
        check(history.redoCount() == 0 && disposed.contains(32L),
                "new branch disposes abandoned redo handles");
        history.recordStandalone(41, 1, disposed::add);
        check(history.undoCount() == 4, "standalone mutations do not merge");

        history.clear(disposed::add);
        check(history.undoCount() == 0 && history.redoCount() == 0,
                "clear releases all actions");
        System.out.println("EDIT_ACTION_HISTORY PASS=" + pass + " FAIL=0");
    }
}
