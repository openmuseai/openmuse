//! The context menus offer Insert Copied Cells while the clipboard holds a copy (#21).

use egui::{Event, Modifiers, PointerButton, pos2};
use egui_kittest::kittest::Queryable;
use gridcraft_engine::Session;
use gridcraft_engine::core::{CellRef, Value};
use gridcraft_ui_egui::{SheetApp, grid};
use serde_json::json;

type Harness = egui_kittest::Harness<'static, SheetApp>;

fn harness() -> Harness {
    let mut session = Session::new();
    session.new_workbook();
    session.execute("range.setValues", json!({"range": "A1", "values": [[1], [2], [3], [4]]})).unwrap();
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        SheetApp::new(session, Default::default()),
    );
    h.run_steps(4);
    h
}

fn value(h: &Harness, cell: &str) -> Value {
    h.state().session.active().unwrap().wb.active().unwrap().value(CellRef::parse(cell).unwrap())
}

fn right_click_row_header(h: &mut Harness, cell: &str) {
    let app = h.state();
    let sh = app.session.active().unwrap().wb.active().unwrap();
    let geo = grid::Geo::new(sh, app.grid.rect.unwrap(), app.view().scroll);
    let y = geo.cell_rect(sh, CellRef::parse(cell).unwrap()).center().y;
    let pos = pos2((geo.rect.left() + geo.cells.left()) / 2.0, y);
    h.input_mut().events.push(Event::PointerMoved(pos));
    h.input_mut().events.push(Event::PointerButton { pos, button: PointerButton::Secondary, pressed: true, modifiers: Modifiers::NONE });
    h.input_mut().events.push(Event::PointerButton { pos, button: PointerButton::Secondary, pressed: false, modifiers: Modifiers::NONE });
    h.run_steps(2);
}

#[test]
fn row_header_menu_inserts_copied_rows() {
    let mut h = harness();
    // Nothing copied: no entry.
    right_click_row_header(&mut h, "A3");
    assert!(h.state().grid.header_menu.is_some());
    assert!(h.query_by_label("Insert Copied Cells").is_none());
    h.state_mut().grid.header_menu = None;
    h.run_steps(2);

    h.state_mut().run("edit.copy", json!({"range": "1:1"})).unwrap();
    right_click_row_header(&mut h, "A3");
    h.get_by_label("Insert Copied Cells").click();
    h.run_steps(2);
    assert_eq!(value(&h, "A3"), Value::Number(1.0));
    assert_eq!(value(&h, "A4"), Value::Number(3.0));
    assert_eq!(value(&h, "A5"), Value::Number(4.0));
}
