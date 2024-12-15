use colored::*;

pub fn print_header() {
    let ko_fi_link = "https://ko-fi.com/ghozimahdi";
    let trakteer_link = "https://teer.id/ghozimahdi";
    let github_sponsor_link = "https://github.com/sponsors/mbahgojol";
    let saweria_link = "https://saweria.co/ghozimahdi";

    let header = format!(
        "\
Ghozi Mahdi CLI tool for Flutter project management

Created by   : Ghozi Mahdi
Occupation   : Mobile Software Engineer
LinkedIn     : https://id.linkedin.com/in/ghozimahdi
YouTube      : https://www.youtube.com/@mbahgojol
Github       : https://github.com/mbahgojol
Email        : ghozimahdi@carpkingdom.com
Website      : www.mbahgojol.com

Support Me
Ko-fi           : {}
Trakteer        : {}
Github Sponsor  : {}
Saweria         : {}
",
        ko_fi_link, trakteer_link, github_sponsor_link, saweria_link
    );

    println!("{}", header.green());
}
