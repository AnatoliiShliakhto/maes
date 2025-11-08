use crate::{prelude::*, services::*};
use ::std::sync::{Arc, LazyLock};
use ::tokio::sync::{RwLock, watch};
use ::tracing::{error, info};
use ::windows::{
    Devices::WiFiDirect::{
        WiFiDirectAdvertisementListenStateDiscoverability, WiFiDirectAdvertisementPublisher,
        WiFiDirectAdvertisementPublisherStatus,
        WiFiDirectAdvertisementPublisherStatusChangedEventArgs, WiFiDirectConnectionListener,
        WiFiDirectConnectionRequest, WiFiDirectConnectionRequestedEventArgs, WiFiDirectDevice,
    },
    Foundation::TypedEventHandler,
    Security::Credentials::PasswordCredential,
    Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize},
    core::{HSTRING, Result},
};

static HOTSPOT: LazyLock<Arc<RwLock<Hotspot>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Hotspot::new())));
static HOTSPOT_STATUS: GlobalSignal<bool> = Signal::global(|| false);
static HOTSPOT_MSG_CH: LazyLock<(watch::Sender<HotspotMsg>, watch::Receiver<HotspotMsg>)> =
    LazyLock::new(|| watch::channel(HotspotMsg::Inactive));

#[derive(PartialEq)]
pub enum HotspotMsg {
    Inactive,
    Active,
    Error(&'static str),
}

pub struct HotspotService;

impl HotspotService {
    pub fn start(ssid: impl Into<String>, passphrase: impl Into<String>, direct: bool) {
        if *HOTSPOT_MSG_CH.1.borrow() == HotspotMsg::Active {
            return;
        }
        let ssid = ssid.into();
        let passphrase = passphrase.into();
        tokio::spawn(async move {
            if let Err(e) = HOTSPOT.write().await.start_hotspot(&ssid, &passphrase, direct) {
                error!("hotspot start failed: {e:?}");
                HOTSPOT_MSG_CH
                    .0
                    .send_replace(HotspotMsg::Error("hotspot-start-failed"));
            }
        });
    }

    pub fn stop() {
        if *HOTSPOT_MSG_CH.1.borrow() == HotspotMsg::Inactive {
            return;
        }
        tokio::spawn(async move {
            if let Err(e) = HOTSPOT.write().await.stop() {
                error!("{e:?}");
                HOTSPOT_MSG_CH
                    .0
                    .send_replace(HotspotMsg::Error("hotspot-stop-failed"));
            }
        });
    }

    pub fn subscribe() -> watch::Receiver<HotspotMsg> {
        HOTSPOT_MSG_CH.1.clone()
    }

    pub fn use_status() -> Signal<bool> {
        HOTSPOT_STATUS.signal()
    }

    pub fn use_init_status() -> Signal<bool> {
        use_coroutine(move |_rx: UnboundedReceiver<()>| async move {
            let mut rx = Self::subscribe();
            loop {
                if rx.changed().await.is_err() {
                    break;
                }
                match &*rx.borrow() {
                    HotspotMsg::Inactive => {
                        let direct = if HOTSPOT.read().await.direct { 1 } else { 0 };
                        HOTSPOT_STATUS.with_mut(|s| *s = false);
                        ToastService::warning(t!("hotspot-stopped", direct = direct))
                    }
                    HotspotMsg::Active => {
                        let direct = if HOTSPOT.read().await.direct { 1 } else { 0 };
                        HOTSPOT_STATUS.with_mut(|s| *s = true);
                        ToastService::success(t!("hotspot-started", direct = direct))
                    }
                    HotspotMsg::Error(e) => {
                        HOTSPOT_STATUS.with_mut(|s| *s = false);
                        ToastService::error(t!(e));
                    }
                }
            }
        });

        HOTSPOT_STATUS.signal()
    }
}

struct Hotspot {
    publisher: Option<WiFiDirectAdvertisementPublisher>,
    listener: Option<WiFiDirectConnectionListener>,
    direct: bool,
}

impl Hotspot {
    fn new() -> Self {
        Self {
            publisher: None,
            listener: None,
            direct: false,
        }
    }

    fn start_hotspot(&mut self, ssid: &str, passphrase: &str, direct: bool) -> Result<()> {
        self.direct = direct;
        let publisher = WiFiDirectAdvertisementPublisher::new()?;
        let advertisement = publisher.Advertisement()?;
        if !direct {
            advertisement.SetIsAutonomousGroupOwnerEnabled(true)?;
        }

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
            HOTSPOT_MSG_CH
                .0
                .send_replace(HotspotMsg::Error("hotspot-legacy-not-supported"));
        }

        self.setup_event_handlers(&publisher)?;

        publisher.Start()?;
        self.publisher = Some(publisher);

        let connection_listener = WiFiDirectConnectionListener::new()?;
        self.setup_connection_listener(&connection_listener)?;
        self.listener = Some(connection_listener);

        info!("SSID {ssid:?} password {passphrase:?}");
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(publisher) = &self.publisher {
            publisher.Stop()?;
        }
        self.publisher = None;
        self.listener = None;
        info!("deactivated");
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
                        WiFiDirectAdvertisementPublisherStatus::Created => (),
                        WiFiDirectAdvertisementPublisherStatus::Started => {
                            HOTSPOT_MSG_CH.0.send_replace(HotspotMsg::Active);
                        }
                        _ => {
                            HOTSPOT_MSG_CH.0.send_replace(HotspotMsg::Inactive);
                        }
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
                        _ = Self::handle_connection_request(request);
                    }
                    Err(e) => error!("{e:?}"),
                }
            }
            Ok(())
        });

        listener.ConnectionRequested(&connection_handler)?;
        Ok(())
    }

    fn handle_connection_request(request: WiFiDirectConnectionRequest) -> Result<()> {
        if let Ok(device_info) = request.DeviceInformation()
            && let Ok(id) = device_info.Id()
        {
            std::thread::spawn(move || {
                unsafe {
                    _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
                }

                let op = match WiFiDirectDevice::FromIdAsync(&id) {
                    Ok(op) => op,
                    Err(_) => {
                        unsafe {
                            CoUninitialize();
                        }
                        return;
                    }
                };

                let wfd = loop {
                    match op.GetResults() {
                        Ok(wfd) => break Some(wfd),
                        Err(_) => {
                            std::thread::sleep(std::time::Duration::from_millis(10));
                            continue;
                        }
                    }
                };

                drop(wfd);

                unsafe {
                    CoUninitialize();
                }
            });
        }
        Ok(())
    }
}

impl Drop for Hotspot {
    fn drop(&mut self) {
        _ = self.stop();
    }
}
