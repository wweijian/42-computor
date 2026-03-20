use std::process::Command;

fn run(input: &str) -> std::process::Output {
    Command::new("cargo")
        .args(["run", "--", input])
        .output()
        .expect("failed to run")
}

fn ok(input: &str) {
    let output = run(input);
    assert!(
        output.status.success(),
        "expected success for {:?}, got stderr: {}",
        input,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn fail(input: &str) {
    let output = run(input);
    assert!(
        !output.status.success(),
        "expected failure for {:?}, but it succeeded\n{}",
        input,
        String::from_utf8_lossy(&output.stderr)
    );
}

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
fn test_high_degree() {
    // parser accepts any degree; solver may reject but parse should succeed
    ok("x^3 + x^2 + x + 1");
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

// ── failures ─────────────────────────────────────────────────────────────────

#[test]
fn test_empty_input() {
    fail("");
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

#[test]
fn test_leading_multiply() {
    fail("* x^2");
}

#[test]
fn test_leading_divide() {
    fail("/ x^2");
}

#[test]
fn test_whitespace_only() {
    fail("   ");
}

/// Leading '=' is treated as an empty LHS; parses the RHS as a valid polynomial.
/// This documents the current (accepted) behaviour rather than asserting it should fail.
#[test]
fn test_leading_equals() {
    ok("= x + 1");
}
