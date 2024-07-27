use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use dashmap::DashMap;
use rayon::prelude::*;

const UTF_LOWER_LIMIT: char = '䷿';
const UTF_UPPER_LIMIT: char = 'ꀀ';

pub fn count_groups(
    file_path: &str,
    n: usize,
    extra_chars: &HashSet<char>,
) -> DashMap<String, usize> {
    let file = File::open(file_path).expect("读取文件失败");
    let reader = BufReader::new(file);

    let groups: DashMap<String, usize> = DashMap::with_capacity(131072);

    if extra_chars.is_empty() {
        reader.lines().par_bridge().for_each(|line| {
            let mut window: VecDeque<char> = VecDeque::with_capacity(n);
            for c in line.expect("读取文件的行失败").chars() {
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
        reader.lines().par_bridge().for_each(|line| {
            let mut window: VecDeque<char> = VecDeque::with_capacity(n);
            for c in line.expect("读取文件的行失败").chars() {
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

    groups
}

pub fn count_words(
    file_path: &str,
    n: usize,
    extra_chars: &HashSet<char>,
    groups: DashMap<String, usize>,
) -> DashMap<String, usize> {
    let file = File::open(file_path).expect("读取文件失败");
    let reader = BufReader::new(file);

    let words: DashMap<String, usize> = DashMap::with_capacity(65536);

    let range = 2 * n - 1;

    if extra_chars.is_empty() {
        reader.lines().par_bridge().for_each(|line| {
            let mut window: VecDeque<char> = VecDeque::with_capacity(range);
            let mut max_word: String = String::new();
            let mut max_freq: usize;
            let mut word: String;

            for c in line.expect("读取文件的行失败").chars() {
                if c > UTF_LOWER_LIMIT && c < UTF_UPPER_LIMIT {
                    window.push_back(c);
                    if window.len() == range {
                        max_freq = 0;

                        for _ in 0..n {
                            word = window.iter().take(n).collect();
                            window.pop_front();
                            let freq = groups.get(&word).expect("找不到逐字统计的结果");
                            if *freq > max_freq {
                                max_word = word;
                                max_freq = *freq;
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
        reader.lines().par_bridge().for_each(|line| {
            let mut window: VecDeque<char> = VecDeque::with_capacity(range);
            let mut max_word: String = String::new();
            let mut max_freq: usize;
            let mut word: String;

            for c in line.expect("读取文件的行失败").chars() {
                if c > UTF_LOWER_LIMIT && c < UTF_UPPER_LIMIT || extra_chars.contains(&c) {
                    window.push_back(c);
                    if window.len() == range {
                        max_freq = 0;

                        for _ in 0..n {
                            word = window.iter().take(n).collect();
                            window.pop_front();
                            let freq = groups.get(&word).expect("找不到逐字统计的结果");
                            if *freq > max_freq {
                                max_word = word;
                                max_freq = *freq;
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

    words
}
