mod run;

use run::*;

#[test]
fn test_empty_input() {
    fail("");
}

#[test]
fn test_whitespace_only() {
    fail("   ");
}

#[test]
fn test_invalid_character() {
    fail("@x^2 + x + 1");
}

#[test]
fn test_double_operator() {
    fail("x^2 + + x");
}

#[test]
fn test_hanging_plus() {
    fail("x^2 +");
}

#[test]
fn test_hanging_minus() {
    fail("x^2 -");
}

#[test]
fn test_leading_multiply() {
    fail("* x^2");
}

#[test]
fn test_leading_divide() {
    fail("/ x^2");
}

#[test]
fn test_division_by_zero() {
    fail("x / 0");
}

#[test]
fn test_double_decimal_point() {
    fail("1.2.3 * x");
}

#[test]
fn test_bare_caret() {
    fail("x ^ + 1");
}

#[test]
fn test_trailing_caret() {
    fail("x^");
}

#[test]
fn test_negative_exponent() {
    fail("x^-2");
}

#[test]
fn test_multiple_equals() {
    fail("x = 1 = 2");
}

// ── degree > 2 ────────────────────────────────────────────────────────────────

#[test]
fn test_degree_three() {
    fail("x^3 + x^2 + x + 1");
}

#[test]
fn test_degree_four() {
    fail("x^4 - 1");
}
