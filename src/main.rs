use reqwest::blocking::Client;
use std::{fs, process};
use regex::Regex;


fn main() {
    // Az egész dolog illegális, bár eleve illegálisan (ingyen) lehet nézni ezeket az animéket, szóval mindegy
    // A kód célja, hogy bemutassa milyen könnyű scrappelni egy weboldalt és, hogy mennyire egyszerű ezt megtenni pár oldalon, mint pl. az AnimeDrive

    // A kódot arra lehet használni, hogy kinyerjük az AnimeDrive videó linkjét egy adott anime epizódhoz
    // Az AnimeDrive egy anime streaming oldal, ami ingyenes és legálisan nézhető, bár a tartalmuk nem teljesen, de mindegy
    // Az AnimeDrive-on található anime epizódok linkjei nem közvetlenül elérhetők, hanem egy videoplayer linken keresztül,
    // de ez a player rendelkezik egy "letöltés" funkcióval, ami a videó linkjét adja vissza
    // Tudni kell, hogy a player betöltése elég lassú, és lehet rate limitelve is van. Nehogy sok requestet küldjünk egyszerre, mert letilthatják a IP címünket


    // http kliens létrehozása 2 perces timeout idővel, az oldal elég lassan töltbe, így az alap 30 másodperces timeout nem elég
    let client = Client::builder().timeout(std::time::Duration::new(120, 0)).build().unwrap();

    // MyAnimeList ID-je a Frieren-nek
    let mal_id: u64 = 52991;
    // Az epizód száma, amit le akarunk kérni
    let episode: u64 = 8;

    
    // AnimeDrive link generálása
    let anime_drive_link = anime_drive_link(&client, mal_id, episode);
    // Ezt fogjuk visszakapni: https://player.animedrive.hu/player_wee.php?id=2603&amp;ep=1

    // A videó html kódjának lekérése
    // Ez a kód tartalmazza a videó linket, amit a player betölt
    let video_html = get_html(&client, anime_drive_link);

    let video_link = extract_highest_quality(&video_html);
    // A legjobb minőségű videó link kinyerése a html kódból
    match video_link {
        Some(link) => {
            println!("Videó link: {}", link);
            // A videó link kiírása fájlba, hogy később is elérhető legyen
            fs::write("video_link.txt", &link).expect("Nem sikerült a fájl írása");
        }
        None => {
            eprintln!("Nem található videó link");
            process::exit(1);
        }
    }
}

fn anime_drive_link(client: &Client, mal_id: u64, episode: u64) -> String {
    // MyAnimeList link csinálása ID alapján
    let mal_link = format!("https://myanimelist.net/anime/{}", mal_id);
    // "Keresés" AnimeDrive-on, ami a MyAnimeList linket használja
    let search_url = format!("https://animedrive.hu/search/?q={}", mal_link);
    // Ezt fogjuk kapni: https://animedrive.hu/search/?q=https://myanimelist.net/anime/52991
    // Ez a link valójában egyből redirectál a megfelelő anime oldalra -> erre: https://animedrive.hu/anime/?id=2603

    // HTTP GET kérés küldése
    let response = client.get(&search_url).send().unwrap();
    // Az url megszerzése
    let final_url = response.url().to_string();
    // A végső url: https://animedrive.hu/anime/?id=2603
    // Az id kinyerése a végső url-ből
    let id = final_url.split("id=").nth(1).unwrap_or("0").split("&").next().unwrap_or("0");

    println!("AnimeDrive ID: {}", id);

    // Ha nem található az anime az AnimeDrive-on, akkor hibaüzenet
    if id == "0" {
        eprintln!("Nem található az anime az AnimeDrive-on");
        process::exit(1);
    }

    let url = format!("https://player.animedrive.hu/player_wee.php?id={}&amp;ep={}", id, episode);
    println!("Player link: {}", url);
    url.to_string()
}

fn get_html(client: &Client, player_link: String) -> String {
    // A linkünk jelenleg: https://player.animedrive.hu/player_wee.php?id=2603&amp;ep=1
    // Ebből a linkből kell kinyerni a videó linket reverse engineering segítségével

    // HTTP GET kérés küldése, majd szöveggé alakítása
    let response = client.get(&player_link).send().unwrap();

    // A válasz teljes kódja, css, html, minden
    let text = response.text().unwrap();

    // A válasz kiírása fájlba, hogy lássuk mi van benne
    fs::write("response.html", &text).expect("Nem sikerült a fájl írása");

    // A videó link kinyerése
    // A link egy js fájlban található, amit a player betölt. Ez a fájl tartalmazza a videó linket és a minőséget is
    text
}

fn extract_highest_quality(js_code: &str) -> Option<String> {

    // A js fájlban található a videó link és a minőség is
    // A link a "src" kulcsszó után található, a minőség pedig a "size" kulcsszó után
    // A regex kifejezés, ami kinyeri a linket és a minőséget
    let re = Regex::new(r#"src:\s*'([^']+)'.*?size:\s*(\d+)"#).unwrap();
    let mut best = None;

    // A regex kifejezés segítségével kinyerjük a linket és a minőséget és megkeressük a legjobbat
    for cap in re.captures_iter(js_code) {
        let url = cap.get(1)?.as_str();
        let size: u32 = cap.get(2)?.as_str().parse().ok()?;

        if best.as_ref().map_or(true, |(_, s)| size > *s) {
            best = Some((url.to_string(), size));
        }
    }

    best.map(|(url, _)| url)
}