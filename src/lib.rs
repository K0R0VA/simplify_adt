#[macro_export]
macro_rules! ok {
    ($body: expr) => {
        if let Ok(_) = $body {}
    };
}

#[macro_export]
macro_rules! some {
    ($body: expr) => {
        if let Some(_) = $body {}
    };
}