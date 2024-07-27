use std::fs::File;
use std::io::Write;
use std::path::Path;

use dashmap::DashMap;

mod counter;
mod settings;

fn sort_words(words: DashMap<String, usize>) -> Vec<(String, usize)> {
    let mut items: Vec<(String, usize)> = words
        .iter()
        .map(|entry| (entry.key().clone(), *entry.value()))
        .collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));
    items
}

fn write_results(file_path: &str, n: usize, sorted_words: Vec<(String, usize)>) {
    let lines: Vec<_> = sorted_words
        .into_iter()
        .map(|(combination, count)| format!("{combination}\t{count}\n"))
        .collect();

    let result_path = Path::new(file_path).with_file_name(format!("{n}字统计结果.txt"));

    File::create(result_path)
        .expect("结果文件创建失败")
        .write_all(lines.concat().as_bytes())
        .expect("结果文件写入失败");
}

fn main() {
    loop {
        let (file_path, n, threshold, extra_chars) = settings::get();
        println!("逐字统计中...");
        let groups = counter::count_groups(&file_path, n, &extra_chars);
        println!("逐字统计完成，筛选中...");
        groups.retain(|_, &mut value| value > threshold);
        println!("筛选完成，进行盲分词统计...");
        let words = counter::count_words(&file_path, n, &extra_chars, threshold, groups);
        println!("盲分词统计完成，筛选中...");
        words.retain(|_, &mut value| value > threshold);
        println!("筛选完成，输出中...");
        write_results(&file_path, n, sort_words(words));
        println!("输出完成，再来一次！\n");
    }
}
