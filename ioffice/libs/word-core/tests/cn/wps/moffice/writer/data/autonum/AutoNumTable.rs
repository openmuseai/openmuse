// function_id=01_function_num_table
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::data::autonum::AutoNumTable::AutoNumTable;

/// NT1: Java AutoNumTable(Transaction) allocates lst/lfo tables.
#[test]
fn nt1_ctor_tables_present() {
    let mut table = AutoNumTable::AutoNumTable(None);
    assert!(table.getLstDataTable().getLstMap().is_empty());
    assert!(table.getLfoDataTable().getLfoDataMap().is_empty());
}

/// NT2: getLstLfoTool is non-null after ctor.
#[test]
fn nt2_get_lst_lfo_tool() {
    let mut table = AutoNumTable::AutoNumTable(None);
    let _tool = table.getLstLfoTool();
}

/// NT3: empty numbering — tables exist and lst map is empty (queryable, no panic).
#[test]
fn nt3_empty_numbering() {
    let mut table = AutoNumTable::AutoNumTable(None);
    let lst = table.getLstDataTable();
    assert!(lst.getLstMap().is_empty());
    assert!(!table.getLstLfoTool().isExisting(1, 0));
}
