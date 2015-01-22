extern crate irc;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

use irc::server::{IrcServer, Server};
use irc::server::utils::Wrapper;
use irc::data::message::Message;

fn main() {
    let mut m = CMD.lock().unwrap();
    m.insert("PRIVMSG", privmsg);
    m.insert("PING", ping);
    m.insert("MODE", mode);
    m.insert("NOTICE", notice);

    let irc_server = IrcServer::new("config.json").unwrap();
    let server = Wrapper::new(&irc_server);
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap();
        match m.get(message.command.as_slice()) {
            Some(cmd) => cmd(message),
            None => default(message)
        }
    }
}

lazy_static! {
    static ref CMD: Mutex<HashMap<&'static str, fn(Message)>> = Mutex::new(HashMap::new());
}

fn default(message: Message) {
    println!("Prefix: {}", match message.prefix { Some(v) => v, None => "None".to_string() });
    println!("Command: {}", message.command);
    println!("Arguments: {:?}", message.args);
    println!("Suffix: {}", match message.suffix { Some(v) => v, None => "None".to_string() });
}

fn privmsg(msg: Message) {
    println!("privmsg!");
}

fn ping(msg: Message) {
    println!("ping!");
}

fn mode(msg: Message) {
    println!("mode!");
}

fn notice(msg: Message) {
    println!("notice!");
}
