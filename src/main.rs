use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    process::Command,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3334").unwrap();

    for stream in listener.incoming() {
        let command = handle_stream(stream.unwrap());

        println!("received command: {command}");

        let split: Vec<&str> = command.split(' ').collect();

        if let Some((command, args)) = split.split_first() {
            Command::new(command).args(args).spawn().ok();
        }
    }
}

fn handle_stream(mut stream: TcpStream) -> String {
    let mut buffer = [0; 1024];
    let read = stream.read(&mut buffer).unwrap();

    String::from_utf8_lossy(&buffer[..read]).to_string()
}
