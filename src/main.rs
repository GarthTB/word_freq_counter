use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, stdin, BufRead, BufReader, Write};
use std::path::Path;

use dashmap::DashMap;
use rayon::prelude::*;

const UTF_LOWER_LIMIT: char = '䷿';
const UTF_UPPER_LIMIT: char = 'ꀀ';

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
    println!("请输入过滤次数，不超过该数则忽略，默认为1：");
    let mut input = String::new();
    if stdin().read_line(&mut input).is_ok() {
        match input.trim().parse::<usize>() {
            Ok(num) => num,
            _ => 1,
        }
    } else {
        1
    }
}

fn remove_chinese(input: &str) -> HashSet<char> {
    input
        .chars()
        .filter(|c| *c <= UTF_LOWER_LIMIT || *c >= UTF_UPPER_LIMIT)
        .collect()
}

fn count_groups(
    file_path: &str,
    n: usize,
    extra_chars: &HashSet<char>,
) -> io::Result<DashMap<String, usize>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let groups: DashMap<String, usize> = DashMap::with_capacity(131072);

    if extra_chars.is_empty() {
        lines.into_par_iter().for_each(|line| {
            let mut window: VecDeque<char> = VecDeque::with_capacity(n);
            for c in line.chars() {
                if c > UTF_LOWER_LIMIT && c < UTF_UPPER_LIMIT {
                    window.push_back(c);
                    if window.len() == n {
                        *groups.entry(window.iter().collect()).or_insert(0) += 1;
                        window.pop_front();
                    }
                } else {
                    window.clear();
                }
            }
        });
    } else {
        lines.into_par_iter().for_each(|line| {
            let mut window: VecDeque<char> = VecDeque::with_capacity(n);
            for c in line.chars() {
                if c > UTF_LOWER_LIMIT && c < UTF_UPPER_LIMIT || extra_chars.contains(&c) {
                    window.push_back(c);
                    if window.len() == n {
                        *groups.entry(window.iter().collect()).or_insert(0) += 1;
                        window.pop_front();
                    }
                } else {
                    window.clear();
                }
            }
        });
    }

    Ok(groups)
}

fn count_words(
    file_path: &str,
    n: usize,
    extra_chars: &HashSet<char>,
    threshold: usize,
    groups: DashMap<String, usize>,
) -> io::Result<DashMap<String, usize>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let words: DashMap<String, usize> = DashMap::with_capacity(65536);

    let range = 2 * n - 1;

    if extra_chars.is_empty() {
        lines.into_par_iter().for_each(|line| {
            let mut window: VecDeque<char> = VecDeque::with_capacity(range);
            let mut max_word: String = String::new();
            let mut max_freq: usize;
            let mut word: String;

            for c in line.chars() {
                if c > UTF_LOWER_LIMIT && c < UTF_UPPER_LIMIT {
                    window.push_back(c);
                    if window.len() == range {
                        max_freq = 0;

                        for _ in 0..n {
                            word = window.iter().take(n).collect();
                            window.pop_front();
                            if let Some(freq) = groups.get(&word) {
                                if *freq > max_freq && *freq > threshold {
                                    max_word = word;
                                    max_freq = *freq;
                                }
                            }
                        }

                        if max_freq > 0 {
                            *words.entry(max_word.clone()).or_insert(0) += 1;
                        }
                    }
                } else {
                    window.clear();
                }
            }
        });
    } else {
        lines.into_par_iter().for_each(|line| {
            let mut window: VecDeque<char> = VecDeque::with_capacity(range);
            let mut max_word: String = String::new();
            let mut max_freq: usize;
            let mut word: String;

            for c in line.chars() {
                if c > UTF_LOWER_LIMIT && c < UTF_UPPER_LIMIT || extra_chars.contains(&c) {
                    window.push_back(c);
                    if window.len() == range {
                        max_freq = 0;

                        for _ in 0..n {
                            word = window.iter().take(n).collect();
                            window.pop_front();
                            if let Some(freq) = groups.get(&word) {
                                if *freq > max_freq && *freq > threshold {
                                    max_word = word;
                                    max_freq = *freq;
                                }
                            }
                        }

                        if max_freq > 0 {
                            *words.entry(max_word.clone()).or_insert(0) += 1;
                        }
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
        println!("过滤次数：{threshold}\n逐字统计中...");
        let groups = count_groups(&file_path, n, &extra_chars)?;
        println!("逐字统计完成，筛选中...");
        groups.retain(|_, &mut value| value > threshold);
        println!("筛选完成，进行盲分词统计...");
        let words = count_words(&file_path, n, &extra_chars, threshold, groups)?;
        println!("盲分词统计完成，筛选中...");
        words.retain(|_, &mut value| value > threshold);
        println!("筛选完成，输出中...");
        write_results(&file_path, n, sort_words(words))?;
        println!("输出完成，再来一次！\n");
    }
}
