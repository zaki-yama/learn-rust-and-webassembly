use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn count(input: impl BufRead) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        // 4. その行を単語で分割する
        for m in re.find_iter(&line) {
            let word = m.as_str().to_string();
            // 5. 出現した単語の出現頻度を数える
            *freqs.entry(word).or_insert(0) += 1;
        }
    }
    freqs
}

fn main() {
    // 1. コマンドラインで指定された引数を読み込む
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    // 2. 指定されたファイルを開く
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    // 3. ファイルから1行ずつ読み込む
    let freqs = count(reader);
    println!("{:?}", freqs);
}
