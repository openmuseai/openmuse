#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    Unnecessary,
    Down,
    Up,
    Ceiling,
    Floor,
    HalfEven,
    HalfDown,
    HalfUp,
}

// Java: public class IntMath
pub struct IntMath;

// Java: public class IntMath
impl IntMath {
    // Java: IntMath()
    // Java: super();

    // Java: static int divide(int p, int q, RoundingMode mode)
    pub fn divide(p: i32, q: i32, mode: RoundingMode) -> i32 {
        // Java: Preconditions.checkNotNull(mode);
        // Java: if (q == 0)
        if q == 0 {
            // Java: throw new ArithmeticException("/ by zero");
            panic!("/ by zero");
        }
        // Java: else
        {
            // Java: int div = p / q;
            let div = p / q;
            // Java: int rem = p - q * div;
            let rem = p - q * div;
            // Java: if (rem == 0)
            if rem == 0 {
                // Java: return div;
                return div;
            }
            // Java: else
            {
                // Java: int signum = 1 | (p ^ q) >> 31;
                let signum = 1i32 | ((p ^ q) >> 31);
                // Java: boolean increment;
                let increment: bool;
                // Java: switch (mode)
                match mode {
                    // Java: case UNNECESSARY:
                    RoundingMode::Unnecessary => {
                        // Java: Assert.assertTrue(rem == 0);
                        assert!(rem == 0);
                        // Java: case DOWN:
                        // Java: increment = false;
                        increment = false;
                        // Java: break;
                    }
                    // Java: case DOWN:
                    RoundingMode::Down => {
                        // Java: increment = false;
                        increment = false;
                        // Java: break;
                    }
                    // Java: case UP:
                    RoundingMode::Up => {
                        // Java: increment = true;
                        increment = true;
                        // Java: break;
                    }
                    // Java: case CEILING:
                    RoundingMode::Ceiling => {
                        // Java: increment = signum > 0;
                        increment = signum > 0;
                        // Java: break;
                    }
                    // Java: case FLOOR:
                    RoundingMode::Floor => {
                        // Java: increment = signum < 0;
                        increment = signum < 0;
                        // Java: break;
                    }
                    // Java: case HALF_EVEN:
                    // Java: case HALF_DOWN:
                    // Java: case HALF_UP:
                    RoundingMode::HalfEven | RoundingMode::HalfDown | RoundingMode::HalfUp => {
                        // Java: int absRem = Math.abs(rem);
                        let abs_rem = rem.abs();
                        // Java: int cmpRemToHalfDivisor = absRem - (Math.abs(q) - absRem);
                        let cmp_rem_to_half_divisor = abs_rem - (q.abs() - abs_rem);
                        // Java: if (cmpRemToHalfDivisor == 0)
                        if cmp_rem_to_half_divisor == 0 {
                            // Java: increment = mode == RoundingMode.HALF_UP || mode == RoundingMode.HALF_EVEN & (div & 1) != 0;
                            increment = mode == RoundingMode::HalfUp || (mode == RoundingMode::HalfEven && (div & 1) != 0);
                        }
                        // Java: else
                        else {
                            // Java: increment = cmpRemToHalfDivisor > 0;
                            increment = cmp_rem_to_half_divisor > 0;
                        }
                        // Java: break;
                    }
                    // Java: default:
                    #[allow(unreachable_patterns)]
                    _ => {
                        // Java: throw new AssertionError();
                        panic!("AssertionError");
                    }
                }
                // Java: return increment ? div + signum : div;
                if increment { div + signum } else { div }
            }
        }
    }
}
