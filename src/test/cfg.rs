//#[macro_use] extern crate named_type_derive;
extern crate named_type_derive;
#[macro_use] extern crate serde_derive;
extern crate toml_config_util;
#[cfg(test)]
mod tests {
    extern crate toml;
    extern crate serde;
    use named_type_derive::*;
    use named_type::NamedType;
    use toml_config_util::cfg::*;

    #[derive(Debug, Serialize, Deserialize, NamedType)]
    struct TestConfig {
        name: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct CommonConfig {
        base_prop: String,
    }

    #[test]
    fn test_configuration_parsing() {
        let cfg = Configuration::new("src/test/test.toml");
        let maybe_cfg = cfg.try_parse::<TestConfig>();
        assert_eq!(maybe_cfg.is_ok(), true);
        let test_cfg = maybe_cfg.unwrap();
        assert_eq!(test_cfg.name, "this is a test name");
        println!("the configuration: {:?}", &test_cfg);
    }

    #[test]
    fn test_base_props_parsing() {
        let cfg = Configuration::new("src/test/test.toml");
        let maybe_cfg = cfg.try_parse_props::<CommonConfig>();
        assert_eq!(maybe_cfg.is_ok(), true);
        let test_cfg = maybe_cfg.unwrap();
        assert_eq!(test_cfg.base_prop, "IsNotNamedType");
        println!("the configuration: {:?}", &test_cfg);
    }

}
