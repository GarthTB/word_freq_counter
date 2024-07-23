use std::collections::HashSet;
use std::fs::File;
use std::io::{self, stdin, BufRead, BufReader, Write};
use std::path::Path;

use dashmap::DashMap;
use rayon::prelude::*;

fn read_input<T: std::str::FromStr>(prompt: &str) -> io::Result<T> {
    println!("{prompt}");
    loop {
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

fn count_combinations(
    file_path: &str,
    n: usize,
    extra_chars: &HashSet<char>,
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
                    window.push(c);
                    if window.len() == n {
                        *combinations.entry(window.iter().collect()).or_insert(0) += 1;
                        window.remove(0);
                    }
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
                    window.push(c);
                    if window.len() == n {
                        *combinations.entry(window.iter().collect()).or_insert(0) += 1;
                        window.remove(0);
                    }
                } else {
                    window.clear();
                }
            }
        });
    }

    Ok(combinations)
}

#[inline(always)]
fn analyze_word(
    n: usize,
    threshold: usize,
    combinations: &DashMap<String, usize>,
    words: &DashMap<String, usize>,
    window: &mut Vec<char>,
) {
    let mut max_word = String::with_capacity(n);
    let mut max_freq: usize = 0;

    for _ in 0..n {
        let word: String = window.iter().take(n).collect();
        let freq: usize = *combinations.get(&word).unwrap();
        if freq >= threshold && freq > max_freq {
            max_word = word;
            max_freq = freq;
        }
        window.remove(0);
    }

    if max_freq > 0 {
        *words.entry(max_word).or_insert(0) += 1;
    }
}

fn count_words(
    file_path: &str,
    n: usize,
    extra_chars: &HashSet<char>,
    threshold: usize,
    combinations: DashMap<String, usize>,
) -> io::Result<DashMap<String, usize>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let words: DashMap<String, usize> = DashMap::new();

    let range = 2 * n - 1;

    if extra_chars.is_empty() {
        lines.into_par_iter().for_each(|line| {
            let mut window: Vec<char> = Vec::with_capacity(range);
            for c in line.chars() {
                if c >= '一' && c <= '鿿' {
                    window.push(c);
                    if window.len() == range {
                        analyze_word(n, threshold, &combinations, &words, &mut window);
                    }
                } else {
                    window.clear();
                }
            }
        });
    } else {
        lines.into_par_iter().for_each(|line| {
            let mut window: Vec<char> = Vec::with_capacity(range);
            for c in line.chars() {
                if c >= '一' && c <= '鿿' || extra_chars.contains(&c) {
                    window.push(c);
                    if window.len() == range {
                        analyze_word(n, threshold, &combinations, &words, &mut window);
                    }
                } else {
                    window.clear();
                }
            }
        });
    }

    Ok(words)
}

fn sort_words(words: DashMap<String, usize>) -> Vec<(String, usize)> {
    let mut items: Vec<(String, usize)> = words
        .iter()
        .map(|entry| (entry.key().clone(), *entry.value()))
        .collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));
    items
}

fn write_results(file_path: &str, n: usize, sorted_words: Vec<(String, usize)>) -> io::Result<()> {
    let result_path = Path::new(file_path).with_file_name(format!("{n}字统计结果.txt"));
    let mut result_file = File::create(result_path)?;

    let lines: Vec<_> = sorted_words
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
        let extra_chars = remove_chinese(&read_input::<String>("请输入要纳入的非汉字（输入为一行）：")?);
        let threshold = get_threshold();
        println!("使用阈值：{threshold}\n逐字统计中...");
        let combinations = count_combinations(&file_path, n, &extra_chars)?;
        println!("逐字统计完成，进行盲分词统计...");
        let words = count_words(&file_path, n, &extra_chars, threshold, combinations)?;
        println!("盲分词统计完成，筛选中...");
        words.retain(|_, &mut value| value >= threshold);
        println!("筛选完成，输出中...");
        write_results(&file_path, n, sort_words(words))?;
        println!("输出完成，再来一次！\n");
    }
}
