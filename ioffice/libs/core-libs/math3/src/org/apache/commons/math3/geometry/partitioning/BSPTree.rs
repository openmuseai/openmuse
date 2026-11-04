// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;
// Java: import org.apache.commons.math3.geometry.Vector
use crate::org::apache::commons::math3::geometry::Vector;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public interface LeafMerger
pub trait LeafMerger {
    // Java: abstract BSPTree<S> merge(BSPTree<S> leaf, BSPTree<S> tree, BSPTree<S> parentTree, boolean isPlusChild, boolean leafFromInstance)
    fn merge(&self) -> Box<dyn std::any::Any>;
}

// Java: public interface VanishingCutHandler
pub trait VanishingCutHandler {
    // Java: abstract BSPTree<S> fixNode(BSPTree<S> node)
    fn fixNode(&self) -> Box<dyn std::any::Any>;
}

// Java: public class represent
pub struct represent {
}

// Java: public class BSPTree
pub struct BSPTree {
    // Java field: SubHyperplane<S> cut
    pub(crate) cut: Box<dyn std::any::Any>,
    // Java field: BSPTree<S> plus
    pub(crate) plus: Box<dyn std::any::Any>,
    // Java field: BSPTree<S> minus
    pub(crate) minus: Box<dyn std::any::Any>,
    // Java field: BSPTree<S> parent
    pub(crate) parent: Box<dyn std::any::Any>,
    // Java field: Object attribute
    pub(crate) attribute: Box<dyn std::any::Any>,
}

// Java: public class represent
impl represent {
}

// Java: public class BSPTree
impl BSPTree {
    // Java: BSPTree()
    pub fn BSPTree(&self) {
        // Java: cut       = null;
        // Java: plus      = null;
        // Java: minus     = null;
        // Java: parent    = null;
        // Java: attribute = null;
    }

    // Java: BSPTree(final Object attribute)
    pub fn BSPTree_2(&self) {
        // Java: cut    = null;
        // Java: plus   = null;
        // Java: minus  = null;
        // Java: parent = null;
        // Java: this.attribute = attribute;
    }

    // Java: BSPTree(final SubHyperplane<S> cut, final BSPTree<S> plus, final BSPTree<S> minus, final Object attribute)
    pub fn BSPTree_3(&self) {
    }

    // Java: boolean insertCut(final Hyperplane<S> hyperplane)
    pub fn insertCut(&self) -> bool {
        // Java: if (cut != null)
        {
            // Java: plus.parent  = null;
            // Java: minus.parent = null;
        }
        // Java: final SubHyperplane<S> chopped = fitToCell(hyperplane.wholeHyperplane());
        // Java: if (chopped == null || chopped.isEmpty())
        {
            // Java: cut          = null;
            // Java: plus         = null;
            // Java: minus        = null;
            // Java: return false;
        }
        // Java: cut          = chopped;
        // Java: plus         = new BSPTree<S>();
        // Java: plus.parent  = this;
        // Java: minus        = new BSPTree<S>();
        // Java: minus.parent = this;
        // Java: return true;
        todo!()
    }

    // Java: BSPTree<S> copySelf()
    pub fn copySelf(&self) -> Box<dyn std::any::Any> {
        // Java: if (cut == null)
        {
            // Java: return new BSPTree<S>(attribute);
        }
        // Java: return new BSPTree<S>(cut.copySelf(), plus.copySelf(), minus.copySelf(),
                           // Java: attribute);
        todo!()
    }

    // Java: SubHyperplane<S> getCut()
    pub fn getCut(&self) -> Box<dyn std::any::Any> {
        // Java: return cut;
        todo!()
    }

    // Java: BSPTree<S> getPlus()
    pub fn getPlus(&self) -> Box<dyn std::any::Any> {
        // Java: return plus;
        todo!()
    }

    // Java: BSPTree<S> getMinus()
    pub fn getMinus(&self) -> Box<dyn std::any::Any> {
        // Java: return minus;
        todo!()
    }

    // Java: BSPTree<S> getParent()
    pub fn getParent(&self) -> Box<dyn std::any::Any> {
        // Java: return parent;
        todo!()
    }

    // Java: void setAttribute(final Object attribute)
    pub fn setAttribute(&self) {
        // Java: this.attribute = attribute;
    }

    // Java: Object getAttribute()
    pub fn getAttribute(&self) -> Box<dyn std::any::Any> {
        // Java: return attribute;
        todo!()
    }

    // Java: void visit(final BSPTreeVisitor<S> visitor)
    pub fn visit(&self) {
        // Java: if (cut == null)
        {
            // Java: visitor.visitLeafNode(this);
        }
        // Java: else
        {
            // Java: switch (visitor.visitOrder(this))
            {
            // Java: case PLUS_MINUS_SUB:
                // Java: plus.visit(visitor);
                // Java: minus.visit(visitor);
                // Java: visitor.visitInternalNode(this);
                // Java: break;
            // Java: case PLUS_SUB_MINUS:
                // Java: plus.visit(visitor);
                // Java: visitor.visitInternalNode(this);
                // Java: minus.visit(visitor);
                // Java: break;
            // Java: case MINUS_PLUS_SUB:
                // Java: minus.visit(visitor);
                // Java: plus.visit(visitor);
                // Java: visitor.visitInternalNode(this);
                // Java: break;
            // Java: case MINUS_SUB_PLUS:
                // Java: minus.visit(visitor);
                // Java: visitor.visitInternalNode(this);
                // Java: plus.visit(visitor);
                // Java: break;
            // Java: case SUB_PLUS_MINUS:
                // Java: visitor.visitInternalNode(this);
                // Java: plus.visit(visitor);
                // Java: minus.visit(visitor);
                // Java: break;
            // Java: case SUB_MINUS_PLUS:
                // Java: visitor.visitInternalNode(this);
                // Java: minus.visit(visitor);
                // Java: plus.visit(visitor);
                // Java: break;
            // Java: default:
                // Java: throw new MathInternalError();
            }
        }
    }

    // Java: SubHyperplane<S> fitToCell(final SubHyperplane<S> sub)
    pub fn fitToCell(&self) -> Box<dyn std::any::Any> {
        // Java: SubHyperplane<S> s = sub;
        // Java: for (BSPTree<S> tree = this; tree.parent != null && s != null; tree = tree.parent)
        {
            // Java: if (tree == tree.parent.plus)
            {
                // Java: s = s.split(tree.parent.cut.getHyperplane()).getPlus();
            }
            // Java: else
            {
                // Java: s = s.split(tree.parent.cut.getHyperplane()).getMinus();
            }
        }
        // Java: return s;
        todo!()
    }

    // Java: BSPTree<S> getCell(final Vector<S> point)
    pub fn getCell(&self) -> Box<dyn std::any::Any> {
        // Java: return getCell((Point<S>) point, 1.0e-10);
        todo!()
    }

    // Java: BSPTree<S> getCell(final Point<S> point, final double tolerance)
    pub fn getCell_2(&self) -> Box<dyn std::any::Any> {
        // Java: if (cut == null)
        {
            // Java: return this;
        }
        // Java: // position of the point with respect to the cut hyperplane
        // Java: final double offset = cut.getHyperplane().getOffset(point);
        // Java: if (FastMath.abs(offset) < tolerance)
        {
            // Java: return this;
        }
        // Java: else if (offset <= 0)
        {
            // Java: // point is on the minus side of the cut hyperplane
            // Java: return minus.getCell(point, tolerance);
        }
        // Java: else
        {
            // Java: // point is on the plus side of the cut hyperplane
            // Java: return plus.getCell(point, tolerance);
        }
        todo!()
    }

    // Java: List<BSPTree<S>> getCloseCuts(final Point<S> point, final double maxOffset)
    pub fn getCloseCuts(&self) -> Box<dyn std::any::Any> {
        // Java: final List<BSPTree<S>> close = new ArrayList<BSPTree<S>>();
        // Java: recurseCloseCuts(point, maxOffset, close);
        // Java: return close;
        todo!()
    }

    // Java: void recurseCloseCuts(final Point<S> point, final double maxOffset, final List<BSPTree<S>> close)
    pub fn recurseCloseCuts(&self) {
    }

    // Java: void condense()
    pub fn condense(&self) {
        // Java: if ((cut != null) && (plus.cut == null) && (minus.cut == null) &&
            // Java: (((plus.attribute == null) && (minus.attribute == null)) ||
             // Java: ((plus.attribute != null) && plus.attribute.equals(minus.attribute))))
             {
            // Java: attribute = (plus.attribute == null) ? minus.attribute : plus.attribute;
            // Java: cut       = null;
            // Java: plus      = null;
            // Java: minus     = null;
             }
    }

    // Java: BSPTree<S> merge(final BSPTree<S> tree, final LeafMerger<S> leafMerger)
    pub fn merge(&self) -> Box<dyn std::any::Any> {
        // Java: return merge(tree, leafMerger, null, false);
        todo!()
    }

    // Java: BSPTree<S> merge(final BSPTree<S> tree, final LeafMerger<S> leafMerger, final BSPTree<S> parentTree, final boolean isPlusChild)
    pub fn merge_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: BSPTree<S> split(final SubHyperplane<S> sub)
    pub fn split(&self) -> Box<dyn std::any::Any> {
        // Java: if (cut == null)
        {
            // Java: return new BSPTree<S>(sub, copySelf(), new BSPTree<S>(attribute), null);
        }
        // Java: final Hyperplane<S> cHyperplane = cut.getHyperplane();
        // Java: final Hyperplane<S> sHyperplane = sub.getHyperplane();
        // Java: final SubHyperplane.SplitSubHyperplane<S> subParts = sub.split(cHyperplane);
        // Java: switch (subParts.getSide())
        {
        // Java: case PLUS :
        // Java: { // the partitioning sub-hyperplane is entirely in the plus sub-tree
            // Java: final BSPTree<S> split = plus.split(sub);
            // Java: if (cut.split(sHyperplane).getSide() == Side.PLUS)
            {
                // Java: split.plus =
                    // Java: new BSPTree<S>(cut.copySelf(), split.plus, minus.copySelf(), attribute);
                // Java: split.plus.condense();
                // Java: split.plus.parent = split;
            }
            // Java: else
            {
                // Java: split.minus =
                    // Java: new BSPTree<S>(cut.copySelf(), split.minus, minus.copySelf(), attribute);
                // Java: split.minus.condense();
                // Java: split.minus.parent = split;
            }
            // Java: return split;
        // Java: case MINUS :
        // Java: { // the partitioning sub-hyperplane is entirely in the minus sub-tree
            // Java: final BSPTree<S> split = minus.split(sub);
            // Java: if (cut.split(sHyperplane).getSide() == Side.PLUS)
            {
                // Java: split.plus =
                    // Java: new BSPTree<S>(cut.copySelf(), plus.copySelf(), split.plus, attribute);
                // Java: split.plus.condense();
                // Java: split.plus.parent = split;
            }
            // Java: else
            {
                // Java: split.minus =
                    // Java: new BSPTree<S>(cut.copySelf(), plus.copySelf(), split.minus, attribute);
                // Java: split.minus.condense();
                // Java: split.minus.parent = split;
            }
            // Java: return split;
        // Java: case BOTH :
            // Java: final SubHyperplane.SplitSubHyperplane<S> cutParts = cut.split(sHyperplane);
            // Java: final BSPTree<S> split =
                // Java: new BSPTree<S>(sub, plus.split(subParts.getPlus()), minus.split(subParts.getMinus()),
                               // Java: null);
            // Java: split.plus.cut          = cutParts.getPlus();
            // Java: split.minus.cut         = cutParts.getMinus();
            // Java: final BSPTree<S> tmp    = split.plus.minus;
            // Java: split.plus.minus        = split.minus.plus;
            // Java: split.plus.minus.parent = split.plus;
            // Java: split.minus.plus        = tmp;
            // Java: split.minus.plus.parent = split.minus;
            // Java: split.plus.condense();
            // Java: split.minus.condense();
            // Java: return split;
        // Java: default :
            // Java: return cHyperplane.sameOrientationAs(sHyperplane) ?
                   // Java: new BSPTree<S>(sub, plus.copySelf(),  minus.copySelf(), attribute) :
                   // Java: new BSPTree<S>(sub, minus.copySelf(), plus.copySelf(),  attribute);
        }
        todo!()
    }

    // Java: void insertInTree(final BSPTree<S> parentTree, final boolean isPlusChild)
    pub fn insertInTree(&self) {
        // Java: insertInTree(parentTree, isPlusChild, new VanishingCutHandler<S>()
        {
            // Java: /** {@inheritDoc} */
            // Java: public BSPTree<S> fixNode(BSPTree<S> node)
            {
                // Java: // the cut should not be null
                // Java: throw new MathIllegalStateException(LocalizedFormats.NULL_NOT_ALLOWED);
            }
        }
    }

    // Java: void insertInTree(final BSPTree<S> parentTree, final boolean isPlusChild, final VanishingCutHandler<S> vanishingHandler)
    pub fn insertInTree_2(&self) {
    }

    // Java: BSPTree<S> pruneAroundConvexCell(final Object cellAttribute, final Object otherLeafsAttributes, final Object internalAttributes)
    pub fn pruneAroundConvexCell(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void chopOffMinus(final Hyperplane<S> hyperplane, final VanishingCutHandler<S> vanishingHandler)
    pub fn chopOffMinus(&self) {
        // Java: if (cut != null)
        {
            // Java: cut = cut.split(hyperplane).getPlus();
            // Java: plus.chopOffMinus(hyperplane, vanishingHandler);
            // Java: minus.chopOffMinus(hyperplane, vanishingHandler);
            // Java: if (cut == null)
            {
                // Java: // the cut sub-hyperplane has vanished
                // Java: final BSPTree<S> fixed = vanishingHandler.fixNode(this);
                // Java: cut       = fixed.cut;
                // Java: plus      = fixed.plus;
                // Java: minus     = fixed.minus;
                // Java: attribute = fixed.attribute;
            }
        }
    }

    // Java: void chopOffPlus(final Hyperplane<S> hyperplane, final VanishingCutHandler<S> vanishingHandler)
    pub fn chopOffPlus(&self) {
        // Java: if (cut != null)
        {
            // Java: cut = cut.split(hyperplane).getMinus();
            // Java: plus.chopOffPlus(hyperplane, vanishingHandler);
            // Java: minus.chopOffPlus(hyperplane, vanishingHandler);
            // Java: if (cut == null)
            {
                // Java: // the cut sub-hyperplane has vanished
                // Java: final BSPTree<S> fixed = vanishingHandler.fixNode(this);
                // Java: cut       = fixed.cut;
                // Java: plus      = fixed.plus;
                // Java: minus     = fixed.minus;
                // Java: attribute = fixed.attribute;
            }
        }
    }

}