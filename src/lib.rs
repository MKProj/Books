use std::fs;
use std::process;
pub const DIR: &str = "MKProj-Books/";

pub struct Book <'a> {
    name: & 'a str,
    slug: & 'a str, 
    compl_sections: f32, 
    total_sections: f32
}

impl Book <'_>{
    pub fn status(self){
        println!("* {} | {} : {}%", self.name, self.slug, self.compl_sections / self.total_sections)
    }
}

pub fn init(){
    fs::create_dir(DIR);
}

pub fn Get(mut s: &str){
    let p = format!("{}-Basics", s);
    let url = format!("https://github.com/MKProj/{}", p);
    let path = format!(" {}{}", DIR, p);
    let clone = process::Command::new("git").arg("clone").arg(url).arg(path).spawn();
}  