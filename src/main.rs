use clap::{arg, Arg, Command};
use mixtapetorrent::{
    formatter::{format_dj_results, format_mixtape_details, format_results},
    parser::Parser,
};

fn cli() -> Command<'static> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    Command::new("mixtapetorrent")
        .version(VERSION)
        .author("Tsiry Sandratraina <tsiry.sndr@aol.com>")
        .about(
            r#"

  __  __ _      _                _______                        _   
 |  \/  (_)    | |              |__   __|                      | |  
 | \  / |___  _| |_ __ _ _ __   ___| | ___  _ __ _ __ ___ _ __ | |_ 
 | |\/| | \ \/ / __/ _` | '_ \ / _ \ |/ _ \| '__| '__/ _ \ '_ \| __|
 | |  | | |>  <| || (_| | |_) |  __/ | (_) | |  | | |  __/ | | | |_ 
 |_|  |_|_/_/\_\\__\__,_| .__/ \___|_|\___/|_|  |_|  \___|_| |_|\__|
                        | |                                         
                        |_|                                         
       
Get the latest torrents from mixtapetorrent.com
"#,
        )
        .subcommand_required(true)
        /* .subcommand(
            Command::new("search").about("Search for mixtapes").arg(
                Arg::with_name("query")
                    .help("The query to search for")
                    .required(true)
                    .index(1),
            ),
        )*/
        .subcommand(Command::new("latest").about("Get the latest mixtapes"))
        .subcommand(Command::new("popular").about("Get the most popular mixtapes"))
        .subcommand(Command::new("dj").about("List all the DJs"))
        .subcommand(
            Command::new("info")
                .about("Get the details of a mixtape")
                .arg(
                    Arg::with_name("url")
                        .help("The url of the mixtape")
                        .required(true)
                        .index(1),
                ),
        )
        .arg(
            arg!(
                -j --json ... "output in json format"
            )
            .required(false),
        )
}

#[tokio::main]
async fn main() -> Result<(), surf::Error> {
    let matches = cli().get_matches();
    let parser = Parser::new();

    let output_in_json = matches.is_present("json");

    match matches.subcommand() {
        Some(("search", matches)) => {
            let query = matches.value_of("query").unwrap();
            parser.search_mixtapes(query).await?;
        }
        Some(("latest", _)) => format_results(parser.list_new_mixtapes().await?, output_in_json),
        Some(("dj", _)) => format_dj_results(parser.list_dj().await.unwrap(), output_in_json),
        Some(("popular", _)) => {
            format_results(parser.list_popular_mixtapes().await?, output_in_json)
        }
        Some(("info", matches)) => {
            let url = matches.value_of("url").unwrap();
            format_mixtape_details(parser.get_mixtape_details(url).await?, output_in_json);
        }
        _ => unreachable!(),
    }

    Ok(())
}
