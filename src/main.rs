use rust_webserver::{HttpRequest, HttpRequestHandler, HttpResponse, Route, Router, ThreadPool};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
    time::Duration,
};

static THREAD_COUNT: usize = 10;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(THREAD_COUNT);
    let router = Arc::new(Router::new(create_routes()));

    for stream in listener.incoming().take(THREAD_COUNT) {
        let stream = stream.unwrap();
        let rt = Arc::clone(&router);

        pool.execute(move || {
            handle_connection(stream, rt);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = HttpRequest::from(String::from_utf8_lossy(&buffer[..]).to_string());

    if let Some(route) = router.get_handler(&request.path) {
        let response = route.handle(request);

        stream.write_all(&response.as_bytes()).unwrap();
        stream.flush().unwrap();

        return;
    }

    let response = HttpRequestHandler::new(request).handle();

    stream.write_all(&response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn create_routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.push(Route::new(
        "/sleep".to_string(),
        Box::new(|_| {
            thread::sleep(Duration::from_secs(5));
            HttpResponse::from("Hello, world!".to_string())
        }),
    ));

    routes
}
