#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;
pub mod cfg {

    use serde::{Serialize, Deserialize};
    use toml::{Value};
    use named_type::NamedType;
    use failure::{Fallible};

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
                file.read_to_string(&mut contents).expect("Could not read_to_string");
                match toml::from_str::<Value>(&contents) {
                    Ok(cfg) => {
                        Configuration {
                            filename,
                            config: cfg,
                        }
                    },
                    Err(e) => {
                        panic!("Error reading config: {}", e);
                    }
                }
            } else {
                panic!("Configuration::new could not find the toml file: {}", filename);
            }
        }

        pub fn try_parse<T> (&self) -> Fallible<T>
            where for<'de> T: Deserialize<'de> + NamedType + Serialize
            {
                match &self.config {
                    Value::Table(obj_map) => {
                        let key = T::short_type_name();
                        if let Some(val) = obj_map.get(*&key) {
                            /*
                            if let Ok(meta) = val.clone().try_into::<T>() {
                                return Ok(meta);
                            }
                            */
                            return Ok(val.clone().try_into::<T>()?);
                        }
                    },
                    _ => {
                    },
                };
                bail!("Could not parse the config");
            }

        pub fn parse_or_default<T> (&self) -> T
            where for<'de> T: Deserialize<'de> + NamedType + std::default::Default + Serialize
            {
                match &self.config {
                    Value::Table(obj_map) => {
                        let key = T::short_type_name();
                        if let Some(val) = obj_map.get(*&key) {
                            if let Ok(meta) = val.clone().try_into::<T>() {
                                return meta;
                            }
                        }
                    },
                    _ => {
                    },
                };
                return T::default();
            }

    }

}
