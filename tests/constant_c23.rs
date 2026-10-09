use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use cbindgen::{Builder, Config, Language, SortKey};

fn generate(config: &str, language: Language) -> String {
    let mut config: Config = toml::from_str(config).unwrap();
    config.language = language;
    config.sort_by = SortKey::Name;
    let bindings = Builder::new()
        .with_src(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/constant_c23/input.rs"))
        .with_config(config)
        .generate()
        .unwrap();
    let mut output = Vec::new();
    bindings.write(&mut output);
    String::from_utf8(output).unwrap()
}

const ENABLED: &str = "[const]\nallow_constexpr_in_c = true\n";

#[test]
fn c_constexpr_is_opt_in() {
    let default = generate("", Language::C);
    assert!(default.contains("#define SMALL 255"));
    assert!(!default.contains("constexpr"));
    assert_eq!(
        default,
        generate("[const]\nallow_constexpr_in_c = false\n", Language::C)
    );
}

#[test]
fn c_constexpr_emits_typed_scalar_constants() {
    let output = generate(ENABLED, Language::C);
    for declaration in [
        "constexpr static const uint8_t SMALL = (uint8_t)(255);",
        "constexpr static const uint64_t LARGE = (uint64_t)(18446744073709551615ull);",
        "constexpr static const int32_t NEGATIVE = (int32_t)(-42);",
        "constexpr static const uint32_t MASK = (uint32_t)(~0u);",
        "constexpr static const uint32_t BITWISE = (uint32_t)((5 | 2));",
        "constexpr static const bool ENABLED = (bool)(true);",
        "constexpr static const uint32_t CHARACTER = (uint32_t)('x');",
        "constexpr static const float FLOAT = (float)(0.1);",
        "constexpr static const double DOUBLE = (double)(0.3333333333333333);",
        "constexpr static const double BIG_FLOAT_SIGNED = (double)(9223372036854775808ull);",
        "constexpr static const uint32_t BOUND = (uint32_t)(UINT32_MAX);",
        "constexpr static const uint32_t Aggregate_ASSOCIATED = (uint32_t)(6);",
    ] {
        assert!(
            output.contains(declaration),
            "Missing {declaration}\n{output}"
        );
    }
}

#[test]
fn c_constexpr_preserves_macro_fallbacks() {
    let output = generate(ENABLED, Language::C);
    for name in [
        "A_REFERENCE",
        "ARITHMETIC",
        "SHIFT",
        "SUM",
        "FLOAT_DIVISION",
        "NESTED_NEGATION",
        "SATURATING_CAST",
        "MIN8",
        "MIN16",
        "MIN32",
        "MIN64",
        "BIG_FLOAT",
        "SIGNED_MIN_LITERAL",
        "LATIN",
        "OFF",
        "OFF_COMPARE",
        "FLOAT_TO_INTEGER",
        "DOUBLE_CAST",
        "POINTER",
        "NULL_POINTER",
        "ARRAY",
        "STRING",
        "AGGREGATE",
        "FIELD",
        "ALIAS",
    ] {
        assert!(
            output.contains(&format!("#define {name} ")),
            "{name}\n{output}"
        );
    }
    assert!(output.find("#define A_REFERENCE").unwrap() < output.find(" Z_VALUE =").unwrap());
}

#[test]
fn c_constexpr_does_not_change_other_languages() {
    for language in [Language::Cxx, Language::Cython] {
        assert_eq!(generate("", language), generate(ENABLED, language));
        assert_eq!(
            generate("[const]\nallow_constexpr = false\n", language),
            generate(&format!("{ENABLED}allow_constexpr = false\n"), language)
        );
    }
}

#[test]
fn c_constexpr_is_independent_of_cpp_constexpr() {
    assert_eq!(
        generate(ENABLED, Language::C),
        generate(&format!("{ENABLED}allow_constexpr = false\n"), Language::C)
    );
}

// The ordinary test suite also supports compilers older than C23. Run this
// explicitly with a C23 compiler, e.g. CC=gcc-14 cargo test --test constant_c23 -- --ignored.
#[test]
#[ignore = "requires a C23 compiler (set CC)"]
fn c_constexpr_output_compiles() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("constants.c");
    let output = directory.path().join("constants");
    let assertions = r#"
static_assert(_Generic(SMALL, uint8_t: 1, default: 0));
static_assert(_Generic(LARGE, uint64_t: 1, default: 0));
static_assert(_Generic(FLOAT, float: 1, default: 0));
static_assert(_Generic(DOUBLE, double: 1, default: 0));
static_assert(SMALL == 255);
static_assert(LARGE == UINT64_MAX);
static_assert(MASK == UINT32_MAX);
static_assert(BOUND == UINT32_MAX);
static_assert(NEGATIVE == -42);
static_assert(ARITHMETIC == 18);
static_assert(BITWISE == 7);
static_assert(A_REFERENCE == 42);
static_assert(ENABLED);
static_assert(CHARACTER == 'x');
static_assert(Aggregate_ASSOCIATED == 6);
int sized_array[ARITHMETIC];
const uint8_t *typed_constant_address = &SMALL;
int main(void) {
  return FLOAT != (float)0.1 || DOUBLE != (double)(1.0 / 3.0)
      || BIG_FLOAT_SIGNED != 9223372036854775808.0;
}
"#;
    fs::write(
        &source,
        format!("{}\n{assertions}", generate(ENABLED, Language::C)),
    )
    .unwrap();
    let result = Command::new(env::var("CC").unwrap_or_else(|_| "cc".into()))
        .args(["-std=c23", "-Wall", "-Werror", "-pedantic-errors"])
        .arg(&source)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    let result = Command::new(&output).output().unwrap();
    assert!(result.status.success(), "{result:?}");
}
