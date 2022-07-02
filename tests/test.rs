use file_diff::diff;
use std::path::Path;
use std::{fs, io, process::Command};

const TMP_DIR: &str = "tests/tmp";
const SAMPLE_DIR: &str = "tests/samples";
const TEST_DAG_NAME: &str = "test_dag.py";
const FINAL_DAG_NAME: &str = "final_dag.py";
const TEST_K8S_NAME: &str = "test_k8s.yml";
const FINAL_K8S_NAME: &str = "final_k8s.yml";

const RULES: &str = r#"{"/":"supernova.yml","/bigdata":"common.yml","/bigdata/search":"search.yml","/bigdata/comment":"comment.yml","/bigdata/fraud":"fraud.yml","/bigdata/lqa":"lqa.yml","/bigdata/recommendation":"recommendation.yml"}"#;

#[test]
fn test_dagfile() {
    let env_name=format!("{TMP_DIR}_dags");
    create_env(&env_name);
    execute_binary(format!("{env_name}/{TEST_DAG_NAME}"));

    let res = diff(
        &*format!("{env_name}/{TEST_DAG_NAME}"),
        &*format!("{env_name}/{FINAL_DAG_NAME}"),
    );

    clean_env(&env_name);
    assert!(res);
}

#[test]
fn test_k8s_ymlfile() {
    let env_name=format!("{TMP_DIR}_k8s");
    create_env(&env_name);
    execute_binary(format!("{env_name}/{TEST_K8S_NAME}"));

    let res = diff(
        &*format!("{env_name}/{TEST_K8S_NAME}"),
        &*format!("{env_name}/{FINAL_K8S_NAME}"),
    );

    clean_env(&env_name);
    assert!(res);
}


fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn create_env(env_name: &str) {
    copy_dir_all(SAMPLE_DIR, env_name).expect("failed to create test environment");
}
fn clean_env(env_name:&str) {
    fs::remove_dir_all(env_name).expect("couldn't do the clean up");
}
fn execute_binary(path: String) {
    let output = Command::new("target/debug/dagger")
        .arg("--image")
        .arg("dagger:1.1.3")
        .arg("--pattern")
        .arg("/bigdata/recommendation/buying-habits")
        .arg("--hash")
        .arg("https://git.digikala.com/bigdata/recommendation/buying-habits")
        .arg(path)
        .env("rules", RULES)
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
