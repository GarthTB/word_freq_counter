use std::collections::HashSet;
use std::fs::File;
use std::io::{self, stdin, BufRead, BufReader, Write};
use std::path::Path;

use dashmap::DashMap;
use rayon::prelude::*;

fn read_input<T: std::str::FromStr>(prompt: &str) -> io::Result<T> {
    loop {
        println!("{prompt}");
        let mut input = String::new();
        stdin().lock().read_line(&mut input)?;
        let input = input.trim();
        match input.parse::<T>() {
            Ok(value) => return Ok(value),
            _ => println!("输入无效！请重新输入。"),
        }
    }
}

fn get_threshold() -> usize {
    println!("请输入次数阈值，低于该数则忽略，默认为2：");
    let mut input = String::new();
    if stdin().read_line(&mut input).is_ok() {
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => num,
            _ => 2,
        }
    } else {
        2
    }
}

fn remove_chinese(input: &str) -> HashSet<char> {
    input
        .chars()
        .filter(|c| *c < '\u{4e00}' || *c > '\u{9fff}')
        .collect()
}

#[inline(always)]
fn accumulate(n: usize, combinations: &DashMap<String, usize>, window: &mut Vec<char>, c: char) {
    window.push(c);
    if window.len() == n {
        let key: String = window.iter().collect();
        *combinations.entry(key).or_insert(0) += 1;
        window.remove(0);
    }
}

fn count_combinations(
    file_path: &str,
    n: usize,
    extra_chars: HashSet<char>,
) -> io::Result<DashMap<String, usize>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let combinations: DashMap<String, usize> = DashMap::new();

    if extra_chars.is_empty() {
        lines.into_par_iter().for_each(|line| {
            let mut window: Vec<char> = Vec::with_capacity(n);
            for c in line.chars() {
                if c >= '一' && c <= '鿿' {
                    accumulate(n, &combinations, &mut window, c);
                } else {
                    window.clear();
                }
            }
        });
    } else {
        lines.into_par_iter().for_each(|line| {
            let mut window: Vec<char> = Vec::with_capacity(n);
            for c in line.chars() {
                if (c >= '一' && c <= '鿿') || extra_chars.contains(&c) {
                    accumulate(n, &combinations, &mut window, c);
                } else {
                    window.clear();
                }
            }
        });
    }

    Ok(combinations)
}

fn sort_combinations(combinations: DashMap<String, usize>) -> Vec<(String, usize)> {
    let mut sorted_combinations: Vec<(String, usize)> = combinations.into_iter().collect();
    sorted_combinations.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_combinations
}

fn write_results(
    file_path: &str,
    n: usize,
    sorted_combinations: Vec<(String, usize)>,
) -> io::Result<()> {
    let result_path = Path::new(file_path).with_file_name(format!("{n}字统计结果.txt"));
    let mut result_file = File::create(result_path)?;

    let lines: Vec<_> = sorted_combinations
        .into_iter()
        .map(|(combination, count)| format!("{combination}\t{count}\n"))
        .collect();
    result_file.write_all(lines.concat().as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    loop {
        let file_path = read_input::<String>("请输入文本文件路径：")?;
        let n = read_input::<usize>("请输入词长：")?;
        let extra_chars = remove_chinese(&read_input::<String>(
            "请输入要纳入的非汉字（输入为一行）：",
        )?);
        let threshold = get_threshold();
        println!("使用阈值：{threshold}\n统计中...");
        let combinations = count_combinations(&file_path, n, extra_chars)?;
        println!("统计完成，筛选中...");
        combinations.retain(|_, &mut value| value >= threshold);
        println!("筛选完成，排序中...");
        let sorted_combinations = sort_combinations(combinations);
        println!("排序完成，输出中...");
        write_results(&file_path, n, sorted_combinations)?;
        println!("输出完成，再来一轮！\n");
    }
}
