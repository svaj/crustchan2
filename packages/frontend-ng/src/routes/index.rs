// src/routes/about.rs
use tuono_lib::{Request, Response, Props};
use serde_json::{from_str,Value};
use std::fs;

#[tuono_lib::handler]
async fn get_board_list(_req: Request) -> Response {
  let parse_result: Value = from_str(&fs::read_to_string("fixtures/boardList.json").expect("Failed to read json file")).unwrap();
  dbg!(&parse_result);
  Response::Props(Props::new(parse_result))
}