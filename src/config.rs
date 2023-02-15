use std::{fs, clone, ascii::AsciiExt};
use serde::{Serialize, Deserialize};
use toml;
use tui::style::Color;

// Controls
#[derive(Serialize, Deserialize, Debug)]
struct ConfigControls{
    quit: Option<char>,
    play_pause: Option<char>,
    skip: Option<char>,
    queue_add: Option<char>,
    queue_remove: Option<char>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Theme{
    foreground: Option<String>,
    background: Option<String>, 
}


// everything
#[derive(Debug)]
pub struct Config{
    quit: char,
    play_pause: char,
    skip: char,
    queue_add: char,
    queue_remove: char,
    foreground: Color,
    background: Color,
}


// for tables
#[derive(Serialize, Deserialize, Debug)]
struct ConfigTOML{
    controls: Option<ConfigControls>,
    theme: Option<Theme>, 
}

// cd
// parent dir
// select list item (up Down)
// switch grid focus (Left, right)
// chage tabs 
// # Black, Red, Yellow, Blue, Magenta, Green, Cyan, Gray, DarkGray, LightRed, LightGreen
// LightYellow, LightBlue, LightMagenta, LightCyan
 


impl Config{
    pub fn new() -> Config {

        let config_paths = [
            "./config.toml",
            "./Config.toml", 
            "~/.config/kronos/config.toml",
            "~/.config/kronos/Config.toml",
        ];

        // placeholder to store config in
        let mut content: String = "".to_owned();

        // for filepaths in above array, check to see if there is a config
        for config in config_paths{
            let result: Result<String, std::io::Error> = fs::read_to_string(config);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        // print content
        // println!("{:?}", content);

        // convert toml file to serialized data
        let config_toml: ConfigTOML = toml::from_str(&content).unwrap_or_else(|_|{
            // if config file not found, set defaults
            println!("FAILED TO CREATE CONFIG OBJECT FROM FILE");
            ConfigTOML{
                controls: None,
                theme: None,
            }
        });

        // convert found controls to variables
        let (quit, play_pause, skip, queue_add, queue_remove) = match config_toml.controls {
            Some(controls) => {

                let quit = controls.quit.unwrap_or_else(|| {
                    'q'
                });

                let play_pause = controls.play_pause.unwrap_or_else(|| {
                    'p'
                });

                let skip = controls.skip.unwrap_or_else(|| {
                    'g'
                });

                let queue_add = controls.queue_add.unwrap_or_else(|| {
                    'a'
                });

                let queue_remove = controls.queue_remove.unwrap_or_else(|| {
                    'r'
                });

                (quit, play_pause, skip, queue_add, queue_remove)
            },
            // if 0 fields filled out
            None => {
                println!("Missing data"); 
                ('q', 'p', 'g', 'a', 'r' )
            },  
        };

        // match theme
        let (foreground, background) = match config_toml.theme {

            Some(theme) => {

                // let foreground = theme.foreground.unwrap_or(Color::Black);
                let foreground = match theme.foreground.unwrap_or("LightCyan".to_string()).to_ascii_lowercase().as_ref() {
                    "black" => Color::Black, 
                    "blue" => Color::Blue,
                    "green" => Color::Green,
                    "red" => Color::Blue,
                    "yellow" => Color::Yellow,
                    "magenta" => Color::Magenta,
                    "cyan" => Color::Cyan,
                    "gray" => Color::Gray,
                    "darkgray" => Color::DarkGray,
                    "lightred" => Color::LightRed,
                    "lightgreen" => Color::LightGreen,
                    "lightyellow" => Color::LightYellow,
                    "lightblue" => Color::LightBlue,
                    "lightmagenta" => Color::LightMagenta,
                    "lightcyan" => Color::LightCyan,
                    "white" => Color::White,
                    _ => Color::Black,
                };

                let background = match theme.background.unwrap().to_ascii_lowercase().as_ref() {
                    "black" => Color::Black, 
                    "blue" => Color::Blue,
                    "green" => Color::Green,
                    "red" => Color::Blue,
                    "yellow" => Color::Yellow,
                    "magenta" => Color::Magenta,
                    "cyan" => Color::Cyan,
                    "gray" => Color::Gray,
                    "darkgray" => Color::DarkGray,
                    "lightred" => Color::LightRed,
                    "lightgreen" => Color::LightGreen,
                    "lightyellow" => Color::LightYellow,
                    "lightblue" => Color::LightBlue,
                    "lightmagenta" => Color::LightMagenta,
                    "lightcyan" => Color::LightCyan,
                    "white" => Color::White,
                    _ => Color::Black,
                };

                (foreground, background)
            }, 

            None => {
                (Color::LightCyan, Color::Black)
            }, 
            
        }; 

        

    
 
        Config {  
            quit: quit, // gathered from above 
            play_pause: play_pause,
            skip: skip,
            queue_add: queue_add,
            queue_remove: queue_remove,
            foreground: foreground,
            background: background,
        }
    }

    pub fn get_quit(&self) -> char {
        self.quit
    }

    pub fn get_play_pause(&self) -> char {
        self.play_pause
    }

    pub fn get_skip(&self) -> char {
        self.skip
    }

    pub fn get_queue_add(&self) -> char {
        self.queue_add
    }

    pub fn get_queue_remove(&self) -> char {
        self.queue_remove
    }

    pub fn get_foreground(&self) -> Color {
        self.foreground
    }

    pub fn get_background(&self) -> Color {
        self.background
    }

}
