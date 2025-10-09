use crate::prelude::*;
use ::gloo_timers::future::TimeoutFuture;

#[derive(Clone, Copy, PartialEq)]
pub enum TimerState {
    Running,
    Paused,
    Expired,
}

#[component]
pub fn Timer(
    mut duration: Signal<i64>,
    on_expired: EventHandler<()>,
    on_warning: Option<EventHandler<i64>>,
) -> Element {
    let mut timer_state = use_signal(|| TimerState::Running);

    use_future(move || async move {
        while timer_state() == TimerState::Running && duration() > 0 {
            TimeoutFuture::new(1000).await;

            if timer_state() == TimerState::Running {
                let new_time = duration() - 1;
                duration.set(new_time);

                match new_time {
                    300 | 180 | 60 | 30 | 10 => {
                        if let Some(on_warning) = on_warning {
                            on_warning.call(new_time);
                        }
                    }
                    0 => {
                        timer_state.set(TimerState::Expired);
                        on_expired.call(());
                    }
                    _ => {}
                }
            }
        }
    });

    let format_time = |seconds: i64| -> String {
        let h = seconds / 3600;
        let m = (seconds % 3600) / 60;
        let s = seconds % 60;
        if h > 0 {
            format!("{:01}:{:02}:{:02}", h, m, s)
        } else {
            format!("{:02}:{:02}", m, s)
        }
    };

    let time_class = match duration() {
        ..=60 => "text-error font-bold",
        61..=300 => "text-warning font-semibold",
        _ => "text-success",
    };

    rsx! {
        div {
            class: "flex items-center gap-2 {time_class}",
            i {
                class: "bi bi-stopwatch",
                class: if duration() <= 30 { "animate-bounce" } else { "" }
            }
            span {
                class: "font-mono text-lg",
                "{format_time(duration())}"
            }
        }
    }
}
