# SMS Core
> Service for managing SMS and other utility traffic between OS and GPRS Shield using a serial port.

# Build
1. Run `cargo install` to fetch all dependencies
2. Build command for arm v7 architecture server: `cargo build --color=always --message-format=json-diagnostic-rendered-ansi --release --target=armv7-unknown-linux-gnueabihf`

Target option can be any of architectures supported by rust compiler:
https://doc.rust-lang.org/nightly/rustc/platform-support.html

# Usage
```sh
Usage: sms-core -p PORT -i INTERFACE -d DEVICE -b BAUD [-t THREADS] [-v|-h|-s]

Options:
    -h, --help          print this help menu
    -v, --version       print program version
    -s, --skip          skip connection check
    -i, --interface INTERFACE
                        set server interface
    -p, --port PORT     set server port
    -d, --device DEVICE set serial device
    -b, --baud BAUD     set serial baud rate
    -t, --thread THREADS
                        set thread pool size
```

# Commands
Curently the service only supports 2 commands to interact with GPRS shield: check and send.

Commands are implementation of TinySMS protocol:[resources/TinySMS_protocol_specification.pdf](https://github.com/Lujo5/sms-core/resources/TinySMS_protocol_specification.pdf)

Sending commands to sms-core service is through direct TCP protocol, currently with no user authentication.

### CHECK
Sends ping to GPRS shield to check status of serial connection between sms-core service and device.

### SEND
Sends SMS message text to requested number in GSM format.