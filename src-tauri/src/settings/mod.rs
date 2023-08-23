use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::Write;

#[derive(Deserialize, Serialize, Debug)]
struct ToServer {
    server: String,
    port: i32,
    user_id: i32,
}
fn getSettingPath() -> String {
    String::from("to_server.json")
}
fn getToServer() -> ToServer {
    if let Ok(file_content) = fs::read_to_string(getSettingPath()) {
        let to_server_res: Result<ToServer, serde_json::Error> =
            serde_json::from_str(&file_content);
        if let Ok(mut to_server) = to_server_res {
            return to_server;
        } else {
            let to_server = ToServer {
                server: String::from(
                    "https://holisticscs.cloud",
                ),
                port: 8080,
                user_id: 0,
            };
            if let Ok(res) = save_to_json(&to_server) {
                return to_server;
            }
        }
    }
    ToServer {
        server: String::from("https://holisticscs.cloud"),
        port: 5177,
        user_id: 0,
    }
}

fn save_to_json(to_server: &ToServer) -> std::io::Result<()> {
    let out_path = getSettingPath();
    // Serialize the struct to a JSON string
    let data = serde_json::to_string(to_server).unwrap();

    // Open the file in write mode
    let mut file = fs::File::create(out_path)?;

    // Write the JSON data to the file
    file.write_all(data.as_bytes())?;

    // Return Ok when everything went well
    Ok(())
}

#[tauri::command]
pub fn save_to_server(server: String, port: i32, user_id: i32) -> usize {
    if let Ok(_) = save_to_json(&ToServer {
        server,
        port,
        user_id,
    }) {
        return 1;
    }
    0
}
#[tauri::command]
pub fn get_to_server() -> (String, i32, i32) {
    let to_server = getToServer();
    (to_server.server, to_server.port, to_server.user_id)
}
