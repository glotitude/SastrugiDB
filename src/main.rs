use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read, BufRead};

#[derive(Debug, Clone)]
pub enum Operator {
    Eq,
    Nq,
    Gt,
    Ge,
    Lt,
    Le,
}

impl Operator {
    pub fn to_string(&self) -> String {
        match self {
            Operator::Eq => "eq",
            Operator::Nq => "nq",
            Operator::Gt => "gt",
            Operator::Ge => "ge",
            Operator::Lt => "lt",
            Operator::Le => "le",
        }.to_string()
    }
}

#[derive(Debug, Clone, Transferable)]
pub struct Condition {
    pub field_name: String,
    pub operator: Operator,
    pub value: String,
}

struct Col {
    name: String,
    value: String,
}

struct Table {
    cols: Vec<Col>,
}

fn insert(lines: &Lines) {

}

fn filter() {

}

fn parse(reader: &mut BufReader<TcpStream>) {
    let mut lines = reader.by_ref().lines();
    let table = lines.next().unwrap().unwrap();
    let mut actions = Vec::new();

    for line in lines {
        let l = line.unwrap();
        if l.starts_with("#") {
            let a = match l.as_ref() {
                "#insert" => insert,
                "#filter" => filter,
                _ => panic!("wrong action"),
            };
            a();
            actions.push(l);
        }

        if l == String::from("") {
            break;
        }

        println!("{}", l);
    };
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    parse(&mut reader);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
