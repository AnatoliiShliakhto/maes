mod details;
mod start;
mod take;
mod finish;

pub use self::{
    details::*,
    start::*,
    take::*,
    finish::*,
};

use crate::prelude::*;

static QUIZ: GlobalSignal<QuizActivity> = Signal::global(QuizActivity::default);
static CURRENT: GlobalSignal<usize> = Signal::global(|| 0_usize);
static TIMER: GlobalSignal<i64> = Signal::global(|| 0_i64);