use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::ops::Add;
use std::path::PathBuf;
use colored::{Color, Colorize};
use terminal_size::Width;

pub fn list_files_in_current_dir(show_hidden:bool,path: PathBuf) {

    let size = match terminal_size::terminal_size() {
        None => {Width(0)}
        Some(val) => {val.0}
    };
    let result= fs::read_dir(path);
    match result
    {

        Ok(value) => {
            let mut max_width:i32 =0;
            // turn it into a map with k,v : k = DirEntry and v = filetype  | use an enum to learn rust ;) ||| check
            let mut arr_of_dirs_and_files: HashMap<MyFileType,Vec<String>> = HashMap::from([(MyFileType::HiddenFile,vec![]),(MyFileType::NormalFile,vec![]),(MyFileType::HiddenFolder,vec![]),(MyFileType::NormalFolder,vec![])]);
                for v in value {
                    match v.as_ref().unwrap().file_type() {
                        Ok(file_type) => {
                            match v.as_ref() {
                                Ok(file) => {
                                    let is_hidden:bool = file.file_name().to_string_lossy().starts_with(".");
                                    if file_type.is_dir() == true  {
                                        if is_hidden && show_hidden
                                        {
                                            if file.file_name().clone().to_string_lossy().chars().count() > max_width as usize
                                            {
                                                max_width = file.file_name().clone().to_string_lossy().chars().count() as i32;
                                            }
                                            arr_of_dirs_and_files = add_element_to_map(arr_of_dirs_and_files, file, &MyFileType::HiddenFolder);
                                        }
                                        else {
                                            if file.file_name().clone().to_string_lossy().chars().count() > max_width as usize
                                            {
                                                max_width = file.file_name().clone().to_string_lossy().chars().count() as i32;
                                            }
                                            arr_of_dirs_and_files = add_element_to_map(arr_of_dirs_and_files, file, &MyFileType::NormalFolder);
                                        }
                                    }
                                    if file_type.is_file() == true  {
                                        if is_hidden && show_hidden
                                        {
                                            if file.file_name().clone().to_string_lossy().chars().count() > max_width as usize
                                            {
                                                max_width = file.file_name().clone().to_string_lossy().chars().count() as i32;
                                            }
                                            arr_of_dirs_and_files = add_element_to_map(arr_of_dirs_and_files, file, &MyFileType::HiddenFile);
                                        }
                                        else {
                                            if file.file_name().clone().to_string_lossy().chars().count() > max_width as usize
                                            {
                                                max_width = file.file_name().clone().to_string_lossy().chars().count() as i32;
                                            }
                                            arr_of_dirs_and_files = add_element_to_map(arr_of_dirs_and_files, file, &MyFileType::NormalFile);
                                        }
                                    }
                                }
                                Err(e) => {eprintln!("{}",e)}
                            }
                        }
                        Err(err) => {eprintln!("{}",err)}
                    }
                }
            //eprintln!("{:?}",arr_of_dirs_and_files);
            arr_of_dirs_and_files.get_mut(&MyFileType::HiddenFile).unwrap().sort_by(|f ,a | f.cmp(a));
            arr_of_dirs_and_files.get_mut(&MyFileType::NormalFile).unwrap().sort_by(|f ,a | f.cmp(a));
            arr_of_dirs_and_files.get_mut(&MyFileType::HiddenFolder).unwrap().sort_by(|f ,a | f.cmp(a));
            arr_of_dirs_and_files.get_mut(&MyFileType::NormalFolder).unwrap().sort_by(|f ,a | f.cmp(a));

            let mut item_count:i32 =1;
            let max_items = calc_max_items(size.0 as i32,max_width,&arr_of_dirs_and_files);
            //Hidden folders
            item_count = print_file_of_type(&MyFileType::HiddenFolder,&arr_of_dirs_and_files, Color::Blue,max_items,item_count,max_width);
            //Hidden files
            item_count = print_file_of_type(&MyFileType::HiddenFile,&arr_of_dirs_and_files, Color::Green,max_items,item_count,max_width);
            //Normal folders
            item_count = print_file_of_type(&MyFileType::NormalFolder,&arr_of_dirs_and_files, Color::BrightBlue,max_items,item_count,max_width);
            //Normal files
            print_file_of_type(&MyFileType::NormalFile,&arr_of_dirs_and_files, Color::BrightGreen,max_items,item_count,max_width);
        }
        Err(e) => {eprintln!("ERROR! : {}",e)}
    };
}

fn add_element_to_map(mut map: HashMap<MyFileType, Vec<String>>, value : &DirEntry, f_type: &MyFileType ) -> HashMap<MyFileType, Vec<String>> {
    match map.get_mut(f_type) {
        None => {}
        Some(value_ref) => {
            value_ref.push(value.file_name().clone().to_string_lossy().parse().unwrap())
        }
    }
    map
}


fn print_file_of_type(file_type:&MyFileType, map:& HashMap<MyFileType, Vec<String>> ,style: colored::Color, items_horizontally:i32, itemz:i32,max_length:i32) -> i32{

    let mut item:i32 =itemz;
    for x in map.get(file_type).unwrap() {
        if item == items_horizontally {
            item =1;
            print!("\n")
        }

        if item == items_horizontally-1 {
            print!("{}",x.color(style));
        }else {
            print!("{}",fill_string(max_length,x.clone()).color(style));
            print!("   ");
        }
        item +=1;
    }
    item
}fn calc_max_items(width:i32, max_length:i32, map:& HashMap<MyFileType, Vec<String>>) -> i32{
    let mut item_count:i64 =0;
    for x in map.clone() {
        item_count = item_count+ x.1.len() as i64
    }
    let ml_and_tabs = max_length + 3;
    let items_horizontally = width/(ml_and_tabs);
    items_horizontally
}

fn fill_string(max_length:i32, file_string:String) -> String{
    let mut appended_spaces: String = String::new();
    for _i in 0..max_length - file_string.chars().count() as i32 {
        appended_spaces.push(' ');
    }
    file_string.add(&*appended_spaces)
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum MyFileType{
    NormalFile,
    HiddenFile,
    NormalFolder,
    HiddenFolder
}
