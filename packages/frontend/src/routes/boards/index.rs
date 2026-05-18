use serde_json::{Value, from_str};
use std::fs;
use tuono_lib::{Props, Response};

#[tuono_lib::handler]
async fn get_board_list() -> Response {
    let parse_result: Value =
        from_str(&fs::read_to_string("fixtures/boardList.json").expect("Failed to read json file"))
            .unwrap();
    dbg!(&parse_result);
    Response::Props(Props::new(parse_result))
}
