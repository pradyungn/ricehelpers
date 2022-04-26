use std::process::Command;

fn main() {
    let activeworkspace = Command::new("herbstclient")
        .arg("sprintf")
        .arg("X")
        .arg("%s")
        .arg("tags.focus.index")
        .arg("echo")
        .arg("X").output().expect("Weird output error");

    let activewinid = Command::new("herbstclient")
        .arg("sprintf")
        .arg("X")
        .arg("%s")
        .arg("clients.focus.winid")
        .arg("echo")
        .arg("X").output().expect("Weird output error");

    let activewinid = String::from_utf8_lossy(&activewinid.stdout);

    let windows = Command::new("wmctrl")
        .arg("-l").output().expect("Well I missed up again...");

    let activeworkspace = String::from_utf8_lossy(&activeworkspace.stdout);
    let windows = String::from_utf8_lossy(&windows.stdout);

    let mut curwins = Vec::new();
    for window in windows.lines(){
        if window.as_bytes()[12]==activeworkspace.as_bytes()[0] {
            let winid = window.get(0..10).unwrap();
            let name = window.split_at(20).1;
            curwins.push([winid, name]);
        }
    }

    let mut xml = String::from("<box orientation='h' class='windows' space-evenly='false'>");

    for win in curwins {
        let title;
        if win[1].chars().count()>15 {
            title=format!("{}...", win[1].get(0..12).unwrap());
        }
        else {title=String::from(win[1]);}
        let class = if win[0].get(3..10)==activewinid.get(2..9) {"active"} else {"normal"};
        let rowxml = format!("<button onclick='herbstclient jumpto {}' class='{}'>{}</button>", win[0], class, title);
        xml.push_str(&rowxml);
    }
    xml.push_str("</box>");

    let _ = Command::new("eww")
        .arg("update")
        .arg(format!("winxml={}", xml))
        .spawn();
}
