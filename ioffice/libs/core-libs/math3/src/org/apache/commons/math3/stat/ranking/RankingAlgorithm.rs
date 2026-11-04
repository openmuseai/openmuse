// Java: public interface RankingAlgorithm
pub trait RankingAlgorithm {
    // Java: abstract double[] rank(double[] data)
    fn rank(&self) -> Vec<f64>;

}
