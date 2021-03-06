use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::env;

fn main(){
    let exe_arg = env::args().collect::<Vec<String>>();
    if exe_arg.len() < 2 {
      panic!("スクリプトのパスを指定して下さい 例) >AhkViewer foo.ahk");
    }
    let path_str = &exe_arg[1];
    let path = Path::new(path_str);

    let mut file = match File::open(&path) {
        Err(why) => panic!("open err: {}", why),
        Ok(file) => file,
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Err(why) => panic!("read err: {}",why),
        Ok(_) => {},
    }

    println!(" ➖ {} ➖",get_file_name(path_str));
    let layout = set_layout(file_content.replace(" ",""));

    // キーボードレイアウトを行に分解
    let keyboard_lines = make_keyboard_lines(|key| {
        match key {
            '\\' | '[' | ']' => true,
            _ => false
        }
    });

    // レイアウトをインデントしながら回す
    let mut indent = 0;
    for line in keyboard_lines {
        for _ in 0..indent {
            print!(" ");
        }

        print_layout_line(& layout, & line);
        if indent < 3 {
            println!("");
            indent += 1;
        }
    }
}

fn to_char_upper(from:String) -> char {
    from.chars().collect::<Vec<char>>()[0].to_uppercase().collect::<Vec<char>>()[0]
}

fn get_file_name(file_path:&String) -> String{
    let mut file_name = String::new();
    for c in file_path.chars() {
      if file_name.contains(r"\") || file_name.contains("/") {
          file_name = String::new();
      }
      file_name += &*c.to_string();
    }
    file_name
}

fn is_target_line(line:String) -> bool {
    let line_len = line.chars().count();
    //コロン、セミコロン等の特殊例に対応、vkBAsc028はコロンのキーコード
    if (line != "" && line.contains("::") && line_len <= 5) ||
    (line.contains("`;") && line_len <= 6) ||
    (line.contains("vkBAsc028") && line_len <= 13)
    {
        true
    }else{
        false
    }
}

fn set_layout(file_content:String) -> HashMap<char,char>{
    let mut layout :HashMap<char,char> = HashMap::new();
    let default_layout = r"1234567890-^\QWERTYUIOP@[ASDFGHJKL;:]ZXCVBNM,./\".to_string();
    for c in default_layout.chars() {
        layout.insert(c,c);
    }

    let lines:Vec<&str> = file_content.split("\n").collect();
    let mut before_key_buff = ' ';
    let mut after_key_buff = ' ';
    let mut splited_buff:Vec<&str> = Vec::new();
    for line in lines {
        if is_target_line(line.to_string()) {
            splited_buff = line.split("::").collect::<Vec<&str>>();

            //スクリプトだと、セミコロンは`;、コロンはvkBAsc028と記述される
            before_key_buff = to_char_upper(splited_buff[0].replace("`","").replace("vkBAsc028",":"));
            after_key_buff = to_char_upper(splited_buff[1].replace("`","").replace("vkBAsc028",":"));

            if layout.contains_key(&to_char_upper(before_key_buff.to_string())){
                layout.insert(
                    before_key_buff,
                    after_key_buff
                );
            }
            splited_buff = Vec::new();
        }
    };
    layout
}

// 1文字ごとに文字を変換しながら出力する
fn print_layout_line(layout: &HashMap<char, char>, line: &String) {
    for key in line.chars() {
        match layout.get(&key) {
            Some(after_key) => {
                print!(" {} ", after_key);
            },
            None => {
                println!("null")
            }
        }
    }
}

// キーボードレイアウトを与えられたクロージャによって行単位に分割する
fn make_keyboard_lines<C>(closure: C) -> Vec<String> where C: Fn(char) -> bool {
    let default_layout = r"1234567890-^\QWERTYUIOP@[ASDFGHJKL;:]ZXCVBNM,./\";

    let mut vec = vec![];
    let mut s = "".to_string();
    for c in default_layout.chars() {
        s.push(c);
        if closure(c) {
            vec.push(s);
            s = "".to_string();
        }
    }
    vec.push(s);
    vec
}
