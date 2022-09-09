#![warn(
    missing_docs,
    unreachable_pub,
    clippy::pedantic,
    clippy::nursery,
    clippy::undocumented_unsafe_blocks
)]
#![allow(clippy::redundant_pub_crate)] // This would otherwise clash with `unreachable_pub`
#![allow(clippy::module_name_repetitions)]

mod kind;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
