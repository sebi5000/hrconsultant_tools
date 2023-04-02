#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{api::dialog::FileDialogBuilder, Window};
use calamine::{open_workbook, Xlsx, Reader};
use std::{thread, time};

fn main() {  
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![choose_file])
    .build(tauri::generate_context!())
    .expect("Error while starting HR Consultant Tools")
    .run(|_app, _event|{
    })
}

#[derive(Clone, serde::Serialize)]
struct Payload{
  status: String
}

#[tauri::command]
fn choose_file(window: Window){

  FileDialogBuilder::new().pick_file(|path|{
      let workbook: Xlsx<_> = open_workbook(path.unwrap().as_os_str()).unwrap();
      let sheet_name = workbook.sheet_names().get(0);

      match sheet_name {
        Some(_x) => {
          let two_seconds = time::Duration::from_millis(2000);
          thread::sleep(two_seconds);
          let result = String::from("Ok");
          let fire_upload_done = move || window.emit("chooseFileDoneEvent", Payload{ status: result }).unwrap();
          fire_upload_done();
        },
        None => println!("No Sheetname")
      }
  });
}
