use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct Inputs<'a> {
    attr: &'a str,
    input: &'a str,
}

pub fn do_work(attr: String, input: String) -> String {
    let port: u16 = String::from_utf8(std::fs::read(".rund").expect("Couldn't read .rund"))
        .expect("Couldn't parse .rund")
        .parse()
        .expect("Could get port from .rund");
    let before = Inputs {
        input: &input,
        attr: &attr,
    };
    let before_ser = serde_json::to_string(&before).unwrap();
    if let Ok(reply) = reqwest::blocking::Client::new()
        .post(format!("http://127.0.0.1:{port}/"))
        .body(before_ser)
        .timeout(Duration::from_secs(2))
        .send()
    {
        reply.text().expect("Couldn't parse reply from cargo-rund")
    } else {
        input
    }
}

#[cfg(test)]
mod test {
    use std::{
        sync::atomic::{AtomicBool, Ordering},
        thread,
        time::Duration,
    };

    use super::*;

    use actix_web::{App, HttpResponse, HttpServer, web};
    use quote::quote;

    fn write_port_to_file(port: u16) {
        std::fs::write(".rund", port.to_string()).unwrap()
    }

    fn input_stream() -> String {
        quote! {
            fn print_hello_world() { println!("Hello world")}
        }
        .to_string()
    }

    #[tokio::test]
    async fn test_connection() -> std::io::Result<()> {
        let mut connected = AtomicBool::new(false);
        let ptr = std::ptr::addr_of_mut!(connected) as usize;

        let echo = move |req: String| async move {
            unsafe {
                (ptr as *mut AtomicBool)
                    .as_ref()
                    .unwrap()
                    .store(true, std::sync::atomic::Ordering::SeqCst);
            }
            HttpResponse::Ok().body(req)
        };

        let mut server;
        let mut port = 8082;
        loop {
            server = HttpServer::new(move || App::new().route("/", web::post().to(echo)));
            match server.bind(("127.0.0.1", port)) {
                Ok(s) => {
                    server = s;
                    write_port_to_file(port);
                    break;
                }
                Err(_) => port += 1,
            }
        }

        //launch in thread, so that it doesn't use tokio, reqwest will
        // use tokio if it sees it
        thread::spawn(|| {
            do_work(String::new(), input_stream());
        });

        let server = server.run();

        let hndl = server.handle();
        let ptr = std::ptr::addr_of_mut!(connected) as usize;

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            unsafe {
                if (ptr as *mut AtomicBool)
                    .as_ref()
                    .unwrap()
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    hndl.stop(false).await
                }
            }
        });

        server.await?;

        assert!(connected.load(Ordering::SeqCst), "Failed connecting");
        Ok(())
    }
}
