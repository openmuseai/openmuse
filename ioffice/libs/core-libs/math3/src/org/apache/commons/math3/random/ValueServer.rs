// Java: import java.io.BufferedReader
// Java: import java.io.IOException
// Java: import java.io.InputStreamReader
// Java: import java.net.MalformedURLException
// Java: import java.net.URL
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: int DIGEST_MODE
pub(crate) const DIGEST_MODE: i32 = 0;
// Java field: int REPLAY_MODE
pub(crate) const REPLAY_MODE: i32 = 0;
// Java field: int UNIFORM_MODE
pub(crate) const UNIFORM_MODE: i32 = 0;
// Java field: int EXPONENTIAL_MODE
pub(crate) const EXPONENTIAL_MODE: i32 = 0;
// Java field: int GAUSSIAN_MODE
pub(crate) const GAUSSIAN_MODE: i32 = 0;
// Java field: int CONSTANT_MODE
pub(crate) const CONSTANT_MODE: i32 = 0;

// Java: public class ValueServer
pub struct ValueServer {
    // Java field: int mode
    pub(crate) mode: i32,
    // Java field: URL valuesFileURL
    pub(crate) valuesFileURL: Box<dyn std::any::Any>,
    // Java field: double mu
    pub(crate) mu: f64,
    // Java field: double sigma
    pub(crate) sigma: f64,
    // Java field: EmpiricalDistribution empiricalDistribution
    pub(crate) empiricalDistribution: Box<dyn std::any::Any>,
    // Java field: BufferedReader filePointer
    filePointer: Option<Box<dyn std::any::Any>>,
    // Java field: RandomDataGenerator randomData
    randomData: Box<dyn std::any::Any>,
}

// Java: public class ValueServer
impl ValueServer {
    // Java: ValueServer()
    pub fn ValueServer(&self) {
        // Java: randomData = new RandomDataGenerator();
    }

    // Java: ValueServer(RandomDataImpl randomData)
    pub fn ValueServer_2(&self) {
        // Java: this.randomData = randomData.getDelegate();
    }

    // Java: ValueServer(RandomGenerator generator)
    pub fn ValueServer_3(&self) {
        // Java: this.randomData = new RandomDataGenerator(generator);
    }

    // Java: double getNext()
    pub fn getNext(&self) -> f64 {
        // Java: switch (mode)
        {
            // Java: case DIGEST_MODE: return getNextDigest();
            // Java: case REPLAY_MODE: return getNextReplay();
            // Java: case UNIFORM_MODE: return getNextUniform();
            // Java: case EXPONENTIAL_MODE: return getNextExponential();
            // Java: case GAUSSIAN_MODE: return getNextGaussian();
            // Java: case CONSTANT_MODE: return mu;
            // Java: default: throw new MathIllegalStateException(
                    // Java: LocalizedFormats.UNKNOWN_MODE,
                    // Java: mode,
                    // Java: "DIGEST_MODE",   DIGEST_MODE,   "REPLAY_MODE",      REPLAY_MODE,
                    // Java: "UNIFORM_MODE",  UNIFORM_MODE,  "EXPONENTIAL_MODE", EXPONENTIAL_MODE,
                    // Java: "GAUSSIAN_MODE", GAUSSIAN_MODE, "CONSTANT_MODE",    CONSTANT_MODE);
        }
        todo!()
    }

    // Java: void fill(double[] values)
    pub fn fill(&self) {
    }

    // Java: double[] fill(int length)
    pub fn fill_2(&self) -> Vec<f64> {
        todo!()
    }

    // Java: void computeDistribution()
    pub fn computeDistribution(&self) {
        // Java: computeDistribution(EmpiricalDistribution.DEFAULT_BIN_COUNT);
    }

    // Java: void computeDistribution(int binCount)
    pub fn computeDistribution_2(&self) {
        // Java: empiricalDistribution = new EmpiricalDistribution(binCount, randomData.getRandomGenerator());
        // Java: empiricalDistribution.load(valuesFileURL);
        // Java: mu = empiricalDistribution.getSampleStats().getMean();
        // Java: sigma = empiricalDistribution.getSampleStats().getStandardDeviation();
    }

    // Java: int getMode()
    pub fn getMode(&self) -> i32 {
        // Java: return mode;
        todo!()
    }

    // Java: void setMode(int mode)
    pub fn setMode(&self) {
        // Java: this.mode = mode;
    }

    // Java: URL getValuesFileURL()
    pub fn getValuesFileURL(&self) -> Box<dyn std::any::Any> {
        // Java: return valuesFileURL;
        todo!()
    }

    // Java: void setValuesFileURL(String url)
    pub fn setValuesFileURL(&self) {
        // Java: this.valuesFileURL = new URL(url);
    }

    // Java: void setValuesFileURL(URL url)
    pub fn setValuesFileURL_2(&self) {
        // Java: this.valuesFileURL = url;
    }

    // Java: EmpiricalDistribution getEmpiricalDistribution()
    pub fn getEmpiricalDistribution(&self) -> Box<dyn std::any::Any> {
        // Java: return empiricalDistribution;
        todo!()
    }

    // Java: void resetReplayFile()
    pub fn resetReplayFile(&self) {
        // Java: if (filePointer != null)
        {
            // Java: try
            {
                // Java: filePointer.close();
                // Java: filePointer = null;
            }
                // Java: // ignore
        }
        // Java: filePointer = new BufferedReader(new InputStreamReader(valuesFileURL.openStream(), "UTF-8"));
    }

    // Java: void closeReplayFile()
    pub fn closeReplayFile(&self) {
        // Java: if (filePointer != null)
        {
            // Java: filePointer.close();
            // Java: filePointer = null;
        }
    }

    // Java: double getMu()
    pub fn getMu(&self) -> f64 {
        // Java: return mu;
        todo!()
    }

    // Java: void setMu(double mu)
    pub fn setMu(&self) {
        // Java: this.mu = mu;
    }

    // Java: double getSigma()
    pub fn getSigma(&self) -> f64 {
        // Java: return sigma;
        todo!()
    }

    // Java: void setSigma(double sigma)
    pub fn setSigma(&self) {
        // Java: this.sigma = sigma;
    }

    // Java: void reSeed(long seed)
    pub fn reSeed(&self) {
        // Java: randomData.reSeed(seed);
    }

    // Java: double getNextDigest()
    pub fn getNextDigest(&self) -> f64 {
        // Java: if ((empiricalDistribution == null) ||
            // Java: (empiricalDistribution.getBinStats().size() == 0))
            {
            // Java: throw new MathIllegalStateException(LocalizedFormats.DIGEST_NOT_INITIALIZED);
            }
        // Java: return empiricalDistribution.getNextValue();
        todo!()
    }

    // Java: double getNextReplay()
    pub fn getNextReplay(&self) -> f64 {
        // Java: String str = null;
        // Java: if (filePointer == null)
        {
            // Java: resetReplayFile();
        }
        // Java: if ((str = filePointer.readLine()) == null)
        {
            // Java: // we have probably reached end of file, wrap around from EOF to BOF
            // Java: closeReplayFile();
            // Java: resetReplayFile();
            // Java: if ((str = filePointer.readLine()) == null)
            {
                // Java: throw new MathIllegalStateException(LocalizedFormats.URL_CONTAINS_NO_DATA,
                                                    // Java: valuesFileURL);
            }
        }
        // Java: return Double.parseDouble(str);
        todo!()
    }

    // Java: double getNextUniform()
    pub fn getNextUniform(&self) -> f64 {
        // Java: return randomData.nextUniform(0, 2 * mu);
        todo!()
    }

    // Java: double getNextExponential()
    pub fn getNextExponential(&self) -> f64 {
        // Java: return randomData.nextExponential(mu);
        todo!()
    }

    // Java: double getNextGaussian()
    pub fn getNextGaussian(&self) -> f64 {
        // Java: return randomData.nextGaussian(mu, sigma);
        todo!()
    }

}
