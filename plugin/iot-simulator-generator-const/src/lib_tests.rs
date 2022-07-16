#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn it_should_generate_the_value_set_at_init() {
        let val = "test".to_string();
        let plugin = unsafe { new_instance(HashMap::from([("val".to_string(), val.clone())])) };
        let result = plugin.write().expect("Write lock failed").generate(Utc::now());

        assert_eq!(result, GenerationResult::Str(val))
    }
}
