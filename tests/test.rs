#[cfg(test)]
mod tests {
    use evm_bunnyhop::bunny_hop;

    #[test]
    fn test_simple() {
        let bytecode = "610004565b5f5ff3";

        let fixed = bunny_hop(bytecode);
        assert_eq!(fixed, "6003565b5f5ff3");
    }
}
