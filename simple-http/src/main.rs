use std::{
    io::{self, Write, Read},
    net::{
        TcpListener,
        TcpStream,
        SocketAddr,
        Ipv4Addr
    }
};

fn create_socket() -> SocketAddr{
    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST), 5500)
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()>{
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let valid_response = "HTTP/2 200\ncontent-type: text/html\nvary: Accept-Encoding\r\n\r\n\
    <html>
    <body>
    <h1>Hello World</h1>
    </body>
    </html>";
    println!("Http response\n{}", valid_response);

    stream.write(&mut valid_response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn serve(socket: SocketAddr) -> io::Result<()> {
    let mut counter = 0;
    let listener = TcpListener::bind(socket)?;
    for stream in listener.incoming(){
        match std::thread::spawn(|| handle_client(&mut stream?)).join() {
            Ok(_) => {
                counter += 1;
                println!("Connected stream.. {}", counter);
            },
            Err(_) => continue,
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let socket = create_socket();
    serve(socket)?;
    Ok(())
}