pub fn print_side_by_side_table(
    data_modules: &[String],
    domain_modules: &[String],
    feature_modules: &[String],
    other_modules: &[String],
) {
    // Tentukan jumlah maksimum baris berdasarkan panjang data terpanjang di semua kolom
    let max_rows = data_modules
        .len()
        .max(domain_modules.len())
        .max(feature_modules.len())
        .max(other_modules.len());

    // Hitung lebar maksimum untuk setiap kolom
    let data_col_width = data_modules
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0)
        .max(12); // "Data Modules".len()
    let domain_col_width = domain_modules
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0)
        .max(14); // "Domain Modules".len()
    let feature_col_width = feature_modules
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0)
        .max(15); // "Feature Modules".len()
    let other_col_width = other_modules
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0)
        .max(13); // "Other Modules".len()

    // Cetak header tabel
    println!(
        "+-{:-<data_width$}-+-{:-<domain_width$}-+-{:-<feature_width$}-+-{:-<other_width$}-+",
        "",
        "",
        "",
        "",
        data_width = data_col_width,
        domain_width = domain_col_width,
        feature_width = feature_col_width,
        other_width = other_col_width,
    );
    println!(
        "| {:<data_width$} | {:<domain_width$} | {:<feature_width$} | {:<other_width$} |",
        "Data Modules",
        "Domain Modules",
        "Feature Modules",
        "Other Modules",
        data_width = data_col_width,
        domain_width = domain_col_width,
        feature_width = feature_col_width,
        other_width = other_col_width,
    );
    println!(
        "+-{:-<data_width$}-+-{:-<domain_width$}-+-{:-<feature_width$}-+-{:-<other_width$}-+",
        "",
        "",
        "",
        "",
        data_width = data_col_width,
        domain_width = domain_col_width,
        feature_width = feature_col_width,
        other_width = other_col_width,
    );

    // Cetak baris data sesuai jumlah data maksimum
    for i in 0..max_rows {
        let data_module = data_modules.get(i).map(String::as_str).unwrap_or("");
        let domain_module = domain_modules.get(i).map(String::as_str).unwrap_or("");
        let feature_module = feature_modules.get(i).map(String::as_str).unwrap_or("");
        let other_module = other_modules.get(i).map(String::as_str).unwrap_or("");

        println!(
            "| {:<data_width$} | {:<domain_width$} | {:<feature_width$} | {:<other_width$} |",
            data_module,
            domain_module,
            feature_module,
            other_module,
            data_width = data_col_width,
            domain_width = domain_col_width,
            feature_width = feature_col_width,
            other_width = other_col_width,
        );
    }

    // Cetak footer tabel
    println!(
        "+-{:-<data_width$}-+-{:-<domain_width$}-+-{:-<feature_width$}-+-{:-<other_width$}-+",
        "",
        "",
        "",
        "",
        data_width = data_col_width,
        domain_width = domain_col_width,
        feature_width = feature_col_width,
        other_width = other_col_width,
    );
}
