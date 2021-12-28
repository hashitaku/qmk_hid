#[cfg(target_os = "linux")]
fn media_meta() -> Vec<(String, String)> {
    use dbus::{arg::RefArg, blocking::stdintf::org_freedesktop_dbus::Properties};
    let conn = dbus::blocking::Connection::new_session().unwrap();
    let (names,): (Vec<String>,) = {
        let proxy = conn.with_proxy(
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            std::time::Duration::from_millis(5000),
        );

        proxy
            .method_call("org.freedesktop.DBus", "ListNames", ())
            .unwrap()
    };

    let players = names
        .iter()
        .filter(|x| x.contains("org.mpris.MediaPlayer2"));

    players
        .map(|x| {
            let proxy = conn.with_proxy(
                x,
                "/org/mpris/MediaPlayer2",
                std::time::Duration::from_millis(5000),
            );

            let meta: dbus::arg::PropMap = proxy
                .get("org.mpris.MediaPlayer2.Player", "Metadata")
                .unwrap();

            match (meta.get("xesam:title"), meta.get("xesam:artist")) {
                (Some(title), Some(artists)) => (
                    title.as_str().unwrap().to_string(),
                    artists
                        .as_static_inner(0)
                        .unwrap()
                        .as_iter()
                        .unwrap()
                        .fold("".to_string(), |acc, x| acc + x.as_str().unwrap() + " ")
                        .trim_end()
                        .to_string(),
                ),

                (Some(title), None) => (title.as_str().unwrap().to_string(), "None".to_string()),

                (None, Some(artists)) => (
                    "None".to_string(),
                    artists
                        .as_static_inner(0)
                        .unwrap()
                        .as_iter()
                        .unwrap()
                        .fold("".to_string(), |acc, x| acc + x.as_str().unwrap() + " ")
                        .trim_end()
                        .to_string(),
                ),

                (_, _) => ("None".to_string(), "None".to_string()),
            }
        })
        .collect()
}

/*
#[cfg(target_os = "windows")]
fn media_title() -> Vec<String> {
    let mgr =
        windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .unwrap()
            .get()
            .unwrap();

    match mgr.GetSessions() {
        Ok(v) => v
            .into_iter()
            .map(|x| {
                x.TryGetMediaPropertiesAsync()
                    .unwrap()
                    .get()
                    .unwrap()
                    .Title()
                    .unwrap()
                    .to_string()
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}
*/

#[cfg(target_os = "linux")]
#[allow(dead_code)]
fn main_loop() {
    let conn = dbus::blocking::Connection::new_session().unwrap();
    let expr = dbus::message::MatchRule::new_signal(
        "org.freedesktop.DBus.Properties",
        "PropertiesChanged",
    );

    use dbus::blocking::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged;

    conn.add_match(expr, |_: PropertiesPropertiesChanged, _, _| {
        println!("{:?}", media_meta());
        true
    })
    .unwrap();

    loop {
        conn.process(std::time::Duration::from_millis(5000))
            .unwrap();
    }
}

#[cfg(target_os = "windows")]
fn main_loop() {
    todo!();
}

fn main() {
    match hidapi::HidApi::new() {
        Ok(hidapi) => {
            for e in hidapi.device_list() {
                println!(
                    "{:x} {:x} {:?} {:?}",
                    e.product_id(),
                    e.vendor_id(),
                    e.product_string(),
                    e.manufacturer_string()
                );
            }
        }
        Err(_) => {}
    }

    println!("{:?}", media_meta());
}
