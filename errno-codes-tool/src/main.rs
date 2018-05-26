extern crate errno_codes;

fn main() -> Result<(), String> {
    let mut args = std::env::args();
    let exe_name = args.next().ok_or_else(|| "Unable to get exe name (argv[0]).")?;

    if args.len() != 1 {
        eprintln!("Usage:");
        eprintln!("    {} <errno number | errno identifier>", exe_name);

        return Err("Invalid command line arguments".into());
    };

    let num_or_id = args.next().expect("Logic error: no num_or_id");

    let result = if let Ok(num) = num_or_id.parse::<std::os::raw::c_int>() {
        errno_codes::search_by_num(errno_codes::QueryFamily::Any, num)
    } else {
        errno_codes::search_by_id(errno_codes::QueryFamily::Any, &num_or_id)
    };
    if result.is_empty() {
        return Err(format!("No errno information found for {}", num_or_id));
    }

    result.iter().for_each(|(family, errno_code)| {
        println!("{} ({})", errno_code, family);
    });

    Ok(())
}
