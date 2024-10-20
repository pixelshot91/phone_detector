use dbus::channel::MatchingReceiver;
use dbus::message::MatchRule;
use dbus_crossroads::Crossroads;
use dbus_tokio::connection;
use futures::future;
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

// This is our "Hello" object that we are going to store inside the crossroads instance.
struct Hello {
    called_count: u32,
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the D-Bus session bus (this is blocking, unfortunately).
    let (resource, conn) = connection::new_session_sync()?;

    // The resource is a task that should be spawned onto a tokio compatible
    // reactor ASAP. If the resource ever finishes, you lost connection to D-Bus.
    //
    // To shut down the connection, both call _handle.abort() and drop the connection.
    let _handle = tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    // or 'VolumeRemoved'
    let phone_connected_rule =
        MatchRule::new_signal("org.gtk.Private.RemoteVolumeMonitor", "VolumeAdded");

    // let incoming_signal = conn.add_match(mr).await?.cb(|_, (source,): (u8,)| {
    //     println!("Hello from {} happened on the bus!", source);
    //     true
    // });

    // let incoming_signal = conn.add_match(mr).await?.msg_cb(|_| {
    //     println!("HelloHappened on the bus!");
    //     true
    // });

    // ..or use the match as a stream if you prefer

    println!("before stream setup");

    use futures_util::stream::StreamExt;
    let (incoming_signal, stream) = conn
        .add_match(phone_connected_rule)
        .await
        .unwrap()
        .msg_stream();
    println!("stream setup");
    let stream = stream.for_each(|m| {
        println!("Phone connected!");
        dbg!(m.get_items());
        let mut items = m.iter_init();

        let third = items.nth(2).unwrap();
        let mut sstruct = third.as_iter().unwrap();
        let phone_mtp_path = sstruct.nth(5).unwrap().as_str().unwrap();
        dbg!(phone_mtp_path);

        let p = format!(
            "/run/user/1000/gvfs/{}",
            phone_mtp_path.replace("://", ":host=")
        );
        dbg!(&p);
        let mtp_dir = std::path::Path::new(&p);

        thread::sleep(Duration::from_millis(500));

        let c = mtp_dir.read_dir().unwrap();
        let nb_of_files = c.count();
        dbg!(nb_of_files);

        if nb_of_files == 0 {
            let open_action_id = "open_id";
            let notif_handle = notify_rust::Notification::new()
                .summary("Phone connected")
                .body("Click on 'Allow' on your phone notification")
                .icon("nix-snowflake")
                .action(open_action_id, "Open phone directory")
                .show()
                .unwrap();

            // TODO: listen to this signal
            // let phone_mounted_rule = MatchRule::new_signal("org.gtk.vfs.MountTracker", "Mounted");

            // notif_handle.wait_for_action(invocation_closure);
        }
        async {}
    });
    println!("ready");
    futures::join!(stream,);

    // Needed here to ensure the "incoming_signal" object is not dropped too early
    conn.remove_match(incoming_signal.token()).await?;

    unreachable!()
}

#[cfg(test)]
mod test {
    #[test]
    fn show_notif() {
        let open_action_id = "open_id";
        let notif_handle = notify_rust::Notification::new()
            .summary("Phone connected")
            .body("Click on 'Allow' on your phone notification")
            .icon("nix-snowflake")
            .action(open_action_id, "Open phone directory")
            .show()
            .unwrap();
        notif_handle.wait_for_action(|action_id| {
            dbg!(action_id);
            if action_id == open_action_id {
                println!("Opening phone dir")
            }
        });
    }
}
