use slog::*;
use std::{fmt, result};

pub struct PrintlnSerializer;

impl Serializer for PrintlnSerializer {
    fn emit_arguments(&mut self, key: Key, val: &fmt::Arguments) -> Result {
        print!("{}={} ", key, val);
        Ok(())
    }
}

pub struct PrintlnDrain;

impl Drain for PrintlnDrain {
    type Ok = ();
    type Err = ();

    fn log(
        &self,
        record: &Record,
        values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {

        print!("{} ", record.msg());

        record
            .kv()
            .serialize(record, &mut PrintlnSerializer)
            .unwrap();
        values.serialize(record, &mut PrintlnSerializer).unwrap();

        println!("");
        Ok(())
    }
}

pub fn simulate_server(log: Logger) {
    let server = log.new(o!("host" => "test-server-2", "port" => "8080"));
    let peer1 = server.new(o!("peer_addr" => "8.8.8.8", "port" => "18230"));
    let peer2 = server.new(o!("peer_addr" => "82.9.9.9", "port" => "42381"));

    info!(server, "starting");
    info!(server, "listening");
    debug!(peer2, "connected");
    debug!(peer2, "message received"; "length" => 2);
    debug!(peer1, "connected");
    warn!(peer2,  "weak encryption requested"; "algo" => "xor");
    debug!(peer2, "response sent"; "length" => 8);
    debug!(peer2, "disconnected");
    debug!(peer1, "message received"; "length" => 2);
    debug!(peer1, "response sent"; "length" => 8);
    debug!(peer1, "disconnected");
    crit!(server, "internal error");
    info!(server, "exit");
}

