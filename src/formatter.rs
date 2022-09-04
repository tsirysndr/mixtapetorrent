use colored_json::ToColoredJson;
use owo_colors::OwoColorize;

use crate::types::{Dj, Mixtape};

pub fn format_results(results: Vec<Mixtape>, json: bool) {
    if results.len() == 0 {
        println!("No mixtapes found.");
        return;
    }
    if json {
        println!(
            "{}",
            serde_json::to_string(&results)
                .unwrap()
                .to_colored_json_auto()
                .unwrap()
        );
        return;
    }
    results.iter().for_each(|mixtape| {
        println!("{} - {}", mixtape.title, mixtape.link.bright_green());
    });
}

pub fn format_dj_results(results: Vec<Dj>, json: bool) {
    if results.len() == 0 {
        println!("No mixtapes found.");
        return;
    }
    if json {
        println!(
            "{}",
            serde_json::to_string(&results)
                .unwrap()
                .to_colored_json_auto()
                .unwrap()
        );
        return;
    }
    results.iter().for_each(|dj| {
        println!("{} - {}", dj.name, dj.link.bright_green());
    });
}

pub fn format_mixtape_details(mixtape: Mixtape, json: bool) {
    if json {
        println!(
            "{}",
            serde_json::to_string(&mixtape)
                .unwrap()
                .to_colored_json_auto()
                .unwrap()
        );
        return;
    }
    println!("\n{} - {}\n", mixtape.title, mixtape.link.bright_green());
    println!("Tracks:");
    mixtape.tracks.iter().for_each(|track| {
        println!("{}", track.bright_green());
    });

    println!("\nCover: {}", mixtape.cover.bright_green());
    println!("Torrent: {}", mixtape.torrent.bright_green());
}
