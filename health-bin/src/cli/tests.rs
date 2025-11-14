use super::*;

#[test]
fn test_parse_output_flag_normal() {
    let args = vec![
        "healthcheck".to_string(),
        "generate-bin".to_string(),
        "--output".to_string(),
        "./bin".to_string(),
    ];
    let result = parse_output_flag(&args, "generate-bin");
    assert_eq!(result, Some("./bin".to_string()));
}

#[test]
fn test_parse_output_flag_none() {
    let args = vec!["healthcheck".to_string(), "generate-bin".to_string()];
    let result = parse_output_flag(&args, "generate-bin");
    assert_eq!(result, None);
}

#[test]
fn test_parse_output_flag_position_independent() {
    // --output can appear anywhere after the command
    let args = vec![
        "healthcheck".to_string(),
        "generate-bin".to_string(),
        "some-other-arg".to_string(),
        "--output".to_string(),
        "./bin".to_string(),
    ];
    let result = parse_output_flag(&args, "generate-bin");
    assert_eq!(result, Some("./bin".to_string()));
}

#[test]
fn test_parse_output_flag_correct_index() {
    // This test catches the off-by-2 index bug
    // args[0] = healthcheck, args[1] = generate-bin, args[2] = --output, args[3] = ./bin
    let args = vec![
        "healthcheck".to_string(),
        "generate-bin".to_string(),
        "--output".to_string(),
        "./bin".to_string(),
    ];
    let result = parse_output_flag(&args, "generate-bin");
    // Should return "./bin" (args[3]), not "generate-bin" (args[1])
    assert_eq!(result, Some("./bin".to_string()));
    assert_ne!(result, Some("generate-bin".to_string()));
}
