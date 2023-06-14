## Receipt Check

Web service allow to scan QR code from receipt and convert receipt items to convenient form.

### Command Line Options

Following command line options are available for service:

* `-a`, `--address` - bind address, default: `127.0.0.1`;
* `-d`, `--database` - database path, default: `db.sqlite`;
* `-p`, `--port` - bind port, default: `8080`;
* `-k`, `--key` - TLS certificate key;
* `-c`, `--certificate` - TLS certificate path.

### Generate Certificate

By default `tls` directory contains generated certificate to simplify server usage. New certificate can be generated
using `openssl` with following command:

```
openssl req -newkey rsa:2048 -nodes -keyout tls/key.pem -x509 -days 365 -out tls/certificate.pem
```

During certificate generation `openssl` will ask several questions.

### Build From Source

Building from source require installed `cargo` and `rust` compiler. To build executable file use following command:

```
cargo build --release
```

Compiled binary will be available in `target/release/receipt-check`.

## LICENSE

This project is licensed under the MIT License.
