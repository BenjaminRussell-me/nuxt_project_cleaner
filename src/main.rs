use std::io;
use std::fs::{self, ReadDir, DirEntry};
use std::path::Path;
use colored::*;
use dialoguer::Confirm;
use walkdir::WalkDir;

fn content() -> Result<String, io::Error> {
    fs::read_to_string("src/store/content.js")
}

fn visit_dirs<'a>() -> io::Result<Vec<String>>{
    let mut files = Vec::new();
    for entry in WalkDir::new("src/img").min_depth(1) {
        let entry = entry?;
        let path = entry.path();
         if path.is_dir() {
         } else {
            files.push(entry);
         }
    }
    let mut processed = Vec::new();
    for file in files {
       let path = file.path().to_str().unwrap();
       let path = path.to_owned();
        processed.push(String::from(path));
    }

    println!("{:?}", processed);
    Ok(processed)
}

fn main() {
    let store;
    let mut pictures = Vec::new();

     match content() {
        Ok(content) => store = content,
        Err(error) => panic!("couldn't read content: {:?}", error)
    };
   match visit_dirs(){
    Ok(good) => pictures = good,
    Err(error) => panic!("can't read files{}", error),
   }

    let compair_list = compair(store, pictures);

    match confirm(&compair_list){
       Ok(good) => {println!("{:?}", good);

       },
       Err(error) => println!("{}",error)
    }
    remove(&compair_list)
}

fn compair(store:  String, pictures:  Vec<String>) ->  Vec<String> { // not just getting name so never finds it in file
    let mut the_chosen = Vec::new();
for pic in pictures {
    let pic_string = pic;
    if store.contains(&pic_string) {
        println!("found pic: {}", pic_string);
    } else {
        println!("pic not found: {}", pic_string);
        the_chosen.push(pic_string);
    }
}
    the_chosen
}
fn confirm (chosen: &Vec<String>) -> std::io::Result<()> {
    println!("{}", "deleting these items".red());
    for item in chosen {
    let pic_string = item;
        println!("{}", pic_string.red());
    }
    if Confirm::new().with_prompt("Do you wish to continue?").interact()?{
    } else {
        panic!("exiting")
    }
 Ok(())
}
fn remove (chosen: &Vec<String>) {
    for file in chosen {
        let mut path = String::from("src/img/");
        path.push_str(&file);
        println!("removing: {}", path);
        match fs::remove_file(path) {
            Ok(g) => println!("{:?}",g),
            Err(e) => println!("can't delete file{}",e)
        }
    }
}
