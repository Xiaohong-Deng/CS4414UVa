//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 1+
//
// Note that this code has serious security risks! You should not run it
// on any system with access to sensitive files.
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3
extern crate regex;
use regex::Regex;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use std::thread;
use std::fs::File;
use std::path::Path;

fn main() {
  let addr = "127.0.0.1:4414";
  // listener::Result<TcpListener>
  let listener = TcpListener::bind(addr).unwrap();
  static mut count: i32 = 0;

  println!("Listening on [{}] ...", addr);
  // listener.incoming() return an iterator over the connection being received on listener
  // stream is Result<TcpStream>
  for stream in listener.incoming() {
    unsafe {
      count += 1;
    }
    match stream {
      Err(_) => (),
      Ok(mut stream) => {
        // Spawn a thread to handle the connection
        // spwan takes a closure or a named function as argument
        // check Closure() for more details on move keyword
        // I guess move is for stream to be use inside the thread
        // double pipe marks is for closure, spawn accepts a named function
        // or closure as argument
        thread::spawn(move|| {
          // peer_addr() return Result<SocketAddr>
          // pn is address, IPv4 or IPv6
          match stream.peer_addr() {
            Err(_) => (),
            Ok(pn) => println!("Received connection from: [{}]", pn),
          }
          // a vec!, 500 zeros
          let mut buf = [0 ;500];
          stream.read(&mut buf).unwrap();
          // Result<&str, Utf8Error>
          match str::from_utf8(&buf) {
            Err(error) => println!("Received request error:\n{}", error),
            Ok(body) => { println!("Received request body:\n{}", body);
              // file path should start with 'whitespace/', end with whitespace
              let re1 = Regex::new(r"^GET.*\n").unwrap();
              let pos1 = re1.find(body).unwrap();
              let first_line = &body[pos1.0..pos1.1];
              let re2 = Regex::new(r"/(\w*(\.\w*)?)?").unwrap();
              let pos2 = re2.find(first_line).unwrap();
              let file_path_str = &first_line[pos2.0..pos2.1];
              match file_path_str.ends_with(".html") {
                true => { let file_path = Path::new(&file_path_str[1..]);
                  let file = File::open(&file_path);
                  match file {
                    Ok(mut file_cont) => { let mut bytes: Vec<u8> = Vec::new(); 
                      match file_cont.read_to_end(&mut bytes) {
                        Ok(_) => println!("file read ok!"),
                        Err(_) => println!("file read failed"),
                      }
                      let info = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
                      let mut info_bytes = info.to_string().into_bytes();
                      info_bytes.append(&mut bytes);
                      let byte_slice = &info_bytes[..];
                      stream.write(byte_slice).unwrap(); },
                    Err(_) => { let response = "HTTP/1.1 404 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                        <doctype !html><html><head><title>404 Error</title>
                        <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                        </style></head>
                        <body>
                        <h1>404 Not Found!</h1>
                        </body></html>\r\n";
                      stream.write(response.as_bytes()).unwrap(); },
                    } },
                false => { match file_path_str {
                  "/" => { unsafe {
                          stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                            <doctype !html><html><head><title>Hello, Rust!</title>
                            <style>body {{ background-color: #111; color: #FFEEAA }}
                            h1 {{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}}
                            h2 {{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}}
                            </style></head>
                            <body>
                            <h1>Greetings, Krusty!</h1>
                            number of visits: {}
                            </body></html>\r\n", count).as_bytes()).unwrap();
                            }
                          println!("Connection terminates."); },
                  _ => { let response = "HTTP/1.1 403 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                        <doctype !html><html><head><title>Some other things happened</title>
                        <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                        </style></head>
                        <body>
                        <h1>Target File is not Permitted!</h1>
                        </body></html>\r\n";
                        stream.write(response.as_bytes()).unwrap(); },
                } },
                }
              },
          };
        });
      },
    }
  }

  drop(listener);
}
