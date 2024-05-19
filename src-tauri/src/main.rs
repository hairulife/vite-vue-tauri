// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(serde::Serialize)]
struct CustomResponse {
    server_addr: String,
    server_port: String,
    name: String,
    local_ip: String,
    local_port: String,
    remote_port: String,
}

#[derive(serde::Serialize)]
struct ResultResponse {
    stdout: String,
    stderr: String,
}

// 获取配置文件
#[tauri::command]
fn get_config() -> Result<CustomResponse, String> {
    let frpc_toml = std::fs::read_to_string("./frp_0.58.0_windows_amd64/frpc.toml").unwrap();
    let mut server_addr = "";
    let mut server_port = "";
    let mut name = "";
    let mut local_ip = "";
    let mut local_port = "";
    let mut remote_port = "";
    for line in frpc_toml.lines() {
        if line.starts_with("serverAddr") {
            server_addr = line.split("=").collect::<Vec<&str>>()[1].trim();
        } else if line.starts_with("serverPort") {
            server_port = line.split("=").collect::<Vec<&str>>()[1].trim();
        } else if line.starts_with("name") {
            name = line.split("=").collect::<Vec<&str>>()[1].trim();
        } else if line.starts_with("localIP") {
            local_ip = line.split("=").collect::<Vec<&str>>()[1].trim();
        } else if line.starts_with("localPort") {
            local_port = line.split("=").collect::<Vec<&str>>()[1].trim();
        } else if line.starts_with("remotePort") {
            remote_port = line.split("=").collect::<Vec<&str>>()[1].trim();
        }
    }
    // 去掉引号
    server_addr = server_addr.trim_matches('"');
    name = name.trim_matches('"');
    local_ip = local_ip.trim_matches('"');
    Ok(CustomResponse {
        server_addr: server_addr.to_string(),
        server_port: server_port.to_string(),
        name: name.to_string(),
        local_ip: local_ip.to_string(),
        local_port: local_port.to_string(),
        remote_port: remote_port.to_string(),
    })
}

#[tauri::command]
fn save_config(
    server_addr: String,
    server_port: String,
    name: String,
    local_ip: String,
    local_port: String,
    remote_port: String,
) -> String {
    // 修复配置文件
    let frpc_toml = format!(
        r#"serverAddr = "{}"
serverPort = {}

[[proxies]]
name = "{}"
type = "tcp"
localIP = "{}"
localPort = {}
remotePort = {}
"#,
        server_addr, server_port, name, local_ip, local_port, remote_port
    );
    std::fs::write("./frp_0.58.0_windows_amd64/frpc.toml", frpc_toml).unwrap();
    format!("保存成功")
}

#[tauri::command]
fn run_frpc() -> Result<(), String> {
    // 启动frpc
    std::process::Command::new("./frp_0.58.0_windows_amd64/frpc.exe")
        .current_dir("./frp_0.58.0_windows_amd64")
        .args(&["-c", "frpc.toml"])
        .spawn()
        .map_err(|e| e.to_string())?;

    // 这里我们不等待子进程结束，所以不会阻塞主线程
    // 如果你需要在子进程结束后处理它的输出，你可以保存 `child` 对象，并在适当的时候调用 `child.wait()` 或 `child.wait_with_output()`

    Ok(())
}

#[tauri::command]
fn stop_frpc() -> Result<ResultResponse, String> {
    // 停止frpc
    let output = std::process::Command::new("taskkill")
        .args(&["/f", "/im", "frpc.exe"])
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    Ok(ResultResponse {
        stdout: stdout.to_string(),
        stderr: stderr.to_string(),
    })
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            run_frpc,
            stop_frpc
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
