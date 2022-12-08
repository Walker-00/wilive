use opencv::{
    imgcodecs,
    prelude::*,
    videoio::{self, VideoCapture},
};
use std::{env, fs::File, io::ErrorKind, process::Command};
use temporary::Directory;
extern crate opencv;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not Enought Args!\nUsage: \n     wilive [FullFilePath without [~, $USER, .., .]] [loop, no]");
        std::process::exit(1);
    } else if args[2] != String::from("loop") && args[2] != String::from("no") {
        eprintln!(
            "Unknown Option!\nloop        loop forever!\nno          end when video/gif is end!"
        );
        std::process::exit(1);
    }
    check_file(args.clone()).expect("File Checking Error due to: ");
    run(args.clone(), args[2].clone());
}

fn check_file(args: Vec<String>) -> Result<bool, String> {
    let file_check = File::open(&args[1]);
    match file_check {
        Ok(_w) => Ok(true),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(String::from("File Not Found!")),
            _other_error => Err(String::from("Unknown File Reading Error!")),
        },
    }
}
fn run(file: Vec<String>, lop: String) {
    let mut wed = true;
    let mut vid = VideoCapture::from_file(&file[1], videoio::CAP_ANY)
        .expect("VideoCapturing From File Error!");
    let mut i = 0;
    let dir = Directory::new("frame").unwrap();
    let mut nlop = true;

    let mut img = Mat::default();
    while wed {
        let read_frame =
            VideoCapture::read(&mut vid, &mut img).expect("Opencv Frame Reading Error!");
        wed = read_frame;
        if wed {
            let mut name = dir
                .join(&i.to_string())
                .into_os_string()
                .into_string()
                .expect("Unknown Error");
            name.push_str(".png");
            if nlop {
                imgcodecs::imwrite(&name, &img, &opencv::core::Vector::default()).unwrap();
            }
            Command::new("feh")
                .arg("--bg-scale")
                .arg(name)
                .spawn()
                .unwrap();
            i = i + 1;
        } else {
            if lop == String::from("loop") {
                nlop = false;
                i = 0;
                wed = true;
                vid = VideoCapture::from_file(&file[1], videoio::CAP_ANY).unwrap();
            } else if lop == String::from("no") {
                std::process::exit(1);
            }
        }
    }
    videoio::VideoCapture::release(&mut vid).unwrap();
}
