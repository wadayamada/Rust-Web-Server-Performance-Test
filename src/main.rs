use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// メインとなる関数
fn main() -> std::io::Result<()> {
    // 127.0.0.1:7878 で待ち受けるリスナーを生成
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    println!("Server listening on http://127.0.0.1:7878/");

    // 接続が来るたびに handle_connection をスレッドで呼び出す
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Failed to handle connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

/// クライアントとの通信を処理する関数
fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    // リクエストを格納するバッファ
    let mut buffer = [0; 1024];
    // リクエストを読み出す
    let bytes_read = stream.read(&mut buffer)?;

    // 何かしらリクエストがある場合のみ処理する
    if bytes_read > 0 {
        // 簡単に HTTP のレスポンスを返す
        let response = "HTTP/1.1 200 OK\r\n\
                        Content-Type: text/plain; charset=UTF-8\r\n\
                        Content-Length: 16\r\n\
                        Connection: close\r\n\
                        \r\n\
                        Hello from Rust!";

        stream.write_all(response.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}
