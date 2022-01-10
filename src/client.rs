use std::collections::BTreeMap;

use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport, TIoChannel, TTcpChannel};

use crate::fb303::Fb303Status;
use crate::scribe::{LogEntry, ResultCode, ScribeSyncClient, TScribeSyncClient};

pub struct ScribeClient {
    // TODO: make private when reconnection is implemented
    pub host: String,
    pub port: u32,
    conn: Box<dyn TScribeSyncClient>,
}

impl ScribeClient {
    pub fn new(host: String, port: u32) -> ScribeClient {
        let mut c = TTcpChannel::new();
        c.open(format!("{}:{}", host, port)).unwrap();
        let (i_chan, o_chan) = c.split().unwrap();

        let i_prot = TBinaryInputProtocol::new(TFramedReadTransport::new(i_chan), false);
        let o_prot = TBinaryOutputProtocol::new(TFramedWriteTransport::new(o_chan), false);

        let conn = Box::new(ScribeSyncClient::new(i_prot, o_prot));

        ScribeClient {
            host: host,
            port: port,
            conn: conn,
        }
    }

    pub fn log(&mut self, messages: Vec<LogEntry>) -> thrift::Result<ResultCode> {
        self.conn.log(messages)
    }

    pub fn get_status(&mut self) -> thrift::Result<Fb303Status> {
        self.conn.get_status()
    }
    pub fn get_name(&mut self) -> thrift::Result<String> {
        self.conn.get_name()
    }
    pub fn get_version(&mut self) -> thrift::Result<String> {
        self.conn.get_version()
    }
    pub fn get_status_details(&mut self) -> thrift::Result<String> {
        self.conn.get_status_details()
    }
    pub fn get_counters(&mut self) -> thrift::Result<BTreeMap<String, i64>> {
        self.conn.get_counters()
    }
    pub fn alive_since(&mut self) -> thrift::Result<i64> {
        self.conn.alive_since()
    }
}
