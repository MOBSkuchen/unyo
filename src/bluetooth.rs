use zbus::{Connection, Proxy};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, OnceLock};
use zvariant::{Dict, Value};

pub static _BLUETOOTH_CTL: OnceLock<BluetoothController> = OnceLock::new();
pub static _BLUETOOTH_DATA: LazyLock<Mutex<Option<PlaybackData>>> =
    LazyLock::new(|| {Mutex::new(None)});

#[derive(Debug)]
pub struct BluetoothController<'a> {
    proxy: Proxy<'a>
}

#[derive(Debug)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

impl From<String> for PlaybackState {
    fn from(value: String) -> Self {
        if value == "playing" { PlaybackState::Playing }
        else if value == "paused" { PlaybackState::Paused }
        else { PlaybackState::Stopped }
    }
}

#[inline]
pub fn limit_string_size(input: &String, max_length: usize) -> String {
    if input.len() > max_length {
        let truncated = input.chars().take(max_length - 3).collect::<String>();
        format!("{}...", truncated)
    } else {
        input.to_string()
    }
}


#[derive(Debug)]
pub struct PlaybackData {
    pub title: String,
    pub artist: String,
    #[allow(dead_code)]
    playback_state: PlaybackState,
    pub position: u32,
    pub duration: u32,
    pub shuffle: bool,
    pub volume: u32,
}

impl PlaybackData {
    pub fn new(title: String,
               artist: String,
               playback_state: PlaybackState,
               position: u32,
               duration: u32,
               shuffle: bool,
               volume: u32) -> Self {
        Self { title, artist, playback_state, position, duration, shuffle, volume }
    }
    
    pub fn line_length(&self, full: i32) -> i32 {
        ((self.position as f32 / self.duration as f32) * full as f32) as i32
    }
}

impl From<(String, String, PlaybackState, u32, u32, bool, u32)> for PlaybackData {
    fn from(value: (String, String, PlaybackState, u32, u32, bool, u32)) -> Self {
        Self::new(limit_string_size(&value.0, 41), limit_string_size(&value.1, 20), value.2, value.3, value.4, value.5, value.6)
    }
}

pub async fn set_bluetooth_device_name(new_name: &str) -> Result<(), zbus::Error> {
    let connection = Connection::system().await?;

    let adapter_proxy = Proxy::new(
        &connection,
        "org.bluez",
        "/org/bluez/hci0",
        "org.freedesktop.DBus.Properties",
    ).await?;

    adapter_proxy.call_method(
        "Set",
        &(
            "org.bluez.Adapter1",
            "Alias",
            Value::new(new_name),
        )
    ).await?;
    Ok(())
}

pub async fn set_bluetooth(new_state: bool) -> Result<(), zbus::Error> {
    let connection = Connection::system().await?;

    let adapter_proxy = Proxy::new(
        &connection,
        "org.bluez",
        "/org/bluez/hci0",
        "org.bluez.Adapter1",
    ).await?;

    adapter_proxy.set_property("Discoverable", new_state).await?;
    adapter_proxy.set_property("Pairable", new_state).await?;

    // If disabling, disconnect all devices!
    if !new_state {
        // TODO: Implement
    }

    Ok(())
}

impl BluetoothController<'_> {
    pub async fn new() -> zbus::Result<Self> {
        let connection = Connection::system().await?;

        let proxy = Proxy::new(
            &connection,
            "org.bluez",
            "/",
            "org.freedesktop.DBus.ObjectManager",
        ).await?;
        
        Ok(Self {proxy})
    }
    
    async fn get_data(&self) -> zbus::Result<Option<(String, String, PlaybackState, u32, u32, bool, u32)>> {
        let managed_objects: HashMap<
            zvariant::OwnedObjectPath,
            HashMap<String, HashMap<String, zvariant::OwnedValue>>,
        > = self.proxy.call("GetManagedObjects", &()).await?;

        // O(2N), but idc
        let mut volume = 0u16;
        for (_, interfaces) in &managed_objects {
            if let Some(player_iface) = interfaces.get("org.bluez.MediaTransport1") {
                if let Some(r_volume) = player_iface.get("Volume") {
                    volume = r_volume.downcast_ref::<u16>()?;
                }
            }
        }
        for (_, interfaces) in managed_objects {
            if let Some(player_iface) = interfaces.get("org.bluez.MediaPlayer1") {
                
                let shuffle = if let Some(shuffle) = player_iface.get("Shuffle") {
                    // Shuffle: off, alltracks, group
                    shuffle.downcast_ref::<String>()? != "off"
                } else {false};

                if let Some(track_value) = player_iface.get("Track") {
                    let track: Dict = track_value.downcast_ref()?;

                    let title: String = track.get(&"Title".to_string())?.unwrap_or("Unknown".to_string());
                    let artist: String = track.get(&"Artist".to_string())?.unwrap_or("Unknown".to_string());
                    let duration: u32 = track.get(&"Duration".to_string())?.unwrap_or(0);

                    if let Some(position_value) = player_iface.get("Position") {
                        if let Ok(pos) = position_value.downcast_ref::<u32>() {
                            if let Some(status_value) = player_iface.get("Status") {
                                if let Ok(status) = status_value.downcast_ref::<String>() {
                                    return Ok(Some((title, artist, status.into(), pos, duration, shuffle, volume as u32)))
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }
    
    pub async fn poll(&self) -> Option<PlaybackData> {
        if let Ok(Some(data)) = self.get_data().await {
            Some(PlaybackData::from(data))
        } else {
            None
        }
    }
}
