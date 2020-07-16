use std::env;
use std::fs::File;
use std::io::Read;
use indoc::formatdoc;
use regex::Regex;
use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    let mut input = String::new();
    let mut output_file = String::new();

    // If this is true then the previous argument was the output flag
    let mut output_flag = false;
    for argument in  env::args().skip(1) {
        if argument == "-o" {
            output_flag = true;
            continue;
        }
        if output_flag {
            output_file = argument;
            output_flag = false;
        } else {
            input = argument;
        }
    }

    if input.len() < 1 {
        println!("Please specify a brainfuck file");
        std::process::exit(1);
    }

    let mut f = File::open(&input).expect("Error opening file");
    let mut brainfuck = String::new();
    f.read_to_string(&mut brainfuck).expect("Error reading to string");

    let mut ir = String::new();

    // Let's go over the stuffs
    let mut chars = brainfuck.chars();

    // Last instruction ID
    let mut id = 0;

    let mut bracket_queue = vec!();
    let mut branch_id = 0;

    loop {
        match chars.next() {
            Some('>') => {
                ir.push_str(
                    formatdoc!(
                    "
                        %{} = load i8*, i8** %sp
                        %{} = getelementptr inbounds i8, i8* %{}, i32 1
                        store i8* %{}, i8** %sp
                    ~"
                    , id+1, id+2, id+1, id+2).as_str()
                );
                id+=2;
            },
            Some('<') => {
                ir.push_str(
                    formatdoc!(
                    "
                        %{} = load i8*, i8** %sp
                        %{} = getelementptr inbounds i8, i8* %{}, i32 -1
                        store i8* %{}, i8** %sp
                    ~"
                    , id+1, id+2, id+1, id+2).as_str()
                );
                id+=2;
            },
            Some('+') => {
                ir.push_str(
                    formatdoc!(
                    "
                        %{} = load i8*, i8** %sp
                        %{} = load i8, i8* %{}
                        %{} = add i8 %{}, 1
                        store i8 %{}, i8* %{}
                    ~"
                    , id+1, id+2, id+1, id+3, id+2, id+3, id+1).as_str()
                );
                id+=3;
            },
            Some('-') => {
                ir.push_str(
                    formatdoc!(
                    "
                        %{} = load i8*, i8** %sp
                        %{} = load i8, i8* %{}
                        %{} = add i8 %{}, -1
                        store i8 %{}, i8* %{}
                    ~"
                    , id+1, id+2, id+1, id+3, id+2, id+3, id+1).as_str()
                );
                id+=3;
            },
            Some('.') => {
                ir.push_str(
                    formatdoc!(
                    "
                        %{} = load i8*, i8** %sp
                        %{} = load i8, i8* %{}
                        %{} = sext i8 %{} to i32
                        %{} = call i32 @putchar(i32 %{})
                    ~"
                    , id+1, id+2, id+1, id+3, id+2, id+4, id+3).as_str()
                );
                id+=4;
            },
            Some(',') => {
                ir.push_str(
                    formatdoc!(
                    "
                        %{} = load i8*, i8** %sp
                        %{} = call i32 @getchar()
                        %{} = trunc i32 %{} to i8
                        store i8 %{}, i8* %{}
                    ~"
                    , id+1, id+2, id+3, id+2, id+3, id+1).as_str()
                );
                id+=3;
            },
            Some('[') => {
                ir.push_str(
                    formatdoc!(
                    "
                        br label %l{}
                        
                    l{}:
                        %{} = load i8*, i8** %sp
                        %{} = load i8, i8* %{}
                        %{} = icmp eq i8 %{}, 0
                        br i1 %{}, label %l{}, label %l{}
                    
                    l{}:
                    "
                    , branch_id+1, branch_id+1, id+1, id+2, id+1, id+3, id+2, id+3, branch_id+3, branch_id+2, branch_id+2).as_str()
                );
                bracket_queue.push((branch_id+1,branch_id+3));
                id+=3;
                branch_id+=3;
            },
            Some(']') => {
                let (header, next) = bracket_queue.pop().expect("Found closing bracket before opening bracket");
                ir.push_str(
                    formatdoc!(
                    "
                        br label %l{}

                    l{}:
                    "
                    , header, next).as_str()
                );
            },
            None => break,
            _ => {},
        }
    }

    //I places some -'s in the snippets to force indoc to leave an indent, I remove them here
    ir = Regex::new("~").unwrap().replace_all(&ir, "").to_string();

    let output = formatdoc!(
        "
        declare i8* @calloc(i32, i32)
        declare void @free(i8*)
        declare i32 @putchar(i32)
        declare i32 @getchar()
        
        define i32 @main() {{
            %stack = call i8* @calloc(i32 1000, i32 1)
        
            %sp = alloca i8*
            store i8* %stack , i8** %sp
        
        {}
        
            call void @free(i8* %stack)
            ret i32 0
        }}
        
        "
    , ir);

    if output_file.len() < 1 {
        println!("{}", output);
    }
    
    // Now we compile this stuff
    else {
        let mut llc = Command::new("llc")
            .args(&["-o", "/tmp/test.s"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .ok()
            .expect("Failed to spawn llc process");

        llc.stdin.as_mut().unwrap().write_all(output.as_bytes()).unwrap();
        let assembly = llc.wait_with_output().unwrap();
        
        //Yet to figure out how to link llc stdout to gcc stdin

        let mut gcc = Command::new("gcc")
            .arg("/tmp/test.s")
            .args(&["-o", &output_file])
            //.arg("-x assembler -")
            .stdin(Stdio::piped())
            .spawn()
            .ok()
            .expect("Failed to spawn gcc process");

        //gcc.stdin.as_mut().unwrap().write_all(&assembly.stdout).unwrap();

    }

}
