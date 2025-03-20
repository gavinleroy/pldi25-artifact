pub trait AsSchedule {}

macro_rules! schedules {
    ($($name:ident),*) => {
        $(
        pub struct $name;
        impl AsSchedule for $name {}
        )*
    };
}

schedules! {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly
}
