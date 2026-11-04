// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.ListIterator
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.JaxenHandler
// Java: import org.jaxen.expr.DefaultAllNodeStep
// Java: import org.jaxen.expr.DefaultCommentNodeStep
// Java: import org.jaxen.expr.DefaultFilterExpr
// Java: import org.jaxen.expr.DefaultNameStep
// Java: import org.jaxen.expr.DefaultProcessingInstructionNodeStep
// Java: import org.jaxen.expr.DefaultStep
// Java: import org.jaxen.expr.DefaultTextNodeStep
// Java: import org.jaxen.expr.DefaultXPathFactory
// Java: import org.jaxen.expr.Expr
// Java: import org.jaxen.expr.FilterExpr
// Java: import org.jaxen.expr.LocationPath
// Java: import org.jaxen.expr.Predicate
// Java: import org.jaxen.expr.PredicateSet
// Java: import org.jaxen.expr.Step
// Java: import org.jaxen.expr.UnionExpr
// Java: import org.jaxen.saxpath.SAXPathException
use crate::org::jaxen::saxpath::SAXPathException::SAXPathException;
// Java: import org.jaxen.saxpath.XPathReader
// Java: import org.jaxen.saxpath.helpers.XPathReaderFactory
use super::Pattern::Pattern;
use super::LocationPathPattern::LocationPathPattern;

// Java field: boolean TRACE
const TRACE: bool = false;
// Java field: boolean USE_HANDLER
const USE_HANDLER: bool = false;

// Java: public class PatternParser
pub struct PatternParser {
}

// Java: public class PatternParser
impl PatternParser {
    // Java: PatternParser()
    pub fn new() -> Self {
      // Java: super();
      PatternParser {}
    }

    // Java: static Pattern parse(String text)
    pub fn parse(_text: &str) -> Result<Box<dyn Pattern>, JaxenException> {
      // Java: XPathReader reader = XPathReaderFactory.createReader();
      // Java: JaxenHandler handler = new JaxenHandler();
      // Java: handler.setXPathFactory(new DefaultXPathFactory());
      // Java: reader.setXPathHandler(handler);
      // Java: reader.parse(text);
      // Java: Pattern pattern = convertExpr(handler.getXPathExpr().getRootExpr());
      // Java: return pattern.simplify();
      todo!("PatternParser::parse")
    }

    // Java: static Pattern convertExpr(Expr expr)
    pub fn convertExpr(_expr: &dyn crate::org::jaxen::expr::Expr::Expr) -> Result<Box<dyn Pattern>, JaxenException> {
      // Java: if (expr instanceof LocationPath)
      // Java: return convertExpr((LocationPath)expr);
      // Java: else if (expr instanceof FilterExpr)
      // Java: LocationPathPattern answer = new LocationPathPattern();
      // Java: answer.addFilter((FilterExpr)expr);
      // Java: return answer;
      // Java: else if (expr instanceof UnionExpr)
      // Java: UnionExpr unionExpr = (UnionExpr)expr;
      // Java: Pattern lhs = convertExpr(unionExpr.getLHS());
      // Java: Pattern rhs = convertExpr(unionExpr.getRHS());
      // Java: return new UnionPattern(lhs, rhs);
      // Java: else
      // Java: LocationPathPattern answer = new LocationPathPattern();
      // Java: answer.addFilter(new DefaultFilterExpr(expr, new PredicateSet()));
      // Java: return answer;
      todo!("PatternParser::convertExpr")
    }

    // Java: static LocationPathPattern convertExpr(LocationPath locationPath)
    pub fn convertExpr_locationPath() -> Result<LocationPathPattern, JaxenException> {
      // Java: LocationPathPattern answer = new LocationPathPattern();
      // Java: List steps = locationPath.getSteps();
      // Java: LocationPathPattern path = answer;
      // Java: boolean first = true;
      // Java: ListIterator iter = steps.listIterator(steps.size());
      // Java: while(iter.hasPrevious())
      // Java: Step step = (Step)iter.previous();
      // Java: if (first)
      // Java: first = false;
      // Java: path = convertStep(path, step);
      // Java: else
      // Java: if (navigationStep(step))
      // Java: LocationPathPattern parent = new LocationPathPattern();
      // Java: int axis = step.getAxis();
      // Java: if (axis != 2 && axis != 12)
      // Java: path.setParentPattern(parent);
      // Java: else
      // Java: path.setAncestorPattern(parent);
      // Java: path = parent;
      // Java: path = convertStep(path, step);
      // Java: if (locationPath.isAbsolute())
      // Java: LocationPathPattern parent = new LocationPathPattern(NodeTypeTest.DOCUMENT_TEST);
      // Java: path.setParentPattern(parent);
      // Java: return answer;
      todo!("PatternParser::convertExpr_locationPath")
    }

    // Java: static LocationPathPattern convertStep(LocationPathPattern path, Step step)
    pub fn convertStep() -> Result<LocationPathPattern, JaxenException> {
      // Java: if (step instanceof DefaultAllNodeStep)
      // Java: int axis = step.getAxis();
      // Java: if (axis == 9)
      // Java: path.setNodeTest(NodeTypeTest.ATTRIBUTE_TEST);
      // Java: else
      // Java: path.setNodeTest(NodeTypeTest.ELEMENT_TEST);
      // Java: else if (step instanceof DefaultCommentNodeStep)
      // Java: path.setNodeTest(NodeTypeTest.COMMENT_TEST);
      // Java: else if (step instanceof DefaultProcessingInstructionNodeStep)
      // Java: path.setNodeTest(NodeTypeTest.PROCESSING_INSTRUCTION_TEST);
      // Java: else if (step instanceof DefaultTextNodeStep)
      // Java: path.setNodeTest(TextNodeTest.SINGLETON);
      // Java: else
      // Java: if (!(step instanceof DefaultCommentNodeStep))
      // Java: if (step instanceof DefaultNameStep)
      // Java: DefaultNameStep nameStep = (DefaultNameStep)step;
      // Java: String localName = nameStep.getLocalName();
      // Java: String prefix = nameStep.getPrefix();
      // Java: int axis = nameStep.getAxis();
      // Java: short nodeType = 1;
      // Java: if (axis == 9)
      // Java: nodeType = 2;
      // Java: if (nameStep.isMatchesAnyName())
      // Java: if (prefix.length() != 0 && !prefix.equals("*"))
      // Java: path.setNodeTest(new NamespaceTest(prefix, nodeType));
      // Java: else if (axis == 9)
      // Java: path.setNodeTest(NodeTypeTest.ATTRIBUTE_TEST);
      // Java: else
      // Java: path.setNodeTest(NodeTypeTest.ELEMENT_TEST);
      // Java: else
      // Java: path.setNodeTest(new NameTest(localName, nodeType));
      // Java: return convertDefaultStep(path, nameStep);
      // Java: if (step instanceof DefaultStep)
      // Java: return convertDefaultStep(path, (DefaultStep)step);
      // Java: throw new JaxenException("Cannot convert: " + step + " to a Pattern");
      // Java: path.setNodeTest(NodeTypeTest.COMMENT_TEST);
      // Java: return path;
      todo!("PatternParser::convertStep")
    }

    // Java: static LocationPathPattern convertDefaultStep(LocationPathPattern path, DefaultStep step)
    pub fn convertDefaultStep() -> Result<LocationPathPattern, JaxenException> {
      // Java: List predicates = step.getPredicates();
      // Java: if (!predicates.isEmpty())
      // Java: FilterExpr filter = new DefaultFilterExpr(new PredicateSet());
      // Java: Iterator iter = predicates.iterator();
      // Java: while(iter.hasNext())
      // Java: filter.addPredicate((Predicate)iter.next());
      // Java: path.addFilter(filter);
      // Java: return path;
      todo!("PatternParser::convertDefaultStep")
    }

    // Java: static boolean navigationStep(Step step)
    pub fn navigationStep() -> bool {
      // Java: if (step instanceof DefaultNameStep)
      // Java: return true;
      // Java: else if (step.getClass().equals(DefaultStep.class))
      // Java: return !step.getPredicates().isEmpty();
      // Java: else
      // Java: return true;
      todo!("PatternParser::navigationStep")
    }
}
