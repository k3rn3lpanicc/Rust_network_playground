use std::io::{BufRead , BufReader , Write};
use std::net::{TcpListener , TcpStream};
use std::thread;


fn main() {
   let listener = TcpListener::bind("127.0.0.1:12321").unwrap();
   for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connected!");
        thread::spawn(move || handle_client(stream));
   }
}

fn send_message(stream:&BufReader<TcpStream> , message : &str) {
    let buf = format!("{}\n" , message);
    stream.get_ref().write(buf.as_bytes()).unwrap(); 
}

fn handle_client(stream : TcpStream) -> () {
    let mut stream  = BufReader::new(stream);
    loop{
        let mut buf = String::new();
        if stream.read_line(&mut buf).is_err() {
            println!("User disconnected!");
            break;
        }
        if buf=="" {
            println!("User disconnected!");
            break;
        }
        println!("{}" , buf);
        buf = format!("Server Back : {}" , buf);
        send_message(&stream , &buf);
        
    }
}
