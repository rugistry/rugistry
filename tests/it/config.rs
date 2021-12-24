use rugistry::config::{Config, Repo};

#[test]
fn test_config() {
    let config: Config = toml::from_str(r#"
        addr = "127.0.0.1:8080"

        [repo.crates]
        type = "crates"
    "#).unwrap();

    assert_eq!(config.addr, "127.0.0.1:8080");
    assert_eq!(config.repo["crates"], Repo::Crates {});
}
