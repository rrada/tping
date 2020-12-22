![Rust](https://github.com/rrada/tping/workflows/Rust/badge.svg?branch=main)
# tping

## About

Simple command-line tool for measuring latency of TCP service.
The inspiration came out from the [tcp-latency](https://github.com/dgzlopes/tcp-latency) made in Python

## Why
Why not? Just for fun and to learn the Rust! :)

## Usage

    $ tping github.com
    google.com:80 seq=0 timeout=1 time=6.3030424 ms
    google.com:80 seq=1 timeout=1 time=5.372251 ms
    google.com:80 seq=2 timeout=1 time=8.78857 ms
    google.com:80 seq=3 timeout=1 time=7.431482 ms
    google.com:80 seq=4 timeout=1 time=8.663496 ms
    ---
    rtt min/avg/max = 5.372251/6.9738364/8.78857 ms
