mod run;

use run::*;

// ── basic ────────────────────────────────────────────────────────────────────

#[test]
fn test_quadratic() {
    ok("x^2 + x + 1");
}

#[test]
fn test_uppercase_variable() {
    ok("X^2 + X + 1");
}

#[test]
fn test_mixed_case_variable() {
    ok("X^2 + x + 1");
}

// ── whitespace ───────────────────────────────────────────────────────────────

#[test]
fn test_spaces_around_caret() {
    ok("x ^ 2 + x + 1");
}

#[test]
fn test_spaces_everywhere() {
    ok("  x ^ 2  +  x  +  1  ");
}

#[test]
fn test_spaces_after_sign() {
    ok("- x ^ 2 + x - 1");
}

#[test]
fn test_spaces_around_multiply() {
    ok("2 * x ^ 2 + 3 * x + 4");
}

/// Spaces inside a number are ignored, so "1 2" is treated as 12.
#[test]
fn test_spaces_inside_coefficient() {
    ok("1 2 x ^ 2 + 3 x + 5");
}

/// Spaces inside the degree value are also ignored.
#[test]
fn test_spaces_inside_degree() {
    ok("x ^ 1 2");   // degree 12 – parses fine even if solver rejects it
}

// ── degrees 0–2+ ─────────────────────────────────────────────────────────────

#[test]
fn test_constant_only() {
    ok("5");
}

#[test]
fn test_linear_only() {
    ok("x");
}

#[test]
fn test_explicit_degree_zero() {
    ok("x^0");
}

#[test]
fn test_explicit_degree_one() {
    ok("x^1");
}

#[test]
fn test_explicit_degree_two() {
    ok("x^2");
}

#[test]
fn test_all_degrees_explicit() {
    ok("x^2 + x^1 + x^0");
}

#[test]
fn test_negative_all_terms() {
    ok("-x^2 - x - 1");
}

#[test]
fn test_repeated_same_degree() {
    // x^2 appears twice; both coefficients should be accumulated
    ok("x^2 + x^2");
}

#[test]
fn test_subtract_like_terms() {
    ok("3*x^2 - x^2 + x");
}

#[test]
fn test_fractional_coefficient() {
    ok("0.5 * x^2 + 1.5 * x + 2.0");
}

#[test]
fn test_division_coefficient() {
    ok("x^2 / 2 + x / 4");
}

#[test]
fn test_explicit_positive_sign() {
    ok("+x^2 + x + 1");
}

// ── equal sign (optional, at most one) ───────────────────────────────────────

#[test]
fn test_no_equals() {
    ok("x^2 + x + 1");
}

#[test]
fn test_equals_zero_rhs() {
    ok("x^2 + x + 1 = 0");
}

#[test]
fn test_equals_with_terms_both_sides() {
    ok("x^2 + x = 1");
}

#[test]
fn test_equals_both_sides_polynomial() {
    ok("x^2 + x + 1 = x + 2");
}

/// Leading '=' is treated as an empty LHS; parses the RHS as a valid polynomial.
/// This documents the current (accepted) behaviour rather than asserting it should fail.
#[test]
fn test_leading_equals() {
    ok("= x + 1");
}
