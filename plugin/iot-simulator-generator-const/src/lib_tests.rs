#[cfg(test)]
mod tests {
    use super::super::*;
    use std::collections::HashMap;

    #[test]
    fn it_should_generate_the_value_set_at_init() {
        let val = "test".to_string();
        let plugin =
            unsafe { new_instance(RHashMap::from(HashMap::from([("val".into(), val.clone())]))) };
        let result = plugin.write().expect("Write lock failed").generate();

        assert_eq!(result, GenerationResult::Str(val.into()))
    }
}
