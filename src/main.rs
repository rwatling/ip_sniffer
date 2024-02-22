use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

struct Arguments {
	ipaddr: IpAddr,
	threads: u16,
}

impl Arguments {
	fn new(args: &[String]) -> Result<Arguments, &'static str> {		
		if args.len() < 2 {
			return Err("need more arguments");
		} else if args.len() > 4 {
			return Err("too many arguments");
		}

		let f = args[1].clone();
		if let Ok(ipaddr) = IpAddr::from_str(&f) {
			return Ok(Arguments {ipaddr, threads: 4});
		} else if args.len() == 4 {
			// Break out early if "-h" is encountered first
			let flag = args[1].clone();
			if flag.contains("-h") || flag.contains("--help") {
				println!("Usage: \n\t-j <num-threads>\n\t-h or --help to show this message");
				return Err("help");
			} else if flag.contains("-j") {
				let threads = match args[2].parse::<u16>(){
					Ok(s) => s,
					Err(_) => return Err("Failed to parse a thread number")
				};
				let ipaddr = match IpAddr::from_str(&args[3]) {
					Ok(s) => s,
					Err(_) => return Err("Not a valid IPADDR; must be IPv4 or IPc6 address")
				};
				return Ok(Arguments{threads, ipaddr});
			} else {
				return Err("Invalid syntax");
			}
		} else {
			// Print help message if args length < 4 and -h option is present
			let flag = args[1].clone();
			if flag.contains("-h") || flag.contains("--help") {
				println!("Usage: \n\t-j <num-threads>\n\t-h or --help to show this message");
				return Err("help");
			} else {
				return Err("Invalid syntax");
			}
		}
	}
}

fn scan (trans: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
	let mut port: u16 = start_port + 1; //Skip port 0
	loop {
		match TcpStream::connect((addr, port)) {
			Ok(_) => {
				print!(".");
				io::stdout().flush().unwrap();
				trans.send(port).unwrap();
			}
			Err(_) => {}
		}

		if (MAX - port) <= num_threads {
			break;
		}
		port += num_threads;
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();
	let arguments = Arguments::new(&args).unwrap_or_else(|err| {
			if err.contains("help") {
				process::exit(0)
			} else {
				eprintln!("{} problem parsing arguments: {}", program, err);
				process::exit(1);
			}
		}
	);

	let num_threads = arguments.threads;
	let addr = arguments.ipaddr;
	let (trans, rec) = channel();
	for i in 0..num_threads {
		let trans = trans.clone();

		//Spawn thread
		thread::spawn(move || {
			scan(trans, i, addr, num_threads);
		});
	}

	let mut out = vec![];
	drop(trans);
	for p in rec {
		out.push(p);
	}

	println!("");
	out.sort();
	for v in out {
		println!("{} is open", v);
	}
}
