// java package cn.wps.moss.engine.linktb;

// java import cn.wps.et.ss.formula.ptg.Ptg;
use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;
// java import cn.wps.et.ss.formula.Formula;
use crate::cn::wps::et::ss::formula::Formula::Formula;
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

// java public final class ExternalNameRecord extends StandardRecord
#[derive(Clone)]
pub struct ExternalNameRecord {
    // java private short field_2_ixals;
    ix: i16,
    // java private String field_4_name;
    text: String,
    // java private Formula field_5_name_definition;
    name_definition: Option<Formula>,
}

impl ExternalNameRecord {
    // java public ExternalNameRecord()
    pub fn ExternalNameRecord() -> Self {
        // java this.field_2_ixals = 0;
        Self { ix: 0, text: String::new(), name_definition: None }
    }

    // java public String getText()
    pub fn getText(&self) -> &str {
        // java return this.field_4_name;
        &self.text
    }

    // java public void setText(String str)
    pub fn setText(&mut self, text: &str) {
        // java this.field_4_name = str;
        self.text = text.to_string();
    }

    // java public short getIx()
    pub fn getIx(&self) -> i16 {
        // java return this.field_2_ixals;
        self.ix
    }

    // java public void setIx(short ix)
    pub fn setIx(&mut self, ix: i16) {
        // java this.field_2_ixals = ix;
        self.ix = ix;
    }

    // java public Ptg[] getParsedExpression()
    pub fn getParsedExpression(&self) -> Option<Vec<Ptg>> {
        // java return Formula.getTokens(this.field_5_name_definition, true);
        self.name_definition.as_ref().map(|formula| formula.getTokens__external(true))
    }

    // java public void setParsedExpression(Ptg[] ptgs)
    pub fn setParsedExpression(&mut self, ptgs: Vec<Ptg>) {
        // java this.field_5_name_definition = Formula.create(ptgs, SpreadsheetVersion.EXCEL97);
        self.name_definition = Some(Formula::create(&ptgs, SpreadsheetVersion::EXCEL97));
    }
}
