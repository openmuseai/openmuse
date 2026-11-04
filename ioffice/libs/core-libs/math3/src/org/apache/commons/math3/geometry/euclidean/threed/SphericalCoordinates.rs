#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

// Java: import java.io.Serializable
// (no direct Rust equivalent)
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
use super::Vector3D::Vector3D;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 20130206;

// Java: public class provides
pub struct provides {
}

// Java: public class SphericalCoordinates
pub struct SphericalCoordinates {
    // Java field: Vector3D v
    pub(crate) v: Vector3D,
    // Java field: double r
    pub(crate) r: f64,
    // Java field: double theta
    pub(crate) theta: f64,
    // Java field: double phi
    pub(crate) phi: f64,
    // Java field: double[][] jacobian
    pub(crate) jacobian: Option<Vec<Vec<f64>>>,
    // Java field: double[][] rHessian
    pub(crate) rHessian: Option<Vec<Vec<f64>>>,
    // Java field: double[][] thetaHessian
    pub(crate) thetaHessian: Option<Vec<Vec<f64>>>,
    // Java field: double[][] phiHessian
    pub(crate) phiHessian: Option<Vec<Vec<f64>>>,
}

// Java: public class DataTransferObject
pub struct DataTransferObject {
    // Java field: double x
    pub(crate) x: f64,
    // Java field: double y
    pub(crate) y: f64,
    // Java field: double z
    pub(crate) z: f64,
}

// Java: public class provides
impl provides {
}

// Java: public class SphericalCoordinates
impl SphericalCoordinates {
    // Java: SphericalCoordinates(final Vector3D v)
    pub fn new_from_vector(v: Vector3D) -> Self {
        // Java: // Cartesian coordinates
        // Java: this.v = v;
        // Java: // remaining spherical coordinates
        // Java: this.r     = v.getNorm();
        let r = v.getNorm();
        // Java: this.theta = v.getAlpha();
        let theta = v.getAlpha();
        // Java: this.phi   = FastMath.acos(v.getZ() / r);
        let phi = FastMath::acos(v.getZ() / r);
        SphericalCoordinates {
            v,
            r,
            theta,
            phi,
            jacobian: None,
            rHessian: None,
            thetaHessian: None,
            phiHessian: None,
        }
    }

    // Java: SphericalCoordinates(final double r, final double theta, final double phi)
    pub fn new(r: f64, theta: f64, phi: f64) -> Self {
        // Java: final double cosTheta = FastMath.cos(theta);
        let cos_theta = FastMath::cos(theta);
        // Java: final double sinTheta = FastMath.sin(theta);
        let sin_theta = FastMath::sin(theta);
        // Java: final double cosPhi   = FastMath.cos(phi);
        let cos_phi = FastMath::cos(phi);
        // Java: final double sinPhi   = FastMath.sin(phi);
        let sin_phi = FastMath::sin(phi);
        // Java: // spherical coordinates
        // Java: this.r     = r;
        // Java: this.theta = theta;
        // Java: this.phi   = phi;
        // Java: // Cartesian coordinates
        // Java: this.v  = new Vector3D(r * cosTheta * sinPhi,
        // Java: r * sinTheta * sinPhi,
        // Java: r * cosPhi);
        let v = Vector3D::new(
            r * cos_theta * sin_phi,
            r * sin_theta * sin_phi,
            r * cos_phi,
        );
        SphericalCoordinates {
            v,
            r,
            theta,
            phi,
            jacobian: None,
            rHessian: None,
            thetaHessian: None,
            phiHessian: None,
        }
    }

    // Java: Vector3D getCartesian()
    pub fn getCartesian(&self) -> &Vector3D {
        // Java: return v;
        &self.v
    }

    // Java: double getR()
    pub fn getR(&self) -> f64 {
        // Java: return r;
        self.r
    }

    // Java: double getTheta()
    pub fn getTheta(&self) -> f64 {
        // Java: return theta;
        self.theta
    }

    // Java: double getPhi()
    pub fn getPhi(&self) -> f64 {
        // Java: return phi;
        self.phi
    }

    // Java: double[] toCartesianGradient(final double[] sGradient)
    pub fn toCartesianGradient(&mut self, s_gradient: &[f64]) -> Vec<f64> {
        // Java: // lazy evaluation of Jacobian
        // Java: computeJacobian();
        self.computeJacobian();
        // Java: // compose derivatives as gradient^T . J
        // Java: // the expressions have been simplified since we know jacobian[1][2] = dTheta/dZ = 0
        // Java: return new double[]
        let j = self.jacobian.as_ref().unwrap();
        vec![
            // Java: sGradient[0] * jacobian[0][0] + sGradient[1] * jacobian[1][0] + sGradient[2] * jacobian[2][0],
            s_gradient[0] * j[0][0] + s_gradient[1] * j[1][0] + s_gradient[2] * j[2][0],
            // Java: sGradient[0] * jacobian[0][1] + sGradient[1] * jacobian[1][1] + sGradient[2] * jacobian[2][1],
            s_gradient[0] * j[0][1] + s_gradient[1] * j[1][1] + s_gradient[2] * j[2][1],
            // Java: sGradient[0] * jacobian[0][2]                                 + sGradient[2] * jacobian[2][2]
            s_gradient[0] * j[0][2]                                 + s_gradient[2] * j[2][2],
        ]
    }

    // Java: double[][] toCartesianHessian(final double[][] sHessian, final double[] sGradient)
    pub fn toCartesianHessian(&mut self, _s_hessian: &Vec<Vec<f64>>, _s_gradient: &[f64]) -> Vec<Vec<f64>> {
        // Java: computeJacobian();
        // Java: computeHessians();
        // Java: // compose derivative as J^T . H_f . J + df/dr H_r + df/dtheta H_theta + df/dphi H_phi
        // Java: // the expressions have been simplified since we know jacobian[1][2] = dTheta/dZ = 0
        // Java: // and H_theta is only a 2x2 matrix as it does not depend on z
        // Java: final double[][] hj = new double[3][3];
        // Java: final double[][] cHessian = new double[3][3];
        // Java: // compute H_f . J
        // Java: // beware we use ONLY the lower-left part of sHessian
        // Java: hj[0][0] = sHessian[0][0] * jacobian[0][0] + sHessian[1][0] * jacobian[1][0] + sHessian[2][0] * jacobian[2][0];
        // Java: hj[0][1] = sHessian[0][0] * jacobian[0][1] + sHessian[1][0] * jacobian[1][1] + sHessian[2][0] * jacobian[2][1];
        // Java: hj[0][2] = sHessian[0][0] * jacobian[0][2]                                   + sHessian[2][0] * jacobian[2][2];
        // Java: hj[1][0] = sHessian[1][0] * jacobian[0][0] + sHessian[1][1] * jacobian[1][0] + sHessian[2][1] * jacobian[2][0];
        // Java: hj[1][1] = sHessian[1][0] * jacobian[0][1] + sHessian[1][1] * jacobian[1][1] + sHessian[2][1] * jacobian[2][1];
        // Java: // don't compute hj[1][2] as it is not used below
        // Java: hj[2][0] = sHessian[2][0] * jacobian[0][0] + sHessian[2][1] * jacobian[1][0] + sHessian[2][2] * jacobian[2][0];
        // Java: hj[2][1] = sHessian[2][0] * jacobian[0][1] + sHessian[2][1] * jacobian[1][1] + sHessian[2][2] * jacobian[2][1];
        // Java: hj[2][2] = sHessian[2][0] * jacobian[0][2]                                   + sHessian[2][2] * jacobian[2][2];
        // Java: // compute lower-left part of J^T . H_f . J
        // Java: cHessian[0][0] = jacobian[0][0] * hj[0][0] + jacobian[1][0] * hj[1][0] + jacobian[2][0] * hj[2][0];
        // Java: cHessian[1][0] = jacobian[0][1] * hj[0][0] + jacobian[1][1] * hj[1][0] + jacobian[2][1] * hj[2][0];
        // Java: cHessian[2][0] = jacobian[0][2] * hj[0][0]                             + jacobian[2][2] * hj[2][0];
        // Java: cHessian[1][1] = jacobian[0][1] * hj[0][1] + jacobian[1][1] * hj[1][1] + jacobian[2][1] * hj[2][1];
        // Java: cHessian[2][1] = jacobian[0][2] * hj[0][1]                             + jacobian[2][2] * hj[2][1];
        // Java: cHessian[2][2] = jacobian[0][2] * hj[0][2]                             + jacobian[2][2] * hj[2][2];
        // Java: // add gradient contribution
        // Java: cHessian[0][0] += sGradient[0] * rHessian[0][0] + sGradient[1] * thetaHessian[0][0] + sGradient[2] * phiHessian[0][0];
        // Java: cHessian[1][0] += sGradient[0] * rHessian[1][0] + sGradient[1] * thetaHessian[1][0] + sGradient[2] * phiHessian[1][0];
        // Java: cHessian[2][0] += sGradient[0] * rHessian[2][0]                                     + sGradient[2] * phiHessian[2][0];
        // Java: cHessian[1][1] += sGradient[0] * rHessian[1][1] + sGradient[1] * thetaHessian[1][1] + sGradient[2] * phiHessian[1][1];
        // Java: cHessian[2][1] += sGradient[0] * rHessian[2][1]                                     + sGradient[2] * phiHessian[2][1];
        // Java: cHessian[2][2] += sGradient[0] * rHessian[2][2]                                     + sGradient[2] * phiHessian[2][2];
        // Java: // ensure symmetry
        // Java: cHessian[0][1] = cHessian[1][0];
        // Java: cHessian[0][2] = cHessian[2][0];
        // Java: cHessian[1][2] = cHessian[2][1];
        // Java: return cHessian;
        todo!()
    }

    // Java: void computeJacobian()
    pub fn computeJacobian(&mut self) {
        // Java: if (jacobian == null)
        if self.jacobian.is_none() {
            // Java: // intermediate variables
            // Java: final double x    = v.getX();
            let x = self.v.getX();
            // Java: final double y    = v.getY();
            let y = self.v.getY();
            // Java: final double z    = v.getZ();
            let z = self.v.getZ();
            // Java: final double rho2 = x * x + y * y;
            let rho2 = x * x + y * y;
            // Java: final double rho  = FastMath.sqrt(rho2);
            let _rho = FastMath::sqrt(rho2);
            // Java: final double r2   = rho2 + z * z;
            let r2 = rho2 + z * z;
            // Java: jacobian = new double[3][3];
            let mut j = vec![vec![0.0_f64; 3]; 3];
            // Java: // row representing the gradient of r
            // Java: jacobian[0][0] = x / r;
            j[0][0] = x / self.r;
            // Java: jacobian[0][1] = y / r;
            j[0][1] = y / self.r;
            // Java: jacobian[0][2] = z / r;
            j[0][2] = z / self.r;
            // Java: // row representing the gradient of theta
            // Java: jacobian[1][0] = -y / rho2;
            j[1][0] = -y / rho2;
            // Java: jacobian[1][1] =  x / rho2;
            j[1][1] = x / rho2;
            // Java: // jacobian[1][2] is already set to 0 at allocation time
            // Java: // row representing the gradient of phi
            // Java: jacobian[2][0] = x * z / (rho * r2);
            j[2][0] = x * z / (_rho * r2);
            // Java: jacobian[2][1] = y * z / (rho * r2);
            j[2][1] = y * z / (_rho * r2);
            // Java: jacobian[2][2] = -rho / r2;
            j[2][2] = -_rho / r2;
            self.jacobian = Some(j);
        }
    }

    // Java: void computeHessians()
    pub fn computeHessians(&mut self) {
        // Java: if (rHessian == null)
        if self.rHessian.is_none() {
            // Java: // intermediate variables
            // Java: final double x      = v.getX();
            let x = self.v.getX();
            // Java: final double y      = v.getY();
            let y = self.v.getY();
            // Java: final double z      = v.getZ();
            let z = self.v.getZ();
            // Java: final double x2     = x * x;
            let x2 = x * x;
            // Java: final double y2     = y * y;
            let y2 = y * y;
            // Java: final double z2     = z * z;
            let z2 = z * z;
            // Java: final double rho2   = x2 + y2;
            let rho2 = x2 + y2;
            // Java: final double rho    = FastMath.sqrt(rho2);
            let rho = FastMath::sqrt(rho2);
            // Java: final double r2     = rho2 + z2;
            let r2 = rho2 + z2;
            // Java: final double xOr    = x / r;
            let x_or = x / self.r;
            // Java: final double yOr    = y / r;
            let y_or = y / self.r;
            // Java: final double zOr    = z / r;
            let z_or = z / self.r;
            // Java: final double xOrho2 = x / rho2;
            let x_orho2 = x / rho2;
            // Java: final double yOrho2 = y / rho2;
            let y_orho2 = y / rho2;
            // Java: final double xOr3   = xOr / r2;
            let x_or3 = x_or / r2;
            // Java: final double yOr3   = yOr / r2;
            let y_or3 = y_or / r2;
            // Java: final double zOr3   = zOr / r2;
            let z_or3 = z_or / r2;

            // Java: // lower-left part of Hessian of r
            // Java: rHessian = new double[3][3];
            let mut rh = vec![vec![0.0_f64; 3]; 3];
            // Java: rHessian[0][0] = y * yOr3 + z * zOr3;
            rh[0][0] = y * y_or3 + z * z_or3;
            // Java: rHessian[1][0] = -x * yOr3;
            rh[1][0] = -x * y_or3;
            // Java: rHessian[2][0] = -z * xOr3;
            rh[2][0] = -z * x_or3;
            // Java: rHessian[1][1] = x * xOr3 + z * zOr3;
            rh[1][1] = x * x_or3 + z * z_or3;
            // Java: rHessian[2][1] = -y * zOr3;
            rh[2][1] = -y * z_or3;
            // Java: rHessian[2][2] = x * xOr3 + y * yOr3;
            rh[2][2] = x * x_or3 + y * y_or3;
            // Java: // upper-right part is symmetric
            // Java: rHessian[0][1] = rHessian[1][0];
            rh[0][1] = rh[1][0];
            // Java: rHessian[0][2] = rHessian[2][0];
            rh[0][2] = rh[2][0];
            // Java: rHessian[1][2] = rHessian[2][1];
            rh[1][2] = rh[2][1];
            self.rHessian = Some(rh);

            // Java: // lower-left part of Hessian of azimuthal angle theta
            // Java: thetaHessian = new double[2][2];
            let mut th = vec![vec![0.0_f64; 2]; 2];
            // Java: thetaHessian[0][0] = 2 * xOrho2 * yOrho2;
            th[0][0] = 2.0 * x_orho2 * y_orho2;
            // Java: thetaHessian[1][0] = yOrho2 * yOrho2 - xOrho2 * xOrho2;
            th[1][0] = y_orho2 * y_orho2 - x_orho2 * x_orho2;
            // Java: thetaHessian[1][1] = -2 * xOrho2 * yOrho2;
            th[1][1] = -2.0 * x_orho2 * y_orho2;
            // Java: // upper-right part is symmetric
            // Java: thetaHessian[0][1] = thetaHessian[1][0];
            th[0][1] = th[1][0];
            self.thetaHessian = Some(th);

            // Java: // lower-left part of Hessian of polar (co-latitude) angle phi
            // Java: final double rhor2       = rho * r2;
            let rhor2 = rho * r2;
            // Java: final double rho2r2      = rho * rhor2;
            let rho2r2 = rho * rhor2;
            // Java: final double rhor4       = rhor2 * r2;
            let rhor4 = rhor2 * r2;
            // Java: final double rho3r4      = rhor4 * rho2;
            let rho3r4 = rhor4 * rho2;
            // Java: final double r2P2rho2    = 3 * rho2 + z2;
            let r2_p2rho2 = 3.0 * rho2 + z2;
            // Java: phiHessian = new double[3][3];
            let mut ph = vec![vec![0.0_f64; 3]; 3];
            // Java: phiHessian[0][0] = z * (rho2r2 - x2 * r2P2rho2) / rho3r4;
            ph[0][0] = z * (rho2r2 - x2 * r2_p2rho2) / rho3r4;
            // Java: phiHessian[1][0] = -x * y * z * r2P2rho2 / rho3r4;
            ph[1][0] = -x * y * z * r2_p2rho2 / rho3r4;
            // Java: phiHessian[2][0] = x * (rho2 - z2) / rhor4;
            ph[2][0] = x * (rho2 - z2) / rhor4;
            // Java: phiHessian[1][1] = z * (rho2r2 - y2 * r2P2rho2) / rho3r4;
            ph[1][1] = z * (rho2r2 - y2 * r2_p2rho2) / rho3r4;
            // Java: phiHessian[2][1] = y * (rho2 - z2) / rhor4;
            ph[2][1] = y * (rho2 - z2) / rhor4;
            // Java: phiHessian[2][2] = 2 * rho * zOr3 / r;
            ph[2][2] = 2.0 * rho * z_or3 / self.r;
            // Java: // upper-right part is symmetric
            // Java: phiHessian[0][1] = phiHessian[1][0];
            ph[0][1] = ph[1][0];
            // Java: phiHessian[0][2] = phiHessian[2][0];
            ph[0][2] = ph[2][0];
            // Java: phiHessian[1][2] = phiHessian[2][1];
            ph[1][2] = ph[2][1];
            self.phiHessian = Some(ph);
        }
    }

    // Java: Object writeReplace()
    pub fn writeReplace(&self) -> DataTransferObject {
        // Java: return new DataTransferObject(v.getX(), v.getY(), v.getZ());
        DataTransferObject { x: self.v.getX(), y: self.v.getY(), z: self.v.getZ() }
    }

}

// Java: public class DataTransferObject
impl DataTransferObject {
    // Java: Object readResolve()
    pub fn readResolve(&self) -> SphericalCoordinates {
        // Java: return new SphericalCoordinates(new Vector3D(x, y, z));
        SphericalCoordinates::new_from_vector(Vector3D::new(self.x, self.y, self.z))
    }

}
