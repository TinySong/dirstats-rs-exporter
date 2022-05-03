// Will create an exporter with a single metric that will randomize the value
// of the metric everytime the exporter duration times out.

//use byte_unit::Byte;
// use env_logger::{Builder, Env};
// use log::info;
// use prometheus_exporter::prometheus::{labels, opts, register, register_gauge};
// use std::net::SocketAddr;

// fn main() {
//     // Setup logger with default level info so we can see the messages from
//     // prometheus_exporter.
//     Builder::from_env(Env::default().default_filter_or("info")).init();

//     // Parse address used to bind exporter to.
//     let addr_raw = "0.0.0.0:8080";
//     let addr: SocketAddr = addr_raw.parse().expect("can not parse listen addr");

//     // Start exporter and update metrics every five seconds.
//     let exporter = prometheus_exporter::start(addr).expect("can not start exporter");

//     // Create metric
//     let random = register_gauge!(opts!(
//         "dir_total_bytes",
//         "the directory file system total bytes.",
//         labels! {"path" => "/var/lib/docker",}
//     ))
//     .unwrap();

//     info!("Updating metrics");
//     let total_bytes = fs2::total_space("./").unwrap() as f64;
//     // let byte = Byte::from_bytes(total_bytes.into());
//     // let adjusted_byte = byte.get_appropriate_unit(false);
//     // println!(" total bytes {}", adjusted_byte);
//     // Update metric with random value.
//     random.set(total_bytes);
// }

use dirstat_rs_exporter::DiskItem;
use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use prometheus::{Encoder, Gauge, TextEncoder};
use std::path::Path;
//use reqwest;
//use std::env;

//use core::str::Split;
use lazy_static::lazy_static;
use prometheus::{labels, opts, register_gauge};

lazy_static! {
    static ref NODE_DISK_TOTAL_BYTES: Gauge = register_gauge!(opts!(
        "node_disk_total_bytes",
        "the directory file system total bytes.",
        labels! {"path" => "/var/lib/docker",}
    ))
    .unwrap();
    static ref NODE_DISK_USAGE_BYTES: Gauge = register_gauge!(opts!(
        "node_disk_usage_bytes",
        "the directory usage bytes.",
        labels! {"path" => "/var/lib/docker",}
    ))
    .unwrap();
}
async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    //TODO mock path
    // // node disk byte total
    let total_bytes = fs2::total_space("./").unwrap() as f64;
    NODE_DISK_TOTAL_BYTES.set(total_bytes);

    let target_dir = Path::new("./");
    let usage = DiskItem::from_analyze(&target_dir, false, 1);
    match usage {
        Ok(u) => println!("{}", u.disk_size), //NODE_DISK_USAGE_BYTES.set(u.disk_size as f64),
        Err(_) => (),
    }

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = ([0, 0, 0, 0], 9898).into();
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(serve_req))
    }));

    if let Err(err) = serve_future.await {
        eprintln!("server error: {}", err);
    }
}
