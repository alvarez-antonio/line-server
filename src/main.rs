use actix_web::{ web, App, HttpResponse, HttpServer };
use std::fs::File;
use std::io::{ BufReader, BufRead, Seek, SeekFrom };
use std::sync::RwLock;


async fn get_line(path: web::Path<(u32,)>, data: web::Data<RwLock<FileReader>>) -> HttpResponse {
    let number = path.0 as usize;
    let mut file_reader = data.write().unwrap();
    if number >= file_reader.positions.len() {
        HttpResponse::PayloadTooLarge().finish()
    } else {
        let mut buffer = Vec::new();
        file_reader.read_line(number, &mut buffer);
        HttpResponse::Ok().body(format!("{}", String::from_utf8(buffer).unwrap()))
    }
}

struct FileReader {
    reader: BufReader<File>,
    positions: Vec<usize>
}

impl FileReader {
    fn new(file_path: String) -> FileReader {
        print!("Opening file {}\n", file_path);
        let file = File::open(file_path).expect("File doesn't exist");
        let mut reader = BufReader::new(file);
        let mut positions = vec![0];
        let mut current_position = 0;
        let mut _buffer = Vec::new();
        loop {
            let count = reader.read_until(b'\n', &mut _buffer).unwrap();
            if count == 0 {
                positions.pop().unwrap();
                break;
            }
            current_position += count;
            positions.push(current_position);
        }
        FileReader { reader, positions }
    }

    fn read_line(&mut self, line_number: usize, buffer: &mut Vec<u8>) {
        let start_read_index = self.positions[line_number];
        self.reader.seek(SeekFrom::Start(start_read_index as u64)).unwrap();
        self.reader.read_until(b'\n', buffer).unwrap();
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let arg = std::env::args().into_iter().nth(1).expect("No argument found");
    let file_reader = FileReader::new(arg.clone());
    let state = web::Data::new(RwLock::new(file_reader));
    HttpServer::new(move || {
        App::new()
        .app_data(state.clone())
        .route("/", web::get().to(|| HttpResponse::Ok().body("teste")))
        .route("/lines/{line_number}", web::get().to(get_line))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}