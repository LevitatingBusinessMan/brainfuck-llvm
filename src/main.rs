use std::env;
use std::fs::File;
use std::io::Read;
use indoc::{formatdoc};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() == 0 {
        println!("Please specify a brainfuck file");
        std::process::exit(1);
    }

    let mut f = File::open(&args[0]).expect("Error opening file");
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

    println!("{}", output);

}
