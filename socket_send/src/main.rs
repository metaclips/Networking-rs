use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;
use std::time;

fn main() {
    println!();
    print!("Pass 1 to start a server, else 2 for a client:\t");
    stdout().flush().unwrap();

    let mut server_option = String::new();
    let _ = stdin()
        .read_line(&mut server_option)
        .expect("could not get user input");

    println!();

    let server_option = server_option.strip_suffix("\n").unwrap();

    let options: u8 = server_option.parse().expect("could not part string to i8");

    match options {
        1 => start_server(),
        2 => start_client(),
        _ => panic!("Specify 1 if you want to start a server else 2 for a client."),
    }
}

fn start_server() {
    println!("Starting server started on.");

    // Bind TCP to localhost, free port is automatically generated.
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();

    let addr = listener.local_addr().unwrap();
    println!("Server started on {}", addr);

    loop {
        let (stream, addr) = listener.accept().expect("unable to get client");
        println!("Found a client. {}", addr);

        thread::spawn(move || {
            // Add a buffered stream to read packets from clients.
            let mut stream = BufReader::new(stream);

            loop {
                let mut msg = String::new();

                let _ = stream.read_line(&mut msg).unwrap();

                match msg.strip_suffix("\n") {
                    Some(msg) => {
                        println!("Message from: {}: {}", addr, msg);
                    }

                    None => {
                        // If a nil message is sent, it is assumed user as exited by calling ctrl-C.
                        println!("User {} exited", addr);
                        break;
                    }
                }
            }
        });
    }
}

fn start_client() {
    print!("Please specify address:port:\t");
    stdout().flush().unwrap();

    let mut addr = String::new();
    let _ = stdin()
        .read_line(&mut addr)
        .expect("could not get user input");

    println!();

    let addr = addr.strip_suffix("\n").unwrap();

    let addr = SocketAddr::from_str(&addr).expect("IP address specified is incorrect.");
    println!("Connecting client...");

    let mut stream = TcpStream::connect_timeout(&addr, time::Duration::new(10, 0)).unwrap();

    println!("Connected");

    loop {
        let mut msg = String::new();
        print!(">:\t");
        stdout().flush().unwrap();

        let _ = stdin()
            .read_line(&mut msg)
            .expect("could not get user input");

        let _ = stream.write_all(msg.as_bytes()).unwrap();
        // Flush out stream ensuring message was sent.
        let _ = stream.flush().unwrap();
    }
}
