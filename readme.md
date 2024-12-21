# Fancy Christmas Lights

Raspberry-pi controlled individually addressable LED christmas lights. There are
two parts to the project:
1. A backend that directly controlls the lights and exposes an interface over a web-server
2. A web-frontend that allows turning on/off, choosing a mode, and setting parameters

## Raspberry Pi

This project will NOT work with the raspberry pi 5. Due to a change in the GPIO
pins, the the rpi-ws281x C library does not (yet?) support the Pi 5. 

## To Get Started

As of right now - this is a fairly involved project - there is NO easy way to use it. 
The following instructions should at least get it close to working.

### Set up the raspberry pi OS

To start out, I set up a new raspberry pi OS using the rpi-imager program with a
host name of 'lights.local', my username and password, added in the connection
to my local wifi, and enabled SSH. This allowed me to work on the raspberry pi
over SSH and move file over using SCP.

### Install build tools

This assumes that you're going to do the build on the raspberry pi. If you want
to do development on another machine and just move the build artifacts over
(cross compiling) then you're currently on you own.

After the PI boots up, connect to it and open up a terminal, then update the system.

```sh
sudo apt update
sudo apt install
```

Then install the generic build tools

```sh
sudo apt install build-essential
```

Then install rust with [rustup](https://rustup.rs/) for building the backend.
Finally install latest node with
[node-version-manager](https://github.com/nvm-sh/nvm) for building the frontend.

### Clone repo

Clone this repo

```sh
git clone (todo: Add this repo uri)
cd repo
```

#### Get the rust LED driver code

clone the repo LED driver repo and remove the .cargo folder

```sh
git clone https://github.com/rpi-ws281x/rpi-ws281x-rust
rm -rf rpi-ws281x-rust/.cargo
```

### Intialize the frontend

```sh
cd frontend
npm install
cd ..
```

### Create self-signed certs

I also created a self-signed cert for the web-page and added the cert to my
computer, phone, and tablet to allow me to control the lights from whatever is
most convenient. (This allowed me to connect from my phone/tablet without major
complaints)

Run the following command to generate a private key and a self-signed certificate:

```sh
openssl req -x509 -newkey rsa:4096 -sha256 -days 365 \
  -nodes -keyout lights.local.key -out lights.local.crt -subj "/CN=lights.local" \
  -addext "subjectAltName=DNS:lights.local,DNS:localhost"
```
 
(asside: you should not add more than 365 days, because many new browsers will
reject certs with too long a life-span. Second, the -nodes command is "no DES",
as in don't use DES encryption on the key, not the work "nodes". Finally, the
addext command is required to create a v3 cert, and the DNS name or IP that you
use is required to go there for many newer TLS implementations such as rustls.)

You server will need access to the key and cert, and you will need to add the 
cert to the device you want to access the web frontend with.

### build

To build the backend, move to the rust directory and use cargo to build the project

```sh
cd rust
cargo build -r
cd ..
```

To build the frontend, move into the frontend directory and use npm/vite to build the project

```sh
cd frontend
npm run build
cd ..
```

If you have successfully built the backend and it exist in `rust/build/release`
directory, and you have successfully built the frontend and it now exist in the
`frontend\dist` directory, and you have the certs in the local directory, then
you should be able to install them using the install script.

```sh
sudo install.sh
```

This script adds the executable to `/opt/lights-<ver>/bin`, adds the certs to
the `/opt/lights-ver/secrets` directory, and the website files to the
`/opt/lights-<ver>/www` directory, and copys lights.service file to
`/etc/systemd/system/` and runs `systemctl enable lights.service` to enable the
system at startup.

## Backend

The backend is used to interface to the individually addressable ws2811 LEDs and
expose them over a web-server, and it is written in rust. 

Why rust? (a) Because I wanted to learn (b) Because I needed (wanted?) to drive
two lines of LEDs with the PI, and the python library didn't appear to allow
that.

### LED driver

The backend interfaces the ws2811 LEDs with the
[rpi-ws281x-rust](https://github.com/rpi-ws281x/rpi-ws281x-rust) crate, which is
itself a wrapper over the [rpi-ws281x](https://github.com/rpi-ws281x/rpi_ws281x)
c library. The crate defaults to building for a 32 bit arm, so I created a very
small edit (removing the .cargo/config file) which now allows me to target the
64bit arm for the raspberry pi 3b+ or 4.

You can use either 1 or 2 gpio pins to control the lights. I wanted to control
the lights wrapping around my house, but I wanted the controller in the middle,
so I chose to use 2 pins that were driven by the PMW0 and PWM1 pins exposed over
the raspberry pi header pins

### Web server

The webserver is hosted on the raspberry PI, and is exposed on my local subnet
and available to other devices on the same wifi-network. The web server is
written using the rust [axum](https://github.com/tokio-rs/axum) crate.

Check out the readme file in rust directory for more details.

## Frontend

The frontend was created with
[Vite+React+Tailwindcss](https://tailwindcss.com/docs/guides/vite#react) and builds
a static site that is then served by the axum web server.

Check out the readme file in the frontend directory for more details.