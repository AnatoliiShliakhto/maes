use ::std::borrow::Cow;
use ::tokio::sync::watch;

#[derive(Clone, PartialEq)]
pub enum DispatcherTask {
    None,
    Running,
    Finished,
    Failed,
}

#[derive(Clone, PartialEq)]
pub enum DispatcherMessage {
    None,
    Info(Cow<'static, str>),
    Success(Cow<'static, str>),
    Warning(Cow<'static, str>),
    Error(Cow<'static, str>),
}

#[derive(Clone)]
pub struct Dispatcher {
    task_tx: watch::Sender<DispatcherTask>,
    task_rx: watch::Receiver<DispatcherTask>,
    msg_tx: watch::Sender<DispatcherMessage>,
    msg_rx: watch::Receiver<DispatcherMessage>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        let (task_tx, task_rx) = watch::channel(DispatcherTask::None);
        let (msg_tx, msg_rx) = watch::channel(DispatcherMessage::None);
        Self { task_tx, task_rx, msg_tx, msg_rx }
    }
    
    pub fn task_subscribe(&self) -> watch::Receiver<DispatcherTask> {
        self.task_rx.clone()
    }

    pub fn msg_subscribe(&self) -> watch::Receiver<DispatcherMessage> {
        self.msg_rx.clone()
    }
    
    pub fn task_send(&self, task: DispatcherTask) {
        self.task_tx.send_replace(task);
    }
    
    pub fn msg_send(&self, msg: DispatcherMessage) {
        self.msg_tx.send_replace(msg);
    }
}