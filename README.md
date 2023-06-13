## Receipt Check

Web service allow to scan QR code from receipt and convert receipt items to convenient form.

### Command Line Options

Following command line options are available for service:

* `-a`, `--address` - default: `127.0.0.1`;
* `-d`, `--database` - default: `db.sqlite`;
* `-p`, `--port` - default: `8080`.

NB: service always use generated TLS certificate from `tls` directory. If it's necessarily to change certificate it can
be replaced with another one.

### Build From Source

Building from source require installed `cargo` and `rust` compiler. To build executable file use following command:

```
cargo build --release
```

Compiled binary will be available in `target/release/receipt-check`.

## LICENSE

This project is licensed under the MIT License.
