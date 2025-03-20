pub trait Direction {}

pub struct Left;
pub struct Right;
pub struct Up;
pub struct Down;

impl Direction for Left {}
impl Direction for Right {}
impl Direction for Up {}
impl Direction for Down {}
