// crates
/*
use std::path::Path;
use walkdir::WalkDir;
use my_public_ip::resolve;
*/ // unused crates
use colored::{Color, Colorize};
use directories::ProjectDirs;
use libmacchina::GeneralReadout;
use libmacchina::PackageReadout;
use local_ip_address::local_ip;
use serde::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::str;
use std::str::FromStr;
use sys_info::{cpu_num, mem_info};
use titlecase::titlecase;
use uname::uname;
//use std::io::Write;

#[derive(Deserialize)]
struct Config {
    // packages: String,
    info_color: String,
    //logo_color: String,
    os: String,
}

pub struct ColorCodeIter<I: Iterator<Item = char>> {
    iter: I,
    color: Color,
    remainder: Option<char>,
    buffer: String,
}

impl<I: Iterator<Item = char>> ColorCodeIter<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            color: Color::Blue, /* or something like that */
            remainder: None,
            buffer: String::with_capacity(4),
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for ColorCodeIter<I> {
    type Item = (char, Color);

    fn next(&mut self) -> Option<(char, Color)> {
        if let Some(r) = self.remainder {
            self.remainder = None;
            return Some((r, self.color));
        }

        match self.iter.next() {
            Some('$') => match self.iter.next() {
                Some('{') => loop {
                    match self.iter.next() {
                        Some('}') => {
                            if let Ok(color) = Color::from_str(self.buffer.as_str()) {
                                self.color = color;
                            }
                            self.buffer.clear();

                            break Some((self.iter.next()?, self.color));
                        }
                        Some(v) => self.buffer.push(v),
                        None => (),
                    }
                },
                Some(v) => {
                    self.remainder = Some(v);
                    Some(('$', self.color))
                }
                None => None,
            },
            Some('\\') => Some((self.iter.next()?, self.color)),
            v => Some((v?, self.color)),
        }
    }
}

fn main() {
    let user_name = titlecase(&whoami::username()); // username
    let host_name = whoami::devicename(); // hostname
    let title_length = host_name.chars().count() + user_name.chars().count() + 3; //length of hostname and username + @ symbol
    let os = whoami::distro(); // distro (and version if not rolling release)

    let kernel = uname().unwrap().release; // kernel

    // User Shell
    let usr_shell = env::var("SHELL").expect("$SHELL is not set");

    // Checks current terminal
    use libmacchina::traits::GeneralReadout as _;
    let mut terminal = titlecase(&GeneralReadout::new().terminal().unwrap());
    if terminal == "Kitty" {
        terminal = terminal + " 🐱";
    }

    // Checks CPU name and cores
    let cpu_info = GeneralReadout::new().cpu_model_name().unwrap();

    let local_ip = ip();
    // Checks which GPUs you have
    //gpu_find();

    // Checks memory info
    let mem = mem_info().unwrap();
    let mem_used = mem.total / 1024 - mem.avail / 1024;
    let mem_percent: f32 = ((mem_used as f32) / ((mem.total as f32) / 1024.0) * 100.0) as f32;

    // Checks Screen Resolution info
    //    let res_info = GeneralReadout::new().resolution().unwrap();

    // print outs
    // println!("{}", title_length); // prints length of title
    if let Some(proj_dirs) = ProjectDirs::from("dev", "Kara-Wilson", "rust-fetch") {
        let config_dir = proj_dirs.config_dir();

        let config_file = fs::read_to_string(config_dir.join("config.toml"));

        let config: Config = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => Config {
                // packages: "path".to_string(),
                info_color: "blue".to_string(),
                //logo_color: "magenta".to_string(),
                os: "arch".to_string(),
            },
        };
        let modules: [String; 19] = [
            format!(
                "{} {} {}",
                user_name.color(config.info_color.clone()),
                "@".blue().bold(),
                host_name.color(config.info_color.clone())
            ),
            format!("{:—<1$}", "", title_length),
            format!(
                "{} {}",
                "OS:".color(config.info_color.clone()).bold(),
                os.normal()
            ),
            if let Some(model) = device_model() {
                if model != "" {
                    format!(
                        "{} {}",
                        "Model:".color(config.info_color.clone()).bold(),
                        model.normal()
                    )
                } else {
                    format!(
                        "{} {}",
                        "Model:".color(config.info_color.clone()).bold(),
                        "Model not found".normal()
                    )
                }
            } else {
                format!(
                    "{} {}",
                    "Model:".color(config.info_color.clone()).bold(),
                    "Model not found".normal()
                )
            },
            format!(
                "{} {}",
                "Kernel:".color(config.info_color.clone()).bold(),
                kernel.normal()
            ),
            format!(
                "{} {}",
                "Uptime:".color(config.info_color.clone()).bold(),
                uptime_time().normal()
            ),
            if let Some(how_many) = packages() {
                if how_many != "" {
                    format!(
                        "{} {}",
                        "Packages:".color(config.info_color.clone()).bold(),
                        how_many.normal()
                    )
                } else {
                    format!(
                        "{} {}",
                        "Packages:".color(config.info_color.clone()).bold(),
                        "packages not found".normal()
                    )
                }
            } else {
                format!(
                    "{} {}",
                    "Packages:".color(config.info_color.clone()).bold(),
                    "packages not found".normal()
                )
            },
            if usr_shell != "" {
                format!(
                    "{} {}",
                    "Defualt Shell:".color(config.info_color.clone()).bold(),
                    usr_shell.normal()
                )
            } else {
                format!(
                    "{} {}",
                    "Default Shell:".color(config.info_color.clone()).bold(),
                    "defualt shell not found".normal()
                )
            },
            /*
            format!("{} {}",
                     "Screen Resolution:".color(config.info_color.clone()).bold(),
                     res_info.normal()),
            */
            format!(
                "{} {}",
                "DE/WM:".color(config.info_color.clone()).bold(),
                wm_de().normal()
            ),
            format!(
                "{} {}",
                "GTK Theme:".color(config.info_color.clone()).bold(),
                gtk_theme_find().normal()
            ),
            format!(
                "{} {}",
                "GTK Icon Theme:".color(config.info_color.clone()).bold(),
                gtk_icon_find().normal()
            ),
            format!(
                "{} {}",
                "Terminal:".color(config.info_color.clone()).bold(),
                terminal.normal()
            ),
            format!(
                "{} {} {}{}{}",
                "CPU:".color(config.info_color.clone()).bold(),
                cpu_info.normal(),
                "(".normal(),
                cpu_usage_info(),
                "%)"
            ),
            if gpu_find().contains(", ") {
                format!(
                    "{} {}",
                    "GPUs:".color(config.info_color.clone()).bold(),
                    gpu_find().normal()
                )
            } else {
                format!(
                    "{} {}",
                    "GPU:".color(config.info_color.clone()).bold(),
                    gpu_find().normal()
                )
            },
            format!(
                "{} {}{}{}{}{:.2}{}",
                "Memory:".color(config.info_color.clone()).bold(),
                mem_used.to_string().normal(),
                "Mib / ",
                mem.total / 1024,
                "Mib (",
                mem_percent,
                "%)"
            ),
            if let Some((per, state)) = battery_percentage() {
                if per != "" && state != "" {
                    format!(
                        "{} {} {}{}{}",
                        "Battery:".color(config.info_color.clone()).bold(),
                        per.normal(),
                        "[",
                        state,
                        "]"
                    )
                } else if per != "" {
                    format!(
                        "{} {}",
                        "Battery:".color(config.info_color.clone()).bold(),
                        per.normal()
                    )
                } else {
                    format!(
                        "{} {}",
                        "Battery:".color(config.info_color.clone()).bold(),
                        "battery info not found".normal()
                    )
                }
            } else {
                format!(
                    "{} {}",
                    "Battery:".color(config.info_color.clone()).bold(),
                    "battery info not found".normal()
                )
            },
            if let Some(users) = user_list() {
                if users != "" {
                    format!(
                        "{} {}",
                        "Users:".color(config.info_color.clone()).bold(),
                        users.normal()
                    )
                } else {
                    format!(
                        "{} {}",
                        "Users:".color(config.info_color.clone()).bold(),
                        "users not found".normal()
                    )
                }
            } else {
                format!(
                    "{} {}",
                    "Users:".color(config.info_color.clone()).bold(),
                    "users not found".normal()
                )
            },
            format!(
                "{} {} {}",
                "IP:".color(config.info_color.clone()).bold(),
                local_ip.normal(),
                "[Local]".normal()
            ),
            format!(""),
        ];
        let ascii = config_dir.join("ascii_art/").join(&config.os);
        let buf = std::fs::read_to_string(ascii).unwrap();
        let mut i = 0;
        for (character, color) in ColorCodeIter::new(buf.chars()) {
            if character != '\n' {
                print!("{}", character.to_string().color(color));
            } else {
                println!("{} ", modules[i]);
                i += 1;
            }
        }
    }
}

pub fn uptime_time() -> String {
    let mut output = String::new();
    let mut uptime_f = File::open("/proc/uptime").expect("Unable to open the file");
    let mut uptime = String::new();
    uptime_f
        .read_to_string(&mut uptime)
        .expect("Unable to open the file");
    let uptime: f32 = uptime.split(' ').collect::<Vec<&str>>()[0].parse().unwrap();

    let hour = uptime.round() as u32 / 3600;
    let rem = uptime as u32 - hour * 3600;
    let minutes = rem / 60;
    let day = hour as u32 / 24;
    let hour = &hour - day * 24;
    if day > 0 {
        output += &day.to_string();
        output += " days, ";
        output += &hour.to_string();
        output += " hours, ";
        output += &minutes.to_string();
        output += " min";
    } else if day <= 0 && hour > 0 {
        output += &hour.to_string();
        output += " hours, ";
        output += &minutes.to_string();
        output += " mins";
    } else {
        output += &minutes.to_string();
        output += " min";
    }
    output
}

pub fn gpu_find() -> String {
    let mut gpus = Command::new("sh");
    gpus.arg("-c");
    gpus.arg("lspci | grep -i 'vga\\|3d\\|2d' | cut -d ':' -f3 | cut -d '[' -f2 | cut -d ']' -f1");
    let gpu_out = gpus.output().expect("failed to execute process").stdout;
    let gpu_out = match str::from_utf8(&gpu_out) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let gpu_out = &gpu_out.replace("\n", ", ");
    let gpu_out = &gpu_out[0..&gpu_out.len() - 2];
    // if gpu_out.contains(", ") {
    //     println!("GPUs: {}", gpu_out);
    // } else {
    //     println!("GPU: {}", gpu_out);
    // }

    gpu_out.to_string()
}

pub fn gtk_theme_find() -> String {
    let gtk_cmd = "cat $HOME/.config/gtk-3.0/settings.ini | grep gtk-theme-name | cut -d '=' -f2";
    let mut gtk_theme = Command::new("sh");
    gtk_theme.arg("-c");
    gtk_theme.arg(gtk_cmd);
    let gtk = gtk_theme
        .output()
        .expect("failed to execute process")
        .stdout;
    let gtk = match str::from_utf8(&gtk) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let gtk = &gtk.replace("\n", "");
    gtk.to_string()
}

pub fn gtk_icon_find() -> String {
    let gtk_cmd =
        "cat $HOME/.config/gtk-3.0/settings.ini | grep gtk-icon-theme-name | cut -d '=' -f2";
    let mut gtk_icon_theme = Command::new("sh");
    gtk_icon_theme.arg("-c");
    gtk_icon_theme.arg(gtk_cmd);
    let gtk_icon = gtk_icon_theme
        .output()
        .expect("failed to execute process")
        .stdout;
    let gtk_icon = match str::from_utf8(&gtk_icon) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let gtk_icon = &gtk_icon.replace("\n", "");
    gtk_icon.to_string()
}

pub fn cpu_usage_info() -> f32 {
    let cores = cpu_num().unwrap();

    let cpu_use_out = Command::new("sh")
        .arg("-c")
        .arg("ps aux | awk 'BEGIN {sum=0} {sum+=$3}; END {print sum}'")
        .output()
        .expect("failed to execute process")
        .stdout;

    let cpu_use = str::from_utf8(&cpu_use_out)
        .expect("cpu usage not utf-8")
        .trim()
        .parse::<f32>()
        .expect("cpu usage not a number");
    // let cpu_use = &cpu_use.replace("\n", "");
    let cpu_avg = (cpu_use / cores as f32).round();

    cpu_avg
}
pub fn battery_percentage() -> Option<(String, String)> {
    let battery_out = Command::new("sh")
        .arg("-c")
        .arg("upower -i `upower -e | grep 'BAT'` | grep 'percentage:' | tail -c 5")
        .output()
        .expect("failed to execute process")
        .stdout;
    let battery_per = str::from_utf8(&battery_out)
        .expect("battery output not utf-8")
        .trim()
        //.replace("%", "")
        //.parse::<i8>()
        .to_string();
    //.expect("battery output not a string");

    let state = Command::new("sh")
        .arg("-c")
        .arg("upower -i `upower -e | grep 'BAT'` | grep 'state:' | awk 'NF>1{print $NF}'")
        .output()
        .expect("failed to execute process")
        .stdout;

    let battery_state = str::from_utf8(&state)
        .expect("battery status not utf-8")
        .trim()
        .to_string();

    return Some((battery_per, battery_state));
}

pub fn user_list() -> Option<String> {
    let users = Command::new("sh")
        .arg("-c")
        .arg("awk -F':' '{ if($3 >= 1000 && $3 <= 6000) {print $1}}' /etc/passwd")
        .output()
        .expect("failed to execute process")
        .stdout;
    let user_out = str::from_utf8(&users)
        .expect("users output not utf-8")
        .trim()
        .replace("\n", ", ");
    return Some(user_out);
}

fn device_model() -> Option<String> {
    let mut product_name =
        File::open("/sys/devices/virtual/dmi/id/product_name").expect("Unable to open the file");

    let mut product_version =
        File::open("/sys/devices/virtual/dmi/id/product_version").expect("Unable to open the file");

    let mut model = String::new();
    product_name
        .read_to_string(&mut model)
        .expect("Unable to read the file"); // gets product name
    let _ = product_version
        .read_to_string(&mut model)
        .expect("Unable to read the file"); // get number revision
    let model = model.replace("\n", " ");
    return Some(model);
}

fn packages() -> Option<String> {
    let mut how_many = String::new();
    use libmacchina::traits::PackageReadout as _;
    let readout = PackageReadout::new();
    let count = readout.count_pkgs();
    for n in count {
        let (pm, count) = n;
        how_many += pm.to_string().as_str();
        how_many += " (";
        how_many += count.to_string().as_str();
        how_many +=") ";
    }
    Some(how_many.to_string())
}

pub fn wm_de() -> String {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();
    let resolution = general_readout
        .desktop_environment()
        .expect("Failed to get desktop environment");
    resolution
}

pub fn ip() -> String {
    let my_local_ip = local_ip().unwrap().to_string();
    //let my_public_ip = my_public_ip::resolve().unwrap().to_string();

    return my_local_ip;
}

/* Todo:
[ X ] OS
[ X ] Host
[ X ] Model
[ X ] kernel
[ X ] Uptime
[ / ] Packages (add appimages)
[ X ] PATH Binaries
[ X ] Shell
[ X ] Resolution
[ X ] DE
[ X ] WM
[ X ] GTK Theme
[ X ] GTK Icons
[ X ] Terminal
[ N ] Terminal Font (as far as i can tell not possible unless testing in every terminal)
[ X ] CPU
[ X ] GPU
[ X ] Memory
Others:
[ X ] CPU Usage
[   ] Disk (KDE partition manager, my results and neofetches results do not line up with any of each other so will do more research later)
[ X ] Battery
[   ] Song
[ X ] Local IP
[ X ] Public IP (these 2 doubled runtime so disabled by default)
[ X ] Users
*/
/* Non-feature Specific Todos:
[ X ] Check for days with uptime
 */

/* ⚠ISSUES:⚠
- [ ] No wayland suppport
- [ ] inaccurate memory usage
*/
