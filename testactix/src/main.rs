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
use std::ops::{Deref, DerefMut};

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

#[derive(Debug)]
struct MyState<T> {
    mystruct: T,
}

impl<T> Deref for MyState<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.mystruct
    }
}

impl<T> DerefMut for MyState<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mystruct
    }
}








// impl MyState {

//     fn new(mystruct: WebStateForKafka) -> MyState {
//         MyState { mystruct: mystruct }
//     }

// }

// impl Deref for MyState {

//     fn deref(&self) -> &mut WebStateForKafka {
//         &self.mystruct
//     }
// }



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
    counter1: web::Data<Mutex<usize>>,
    counter2: web::Data<Cell<u32>>,
    counter3: web::Data<AtomicUsize>,
    state1: web::Data<MyState<WebStateForKafka>>,
    req: HttpRequest,
) -> HttpResponse {
    println!("req={:?}", req);
    println!("state={:?}", state1);
//    println!("state.name={} and counter={}", state1.name, state1.counter);

    // Increment the counters
    *counter1.lock().unwrap() += 1;
    counter2.set(counter2.get() + 1);
    counter3.fetch_add(1, Ordering::SeqCst);
    state1.mystruct.counter += 1;
    // let body = format!(
    //     "global mutex counter: {}, local counter: {}, global atomic counter: {}",
    //     *counter1.lock().unwrap(),
    //     counter2.get(),
    //     counter3.load(Ordering::SeqCst),
    // );
    let body = format!(
        "state1.counter: {}",
        state1.mystruct.counter,
    );
    HttpResponse::Ok().body(body)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create some global state prior to building the server
    let state1 = web::Data::new(MyState {mystruct: WebStateForKafka::new("InsideGrog".to_string(), 99) });
    let counter1 = web::Data::new(Mutex::new(0usize));
    let counter3 = web::Data::new(AtomicUsize::new(0usize));

    // move is necessary to give closure below ownership of counter1
    HttpServer::new(move || {
        // Create some thread-local state
        let counter2 = Cell::new(0u32);
        let state = WebStateForKafka::new("HiJon".to_string(), 42);

        App::new()
            .app_data(state1.clone()) // add shared state
            .app_data(counter1.clone()) // add shared state
            .app_data(counter3.clone()) // add shared state
            .data(counter2) // add thread-local state
            .data(state) // add thread-local state
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}