use ::std::{collections::HashMap, cell::RefCell, sync::{LazyLock, RwLock}};
use ::fluent_bundle::{FluentBundle, FluentResource};
use ::unic_langid::LanguageIdentifier;
pub use ::fluent_bundle::FluentArgs;

static CURRENT_LOCALE: LazyLock<RwLock<LanguageIdentifier>> = LazyLock::new(|| {
    let uk: LanguageIdentifier = "uk-UA".parse().expect("bad lang id");
    RwLock::new(uk)
});

static I18N_SOURCES: LazyLock<I18nSources> = LazyLock::new(|| {
    const UK_UA: &str = include_str!("../../../i18n/uk_UA.ftl");

    let uk: LanguageIdentifier = "uk-UA".parse().unwrap();

    let mut sources = HashMap::new();
    sources.insert(uk, vec![UK_UA]);

    I18nSources { sources }
});

struct I18nSources {
    sources: HashMap<LanguageIdentifier, Vec<&'static str>>,
}

thread_local! {
    static TLS_BUNDLE: RefCell<Option<(LanguageIdentifier, FluentBundle<FluentResource>)>> = RefCell::new(None);
}

fn build_bundle(lang: &LanguageIdentifier, ftl_sources: &[&'static str]) -> FluentBundle<FluentResource> {
    let mut bundle = FluentBundle::new(vec![lang.clone()]);
    for src in ftl_sources {
        let res = FluentResource::try_new((*src).to_string()).expect("invalid FTL");
        bundle.add_resource_overriding(res);
    }
    bundle
}

fn with_current_bundle<F, R>(f: F) -> R
where
    F: FnOnce(&FluentBundle<FluentResource>) -> R,
{
    let current = {
        let g = CURRENT_LOCALE.read().expect("i18n poisoned");
        g.clone()
    };

    let sources_for_lang = I18N_SOURCES.sources.get(&current).map(|v| v.as_slice());

    TLS_BUNDLE.with(|cell| {
        let mut opt = cell.borrow_mut();
        let need_rebuild = match &*opt {
            Some((cached_lang, _)) => cached_lang != &current,
            None => true,
        };

        if need_rebuild {
            let bundle = match sources_for_lang {
                Some(srcs) => build_bundle(&current, srcs),
                None => build_bundle(&current, &[]),
            };
            *opt = Some((current.clone(), bundle));
        }

        let (_, bundle) = opt.as_ref().expect("bundle must exist");
        f(bundle)
    })
}

pub fn i18n_set_locale(lang: &str) -> bool {
    let lang: LanguageIdentifier = match lang.parse() {
        Ok(l) => l,
        Err(_) => return false,
    };

    let known = I18N_SOURCES.sources.contains_key(&lang);

    {
        let mut g = CURRENT_LOCALE.write().expect("i18n poisoned");
        *g = lang;
    }

    TLS_BUNDLE.replace(None);

    known
}

pub fn i18n_get_locale() -> String {
    let g = CURRENT_LOCALE.read().expect("i18n poisoned");
    g.to_string()
}

pub fn t(key: impl AsRef<str>) -> String {
    with_current_bundle(|bundle| format_msg(bundle, key.as_ref(), None))
}

pub fn t_args(key: impl AsRef<str>, args: &FluentArgs) -> String {
    with_current_bundle(|bundle| format_msg(bundle, key.as_ref(), Some(args)))
}

fn format_msg(
    bundle: &FluentBundle<FluentResource>,
    key: &str,
    args: Option<&FluentArgs>,
) -> String {
    let msg = bundle.get_message(key).and_then(|m| m.value());
    let Some(pattern) = msg else { return key.to_string() };
    let mut errors = vec![];
    let value = bundle.format_pattern(pattern, args, &mut errors);
    value.into_owned()
}

#[macro_export]
macro_rules! t {
    ($key:expr) => {
        ::shared::services::i18n::t($key)
    };

    // ($key:expr, $args:expr) => {
    //     ::shared::service::i18n::t_args($key, &$args)
    // };

    ($key:expr, $($name:ident = $value:expr),+ $(,)?) => {{
        let mut __fa = ::shared::services::i18n::FluentArgs::new();
        $(
            __fa.set(stringify!($name), $value);
        )+
        ::shared::services::i18n::t_args($key, &__fa)
    }};
}
