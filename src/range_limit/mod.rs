pub mod serde;
pub mod limits;
pub mod range_limit;

pub trait RangeBound: Default {
    fn match_range(input: usize) -> SizeStatus;
}

pub enum SizeStatus {
    Ok,
    TooLarge(usize),
    TooSmall(usize),
    FIxSize(usize),
    Custom(Box<dyn std::error::Error>),
}

impl SizeStatus {
    pub fn custom<E: std::error::Error + 'static>(err: E) -> Self {
        let b = Box::new(err) as Box<dyn std::error::Error>;
        Self::Custom(b)
    }
}
