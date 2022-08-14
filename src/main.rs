use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_word = &args[1];
    let mut request_url = String::from("http://small.dic.daum.net/search.do?q=");
    request_url.push_str(input_word);

    let body: String = reqwest::blocking::get(&request_url)
        .unwrap()
        .text()
        .unwrap();

    let document = Document::from(body.as_str());
    for node in document.find(Class("search_box").descendant(Class("cleanword_type"))) {
        let title = node.find(Class("txt_emph1")).next().unwrap().text();
        println!("[ {} ]", title);
        /* TODO: save the means to some datatype */
        for t in node.find(Name("li")) {
            let num = t.find(Class("num_search")).next().unwrap().text();
            let mean = t.find(Name("daum:word")).next().unwrap().text();
            println!("{} {}", num, mean);
        }
    }
}
