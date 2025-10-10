mod details;
mod start;
mod take;
mod finish;
mod retry;

pub use self::{
    details::*,
    start::*,
    take::*,
    finish::*,
    retry::*,
};

use crate::prelude::*;

static SURVEY: GlobalSignal<SurveyRecord> = Signal::global(SurveyRecord::default);
static CURRENT: GlobalSignal<usize> = Signal::global(|| 0_usize);
