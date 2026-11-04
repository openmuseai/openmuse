#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java field: Object SELECTION_MEMORY
// Java field: Object SELECTION_START
// Java field: Object SELECTION_END

// Java: public class Selection
pub struct Selection {
}

// Java: public class MemoryTextWatcher
pub struct MemoryTextWatcher {
}

// Java: public class START
pub struct START {
}

// Java: public class END
pub struct END {
}

// Java: public class MEMORY
pub struct MEMORY {
}

// Java: public interface PositionIterator
pub trait PositionIterator {
    // Java field: int DONE
    // Java: abstract int preceding(int var1)
    fn preceding(&self, _var1: i32) -> i32;

    // Java: abstract int following(int var1)
    fn following(&self, _var1: i32) -> i32;
}

pub const POSITION_ITERATOR_DONE: i32 = -1;

// Java: public class Selection
impl Selection {
    // Java: Selection()
    pub fn new() -> Self {
        // Java: super();
        Selection {}
    }

    // Java: static int getSelectionStart(CharSequence text)
    pub fn getSelectionStart(_text: i32) -> i32 {
        // Java: return text instanceof Spanned ? ((Spanned)text).getSpanStart(SELECTION_START) : -1;
        -1
    }

    // Java: static int getSelectionEnd(CharSequence text)
    pub fn getSelectionEnd(_text: i32) -> i32 {
        // Java: return text instanceof Spanned ? ((Spanned)text).getSpanStart(SELECTION_END) : -1;
        -1
    }

    // Java: static int getSelectionMemory(CharSequence text)
    pub fn getSelectionMemory(_text: i32) -> i32 {
        // Java: return text instanceof Spanned ? ((Spanned)text).getSpanStart(SELECTION_MEMORY) : -1;
        -1
    }

    // Java: static void setSelection(Spannable text, int start, int stop)
    pub fn setSelection(_text: i32, _start: i32, _stop: i32) {
        // Java: setSelection(text, start, stop, -1);
        Self::setSelection_with_memory(_text, _start, _stop, -1);
    }

    // Java: static void setSelection(Spannable text, int start, int stop, int memory)
    pub fn setSelection_with_memory(_text: i32, _start: i32, _stop: i32, _memory: i32) {
        // Java: int ostart = getSelectionStart(text);
        let _ostart = Self::getSelectionStart(_text);
        // Java: int oend = getSelectionEnd(text);
        let _oend = Self::getSelectionEnd(_text);
        // Java: if (ostart != start || oend != stop)
        if _ostart != _start || _oend != _stop
        {
            // Java: text.setSpan(SELECTION_START, start, start, 546);
            // Java: text.setSpan(SELECTION_END, stop, stop, 34);
            // Java: updateMemory(text, memory);
            Self::updateMemory(_text, _memory);
        }
    }

    // Java: static void updateMemory(Spannable text, int memory)
    pub fn updateMemory(_text: i32, _memory: i32) {
        // Java: if (memory > -1)
        if _memory > -1
        {
            // Java: int currentMemory = getSelectionMemory(text);
            let _currentMemory = Self::getSelectionMemory(_text);
            // Java: if (memory != currentMemory)
            if _memory != _currentMemory
            {
                // Java: text.setSpan(SELECTION_MEMORY, memory, memory, 34);
                // Java: if (currentMemory == -1)
                if _currentMemory == -1
                {
                    // Java: TextWatcher watcher = new MemoryTextWatcher();
                    // Java: text.setSpan(watcher, 0, text.length(), 18);
                }
            }
        }
        // Java: else
        else
        {
            // Java: removeMemory(text);
            Self::removeMemory(_text);
        }
    }

    // Java: static void removeMemory(Spannable text)
    pub fn removeMemory(_text: i32) {
        // Java: text.removeSpan(SELECTION_MEMORY);
        // Java: MemoryTextWatcher[] watchers = (MemoryTextWatcher[])text.getSpans(0, text.length(), MemoryTextWatcher.class);
        // Java: for(MemoryTextWatcher watcher : watchers)
        {
            // Java: text.removeSpan(watcher);
        }
    }

    // Java: static void setSelection(Spannable text, int index)
    pub fn setSelection_index(_text: i32, _index: i32) {
        // Java: setSelection(text, index, index);
        Self::setSelection(_text, _index, _index);
    }

    // Java: static void selectAll(Spannable text)
    pub fn selectAll(_text: i32) {
        // Java: setSelection(text, 0, text.length());
        Self::setSelection(_text, 0, 0);
    }

    // Java: static void extendSelection(Spannable text, int index)
    pub fn extendSelection(_text: i32, _index: i32) {
        // Java: extendSelection(text, index, -1);
        Self::extendSelection_with_memory(_text, _index, -1);
    }

    // Java: static void extendSelection(Spannable text, int index, int memory)
    pub fn extendSelection_with_memory(_text: i32, _index: i32, _memory: i32) {
        // Java: if (text.getSpanStart(SELECTION_END) != index)
        {
            // Java: text.setSpan(SELECTION_END, index, index, 34);
        }
        // Java: updateMemory(text, memory);
        Self::updateMemory(_text, _memory);
    }

    // Java: static void removeSelection(Spannable text)
    pub fn removeSelection(_text: i32) {
        // Java: text.removeSpan(SELECTION_START, 512);
        // Java: text.removeSpan(SELECTION_END);
        // Java: removeMemory(text);
        Self::removeMemory(_text);
    }

    // Java: static boolean moveUp(Spannable text, Layout layout)
    pub fn moveUp(_text: i32, _layout: i32) -> bool {
        // Java: int start = getSelectionStart(text);
        let _start = Self::getSelectionStart(_text);
        // Java: int end = getSelectionEnd(text);
        let _end = Self::getSelectionEnd(_text);
        // Java: if (start != end)
        if _start != _end
        {
            // Java: int min = Math.min(start, end);
            let _min = _start.min(_end);
            // Java: int max = Math.max(start, end);
            let _max = _start.max(_end);
            // Java: setSelection(text, min);
            Self::setSelection_index(_text, _min);
            // Java: return min != 0 || max != text.length();
            return _min != 0 || _max != 0;
        }
        // Java: else
        else
        {
            // Java: int line = layout.getLineForOffset(end);
            let _line = 0i32;
            // Java: if (line > 0)
            if _line > 0
            {
                // Java: setSelectionAndMemory(text, layout, line, end, -1, false);
                Self::setSelectionAndMemory(_text, _layout, _line, _end, -1, false);
                // Java: return true;
                return true;
            }
            // Java: else if (end != 0)
            else if _end != 0
            {
                // Java: setSelection(text, 0);
                Self::setSelection_index(_text, 0);
                // Java: return true;
                return true;
            }
            // Java: else
            else
            {
                // Java: return false;
                return false;
            }
        }
    }

    // Java: static void setSelectionAndMemory(Spannable text, Layout layout, int line, int end, int direction, boolean extend)
    pub fn setSelectionAndMemory(_text: i32, _layout: i32, _line: i32, _end: i32, _direction: i32, _extend: bool) {
        // Java: int move;
        let mut _move: i32;
        // Java: int newMemory;
        let mut _newMemory: i32;
        // Java: if (layout.getParagraphDirection(line) == layout.getParagraphDirection(line + direction))
        if true
        {
            // Java: int memory = getSelectionMemory(text);
            let _memory = Self::getSelectionMemory(_text);
            // Java: if (memory > -1)
            if _memory > -1
            {
                // Java: float h = layout.getPrimaryHorizontal(memory);
                // Java: move = layout.getOffsetForHorizontal(line + direction, h);
                _move = 0;
                // Java: newMemory = memory;
                _newMemory = _memory;
            }
            // Java: else
            else
            {
                // Java: float h = layout.getPrimaryHorizontal(end);
                // Java: move = layout.getOffsetForHorizontal(line + direction, h);
                _move = 0;
                // Java: newMemory = end;
                _newMemory = _end;
            }
        }
        // Java: else
        else
        {
            // Java: move = layout.getLineStart(line + direction);
            _move = 0;
            // Java: newMemory = -1;
            _newMemory = -1;
        }
        // Java: if (extend)
        if _extend
        {
            // Java: extendSelection(text, move, newMemory);
            Self::extendSelection_with_memory(_text, _move, _newMemory);
        }
        // Java: else
        else
        {
            // Java: setSelection(text, move, move, newMemory);
            Self::setSelection_with_memory(_text, _move, _move, _newMemory);
        }
    }

    // Java: static boolean moveDown(Spannable text, Layout layout)
    pub fn moveDown(_text: i32, _layout: i32) -> bool {
        // Java: int start = getSelectionStart(text);
        let _start = Self::getSelectionStart(_text);
        // Java: int end = getSelectionEnd(text);
        let _end = Self::getSelectionEnd(_text);
        // Java: if (start != end)
        if _start != _end
        {
            // Java: int min = Math.min(start, end);
            let _min = _start.min(_end);
            // Java: int max = Math.max(start, end);
            let _max = _start.max(_end);
            // Java: setSelection(text, max);
            Self::setSelection_index(_text, _max);
            // Java: return min != 0 || max != text.length();
            return _min != 0 || _max != 0;
        }
        // Java: else
        else
        {
            // Java: int line = layout.getLineForOffset(end);
            let _line = 0i32;
            // Java: if (line < layout.getLineCount() - 1)
            if _line < 0
            {
                // Java: setSelectionAndMemory(text, layout, line, end, 1, false);
                Self::setSelectionAndMemory(_text, _layout, _line, _end, 1, false);
                // Java: return true;
                return true;
            }
            // Java: else if (end != text.length())
            else if _end != 0
            {
                // Java: setSelection(text, text.length());
                Self::setSelection_index(_text, 0);
                // Java: return true;
                return true;
            }
            // Java: else
            else
            {
                // Java: return false;
                return false;
            }
        }
    }

    // Java: static boolean moveLeft(Spannable text, Layout layout)
    pub fn moveLeft(_text: i32, _layout: i32) -> bool {
        // Java: int start = getSelectionStart(text);
        let _start = Self::getSelectionStart(_text);
        // Java: int end = getSelectionEnd(text);
        let _end = Self::getSelectionEnd(_text);
        // Java: if (start != end)
        if _start != _end
        {
            // Java: setSelection(text, chooseHorizontal(layout, -1, start, end));
            Self::setSelection_index(_text, Self::chooseHorizontal(_layout, -1, _start, _end));
            // Java: return true;
            return true;
        }
        // Java: else
        else
        {
            // Java: int to = layout.getOffsetToLeftOf(end);
            let _to = 0i32;
            // Java: if (to != end)
            if _to != _end
            {
                // Java: setSelection(text, to);
                Self::setSelection_index(_text, _to);
                // Java: return true;
                return true;
            }
            // Java: else
            else
            {
                // Java: return false;
                return false;
            }
        }
    }

    // Java: static boolean moveRight(Spannable text, Layout layout)
    pub fn moveRight(_text: i32, _layout: i32) -> bool {
        // Java: int start = getSelectionStart(text);
        let _start = Self::getSelectionStart(_text);
        // Java: int end = getSelectionEnd(text);
        let _end = Self::getSelectionEnd(_text);
        // Java: if (start != end)
        if _start != _end
        {
            // Java: setSelection(text, chooseHorizontal(layout, 1, start, end));
            Self::setSelection_index(_text, Self::chooseHorizontal(_layout, 1, _start, _end));
            // Java: return true;
            return true;
        }
        // Java: else
        else
        {
            // Java: int to = layout.getOffsetToRightOf(end);
            let _to = 0i32;
            // Java: if (to != end)
            if _to != _end
            {
                // Java: setSelection(text, to);
                Self::setSelection_index(_text, _to);
                // Java: return true;
                return true;
            }
            // Java: else
            else
            {
                // Java: return false;
                return false;
            }
        }
    }

    // Java: static boolean extendUp(Spannable text, Layout layout)
    pub fn extendUp(_text: i32, _layout: i32) -> bool {
        // Java: int end = getSelectionEnd(text);
        let _end = Self::getSelectionEnd(_text);
        // Java: int line = layout.getLineForOffset(end);
        let _line = 0i32;
        // Java: if (line > 0)
        if _line > 0
        {
            // Java: setSelectionAndMemory(text, layout, line, end, -1, true);
            Self::setSelectionAndMemory(_text, _layout, _line, _end, -1, true);
            // Java: return true;
            return true;
        }
        // Java: else if (end != 0)
        else if _end != 0
        {
            // Java: extendSelection(text, 0);
            Self::extendSelection(_text, 0);
            // Java: return true;
            return true;
        }
        // Java: else
        else
        {
            // Java: return true;
            return true;
        }
    }

    // Java: static boolean extendDown(Spannable text, Layout layout)
    pub fn extendDown(_text: i32, _layout: i32) -> bool {
        // Java: int end = getSelectionEnd(text);
        let _end = Self::getSelectionEnd(_text);
        // Java: int line = layout.getLineForOffset(end);
        let _line = 0i32;
        // Java: if (line < layout.getLineCount() - 1)
        if _line < 0
        {
            // Java: setSelectionAndMemory(text, layout, line, end, 1, true);
            Self::setSelectionAndMemory(_text, _layout, _line, _end, 1, true);
            // Java: return true;
            return true;
        }
        // Java: else if (end != text.length())
        else if _end != 0
        {
            // Java: extendSelection(text, text.length(), -1);
            Self::extendSelection_with_memory(_text, 0, -1);
            // Java: return true;
            return true;
        }
        // Java: else
        else
        {
            // Java: return true;
            return true;
        }
    }

    // Java: static boolean extendLeft(Spannable text, Layout layout)
    pub fn extendLeft(_text: i32, _layout: i32) -> bool {
        // Java: int end = getSelectionEnd(text);
        let _end = Self::getSelectionEnd(_text);
        // Java: int to = layout.getOffsetToLeftOf(end);
        let _to = 0i32;
        // Java: if (to != end)
        if _to != _end
        {
            // Java: extendSelection(text, to);
            Self::extendSelection(_text, _to);
            // Java: return true;
            return true;
        }
        // Java: else
        else
        {
            // Java: return true;
            return true;
        }
    }

    // Java: static boolean extendRight(Spannable text, Layout layout)
    pub fn extendRight(_text: i32, _layout: i32) -> bool {
        // Java: int end = getSelectionEnd(text);
        let _end = Self::getSelectionEnd(_text);
        // Java: int to = layout.getOffsetToRightOf(end);
        let _to = 0i32;
        // Java: if (to != end)
        if _to != _end
        {
            // Java: extendSelection(text, to);
            Self::extendSelection(_text, _to);
            // Java: return true;
            return true;
        }
        // Java: else
        else
        {
            // Java: return true;
            return true;
        }
    }

    // Java: static boolean extendToLeftEdge(Spannable text, Layout layout)
    pub fn extendToLeftEdge(_text: i32, _layout: i32) -> bool {
        // Java: int where = findEdge(text, layout, -1);
        let _where = Self::findEdge(_text, _layout, -1);
        // Java: extendSelection(text, where);
        Self::extendSelection(_text, _where);
        // Java: return true;
        true
    }

    // Java: static boolean extendToRightEdge(Spannable text, Layout layout)
    pub fn extendToRightEdge(_text: i32, _layout: i32) -> bool {
        // Java: int where = findEdge(text, layout, 1);
        let _where = Self::findEdge(_text, _layout, 1);
        // Java: extendSelection(text, where);
        Self::extendSelection(_text, _where);
        // Java: return true;
        true
    }

    // Java: static boolean moveToLeftEdge(Spannable text, Layout layout)
    pub fn moveToLeftEdge(_text: i32, _layout: i32) -> bool {
        // Java: int where = findEdge(text, layout, -1);
        let _where = Self::findEdge(_text, _layout, -1);
        // Java: setSelection(text, where);
        Self::setSelection_index(_text, _where);
        // Java: return true;
        true
    }

    // Java: static boolean moveToRightEdge(Spannable text, Layout layout)
    pub fn moveToRightEdge(_text: i32, _layout: i32) -> bool {
        // Java: int where = findEdge(text, layout, 1);
        let _where = Self::findEdge(_text, _layout, 1);
        // Java: setSelection(text, where);
        Self::setSelection_index(_text, _where);
        // Java: return true;
        true
    }

    // Java: static boolean moveToPreceding(Spannable text, PositionIterator iter, boolean extendSelection)
    pub fn moveToPreceding(_text: i32, _iter: &dyn PositionIterator, _extendSelection: bool) -> bool {
        // Java: int offset = iter.preceding(getSelectionEnd(text));
        let _offset = _iter.preceding(Self::getSelectionEnd(_text));
        // Java: if (offset != -1)
        if _offset != -1
        {
            // Java: if (extendSelection)
            if _extendSelection
            {
                // Java: extendSelection(text, offset);
                Self::extendSelection(_text, _offset);
            }
            // Java: else
            else
            {
                // Java: setSelection(text, offset);
                Self::setSelection_index(_text, _offset);
            }
        }
        // Java: return true;
        true
    }

    // Java: static boolean moveToFollowing(Spannable text, PositionIterator iter, boolean extendSelection)
    pub fn moveToFollowing(_text: i32, _iter: &dyn PositionIterator, _extendSelection: bool) -> bool {
        // Java: int offset = iter.following(getSelectionEnd(text));
        let _offset = _iter.following(Self::getSelectionEnd(_text));
        // Java: if (offset != -1)
        if _offset != -1
        {
            // Java: if (extendSelection)
            if _extendSelection
            {
                // Java: extendSelection(text, offset);
                Self::extendSelection(_text, _offset);
            }
            // Java: else
            else
            {
                // Java: setSelection(text, offset);
                Self::setSelection_index(_text, _offset);
            }
        }
        // Java: return true;
        true
    }

    // Java: static int findEdge(Spannable text, Layout layout, int dir)
    pub fn findEdge(_text: i32, _layout: i32, _dir: i32) -> i32 {
        // Java: int pt = getSelectionEnd(text);
        let _pt = Self::getSelectionEnd(_text);
        // Java: int line = layout.getLineForOffset(pt);
        let _line = 0i32;
        // Java: int pdir = layout.getParagraphDirection(line);
        let _pdir = 1i32;
        // Java: if (dir * pdir < 0)
        if _dir * _pdir < 0
        {
            // Java: return layout.getLineStart(line);
            return 0;
        }
        // Java: else
        else
        {
            // Java: int end = layout.getLineEnd(line);
            let _end = 0i32;
            // Java: return line == layout.getLineCount() - 1 ? end : end - 1;
            return _end;
        }
    }

    // Java: static int chooseHorizontal(Layout layout, int direction, int off1, int off2)
    pub fn chooseHorizontal(_layout: i32, _direction: i32, _off1: i32, _off2: i32) -> i32 {
        // Java: int line1 = layout.getLineForOffset(off1);
        let _line1 = 0i32;
        // Java: int line2 = layout.getLineForOffset(off2);
        let _line2 = 0i32;
        // Java: if (line1 == line2)
        if _line1 == _line2
        {
            // Java: float h1 = layout.getPrimaryHorizontal(off1);
            let _h1 = 0.0f32;
            // Java: float h2 = layout.getPrimaryHorizontal(off2);
            let _h2 = 0.0f32;
            // Java: if (direction < 0)
            if _direction < 0
            {
                // Java: return h1 < h2 ? off1 : off2;
                return if _h1 < _h2 { _off1 } else { _off2 };
            }
            // Java: else
            else
            {
                // Java: return h1 > h2 ? off1 : off2;
                return if _h1 > _h2 { _off1 } else { _off2 };
            }
        }
        // Java: else
        else
        {
            // Java: int line = layout.getLineForOffset(off1);
            let _line = 0i32;
            // Java: int textdir = layout.getParagraphDirection(line);
            let _textdir = 1i32;
            // Java: return textdir == direction ? Math.max(off1, off2) : Math.min(off1, off2);
            return if _textdir == _direction { _off1.max(_off2) } else { _off1.min(_off2) };
        }
    }
}

// Java: public class MemoryTextWatcher
impl MemoryTextWatcher {
    // Java: MemoryTextWatcher()
    pub fn new() -> Self {
         // Java: super();
         MemoryTextWatcher {}
    }

    // Java: void beforeTextChanged(CharSequence s, int start, int count, int after)
    pub fn beforeTextChanged(&self, _s: i32, _start: i32, _count: i32, _after: i32) {
    }

    // Java: void onTextChanged(CharSequence s, int start, int before, int count)
    pub fn onTextChanged(&self, _s: i32, _start: i32, _before: i32, _count: i32) {
    }

    // Java: void afterTextChanged(Editable s)
    pub fn afterTextChanged(&self, _s: i32) {
         // Java: s.removeSpan(Selection.SELECTION_MEMORY);
         // Java: s.removeSpan(this);
    }
}

// Java: public class START
impl START {
    // Java: START()
    pub fn new() -> Self {
         // Java: super();
         START {}
    }
}

// Java: public class END
impl END {
    // Java: END()
    pub fn new() -> Self {
         // Java: super();
         END {}
    }
}

// Java: public class MEMORY
impl MEMORY {
    // Java: MEMORY()
    pub fn new() -> Self {
         // Java: super();
         MEMORY {}
    }
}
