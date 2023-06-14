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

## Docker (optional)
Download docker (only for ubuntu users):
```bash
sudo apt update && sudo apt upgrade -y
sudo apt install nala -y
sudo nala install ca-certificates curl gnupg lsb-release -y
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo nala update
sudo nala install docker-ce docker-ce-cli containerd.io docker-compose-plugin -y
sudo service docker start
sudo rm /etc/apt/sources.list.d/docker.list
```

Build the docker image:
```bash
sudo docker builder build . --tag www:latest
```

Run the docker image:
```bash
sudo docker run --net=host --rm www
```

Run in detached mode:
```bash
sudo docker run --restart unless-stopped -d -it --name bot --net=host www
```