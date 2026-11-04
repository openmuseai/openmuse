// java package cn.wps.et.ss.formula.parser;
// 按 koffice FormulaParser 的递归下降实现：
// 数字、字符串、单元格/区域、比较、加减乘除乘方百分号、一元正负、括号、IF/SUM/SUMIF。
// 未实现的产生式抛 FormulaParseException，不返回空 token。

use crate::cn::wps::et::ss::formula::atp::AnalysisToolPak::AnalysisToolPak;
use crate::cn::wps::et::ss::formula::parser::ParserOption::ParserOption;
use crate::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use crate::cn::wps::et::ss::formula::ptg::Ptg::{
    area3d_ptg, area_ptg, attr_sum, func_ptg, func_var, func_var_ext, function_by_name, int_ptg, number_ptg,
    ref3d_ptg, ref_ptg, string_ptg, to_token_array, transform_formula, FuncMeta, ParseNode, Ptg,
    PtgKind,
};
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[derive(Debug)]
pub struct FormulaParseException {
    pub message: String,
}

impl FormulaParseException {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

struct SheetIden {
    book: Option<String>,
    sheet: String,
}

struct Parser<'a> {
    formula: &'a [char],
    pointer: i32,
    look: char,
    formula_type: i32,
    sheet_index: i32,
    sv: SpreadsheetVersion,
    param_sep: char,
    workbook: &'a EvaluationWorkbook,
}

impl<'a> Parser<'a> {
    fn len(&self) -> i32 {
        self.formula.len() as i32
    }

    // java private void GetChar()
    fn get_char(&mut self) {
        if self.pointer > self.len() {
            panic!("too far");
        }
        if self.pointer < self.len() {
            self.look = self.formula[self.pointer as usize];
        } else {
            self.look = '\0';
        }
        self.pointer += 1;
    }

    fn reset_pointer(&mut self, ptr: i32) {
        self.pointer = ptr;
        if self.pointer <= self.len() && self.pointer > 0 {
            self.look = self.formula[(self.pointer - 1) as usize];
        } else {
            self.look = '\0';
        }
    }

    fn expected(&self, s: &str) -> FormulaParseException {
        FormulaParseException::new(format!(
            "Parse error near char {} '{:?}' in specified formula. Expected {s}",
            self.pointer - 1,
            self.look
        ))
    }

    fn skip_white(&mut self) {
        while is_white(self.look) {
            self.get_char();
        }
    }

    fn match_char(&mut self, x: char) -> Result<(), FormulaParseException> {
        if self.look != x {
            return Err(self.expected(&format!("'{x}'")));
        }
        self.get_char();
        Ok(())
    }

    fn get_num(&mut self) -> Option<String> {
        let mut value = String::new();
        while self.look.is_ascii_digit() {
            value.push(self.look);
            self.get_char();
        }
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    }

    fn parse_root(&mut self) -> Result<ParseNode, FormulaParseException> {
        // java this._pointer = 0; this.GetChar(); this._rootNode = this.unionExpression();
        self.pointer = 0;
        self.get_char();
        let root = self.union_expression()?;
        if self.pointer <= self.len() {
            return Err(FormulaParseException::new(format!(
                "Unused input after attempting to parse the formula"
            )));
        }
        Ok(root)
    }

    fn union_expression(&mut self) -> Result<ParseNode, FormulaParseException> {
        // 样本公式的逗号都在函数参数里，顶层不走并集。
        self.comparison_expression()
    }

    // java private ParseNode comparisonExpression()
    fn comparison_expression(&mut self) -> Result<ParseNode, FormulaParseException> {
        let mut result = self.concat_expression()?;
        loop {
            self.skip_white();
            let operator = match to_std_half_horn(self.look) {
                '<' => {
                    self.get_char();
                    match to_std_half_horn(self.look) {
                        '=' => {
                            self.get_char();
                            PtgKind::LessEqual
                        }
                        '>' => {
                            self.get_char();
                            PtgKind::NotEqual
                        }
                        _ => PtgKind::LessThan,
                    }
                }
                '=' => {
                    self.get_char();
                    PtgKind::Equal
                }
                '>' => {
                    self.get_char();
                    if to_std_half_horn(self.look) == '=' {
                        self.get_char();
                        PtgKind::GreaterEqual
                    } else {
                        PtgKind::GreaterThan
                    }
                }
                '≠' => {
                    self.get_char();
                    PtgKind::NotEqual
                }
                '≤' => {
                    self.get_char();
                    PtgKind::LessEqual
                }
                '≥' => {
                    self.get_char();
                    PtgKind::GreaterEqual
                }
                _ => return Ok(result),
            };
            let other = self.concat_expression()?;
            result = ParseNode::with(Ptg::base(operator), vec![result, other]);
        }
    }

    fn concat_expression(&mut self) -> Result<ParseNode, FormulaParseException> {
        let mut result = self.additive_expression()?;
        loop {
            self.skip_white();
            if self.look != '&' {
                return Ok(result);
            }
            self.get_char();
            let other = self.additive_expression()?;
            result = ParseNode::with(Ptg::base(PtgKind::Concat), vec![result, other]);
        }
    }

    fn additive_expression(&mut self) -> Result<ParseNode, FormulaParseException> {
        let mut result = self.term()?;
        loop {
            self.skip_white();
            let operator = match to_std_half_horn(self.look) {
                '+' => PtgKind::Add,
                '-' => PtgKind::Subtract,
                _ => return Ok(result),
            };
            self.get_char();
            let other = self.term()?;
            result = ParseNode::with(Ptg::base(operator), vec![result, other]);
        }
    }

    fn term(&mut self) -> Result<ParseNode, FormulaParseException> {
        let mut result = self.power_factor()?;
        loop {
            self.skip_white();
            let operator = match to_std_half_horn(self.look) {
                '*' | '×' => PtgKind::Multiply,
                '/' | '÷' => PtgKind::Divide,
                _ => return Ok(result),
            };
            self.get_char();
            let other = self.power_factor()?;
            result = ParseNode::with(Ptg::base(operator), vec![result, other]);
        }
    }

    // java private ParseNode powerFactor()
    fn power_factor(&mut self) -> Result<ParseNode, FormulaParseException> {
        let mut result = self.percent_factor()?;
        loop {
            self.skip_white();
            if to_std_half_horn(self.look) != '^' {
                return Ok(result);
            }
            self.get_char();
            let other = self.percent_factor()?;
            result = ParseNode::with(Ptg::base(PtgKind::Power), vec![result, other]);
        }
    }

    // java private ParseNode percentFactor()
    fn percent_factor(&mut self) -> Result<ParseNode, FormulaParseException> {
        let mut result = self.parse_simple_factor()?;
        loop {
            self.skip_white();
            if to_std_half_horn(self.look) != '%' {
                return Ok(result);
            }
            self.get_char();
            result = ParseNode::with(Ptg::base(PtgKind::Percent), vec![result]);
        }
    }

    fn parse_simple_factor(&mut self) -> Result<ParseNode, FormulaParseException> {
        self.skip_white();
        match self.look {
            '"' => Ok(ParseNode::leaf(string_ptg(self.parse_string()?))),
            '#' => Ok(ParseNode::leaf(Ptg::base(PtgKind::Err(self.parse_error_literal()?)))),
            '(' => {
                self.get_char();
                let inside = self.union_expression()?;
                if self.look != '\0' {
                    self.match_char(')')?;
                }
                Ok(ParseNode::with(Ptg::base(PtgKind::Paren), vec![inside]))
            }
            '+' => {
                self.get_char();
                self.parse_unary(true)
            }
            '-' => {
                self.get_char();
                self.parse_unary(false)
            }
            '{' => Err(self.expected("array")),
            _ => {
                if is_alpha(self.look) || self.look.is_ascii_digit() || self.look == '\'' || self.look == '['
                {
                    self.parse_range_expression()
                } else if self.look == '.' {
                    Ok(ParseNode::leaf(self.parse_number()?))
                } else {
                    Err(self.expected("cell ref or constant literal"))
                }
            }
        }
    }

    fn parse_unary(&mut self, is_plus: bool) -> Result<ParseNode, FormulaParseException> {
        let number_follows = self.look.is_ascii_digit() || self.look == '.';
        let factor = self.power_factor()?;
        if number_follows {
            match &factor.token.kind {
                PtgKind::Number(value) if !is_plus => {
                    return Ok(ParseNode::leaf(number_ptg(-value)));
                }
                PtgKind::Int(value) if !is_plus => {
                    return Ok(ParseNode::leaf(number_ptg(-(*value as f64))));
                }
                PtgKind::Number(_) | PtgKind::Int(_) if is_plus => return Ok(factor),
                _ => {}
            }
        }
        let kind = if is_plus {
            PtgKind::UnaryPlus
        } else {
            PtgKind::UnaryMinus
        };
        Ok(ParseNode::with(Ptg::base(kind), vec![factor]))
    }

    fn parse_range_expression(&mut self) -> Result<ParseNode, FormulaParseException> {
        self.parse_rangeable()
    }

    fn parse_rangeable(&mut self) -> Result<ParseNode, FormulaParseException> {
        self.skip_white();
        let mut save = self.pointer;
        // java SheetIdentifier sheetIden = this.parseSheetName();
        let sheet = self.parse_sheet_name();
        if sheet.is_none() {
            // java this.resetPointer(savePointer);
            self.reset_pointer(save);
        } else {
            // java this.SkipWhite(); savePointer = this._pointer;
            self.skip_white();
            save = self.pointer;
        }
        if let Some(part1) = self.parse_simple_range_part()? {
            self.skip_white();
            if self.look == ':' {
                self.get_char();
                self.skip_white();
                let part2 = self.parse_simple_range_part()?;
                return self.create_area(sheet.as_ref(), part1, part2);
            }
            if part1.is_cell() && self.is_valid_cell_reference(&part1.rep)? {
                return self.create_area(sheet.as_ref(), part1, None);
            }
            if sheet.is_some() {
                // java 工作表上的名称不存在时抛 FormulaParseException，不改成普通引用。
                return Err(FormulaParseException::new(
                    "Specified named range does not exist in the current workbook.",
                ));
            }
            return self.parse_non_range(save);
        }
        if sheet.is_some() {
            return Err(self.expected("defined name scoped to sheet"));
        }
        self.parse_non_range(save)
    }

    // java private SheetIdentifier parseSheetName()
    fn parse_sheet_name(&mut self) -> Option<SheetIden> {
        if self.look == '\'' {
            return self.parse_quoted_sheet();
        }
        let mut book = None;
        if self.look == '[' {
            self.get_char();
            let mut name = String::new();
            while self.look != ']' && self.look != '\0' {
                name.push(self.look);
                self.get_char();
            }
            if self.look == ']' {
                self.get_char();
            }
            book = Some(name);
        }
        if is_illegal_sheet(self.look) {
            return None;
        }
        let mut sheet = String::new();
        while is_unquoted_sheet(self.look) {
            sheet.push(self.look);
            self.get_char();
        }
        self.skip_white();
        if self.look == '!' {
            self.get_char();
            return Some(SheetIden { book, sheet });
        }
        None
    }

    fn parse_quoted_sheet(&mut self) -> Option<SheetIden> {
        // java GetChar(); done = look == '\'';
        self.get_char();
        let mut text = String::new();
        let mut done = self.look == '\'';
        while !done {
            if self.look == '\0' {
                return None;
            }
            text.push(self.look);
            self.get_char();
            if self.look == '\'' {
                self.get_char();
                done = self.look != '\'';
            }
        }
        self.skip_white();
        if self.look != '!' {
            return None;
        }
        self.get_char();
        let open = text.find('[');
        let close = text.find(']');
        if let (Some(open), Some(close)) = (open, close) {
            if open < close {
                let book = format!("{}{}", &text[..open], &text[open + 1..close]);
                return Some(SheetIden {
                    book: Some(book),
                    sheet: text[close + 1..].to_string(),
                });
            }
        }
        Some(SheetIden {
            book: None,
            sheet: text,
        })
    }

    fn parse_simple_range_part(&mut self) -> Result<Option<RangePart>, FormulaParseException> {
        let start = self.pointer - 1;
        if start < 0 || start >= self.len() {
            return Ok(None);
        }
        let mut ptr = start;
        let mut has_digits = false;
        let mut has_letters = false;
        while ptr < self.len() {
            let ch = self.formula[ptr as usize];
            if ch.is_ascii_digit() {
                has_digits = true;
            } else if ch.is_ascii_alphabetic() {
                has_letters = true;
            } else if ch != '$' {
                break;
            }
            ptr += 1;
        }
        if ptr <= start {
            return Ok(None);
        }
        let rep: String = self.formula[start as usize..ptr as usize].iter().collect();
        if !cell_ref_pattern(&rep) {
            return Ok(None);
        }
        if has_letters && has_digits {
            if !self.cell_in_range(&rep) {
                return Ok(None);
            }
        } else if has_letters {
            if !column_in_range(&rep, self.sv) {
                return Ok(None);
            }
        } else if has_digits {
            let digits: String = rep.chars().filter(|ch| *ch != '$').collect();
            let row = digits.parse::<i32>().unwrap_or(0);
            if row < 1 || row > self.sv.getLastRowIndex() + 1 {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }
        self.reset_pointer(ptr + 1);
        Ok(Some(RangePart {
            rep,
            has_letters,
            has_digits,
        }))
    }

    fn cell_in_range(&self, rep: &str) -> bool {
        parse_cell(rep, self.sv).is_some()
    }

    fn is_valid_cell_reference(&mut self, rep: &str) -> Result<bool, FormulaParseException> {
        if !self.cell_in_range(rep) {
            return Ok(false);
        }
        if function_by_name(rep).is_some() {
            let save = self.pointer;
            self.reset_pointer(self.pointer + rep.chars().count() as i32);
            self.skip_white();
            let result = self.look != '(';
            self.reset_pointer(save);
            return Ok(result);
        }
        Ok(true)
    }

    fn create_area(
        &self,
        sheet: Option<&SheetIden>,
        part1: RangePart,
        part2: Option<RangePart>,
    ) -> Result<ParseNode, FormulaParseException> {
        let Some(cell1) = parse_cell(&part1.rep, self.sv) else {
            return Err(FormulaParseException::new("not a proper reference"));
        };
        let extern_index = if let Some(sheet) = sheet {
            // java sheetIden.getBookName() != null 走外部工作簿；否则 checkExternalSheetIndex(sheet, formulaType)
            Some(self.extern_index(sheet)?)
        } else {
            None
        };
        if let Some(part2) = part2 {
            let Some(cell2) = parse_cell(&part2.rep, self.sv) else {
                return Err(FormulaParseException::new("not a proper reference"));
            };
            let token = if let Some(extern_index) = extern_index {
                area3d_ptg(
                    extern_index,
                    cell1.row,
                    cell2.row,
                    cell1.col,
                    cell2.col,
                    cell1.row_rel,
                    cell2.row_rel,
                    cell1.col_rel,
                    cell2.col_rel,
                    self.sv,
                )
            } else {
                area_ptg(
                    cell1.row,
                    cell2.row,
                    cell1.col,
                    cell2.col,
                    cell1.row_rel,
                    cell2.row_rel,
                    cell1.col_rel,
                    cell2.col_rel,
                    self.sv,
                )
            };
            Ok(ParseNode::leaf(token))
        } else {
            let token = if let Some(extern_index) = extern_index {
                ref3d_ptg(
                    extern_index,
                    cell1.row,
                    cell1.col,
                    cell1.row_rel,
                    cell1.col_rel,
                    self.sv,
                )
            } else {
                ref_ptg(
                    cell1.row,
                    cell1.col,
                    cell1.row_rel,
                    cell1.col_rel,
                    self.sv,
                )
            };
            Ok(ParseNode::leaf(token))
        }
    }

    fn extern_index(&self, sheet: &SheetIden) -> Result<i32, FormulaParseException> {
        let ext = if sheet.book.is_some() {
            self.workbook
                .checkExternalSheetIndex__book(sheet.book.as_deref().unwrap_or(""), &sheet.sheet)
        } else {
            self.workbook
                .checkExternalSheetIndex(&sheet.sheet, self.formula_type)
        };
        // java if (extIx < 0) throw new FormulaParseException("given sheet name not found");
        if ext < 0 {
            Err(FormulaParseException::new("given sheet name not found"))
        } else {
            Ok(ext)
        }
    }

    fn parse_non_range(&mut self, save: i32) -> Result<ParseNode, FormulaParseException> {
        self.reset_pointer(save);
        if !is_name_first(self.look) {
            return Ok(ParseNode::leaf(self.parse_number()?));
        }
        if self.look == '"' {
            return Ok(ParseNode::leaf(string_ptg(self.parse_string()?)));
        }
        let mut name = String::new();
        if !self.look.is_ascii_alphabetic() && self.look != '_' {
            return Err(self.expected("number, string, or defined name"));
        }
        while is_name_char(self.look) {
            name.push(self.look);
            self.get_char();
        }
        self.skip_white();
        if self.look == '(' {
            return self.function(&name);
        }
        let upper = name.to_ascii_uppercase();
        if upper == "TRUE" || upper == "FALSE" {
            return Ok(ParseNode::leaf(Ptg::base(PtgKind::Bool(upper == "TRUE"))));
        }
        // java IEvalName evalName = this._book.getNameByText(name, this._sheetIndex);
        let Some(eval_name) = self.workbook.getNameByText(&name, self.sheet_index) else {
            return Err(FormulaParseException::new(format!(
                "Specified named range '{name}' does not exist in the current workbook."
            )));
        };
        // java hasFormula 的两个分支都是 createNamePtg()
        let _ = eval_name.hasFormula();
        Ok(ParseNode::leaf(eval_name.createNamePtg()))
    }

    fn function(&mut self, name: &str) -> Result<ParseNode, FormulaParseException> {
        // java if (!AbstractFunctionPtg.isBuiltInFunctionName(name))
        let name_ptg = if function_by_name(name).is_none() {
            if AnalysisToolPak::isAtpFunction(name) {
                // java nameToken = this._book.checkUDFName(name);
                Some(self.workbook.checkUDFName(name))
            } else {
                // java getUDFName 找不到时 getUnKnowUDFName 仍返回 NamePtg
                Some(self.workbook.getUDFName(name))
            }
        } else {
            None
        };
        self.match_char('(')?;
        let args = self.arguments()?;
        if self.look != '\0' {
            self.match_char(')')?;
        }
        self.get_function(name, name_ptg, args)
    }

    fn arguments(&mut self) -> Result<Vec<ParseNode>, FormulaParseException> {
        let mut temp = Vec::new();
        self.skip_white();
        if self.look == ')' {
            return Ok(temp);
        }
        let mut missed = true;
        loop {
            self.skip_white();
            if self.look == '\0' || self.look == self.param_sep || self.look == ')' {
                if missed {
                    return Err(self.expected("argument"));
                }
                if self.look == '\0' || self.look == ')' {
                    return Ok(temp);
                }
                self.match_char(self.param_sep)?;
                missed = true;
            } else {
                temp.push(self.comparison_expression()?);
                missed = false;
                self.skip_white();
                if self.look != self.param_sep && self.look != ')' && self.look != '\0' {
                    return Err(self.expected("',' or ')'"));
                }
            }
        }
    }

    fn get_function(
        &self,
        name: &str,
        name_ptg: Option<Ptg>,
        args: Vec<ParseNode>,
    ) -> Result<ParseNode, FormulaParseException> {
        // java FunctionMetadata fm = FunctionMetadataRegistry.getFunctionByName(name.toUpperCase());
        let Some(meta) = function_by_name(name) else {
            // java NamePtg 作为第一个参数，FuncVarPtg.create(name, numArgs + 1)，index 255
            let Some(name_ptg) = name_ptg else {
                panic!("IllegalStateException");
            };
            let mut all = Vec::with_capacity(args.len() + 1);
            all.push(ParseNode::leaf(name_ptg));
            all.extend(args);
            let nargs = all.len() as i32;
            return Ok(ParseNode::with(func_var_ext(nargs), all));
        };
        if name_ptg.is_some() {
            panic!("IllegalStateException");
        }
        self.validate_num_args(args.len() as i32, meta)?;
        if meta.getIndex() == 4 && args.len() == 1 {
            return Ok(ParseNode::with(attr_sum(), args));
        }
        // java isVarArgs ? FuncVarPtg.create(name, numArgs) : FuncPtg.create(funcIx)
        let ptg = if meta.has_fixed_args() { func_ptg(meta) } else { func_var(meta, args.len() as i32) };
        Ok(ParseNode::with(ptg, args))
    }

    fn validate_num_args(&self, num_args: i32, meta: &FuncMeta) -> Result<(), FormulaParseException> {
        if num_args < meta.getMinParams() || num_args > meta.getMaxParams() {
            return Err(FormulaParseException::new(format!(
                "argument count {num_args} for {}",
                meta.getName()
            )));
        }
        Ok(())
    }

    fn parse_number(&mut self) -> Result<Ptg, FormulaParseException> {
        let number1 = self.get_num();
        let mut number2 = None;
        let mut exponent = None;
        if self.look == '.' {
            self.get_char();
            number2 = self.get_num();
        }
        if self.look == 'E' || self.look == 'e' {
            self.get_char();
            let mut sign = String::new();
            if self.look == '+' {
                self.get_char();
            } else if self.look == '-' {
                self.get_char();
                sign.push('-');
            }
            let Some(number) = self.get_num() else {
                return Err(self.expected("Integer"));
            };
            exponent = Some(format!("{sign}{number}"));
        }
        if number1.is_none() && number2.is_none() {
            return Err(self.expected("Integer"));
        }
        Ok(number_from_parts(number1, number2, exponent))
    }

    // java private int parseErrorLiteral()
    fn parse_error_literal(&mut self) -> Result<u8, FormulaParseException> {
        self.match_char('#')?;
        let Some(part1) = self.parse_unquoted_identifier() else {
            return Err(self.expected("remainder of error constant literal"));
        };
        let part1 = part1.to_ascii_uppercase();
        let code = match part1.chars().next() {
            Some('D') if part1 == "DIV" => {
                self.match_char('/')?;
                self.match_char('0')?;
                self.match_char('!')?;
                7
            }
            Some('N') if part1 == "NAME" => {
                self.match_char('?')?;
                29
            }
            Some('N') if part1 == "NUM" => {
                self.match_char('!')?;
                36
            }
            Some('N') if part1 == "NULL" => {
                self.match_char('!')?;
                0
            }
            Some('N') if part1 == "N" => {
                self.match_char('/')?;
                let half = to_std_half_horn(self.look);
                if half != 'A' && half != 'a' {
                    return Err(self.expected("#N/A"));
                }
                self.get_char();
                42
            }
            Some('R') if part1 == "REF" => {
                self.match_char('!')?;
                23
            }
            Some('V') if part1 == "VALUE" => {
                self.match_char('!')?;
                15
            }
            _ => {
                return Err(self.expected(
                    "#VALUE!, #REF!, #DIV/0!, #NAME?, #NUM!, #NULL! or #N/A",
                ));
            }
        };
        Ok(code)
    }

    // java private String parseUnquotedIdentifier()
    fn parse_unquoted_identifier(&mut self) -> Option<String> {
        if to_std_half_horn(self.look) == '\'' {
            return None;
        }
        let mut text = String::new();
        loop {
            let half = to_std_half_horn(self.look);
            if !(half.is_ascii_alphanumeric() || half == '.') {
                break;
            }
            text.push(half);
            self.get_char();
        }
        if text.is_empty() { None } else { Some(text) }
    }

    fn parse_string(&mut self) -> Result<String, FormulaParseException> {
        self.match_char('"')?;
        let mut token = String::new();
        loop {
            if self.look == '"' {
                self.get_char();
                if self.look != '"' {
                    return Ok(token);
                }
            }
            if self.look == '\0' {
                return Err(self.expected("string end"));
            }
            token.push(self.look);
            self.get_char();
        }
    }
}

fn number_from_parts(number1: Option<String>, number2: Option<String>, exponent: Option<String>) -> Ptg {
    if let Some(number2) = number2 {
        let mut number = String::new();
        if let Some(number1) = number1 {
            number.push_str(&number1);
        }
        number.push('.');
        number.push_str(&number2);
        if let Some(exponent) = exponent {
            number.push('E');
            number.push_str(&exponent);
        }
        let value = number.parse::<f64>().unwrap_or(f64::NAN);
        return number_ptg(value);
    }
    let mut number = number1.unwrap_or_default();
    if let Some(exponent) = exponent {
        number.push('E');
        number.push_str(&exponent);
    }
    if let Ok(int_val) = number.parse::<i32>() {
        if (0..=65535).contains(&int_val) {
            return int_ptg(int_val);
        }
    }
    number_ptg(number.parse::<f64>().unwrap_or(f64::NAN))
}

struct CellPoint {
    row: i32,
    col: i32,
    row_rel: bool,
    col_rel: bool,
}

struct RangePart {
    rep: String,
    has_letters: bool,
    has_digits: bool,
}

impl RangePart {
    fn is_cell(&self) -> bool {
        self.has_letters && self.has_digits
    }
}

fn parse_cell(rep: &str, sv: SpreadsheetVersion) -> Option<CellPoint> {
    let mut col_rel = true;
    let mut row_rel = true;
    let chars: Vec<char> = rep.chars().collect();
    let mut i = 0;
    if chars.first() == Some(&'$') {
        col_rel = false;
        i += 1;
    }
    let col_start = i;
    while i < chars.len() && chars[i].is_ascii_alphabetic() {
        i += 1;
    }
    if i == col_start {
        return None;
    }
    let col = column_index(&chars[col_start..i])?;
    if chars.get(i) == Some(&'$') {
        row_rel = false;
        i += 1;
    }
    if i >= chars.len() {
        return None;
    }
    let row_text: String = chars[i..].iter().collect();
    if !row_text.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    let row_num = row_text.parse::<i32>().ok()?;
    let row = row_num - 1;
    if row < 0 || row > sv.getLastRowIndex() || col > sv.getLastColumnIndex() {
        return None;
    }
    Some(CellPoint {
        row,
        col,
        row_rel,
        col_rel,
    })
}

fn column_index(letters: &[char]) -> Option<i32> {
    let mut col = 0i32;
    for ch in letters {
        let digit = ch.to_ascii_uppercase() as i32 - 'A' as i32 + 1;
        col = col.checked_mul(26)?.checked_add(digit)?;
    }
    Some(col - 1)
}

fn column_in_range(rep: &str, sv: SpreadsheetVersion) -> bool {
    let letters: String = rep.chars().filter(|ch| *ch != '$').collect();
    match column_index(&letters.chars().collect::<Vec<_>>()) {
        Some(col) => col >= 0 && col <= sv.getLastColumnIndex(),
        None => false,
    }
}

fn cell_ref_pattern(rep: &str) -> bool {
    let chars: Vec<char> = rep.chars().collect();
    let mut i = 0;
    if chars.get(i) == Some(&'$') {
        i += 1;
    }
    let letter_start = i;
    while i < chars.len() && chars[i].is_ascii_alphabetic() {
        i += 1;
    }
    let has_letters = i > letter_start;
    if chars.get(i) == Some(&'$') {
        i += 1;
    }
    let digit_start = i;
    while i < chars.len() && chars[i].is_ascii_digit() {
        i += 1;
    }
    let has_digits = i > digit_start;
    i == chars.len() && (has_letters || has_digits)
}

// java public static char toStdHalfHorn(char c) 里不依赖 LangHelper 的分支。
fn to_std_half_horn(ch: char) -> char {
    match ch {
        '—' | '―' => '-',
        '‘' | '’' => '\'',
        '【' => '[',
        '】' => ']',
        '〓' => '=',
        _ => ch,
    }
}

// java public static char toStdHalfHorn(char c){char ret=LangHelper.toHalfHornChar(c);switch(ret){case '—':case '―':return '-';case '‘':case '’':return '\'';case '【':return '[';case '】':return ']';case '〓':return '=';default:return ret;}}
pub fn toStdHalfHorn(c:u16)->u16{let ret=crate::cn::wps::et::ss::constant::LangHelper::LangHelper::toHalfHornChar(c);match ret{0x2014|0x2015=>45,0x2018|0x2019=>39,0x3010=>91,0x3011=>93,0x3013=>61,_=>ret}}
// java public static boolean isValidDefinedNameFirstChar(char ch){char half_horn=toStdHalfHorn(ch);if((half_horn&'\uff00')==0&&!Character.isLetter(half_horn)&&half_horn!='_'){switch(ch){case '—':case '‘':case '’':case '“':case '”':case '【':case '】':return true;default:return false;}}else{return true;}}
pub fn isValidDefinedNameFirstChar(ch:u16)->bool{let half_horn=toStdHalfHorn(ch);if half_horn&0xff00==0&&!crate::java_runtime::Character::isLetter(half_horn)&&half_horn!=95{matches!(ch,0x2014|0x2018|0x2019|0x201c|0x201d|0x3010|0x3011)}else{true}}
// java public static boolean isPivotValidDefinedNameFirstChar(char ch){char half_horn=toStdHalfHorn(ch);if((half_horn&'\uff00')==0&&!Character.isLetter(half_horn)&&half_horn!='_'){switch(ch){case '—':case '“':case '”':case '【':case '】':return true;default:return false;}}else{return true;}}
pub fn isPivotValidDefinedNameFirstChar(ch:u16)->bool{let half_horn=toStdHalfHorn(ch);if half_horn&0xff00==0&&!crate::java_runtime::Character::isLetter(half_horn)&&half_horn!=95{matches!(ch,0x2014|0x201c|0x201d|0x3010|0x3011)}else{true}}
// java public static boolean isValidDefinedNameChar(char ch){char half_horn=toStdHalfHorn(ch);if((half_horn&'\uff00')==0&&!Character.isLetterOrDigit(half_horn)){switch(half_horn){case '.':case '?':case '\\':case '_':return true;default:switch(ch){case '—':case '‘':case '’':case '“':case '”':case '【':case '】':return true;default:return false;}}}else{return true;}}
pub fn isValidDefinedNameChar(ch:u16)->bool{let half_horn=toStdHalfHorn(ch);if half_horn&0xff00==0&&!crate::java_runtime::Character::isLetter(half_horn)&&!crate::java_runtime::Character::isDigit(half_horn){if matches!(half_horn,46|63|92|95){true}else{matches!(ch,0x2014|0x2018|0x2019|0x201c|0x201d|0x3010|0x3011)}}else{true}}

fn is_white(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n'
}
fn is_alpha(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '$' || ch == '_'
}
fn is_name_first(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}
fn is_name_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '.' || ch == '?' || ch == '\\' || ch == '_'
}
fn is_unquoted_sheet(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '.' || ch == '_'
}
fn is_illegal_sheet(ch: char) -> bool {
    matches!(ch, '\\' | '/' | '?' | '*' | '[' | ']' | '\'')
}

fn filter_formula(formula: &str) -> Vec<char> {
    let mut chars: Vec<char> = formula.chars().collect();
    if chars.len() > 1 && chars[0] == '=' {
        chars.remove(0);
    }
    chars.retain(|ch| *ch != '\r' && *ch != '\n');
    chars
}

// java public static Ptg[] parse(String formula, IEvalWorkbook workbook, int formulaType, int sheetIndex, SpreadsheetVersion sv, ParserOption opt)
pub fn parse(
    formula: &str,
    workbook: &EvaluationWorkbook,
    formula_type: i32,
    sheet_index: i32,
    sv: SpreadsheetVersion,
    opt: &ParserOption,
) -> Result<Vec<Ptg>, FormulaParseException> {
    let chars = filter_formula(formula);
    let mut parser = Parser {
        formula: &chars,
        pointer: 0,
        look: '\0',
        formula_type,
        sheet_index,
        sv,
        param_sep: opt.PARAMETER_SEPARATOR,
        workbook,
    };
    let _ = parser.formula_type;
    let _ = workbook.book();
    let mut root = parser.parse_root()?;
    // java OperandClassTransformer.transformFormula(formulaType, root)
    transform_formula(&mut root, formula_type).map_err(FormulaParseException::new)?;
    let tokens = to_token_array(&root);
    if tokens.is_empty() {
        return Err(FormulaParseException::new("empty token"));
    }
    Ok(tokens)
}
