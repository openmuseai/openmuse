use super::Token::Token;
use super::TokenTypes::TokenTypes;
use super::XPathLexer::XPathLexer;
use crate::org::jaxen::saxpath::SAXPathException::SAXPathException;
use crate::org::jaxen::saxpath::XPathHandler::XPathHandler;
use crate::org::jaxen::saxpath::XPathSyntaxException::XPathSyntaxException;
use crate::org::jaxen::saxpath::helpers::DefaultXPathHandler::DefaultXPathHandler;

// Java: public class XPathReader implements org.jaxen.saxpath.XPathReader
pub struct XPathReader {
    // Java field: ArrayList tokens
    tokens: Vec<Token>,
    // Java field: XPathLexer lexer
    lexer: Option<XPathLexer>,
    // Java field: XPathHandler handler
    handler: Box<dyn XPathHandler>,
}

// Java: public class XPathReader implements org.jaxen.saxpath.XPathReader
impl XPathReader {
    // Java field: XPathHandler defaultHandler

    // Java: XPathReader()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.setXPathHandler(defaultHandler);
      XPathReader {
        tokens: Vec::new(),
        lexer: None,
        handler: Box::new(DefaultXPathHandler::new()),
      }
    }

    // Java: void setXPathHandler(XPathHandler handler)
    pub fn setXPathHandler(&mut self, handler: Box<dyn XPathHandler>) {
      // Java: this.handler = handler;
      self.handler = handler;
    }

    // Java: XPathHandler getXPathHandler()
    pub fn getXPathHandler(&mut self) -> &mut dyn XPathHandler {
      // Java: return this.handler;
      &mut *self.handler
    }

    // Java: void parse(String xpath)
    pub fn parse(&mut self, xpath: &str) -> Result<(), SAXPathException> {
      // Java: this.setUpParse(xpath);
      self.setUpParse(xpath);
      // Java: this.getXPathHandler().startXPath();
      self.handler.startXPath()?;
      // Java: this.expr();
      self.expr()?;
      // Java: this.getXPathHandler().endXPath();
      self.handler.endXPath()?;
      // Java: if (this.LA(1) != -1)
      if self.LA(1) != -1
      {
         // Java: XPathSyntaxException ex = this.createSyntaxException("Unexpected '" + this.LT(1).getTokenText() + "'");
         let msg = format!("Unexpected '{}'", self.LT(1).getTokenText());
         let ex = self.createSyntaxException(&msg);
         // Java: throw ex;
         return Err(SAXPathException::new(ex.toString()));
      }
      // Java: else
      else
      {
         // Java: this.lexer = null;
         self.lexer = None;
         // Java: this.tokens = null;
         self.tokens.clear();
      }
      Ok(())
    }

    // Java: void setUpParse(String xpath)
    pub fn setUpParse(&mut self, xpath: &str) {
      // Java: this.tokens = new ArrayList();
      self.tokens = Vec::new();
      // Java: this.lexer = new XPathLexer(xpath);
      self.lexer = Some(XPathLexer::new(xpath.to_string()));
    }

    // Java: void pathExpr()
    fn pathExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.getXPathHandler().startPathExpr();
      self.handler.startPathExpr()?;
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 9:
         // Java: case 14:
         // Java: case 15:
         // Java: case 17:
         9 | 14 | 15 | 17 => {
            // Java: this.locationPath(false);
            self.locationPath(false)?;
            // Java: break;
         }
         // Java: case 10:
         // Java: case 11:
         // Java: case 18:
         // Java: case 19:
         // Java: case 20:
         // Java: case 21:
         // Java: case 22:
         // Java: case 24:
         // Java: case 27:
         // Java: case 28:
         // Java: default:
         10 | 11 | 18 | 19 | 20 | 21 | 22 | 24 | 27 | 28 => {
            // Java: XPathSyntaxException ex = this.createSyntaxException("Unexpected '" + this.LT(1).getTokenText() + "'");
            let msg = format!("Unexpected '{}'", self.LT(1).getTokenText());
            let ex = self.createSyntaxException(&msg);
            // Java: throw ex;
            return Err(SAXPathException::new(ex.toString()));
         }
         // Java: case 12:
         // Java: case 13:
         12 | 13 => {
            // Java: this.locationPath(true);
            self.locationPath(true)?;
            // Java: break;
         }
         // Java: case 16:
         16 => {
            // Java: if (this.LA(2) == 23 && !this.isNodeTypeName(this.LT(1)) || this.LA(2) == 19 && this.LA(4) == 23)
            let lt1 = self.LT(1);
            if self.LA(2) == 23 && !self.isNodeTypeName(&lt1) || self.LA(2) == 19 && self.LA(4) == 23
            {
               // Java: this.filterExpr();
               self.filterExpr()?;
               // Java: if (this.LA(1) == 12 || this.LA(1) == 13)
               if self.LA(1) == 12 || self.LA(1) == 13
               {
                  // Java: this.locationPath(false);
                  self.locationPath(false)?;
               }
            }
            // Java: else
            else
            {
               // Java: this.locationPath(false);
               self.locationPath(false)?;
            }
            // Java: break;
         }
         // Java: case 23:
         // Java: case 25:
         23 | 25 => {
            // Java: this.filterExpr();
            self.filterExpr()?;
            // Java: if (this.LA(1) == 12 || this.LA(1) == 13)
            if self.LA(1) == 12 || self.LA(1) == 13
            {
               // Java: this.locationPath(false);
               self.locationPath(false)?;
            }
            // Java: break;
         }
         // Java: case 26:
         // Java: case 29:
         26 | 29 => {
            // Java: this.filterExpr();
            self.filterExpr()?;
            // Java: if (this.LA(1) == 12 || this.LA(1) == 13)
            if self.LA(1) == 12 || self.LA(1) == 13
            {
               // Java: ex = this.createSyntaxException("Node-set expected");
               let ex = self.createSyntaxException("Node-set expected");
               // Java: throw ex;
               return Err(SAXPathException::new(ex.toString()));
            }
         }
         _ => {
            let msg = format!("Unexpected '{}'", self.LT(1).getTokenText());
            let ex = self.createSyntaxException(&msg);
            return Err(SAXPathException::new(ex.toString()));
         }
      }
      // Java: this.getXPathHandler().endPathExpr();
      self.handler.endPathExpr()?;
      Ok(())
    }

    // Java: void literal()
    fn literal_expr(&mut self) -> Result<(), SAXPathException> {
      // Java: Token token = this.match(26);
      let token = self.match_token(26)?;
      // Java: this.getXPathHandler().literal(token.getTokenText());
      let text = token.getTokenText().to_string();
      self.handler.literal(&text)?;
      Ok(())
    }

    // Java: void functionCall()
    fn functionCall(&mut self) -> Result<(), SAXPathException> {
      // Java: String prefix = null;
      // Java: String functionName = null;
      let prefix;
      // Java: if (this.LA(2) == 19)
      if self.LA(2) == 19
      {
         // Java: prefix = this.match(16).getTokenText();
         prefix = self.match_token(16)?.getTokenText().to_string();
         // Java: this.match(19);
         self.match_token(19)?;
      }
      // Java: else
      else
      {
         // Java: prefix = "";
         prefix = String::new();
      }
      // Java: functionName = this.match(16).getTokenText();
      let functionName = self.match_token(16)?.getTokenText().to_string();
      // Java: this.getXPathHandler().startFunction(prefix, functionName);
      self.handler.startFunction(&prefix, &functionName)?;
      // Java: this.match(23);
      self.match_token(23)?;
      // Java: this.arguments();
      self.arguments()?;
      // Java: this.match(24);
      self.match_token(24)?;
      // Java: this.getXPathHandler().endFunction();
      self.handler.endFunction()?;
      Ok(())
    }

    // Java: void arguments()
    fn arguments(&mut self) -> Result<(), SAXPathException> {
      // Java: while(true)
      loop {
         // Java: if (this.LA(1) != 24)
         if self.LA(1) != 24 {
            // Java: this.expr();
            self.expr()?;
            // Java: if (this.LA(1) == 30)
            if self.LA(1) == 30 {
               // Java: this.match(30);
               self.match_token(30)?;
               // Java: continue;
               continue;
            }
         }
         // Java: return;
         return Ok(());
      }
    }

    // Java: void filterExpr()
    fn filterExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.getXPathHandler().startFilterExpr();
      self.handler.startFilterExpr()?;
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 16:
         16 => {
            // Java: this.functionCall();
            self.functionCall()?;
         }
         // Java: case 17:
         // Java: case 18:
         // Java: case 19:
         // Java: case 20:
         // Java: case 21:
         // Java: case 22:
         // Java: case 24:
         // Java: case 27:
         // Java: case 28:
         // Java: default:
         17 | 18 | 19 | 20 | 21 | 22 | 24 | 27 | 28 => {
            // Java: break;
         }
         // Java: case 23:
         23 => {
            // Java: this.match(23);
            self.match_token(23)?;
            // Java: this.expr();
            self.expr()?;
            // Java: this.match(24);
            self.match_token(24)?;
            // Java: break;
         }
         // Java: case 25:
         25 => {
            // Java: this.variableReference();
            self.variableReference()?;
            // Java: break;
         }
         // Java: case 26:
         26 => {
            // Java: this.literal();
            self.literal_expr()?;
            // Java: break;
         }
         // Java: case 29:
         29 => {
            // Java: Token token = this.match(29);
            let token = self.match_token(29)?;
            // Java: this.getXPathHandler().number(Double.parseDouble(token.getTokenText()));
            let val: f64 = token.getTokenText().parse().unwrap_or(0.0);
            self.handler.number_double(val)?;
         }
         _ => {}
      }
      // Java: this.predicates();
      self.predicates()?;
      // Java: this.getXPathHandler().endFilterExpr();
      self.handler.endFilterExpr()?;
      Ok(())
    }

    // Java: void variableReference()
    fn variableReference(&mut self) -> Result<(), SAXPathException> {
      // Java: this.match(25);
      self.match_token(25)?;
      // Java: String prefix = null;
      // Java: String variableName = null;
      let prefix;
      // Java: if (this.LA(2) == 19)
      if self.LA(2) == 19 {
         // Java: prefix = this.match(16).getTokenText();
         prefix = self.match_token(16)?.getTokenText().to_string();
         // Java: this.match(19);
         self.match_token(19)?;
      }
      // Java: else
      else {
         // Java: prefix = "";
         prefix = String::new();
      }
      // Java: variableName = this.match(16).getTokenText();
      let variableName = self.match_token(16)?.getTokenText().to_string();
      // Java: this.getXPathHandler().variableReference(prefix, variableName);
      self.handler.variableReference(&prefix, &variableName)?;
      Ok(())
    }

    // Java: void locationPath(boolean isAbsolute)
    pub fn locationPath(&mut self, isAbsolute: bool) -> Result<(), SAXPathException> {
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 9:
         // Java: case 14:
         // Java: case 15:
         // Java: case 16:
         // Java: case 17:
         9 | 14 | 15 | 16 | 17 => {
            // Java: this.relativeLocationPath();
            self.relativeLocationPath()?;
            // Java: break;
         }
         // Java: case 10:
         // Java: case 11:
         // Java: default:
         10 | 11 => {
            // Java: XPathSyntaxException ex = this.createSyntaxException("Unexpected '" + this.LT(1).getTokenText() + "'");
            let msg = format!("Unexpected '{}'", self.LT(1).getTokenText());
            let ex = self.createSyntaxException(&msg);
            // Java: throw ex;
            return Err(SAXPathException::new(ex.toString()));
         }
         // Java: case 12:
         // Java: case 13:
         12 | 13 => {
            // Java: if (isAbsolute)
            if isAbsolute {
               // Java: this.absoluteLocationPath();
               self.absoluteLocationPath()?;
            }
            // Java: else
            else {
               // Java: this.relativeLocationPath();
               self.relativeLocationPath()?;
            }
         }
         _ => {
            let msg = format!("Unexpected '{}'", self.LT(1).getTokenText());
            let ex = self.createSyntaxException(&msg);
            return Err(SAXPathException::new(ex.toString()));
         }
      }
      Ok(())
    }

    // Java: void absoluteLocationPath()
    fn absoluteLocationPath(&mut self) -> Result<(), SAXPathException> {
      // Java: this.getXPathHandler().startAbsoluteLocationPath();
      self.handler.startAbsoluteLocationPath()?;
      // Java: label16:
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 12:
         12 => {
            // Java: this.match(12);
            self.match_token(12)?;
            // Java: switch (this.LA(1))
            match self.LA(1) {
               // Java: case 9:
               // Java: case 14:
               // Java: case 15:
               // Java: case 16:
               // Java: case 17:
               9 | 14 | 15 | 16 | 17 => {
                  // Java: this.steps();
                  self.steps()?;
               }
               // Java: case 10:
               // Java: case 11:
               // Java: case 12:
               // Java: case 13:
               // Java: default:
               _ => {
                  // Java: break label16;
               }
            }
         }
         // Java: case 13:
         13 => {
            // Java: this.getXPathHandler().startAllNodeStep(12);
            self.handler.startAllNodeStep(12)?;
            // Java: this.getXPathHandler().endAllNodeStep();
            self.handler.endAllNodeStep()?;
            // Java: this.match(13);
            self.match_token(13)?;
            // Java: switch (this.LA(1))
            match self.LA(1) {
               // Java: case 9:
               // Java: case 14:
               // Java: case 15:
               // Java: case 16:
               // Java: case 17:
               9 | 14 | 15 | 16 | 17 => {
                  // Java: this.steps();
                  self.steps()?;
                  // Java: break;
               }
               // Java: case 10:
               // Java: case 11:
               // Java: case 12:
               // Java: case 13:
               // Java: default:
               _ => {
                  // Java: XPathSyntaxException ex = this.createSyntaxException("Location path cannot end with //");
                  let ex = self.createSyntaxException("Location path cannot end with //");
                  // Java: throw ex;
                  return Err(SAXPathException::new(ex.toString()));
               }
            }
         }
         _ => {}
      }
      // Java: this.getXPathHandler().endAbsoluteLocationPath();
      self.handler.endAbsoluteLocationPath()?;
      Ok(())
    }

    // Java: void relativeLocationPath()
    fn relativeLocationPath(&mut self) -> Result<(), SAXPathException> {
      // Java: this.getXPathHandler().startRelativeLocationPath();
      self.handler.startRelativeLocationPath()?;
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 12:
         12 => {
            // Java: this.match(12);
            self.match_token(12)?;
            // Java: break;
         }
         // Java: case 13:
         13 => {
            // Java: this.getXPathHandler().startAllNodeStep(12);
            self.handler.startAllNodeStep(12)?;
            // Java: this.getXPathHandler().endAllNodeStep();
            self.handler.endAllNodeStep()?;
            // Java: this.match(13);
            self.match_token(13)?;
         }
         _ => {}
      }
      // Java: this.steps();
      self.steps()?;
      // Java: this.getXPathHandler().endRelativeLocationPath();
      self.handler.endRelativeLocationPath()?;
      Ok(())
    }

    // Java: void steps()
    fn steps(&mut self) -> Result<(), SAXPathException> {
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case -1:
         -1 => {
            // Java: return;
            return Ok(());
         }
         // Java: case 0:
         // Java: case 1:
         // Java: case 2:
         // Java: case 3:
         // Java: case 4:
         // Java: case 5:
         // Java: case 6:
         // Java: case 7:
         // Java: case 8:
         // Java: case 10:
         // Java: case 11:
         // Java: case 12:
         // Java: case 13:
         // Java: default:
         0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 10 | 11 | 12 | 13 => {
            // Java: XPathSyntaxException ex = this.createSyntaxException("Expected one of '.', '..', '@', '*', <QName>");
            let ex = self.createSyntaxException("Expected one of '.', '..', '@', '*', <QName>");
            // Java: throw ex;
            return Err(SAXPathException::new(ex.toString()));
         }
         // Java: case 9:
         // Java: case 14:
         // Java: case 15:
         // Java: case 16:
         // Java: case 17:
         9 | 14 | 15 | 16 | 17 => {
            // Java: this.step();
            self.step()?;
            // Java: while(this.LA(1) == 12 || this.LA(1) == 13)
            while self.LA(1) == 12 || self.LA(1) == 13 {
               // Java: switch (this.LA(1))
               match self.LA(1) {
                  // Java: case 12:
                  12 => {
                     // Java: this.match(12);
                     self.match_token(12)?;
                     // Java: break;
                  }
                  // Java: case 13:
                  13 => {
                     // Java: this.getXPathHandler().startAllNodeStep(12);
                     self.handler.startAllNodeStep(12)?;
                     // Java: this.getXPathHandler().endAllNodeStep();
                     self.handler.endAllNodeStep()?;
                     // Java: this.match(13);
                     self.match_token(13)?;
                  }
                  _ => {}
               }
               // Java: switch (this.LA(1))
               match self.LA(1) {
                  // Java: case 9:
                  // Java: case 14:
                  // Java: case 15:
                  // Java: case 16:
                  // Java: case 17:
                  9 | 14 | 15 | 16 | 17 => {
                     // Java: this.step();
                     self.step()?;
                     // Java: break;
                  }
                  // Java: case 10:
                  // Java: case 11:
                  // Java: case 12:
                  // Java: case 13:
                  // Java: default:
                  _ => {
                     // Java: ex = this.createSyntaxException("Expected one of '.', '..', '@', '*', <QName>");
                     let ex = self.createSyntaxException("Expected one of '.', '..', '@', '*', <QName>");
                     // Java: throw ex;
                     return Err(SAXPathException::new(ex.toString()));
                  }
               }
            }
         }
         _ => {
            let ex = self.createSyntaxException("Expected one of '.', '..', '@', '*', <QName>");
            return Err(SAXPathException::new(ex.toString()));
         }
      }
      Ok(())
    }

    // Java: void step()
    pub fn step(&mut self) -> Result<(), SAXPathException> {
      // Java: int axis = 0;
      let mut axis = 0;
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 9:
         9 => {
            // Java: axis = 1;
            axis = 1;
         }
         // Java: case 10:
         // Java: case 11:
         // Java: case 12:
         // Java: case 13:
         // Java: default:
         10 | 11 | 12 | 13 => {
            // Java: break;
         }
         // Java: case 14:
         // Java: case 15:
         14 | 15 => {
            // Java: this.abbrStep();
            self.abbrStep()?;
            // Java: return;
            return Ok(());
         }
         // Java: case 16:
         16 => {
            // Java: if (this.LA(2) == 20)
            if self.LA(2) == 20 {
               // Java: axis = this.axisSpecifier();
               axis = self.axisSpecifier()?;
            }
            // Java: else
            else {
               // Java: axis = 1;
               axis = 1;
            }
            // Java: break;
         }
         // Java: case 17:
         17 => {
            // Java: axis = this.axisSpecifier();
            axis = self.axisSpecifier()?;
         }
         _ => {}
      }
      // Java: this.nodeTest(axis);
      self.nodeTest(axis)?;
      Ok(())
    }

    // Java: int axisSpecifier()
    fn axisSpecifier(&mut self) -> Result<i32, SAXPathException> {
      // Java: int axis = 0;
      let mut axis = 0i32;
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 16:
         16 => {
            // Java: Token token = this.LT(1);
            let token = self.LT(1);
            // Java: axis = Axis.lookup(token.getTokenText());
            axis = crate::org::jaxen::saxpath::Axis::Axis::lookup_by_name(token.getTokenText());
            // Java: if (axis == 0)
            if axis == 0 {
               // Java: this.throwInvalidAxis(token.getTokenText());
               let text = token.getTokenText().to_string();
               self.throwInvalidAxis(&text)?;
            }
            // Java: this.match(16);
            self.match_token(16)?;
            // Java: this.match(20);
            self.match_token(20)?;
            // Java: break;
         }
         // Java: case 17:
         17 => {
            // Java: this.match(17);
            self.match_token(17)?;
            // Java: axis = 9;
            axis = 9;
         }
         _ => {}
      }
      // Java: return axis;
      Ok(axis)
    }

    // Java: void nodeTest(int axis)
    fn nodeTest(&mut self, axis: i32) -> Result<(), SAXPathException> {
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 9:
         9 => {
            // Java: this.nameTest(axis);
            self.nameTest(axis)?;
            // Java: return;
            return Ok(());
         }
         // Java: case 16:
         16 => {
            // Java: switch (this.LA(2))
            match self.LA(2) {
               // Java: case 23:
               23 => {
                  // Java: this.nodeTypeTest(axis);
                  self.nodeTypeTest(axis)?;
                  // Java: return;
                  return Ok(());
               }
               // Java: default:
               _ => {
                  // Java: this.nameTest(axis);
                  self.nameTest(axis)?;
                  // Java: return;
                  return Ok(());
               }
            }
         }
         // Java: default:
         _ => {
            // Java: XPathSyntaxException ex = this.createSyntaxException("Expected <QName> or *");
            let ex = self.createSyntaxException("Expected <QName> or *");
            // Java: throw ex;
            return Err(SAXPathException::new(ex.toString()));
         }
      }
    }

    // Java: void nodeTypeTest(int axis)
    fn nodeTypeTest(&mut self, axis: i32) -> Result<(), SAXPathException> {
      // Java: Token nodeTypeToken = this.match(16);
      let nodeTypeToken = self.match_token(16)?;
      // Java: String nodeType = nodeTypeToken.getTokenText();
      let nodeType = nodeTypeToken.getTokenText().to_string();
      // Java: this.match(23);
      self.match_token(23)?;
      // Java: if ("processing-instruction".equals(nodeType))
      if nodeType == "processing-instruction" {
         // Java: String piName = "";
         let mut piName = String::new();
         // Java: if (this.LA(1) == 26)
         if self.LA(1) == 26 {
            // Java: piName = this.match(26).getTokenText();
            piName = self.match_token(26)?.getTokenText().to_string();
         }
         // Java: this.match(24);
         self.match_token(24)?;
         // Java: this.getXPathHandler().startProcessingInstructionNodeStep(axis, piName);
         self.handler.startProcessingInstructionNodeStep(axis, &piName)?;
         // Java: this.predicates();
         self.predicates()?;
         // Java: this.getXPathHandler().endProcessingInstructionNodeStep();
         self.handler.endProcessingInstructionNodeStep()?;
      }
      // Java: else if ("node".equals(nodeType))
      else if nodeType == "node" {
         // Java: this.match(24);
         self.match_token(24)?;
         // Java: this.getXPathHandler().startAllNodeStep(axis);
         self.handler.startAllNodeStep(axis)?;
         // Java: this.predicates();
         self.predicates()?;
         // Java: this.getXPathHandler().endAllNodeStep();
         self.handler.endAllNodeStep()?;
      }
      // Java: else if ("text".equals(nodeType))
      else if nodeType == "text" {
         // Java: this.match(24);
         self.match_token(24)?;
         // Java: this.getXPathHandler().startTextNodeStep(axis);
         self.handler.startTextNodeStep(axis)?;
         // Java: this.predicates();
         self.predicates()?;
         // Java: this.getXPathHandler().endTextNodeStep();
         self.handler.endTextNodeStep()?;
      }
      // Java: else
      else {
         // Java: if (!"comment".equals(nodeType))
         if nodeType != "comment" {
            // Java: XPathSyntaxException ex = this.createSyntaxException("Expected node-type");
            let ex = self.createSyntaxException("Expected node-type");
            // Java: throw ex;
            return Err(SAXPathException::new(ex.toString()));
         }
         // Java: this.match(24);
         self.match_token(24)?;
         // Java: this.getXPathHandler().startCommentNodeStep(axis);
         self.handler.startCommentNodeStep(axis)?;
         // Java: this.predicates();
         self.predicates()?;
         // Java: this.getXPathHandler().endCommentNodeStep();
         self.handler.endCommentNodeStep()?;
      }
      Ok(())
    }

    // Java: void nameTest(int axis)
    fn nameTest(&mut self, axis: i32) -> Result<(), SAXPathException> {
      // Java: String prefix = null;
      // Java: String localName = null;
      let mut prefix: Option<String> = None;
      let mut localName = String::new();
      // Java: switch (this.LA(2))
      // Java: case 19:
      if self.LA(2) == 19 {
         // Java: switch (this.LA(1))
         // Java: case 16:
         if self.LA(1) == 16 {
            // Java: prefix = this.match(16).getTokenText();
            prefix = Some(self.match_token(16)?.getTokenText().to_string());
            // Java: this.match(19);
            self.match_token(19)?;
         }
      }
      // Java: default:
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 9:
         9 => {
            // Java: this.match(9);
            self.match_token(9)?;
            // Java: localName = "*";
            localName = "*".to_string();
            // Java: break;
         }
         // Java: case 16:
         16 => {
            // Java: localName = this.match(16).getTokenText();
            localName = self.match_token(16)?.getTokenText().to_string();
         }
         _ => {}
      }
      // Java: if (prefix == null)
      if prefix.is_none() {
         // Java: prefix = "";
         prefix = Some(String::new());
      }
      // Java: this.getXPathHandler().startNameStep(axis, prefix, localName);
      self.handler.startNameStep(axis, prefix.as_ref().unwrap(), &localName)?;
      // Java: this.predicates();
      self.predicates()?;
      // Java: this.getXPathHandler().endNameStep();
      self.handler.endNameStep()?;
      Ok(())
    }

    // Java: void abbrStep()
    fn abbrStep(&mut self) -> Result<(), SAXPathException> {
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 14:
         14 => {
            // Java: this.match(14);
            self.match_token(14)?;
            // Java: this.getXPathHandler().startAllNodeStep(11);
            self.handler.startAllNodeStep(11)?;
            // Java: this.predicates();
            self.predicates()?;
            // Java: this.getXPathHandler().endAllNodeStep();
            self.handler.endAllNodeStep()?;
            // Java: break;
         }
         // Java: case 15:
         15 => {
            // Java: this.match(15);
            self.match_token(15)?;
            // Java: this.getXPathHandler().startAllNodeStep(3);
            self.handler.startAllNodeStep(3)?;
            // Java: this.predicates();
            self.predicates()?;
            // Java: this.getXPathHandler().endAllNodeStep();
            self.handler.endAllNodeStep()?;
         }
         _ => {}
      }
      Ok(())
    }

    // Java: void predicates()
    fn predicates(&mut self) -> Result<(), SAXPathException> {
      // Java: while(this.LA(1) == 21)
      while self.LA(1) == 21 {
         // Java: this.predicate();
         self.predicate()?;
      }
      Ok(())
    }

    // Java: void predicate()
    pub fn predicate(&mut self) -> Result<(), SAXPathException> {
      // Java: this.getXPathHandler().startPredicate();
      self.handler.startPredicate()?;
      // Java: this.match(21);
      self.match_token(21)?;
      // Java: this.predicateExpr();
      self.predicateExpr()?;
      // Java: this.match(22);
      self.match_token(22)?;
      // Java: this.getXPathHandler().endPredicate();
      self.handler.endPredicate()?;
      Ok(())
    }

    // Java: void predicateExpr()
    fn predicateExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.expr();
      self.expr()
    }

    // Java: void expr()
    fn expr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.orExpr();
      self.orExpr()
    }

    // Java: void orExpr()
    fn orExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.getXPathHandler().startOrExpr();
      self.handler.startOrExpr()?;
      // Java: this.andExpr();
      self.andExpr()?;
      // Java: boolean create = false;
      let mut create = false;
      // Java: switch (this.LA(1))
      // Java: case 28:
      if self.LA(1) == 28 {
         // Java: create = true;
         create = true;
         // Java: this.match(28);
         self.match_token(28)?;
         // Java: this.orExpr();
         self.orExpr()?;
      }
      // Java: default:
      // Java: this.getXPathHandler().endOrExpr(create);
      self.handler.endOrExpr(create)?;
      Ok(())
    }

    // Java: void andExpr()
    fn andExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.getXPathHandler().startAndExpr();
      self.handler.startAndExpr()?;
      // Java: this.equalityExpr();
      self.equalityExpr()?;
      // Java: boolean create = false;
      let mut create = false;
      // Java: switch (this.LA(1))
      // Java: case 27:
      if self.LA(1) == 27 {
         // Java: create = true;
         create = true;
         // Java: this.match(27);
         self.match_token(27)?;
         // Java: this.andExpr();
         self.andExpr()?;
      }
      // Java: default:
      // Java: this.getXPathHandler().endAndExpr(create);
      self.handler.endAndExpr(create)?;
      Ok(())
    }

    // Java: void equalityExpr()
    fn equalityExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.relationalExpr();
      self.relationalExpr()?;
      // Java: for(int la = this.LA(1); la == 1 || la == 2; la = this.LA(1))
      loop {
        let la = self.LA(1);
        if la != 1 && la != 2 { break; }
        // Java: switch (la)
        match la {
           // Java: case 1:
           1 => {
              // Java: this.match(1);
              self.match_token(1)?;
              // Java: this.getXPathHandler().startEqualityExpr();
              self.handler.startEqualityExpr()?;
              // Java: this.relationalExpr();
              self.relationalExpr()?;
              // Java: this.getXPathHandler().endEqualityExpr(1);
              self.handler.endEqualityExpr(1)?;
              // Java: break;
           }
           // Java: case 2:
           2 => {
              // Java: this.match(2);
              self.match_token(2)?;
              // Java: this.getXPathHandler().startEqualityExpr();
              self.handler.startEqualityExpr()?;
              // Java: this.relationalExpr();
              self.relationalExpr()?;
              // Java: this.getXPathHandler().endEqualityExpr(2);
              self.handler.endEqualityExpr(2)?;
           }
           _ => {}
        }
      }
      Ok(())
    }

    // Java: void relationalExpr()
    fn relationalExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.additiveExpr();
      self.additiveExpr()?;
      // Java: for(int la = this.LA(1); la == 3 || la == 5 || la == 4 || la == 6; la = this.LA(1))
      loop {
        let la = self.LA(1);
        if la != 3 && la != 5 && la != 4 && la != 6 { break; }
        // Java: switch (la)
        match la {
           // Java: case 3:
           3 => {
              // Java: this.match(3);
              self.match_token(3)?;
              // Java: this.getXPathHandler().startRelationalExpr();
              self.handler.startRelationalExpr()?;
              // Java: this.additiveExpr();
              self.additiveExpr()?;
              // Java: this.getXPathHandler().endRelationalExpr(3);
              self.handler.endRelationalExpr(3)?;
              // Java: break;
           }
           // Java: case 4:
           4 => {
              // Java: this.match(4);
              self.match_token(4)?;
              // Java: this.getXPathHandler().startRelationalExpr();
              self.handler.startRelationalExpr()?;
              // Java: this.additiveExpr();
              self.additiveExpr()?;
              // Java: this.getXPathHandler().endRelationalExpr(4);
              self.handler.endRelationalExpr(4)?;
              // Java: break;
           }
           // Java: case 5:
           5 => {
              // Java: this.match(5);
              self.match_token(5)?;
              // Java: this.getXPathHandler().startRelationalExpr();
              self.handler.startRelationalExpr()?;
              // Java: this.additiveExpr();
              self.additiveExpr()?;
              // Java: this.getXPathHandler().endRelationalExpr(5);
              self.handler.endRelationalExpr(5)?;
              // Java: break;
           }
           // Java: case 6:
           6 => {
              // Java: this.match(6);
              self.match_token(6)?;
              // Java: this.getXPathHandler().startRelationalExpr();
              self.handler.startRelationalExpr()?;
              // Java: this.additiveExpr();
              self.additiveExpr()?;
              // Java: this.getXPathHandler().endRelationalExpr(6);
              self.handler.endRelationalExpr(6)?;
           }
           _ => {}
        }
      }
      Ok(())
    }

    // Java: void additiveExpr()
    fn additiveExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.multiplicativeExpr();
      self.multiplicativeExpr()?;
      // Java: for(int la = this.LA(1); la == 7 || la == 8; la = this.LA(1))
      loop {
        let la = self.LA(1);
        if la != 7 && la != 8 { break; }
        // Java: switch (la)
        match la {
           // Java: case 7:
           7 => {
              // Java: this.match(7);
              self.match_token(7)?;
              // Java: this.getXPathHandler().startAdditiveExpr();
              self.handler.startAdditiveExpr()?;
              // Java: this.multiplicativeExpr();
              self.multiplicativeExpr()?;
              // Java: this.getXPathHandler().endAdditiveExpr(7);
              self.handler.endAdditiveExpr(7)?;
              // Java: break;
           }
           // Java: case 8:
           8 => {
              // Java: this.match(8);
              self.match_token(8)?;
              // Java: this.getXPathHandler().startAdditiveExpr();
              self.handler.startAdditiveExpr()?;
              // Java: this.multiplicativeExpr();
              self.multiplicativeExpr()?;
              // Java: this.getXPathHandler().endAdditiveExpr(8);
              self.handler.endAdditiveExpr(8)?;
           }
           _ => {}
        }
      }
      Ok(())
    }

    // Java: void multiplicativeExpr()
    fn multiplicativeExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.unaryExpr();
      self.unaryExpr()?;
      // Java: for(int la = this.LA(1); la == 31 || la == 11 || la == 10; la = this.LA(1))
      loop {
        let la = self.LA(1);
        if la != 31 && la != 11 && la != 10 { break; }
        // Java: switch (la)
        match la {
           // Java: case 9:
           // Java: case 31:
           9 | 31 => {
              // Java: this.match(31);
              self.match_token(31)?;
              // Java: this.getXPathHandler().startMultiplicativeExpr();
              self.handler.startMultiplicativeExpr()?;
              // Java: this.unaryExpr();
              self.unaryExpr()?;
              // Java: this.getXPathHandler().endMultiplicativeExpr(9);
              self.handler.endMultiplicativeExpr(9)?;
              // Java: break;
           }
           // Java: case 10:
           10 => {
              // Java: this.match(10);
              self.match_token(10)?;
              // Java: this.getXPathHandler().startMultiplicativeExpr();
              self.handler.startMultiplicativeExpr()?;
              // Java: this.unaryExpr();
              self.unaryExpr()?;
              // Java: this.getXPathHandler().endMultiplicativeExpr(10);
              self.handler.endMultiplicativeExpr(10)?;
              // Java: break;
           }
           // Java: case 11:
           11 => {
              // Java: this.match(11);
              self.match_token(11)?;
              // Java: this.getXPathHandler().startMultiplicativeExpr();
              self.handler.startMultiplicativeExpr()?;
              // Java: this.unaryExpr();
              self.unaryExpr()?;
              // Java: this.getXPathHandler().endMultiplicativeExpr(11);
              self.handler.endMultiplicativeExpr(11)?;
           }
           _ => {}
        }
      }
      Ok(())
    }

    // Java: void unaryExpr()
    fn unaryExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: switch (this.LA(1))
      match self.LA(1) {
         // Java: case 8:
         8 => {
            // Java: this.getXPathHandler().startUnaryExpr();
            self.handler.startUnaryExpr()?;
            // Java: this.match(8);
            self.match_token(8)?;
            // Java: this.unaryExpr();
            self.unaryExpr()?;
            // Java: this.getXPathHandler().endUnaryExpr(12);
            self.handler.endUnaryExpr(12)?;
            // Java: break;
         }
         // Java: default:
         _ => {
            // Java: this.unionExpr();
            self.unionExpr()?;
         }
      }
      Ok(())
    }

    // Java: void unionExpr()
    fn unionExpr(&mut self) -> Result<(), SAXPathException> {
      // Java: this.getXPathHandler().startUnionExpr();
      self.handler.startUnionExpr()?;
      // Java: this.pathExpr();
      self.pathExpr()?;
      // Java: boolean create = false;
      let mut create = false;
      // Java: switch (this.LA(1))
      // Java: case 18:
      if self.LA(1) == 18 {
         // Java: this.match(18);
         self.match_token(18)?;
         // Java: create = true;
         create = true;
         // Java: this.expr();
         self.expr()?;
      }
      // Java: default:
      // Java: this.getXPathHandler().endUnionExpr(create);
      self.handler.endUnionExpr(create)?;
      Ok(())
    }

    // Java: Token match(int tokenType)
    fn match_token(&mut self, tokenType: i32) -> Result<Token, SAXPathException> {
      // Java: this.LT(1);
      self.LT(1);
      // Java: Token token = (Token)this.tokens.get(0);
      let token = self.tokens[0].clone();
      // Java: if (token.getTokenType() == tokenType)
      if token.getTokenType() == tokenType {
         // Java: this.tokens.remove(0);
         self.tokens.remove(0);
         // Java: return token;
         Ok(token)
      }
      // Java: else
      else {
         // Java: XPathSyntaxException ex = this.createSyntaxException("Expected: " + TokenTypes.getTokenText(tokenType));
         let msg = format!("Expected: {}", TokenTypes::getTokenText(tokenType));
         let ex = self.createSyntaxException(&msg);
         // Java: throw ex;
         Err(SAXPathException::new(ex.toString()))
      }
    }

    // Java: int LA(int position)
    fn LA(&mut self, position: i32) -> i32 {
      // Java: return this.LT(position).getTokenType();
      self.LT(position).getTokenType()
    }

    // Java: Token LT(int position)
    fn LT(&mut self, position: i32) -> Token {
      // Java: if (this.tokens.size() <= position - 1)
      if self.tokens.len() <= (position - 1) as usize {
         // Java: for(int i = 0; i < position; ++i)
         for _i in 0..position {
            // Java: this.tokens.add(this.lexer.nextToken());
            let token = self.lexer.as_mut().unwrap().nextToken();
            self.tokens.push(token);
         }
      }
      // Java: return (Token)this.tokens.get(position - 1);
      self.tokens[(position - 1) as usize].clone()
    }

    // Java: boolean isNodeTypeName(Token name)
    fn isNodeTypeName(&self, name: &Token) -> bool {
      // Java: String text = name.getTokenText();
      let text = name.getTokenText();
      // Java: return "node".equals(text) || "comment".equals(text) || "text".equals(text) || "processing-instruction".equals(text);
      text == "node" || text == "comment" || text == "text" || text == "processing-instruction"
    }

    // Java: XPathSyntaxException createSyntaxException(String message)
    fn createSyntaxException(&mut self, message: &str) -> XPathSyntaxException {
      // Java: String xpath = this.lexer.getXPath();
      let xpath = self.lexer.as_ref().unwrap().getXPath().to_string();
      // Java: int position = this.LT(1).getTokenBegin();
      let position = self.LT(1).getTokenBegin();
      // Java: return new XPathSyntaxException(xpath, position, message);
      XPathSyntaxException::new(xpath, position, message.to_string())
    }

    // Java: void throwInvalidAxis(String invalidAxis)
    fn throwInvalidAxis(&mut self, invalidAxis: &str) -> Result<(), SAXPathException> {
      // Java: String xpath = this.lexer.getXPath();
      let xpath = self.lexer.as_ref().unwrap().getXPath().to_string();
      // Java: int position = this.LT(1).getTokenBegin();
      let position = self.LT(1).getTokenBegin();
      // Java: String message = "Expected valid axis name instead of [" + invalidAxis + "]";
      let message = format!("Expected valid axis name instead of [{}]", invalidAxis);
      // Java: throw new XPathSyntaxException(xpath, position, message);
      Err(SAXPathException::new(XPathSyntaxException::new(xpath, position, message).toString()))
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token::new(self.tokenType, self.parseText.clone(), self.tokenBegin, self.tokenEnd)
    }
}
