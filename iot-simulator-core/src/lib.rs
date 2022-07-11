extern crate core;

#[cfg(test)]
mod tests {
    use iot_simulator_api::generator::GeneratorPlugin;

    #[test]
    fn it_works() {
        let _: Result<String, Box<dyn std::error::Error>> = unsafe {
            let lib = libloading::Library::new(
                "/home/ntrp/_pws/iot-simulator-rs/target/debug/libiot_simulator_generator_faker.so",
            )
            .unwrap();
            let new_fn: libloading::Symbol<
                unsafe extern "C" fn() -> Box<dyn GeneratorPlugin<(), String>>,
            > = lib.get(b"new_instance").unwrap();
            let mut plugin = new_fn();
            Ok(plugin.generate(()))
        };

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod parser;