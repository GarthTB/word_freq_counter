use std::collections::HashSet;
use std::io;
use std::path::Path;

fn get_filepath(prompt: &str) -> String {
    println!("{prompt}");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        let filepath = input.trim().to_string();
        if Path::new(&filepath).exists() {
            return filepath;
        } else {
            println!("文件不存在，请重新输入！");
        }
    }
}

fn get_input_with_default(prompt: &str, allow_zero: bool, default: usize) -> usize {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    if let Ok(value) = input.trim().parse() {
        if value > 0 || (allow_zero && value == 0) {
            return value;
        }
    }
    println!("已使用默认值：{default}");
    return default;
}

fn get_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
}

fn get_extra(input: &str) -> HashSet<char> {
    input.chars().filter(|c| *c < '一' || *c > '鿿').collect()
}

pub fn get() -> (String, usize, usize, HashSet<char>) {
    let file_path = get_filepath("请输入文本文件路径：");
    let n = get_input_with_default("请输入词长：", false, 2);
    let threshold = get_input_with_default("请输入过滤次数，不超过该数则忽略：", true, 1);
    let extra_chars = get_extra(&get_input("请输入要纳入的非汉字（输入为一行）："));
    (file_path, n, threshold, extra_chars)
}
