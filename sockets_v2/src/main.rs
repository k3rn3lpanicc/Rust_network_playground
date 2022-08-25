use tokio::{net::{TcpListener,TcpStream}, io::{AsyncReadExt, AsyncWriteExt}, spawn};
use std::env::args;
use std::{thread,time::Duration};

const ADDRESS : &str = "127.0.0.1:1234";

#[tokio::main]
async fn main(){
    let delay = args().nth(1).unwrap_or_default().parse::<u64>().unwrap_or(0);
    println!("Delay : {}", delay);
    println!("Server starting on {}", ADDRESS);
    if let Ok(listener) = TcpListener::bind(ADDRESS).await {
        loop{
            println!("Waiting for connection...");
            let (stream,_) = listener.accept().await.unwrap();
            println!("New connection {}:{}" , 
                stream.local_addr().unwrap().ip(),
                stream.local_addr().unwrap().port()
            );
            spawn(async move {
                handle_connection(stream, delay).await;
            });
            println!("Hellloooo!");
            //handle_connection(_stream,delay).await; //handle the client async way
        }
    }
    else{
        println!("Error binding to {}", ADDRESS);
    }

}

async fn handle_connection (mut stream:TcpStream , delay : u64){
    //read the stream
    loop{
        let mut buf = [0; 1024];
        let len = stream.read(&mut buf).await.unwrap();
        if len>0{ //if we read something
            println!("size : {}" , len);
            let message = String::from_utf8_lossy(buf[..len].as_ref());
            println!("Received {}", message.to_owned().to_string());
            thread::sleep(Duration::from_millis(delay));
            stream.write_all(message.as_ref().as_bytes()).await.unwrap();
            println!("sent : {}", message.to_owned().to_string());
        }
        else{  //if the readed size is 0 it means that the client is disconnected
            println!("Connection closed");
            break;
        }
    }
}