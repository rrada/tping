use std::net::{TcpStream, ToSocketAddrs, SocketAddr};
use std::time::{Duration, Instant};
use structopt::StructOpt;

use tping::utils;

// CLI arguments settings
#[derive(StructOpt)]
struct Cli {
    host: String,

    #[structopt(short = "p", long = "port", default_value = "80", help = "Service port")]
    port: u32,
    #[structopt(short = "t", long = "timeout", default_value = "1", help = "Connection timeout [secs]")]
    timeout: f32,
    #[structopt(short = "r", long = "reps", default_value = "5", help = "Number of repetions")]
    reps: u32,
    #[structopt(short = "w", long = "wait", default_value = "1", help = "Wait time in between measurements [secs]")]
    wait: u64,
}

fn get_server(host: &str, port: &u32) -> SocketAddr {
    let conn_string: String = format!("{}:{}", host, port);

    match conn_string.to_socket_addrs() {
        Ok(mut addr) => {
            match addr.next() {
                Some(s) => return s,
                None => panic!("Not a valid socket addr"), // Not a SocketAddr
            }
        },
        Err(e) => {
            eprintln!("error: {}:{} ({})", host, port, e);
            std::process::exit(1);
        }
    }
}

fn measure_latency(server: &SocketAddr, timeout: f32) -> f32 {
    let timer = Instant::now();
    let stream = TcpStream::connect_timeout(&server, Duration::from_secs_f32(timeout));
    match stream {
        Ok(_) => return timer.elapsed().as_secs_f32() * 1000.0,
        Err(_) => -1.0,
    }
}

fn main() {
    let args = Cli::from_args();

    let host = &args.host;
    let port = &args.port;
    let timeout = &args.timeout;
    let reps = &args.reps;
    let wait = &args.wait;

    let mut latency_measurements: Vec<f32> = Vec::new();

    let server = get_server(&host, &port);

    for r in 0..*reps {
        std::thread::sleep(Duration::new(*wait, 0));
        let last_latency_measurement: f32 = measure_latency(&server, *timeout);

        if last_latency_measurement >= 0.0 {
            println!("{}:{} seq={} timeout={} time={} ms", host, port, r, timeout, last_latency_measurement);
        } else {
            println!("{}:{} seq={} timeout={} failed", host, port, r, timeout);
        }

        // print stats at the end
        if r == *reps-1 {
            if latency_measurements.len() > 0 {
                let min: f32 = utils::vec_min(&latency_measurements);
                let max: f32 = utils::vec_max(&latency_measurements);
                let avg: f32 = utils::vec_avg(&latency_measurements);

                println!("---");
                println!("rtt min/avg/max = {}/{}/{} ms", min, avg, max);
            }
        }

        latency_measurements.push(last_latency_measurement);
    }
}