// crates
/*
use std::path::Path;
*/ // unused crates
use std::process::Command;
use walkdir::WalkDir;
use std::fs::File;
use std::fs;
use std::io::Read;
use titlecase::titlecase;
use uname::uname;
use std::env;
use libmacchina::GeneralReadout;
extern crate sys_info;
use sys_info::mem_info;


fn main() {
    println!("rustFetch");

    /* let mut host_name_file = File::open("/etc/hostname").unwrap();
    let mut host_name = String::new();
    host_name_file.read_to_string(&mut host_name).unwrap();*/

    let user_name = titlecase(&whoami::username()); // username
    let host_name = whoami::devicename(); // hostname
    let title_length = host_name.chars().count() + user_name.chars().count() + 1; //length of hostname and username + @ symbol
    let os = whoami::distro(); // distro (and version if not rolling release)


    // devices model
    let mut product_name = File::open("/sys/devices/virtual/dmi/id/product_name")
        .expect("Unable to open the file");

    let mut product_version = File::open("/sys/devices/virtual/dmi/id/product_version")
        .expect("Unable to open the file");

    let mut model = String::new();
    product_name.read_to_string(&mut model)
        .expect("Unable to read the file"); // gets product name
    let _ = product_version.read_to_string(&mut model)
        .expect("Unable to read the file"); // get number revision
    let model = model.replace("\n", " ");

    let kernel = uname().unwrap().release; // kernel

    // get uptime
    let mut uptime_f = File::open("/proc/uptime")
        .expect("Unable to open the file");
    let mut uptime = String::new();
    uptime_f.read_to_string(&mut uptime)
        .expect("Unable to open the file");
      let uptime: f32 = uptime.split(' ').collect::<Vec<&str>>()[0].parse().unwrap();

      let hour = uptime.round() as u32 / 3600;
      let rem = uptime as u32 - hour * 3600;
      let minutes = rem / 60;

    // Do packages fully later
    // Package managers
    /* Todo List: // append all that exist to string and print string
    [/] pacman
    [/] apt
    [/] pip
    [X] cargo
    [/] flatpak
     */
    // check if pacman exists
    let pac_e = std::path::Path::new("/bin/pacman").exists() | std::path::Path::new("/usr/bin/pacman").exists();
    // check if apt exists
    let apt_e = std::path::Path::new("/bin/apt").exists() | std::path::Path::new("/usr/bin/apt").exists();
    // check if pip exists
    let pip_e = std::path::Path::new("/bin/pip").exists() | std::path::Path::new("/usr/bin/pip").exists() | std::path::Path::new("/bin/pip3").exists() | std::path::Path::new("/usr/bin/pip3").exists();
    // check if cargo exists
    let cargo_e = std::path::Path::new("/bin/cargo").exists() | std::path::Path::new("/usr/bin/cargo").exists();
    // check if flatpak exists
    let flatpak_e = std::path::Path::new("/bin/flatpak").exists() | std::path::Path::new("/usr/bin/flatpak").exists();

    // checks how many files cargo has
    let cargo_dir: String = "/home/".to_owned() + &whoami::username() + "/.cargo/bin";
    let cargo = fs::read_dir(cargo_dir).unwrap().count();

    // User Shell
    let usr_shell = env::var("SHELL").expect("$SHELL is not set");

    // Checks users desktop Env
    let mut de = env::var("XDG_CURRENT_DESKTOP")
        .expect("$XDG_CURRENT_DESKTOP is not set"); /* +
        " " +
        &env::var("DESKTOP_SESSION")
        .expect("$DESKTOP_SESSION is not set"); */
    de = titlecase(&de);

    // Checks current terminal
    use libmacchina::traits::GeneralReadout as _;
    let terminal = titlecase(&GeneralReadout::new().terminal().unwrap());

    // Checks CPU name and cores
    let cpu_info = GeneralReadout::new().cpu_model_name().unwrap();

    // Checks memory info
    let mem = mem_info().unwrap();
      let mem_used = mem.total/1024 - mem.avail/1024;
      let mem_percent: f32  = ((mem_used as f32)/((mem.total as f32)/1024.0)*100.0) as f32;



    // print outs
    println!("{}@{}", user_name, host_name);
    // println!("{}", title_length); // prints length of title
    println!("{:-<1$}", "", title_length);
    println!("OS: {}", os);
    println!("model: {}", model);
    println!("Kernel: {}", kernel);
    if hour > 0 {
        println!("Uptime: {} hours, {} minutes", hour, minutes);
    } else {
        println!("Uptime: {} minutes", minutes);
    };
    // files exists?
    println!(" - Pacman exists? {}", pac_e);
    println!(" - APT exists? {}", apt_e);
    println!(" - PIP exists? {}", pip_e);
    println!(" - Cargo exists? {}", cargo_e);
    println!(" - Flatpak exists? {}", flatpak_e);
    println!(" - Cargo ({})", cargo);
    println!("Defualt Shell: {}", usr_shell);
    println!("DE/WM: {}", de);
    println!("Terminal: {}", terminal);
    println!("CPU: {}", cpu_info);
    println!("Memory: {}Mib / {}Mib ({:.2}%)", mem_used, mem.total/1024, mem_percent);
}



/*
[ X ] OS
[ X ] Host
[ X ] Model
[ X ] kernel
[ X ] Uptime
[ / ] Packages
[ X ] Shell
[   ] Resolution
[ X ] DE
[ X ] WM
[   ] Theme
[   ] Icons
[ X ] Terminal
[   ] Terminal Font
[ X ] CPU
[   ] GPU
[ X ] Memory
*/