use std::str::FromStr;

use abi_stable::std_types::RHashMap;

pub fn unwrap_arg<T: FromStr>(arg: &str, args: &RHashMap<String, String>) -> T {
    match args
        .get(arg)
        .unwrap_or_else(|| panic!("No argument named {} available in the args map", arg))
        .parse::<T>()
    {
        Ok(val) => val,
        Err(_) => panic!("Failed to parse param '{}'", arg),
    }
}
