//! Application may have multiple data objects that are shared across
//! all handlers within same Application.
//!
//! For global shared state, we wrap our state in a `actix_web::web::Data` and move it into
//! the factory closure. The closure is called once-per-thread, and we clone our state
//! and attach to each instance of the `App` with `.app_data(state.clone())`.
//!
//! For thread-local state, we construct our state within the factory closure and attach to
//! the app with `.data(state)`.
//!
//! We retrieve our app state within our handlers with a `state: Data<...>` argument.
//!
//! By default, `actix-web` runs one `App` per logical cpu core.
//! When running on <N> cores, we see that the example will increment `counter1` (global state via
//! Mutex) and `counter3` (global state via Atomic variable) each time the endpoint is called,
//! but only appear to increment `counter2` every Nth time on average (thread-local state). This
//! is because the workload is being shared equally among cores.
//!
//! Check [user guide](https://actix.rs/docs/application/#state) for more info.

use std::cell::Cell;
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

#[derive(Debug)]
struct WebStateForKafka {
    name: String,
    counter: i64,
}

impl WebStateForKafka {
    fn new(name: String, counter: i64) -> WebStateForKafka {
        WebStateForKafka { name: name, counter: counter }
    }
}

/// simple handle
async fn index(
    state: web::Data<Mutex<WebStateForKafka>>,
    req: HttpRequest,
) -> HttpResponse {
    println!("{:?}", *state);

    // Increment the counter and the name
    // *state.counter += 1;
    // *state.name = format!(
    //     "{}{}", state.name, state.counter,
    // );

    let body = format!(
        "global state: {:?}", *state,
    );
    HttpResponse::Ok().body(body)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();




    // move is necessary to give closure below ownership of counter1
    HttpServer::new(move || {
        // Create some thread-local state
        let state = web::Data::new(WebStateForKafka::new("HiJon".to_string(), 42));

        App::new()
            .data(state) // thread local data
            .wrap(middleware::Logger::default())
            // register simple handler
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}