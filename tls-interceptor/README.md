
# tls-interceptor

Simple TLS parser.

This Rust crate is a proof-of-concept implementation of a TLS certificate validator. As a PoC
currently we only use pcap packet traces as input. The current progress is tracked in `examples/` and
the active developing part is in `src/`.

## Examples (missing)
```sh
# running the last version
$ cargo run --example last
# this command will parse tls-all.pcap file and give the output
$ cargo run --example parser-all
# this command will parse tls-cert.pcap file and give the output
$ cargo run --example parser-cert
```
## TODO
* handle tcp segment reassemble so that we can retrieve the certificate.


Current version: 0.1.0

Some additional info here

License: MIT OR Apache-2.0
