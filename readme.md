# AnimeDrive Scraper – Videó link kinyerő

Ez egy **bemutató célú** Rust program, amely megmutatja, mennyire **egyszerű lehet egy weboldalt scrappelni**, jelen esetben az [AnimeDrive.hu](https://animedrive.hu/) oldalt.

> ⚠️ **Jogi nyilatkozat:** Ez a kód *csak oktatási céllal* készült. Az AnimeDrive nem biztos, hogy legálisan tesz közzé tartalmakat, és a scraping technikák használata a felhasználási feltételek megsértését jelentheti.  
> A kódot csak saját felelősségre használd!

---

## 🧠 Mit csinál ez a program?

- Egy adott anime (MyAnimeList ID alapján) és epizód szám alapján:
  1. Lekéri az adott anime AnimeDrive ID-jét.
  2. Összerakja a videoplayer linket.
  3. Betölti a videoplayer HTML kódját.
  4. Kinyeri belőle a legjobb minőségű videó linket.
  5. Ezt kiírja a `video_link.txt` fájlba.

---

## ⚙️ Függőségek

A kód használatához a következő crate-ek szükségesek:

```toml
# Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
regex = "1.7"
```

---

## 📦 Build és futtatás

1. Klónozd vagy másold le a kódot egy Rust projektbe.
2. Add hozzá a fent említett függőségeket a `Cargo.toml`-hoz.
3. Buildeld és futtasd:

```bash
cargo run
```

A kimenet a `video_link.txt` fájlban lesz, ami tartalmazza a videó közvetlen elérhetőségét.

---

## 💡 Példa

A kódban példaként a *Sousou no Frieren* (MAL ID: `52991`) 8. epizódja van beállítva. Ez a rész automatikusan megkeresésre, betöltésre és feldolgozásra kerül.

Részlet a működésből:

```rust
let mal_id: u64 = 52991;
let episode: u64 = 8;

let anime_drive_link = anime_drive_link(&client, mal_id, episode);
let video_html = get_html(&client, anime_drive_link);
let video_link = extract_highest_quality(&video_html);

match video_link {
    Some(link) => {
        println!("Videó link: {}", link);
        fs::write("video_link.txt", &link).expect("Nem sikerült a fájl írása");
    }
    None => {
        eprintln!("Nem található videó link");
        process::exit(1);
    }
}
```

---

## 🔍 Hogyan működik?

### 1. AnimeDrive ID lekérése

A MyAnimeList link alapján történik egy keresés az AnimeDrive-on, ami automatikusan átirányít a megfelelő oldalra. Az `id` paraméter innen nyerhető ki.

```rust
let search_url = format!("https://animedrive.hu/search/?q=https://myanimelist.net/anime/{}", mal_id);
let response = client.get(&search_url).send().unwrap();
let final_url = response.url().to_string();
```

### 2. Player oldal lekérése

A kapott `id`-ből generáljuk a `player_wee.php` linket, ami betölti a videót.

```rust
let url = format!("https://player.animedrive.hu/player_wee.php?id={}&amp;ep={}", id, episode);
```

### 3. HTML kinyerés

A teljes HTML letöltésre kerül, majd fájlba is íródik (`response.html`), hogy akár kézzel is elemezhető legyen.

### 4. Videó link kinyerés (Regex)

A JavaScript kódból egy regex segítségével kiszűrjük a videó linkeket és a hozzájuk tartozó felbontást. A legnagyobb felbontású link kerül kiválasztásra:

```rust
let re = Regex::new(r#"src:\s*'([^']+)'.*?size:\s*(\d+)"#).unwrap();
```

---

## 📁 Kimeneti fájlok

- `video_link.txt` – A végső videó URL
- `response.html` – A player oldal teljes HTML kódja (debug célra)

---

## ❗ Fontos megjegyzések

- A videoplayer oldal nagyon lassan tölt be, így a HTTP kliens timeoutja 2 percre van állítva.
- Ne küldj túl sok lekérést egyszerre, mert az oldal letilthatja az IP címedet.

---

Készítette: Egy unatkozó Rust fejlesztő 🤓