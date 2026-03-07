use std::{
    net::{TcpListener, TcpStream },
    io::{BufReader, BufRead, Write, Error},
    fs
};

fn main() -> Result<(), Error> {
    // binding a server to an Ip address and a port
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    // making a for loop that waits for client, if he find one make a new stream
    for stream in listener.incoming(){
        handle_connection(stream?);
    }
    Ok(())
}
fn handle_connection(mut stream: TcpStream) {
    // store  request as a buffer(by reading it), if the request
    //sends an empty string, then end that buffer (the defualt behavior of a request)
    let buffer_read =  BufReader::new(&stream);
    let http_request: Vec<_> = buffer_read
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    dbg!(http_request);
    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("index.html").unwrap();
    let length = content.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    // send the response buffer to the steam, by using write method to overwrite the buffer
    stream.write(response.as_bytes()).unwrap();
}
#[cfg(test)]
mod test {}
