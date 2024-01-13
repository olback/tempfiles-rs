# Tempfiles

**Share files for up to 24h hours.**

Everything you upload, including filename and content-type is encrypted with [`aes-gcm-siv`](https://en.wikipedia.org/wiki/AES-GCM-SIV).

**Table setup**
| column          | data type      | description                                                |
| --------------- | -------------- |----------------------------------------------------------- |
| id              | `VARCHAR(32)`  | file id                                                    |
| iv              | `bytea`        | encryption iv (nonce)                                      |
| content         | `bytea`        | encrypted file data                                        |
| views           | `integer`      | valid file views                                           |
| max_views       | `integer/null` | delete after this amount of views, null if no limit is set |
| delete_password | `VARCHAR(32)`  | password used for file deletion                            |
| timestamp       | `timestamp`    | used to determine if the file is older than 24h            |

Inspired by [Carlgo11/TempFiles](https://github.com/carlgo11/TempFiles)

# Setup

## Backend

**Prerequisites**
* [musl](https://musl.libc.org/)
* [Rust](https://www.rust-lang.org/) (cargo, rustup)


**Add support for musl targets:**
```terminal
rustup target add x86_64-unknown-linux-musl --toolchain=nightly
rustup component add rust-std --target=x86_64-unknown-linux-musl
```

**Build:**
```terminal
cargo build --target x86_64-unknown-linux-musl --release
```

## Frontend

**Prerequisites**
* NodeJS / NPM

**Build:**
```terminal
cd frontend
npm i
npm run build
```

## Deploy

* Copy `.sample.env` to `.env` and modify.
* Start services: `docker-compose up` (add `-d` to detach)
* To configure the database: `scripts/postgres_setup.sh`
* Done!
