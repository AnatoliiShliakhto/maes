use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Steps {
    inner: Signal<Vec<String>>,
    current: Signal<usize>,
}

impl Steps {
    pub fn get_names(&self) -> Signal<Vec<String>> {
        self.inner
    }

    pub fn current(&self) -> Signal<usize> {
        self.current
    }

    pub fn get_step_name(&self, idx: usize) -> String {
        let idx = if idx == 0 {
            1
        } else if idx > self.inner.len() {
            self.inner.len()
        } else {
            idx - 1
        };
        self.inner.peek().get(idx).cloned().unwrap_or_default()
    }
}

pub fn use_init_steps(steps: Vec<String>) -> Steps {
    use_context_provider(|| Steps {
        inner: Signal::new(steps),
        current: Signal::new(1),
    })
}

pub fn use_steps() -> Steps {
    use_context::<Steps>()
}

#[component]
pub fn StepsContainer() -> Element {
    let steps = use_steps();

    rsx! {
        ul {
            class: "steps w-full my-4",
            for (idx, name) in steps.get_names().peek().iter().enumerate() {
                li {
                    class: format!("step {class}", class = if steps.current()() > idx { "step-primary" } else { "" }),
                    span {
                        class: "mt-1 text-xs",
                        "{name}"
                    }
                }
            }
        }
    }
}
