use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::env;
use std::sync::{Once, ONCE_INIT};
use std::option::Option;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::time::Instant;
use std::mem::forget;
use std::cmp::min;

const MAX_PROBE:usize = 6;

fn load_dict(path: &str) -> Option<HashMap<String, Vec<String>>> {
    let mut dict = HashMap::new();
    let mut content = String::new();

    File::open(path)
        .expect(&format!("{} failure", path))
        .read_to_string(&mut content).ok();
    
    content.lines().map(|x| {
        let vec = x.split("\t")
            .map(|x| x.replace(",", ""))
            .collect::<Vec<String>>();
        if vec[0].trim() != "" {
            dict.insert(vec[0].clone(), vec);
        }
    }).count();

    return Some(dict)
}

thread_local!(static DIGIT_NUM_DICT: HashSet<char> = ['十', '百', '千', '万'].iter().cloned().collect());
fn is_digit_and_number_on_thread_local(c: char) -> bool {
    if c.is_digit(36) {
        true
    } else {
        let b = DIGIT_NUM_DICT.with(|d|{d.contains(&c)});
        b
    }
}

static mut DIGIT_NUM_DICT_GLOBAL: Option<&'static HashSet<char>> = None;
static DIGIT_NUM_DICT_GLOBAL_SET: Once = ONCE_INIT;

fn digit_num_dict_global() -> &'static HashSet<char> {
    DIGIT_NUM_DICT_GLOBAL_SET.call_once(|| unsafe {
        //println!("DIGIT_NUM_DICT_GLOBAL_SET call...");
        DIGIT_NUM_DICT_GLOBAL = Some(leak(['十', '百', '千', '万'].iter().cloned().collect()))
    });
    unsafe {DIGIT_NUM_DICT_GLOBAL.unwrap()}
}

fn leak<T>(v: T) -> &'static T {
    unsafe {
        let b = Box::new(v);
        let p: *const T = &*b;
        forget(b); // leak our reference, so that `b` is never freed
        &*p
    }
 }

fn is_digit_and_number_on_global(c: char) -> bool {
    if c.is_digit(36) {
        true
    } else {
        let d = digit_num_dict_global();
        //println!("digit_num_dict_global address: {:?}", d as *const HashSet<char>);
        d.contains(&c)
    }
}

#[macro_use]
extern crate lazy_static;

fn is_digit_and_number_on_lazy_static(c: char) -> bool {
    lazy_static! {
        static ref DIGIT_NUM_DICT_LAZY_STATIC: HashSet<char> = {
            let r = ['十', '百', '千', '万'].iter().cloned().collect();
            r
        };
    }
    if c.is_digit(36) {
        true
    } else {
        DIGIT_NUM_DICT_LAZY_STATIC.contains(&c)
    }
}

fn is_digit_and_number(c: char) -> bool {
    //is_digit_and_number_on_thread_local(c)
    is_digit_and_number_on_global(c)
    //is_digit_and_number_on_lazy_static(c)
}

fn extract_digit_or_alpha(word_vec: &Vec<String>, start: usize) -> (String, usize) {
    let mut da: String = "".to_string();
    let mut idx = start;

    for word in word_vec[0..].iter() {
        if !is_digit_and_number(word.chars().nth(0).unwrap()) { break; }
        da += word;
        idx += 1;
    }
    (da, idx)
}

fn ws(wsdict: &HashMap<String, Vec<String>>, string: &str) -> Vec<String> {
    let mut wordlist: Vec<String> = vec![];
    let words = string.chars().into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let mut idx:usize = 0;
    let words_count = words.len();

    while idx < words_count {
        let end = min(words_count, idx+MAX_PROBE);
        let mut word_vec = &words[idx..end];

        if is_digit_and_number(word_vec[0].chars().nth(0).unwrap()) {
            let d = extract_digit_or_alpha(&word_vec.to_vec(), idx);
            wordlist.push(d.0);
            idx = d.1;
            continue
        }

        while word_vec.len() > 0 {
            let w = word_vec.concat();
            if wsdict.contains_key(&w) {
                wordlist.push(w);
                break;
            } else {
                word_vec = &word_vec[0..word_vec.len()-1]
            }
        }
        if word_vec.is_empty() {
            wordlist.push(words[idx].to_string());
            idx += 1;
        } else {
            idx += word_vec.len();
        }
    }
    wordlist
}

const NANOS_PER_MILLI: u32 = 1_000_000;
pub trait DurationMills {
    fn sec_and_millis(&self) -> String;
}

impl DurationMills for Duration {
    fn sec_and_millis(&self) -> String {
        let s = self.as_secs().to_string() + "." + &(self.subsec_nanos() / NANOS_PER_MILLI).to_string();
        s
    }
}

fn read_from_stdin(input: String, buf: &mut String) -> io::Result<()> {
    println!("{}", input);
    try!(io::stdin().read_line(buf));
    Ok(())
}

fn cut_word_from_file(dict: HashMap<String, Vec<String>>, file_name: &str, show: bool) {
    let mut content = String::new();
    let now = Instant::now();
    File::open(file_name)
        .expect(&format!("open data file failure"))
        .read_to_string(&mut content).ok();
    println!("open file time: {}s", now.elapsed().sec_and_millis()); 
    let mut line_count = 0;
    let mut total_text_count = 0;
    let mut total_word_count = 0;
    content.lines().map(|x| {
        let words = ws(&dict, x);
        /*
        if line_count == 153 {
            println!("{} {:#?}", x, words);
        }
        */
        total_text_count += x.bytes().count();
        total_word_count += words.len();
        line_count += 1;
        if show == true {
            println!("{:?}", words);
        }
    }).count();
    let elapsed = now.elapsed();
    let mut rate = 0;
    if elapsed.as_secs() != 0 {
        rate = (total_text_count as u64 / elapsed.as_secs())/1000;
    }
    println!("segment time: {}s", elapsed.sec_and_millis());
    println!("word text count: {} rate: {}KB/s", total_text_count, rate);
    println!("word count: {}", total_word_count);
}

fn main() {
    
    let dict = load_dict("../data/Freq/word.dict").unwrap();

    let mut arguments = Vec::new();
    for argument in env::args() {
        arguments.push(argument);
    }

    // process file
    if arguments.len() > 2 && arguments[1] == "-f" {
        let file_name = &arguments[2];
        cut_word_from_file(dict, file_name, false);
        return
    }

    // process cmd input

    //println!("{:#?} {}", dict, dict.iter().count());
    println!("{:#?}", ws(&dict, "我是中a国人123好吧！!"));
    //println!("{:#?}", ws(&dict, "OWL DL 支持那些需要最强表达能力的推理系统的用户，且这个推理系统能够保证计算的完全性（computational completeness，即所有的结论都能够保证被计算出来）和可判定性（decidability，即所有的计算都在有限的时间内完成）。它包括了OWL语言的所有成分，但有一定的限制，如类型的分离（一个类不能同时是一个个体或属性，一个属性不能同时是一个个体或类）。OWL DL 这么命名是因为它对应于[描述逻辑]，这是一个研究一阶逻辑的一个特定可判定片断的领域。OWL DL旨在支持已有的描述逻辑商业处理（business segment）和具有良好计算性质的推理系统。"));

    let mut buf = String::new();

    read_from_stdin("please input: ".to_string(), &mut buf);
    println!("{:#?}", buf);
    if buf.trim().len() > 1 {
        println!("{:#?}", ws(&dict, buf.as_str()));
    } else {
        cut_word_from_file(dict, "../data/news/news.sohunews.210806.txt.utf8", false);
    }

}
