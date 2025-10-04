use crate::prelude::*;
use ::std::sync::{Arc, LazyLock};
use ::tokio::sync::RwLock;
use ::tracing::{error, info, warn};
use ::windows::{
    Devices::WiFiDirect::{
        WiFiDirectAdvertisementListenStateDiscoverability, WiFiDirectAdvertisementPublisher,
        WiFiDirectAdvertisementPublisherStatus,
        WiFiDirectAdvertisementPublisherStatusChangedEventArgs, WiFiDirectConnectionListener,
        WiFiDirectConnectionRequest, WiFiDirectConnectionRequestedEventArgs,
    },
    Foundation::TypedEventHandler,
    Security::Credentials::PasswordCredential,
    core::{HSTRING, Result},
};

static HOTSPOT: LazyLock<Arc<RwLock<Hotspot>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Hotspot::new())));
static HOTSPOT_STATUS: GlobalSignal<bool> = Signal::global(|| false);

pub struct HotspotService;

impl HotspotService {
    pub fn start(ssid: impl Into<String>, passphrase: impl Into<String>) {
        if *HOTSPOT_STATUS.read() {
            return;
        }
        let ssid = ssid.into();
        let passphrase = passphrase.into();
        spawn(async move {
            if let Err(e) = HOTSPOT
                .write()
                .await
                .start_hotspot(&ssid, &passphrase)
            {
                error!("hotspot start failed: {e:?}");
            }
        });
    }

    pub fn stop() {
        if !*HOTSPOT_STATUS.read() {
            return;
        }
        spawn(async move {
            if let Err(e) = HOTSPOT.write().await.stop() {
                error!("hotspot stop failed: {e:?}");
            }
        });
    }

    pub fn status() -> Signal<bool> {
        HOTSPOT_STATUS.signal()
    }
}

struct Hotspot {
    publisher: Option<WiFiDirectAdvertisementPublisher>,
    listener: Option<WiFiDirectConnectionListener>,
}

impl Hotspot {
    fn new() -> Self {
        Self {
            publisher: None,
            listener: None,
        }
    }

    fn start_hotspot(&mut self, ssid: &str, passphrase: &str) -> Result<()> {
        let publisher = WiFiDirectAdvertisementPublisher::new()?;
        let advertisement = publisher.Advertisement()?;
        advertisement.SetIsAutonomousGroupOwnerEnabled(true)?;

        advertisement.SetListenStateDiscoverability(
            WiFiDirectAdvertisementListenStateDiscoverability::Normal,
        )?;

        if let Ok(legacy) = advertisement.LegacySettings() {
            legacy.SetIsEnabled(true)?;
            legacy.SetSsid(&HSTRING::from(ssid))?;

            let cred = PasswordCredential::new()?;
            cred.SetResource(&HSTRING::from("WiFiDirectPassphrase"))?;
            cred.SetUserName(&HSTRING::from("user"))?;
            cred.SetPassword(&HSTRING::from(passphrase))?;
            legacy.SetPassphrase(&cred)?;
        } else {
            warn!("Device does not support legacy settings");
        }

        self.setup_event_handlers(&publisher)?;

        publisher.Start()?;
        self.publisher = Some(publisher);

        let connection_listener = WiFiDirectConnectionListener::new()?;
        self.setup_connection_listener(&connection_listener)?;
        self.listener = Some(connection_listener);

        info!("started SSID {ssid:?} password {passphrase:?}");
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(publisher) = &self.publisher {
            publisher.Stop()?;
        }
        self.publisher = None;
        self.listener = None;
        info!("stopped");
        Ok(())
    }

    fn setup_event_handlers(&self, publisher: &WiFiDirectAdvertisementPublisher) -> Result<()> {
        let status_handler = TypedEventHandler::<
            WiFiDirectAdvertisementPublisher,
            WiFiDirectAdvertisementPublisherStatusChangedEventArgs,
        >::new(move |sender, _args| -> Result<()> {
            if let Some(p) = sender.as_ref() {
                let p: &WiFiDirectAdvertisementPublisher = p;
                if let Ok(status) = p.Status() {
                    match status {
                        WiFiDirectAdvertisementPublisherStatus::Started =>
                            HOTSPOT_STATUS.with_mut(|s| *s = true),
                        WiFiDirectAdvertisementPublisherStatus::Stopped =>
                            HOTSPOT_STATUS.with_mut(|s| *s = false),
                        _ => (),
                    }
                }
            }
            Ok(())
        });

        publisher.StatusChanged(&status_handler)?;
        Ok(())
    }

    fn setup_connection_listener(&self, listener: &WiFiDirectConnectionListener) -> Result<()> {
        let connection_handler = TypedEventHandler::<
            WiFiDirectConnectionListener,
            WiFiDirectConnectionRequestedEventArgs,
        >::new(move |_sender, args| -> Result<()> {
            if let Some(args) = args.as_ref() {
                let args: &WiFiDirectConnectionRequestedEventArgs = args;
                match args.GetConnectionRequest() {
                    Ok(request) => {
                        let _ = Self::handle_connection_request(request);
                    }
                    Err(e) => error!("connection request: {e:?}"),
                }
            }
            Ok(())
        });

        listener.ConnectionRequested(&connection_handler)?;
        Ok(())
    }

    fn handle_connection_request(request: WiFiDirectConnectionRequest) -> Result<()> {
        if let Ok(device_info) = request.DeviceInformation() && let Ok(id) = device_info.Id() {
            info!("device connected: {id}");
        }
        Ok(())
    }
}

impl Drop for Hotspot {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
