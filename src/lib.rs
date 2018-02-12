extern crate data_encoding;
extern crate ring;

pub mod chunk;
pub mod bundle;
mod digestutils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
