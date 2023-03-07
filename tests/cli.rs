use assert_cmd::Command;
use std::{error::Error, fs};

type TestResult = Result<(), Box<dyn Error>>;

const PRG_NAME: &str = "srt2txt";
const INPUTS_PATH: &str = "tests/inputs";
const OUTPUTS_PATH: &str = "tests/outputs";

fn run(file_name: &str) -> TestResult {
    let input_file = format!("{}/{}.srt", INPUTS_PATH, file_name);
    let expected_file = format!("{}/{}.txt", OUTPUTS_PATH, file_name);
    let output_file = format!("{}/{}.txt", INPUTS_PATH, file_name);

    Command::cargo_bin(PRG_NAME)?
        .args([input_file])
        .assert()
        .success();

    let expected = fs::read_to_string(expected_file)?;
    let actual = fs::read_to_string(&output_file)?;
    assert_eq!(actual, expected);

    fs::remove_file(output_file)?;
    Ok(())
}

#[test]
fn test_cli() -> TestResult {
    run("input1")?;
    Ok(())
}
