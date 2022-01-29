use cursive::theme::{BaseColor, Color};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use std;
use std::default::Default;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

pub const VIEW_WIDTH: usize = 36;
pub const VIEW_HEIGHT: usize = 8;

#[derive(Serialize, Deserialize)]
pub struct Look {
    #[serde(default = "base_char")]
    pub true_chr: char,
    #[serde(default = "base_char")]
    pub false_chr: char,
    #[serde(default = "base_char")]
    pub future_chr: char,
    #[serde(default = "grid_size")]
    pub grid_size: usize,
}

fn base_char() -> char {
    '·'
}

fn grid_size() -> usize {
    3
}

impl Default for Look {
    fn default() -> Self {
        Look {
            true_chr: '·',
            false_chr: '·',
            future_chr: '·',
            grid_size: 3,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Colors {
    #[serde(default = "cyan")]
    pub reached: String,
    #[serde(default = "magenta")]
    pub todo: String,
    #[serde(default = "light_black")]
    pub inactive: String,
}

fn cyan() -> String {
    "cyan".into()
}
fn magenta() -> String {
    "magenta".into()
}
fn light_black() -> String {
    "light black".into()
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            reached: cyan(),
            todo: magenta(),
            inactive: light_black(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub look: Look,

    #[serde(default)]
    pub colors: Colors,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            look: Default::default(),
            colors: Default::default(),
        }
    }
}

impl AppConfig {
    // TODO: implement string parsing from config.json
    pub fn reached_color(&self) -> Color {
        Color::parse(&self.colors.reached).unwrap_or(Color::Dark(BaseColor::Cyan))
    }
    pub fn todo_color(&self) -> Color {
        Color::parse(&self.colors.todo).unwrap_or(Color::Dark(BaseColor::Magenta))
    }
    pub fn inactive_color(&self) -> Color {
        Color::parse(&self.colors.inactive).unwrap_or(Color::Light(BaseColor::Black))
    }
    pub fn grid_size(&self) -> usize {
        self.look.grid_size
    }
}

pub fn load_configuration_file() -> AppConfig {
    let cf = config_file();
    if let Ok(ref mut f) = File::open(&cf) {
        let mut j = String::new();
        f.read_to_string(&mut j);
        return toml::from_str(&j).unwrap_or_else(|e| panic!("Invalid config file: `{}`", e));
    } else {
        if let Ok(dc) = toml::to_string(&AppConfig::default()) {
            match OpenOptions::new().create(true).write(true).open(&cf) {
                Ok(ref mut file) => file.write(dc.as_bytes()).unwrap(),
                Err(_) => panic!("Unable to write config file to disk!"),
            };
        }
        return Default::default();
    }
}

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("rs", "nerdypepper", "dijo")
        .unwrap_or_else(|| panic!("Invalid home directory!"))
}

pub fn config_file() -> PathBuf {
    let proj_dirs = project_dirs();
    let mut data_file = PathBuf::from(proj_dirs.config_dir());
    fs::create_dir_all(&data_file);
    data_file.push("config.toml");
    return data_file;
}

pub fn habit_file() -> PathBuf {
    let proj_dirs = project_dirs();
    let mut data_file = PathBuf::from(proj_dirs.data_dir());
    fs::create_dir_all(&data_file);
    data_file.push("habit_record.json");
    return data_file;
}

pub fn auto_habit_file() -> PathBuf {
    let proj_dirs = project_dirs();
    let mut data_file = PathBuf::from(proj_dirs.data_dir());
    fs::create_dir_all(&data_file);
    data_file.push("habit_record[auto].json");
    return data_file;
}
