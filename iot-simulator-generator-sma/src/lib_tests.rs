#[cfg(test)]
mod lib_tests {
    use super::super::{*};

    #[test]
    fn it_should_generate() {
        let mut plugin = new_instance(10.0, 20.0, 3, 5);

        assert!(plugin.generate(()) > 10.0);
        assert!(plugin.generate(()) < 20.0);
    }
}