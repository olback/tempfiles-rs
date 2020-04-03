# Tempfiles

Inspired by [Carlgo11/TempFiles](https://github.com/carlgo11/TempFiles)

## Backend

[To take advantage of high performance AES-NI and CLMUL CPU intrinsics](https://docs.rs/aes-gcm-siv/0.4.1/aes_gcm_siv/#performance-notes):
```terminal
source scripts/env
```

**Build:**
```terminal
cargo build
```

## Frontend

**Build:**
```terminal
cd frontend
npm i
npm run build
```
