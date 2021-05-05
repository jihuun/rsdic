use std::env;
use std::io::Read;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_word = &args[1];
    let mut requst_url = String::from("http://small.dic.daum.net/search.do?q=");
    requst_url.push_str(input_word);
    
    let client = reqwest::blocking::Client::new();
    let mut res = client.get(&requst_url).send().unwrap();
    if res.status() != 200 {
        println!("Failed, Url:{}, Status code:{}", requst_url, res.status());
    }

    let mut body  = String::new();
    res.read_to_string(&mut body).unwrap();

    let document = Document::from(body.as_str());
    for node in document
        .find(Class("search_box")
        .descendant(Class("cleanword_type"))) {
        
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
