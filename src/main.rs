extern crate core;

use std::env;
use std::io::ErrorKind;

mod ls;

fn main() {
    let args = env::args();

    let mut show_hidden=false;

    let path= match env::current_dir() {
        Ok(v) => {v}
        Err(error) => { match error.kind() {
            ErrorKind::NotFound => {panic!("Path does not exist")}
            ErrorKind::PermissionDenied => {panic!("Lack of Permission")}
            ErrorKind::Other => {panic!("wee")}
            _ =>{panic!("Something bad happened")}
        } }
    };
    for x in args {
        if x.eq("-a"){
            show_hidden = true;
        }
    }
    ls::list_files_in_current_dir(show_hidden, path);
    print!("\n");
}
