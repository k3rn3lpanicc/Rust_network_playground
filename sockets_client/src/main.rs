use std::io::{BufReader,BufRead,Write};
use std::net::TcpStream;

fn formify(value: String) -> String { //Remove the \n from end of it
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str().to_owned()
}
fn read_line(reader : &mut BufReader<TcpStream> , st : &mut String){
    reader.read_line(st).unwrap();
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:12321").unwrap();
    println!("Connected to server!");
    let len = stream.write(b"Hello socket world!\n");
    println!("written : {:?}" , len);
    let mut reader = BufReader::new(stream);
    let mut line :String =  String::new();
    read_line(&mut reader , &mut line);
    println!("{:?}" , formify(line));
}
