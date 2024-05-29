use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;

// Max IP Port
const MAX: u16 = 65535;

// Address fallback
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

// CLI Arguments
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    // Address argument.  Accepts -a and --address and an IpAddr type. Falls back to the above constant.
    #[bpaf(long, short, argument("Address"), fallback(IPFALLBACK))]

	/// The address that you want to sniff.  Must be a valid ipv4 address.  Falls back to 127.0.0.1
    pub address: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        guard(start_port_guard, "Start port must be greater than 0"),
        fallback(1u16)
    )]

    /// The start port for the sniffer
    pub start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        guard(end_port_guard, "End port must than or equal to 65535"),
        fallback(MAX)
    )]

    /// The end port for the sniffer
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}

// Scan the port.
async fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr) {

	// Attempts Connects to the address and the given port.
    match TcpStream::connect(format!("{}:{}", addr, start_port)).await {

		// If the connection is successful, print out a . and then pass the port through the channel.
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(start_port).unwrap();
        }

		// If the connection is unsuccessful, do nothing. Means port is not open.
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {

    // collect the arguments.
    let opts = arguments().run();

    // Initialize the channel.
    let (tx, rx) = channel();

    // Iterate through all of the ports (based on user input) so that we can spawn a single 
	// task (tokio green thread) for each.
    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();

        task::spawn(async move { scan(tx, i, opts.address).await });
    }

    // Wait for all of the outputs to finish and push them into the vector.
    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
