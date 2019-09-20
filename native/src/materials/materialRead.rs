use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::{self, DirEntry};
use serde::Deserialize;
use toml::{ Value, Deserializer};
use toml;
use std::collections::HashMap;

#[derive(Default, Debug, Deserialize)]
pub struct Material {
    pub name: String,
    pub rgb_color: Vec<i32>,
    pub density: i32,
    pub force_diffusion: f32,
    pub slice_resistance: i32
}

#[derive(Debug)]
pub struct Material_Mem {
    pub Materials: HashMap<String, Material>
}

impl Material_Mem {
    pub fn new() -> Material_Mem {
        Material_Mem { Materials: HashMap::new() }
    }

    pub fn read_files() -> Material_Mem {
        let mut mat_builder = Material_Mem { Materials: HashMap::new()};
        let mut mat_toml = String::new();

        let dir = Path::new("./src/materials");
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries{
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().unwrap() == "toml" {
                        let mut file = match File::open(&path) {
                            Ok(file) => file,
                            Err(_) => {
                                return Material_Mem::new();
                            }
                        };

                        file.read_to_string(&mut mat_toml).unwrap_or_else(|err| panic!("error while reading material [{}]", err));

                        let mat: Material = toml::from_str(&mat_toml).unwrap();
                        let name = &mat.name;

                        mat_builder.Materials.insert(name.to_string(), mat);
                    }
                }
            }
        }
        mat_builder        
    }
}

