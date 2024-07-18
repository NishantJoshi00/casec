use rand::distributions::Alphanumeric;
use rand::Rng;
use time::PrimitiveDateTime;

pub trait Randr {
    fn factory<F>(factory: F) -> Self
    where
        F: FnOnce() -> Self,
        Self: Sized,
    {
        factory()
    }

    fn default() -> Self
    where
        Self: Sized;

    fn randr(factory: Option<fn() -> Self>, transformer: Option<fn(Self) -> Self>) -> Self
    where
        Self: Sized,
    {
        let output = factory.map(Self::factory).unwrap_or_else(Self::default);

        match transformer {
            Some(transformer) => transformer(output),
            None => output,
        }
    }
}

impl Randr for String {
    fn default() -> Self {
        String::from_utf8(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .collect::<Vec<_>>(),
        )
        .expect("Error while generating random string")
    }
}

impl Randr for i64 {
    fn default() -> Self {
        rand::thread_rng().gen_range(i64::MIN..i64::MAX)
    }
}

impl Randr for i16 {
    fn default() -> Self {
        rand::thread_rng().gen_range(i16::MIN..i16::MAX)
    }
}

impl<T: Randr> Randr for Option<T> {
    fn default() -> Self {
        bool::randr(None, None).then(|| T::default())
    }
}

impl Randr for PrimitiveDateTime {
    fn default() -> Self
    where
        Self: Sized,
    {
        let utc_date_time = time::OffsetDateTime::now_utc();
        PrimitiveDateTime::new(utc_date_time.date(), utc_date_time.time())
    }
}

impl Randr for serde_json::Value {
    fn default() -> Self {
        // Fix this
        serde_json::Value::Null
    }
}

impl Randr for bool {
    fn default() -> Self {
        rand::thread_rng().gen_bool(0.5)
    }
}
