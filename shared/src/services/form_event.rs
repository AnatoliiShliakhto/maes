use ::std::str::FromStr;
use ::dioxus::prelude::FormEvent;
use ::dioxus::events::FormValue;

#[inline]
pub fn __fv_get_value(evt: &impl FormEventExt, name: &str) -> Option<String> {
    evt.get_value(name)
}
#[inline]
pub fn __fv_get_values(evt: &impl FormEventExt, name: &str) -> Option<Vec<String>> {
    evt.get_values(name)
}
#[inline]
pub fn __fv_get_parsed_value<T: FromStr>(evt: &impl FormEventExt, name: &str) -> Option<T> {
    evt.get_parsed_value::<T>(name)
}
#[inline]
pub fn __fv_get_parsed_values<T: FromStr>(evt: &impl FormEventExt, name: &str) -> Option<Vec<T>> {
    evt.get_parsed_values::<T>(name)
}

#[macro_export]
macro_rules! form_values {
    ($evt:expr $(, $rest:tt)* $(,)?) => {
        $crate::form_values!(@acc $evt; (); $($rest,)* @end)
    };

    (@acc $evt:expr; ($($acc:expr),*); @end) => {
        ($($acc),*)
    };

    (@acc $evt:expr; ($($acc:expr),*); $name:literal, $($rest:tt)*) => {
        $crate::form_values!(@acc $evt; ($($acc,)* $crate::services::__fv_get_value(&$evt, $name)); $($rest)*)
    };

    (@acc $evt:expr; ($($acc:expr),*); $name:literal as $ty:ty, $($rest:tt)*) => {
        $crate::form_values!(@acc $evt; ($($acc,)* $crate::services::__fv_get_parsed_value::<$ty>(&$evt, $name)); $($rest)*)
    };

    (@acc $evt:expr; ($($acc:expr),*); [$name:literal], $($rest:tt)*) => {
        $crate::form_values!(@acc $evt; ($($acc,)* $crate::services::__fv_get_values(&$evt, $name)); $($rest)*)
    };

    (@acc $evt:expr; ($($acc:expr),*); [$name:literal as $ty:ty], $($rest:tt)*) => {
        $crate::form_values!(@acc $evt; ($($acc,)* $crate::services::__fv_get_parsed_values::<$ty>(&$evt, $name)); $($rest)*)
    };
}

pub trait FormEventExt {
    fn stop(&self);
    fn get_value(&self, name: &str) -> Option<String>;
    fn get_values(&self, name: &str) -> Option<Vec<String>>;
    fn get_parsed_value<T: FromStr>(&self, name: &str) -> Option<T>;
    fn get_parsed_values<T: FromStr>(&self, name: &str) -> Option<Vec<T>>;
}

impl FormEventExt for FormEvent {
    fn stop(&self) {
        self.stop_propagation();
        self.prevent_default();
    }

    fn get_value(&self, name: &str) -> Option<String> {
        self.get(name).first().and_then(|v| form_value_to_str(v).map(|s| s.trim().to_string()))
    }

    fn get_values(&self, name: &str) -> Option<Vec<String>> {
        self.get(name).iter().map(|v| form_value_to_str(v).map(|s| s.trim().to_string())).collect()
    }

    fn get_parsed_value<T: FromStr>(&self, name: &str) -> Option<T> {
        match self.get(name).first().and_then(|v| form_value_to_str(v).map(|s| s.trim())) {
            Some(v) => v.parse::<T>().ok(),
            None => None
        }
    }

    fn get_parsed_values<T: FromStr>(&self, name: &str) -> Option<Vec<T>> {
        let results: Result<Vec<T>, <T as FromStr>::Err> = self
            .get(name)
            .iter()
            .filter_map(|v| form_value_to_str(v))
            .map(|s| s.trim().parse::<T>())
            .collect();

        results.ok()
    }
}

fn form_value_to_str(value: &FormValue) -> Option<&String> {
    match value {
        FormValue::Text(s) => Some(s),
        FormValue::File(_) => None,
    }
}