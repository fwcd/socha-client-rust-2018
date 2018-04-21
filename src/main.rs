mod arg_parser;
mod game;
mod xml_client;
mod xml_utils;

use arg_parser::ArgParser;
use xml_client::XMLClient;

fn main() {
	let args = ArgParser::new();
	let host = args.get_string("-h").or(args.get_string("--host")).unwrap_or("localhost".to_string());
	let port = args.get_string("-p").or(args.get_string("--port")).unwrap_or("13050".to_string());
	let reservation = args.get_string("-r").or(args.get_string("--reservation")).unwrap_or("".to_string());
	let client = XMLClient::new();
	client.run(&(host + ":" + port.as_str()), &reservation);
}