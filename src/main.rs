extern crate winreg;
extern crate steamy_vdf as vdf;
use std::io;
use std::io::{Error, ErrorKind};
use std::collections::{HashSet, HashMap};
use winreg::enums::*;
use winreg::RegKey;
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

fn main() -> io::Result<()>{

    let steam_path = match read_steam_path() {
        Ok(first_number) => first_number,
        Err(_e) => { return Err(Error::new(ErrorKind::InvalidInput, "Steam installation not found.")); }
    };
    println!("SteamPath = {}", steam_path);

    // let contents = fs::read_to_string(steam_path + "/config/loginusers.vdf").expect("Something went wrong reading the file.");
    let config = vdf::load(steam_path + "/config/loginusers.vdf").expect("Something went wrong reading the file.");
    let configTable = config.as_table().unwrap();

    let set1 = configTable.keys();
    // println!("result = {}", set1);

    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
    app.run().unwrap();

    Ok(())
}

fn read_steam_path() -> io::Result<String> {
    // HKEY_CURRENT_USER\SOFTWARE\Valve\Steam
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Valve\\Steam")?; 
    let steam_path: String = cur_ver.get_value("SteamPath")?;
    
    Ok(steam_path)
}