use chrono::{DateTime, Local, TimeDelta};
use futures_util::{stream::StreamExt, try_join};
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;
use zbus::Connection;
use zbus_macros::proxy;

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

    #[zbus(signal)]
    fn lock(&self) -> zbus::Result<()>;

    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn active(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn state(&self) -> zbus::Result<String>;
}

#[derive(Debug)]
struct Timer {
    timestamp: DateTime<Local>,
    start_time: DateTime<Local>,
    break_duration: TimeDelta,
}

impl Timer {
    fn new() -> Timer {
        let now = Local::now();
        return Timer {
            timestamp: now,
            start_time: now,
            break_duration: TimeDelta::zero(),
        };
    }
    fn update_timestamp(&mut self, new_value: DateTime<Local>) {
        self.timestamp = new_value;
    }

    fn update_break_duration(&mut self, new_value: TimeDelta) {
        self.break_duration = self.break_duration.checked_add(&new_value).unwrap();
    }

    fn calculate_diff(&self, current_time: DateTime<Local>) -> TimeDelta {
        current_time.time() - self.timestamp.time()
    }
}

#[tokio::main]
pub async fn run_timer() -> zbus::Result<()> {
    // est connections
    let session_connection = Connection::session().await?;
    let system_connection = Connection::system().await?;

    // setup proxy with the connections
    let session_manager_proxy = SessionManagerProxy::new(&session_connection).await?;
    let login_session_proxy = Login1SessionProxy::new(&system_connection).await?;

    //setup streams
    let mut session_manager_stream = session_manager_proxy.receive_status_changed().await?;
    let mut login_state_stream = login_session_proxy.receive_unlock().await?;

    let timer = Arc::new(Mutex::new(Timer::new()));
    let timer_clone_1 = Arc::clone(&timer);
    let timer_clone_2 = Arc::clone(&timer);

    // handler for the unlock dbus signal
    let unlock_stream_handle = tokio::spawn(async move {
        while let Some(_) = login_state_stream.next().await {
            let current_time = Local::now();
            println!("logged in mate: {}", current_time);
        }
    });

    // handler for the screensaver dbus signal
    let lock_stream_handle = tokio::spawn(async move {
        while let Some(signal) = session_manager_stream.next().await {
            let args: StatusChangedArgs = signal.args().expect("Error parsing the args");
            match args.status {
                3 => {
                    let mut timer = timer_clone_2.lock().await;
                    let locktime = Local::now();
                    timer.update_timestamp(locktime);
                    println!("this is the lock part: {}", timer.timestamp);
                }
                _ => {}
            }
        }
    });

    try_join!(lock_stream_handle, unlock_stream_handle);

    Ok(())
}
