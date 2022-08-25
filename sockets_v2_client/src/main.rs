use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};
mod decode;


const SERVER: &str = "127.0.0.1:1234";
#[tokio::main]
async fn main() {
    println!("Connecting to {}", SERVER);
    //use another module in rust!
    decode::print_message();
    
    if let Ok(mut stream)= TcpStream::connect(SERVER).await {
        println!("Connected! {}:{}" , 
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );
        let message = "Hello World!";
        let _ = stream.write_all(message.as_bytes()).await;
        println!("Sent: {}", message);
        let mut buffer = [0; 1024];
        let _ = stream.read(&mut buffer).await.unwrap();
        println!("Received: {}", String::from_utf8_lossy(&buffer));
        println!("Hey, I'm done!");
    }
    else{
        println!("Failed to connect to {}", SERVER);        
    }

}