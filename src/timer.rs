use std::sync::mpsc::Sender;

use chrono::Local;
use futures_util::{stream::StreamExt, try_join};
use zbus::Connection;
use zbus_macros::proxy;

use crate::app::Message;

#[proxy(
    default_service = "org.gnome.SessionManager",
    default_path = "/org/gnome/SessionManager/Presence",
    interface = "org.gnome.SessionManager.Presence"
)]
trait SessionManager {
    #[zbus(signal)]
    fn status_changed(&self, status: u32) -> zbus::Result<()>;
}

#[proxy(
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1/session/_32",
    interface = "org.freedesktop.login1.Session"
)]
trait Login1Session {
    #[zbus(signal)]
    fn unlock(&self) -> zbus::Result<()>;
}

#[tokio::main]
pub async fn run_timer(tx: Sender<Message>) -> color_eyre::Result<()> {
    // est connections
    let session_connection = Connection::session().await?;
    let system_connection = Connection::system().await?;

    // setup proxy with the connections
    let session_manager_proxy = SessionManagerProxy::new(&session_connection).await?;
    let login_session_proxy = Login1SessionProxy::new(&system_connection).await?;

    //setup streams
    let mut session_manager_stream = session_manager_proxy.receive_status_changed().await?;
    let mut login_state_stream = login_session_proxy.receive_unlock().await?;

    // handler for the unlock dbus signal
    let tx_clone = tx.clone();
    let unlock_stream_handle = tokio::spawn(async move {
        while let Some(_) = login_state_stream.next().await {
            tx_clone.send(Message::Unlock).unwrap();
        }
    });

    // handler for the screensaver dbus signal
    let lock_stream_handle = tokio::spawn(async move {
        while let Some(signal) = session_manager_stream.next().await {
            let args: StatusChangedArgs = signal.args().expect("Error parsing the args");
            match args.status {
                3 => {
                    tx.send(Message::Lock).unwrap();
                }
                _ => {}
            }
        }
    });

    try_join!(lock_stream_handle, unlock_stream_handle)?;

    Ok(())
}
