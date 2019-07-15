#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
pub mod cfg {

    use failure::Fallible;
    use named_type::NamedType;
    pub use serde::{Deserialize, Serialize};
    use toml::Value;

    #[derive(Debug, Deserialize)]
    pub struct Configuration {
        pub filename: &'static str,
        config: Value,
    }

    impl Configuration {
        pub fn new(filename: &'static str) -> Self {
            use std::fs::File;
            use std::io::prelude::*;
            if let Ok(mut file) = File::open(filename) {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Could not read_to_string");
                match toml::from_str::<Value>(&contents) {
                    Ok(cfg) => Configuration {
                        filename,
                        config: cfg,
                    },
                    Err(e) => {
                        panic!("Error reading config: {}", e);
                    }
                }
            } else {
                panic!(
                    "Configuration::new could not find the toml file: {}",
                    filename
                );
            }
        }

        pub fn try_parse<T>(&self) -> Fallible<T>
        where
            for<'de> T: Deserialize<'de> + NamedType + Serialize,
        {
            match &self.config {
                Value::Table(obj_map) => {
                    let key = T::short_type_name();
                    if let Some(val) = obj_map.get(*&key) {
                        return Ok(val.clone().try_into::<T>()?);
                    }
                }
                _ => {}
            };
            bail!("Could not parse the config, it must return at the very least a Value::Table");
        }

        // try parsing the root config with the base root props, this is for when reused prop
        // values and to avoid repeating identical properties in each main subheading
        pub fn try_parse_props<T>(&self) -> Fallible<T>
        where
            for<'de> T: Deserialize<'de> + Serialize,
        {
            return Ok(self.config.clone().try_into::<T>()?);
        }

        pub fn parse_or_default<T>(&self) -> T
        where
            for<'de> T: Deserialize<'de> + NamedType + std::default::Default + Serialize,
        {
            match &self.config {
                Value::Table(obj_map) => {
                    let key = T::short_type_name();
                    if let Some(val) = obj_map.get(*&key) {
                        if let Ok(meta) = val.clone().try_into::<T>() {
                            return meta;
                        }
                    }
                }
                _ => {}
            };
            return T::default();
        }
    }

}
