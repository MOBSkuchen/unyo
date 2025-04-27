use zbus::{Connection, Proxy};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, MutexGuard, OnceLock};
use chrono::Timelike;
use zvariant::{Dict};

fn format_time(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{}:{:02}", minutes, seconds)
}


pub static _BLUETOOTH_CTL: OnceLock<BluetoothController> = OnceLock::new();
pub static _BLUETOOTH_DATA: LazyLock<Mutex<PlaybackData>> =
    LazyLock::new(|| {Mutex::new(PlaybackData::new(String::new(), String::new(), PlaybackState::Stopped, 0, 0))});

pub async fn UPDATE_BLUETOOTH_DATA() {
    let data = _BLUETOOTH_CTL.get().unwrap().poll().await.unwrap();
    *_BLUETOOTH_DATA.lock().unwrap() = data;
}

pub fn BLUETOOTH_DATA<'a>() -> MutexGuard<'a, PlaybackData> {
    _BLUETOOTH_DATA.lock().unwrap()
}

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

#[derive(Debug)]
pub struct PlaybackData {
    title: String,
    artist: String,
    playback_state: PlaybackState,
    position: u32,
    duration: u32
}

impl PlaybackData {
    pub fn new(title: String,
               artist: String,
               playback_state: PlaybackState,
               position: u32,
               duration: u32) -> Self {
        Self { title, artist, playback_state, position, duration }
    }
}

impl From<(String, String, PlaybackState, u32, u32)> for PlaybackData {
    fn from(value: (String, String, PlaybackState, u32, u32)) -> Self {
        Self::new(value.0, value.1, value.2, value.3, value.4)
    }
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
    
    async fn get_data(&self) -> zbus::Result<Option<(String, String, PlaybackState, u32, u32)>> {
        let managed_objects: HashMap<
            zvariant::OwnedObjectPath,
            HashMap<String, HashMap<String, zvariant::OwnedValue>>,
        > = self.proxy.call("GetManagedObjects", &()).await?;
        
        for (path, interfaces) in managed_objects {
            if let Some(player_iface) = interfaces.get("org.bluez.MediaPlayer1") {
                
                if let Some(track_value) = player_iface.get("Track") {
                    let track: Dict = track_value.downcast_ref()?;

                    let title: String = track.get(&"Title".to_string())?.or(Some("Unknown".to_string())).unwrap();
                    let artist: String = track.get(&"Artist".to_string())?.or(Some("Unknown".to_string())).unwrap();
                    let duration: u32 = track.get(&"Duration".to_string())?.or(Some(0)).unwrap();

                    if let Some(position_value) = player_iface.get("Position") {
                        if let Ok(pos) = position_value.downcast_ref::<u32>() {
                            if let Some(status_value) = player_iface.get("Status") {
                                if let Ok(status) = status_value.downcast_ref::<String>() {
                                    return Ok(Some((title, artist, status.into(), pos, duration)))
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
