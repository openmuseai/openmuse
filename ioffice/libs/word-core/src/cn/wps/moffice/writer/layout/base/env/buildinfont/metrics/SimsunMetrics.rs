// Java: public class SimsunMetrics extends DefaultMetricsFactory.DefaultMetricsGetter
// Java file: writer/core/.../buildinfont/metrics/SimsunMetrics.java
//
// DefaultMetricsFactory.initRegister in this tree does not register 宋体.
// The product still needs getWidthForSpecial: AndroidTypeface.getWriterLayoutAdavancesEM
// reaches Paint only after FreeType returns -1, and the PKM110 capture of U+000D
// at srcEmSize 100 is raw 25, the same value this method returns for '\r'.

pub struct SimsunMetrics;

impl SimsunMetrics {
    // Java: protected int getWidthForSpecial(char c)
    pub fn getWidthForSpecial(c: u16) -> i32 {
        // Java: switch (c)
        match c {
            // Java: case '\u0007': return 80;
            0x0007 => 80,
            // Java: case '\t': return 64;
            0x0009 => 64,
            // Java: case '\r': return 25;
            0x000D => 25,
            // Java: default: return super.getWidthForSpecial(c);
            _ => -1,
        }
    }
}
