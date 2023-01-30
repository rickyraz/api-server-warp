use warp::{Filter, Rejection, Reply};

async fn hello1(param: String) -> Result<impl Reply, Rejection> {
    Ok(format!("Hello, {}!", param))
}

#[tokio::main]
async fn main() {
    //-- GET /hello/warp => 200 OK with body "Hello, warp!" --//

    // Mensimpelkan code dibawah
    // let hello = warp::path!("hello" / String).map(|name| format!("koko, {}!", name));

    /*
    let hello1 = warp::get()
        // Memberikan path awal
        .and(warp::path("hello"))
        // Memberikan parameter suatu path
        .and(warp::path::param::<String>())
        // Memberikan kondisi akhir dari path
        .and(warp::path::end())
        // Menggunakan fungsi hello untuk mengembalikan hasil
        .and_then(hello1);
    */

    // warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;

    //  -- SALAHH --
    // let hello = warp::get()
    //     .and(warp::path::param().map(|id: u32| format!("You asked for /{}", id)))
    //     .and(warp::path::end().map(|| "Hello, World!"));

    //  -- BENAR --
    let root = warp::get().and(warp::path::end().map(|| "Hello World"));
    let name =
        warp::get().and(warp::path::param().map(|nama: String| format!("Hello World, {}", nama)));

    warp::serve(root.or(name)).run(([127, 0, 0, 1], 3030)).await;
}
