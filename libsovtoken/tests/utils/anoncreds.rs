use utils::environment::EnvironmentUtils;

pub fn tails_writer_config() -> String {
    let mut base_dir = EnvironmentUtils::tmp_path();
    base_dir.push("tails");

    let json = json!({
                "base_dir": base_dir.to_str().unwrap(),
                "uri_pattern":"",
            });
    json.to_string()
}