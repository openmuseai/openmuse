// Java: import java.util.Arrays
// Java: import java.util.List
// Java: import org.apache.commons.math3.fraction.BigFraction
use crate::org::apache::commons::math3::fraction::BigFraction;
// Java: import org.apache.commons.math3.geometry.enclosing.EnclosingBall
use crate::org::apache::commons::math3::geometry::enclosing::EnclosingBall;
// Java: import org.apache.commons.math3.geometry.enclosing.SupportBallGenerator
use crate::org::apache::commons::math3::geometry::enclosing::SupportBallGenerator;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.DiskGenerator
use crate::org::apache::commons::math3::geometry::euclidean::twod::DiskGenerator;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Euclidean2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Euclidean2D;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class SphereGenerator implements SupportBallGenerator
pub struct SphereGenerator {
}

// Java: public class SphereGenerator implements SupportBallGenerator
#[allow(non_snake_case)]
impl SphereGenerator {
    // Java: public EnclosingBall<Euclidean3D, Vector3D> ballOnSupport(final List<Vector3D> support)
    pub fn ballOnSupport(&self, _support: &[Box<dyn std::any::Any>]) -> Box<dyn std::any::Any> {
        // Java: if (support.size() < 1)
        {
            // Java: return new EnclosingBall<Euclidean3D, Vector3D>(Vector3D.ZERO, Double.NEGATIVE_INFINITY);
        }
        // Java: else
        {
            // Java: final Vector3D vA = support.get(0);
            // Java: if (support.size() < 2)
            {
                // Java: return new EnclosingBall<Euclidean3D, Vector3D>(vA, 0, vA);
            }
            // Java: else
            {
                // Java: final Vector3D vB = support.get(1);
                // Java: if (support.size() < 3)
                {
                    // Java: return new EnclosingBall<Euclidean3D, Vector3D>(new Vector3D(0.5, vA, 0.5, vB),
                    // Java: 0.5 * vA.distance(vB),
                    // Java: vA, vB);
                }
                // Java: else
                {
                    // Java: final Vector3D vC = support.get(2);
                    // Java: if (support.size() < 4)
                    {
                        // Java: // delegate to 2D disk generator
                        // Java: final Plane p = new Plane(vA, vB, vC,
                        // Java: 1.0e-10 * (vA.getNorm1() + vB.getNorm1() + vC.getNorm1()));
                        // Java: final EnclosingBall<Euclidean2D, Vector2D> disk =
                        // Java: new DiskGenerator().ballOnSupport(Arrays.asList(p.toSubSpace(vA),
                        // Java: p.toSubSpace(vB),
                        // Java: p.toSubSpace(vC)));
                        // Java: // convert back to 3D
                        // Java: return new EnclosingBall<Euclidean3D, Vector3D>(p.toSpace(disk.getCenter()),
                        // Java: disk.getRadius(), vA, vB, vC);
                    }
                    // Java: else
                    {
                        // Java: final Vector3D vD = support.get(3);
                        // Java: // a sphere is 3D can be defined as:
                        // Java: // (1)   (x - x_0)^2 + (y - y_0)^2 + (z - z_0)^2 = r^2
                        // Java: // which can be written:
                        // Java: // (2)   (x^2 + y^2 + z^2) - 2 x_0 x - 2 y_0 y - 2 z_0 z + (x_0^2 + y_0^2 + z_0^2 - r^2) = 0
                        // Java: // or simply:
                        // Java: // (3)   (x^2 + y^2 + z^2) + a x + b y + c z + d = 0
                        // Java: // with sphere center coordinates -a/2, -b/2, -c/2
                        // Java: // If the sphere exists, a b, c and d are a non zero solution to
                        // Java: // [ (x^2  + y^2  + z^2)    x    y   z    1 ]   [ 1 ]   [ 0 ]
                        // Java: // [ (xA^2 + yA^2 + zA^2)   xA   yA  zA   1 ]   [ a ]   [ 0 ]
                        // Java: // [ (xB^2 + yB^2 + zB^2)   xB   yB  zB   1 ] * [ b ] = [ 0 ]
                        // Java: // [ (xC^2 + yC^2 + zC^2)   xC   yC  zC   1 ]   [ c ]   [ 0 ]
                        // Java: // [ (xD^2 + yD^2 + zD^2)   xD   yD  zD   1 ]   [ d ]   [ 0 ]
                        // Java: // So the determinant of the matrix is zero. Computing this determinant
                        // Java: // by expanding it using the minors m_ij of first row leads to
                        // Java: // (4)   m_11 (x^2 + y^2 + z^2) - m_12 x + m_13 y - m_14 z + m_15 = 0
                        // Java: // So by identifying equations (2) and (4) we get the coordinates
                        // Java: // of center as:
                        // Java: //      x_0 = +m_12 / (2 m_11)
                        // Java: //      y_0 = -m_13 / (2 m_11)
                        // Java: //      z_0 = +m_14 / (2 m_11)
                        // Java: // Note that the minors m_11, m_12, m_13 and m_14 all have the last column
                        // Java: // filled with 1.0, hence simplifying the computation
                        // Java: final BigFraction[] c2 = new BigFraction[]
                        {
                            // Java: new BigFraction(vA.getX()), new BigFraction(vB.getX()),
                            // Java: new BigFraction(vC.getX()), new BigFraction(vD.getX())
                        }
                        // Java: final BigFraction[] c3 = new BigFraction[]
                        {
                            // Java: new BigFraction(vA.getY()), new BigFraction(vB.getY()),
                            // Java: new BigFraction(vC.getY()), new BigFraction(vD.getY())
                        }
                        // Java: final BigFraction[] c4 = new BigFraction[]
                        {
                            // Java: new BigFraction(vA.getZ()), new BigFraction(vB.getZ()),
                            // Java: new BigFraction(vC.getZ()), new BigFraction(vD.getZ())
                        }
                        // Java: final BigFraction[] c1 = new BigFraction[]
                        {
                            // Java: c2[0].multiply(c2[0]).add(c3[0].multiply(c3[0])).add(c4[0].multiply(c4[0])),
                            // Java: c2[1].multiply(c2[1]).add(c3[1].multiply(c3[1])).add(c4[1].multiply(c4[1])),
                            // Java: c2[2].multiply(c2[2]).add(c3[2].multiply(c3[2])).add(c4[2].multiply(c4[2])),
                            // Java: c2[3].multiply(c2[3]).add(c3[3].multiply(c3[3])).add(c4[3].multiply(c4[3]))
                        }
                        // Java: final BigFraction twoM11  = minor(c2, c3, c4).multiply(2);
                        // Java: final BigFraction m12     = minor(c1, c3, c4);
                        // Java: final BigFraction m13     = minor(c1, c2, c4);
                        // Java: final BigFraction m14     = minor(c1, c2, c3);
                        // Java: final BigFraction centerX = m12.divide(twoM11);
                        // Java: final BigFraction centerY = m13.divide(twoM11).negate();
                        // Java: final BigFraction centerZ = m14.divide(twoM11);
                        // Java: final BigFraction dx      = c2[0].subtract(centerX);
                        // Java: final BigFraction dy      = c3[0].subtract(centerY);
                        // Java: final BigFraction dz      = c4[0].subtract(centerZ);
                        // Java: final BigFraction r2      = dx.multiply(dx).add(dy.multiply(dy)).add(dz.multiply(dz));
                        // Java: return new EnclosingBall<Euclidean3D, Vector3D>(new Vector3D(centerX.doubleValue(),
                        // Java: centerY.doubleValue(),
                        // Java: centerZ.doubleValue()),
                        // Java: FastMath.sqrt(r2.doubleValue()),
                        // Java: vA, vB, vC, vD);
                    }
                }
            }
        }
        todo!()
    }

    // Java: BigFraction minor(final BigFraction[] c1, final BigFraction[] c2, final BigFraction[] c3)
    pub fn minor(&self, _c1: &[Box<dyn std::any::Any>], _c2: &[Box<dyn std::any::Any>], _c3: &[Box<dyn std::any::Any>]) -> Box<dyn std::any::Any> {
        // Java: return      c2[0].multiply(c3[1]).multiply(c1[2].subtract(c1[3])).
        // Java: add(c2[0].multiply(c3[2]).multiply(c1[3].subtract(c1[1]))).
        // Java: add(c2[0].multiply(c3[3]).multiply(c1[1].subtract(c1[2]))).
        // Java: add(c2[1].multiply(c3[0]).multiply(c1[3].subtract(c1[2]))).
        // Java: add(c2[1].multiply(c3[2]).multiply(c1[0].subtract(c1[3]))).
        // Java: add(c2[1].multiply(c3[3]).multiply(c1[2].subtract(c1[0]))).
        // Java: add(c2[2].multiply(c3[0]).multiply(c1[1].subtract(c1[3]))).
        // Java: add(c2[2].multiply(c3[1]).multiply(c1[3].subtract(c1[0]))).
        // Java: add(c2[2].multiply(c3[3]).multiply(c1[0].subtract(c1[1]))).
        // Java: add(c2[3].multiply(c3[0]).multiply(c1[2].subtract(c1[1]))).
        // Java: add(c2[3].multiply(c3[1]).multiply(c1[0].subtract(c1[2]))).
        // Java: add(c2[3].multiply(c3[2]).multiply(c1[1].subtract(c1[0])));
        todo!()
    }

}
