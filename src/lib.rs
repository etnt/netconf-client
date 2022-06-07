use std::io;
use transport::Transport;

pub mod args;
pub mod read;
pub mod session;
pub mod transport;
pub mod write;

const CHUNK_SIZE: usize = 16 * 1024;

const HELLO: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<hello xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <capabilities>
        <capability>urn:ietf:params:netconf:base:1.0</capability>
    </capabilities>
</hello>"#;

const GET_CONFIG: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc message-id="100"
    xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <get-config>
        <source>
            <running/>
        </source>
    </get-config>
</rpc>"#;

const CLOSE_SESSION: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc message-id="106"
     xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
   <close-session/>
</rpc>"#;

const CREATE_SUBSCRIPTION: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc xmlns="urn:ietf:params:xml:ns:netconf:base:1.0" message-id="1">
  <create-subscription xmlns="urn:ietf:params:xml:ns:netconf:notification:1.0">
    <stream>NETCONF</stream>
  </create-subscription>
</rpc>"#;

///
/// The 'transport/ssh' code is taken from netconf-rs.
///
/// A connection to NETCONF server
pub struct Connection {
    pub(crate) transport: Box<dyn Transport + Send + 'static>,
}

impl Connection {
    pub fn new(transport: impl Transport + 'static) -> io::Result<Connection> {
        let res = Connection {
            transport: Box::from(transport),
        };
        Ok(res)
    }

    pub fn send_hello(&mut self) -> io::Result<String> {
        self.transport.write(HELLO)?;
        let resp = self.transport.read()?;
        Ok(resp)
    }

    pub fn get_config(&mut self) -> io::Result<String> {
        self.transport.write(GET_CONFIG)?;
        let resp = self.transport.read()?;
        Ok(resp)
    }

    pub fn create_subscription(&mut self) -> io::Result<String> {
        self.transport.write(CREATE_SUBSCRIPTION)?;
        let resp = self.transport.read()?;
        Ok(resp)
    }

    pub fn close_session(&mut self) -> io::Result<String> {
        self.transport.write(CLOSE_SESSION)?;
        let resp = self.transport.read()?;
        Ok(resp)
    }
}
