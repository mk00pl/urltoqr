#![feature(exclusive_range_pattern)]
use std::env;
use std::process;
use std::path::Path;
use qrcode_generator::QrCodeEcc;

fn usage(name: String){
    let new_name = Path::new(&name).file_name().unwrap();
    println!("Usage: {:?} url \"output image\"",new_name);
    println!("if output file wont be specified,a qr");
    println!("code on terminal will appear instead.");
    process::exit(0x0100);
}

fn qr_code_terminal(url: String){
    qr2term::print_qr(url.clone()).unwrap()
}

fn qr_code_image(url: String,name: String){
    qrcode_generator::to_png_to_file(url.clone(), QrCodeEcc::Low,1024,name).unwrap();
    println!("qr image for {:?} generate successfully!",url);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => qr_code_terminal(args[1].clone()),
        3 => qr_code_image(args[1].clone(),args[2].clone()),
        _ => usage(args[0].clone()),
    }
}
