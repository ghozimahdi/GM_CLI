pub fn print_side_by_side_table(
    data_modules: &[String],
    domain_modules: &[String],
    feature_modules: &[String],
    other_modules: &[String],
) {
    let max_rows = 30;

    println!(
        "+------------------------------+------------------------------+------------------------------+------------------------------+"
    );
    println!(
        "| {:<28} | {:<28} | {:<28} | {:<28} |",
        "Data Modules", "Domain Modules", "Feature Modules", "Other Modules"
    );
    println!(
        "+------------------------------+------------------------------+------------------------------+------------------------------+"
    );

    for i in 0..max_rows {
        let data_module = data_modules.get(i).map(String::as_str).unwrap_or("");
        let domain_module = domain_modules.get(i).map(String::as_str).unwrap_or("");
        let feature_module = feature_modules.get(i).map(String::as_str).unwrap_or("");
        let other_module = other_modules.get(i).map(String::as_str).unwrap_or("");

        if !data_module.is_empty()
            || !domain_module.is_empty()
            || !feature_module.is_empty()
            || !other_module.is_empty()
        {
            println!(
                "| {:<28} | {:<28} | {:<28} | {:<28} |",
                data_module, domain_module, feature_module, other_module
            );
        }
    }

    println!(
        "+------------------------------+------------------------------+------------------------------+------------------------------+"
    );
}
