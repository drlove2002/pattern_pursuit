# Pattern Pursuit
Unraveling Human Predictability

# Development
## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [npm](https://www.npmjs.com/get-npm)

## Setup
Install the required npm packages:
```bash
cd frontend
npm install
```
Install the required rust packages:
```bash
cd backend
cargo install cargo-watch
cargo update
```
Setup the environment variables:
```bash
cd backend
mv .env.example .env

# Set the environment variables in .env
```

## Run
To start auto compiling of js, scss and html files run:
```bash
npm run watch
```
Start a new terminal.
Then start the server with auto reloading on changes with:
```bash
cargo watch -q -c -w src/ -x run
```
