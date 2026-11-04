use crate::java_runtime::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::cn::wps::moss::app::IGridSheet::IGridSheet;
use crate::cn::wps::moss::app::KmoBookApi::KmoBookApi;
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;

pub const ENGINE_ID: &str = "ss-core";

static TEMP_FILE_SEQ: AtomicU64 = AtomicU64::new(0);

pub struct SpreadsheetLoadConfig {
    pub ignore_security: bool,
}

impl Default for SpreadsheetLoadConfig {
    fn default() -> Self {
        Self {
            ignore_security: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellSnapshot {
    pub row: i32,
    pub col: i32,
    pub raw_type: i32,
    pub raw_value: String,
    pub display_text: String,
    pub style: Option<serde_json::Value>,
    pub format_color: Option<i32>,
    pub fill_pos: i32,
    pub fill_char: u32,
    pub numeric_as_text: bool,
    pub rich_format_invalid: bool,
    pub ixf: i32,
    pub formula_type: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SheetSnapshot {
    pub index: i32,
    pub sheet_id: i32,
    pub name: String,
    pub hidden: bool,
    pub row_heights: Vec<i32>,
    pub column_widths: Vec<i32>,
    pub merged_ranges: Vec<[i32; 4]>,
    pub cells: Vec<CellSnapshot>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorkbookSnapshot {
    pub engine_id: &'static str,
    pub input_sha256: String,
    pub open_code: i32,
    pub sheet_count: i32,
    pub xf_count: i32,
    pub cell_count: i32,
    pub window_row: i32,
    pub window_col: i32,
    pub sheets: Vec<SheetSnapshot>,
}

static NATIVE_TEMP_DIR: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
// rust: Android supplies its app cache directory as the native file-system context.
pub fn set_native_temp_directory(path: std::path::PathBuf) {
    let _ = NATIVE_TEMP_DIR.set(path);
}
pub struct SpreadsheetSession {
    cf_evaluator: Rc<crate::cn::wps::moss::app::condfmt::KmoCondFmtEval::KmoCondFmtEval>,
    api: Rc<KmoBookApi>,
    input_sha256: String,
    open_code: i32,
}

impl SpreadsheetSession {
    pub fn open_path(path: &str, config: SpreadsheetLoadConfig) -> Self {
        let bytes = std::fs::read(path).unwrap_or_default();
        Self::open_loaded(path, &bytes, config)
    }

    pub fn open_bytes(bytes: &[u8], config: SpreadsheetLoadConfig) -> Self {
        let seq = TEMP_FILE_SEQ.fetch_add(1, Ordering::Relaxed);
        let path = NATIVE_TEMP_DIR
            .get()
            .cloned()
            .unwrap_or_else(std::env::temp_dir)
            .join(format!(
                "ss-core-{}-{}-{}.xlsx",
                std::process::id(),
                seq,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0)
            ));
        let path_str = path.to_string_lossy().to_string();
        if std::fs::write(&path, bytes).is_err() {
            return Self {
                cf_evaluator: Rc::new(
                    crate::cn::wps::moss::app::condfmt::KmoCondFmtEval::KmoCondFmtEval::new(),
                ),
                api: KmoBookApi::KmoBookApi(),
                input_sha256: sha256_hex(bytes),
                open_code: 2,
            };
        }
        let session = Self::open_loaded(&path_str, bytes, config);
        let _ = std::fs::remove_file(path);
        session
    }

    fn open_loaded(path: &str, bytes: &[u8], config: SpreadsheetLoadConfig) -> Self {
        let api = KmoBookApi::KmoBookApi();
        let open_code = api.open(path, None, config.ignore_security);
        Self {
            cf_evaluator: Rc::new(
                crate::cn::wps::moss::app::condfmt::KmoCondFmtEval::KmoCondFmtEval::new(),
            ),
            api,
            input_sha256: sha256_hex(bytes),
            open_code,
        }
    }

    pub fn open_code(&self) -> i32 {
        self.open_code
    }

    pub fn input_sha256(&self) -> &str {
        &self.input_sha256
    }

    pub fn is_dirty(&self) -> bool {
        self.api.book().isDirty()
    }

    // rust: Product transport invokes the original grid/theme/text-data classes under a native Paint host.
    pub fn layout_geometry(
        &self,
        index: i32,
        dpi: f32,
        zoom: i32,
        rows: i32,
        cols: i32,
    ) -> serde_json::Value {
        self.layout_evidence(index, dpi, zoom, rows, cols, false)
    }
    pub fn layout_evidence(
        &self,
        index: i32,
        dpi: f32,
        zoom: i32,
        rows: i32,
        cols: i32,
        render: bool,
    ) -> serde_json::Value {
        use crate::cn::wps::moffice::spreadsheet::control::grid::{
            core::GridSheetLayoutInfo::GridSheetLayoutInfo,
            layout::data::CellTextData::CellTextData, render::data::GridClientData::GridClientData,
        };
        use crate::cn::wps::moss::app::UnitsConverter::UnitsConverter;
        let book = self.api.book();
        let sheet = book.sheetAt(index);
        if let Some(worker) = self.cf_evaluator.requestEval(
            sheet.clone(),
            Rc::new(vec![
                crate::cn::wps::moss::util::KmoRect::KmoRect::KmoRect__bounds(
                    0,
                    0,
                    rows - 1,
                    cols - 1,
                ),
            ]),
        ) {
            let _ = worker.join();
        }
        let grid = KmoSheet::sharedReadOnlyGridSheet(&sheet);
        let mut converter = UnitsConverter::UnitsConverter__2(dpi, dpi);
        converter.setZoom(zoom);
        let mut layout = GridSheetLayoutInfo::new(
            grid.clone(),
            converter,
            None,
            Rc::new(GridClientData::new()),
        );
        layout.setIgnoreHeadSize(true);
        layout.mWidth = 1024;
        layout.mHeight = 768;
        layout.reset(grid);
        let mut cells = vec![];
        let mut chooser=crate::cn::wps::moffice::spreadsheet::control::grid::layout::CellTextLayout::CellTextLayout::default();
        for row in 0..rows {
            for col in 0..cols {
                let sh = sheet.borrow();
                if sh.getCellData(row, col).isNull() {
                    continue;
                }
                let merged = sh.getMergedRange(row, col);
                if let Some(m) = &merged {
                    if m.tl.row != row || m.tl.col != col {
                        continue;
                    }
                }
                drop(sh);
                let width = merged
                    .as_ref()
                    .map(|m| {
                        (m.tl.col..=m.br.col)
                            .map(|c| layout.columnWidthInPixels(c))
                            .sum()
                    })
                    .unwrap_or_else(|| layout.columnWidthInPixels(col));
                let height = merged
                    .as_ref()
                    .map(|m| {
                        (m.tl.row..=m.br.row)
                            .map(|r| layout.rowHeightInPixels(r))
                            .sum()
                    })
                    .unwrap_or_else(|| layout.rowHeightInPixels(row));
                if width == 0 || height == 0 {
                    continue;
                }
                let mut data = CellTextData::default();
                let valid = data.updateData(&layout, width, height, row, col, true, None);
                let mut c = serde_json::json!({"row":row,"col":col,"width":width,"height":height,"valid":valid});
                if valid {
                    c["text"] = serde_json::json!(
                        String::from_utf16(data.source()).expect("UTF16 cell text")
                    );
                    c["cellType"] = serde_json::json!(data.mCellType);
                    c["richText"] = serde_json::json!(data.mIsRichText);
                    c["merged"] = serde_json::json!(data.mIsMergedCell);
                    c["centerCrossWidth"] = serde_json::json!(data.mCenterCrossWidth);
                    c["para"] = serde_json::to_value(&data.mPara).unwrap();
                    c["firstSpan"] = serde_json::to_value(&data.mFirstSpan).unwrap();
                    let fm=crate::cn::wps::moffice::spreadsheet::control::grid::layout::typo::PaintFontHelper::PaintFontHelper::getFontMetricsInt(&data.mFirstSpan);
                    c["fontMetrics"] = serde_json::json!({"top":fm.top,"ascent":fm.ascent,"descent":fm.descent,"bottom":fm.bottom,"leading":fm.leading});
                    c["zeroWidth"]=serde_json::json!(crate::cn::wps::moffice::spreadsheet::control::grid::layout::typo::PaintFontHelper::PaintFontHelper::getCharWidth__span(&[48],&data.mFirstSpan)as f64);
                    c["otherSpans"] = serde_json::to_value(&data.mOtherSpans).unwrap();
                    let tl = chooser.getTextLayout(&data);
                    tl.typo(&layout, &mut data, width, height, true);
                    c["layoutClass"] = serde_json::json!(tl.className());
                    c["textRect"] = serde_json::json!(tl.getTextRect().as_array());
                    let p = tl.getTypoOffset();
                    c["offset"] = serde_json::json!([p.x as f64, p.y as f64]);
                }
                cells.push(c);
            }
        }
        let mut result = serde_json::json!({"dpi":dpi as f64,"zoom":zoom,"defaultCharWidth":layout.defaultCharWidthInPixels()as f64,"rowHeights":(0..rows).map(|r|layout.rowHeightInPixels(r)).collect::<Vec<_>>(),"columnWidths":(0..cols).map(|c|layout.columnWidthInPixels(c)).collect::<Vec<_>>(),"rowY":(0..=rows).map(|r|layout.rowY(r)).collect::<Vec<_>>(),"columnX":(0..=cols).map(|c|layout.columnX(c)).collect::<Vec<_>>(),"cells":cells});
        if render {
            use crate::cn::wps::moffice::spreadsheet::control::grid::{
                core::GridRange::GridRange,
                render::{
                    graphics::{Canvas, Paint, Rect},
                    layer::celllayer::CellsRenderLayer::CellsRenderLayer,
                },
            };
            let parent = CellsRenderLayer::new(Rc::new(GridClientData::new()));
            *parent.mPreClientRect.borrow_mut() = Some(Rect::default());
            *parent.mCurClientRect.borrow_mut() = Some(Rect::new(0, 0, 1024, 768));
            let range = GridRange::new(0, rows - 1, 0, cols - 1);
            parent
                .mMergedRangeCache
                .borrow_mut()
                .update(&sheet.borrow(), &range);
            parent.mCellXfCache.update();
            let mut canvas = Canvas::new(1024, 768);
            canvas.drawColor(-1);
            let mut paint = Paint::default();
            for drawer in parent.mLayerDrawers.borrow_mut().iter_mut().flatten() {
                drawer.draw(&mut canvas, &mut paint, &range, &layout);
            }
            let raster = crate::cn::wps::moffice::spreadsheet::control::grid::render::native::host(
                serde_json::json!({"op":"canvasRaster","width":1024,"height":768,"commands":canvas.commands}),
            );
            result["render"] = raster;
            result["commands"] = serde_json::json!(canvas.commands);
        }
        result
    }

    // rust: Function-observation transport; every value is returned by the original named measurement facade.
    pub fn layout_measure(
        &self,
        index: i32,
        dpi: f32,
        zoom: i32,
        rows: i32,
        cols: i32,
    ) -> serde_json::Value {
        use crate::cn::wps::moffice::spreadsheet::control::grid::{
            core::GridSheetLayoutInfo::GridSheetLayoutInfo,
            layout::{
                data::CellTextData::CellTextData,
                measure::TextLayoutMeasure::TextLayoutMeasure as M,
            },
            render::data::GridClientData::GridClientData,
        };
        use crate::cn::wps::moss::{
            app::UnitsConverter::UnitsConverter, framework::EtFontOp::INSTANCE as ET_FONT_OP,
        };
        let sheet = self.api.book().sheetAt(index);
        let grid = KmoSheet::sharedReadOnlyGridSheet(&sheet);
        let mut converter = UnitsConverter::UnitsConverter__2(dpi, dpi);
        converter.setZoom(zoom);
        let mut layout = GridSheetLayoutInfo::new(
            grid.clone(),
            converter,
            None,
            Rc::new(GridClientData::new()),
        );
        layout.setIgnoreHeadSize(true);
        layout.mWidth = 1024;
        layout.mHeight = 768;
        layout.reset(grid);
        if let Some(worker) = self.cf_evaluator.requestEval(
            sheet.clone(),
            Rc::new(vec![
                crate::cn::wps::moss::util::KmoRect::KmoRect::KmoRect__bounds(
                    0,
                    0,
                    rows - 1,
                    cols - 1,
                ),
            ]),
        ) {
            let _ = worker.join();
        }
        let mut cells = vec![];
        for row in 0..rows {
            for col in 0..cols {
                if sheet.borrow().getCellData(row, col).isNull()
                    && !(row == 39 && col == 23)
                    && !(row == 0 && col == 1)
                {
                    continue;
                }
                let font = ET_FONT_OP.getFont(&sheet.borrow(), row, col);
                let mut c = serde_json::json!({"row":row,"col":col,"fontHash":font.as_ref().map(|f|f.hashCode()),"emptyWidth":M::measureEmptyTypoWidth(&layout,row,col)as f64});
                if let Some(font) = font {
                    c["emptyHeight"] =
                        serde_json::json!(M::measureEmptyTypoHeight(&layout, &font) as f64);
                }
                for best in [false, true] {
                    let prefix = if best { "best" } else { "grow" };
                    c[format!("{prefix}Width")] =
                        serde_json::json!(M::getAdjustedCellWidth(&layout, row, col, best) as f64);
                    c[format!("{prefix}Height")] =
                        serde_json::json!(M::getAdjustedCellHeight(&layout, row, col, best) as f64);
                    c[format!("{prefix}CoreHeight")] = serde_json::json!(
                        M::getAdjustedCellHeightByCoreWidth(&layout, row, col, best) as f64
                    );
                }
                let width = layout.columnWidthInPixels(col);
                let height = layout.rowHeightInPixels(row);
                let mut data = CellTextData::default();
                if width > 0
                    && height > 0
                    && data.updateData(&layout, width, height, row, col, true, None)
                {
                    c["typoWidth"] =
                        serde_json::json!(M::measureTypoWidth(&data, width, height) as f64);
                    c["typoHeight"] =
                        serde_json::json!(M::measureTypoHeight(&data, width, height) as f64);
                }
                cells.push(c);
            }
        }
        serde_json::json!({"dpi":dpi as f64,"zoom":zoom,"cells":cells,"endColumns":([0,3,6,9,23].into_iter().map(|c|M::getCellTextEndCol(&layout,0,40,c)).collect::<Vec<_>>())})
    }

    // rust: Product viewport transport. Original CellsRenderLayer.directDraw owns layer order and clipping.
    // Unlike layout_evidence this path performs no observer geometry/typo prepass.
    pub fn render_grid(
        &self,
        index: i32,
        dpi: f32,
        zoom: i32,
        rows: i32,
        cols: i32,
        width: i32,
        height: i32,
    ) -> serde_json::Value {
        use crate::cn::wps::moffice::spreadsheet::control::grid::{
            core::{GridRange::GridRange, GridSheetLayoutInfo::GridSheetLayoutInfo},
            render::{
                data::GridClientData::GridClientData,
                graphics::{Canvas, Paint, Rect},
                layer::celllayer::CellsRenderLayer::CellsRenderLayer,
            },
        };
        use crate::cn::wps::moss::app::UnitsConverter::UnitsConverter;
        let sheet = self.api.book().sheetAt(index);
        let grid = KmoSheet::sharedReadOnlyGridSheet(&sheet);
        let clients = Rc::new(GridClientData::new());
        let mut converter = UnitsConverter::UnitsConverter__2(dpi, dpi);
        converter.setZoom(zoom);
        let mut layout = GridSheetLayoutInfo::new(grid.clone(), converter, None, clients.clone());
        layout.setIgnoreHeadSize(true);
        layout.mWidth = width;
        layout.mHeight = height;
        layout.reset(grid);
        if let Some(worker) = self.cf_evaluator.requestEval(
            sheet.clone(),
            Rc::new(vec![
                crate::cn::wps::moss::util::KmoRect::KmoRect::KmoRect__bounds(
                    0,
                    0,
                    rows - 1,
                    cols - 1,
                ),
            ]),
        ) {
            let _ = worker.join();
        }
        {
            let client = clients.mClients.borrow()[3].as_ref().unwrap().clone();
            let mut client = client.borrow_mut();
            client.mCurClient = Rect::new(0, 0, width, height);
            client.mCurGridRange = GridRange::new(0, rows - 1, 0, cols - 1);
        }
        let layer = CellsRenderLayer::new(clients);
        let mut canvas = Canvas::new(width, height);
        let mut paint = Paint::default();
        layer.directDraw(&mut canvas, &mut paint, &layout);
        let render = crate::cn::wps::moffice::spreadsheet::control::grid::render::native::host(
            serde_json::json!({"op":"canvasRaster","width":width,"height":height,"commands":canvas.commands}),
        );
        serde_json::json!({"inputSha256":self.input_sha256,"dpi":dpi as f64,"zoom":zoom,"width":width,"height":height,"rowHeights":(0..rows).map(|r|layout.rowHeightInPixels(r)).collect::<Vec<_>>(),"columnWidths":(0..cols).map(|c|layout.columnWidthInPixels(c)).collect::<Vec<_>>(),"render":render,"commands":canvas.commands})
    }

    pub fn snapshot(&self, row: i32, col: i32, rows: i32, cols: i32) -> WorkbookSnapshot {
        let book = self.api.book();
        let sheet_count = book.sheetCount();
        let xf_count = book.getXFManager().borrow().getNumExFormats();
        let mut sheets = Vec::new();
        let mut cell_count = 0i32;
        if self.open_code == 1 {
            for index in 0..sheet_count {
                let sheet_rc = book.sheetAt(index);
                // java BaseCoreView.evalCondFmt: if(book.isLargeFile || !book.isCompletelyReaded || !book.isCalcChainInitialized) return;
                // rust: Kotlin host source et-sdk/.../BaseCoreView.kt:1808. A native FFI capture joins the original request worker before collecting a stable snapshot.
                if !book.isLargeFile()
                    && book.isCompletelyReaded()
                    && book.isCalcChainInitialized()
                    && rows > 0
                    && cols > 0
                {
                    let area = Rc::new(vec![
                        crate::cn::wps::moss::util::KmoRect::KmoRect::KmoRect__bounds(
                            row,
                            col,
                            row + rows - 1,
                            col + cols - 1,
                        ),
                    ]);
                    if let Some(worker) = self.cf_evaluator.requestEval(sheet_rc.clone(), area) {
                        let _ = worker.join();
                    }
                }
                let grid = KmoSheet::sharedReadOnlyGridSheet(&sheet_rc);
                let mut row_heights = Vec::new();
                for offset in 0..rows.max(0) {
                    row_heights.push(grid.rowHeight(row + offset));
                }
                let mut column_widths = Vec::new();
                for offset in 0..cols.max(0) {
                    column_widths.push(grid.columnWidth(col + offset));
                }
                let core = grid.coreSheet();
                let sheet = core.borrow();
                let merged_ranges = sheet
                    .allMergedRangesForSnapshot()
                    .iter()
                    .map(|r| [r.tl.row, r.tl.col, r.br.row, r.br.col])
                    .collect();
                let mut cells = Vec::new();
                for r_off in 0..rows.max(0) {
                    for c_off in 0..cols.max(0) {
                        let r = row + r_off;
                        let c = col + c_off;
                        let data = sheet.getCellData(r, c);
                        if data.isNull() {
                            continue;
                        }
                        let raw_type = sheet.getCellValueType__2(r, c);
                        // Java EtParityDump 用 getCellValueId，公式 id 先解开缓存值。
                        let value_id = sheet.getCellValueId(r, c);
                        let raw_value = raw_value(&sheet, value_id, raw_type);
                        // Java GridSheetData.getCellValue delegates to KmoSheet.getNumFormatResult.
                        let formatted = grid.getCellValue(r, c, i32::MAX);
                        let formatted = formatted.borrow();
                        let display_text = formatted.numfmtStr().to_string();
                        let style=sheet.getCellXF__2(r,c).as_ref().map(|xf|{
                            let cf=sheet.getCFormatter();
                            let font=cf.getMergeXF(r,c,xf.clone(),2);let pattern=cf.getMergeXF(r,c,xf.clone(),0);let border=cf.getMergeXF(r,c,xf.clone(),1);
                            let mut value=style_snapshot(&font);let p=style_snapshot(&pattern);let b=style_snapshot(&border);
                            for name in ["fillPattern","fillForeground","fillBackground"]{value[name]=p[name].clone();}
                            for name in ["borderLeft","borderRight","borderTop","borderBottom","leftBorderColor","rightBorderColor","topBorderColor","bottomBorderColor"]{value[name]=b[name].clone();}
                            let palette=book.getPalette();let rawFontColor=font.getFont().map(|f|f.getColorPaletteIndex()).unwrap_or(32767);
                            // java SpanInfo.reset: palette index -> palette.getRGBColor; then isAutomaticColor -> black.
                            let mut fontColor=if (rawFontColor>>24)&255==0{palette.getRGBColor(rawFontColor as i16)}else{rawFontColor};
                            if crate::cn::wps::moss::util::KmoColor::KmoColor::isAutomaticColor(fontColor){fontColor=-16777216;}
                            // java CellTextData: format color overrides only when the conditional font color equals the base color.
                            if let Some(color)=formatted.mNumFmtColorIdx.filter(|_|xf.getFont().map(|f|f.getColorPaletteIndex())==font.getFont().map(|f|f.getColorPaletteIndex())){fontColor=palette.getFormatColor(color);}
                            value["fontColorRgba"]=serde_json::json!(fontColor);
                            value["fillForegroundRgba"]=serde_json::json!(palette.getFormatColor(pattern.getFillForeground()));
                            value["fillBackgroundRgba"]=serde_json::json!(palette.getFormatColor(pattern.getFillBackground()));
                            value["showValue"]=serde_json::json!(cf.isShowValue(r,c));
                            value["effectiveNumFormat"]=serde_json::json!(cf.getNumFmtCode(&book.getXFManager().borrow(),r,c).or_else(||xf.getNumFormat()));
                            value["conditionalFormat"]=serde_json::json!({
                                "priorities":cf.getGenerals(r,c).unwrap_or_default().iter().map(|r|r.getRule().getPriority()).collect::<Vec<_>>(),
                                "dataBar":cf.getDataBar(r,c).map(|v|serde_json::json!({"priority":v.getPriority(),"color":v.getColor(),"length":v.getLength(),"dir":v.getDir(),"showValue":v.isShowValue()})),
                                "colorScale":cf.getColorScale(r,c).map(|v|serde_json::json!({"priority":v.getPriority(),"color":v.getColor()})),
                                "iconSet":cf.getIconSet(r,c).map(|v|serde_json::json!({"priority":v.getPriority(),"index":v.getIndex(),"type":v.getType().map(|t|format!("${}",&format!("{t:?}")[1..])),"showValue":v.isShowValue()}))});value});
                        cells.push(CellSnapshot {
                            row: r,
                            col: c,
                            raw_type,
                            raw_value,
                            display_text,
                            style,
                            format_color: formatted.mNumFmtColorIdx,
                            fill_pos: formatted.mFillPos,
                            fill_char: formatted.mFillChar as u32,
                            numeric_as_text: formatted.mNumericAsText,
                            rich_format_invalid: formatted.richFormatInvalid,
                            ixf: data.ixf,
                            formula_type: data.formulaType,
                        });
                    }
                }
                cell_count += cells.len() as i32;
                sheets.push(SheetSnapshot {
                    index,
                    sheet_id: sheet.getSheetId(),
                    name: sheet.name(),
                    hidden: sheet.isSheetHidden(),
                    row_heights,
                    column_widths,
                    merged_ranges,
                    cells,
                });
            }
        }
        WorkbookSnapshot {
            engine_id: ENGINE_ID,
            input_sha256: self.input_sha256.clone(),
            open_code: self.open_code,
            sheet_count,
            xf_count,
            cell_count,
            window_row: row,
            window_col: col,
            sheets,
        }
    }

    pub fn execute(&self, id: &str) -> Result<(), String> {
        let _ = self;
        Err(format!("unsupported capability: {id}"))
    }

    pub fn export(&self, format: &str) -> Result<Vec<u8>, String> {
        let _ = self;
        Err(format!("unsupported capability: export {format}"))
    }
}

fn raw_value(sheet: &KmoSheet, id: i32, raw_type: i32) -> String {
    let store_rc = sheet.getCellValueStore();
    let mut store = store_rc.borrow_mut();
    match raw_type {
        1 => format_number(store.doubleValue(id)),
        2 => store.stringValue(id),
        5 => {
            if store.booleanValue(id) {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        6 => store.errorValue(id).to_string(),
        _ => String::new(),
    }
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut state = [
        0x6a09e667u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes(chunk[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }
    let mut out = String::new();
    for word in state {
        out.push_str(&format!("{word:08x}"));
    }
    out
}

// Transport of original XF/Font getters; formatting stays in KmoSheet and KFC.
fn style_snapshot(x: &crate::cn::wps::moss::app::xf::KmoXF::KmoXF) -> serde_json::Value {
    let font = x.getFont();
    serde_json::json!({
        "formatIndex":x.getFormatIndex(),"numFormat":x.getNumFormat(),"fontIndex":x.getFontIndex(),
        "fontName":font.as_ref().map(|f|f.getFontName()),"fontHeight":font.as_ref().map(|f|f.getFontHeight()),
        "boldWeight":font.as_ref().map(|f|f.getBoldWeight()),"italic":font.as_ref().map(|f|f.isItalic()),
        "underline":font.as_ref().map(|f|f.getUnderline()),"strike":font.as_ref().map(|f|f.isStrikeout()),"superSub":font.as_ref().map(|f|f.getSuperSubScript()),"fontFamily":font.as_ref().map(|f|f.getFamily()),"charset":font.as_ref().map(|f|f.getCharset()),"fontColor":font.as_ref().map(|f|f.getColorPaletteIndex()),
        "alignment":x.getAlignment(),"verticalAlignment":x.getVerticalAlignment(),"wrapText":x.getWrapText(),
        "rotation":x.getRotation(),"indent":x.getIndent(),"shrinkToFit":x.getShrinkToFit(),
        "locked":x.isLocked(),"hidden":x.isHidden(),
        "fillPattern":x.getAdtlFillPattern(),"fillForeground":x.getFillForeground(),"fillBackground":x.getFillBackground(),
        "borderLeft":x.getBorderLeft(),"borderRight":x.getBorderRight(),"borderTop":x.getBorderTop(),"borderBottom":x.getBorderBottom(),
        "leftBorderColor":x.getLeftBorderPaletteIdx(),"rightBorderColor":x.getRightBorderPaletteIdx(),"topBorderColor":x.getTopBorderPaletteIdx(),"bottomBorderColor":x.getBottomBorderPaletteIdx()
    })
}

impl Drop for SpreadsheetSession {
    fn drop(&mut self) {
        self.cf_evaluator.requestStop();
    }
}
