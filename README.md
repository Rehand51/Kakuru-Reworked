# Kakuru-Reworked

KAKURU is a collection of mathematics calculators, migrated from Streamlit to Rust with Axum and Askama templates.

## Requirements

- Rust and Cargo (install with [rustup](https://rustup.rs/))

## Run locally

```powershell
cargo run
```

Open [http://127.0.0.1:3000](http://127.0.0.1:3000). Set `KAKURU_ADDRESS` to change the bind address, for example:

```powershell
$env:KAKURU_ADDRESS = "0.0.0.0:3000"
cargo run
```

The navigation menu contains all available calculators. Enter values and submit the form to see results. The original formulas—including existing quirks—are retained for compatibility.
