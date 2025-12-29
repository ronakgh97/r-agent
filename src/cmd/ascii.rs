use colored::Colorize;

pub async fn run_ascii_art() {
    let ascii_art = r#"          
                                     ██   
    ████▄    ▀▀█▄ ▄████ ▄█▀█▄ ████▄ ▀██▀▀ 
    ██ ▀▀   ▄█▀██ ██ ██ ██▄█▀ ██ ██  ██   
    ██      ▀█▄██ ▀████ ▀█▄▄▄ ██ ██  ██   
                     ██                   
                   ▀▀▀                           
    "#;

    println!("{}\n\n", ascii_art.to_string().magenta());
    println!(
        " Github: {}\n",
        "https://github.com/ronakg/r-agent".to_string().cyan()
    );
}
