# Line Server

## How does your system work?
The goal of the system is to serve lines of a file over a REST API.

The system is written in Rust, because it forces me to pay close attention to memory usage and avoiding concurrency issues. It is also very performant and efficient.

The REST API was implemented using Actix https://actix.rs/ because it was easy to setup, has a large amount of support, the documentation was decent and it performs well in benchmarks: https://www.techempower.com/benchmarks/

The file is read using a shared Buffered Reader, to ensure fast reads, to not have to keep opening and closing the file and to not have to duplicate the data in a database. In order to guarantee threadsafety it uses a read/write lock.

## How will your system perform with a 1 GB file? a 10 GB file? a 100 GB file?

