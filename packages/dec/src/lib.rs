pub mod response;
pub mod msg;

pub use crate::msg::{ HouseHandleMsg, HouseQueryMsg };
pub use crate::response::{ HouseResponse };

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
