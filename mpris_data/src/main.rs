use mpris::PlayerFinder;
use reqwest::blocking::{Client, get};
use std::env;
use std::fs::{copy, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use std::str;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_metadata(None);
    } else if args.len() == 2 {
        print_metadata(Some(&args[1]));
    } else if args.len() == 3 {
        let data = print_metadata(Some(&args[1]));
        match data {
            Some(x) => notify(x),
            None => abortify(),
        }
    }
}

fn print_metadata(name: Option<&str>) -> Option<[String; 3]> {
    let name = match name {
        Some(x) => x,
        None => "",
    };

    if name=="RemoteSpotify" {
        fetch_spt();
        return None
    }

    let player_finder = PlayerFinder::new().unwrap();

    for player in player_finder.find_all().unwrap() {
        if player.identity() == name {
            let mut title: String;
            let mut singstr: String;
            let displaydur: String;

            let metadata = player.get_metadata().unwrap();

            if name == "Spotify" {
                title = metadata.title().unwrap().to_string();

                if title.chars().count() > 25 as usize {
                    let mut newtitle = String::new();
                    for (i, chara) in title.chars().into_iter().enumerate() {
                        if i > 22 {
                            break;
                        }
                        newtitle.push(chara);
                    }
                    newtitle.push_str("...");
                    title = newtitle;
                }

                println!("{}", title);

                singstr = String::new();
                for singer in metadata.artists().unwrap() {
                    &mut singstr.push_str(singer);
                    &mut singstr.push_str(",");
                }
                singstr.pop();

                if singstr.chars().count() > 35 as usize {
                    let mut newsingers = String::new();
                    for (i, chara) in singstr.chars().into_iter().enumerate() {
                        if i > 32 {
                            break;
                        }
                        newsingers.push(chara);
                    }
                    newsingers.push_str("...");
                    singstr = newsingers;
                }

                singstr = format!("artist={}", singstr);

                let _ = Command::new("eww")
                    .arg("update")
                    .arg(&singstr)
                    .spawn();

            } else {
                singstr = std::fs::read_to_string("/home/pradyungn/.config/eww/data/mpartist")
                    .expect("Oopsies!!!");

                title = std::fs::read_to_string("/home/pradyungn/.config/eww/data/mptitle")
                    .expect("Oopsies!!!");

                if singstr.chars().count() > 35 as usize {
                    let mut newsingers = String::new();
                    for (i, chara) in singstr.chars().into_iter().enumerate() {
                        if i > 32 {
                            break;
                        }
                        newsingers.push(chara);
                    }
                    newsingers.push_str("...");
                    singstr = newsingers;
                }

                let _ = Command::new("eww")
                    .arg("update")
                    .arg(format!("artist={}", singstr))
                    .spawn();

                if title.chars().count() > 25 as usize {
                    let mut newtitle = String::new();
                    for (i, chara) in title.chars().into_iter().enumerate() {
                        if i > 22 {
                            break;
                        }
                        newtitle.push(chara);
                    }
                    newtitle.push_str("...");
                    title = newtitle;
                }
                println!("{}", title);
            }

            let duration = metadata.length().unwrap().as_secs();

            let position = player.get_position().unwrap().as_secs();

            let seconds = duration % 60;
            if seconds < 10 {
                displaydur = format!("{}:0{}", duration / 60, seconds);
            } else {
                displaydur = format!("{}:{}", duration / 60, seconds);
            }

            let seconds = &position % 60;
            let displsec: String;
            if seconds < 10 {
                displsec = format!("0{}", seconds);
            } else {
                displsec = format!("{}", seconds)
            }

            let status = format!("{:#?}", player.get_playback_status().unwrap());

            let _ = Command::new("eww")
                .arg("update")
                .arg(format!("playing={}", status))
                .arg(format!("duration={}", duration))
                .arg(format!("displaypos={}:{}", &position / 60, displsec))
                .arg(format!("displaydur={}", displaydur))
                .arg(format!("position={}", position))
                .spawn();

            if name == "Music Player Daemon" {
                let url = match metadata.art_url() {
                    Some(x) => &x[7..],
                    None => "/home/pradyungn/Pictures/forest.jpg",
                };

                let _ = copy(
                    Path::new(url),
                    Path::new("/home/pradyungn/.config/eww/temp.jpg"),
                );
            } else {
                let url = metadata.art_url().unwrap();

                if url!="" {
                    let res = get(url).unwrap().bytes().unwrap();
                    let mut file =
                        File::create("/home/pradyungn/.config/eww/temp.jpg").expect("create failed");
                    file.write(&res).expect("write failed");
                } else {
                    let _ = copy(
                        Path::new("/home/pradyungn/Pictures/forest.jpg"),
                        Path::new("/home/pradyungn/.config/eww/temp.jpg"),
                    );
                }
            }

            return Some([title, singstr, displaydur]);
        }
    }

    let _ = Command::new("eww")
        .arg("update")
        .arg("artist=")
        .arg("title=Nothing playing right now...")
        .arg("duration=0")
        .arg("position=0")
        .arg("displaydur=")
        .arg("displaypos=")
        .arg("playing=Playing")
        .spawn();

    return None;
}

fn notify(data: [String; 3]) {
    let _ = Command::new("notify-send")
        .arg("-i")
        .arg("/home/pradyungn/.config/eww/temp.jpg")
        .arg("Now Playing ♫")
        .arg(format!("{}\n{} // {}", data[0], data[1], data[2]))
        .spawn();
}

fn abortify() {
    let _ = Command::new("notify-send")
        .arg("Issues Fetching Data Ya Boomer")
        .spawn();
}

fn fetch_spt() {
    let title = Command::new("spt")
        .arg("pb")
        .arg("-f").arg("%t").output().expect("Something went wrong with spt fetch");

    let mut title = match str::from_utf8(&title.stdout) {
        Ok(s) => s,
        Err(_) => panic!("Ay papi something went wrong with string conversion!!!")
    }.strip_suffix("\n").unwrap().to_string();

    if title.chars().count() > 25 as usize {
        let mut newtitle = String::new();
        for (i, chara) in title.chars().into_iter().enumerate() {
            if i > 22 {
                break;
            }
            newtitle.push(chara);
        }
        newtitle.push_str("...");
        title = newtitle;
    }

    println!("{}", title);

    let artists = Command::new("spt")
        .arg("pb")
        .arg("-f").arg("%a").output().expect("Spt fetch went ooga booga");

    let mut artists = match str::from_utf8(&artists.stdout) {
        Ok(s) => s,
        Err(_) => panic!("Ay papi something went wrong with string conversion!!!")
    }.strip_suffix("\n").unwrap().to_string();

    if artists.chars().count() > 35 as usize {
        let mut newsingers = String::new();
        for (i, chara) in artists.chars().into_iter().enumerate() {
            if i > 32 {
                break;
            }
            newsingers.push(chara);
        }
        newsingers.push_str("...");
        artists = newsingers;
    }

    artists = format!("artist={}", artists);

    let _ = Command::new("eww")
        .arg("update")
        .arg(&artists)
        .spawn();

    let id = Command::new("spt")
        .arg("pb")
        .arg("--share-track").output().expect("Spt fetch went ooga booga");

    let id = match str::from_utf8(&id.stdout) {
        Ok(s) => s,
        Err(_) => panic!("couldn't decode link")
    }.replace("https://open.spotify.com/track/", "");

    if id != "" {
        let authid = "BQCGdAz1EOV_tNTzZl272hC7YkpGXmMBYeqfwJs5MGrag6DdTjHSjI-Yz_RK6AnFe_rO0eO5ex8V54cYZ5jjsLfrPoA8NQDuVuDym8eFxk-w_Sk4ivKksTYESM6WtV4Yf9PeLvdrDET11sja0hQ";
        let client = Client::new();

        let data = client.get(format!("https://api.spotify.com/v1/tracks/{}",id))
            .header(AUTHORIZATION, format!("Bearer {}", authid))
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json").send().unwrap();

        let data: Value = serde_json::from_str(&data.text().unwrap()).expect("ay caramba");

        let img = get(&data["album"]["images"][1]["url"].to_string().replace("\"","")).expect("image fetch failed").bytes().unwrap();

        let mut file =
            File::create("/home/pradyungn/.config/eww/temp.jpg").expect("create failed");
        file.write(&img).expect("write failed");
    } else {
        let _ = copy(
            Path::new("/home/pradyungn/Pictures/forest.jpg"),
            Path::new("/home/pradyungn/.config/eww/temp.jpg"),
        );
    }
}
