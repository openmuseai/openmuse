#![allow(non_snake_case, dead_code, unused_variables)]

use super::Assert::Assert;

// Java: public class ComparisonFailure extends AssertionFailedError
pub struct ComparisonFailure {
    message: String,
    // Java field: String fActual
    fActual: Option<String>,
    // Java field: String fExpected
    fExpected: Option<String>,
}

// Java: public class ComparisonFailure extends AssertionFailedError
impl ComparisonFailure {
    // Java: ComparisonFailure(String message, String expected, String actual)
    pub fn new(message: &str, expected: &str, actual: &str) -> Self {
        // Java: super(message);
        // Java: this.fExpected = expected;
        // Java: this.fActual = actual;
        Self {
            message: message.to_string(),
            fExpected: Some(expected.to_string()),
            fActual: Some(actual.to_string()),
        }
    }

    // Java: String getMessage()
    pub fn getMessage(&self) -> String {
        // Java: if (this.fExpected != null && this.fActual != null)
        if self.fExpected.is_some() && self.fActual.is_some()
        {
            let f_expected = self.fExpected.as_ref().unwrap();
            let f_actual = self.fActual.as_ref().unwrap();
            let f_expected_chars: Vec<char> = f_expected.chars().collect();
            let f_actual_chars: Vec<char> = f_actual.chars().collect();
            // Java: int end = Math.min(this.fExpected.length(), this.fActual.length());
            let end = std::cmp::min(f_expected_chars.len(), f_actual_chars.len());
            // Java: int i;
            let mut i: usize = 0;
            // Java: for(i = 0; i < end && this.fExpected.charAt(i) == this.fActual.charAt(i); ++i)
            while i < end && f_expected_chars[i] == f_actual_chars[i]
            {
                i += 1;
            }
            // Java: int j = this.fExpected.length() - 1;
            let mut j: isize = f_expected_chars.len() as isize - 1;
            // Java: int k;
            // Java: for(k = this.fActual.length() - 1; k >= i && j >= i && this.fExpected.charAt(j) == this.fActual.charAt(k); --j)
            let mut k: isize = f_actual_chars.len() as isize - 1;
            while k >= i as isize && j >= i as isize && f_expected_chars[j as usize] == f_actual_chars[k as usize]
            {
                // Java: --k;
                k -= 1;
                j -= 1;
            }
            // Java: String actual;
            let actual: String;
            // Java: String expected;
            let expected: String;
            // Java: if (j < i && k < i)
            if (j < i as isize) && (k < i as isize)
            {
                // Java: expected = this.fExpected;
                expected = f_expected.clone();
                // Java: actual = this.fActual;
                actual = f_actual.clone();
            }
            // Java: else
            else
            {
                // Java: expected = this.fExpected.substring(i, j + 1);
                expected = f_expected_chars[i..(j as usize + 1)].iter().collect();
                // Java: actual = this.fActual.substring(i, k + 1);
                actual = f_actual_chars[i..(k as usize + 1)].iter().collect();
                let mut expected = expected;
                let mut actual = actual;
                // Java: if (i <= end && i > 0)
                if i <= end && i > 0
                {
                    // Java: expected = "..." + expected;
                    expected = format!("...{}", expected);
                    // Java: actual = "..." + actual;
                    actual = format!("...{}", actual);
                }
                // Java: if (j < this.fExpected.length() - 1)
                if j < f_expected_chars.len() as isize - 1
                {
                    // Java: expected = expected + "...";
                    expected = format!("{}...", expected);
                }
                // Java: if (k < this.fActual.length() - 1)
                if k < f_actual_chars.len() as isize - 1
                {
                    // Java: actual = actual + "...";
                    actual = format!("{}...", actual);
                }
                // Java: return Assert.format(super.getMessage(), expected, actual);
                return Assert::format(Some(&self.message), &expected, &actual);
            }
            // Java: return Assert.format(super.getMessage(), expected, actual);
            return Assert::format(Some(&self.message), &expected, &actual);
        }
        // Java: else
        else
        {
            // Java: return Assert.format(super.getMessage(), this.fExpected, this.fActual);
            return Assert::format(
                Some(&self.message),
                self.fExpected.as_deref().unwrap_or("null"),
                self.fActual.as_deref().unwrap_or("null"),
            );
        }
    }
}
