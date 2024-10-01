use dbus::channel::MatchingReceiver;
use dbus::message::MatchRule;
use dbus_crossroads::Crossroads;
use dbus_tokio::connection;
use futures::future;
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
    let mr = MatchRule::new_signal("org.gtk.Private.RemoteVolumeMonitor", "VolumeAdded");
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
    let (incoming_signal, stream) = conn.add_match(mr).await.unwrap().msg_stream();
    println!("stream setup");
    let stream = stream.for_each(|_| {
        println!("Phone connected!");
        async {}
    });
    println!("ready");
    futures::join!(stream,);

    // Needed here to ensure the "incoming_signal" object is not dropped too early
    conn.remove_match(incoming_signal.token()).await?;

    unreachable!()
}
