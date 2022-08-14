use std::env;
use std::result::Result;
use select::document::Document;
use select::node::Node;
//use select::predicate::{Attr, Class, Name, Predicate};
use select::predicate::{Class, Name, Predicate};

enum Attrs {
    Name,
    Class,
}

fn find_object(dom: &Node, attr: Attrs, s: &str) -> Option<String> {
    let text: String;
    match attr {
        Attrs::Name => {
            text = dom.find(Name(s)).next().unwrap().text();
        }
        Attrs::Class => {
            text = dom.find(Class(s)).next().unwrap().text();
        }
    }
    return Some(text);
}

fn document(body: String) -> Result<bool, String> {
    let document = Document::from(body.as_str());
    for node in document
        .find(Class("search_box")
        .descendant(Class("cleanword_type"))) {
        
        //let title = node.find(Class("txt_emph1")).next().unwrap().text();
        match find_object(&node, Attrs::Class, "txt_emph1") {
            Some(title) => println!("[ {} ]", title),
            None => {
                return Err("Err: Could not find \"txt_emph1\" Class in DOM".to_string());
            }
        }
        /* TODO: save the means to some datatype */
        for t in node.find(Name("li")) {
            //let num = t.find(Class("num_search")).next().unwrap().text();
            let num = match find_object(&t, Attrs::Class, "num_search") {
                Some(number) => number,
                None => {
                    return Err("Err: Could not find \"num_search\" Class in DOM".to_string());
                }
            };
            //let mean = t.find(Name("daum:word")).next().unwrap().text();
            let mean = match find_object(&t, Attrs::Name, "daum:word") {
                Some(meaning) => meaning,
                None => {
                    return Err("Err: Could not find \"daum:word\" Name in DOM".to_string());
                }
            };
            println!("{} {}", num, mean);
        }
    }
    Ok(true)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_word = &args[1];
    let mut request_url = String::from("http://small.dic.daum.net/search.do?q=");
    request_url.push_str(input_word);

    let body: String = reqwest::blocking::get(&request_url)
        .unwrap()
        .text()
        .unwrap();

    document(body).unwrap();
}
