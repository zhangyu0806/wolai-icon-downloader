use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Emitter;

#[derive(Serialize, Deserialize)]
pub struct DownloadRequest {
    start_date: String,
    end_date: String,
    color: String,
    save_dir: String,
}

#[derive(Clone, Serialize)]
struct DownloadProgress {
    current: u32,
    total: u32,
    filename: String,
    success: bool,
}

#[tauri::command]
async fn download_icons(app: tauri::AppHandle, request: DownloadRequest) -> Result<String, String> {
    let start = NaiveDate::parse_from_str(&request.start_date, "%Y-%m-%d")
        .map_err(|e| format!("起始日期格式错误: {}", e))?;
    let end = NaiveDate::parse_from_str(&request.end_date, "%Y-%m-%d")
        .map_err(|e| format!("结束日期格式错误: {}", e))?;

    if start > end {
        return Err("起始日期不能晚于结束日期".to_string());
    }

    let save_dir = PathBuf::from(&request.save_dir);
    if !save_dir.exists() {
        fs::create_dir_all(&save_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let total_days = (end - start).num_days() as u32 + 1;
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    let mut success_count = 0u32;
    let mut current_date = start;
    let mut index = 0u32;

    while current_date <= end {
        index += 1;
        let year = current_date.format("%Y").to_string();
        let date_str = current_date.format("%Y-%m-%d").to_string();
        let filename = current_date.format("%m-%d").to_string();

        let year_dir = save_dir.join(&year);
        if !year_dir.exists() {
            fs::create_dir_all(&year_dir).map_err(|e| format!("创建年份目录失败: {}", e))?;
        }

        let file_path = year_dir.join(format!("{}.svg", filename));

        if file_path.exists() {
            let _ = app.emit(
                "download-progress",
                DownloadProgress {
                    current: index,
                    total: total_days,
                    filename: format!("{}/{}.svg", year, filename),
                    success: true,
                },
            );
            success_count += 1;
            current_date += chrono::Duration::days(1);
            continue;
        }

        let url = format!(
            "https://api.wolai.com/v1/icon?type=2&locale=cn&date={}&pro=0&color={}",
            date_str, request.color
        );

        let success = match client.get(&url).send().await {
            Ok(resp) => match resp.text().await {
                Ok(body) if body.starts_with("<svg") => {
                    fs::write(&file_path, &body).is_ok()
                }
                _ => false,
            },
            Err(_) => false,
        };

        if success {
            success_count += 1;
        }

        let _ = app.emit(
            "download-progress",
            DownloadProgress {
                current: index,
                total: total_days,
                filename: format!("{}/{}.svg", year, filename),
                success,
            },
        );

        current_date += chrono::Duration::days(1);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    Ok(format!(
        "下载完成！成功 {}/{} 个图标",
        success_count, total_days
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![download_icons])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
