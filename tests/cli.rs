use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Output, Stdio};

fn server() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_server"));
    command.env_remove("PORT").stdout(Stdio::piped()).stderr(Stdio::piped());

    command
}

fn read_until_url(server: &mut Child) -> Vec<String> {
    let stdout = server.stdout.take().expect("the server has a stdout");
    let mut read = Vec::new();

    for line in BufReader::new(stdout).lines() {
        let line = line.expect("the server writes text");
        let is_url = line.starts_with("listening on ");
        read.push(line);

        if is_url {
            server.kill().expect("the server stops");
            return read;
        }
    }

    panic!("the server stopped before it wrote its url, it wrote {read:?}");
}

fn run_until_exit(command: &mut Command) -> Output {
    command.spawn().expect("the server starts").wait_with_output().expect("the server exits")
}

fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0").expect("a free port exists").local_addr().expect("the port has an address").port()
}

#[test]
fn prints_a_clickable_url_of_the_port_in_port() {
    let port = free_port();

    let mut server = server().env("PORT", port.to_string()).spawn().expect("the server starts");

    assert_eq!(read_until_url(&mut server), vec![format!("listening on http://localhost:{port}")]);
}

#[test]
fn listens_on_the_next_free_port_when_port_8000_is_in_use() {
    let _other_server_on_8000 = TcpListener::bind("127.0.0.1:8000");

    let mut server = server().spawn().expect("the server starts");
    let read = read_until_url(&mut server);

    assert_eq!(read[0], "port [8000] is in use");

    let port: u16 = read
        .last()
        .and_then(|line| line.strip_prefix("listening on http://localhost:"))
        .and_then(|port| port.parse().ok())
        .expect("the last line is the url");

    assert!((8001..=8010).contains(&port));
}

#[test]
fn stops_when_the_port_in_port_is_in_use() {
    let other_server = TcpListener::bind("127.0.0.1:0").expect("a free port exists");
    let port = other_server.local_addr().expect("the port has an address").port();

    let output = run_until_exit(server().env("PORT", port.to_string()));

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8(output.stderr).expect("the server writes text"),
        format!("cannot listen on port [{port}], because it is in use\n"),
    );
}

#[test]
fn stops_when_port_is_not_a_number() {
    let output = run_until_exit(server().env("PORT", "eighty"));

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8(output.stderr).expect("the server writes text"),
        "the variable [PORT] must be a number from [0] to [65535], got [eighty]\n",
    );
}

fn get(port: u16, path: &str) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("the server accepts a connection");
    write!(stream, "GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n").expect("the server reads the request");

    let mut response = String::new();
    stream.read_to_string(&mut response).expect("the server writes text");

    response
}

#[test]
fn answers_a_path_that_does_not_exist_with_the_page_not_found() {
    let port = free_port();
    let mut server = server().env("PORT", port.to_string()).spawn().expect("the server starts");
    let stdout = server.stdout.take().expect("the server has a stdout");
    BufReader::new(stdout).lines().next().expect("the server writes its url").expect("the server writes text");

    let unknown_path = get(port, "/a-path-that-does-not-exist");
    let unknown_post = get(port, "/posts/a-post-that-does-not-exist");
    server.kill().expect("the server stops");
    server.wait().expect("the server exits");

    for response in [unknown_path, unknown_post] {
        assert!(response.starts_with("HTTP/1.1 404 Not Found"));
        assert!(response.contains("<title>nunomaduro · Page not found</title>"));
    }
}
