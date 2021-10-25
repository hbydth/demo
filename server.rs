use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();//TcpListener 用于监听 TCP 连接, bind 函数类似于 new 函数，在这里它返回一个新的 TcpListener 实例

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);//server处理func
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];  //缓冲区

    stream.read(&mut buffer).unwrap();//读取流

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));//打印流
}
