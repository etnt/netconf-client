use crate::{args::Args, transport::ssh::SSHTransport, Connection};
use crossbeam::channel::{Receiver, Sender};
use log::debug;
use std::io::Result;
//use memchr::memmem;

pub fn session_loop(
    args: Args,
    _session_rx: Receiver<Vec<u8>>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let addr: String = format!("{}:{}", args.host, args.port);
    let user = args.user;
    let passwd = args.passwd;

    let ssh: SSHTransport =
        SSHTransport::connect(&addr, &user, &passwd).unwrap();
    let mut conn: Connection = Connection::new(ssh).unwrap();
    let _ = conn.send_hello();
    debug!("hello, got reply");

    // Just get the config and we are done!
    if args.get_config {
        debug!("get-config, send request");
        let get_config_reply = conn.get_config().unwrap();
        debug!("get-config, Reply: {}", get_config_reply);

        let _ = write_tx.send(get_config_reply.as_bytes().to_vec());

        let close_session_reply = conn.close_session().unwrap();
        debug!("close-session, Reply: {}", close_session_reply);

        return Ok(());
    } else if args.create_subscription {
        // FIXME should probably run in a separate thread either
        //       writing output to stdout or a file; also the
        //       stream should be configurable, not just 'NETCONF'
        //       This way we could handle input request as well as
        //       the output from the subscription.
        let create_subscription_reply = conn.create_subscription().unwrap();
        debug!("create-subscription, Reply: {}", create_subscription_reply);
        let _ = write_tx.send(create_subscription_reply.as_bytes().to_vec());
        loop{
            let resp = conn.transport.read()?;
            debug!("create-subscription, Reply: {}", resp);
            let _ = write_tx.send(resp.as_bytes().to_vec());
        }
    }

    Ok(())
}
