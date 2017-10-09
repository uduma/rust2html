use std::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = &args[1];
    let mut output = &args[2];

    if args.len() < 2 || args.len() > 3 {
        println!("Rust to HTML converter\n");
        println!("Usage: rust2Html <program>.rs [<file>.htm]\n");

    }



    let mut src = File::open(input.to_string()).expect("file not found");
    let mut dst = File::create(output.to_string()).expect("unable to create file");
    let mut title = input.to_string();

let mut style =String::from(" <style type = ".to_owned()+"text/css"+">body{font-family: Calligraffitti;font-size: 14px;text-shadow: 1px 1px 1px DimGrey;}</style>");
  
    let mut buf = "<html><head><title>".to_string() + &title + "</title>"+&style.to_string()+"</head>";
    dst.write(buf.as_bytes());
    dst.write_all(b"<body color='#000000' bgcolor='#FFFFFF'><pre><code>");

    let mut contents = String::new();
    src.read_to_string(&mut contents).expect("file not found");

    let v: Vec<&str> = contents.split(' ').collect();
   
    for line in v.iter() {
        if line.to_string() == String::from("fn") || line.to_string() == String::from("let") ||
            line.to_string() == String::from("mut") ||
            line.to_string() == String::from("for") ||
            line.to_string() == String::from("use") ||
            line.to_string() == String::from("match") ||
            line.to_string() == String::from("extern") ||
            line.to_string() == String::from("loop") ||
            line.to_string() == String::from("while") ||
            line.to_string() == String::from("true") ||
            line.to_string() == String::from("false") ||
            line.to_string() == String::from("break") ||
            line.to_string() == String::from("continue") ||
            line.to_string() == String::from("Ok") ||
            line.to_string() == String::from("Err") ||
            line.to_string() == String::from("impl") ||
            line.to_string() == String::from("struct") ||
            line.to_string() == String::from("str") ||
            line.to_string() == String::from("self") ||
            line.to_string() == String::from("pub") ||
            line.to_string() == String::from("if") ||
            line.to_string() == String::from("else")||
            line.to_string() == String::from("return")||
            line.to_string() == String::from("enum")||
            line.to_string() == String::from("i8")||
            line.to_string() == String::from("i16")||
            line.to_string() == String::from("i32")||
            line.to_string() == String::from("i64")||
            line.to_string() == String::from("u8")||
            line.to_string() == String::from("u16")||
            line.to_string() == String::from("u32")||
            line.to_string() == String::from("u64")||
            line.to_string() == String::from("isize")||
            line.to_string() == String::from("usize")||
            line.to_string() == String::from("f32")||
            line.to_string() == String::from("f64")||
            line.to_string() == String::from("bool")||
            line.to_string() == String::from("char")||
            line.to_string() == String::from("ref")||
            line.to_string() == String::from("Some")||
            line.to_string() == String::from("None")||
            line.to_string() == String::from("Option")||
            line.to_string() == String::from("Result")||
            line.to_string() == String::from("Drop")||
            line.to_string() == String::from("trait")||
            line.to_string() == String::from("static")||
            line.to_string() == String::from("unsafe")||
            line.to_string() == String::from("as")||
            line.to_string() == String::from("type")                           
        {
            let mut buf3 = "<font color=#1A237E>".to_string() + &line.to_string() + "  " +
                "</font>";
            dst.write(buf3.as_bytes());

        } else if line.to_string() != String::from("fn") ||
                   line.to_string() != String::from("let") ||
                   line.to_string() != String::from("mut") ||
                   line.to_string() != String::from("for") ||
                   line.to_string() != String::from("use") ||
                   line.to_string() != String::from("match") ||
                   line.to_string() != String::from("extern") ||
                   line.to_string() != String::from("loop") ||
                   line.to_string() != String::from("while") ||
                   line.to_string() != String::from("true") ||
                   line.to_string() != String::from("false") ||
                   line.to_string() != String::from("break") ||
                   line.to_string() != String::from("continue")||
                   line.to_string() != String::from("Ok") ||
            line.to_string() != String::from("Err") ||
            line.to_string() != String::from("impl") ||
            line.to_string() != String::from("struct") ||
            line.to_string() != String::from("str") ||
            line.to_string() != String::from("self") ||
            line.to_string() != String::from("pub") ||
            line.to_string() != String::from("if") ||
            line.to_string() != String::from("else")||
            line.to_string() != String::from("return")||
            line.to_string() != String::from("enum")||
            line.to_string() != String::from("i8")||
            line.to_string() != String::from("i16")||
            line.to_string() != String::from("i32")||
            line.to_string() != String::from("i64")||
            line.to_string() != String::from("u8")||
            line.to_string() != String::from("u16")||
            line.to_string() != String::from("u32")||
            line.to_string() != String::from("u64")||
            line.to_string() != String::from("isize")||
            line.to_string() != String::from("usize")||
            line.to_string() != String::from("f32")||
            line.to_string() != String::from("f64")||
            line.to_string() != String::from("bool")||
            line.to_string() != String::from("char")||
            line.to_string() != String::from("ref")||
            line.to_string() != String::from("Some")||
            line.to_string() != String::from("None")||
            line.to_string() != String::from("Option")||
            line.to_string() != String::from("Result")||
            line.to_string() != String::from("Drop")||
            line.to_string() != String::from("trait")||
            line.to_string() != String::from("static")||
            line.to_string() != String::from("unsafe")||
            line.to_string() != String::from("as")||
            line.to_string() != String::from("type")                           
        
        {
            let mut l = line.to_string() + &String::from("  ");
            dst.write(l.as_bytes());
        }

    }



    dst.write(b"</code></pre></body></html>");


}
