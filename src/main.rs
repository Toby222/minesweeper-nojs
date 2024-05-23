// If changed, also change error message
const MAX_SIZE: usize = 50;
const PORT: u16 = 55782;
const STYLESHEET: &[u8; 776] = include_bytes!("../style.css");
const FAVICON: &[u8; 111468] = include_bytes!("../favicon.ico");

use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    server::conn::http1,
    service::service_fn,
    Method, Request, Response, StatusCode,
};
use std::{
    collections::HashMap,
    convert::Infallible,
    net::SocketAddr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::net::TcpListener;

// Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
//
// https://github.com/rust-lang/rust/blob/1.55.0/library/core/src/slice/sort.rs#L559-L573
pub fn random_numbers() -> std::iter::RepeatWith<impl FnMut() -> usize> {
    let mut random = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::new(0, 0))
        .as_nanos() as usize;
    std::iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> (if usize::BITS <= 32 { 17 } else { 7 });
        random ^= random << (if usize::BITS <= 32 { 5 } else { 17 });
        random
    })
}

fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::from_static(b"Not found")))
        .unwrap()
}

fn invalid_method() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Full::new(Bytes::from_static(b"Method not allowed")))
        .unwrap()
}

async fn stylesheet() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(Full::new(Bytes::from_static(STYLESHEET)))
        .unwrap()
}

async fn favicon() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/x-icon")
        .body(Full::new(Bytes::from_static(FAVICON)))
        .unwrap()
}

#[derive(Default, Clone, Copy)]
struct Field {
    mine: bool,
    open: bool,
    neighbor_mines: u8,
}

async fn create_board(width: usize, height: usize, mines: usize) -> Response<Full<Bytes>> {
    let start = SystemTime::now();
    if width == 0 || height == 0 || mines == 0 {
        Response::builder()
            .status(StatusCode::RANGE_NOT_SATISFIABLE)
            .header("Cache-Control", "no-store")
            .body(Full::new(Bytes::from_static(b"NO 0 VALUES ALLOWED")))
            .unwrap()
    } else if width > MAX_SIZE || height > MAX_SIZE {
        Response::builder()
            .status(StatusCode::RANGE_NOT_SATISFIABLE)
            .header("Cache-Control", "no-store")
            .body(Full::new(Bytes::from_static(b"FIELD TOO BIG (MAX 50)")))
            .unwrap()
    } else if mines >= width * height {
        Response::builder()
            .status(StatusCode::RANGE_NOT_SATISFIABLE)
            .header("Cache-Control", "no-store")
            .body(Full::new(Bytes::from_static(
                b"MUST BE LESS MINES THAN FIELDS",
            )))
            .unwrap()
    } else {
        let mut fields = vec![Field::default(); width * height];
        let mut safe_indices = (0..width * height).collect::<Vec<_>>();

        let get_neighbor_indices = |idx: usize| -> Vec<usize> {
            let left_edge = (idx % width) == 0;
            let top_edge = idx < width;
            let bottom_edge = idx >= ((height - 1) * width);
            let right_edge = (idx % width) == (width - 1);

            let mut indices = Vec::with_capacity(8);

            if !left_edge {
                indices.push(idx - 1);
            }
            if !left_edge && !top_edge {
                indices.push(idx - 1 - width);
            }
            if !left_edge && !bottom_edge {
                indices.push(idx - 1 + width);
            }

            if !right_edge {
                indices.push(idx + 1);
            }
            if !right_edge && !top_edge {
                indices.push(idx + 1 - width);
            }
            if !right_edge && !bottom_edge {
                indices.push(idx + 1 + width);
            }

            if !top_edge {
                indices.push(idx - width);
            }

            if !bottom_edge {
                indices.push(idx + width);
            }
            indices
        };

        for (placed_mines, random) in (0..mines).zip(random_numbers()) {
            let mine_index = safe_indices.swap_remove(random % ((width * height) - placed_mines));

            fields[mine_index].mine = true;
            for neighbor_index in get_neighbor_indices(mine_index) {
                fields[neighbor_index].neighbor_mines += 1;
            }
        }

        // SAFETY: random_numbers returns a RepeatWith, which will always have a next element
        let mut open_indices = Vec::with_capacity(fields.len() - mines);
        open_indices.push(
            safe_indices
                [unsafe { random_numbers().next().unwrap_unchecked() } % safe_indices.len()],
        );
        while let Some(open_index) = open_indices.pop() {
            if !fields[open_index].open {
                fields[open_index].open = true;
                if fields[open_index].neighbor_mines == 0 {
                    open_indices.append(&mut get_neighbor_indices(open_index));
                }
            }
        }

        // Can't figure out how to do regression, otherwise I'd make these into functions to calculate the
        // allocated size based on width and height
        let mut style_string = String::new();
        let mut inputs_string = String::new();
        let mut table_string = String::new();

        for (field_index, &field) in fields.iter().enumerate() {
            table_string += format!("<td><label for=input_{field_index}></label></td>").as_str();
            if (field_index % width) == (width - 1) && field_index != width * height - 1 {
                table_string += "</tr><tr>";
            }

            inputs_string += format!(
                "<input id=input_{field_index} type=checkbox data-{}{}></input>",
                if field.mine { "mine" } else { "safe" },
                if field.open { " checked" } else { "" },
            )
            .as_str();

            style_string += format!(
                "#input_{field_index}:checked ~ main label[for=\"input_{field_index}\"]::before {{ content: \"{}\"; }}\n#input_{field_index}:checked ~ main label[for=\"input_{field_index}\"] {{ pointer-events: none; }}",
                if field.mine { "X".into() } else { field.neighbor_mines.to_string() })
            .as_str();
        }

        let response_string = format!(
            "<!DOCTYPE html><html><head><link rel=stylesheet href=style.css><link rel=icon type=\"image/x-icon\" href=favicon.ico><style>{}</style></head><body>{}<main><table><tbody><tr>{}</tr></tbody></table></main><footer><span id=lost>HAHA you lost >:)</span><span id=won>YAY, you WON!!!</span></footer></body></html>",
            style_string,
            inputs_string,
            table_string
        );

        let time = SystemTime::now()
            .duration_since(start)
            .map(|x| x.as_nanos().to_string())
            .unwrap_or("Filed to track time".to_string());
        dbg!(time);
        Response::builder()
            .status(StatusCode::OK)
            .header("Cache-Control", "no-store")
            .body(Full::from(response_string))
            .expect("Failed to respond?")
    }
}

async fn respond(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    if !matches!(req.method(), &Method::GET) {
        Ok(invalid_method())
    } else {
        let (width, height, mines) = match req.uri().query() {
            Some(query) => {
                let options_map = url::form_urlencoded::parse(query.as_bytes())
                    .into_owned()
                    .collect::<HashMap<String, String>>();
                let height = options_map
                    .get("height")
                    .unwrap_or(&"NaN".to_string())
                    .parse()
                    .unwrap_or(10);
                let width = options_map
                    .get("width")
                    .unwrap_or(&"NaN".to_string())
                    .parse()
                    .unwrap_or(10);
                let mines = options_map
                    .get("mines")
                    .unwrap_or(&"NaN".to_string())
                    .parse()
                    .unwrap_or(10);

                (width, height, mines)
            }
            None => (10, 10, 10),
        };

        Ok(match req.uri().path() {
            "/style.css" => stylesheet().await,
            "/favicon.ico" => favicon().await,
            "/" => create_board(width, height, mines).await,
            _ => not_found(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;

        let io = hyper_util::rt::TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(respond))
                .await
            {
                println!("Error serving connection: {err:?}");
            }
        });
    }
}
