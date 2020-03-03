# Line Server

The goal of the system is to serve lines of a file over a REST API. Perfomance wise I wanted reads to be O(1) and to not have to duplicate the data from the text file anywhere else (except the file itself).

## How does your system work?

During startup the system will first open and scan the file using a BufferedReader and store both the reader and the position of every \n in the file. Then, it starts a web server with the desired endpoint to get a line from it's number.

Whenever a GET request is issued to /lines/{line_number} the server will use the shared BufferedReader and new line character position array created during startup and read the bytes between the position of the \n character that matches the line number and the next \n character into a buffer. Finally, the byte buffer is converted to String and returned.

## How will your system perform with a 1 GB file? a 10 GB file? a 100 GB file?

The read is O(1), since every line is indexed by it's byte position in the file. So, file size should not have a bearing in read access speed (startup may slow down slightly and it may hit framework/machine limits).

I've tested with a file up to 10GB with success. However, the number of lines may become an issue if somehow they can't fit into a u64.

## How will your system perform with 100 users? 10000 users? 1000000 users?



## What documentation, websites, papers, etc did you consult in doing this assignment?

Rust docs https://doc.rust-lang.org/std/index.html

Actix docs https://actix.rs/docs/ and github https://github.com/actix

A healthy dose of google-fu and stackoverflow-karate.

A question in Rust languague official discord https://discordapp.com/invite/rust-lang when trying to understand a specific bug regarding ownership.

## What third-party libraries or other tools does the system use? How did you choose each library or framework you used?

The system is written in Rust. Firstly, because of my own personal interest in it, then because it forces me to pay close attention to how memory is used and shared and the compiler can help me guarantee safety. 

The REST API was implemented using Actix https://actix.rs/ because it was easy to setup, has a large amount of support, the documentation was decent and it performs well in benchmarks: https://www.techempower.com/benchmarks/

## How long did you spend on this exercise? If you had unlimited more time to spend on this, how would you spend it and how would you prioritize each item?

About 8h. The first hour just researching a bit and putting the solution together in my head. About 5h of development time, where much was dealing with the Rust borrow checker (worth it). The last 2h spent load testing and writing the docs and scripts.

The time spent would depend on the actual use case of the system. I'd first try to really understand what it's used for and see if all the constraints remain true in order to solve the problem. If the constraints didn't change I'd probably try to figure out a better representation of the file than the .txt, even if it meant duplicating the data it would probably be worth it, since storage space is usually cheap. If performance was still an issue I'd look at trying to create some sort of distributed system that would at startup split the file (if large enough) between different machines and each would serve the lines that match their chunk. Overall, I'd first question the constraints of the problem and the usage and then act on that.

## If you were to critique your code, what would you have to say about it?

Lacks tests.
Some crude .unwraps() that should be handled.
Split FileReader into it's own file.
Needs a config/resource file to not have stuff hardcoded.
It's overall a bit raw.