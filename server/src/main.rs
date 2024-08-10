use cache::Cache;
use mimalloc::MiMalloc;
use ntex::{
    http::header::{HeaderValue, CONTENT_ENCODING},
    web::{self, types::State, App, HttpRequest, HttpResponse},
};

mod cache;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

async fn cached_files(req: HttpRequest, cache: State<Cache>) -> HttpResponse {
    let path = req.path();

    if let Some((file, compressed)) = cache.get(path) {
        let mut response = HttpResponse::Ok()
            .content_type(unsafe { HeaderValue::from_shared_unchecked(file.content_type.clone()) })
            .body(file.data);

        if compressed {
            response
                .headers_mut()
                .insert(CONTENT_ENCODING, HeaderValue::from_static("br"));
        }

        response
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let cache = Cache::new("./dist");
    println!("Initializing web server");

    web::server(move || {
        App::new()
            .default_service(web::route().to(cached_files))
            .state(cache.clone())
    })
    .bind("0.0.0.0:5174")?
    .run()
    .await
}
