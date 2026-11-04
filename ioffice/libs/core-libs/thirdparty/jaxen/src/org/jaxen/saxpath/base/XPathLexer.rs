use super::Token::Token;
use super::Verifier::Verifier;

// Java: public class XPathLexer
pub struct XPathLexer {
    // Java field: String xpath
    xpath: String,
    // Java field: int currentPosition
    currentPosition: i32,
    // Java field: int endPosition
    endPosition: i32,
    // Java field: boolean expectOperator
    expectOperator: bool,
}

// Java: public class XPathLexer
impl XPathLexer {
    // Java: XPathLexer(String xpath)
    pub fn new(xpath: String) -> Self {
      // Java: super();
      // Java: this.setXPath(xpath);
      let endPosition = xpath.len() as i32;
      XPathLexer { xpath, currentPosition: 0, endPosition, expectOperator: false }
    }

    // Java: void setXPath(String xpath)
    fn setXPath(&mut self, xpath: String) {
      // Java: this.xpath = xpath;
      self.endPosition = xpath.len() as i32;
      // Java: this.currentPosition = 0;
      self.currentPosition = 0;
      // Java: this.endPosition = xpath.length();
      self.xpath = xpath;
    }

    // Java: String getXPath()
    pub fn getXPath(&self) -> &str {
      // Java: return this.xpath;
      &self.xpath
    }

    // Java: Token nextToken()
    pub fn nextToken(&mut self) -> Token {
      // Java: Token token = null;
      let mut token: Option<Token>;
      // Java: do
      loop
      {
         // Java: token = null;
         token = None;
         // Java: label52:
         // Java: switch (this.LA(1))
         match self.LA(1)
         {
            // Java: case '\t':
            // Java: case '\n':
            // Java: case '\r':
            // Java: case ' ':
            '\t' | '\n' | '\r' | ' ' => {
               // Java: token = this.whitespace();
               token = self.whitespace();
               // Java: break;
            }
            // Java: case '!':
            '!' => {
               // Java: if (this.LA(2) == '=')
               if self.LA(2) == '='
               {
                  // Java: token = this.notEquals();
                  token = Some(self.notEquals());
               }
               // Java: break;
            }
            // Java: case '"':
            // Java: case '\'':
            '"' | '\'' => {
               // Java: token = this.literal();
               token = self.literal();
               // Java: break;
            }
            // Java: case '$':
            '$' => {
               // Java: token = this.dollar();
               token = Some(self.dollar());
               // Java: break;
            }
            // Java: case '(':
            '(' => {
               // Java: token = this.leftParen();
               token = Some(self.leftParen());
               // Java: break;
            }
            // Java: case ')':
            ')' => {
               // Java: token = this.rightParen();
               token = Some(self.rightParen());
               // Java: break;
            }
            // Java: case '*':
            '*' => {
               // Java: token = this.star();
               token = Some(self.star());
               // Java: break;
            }
            // Java: case '+':
            '+' => {
               // Java: token = this.plus();
               token = Some(self.plus());
               // Java: break;
            }
            // Java: case ',':
            ',' => {
               // Java: token = this.comma();
               token = Some(self.comma());
               // Java: break;
            }
            // Java: case '-':
            '-' => {
               // Java: token = this.minus();
               token = Some(self.minus());
               // Java: break;
            }
            // Java: case '.':
            '.' => {
               // Java: switch (this.LA(2))
               match self.LA(2)
               {
                  // Java: case '0':
                  // Java: case '1':
                  // Java: case '2':
                  // Java: case '3':
                  // Java: case '4':
                  // Java: case '5':
                  // Java: case '6':
                  // Java: case '7':
                  // Java: case '8':
                  // Java: case '9':
                  '0'..='9' => {
                     // Java: token = this.number();
                     token = Some(self.number());
                     // Java: break label52;
                  }
                  // Java: default:
                  _ => {
                     // Java: token = this.dots();
                     token = Some(self.dots());
                     // Java: break label52;
                  }
               }
            }
            // Java: case '/':
            '/' => {
               // Java: token = this.slashes();
               token = Some(self.slashes());
               // Java: break;
            }
            // Java: case '0':
            // Java: case '1':
            // Java: case '2':
            // Java: case '3':
            // Java: case '4':
            // Java: case '5':
            // Java: case '6':
            // Java: case '7':
            // Java: case '8':
            // Java: case '9':
            '0'..='9' => {
               // Java: token = this.number();
               token = Some(self.number());
               // Java: break;
            }
            // Java: case ':':
            ':' => {
               // Java: if (this.LA(2) == ':')
               if self.LA(2) == ':'
               {
                  // Java: token = this.doubleColon();
                  token = Some(self.doubleColon());
               }
               // Java: else
               else
               {
                  // Java: token = this.colon();
                  token = Some(self.colon());
               }
               // Java: break;
            }
            // Java: case '<':
            // Java: case '>':
            '<' | '>' => {
               // Java: token = this.relationalOperator();
               token = self.relationalOperator();
               // Java: break;
            }
            // Java: case '=':
            '=' => {
               // Java: token = this.equals();
               token = Some(self.equals());
               // Java: break;
            }
            // Java: case '@':
            '@' => {
               // Java: token = this.at();
               token = Some(self.at());
               // Java: break;
            }
            // Java: case '[':
            '[' => {
               // Java: token = this.leftBracket();
               token = Some(self.leftBracket());
               // Java: break;
            }
            // Java: case ']':
            ']' => {
               // Java: token = this.rightBracket();
               token = Some(self.rightBracket());
               // Java: break;
            }
            // Java: case '|':
            '|' => {
               // Java: token = this.pipe();
               token = Some(self.pipe());
            }
            // Java: case '\u000b':
            // Java: case '\f':
            // Java: case '\u000e':
            // Java: case '\u000f':
            // Java: case '\u0010':
            // Java: case '\u0011':
            // Java: case '\u0012':
            // Java: case '\u0013':
            // Java: case '\u0014':
            // Java: case '\u0015':
            // Java: case '\u0016':
            // Java: case '\u0017':
            // Java: case '\u0018':
            // Java: case '\u0019':
            // Java: case '\u001a':
            // Java: case '\u001b':
            // Java: case '\u001c':
            // Java: case '\u001d':
            // Java: case '\u001e':
            // Java: case '\u001f':
            // Java: case '#':
            // Java: case '%':
            // Java: case '&':
            // Java: case ';':
            // Java: case '?':
            // Java: case 'A':
            // Java: case 'B':
            // Java: case 'C':
            // Java: case 'D':
            // Java: case 'E':
            // Java: case 'F':
            // Java: case 'G':
            // Java: case 'H':
            // Java: case 'I':
            // Java: case 'J':
            // Java: case 'K':
            // Java: case 'L':
            // Java: case 'M':
            // Java: case 'N':
            // Java: case 'O':
            // Java: case 'P':
            // Java: case 'Q':
            // Java: case 'R':
            // Java: case 'S':
            // Java: case 'T':
            // Java: case 'U':
            // Java: case 'V':
            // Java: case 'W':
            // Java: case 'X':
            // Java: case 'Y':
            // Java: case 'Z':
            // Java: case '\\':
            // Java: case '^':
            // Java: case '_':
            // Java: case '`':
            // Java: case 'a':
            // Java: case 'b':
            // Java: case 'c':
            // Java: case 'd':
            // Java: case 'e':
            // Java: case 'f':
            // Java: case 'g':
            // Java: case 'h':
            // Java: case 'i':
            // Java: case 'j':
            // Java: case 'k':
            // Java: case 'l':
            // Java: case 'm':
            // Java: case 'n':
            // Java: case 'o':
            // Java: case 'p':
            // Java: case 'q':
            // Java: case 'r':
            // Java: case 's':
            // Java: case 't':
            // Java: case 'u':
            // Java: case 'v':
            // Java: case 'w':
            // Java: case 'x':
            // Java: case 'y':
            // Java: case 'z':
            // Java: case '{':
            // Java: default:
            _ => {
               // Java: if (Verifier.isXMLNCNameStartCharacter(this.LA(1)))
               if Verifier::isXMLNCNameStartCharacter(self.LA(1))
               {
                  // Java: token = this.identifierOrOperatorName();
                  token = self.identifierOrOperatorName();
               }
               // Java: break;
            }
         }

         // Java: if (token == null)
         if token.is_none()
         {
            // Java: if (!this.hasMoreChars())
            if !self.hasMoreChars()
            {
               // Java: token = new Token(-1, this.getXPath(), this.currentPosition, this.endPosition);
               token = Some(Token::new(-1, self.xpath.clone(), self.currentPosition, self.endPosition));
            }
            // Java: else
            else
            {
               // Java: token = new Token(-3, this.getXPath(), this.currentPosition, this.endPosition);
               token = Some(Token::new(-3, self.xpath.clone(), self.currentPosition, self.endPosition));
            }
         }

         if token.as_ref().unwrap().getTokenType() != -2 { break; }
      }

      let token = token.unwrap();
      // Java: switch (token.getTokenType())
      match token.getTokenType()
      {
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
         // Java: case 17:
         // Java: case 18:
         // Java: case 19:
         // Java: case 20:
         // Java: case 21:
         // Java: case 23:
         // Java: case 25:
         // Java: case 27:
         // Java: case 28:
         // Java: case 30:
         // Java: case 31:
         1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 10 | 11 | 12 | 13 | 17 | 18 | 19 | 20 | 21 | 23 | 25 | 27 | 28 | 30 | 31 => {
            // Java: this.expectOperator = false;
            self.expectOperator = false;
            // Java: break;
         }
         // Java: case 9:
         // Java: case 14:
         // Java: case 15:
         // Java: case 16:
         // Java: case 22:
         // Java: case 24:
         // Java: case 26:
         // Java: case 29:
         // Java: default:
         _ => {
            // Java: this.expectOperator = true;
            self.expectOperator = true;
         }
      }
      // Java: return token;
      token
    }

    // Java: private Token identifierOrOperatorName()
    fn identifierOrOperatorName(&mut self) -> Option<Token>
    {
      // Java: Token token = null;
      let token;
      // Java: if (this.expectOperator)
      if self.expectOperator
      {
         // Java: token = this.operatorName();
         token = self.operatorName();
      }
      // Java: else
      else
      {
         // Java: token = this.identifier();
         token = Some(self.identifier());
      }
      // Java: return token;
      token
    }

    // Java: private Token identifier()
    fn identifier(&mut self) -> Token
    {
      // Java: Token token = null;
      // Java: int start = this.currentPosition;
      let start = self.currentPosition;
      // Java: while(this.hasMoreChars() && Verifier.isXMLNCNameCharacter(this.LA(1)))
      while self.hasMoreChars() && Verifier::isXMLNCNameCharacter(self.LA(1))
      {
         // Java: this.consume();
         self.consume();
      }
      // Java: token = new Token(16, this.getXPath(), start, this.currentPosition);
      let token = Token::new(16, self.xpath.clone(), start, self.currentPosition);
      // Java: return token;
      token
    }

    // Java: private Token operatorName()
    fn operatorName(&mut self) -> Option<Token>
    {
      // Java: Token token = null;
      let token;
      // Java: switch (this.LA(1))
      match self.LA(1)
      {
         // Java: case 'a':
         'a' => {
            // Java: token = this.and();
            token = self.and();
            // Java: break;
         }
         // Java: case 'd':
         'd' => {
            // Java: token = this.div();
            token = self.div();
            // Java: break;
         }
         // Java: case 'm':
         'm' => {
            // Java: token = this.mod();
            token = self.mod_token();
            // Java: break;
         }
         // Java: case 'o':
         'o' => {
            // Java: token = this.or();
            token = self.or();
         }
         _ => { token = None; }
      }
      // Java: return token;
      token
    }

    // Java: private Token mod()
    fn mod_token(&mut self) -> Option<Token>
    {
      // Java: Token token = null;
      let mut token = None;
      // Java: if (this.LA(1) == 'm' && this.LA(2) == 'o' && this.LA(3) == 'd')
      if self.LA(1) == 'm' && self.LA(2) == 'o' && self.LA(3) == 'd'
      {
         // Java: token = new Token(10, this.getXPath(), this.currentPosition, this.currentPosition + 3);
         token = Some(Token::new(10, self.xpath.clone(), self.currentPosition, self.currentPosition + 3));
         // Java: this.consume();
         self.consume();
         // Java: this.consume();
         self.consume();
         // Java: this.consume();
         self.consume();
      }
      // Java: return token;
      token
    }

    // Java: private Token div()
    fn div(&mut self) -> Option<Token>
    {
      // Java: Token token = null;
      let mut token = None;
      // Java: if (this.LA(1) == 'd' && this.LA(2) == 'i' && this.LA(3) == 'v')
      if self.LA(1) == 'd' && self.LA(2) == 'i' && self.LA(3) == 'v'
      {
         // Java: token = new Token(11, this.getXPath(), this.currentPosition, this.currentPosition + 3);
         token = Some(Token::new(11, self.xpath.clone(), self.currentPosition, self.currentPosition + 3));
         // Java: this.consume();
         self.consume();
         // Java: this.consume();
         self.consume();
         // Java: this.consume();
         self.consume();
      }
      // Java: return token;
      token
    }

    // Java: private Token and()
    fn and(&mut self) -> Option<Token>
    {
      // Java: Token token = null;
      let mut token = None;
      // Java: if (this.LA(1) == 'a' && this.LA(2) == 'n' && this.LA(3) == 'd')
      if self.LA(1) == 'a' && self.LA(2) == 'n' && self.LA(3) == 'd'
      {
         // Java: token = new Token(27, this.getXPath(), this.currentPosition, this.currentPosition + 3);
         token = Some(Token::new(27, self.xpath.clone(), self.currentPosition, self.currentPosition + 3));
         // Java: this.consume();
         self.consume();
         // Java: this.consume();
         self.consume();
         // Java: this.consume();
         self.consume();
      }
      // Java: return token;
      token
    }

    // Java: private Token or()
    fn or(&mut self) -> Option<Token>
    {
      // Java: Token token = null;
      let mut token = None;
      // Java: if (this.LA(1) == 'o' && this.LA(2) == 'r')
      if self.LA(1) == 'o' && self.LA(2) == 'r'
      {
         // Java: token = new Token(28, this.getXPath(), this.currentPosition, this.currentPosition + 2);
         token = Some(Token::new(28, self.xpath.clone(), self.currentPosition, self.currentPosition + 2));
         // Java: this.consume();
         self.consume();
         // Java: this.consume();
         self.consume();
      }
      // Java: return token;
      token
    }

    // Java: private Token number()
    fn number(&mut self) -> Token
    {
      // Java: int start = this.currentPosition;
      let start = self.currentPosition;
      // Java: boolean periodAllowed = true;
      let mut periodAllowed = true;
      // Java: while(true)
      loop
      {
         // Java: switch (this.LA(1))
         match self.LA(1)
         {
            // Java: case '.':
            '.' => {
               // Java: if (periodAllowed)
               if periodAllowed
               {
                  // Java: periodAllowed = false;
                  periodAllowed = false;
                  // Java: this.consume();
                  self.consume();
                  // Java: break;
                  continue;
               }
               // Java: return new Token(29, this.getXPath(), start, this.currentPosition);
               return Token::new(29, self.xpath.clone(), start, self.currentPosition);
            }
            // Java: case '0':
            // Java: case '1':
            // Java: case '2':
            // Java: case '3':
            // Java: case '4':
            // Java: case '5':
            // Java: case '6':
            // Java: case '7':
            // Java: case '8':
            // Java: case '9':
            '0'..='9' => {
               // Java: this.consume();
               self.consume();
            }
            // Java: case '/':
            // Java: default:
            _ => {
               // Java: return new Token(29, this.getXPath(), start, this.currentPosition);
               return Token::new(29, self.xpath.clone(), start, self.currentPosition);
            }
         }
      }
    }

    // Java: private Token whitespace()
    fn whitespace(&mut self) -> Option<Token>
    {
      // Java: this.consume();
      self.consume();
      // Java: for(; this.hasMoreChars(); this.consume())
      while self.hasMoreChars()
      {
         // Java: switch (this.LA(1))
         match self.LA(1)
         {
            // Java: case '\t':
            // Java: case '\n':
            // Java: case '\r':
            // Java: case ' ':
            '\t' | '\n' | '\r' | ' ' => {
               self.consume();
            }
            // Java: default:
            _ => {
               // Java: return new Token(-2, this.getXPath(), 0, 0);
               return Some(Token::new(-2, self.xpath.clone(), 0, 0));
            }
         }
      }
      // Java: return new Token(-2, this.getXPath(), 0, 0);
      Some(Token::new(-2, self.xpath.clone(), 0, 0))
    }

    // Java: private Token comma()
    fn comma(&mut self) -> Token
    {
      // Java: Token token = new Token(30, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(30, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token equals()
    fn equals(&mut self) -> Token
    {
      // Java: Token token = new Token(1, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(1, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token minus()
    fn minus(&mut self) -> Token
    {
      // Java: Token token = new Token(8, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(8, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token plus()
    fn plus(&mut self) -> Token
    {
      // Java: Token token = new Token(7, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(7, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token dollar()
    fn dollar(&mut self) -> Token
    {
      // Java: Token token = new Token(25, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(25, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token pipe()
    fn pipe(&mut self) -> Token
    {
      // Java: Token token = new Token(18, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(18, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token at()
    fn at(&mut self) -> Token
    {
      // Java: Token token = new Token(17, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(17, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token colon()
    fn colon(&mut self) -> Token
    {
      // Java: Token token = new Token(19, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(19, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token doubleColon()
    fn doubleColon(&mut self) -> Token
    {
      // Java: Token token = new Token(20, this.getXPath(), this.currentPosition, this.currentPosition + 2);
      let token = Token::new(20, self.xpath.clone(), self.currentPosition, self.currentPosition + 2);
      // Java: this.consume();
      self.consume();
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token notEquals()
    fn notEquals(&mut self) -> Token
    {
      // Java: Token token = new Token(2, this.getXPath(), this.currentPosition, this.currentPosition + 2);
      let token = Token::new(2, self.xpath.clone(), self.currentPosition, self.currentPosition + 2);
      // Java: this.consume();
      self.consume();
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token relationalOperator()
    fn relationalOperator(&mut self) -> Option<Token>
    {
      // Java: Token token = null;
      let mut token = None;
      // Java: switch (this.LA(1))
      match self.LA(1)
      {
         // Java: case '<':
         '<' => {
            // Java: if (this.LA(2) == '=')
            if self.LA(2) == '='
            {
               // Java: token = new Token(4, this.getXPath(), this.currentPosition, this.currentPosition + 2);
               token = Some(Token::new(4, self.xpath.clone(), self.currentPosition, self.currentPosition + 2));
               // Java: this.consume();
               self.consume();
            }
            // Java: else
            else
            {
               // Java: token = new Token(3, this.getXPath(), this.currentPosition, this.currentPosition + 1);
               token = Some(Token::new(3, self.xpath.clone(), self.currentPosition, self.currentPosition + 1));
            }
            // Java: this.consume();
            self.consume();
            // Java: break;
         }
         // Java: case '>':
         '>' => {
            // Java: if (this.LA(2) == '=')
            if self.LA(2) == '='
            {
               // Java: token = new Token(6, this.getXPath(), this.currentPosition, this.currentPosition + 2);
               token = Some(Token::new(6, self.xpath.clone(), self.currentPosition, self.currentPosition + 2));
               // Java: this.consume();
               self.consume();
            }
            // Java: else
            else
            {
               // Java: token = new Token(5, this.getXPath(), this.currentPosition, this.currentPosition + 1);
               token = Some(Token::new(5, self.xpath.clone(), self.currentPosition, self.currentPosition + 1));
            }
            // Java: this.consume();
            self.consume();
         }
         _ => {}
      }
      // Java: return token;
      token
    }

    // Java: private Token star()
    fn star(&mut self) -> Token
    {
      // Java: int tokenType = this.expectOperator ? 31 : 9;
      let tokenType = if self.expectOperator { 31 } else { 9 };
      // Java: Token token = new Token(tokenType, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(tokenType, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token literal()
    fn literal(&mut self) -> Option<Token>
    {
      // Java: Token token = null;
      let mut token = None;
      // Java: char match = this.LA(1);
      let match_char = self.LA(1);
      // Java: this.consume();
      self.consume();
      // Java: for(int start = this.currentPosition; token == null && this.hasMoreChars(); this.consume())
      let start = self.currentPosition;
      while token.is_none() && self.hasMoreChars()
      {
         // Java: if (this.LA(1) == match)
         if self.LA(1) == match_char
         {
            // Java: token = new Token(26, this.getXPath(), start, this.currentPosition);
            token = Some(Token::new(26, self.xpath.clone(), start, self.currentPosition));
         }
         self.consume();
      }
      // Java: return token;
      token
    }

    // Java: private Token dots()
    fn dots(&mut self) -> Token
    {
      // Java: Token token = null;
      let token;
      // Java: switch (this.LA(2))
      match self.LA(2)
      {
         // Java: case '.':
         '.' => {
            // Java: token = new Token(15, this.getXPath(), this.currentPosition, this.currentPosition + 2);
            token = Token::new(15, self.xpath.clone(), self.currentPosition, self.currentPosition + 2);
            // Java: this.consume();
            self.consume();
            // Java: this.consume();
            self.consume();
            // Java: break;
         }
         // Java: default:
         _ => {
            // Java: token = new Token(14, this.getXPath(), this.currentPosition, this.currentPosition + 1);
            token = Token::new(14, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
            // Java: this.consume();
            self.consume();
         }
      }
      // Java: return token;
      token
    }

    // Java: private Token leftBracket()
    fn leftBracket(&mut self) -> Token
    {
      // Java: Token token = new Token(21, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(21, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token rightBracket()
    fn rightBracket(&mut self) -> Token
    {
      // Java: Token token = new Token(22, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(22, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token leftParen()
    fn leftParen(&mut self) -> Token
    {
      // Java: Token token = new Token(23, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(23, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token rightParen()
    fn rightParen(&mut self) -> Token
    {
      // Java: Token token = new Token(24, this.getXPath(), this.currentPosition, this.currentPosition + 1);
      let token = Token::new(24, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
      // Java: this.consume();
      self.consume();
      // Java: return token;
      token
    }

    // Java: private Token slashes()
    fn slashes(&mut self) -> Token
    {
      // Java: Token token = null;
      let token;
      // Java: switch (this.LA(2))
      match self.LA(2)
      {
         // Java: case '/':
         '/' => {
            // Java: token = new Token(13, this.getXPath(), this.currentPosition, this.currentPosition + 2);
            token = Token::new(13, self.xpath.clone(), self.currentPosition, self.currentPosition + 2);
            // Java: this.consume();
            self.consume();
            // Java: this.consume();
            self.consume();
            // Java: break;
         }
         // Java: default:
         _ => {
            // Java: token = new Token(12, this.getXPath(), this.currentPosition, this.currentPosition + 1);
            token = Token::new(12, self.xpath.clone(), self.currentPosition, self.currentPosition + 1);
            // Java: this.consume();
            self.consume();
         }
      }
      // Java: return token;
      token
    }

    // Java: private char LA(int i)
    fn LA(&self, i: i32) -> char
    {
      // Java: return this.currentPosition + (i - 1) >= this.endPosition ? '\uffff' : this.getXPath().charAt(this.currentPosition + (i - 1));
      let pos = (self.currentPosition + (i - 1)) as usize;
      if (self.currentPosition + (i - 1)) >= self.endPosition {
        '\u{FFFF}'
      } else {
        self.xpath.as_bytes()[pos] as char
      }
    }

    // Java: private void consume()
    fn consume(&mut self)
    {
      // Java: ++this.currentPosition;
      self.currentPosition += 1;
    }

    // Java: private boolean hasMoreChars()
    fn hasMoreChars(&self) -> bool
    {
      // Java: return this.currentPosition < this.endPosition;
      self.currentPosition < self.endPosition
    }
}
