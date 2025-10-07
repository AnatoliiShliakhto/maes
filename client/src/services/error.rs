use crate::prelude::*;

static MESSAGE: GlobalSignal<String> = Signal::global(String::new);

#[derive(Copy, Clone)]
pub struct ErrorService; 

impl ErrorService {
    pub fn message() -> Signal<String> {
        MESSAGE.signal()
    }
    
    pub fn show(message: impl Into<String>) {
        MESSAGE.signal().set(message.into());
        use_navigator().push(Route::ErrorPage {});
    }
}