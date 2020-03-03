use actix_web::{ web, App, HttpResponse, HttpServer };
use std::fs::File;
use std::io::{ BufReader, BufRead, Seek, SeekFrom };
use std::sync::RwLock;

// File Reader will be a singleton that stores the position of every \n and a Buffered Reader
struct FileReader {
    reader: BufReader<File>,
    positions: Vec<usize>
}

impl FileReader {

    // Instanciate our FileReader by going opening a 
    // buffered reader into the desired file and scanning 
    // it for the position of every \n to store in a vec
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

    // Reads the line number from the associated buffer position to the next \n
    fn read_line(&mut self, line_number: usize, buffer: &mut Vec<u8>) {
        let start_read_index = self.positions[line_number];
        self.reader.seek(SeekFrom::Start(start_read_index as u64)).unwrap();
        self.reader.read_until(b'\n', buffer).unwrap();
    }
}

async fn get_line(path: web::Path<(u32,)>, data: web::Data<RwLock<FileReader>>) -> HttpResponse {

    // Get line_number from request path
    let number = path.0 as usize;

    // Check if it's indexed
    if number >= data.read().unwrap().positions.len() {
        HttpResponse::PayloadTooLarge().finish()
    } else {

        // Call the FileReader singleton to read the file
        let mut buffer = Vec::new();
        data.write().unwrap().read_line(number, &mut buffer);
        HttpResponse::Ok().body(format!("{}", String::from_utf8(buffer).unwrap()))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Read argument: the text file to serve
    let arg = std::env::args().into_iter().nth(1).expect("No argument found");

    // Create a FileReader that stores the positions of every \n and a bufreader to the file
    let file_reader = FileReader::new(arg.clone());

    // Add ReadWrite lock and wrap it in web::Data to allow it to be shared across threads
    let state = web::Data::new(RwLock::new(file_reader));

    // Start server
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