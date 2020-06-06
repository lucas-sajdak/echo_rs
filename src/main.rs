use std::io::prelude::*;
use std::thread;
use std::net::{TcpListener, TcpStream};

fn process_client(mut client: TcpStream){
    println!("connected... {}", client.peer_addr().unwrap() );
    let mut rcv_buf = [0; 2048];
    loop {
        let received = client.read(&mut rcv_buf);
        match received {
            Ok(n) => {
                let sent = client.write_all(&rcv_buf[0 .. n]);
                match sent {
                    Ok(_) => {},
                    Err(_) => {
                        break;
                    },
                }
            },
            Err(_)=>{
                break;
            },
        }
    };
}

fn listen_for_clients() -> std::io::Result<()>{
    let listener = TcpListener::bind("localhost:8080")?;
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(||{
                    process_client(s );
                });
            },
            Err(_) => {},
        }
    }

    Ok(())
}

fn main(){
    let r = listen_for_clients();
    r.expect("Listen error")
} 