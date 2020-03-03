# Line Server

The goal of the system is to serve lines of a file over a REST API. Perfomance wise I wanted reads to be O(1) and to not have to duplicate the data from the text file anywhere else (except the file itself).

## How does your system work?

During startup the system will first open and scan the file using a BufferedReader and store both the reader and the position of every \n in the file. Then, it starts a web server with the desired endpoint to get a line from it's number.

Whenever a GET request is issued to /lines/{line_number} the server will use the shared BufferedReader and new line character position array created during startup and read the bytes between the position of the \n character that matches the line number and the next \n character into a buffer. Finally, the byte buffer is converted to String and returned.

## How will your system perform with a 1 GB file? a 10 GB file? a 100 GB file?

The read should be O(1), since every line is indexed by it's first byte position in the file. So, file size should not have a bearing in read access speed (startup may slow down slightly and it may hit framework/machine limits).

I've tested with a file up to 10GB with success. However, the number of lines may become an issue if somehow they can't fit into a u64.

## How will your system perform with 100 users? 10000 users? 1000000 users?

Using my 8 GB file with 262145 lines I got the following results:

#### 100 concurrent connections

```
wrk -t4 -c100 -d2m -s test/random_request.lua http://127.0.0.1:8088/ --latency
Running 2m test @ http://127.0.0.1:8088/
  4 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    23.69ms   21.64ms 380.69ms   94.53%
    Req/Sec     1.20k   414.46     2.75k    71.08%
  Latency Distribution
     50%   19.00ms
     75%   24.51ms
     90%   33.12ms
     99%  129.11ms
  571655 requests in 2.00m, 17.25GB read
Requests/sec:   4762.88
Transfer/sec:    147.14MB
```

#### 1000 concurrent connections

```
wrk -t2 -c10000 -d30s -s test/random_request.lua http://127.0.0.1:8088/ --latency
Running 30s test @ http://127.0.0.1:8088/
  2 threads and 10000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   110.30ms   31.92ms 358.65ms   84.45%
    Req/Sec     1.03k   418.75     2.08k    70.90%
  Latency Distribution
     50%  106.20ms
     75%  125.96ms
     90%  139.63ms
     99%  263.99ms
  61692 requests in 30.07s, 1.86GB read
  Socket errors: connect 9749, read 59, write 0, timeout 0
Requests/sec:   2051.40
Transfer/sec:     63.37MB
```
Note: connection errors match the connection limit set by default in actix (256).

#### 250 concurrent connections
```
wrk -t4 -c250 -d2m -s test/random_request.lua http://127.0.0.1:8088/ --latency
Running 2m test @ http://127.0.0.1:8088/
  4 threads and 250 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    63.23ms   46.78ms 804.45ms   93.46%
    Req/Sec     0.98k   360.34     2.06k    73.62%
  Latency Distribution
     50%   52.07ms
     75%   65.46ms
     90%   89.52ms
     99%  265.83ms
  467247 requests in 2.00m, 14.10GB read
  Socket errors: connect 0, read 277, write 0, timeout 0
Requests/sec:   3890.39
Transfer/sec:    120.19MB
```
Some errors start showing up when pushing 250 concurrent connections and performance is worse than with 100 connections.

### Conclusion: 

While the definition of having X users is not straight (is it total over a year? Is it per day? Is it concurrent connections?) I can say the system handles 100 concurrent connections very well. However, it is not scaling with concurrent connections. The causes for this may be disk IO (since we are reading straight from file), http server limits, the machine it's running on or my own fault.

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

The time spent would depend on the actual use case of the system. I'd first try to really understand what it's used for and see if all the constraints remain true in order to solve the problem. If the constraints didn't change I'd probably try to figure out a better representation of the file than the .txt, even if it meant duplicating the data it would probably be worth it, since storage space is usually cheap. If performance was still an issue I'd look at trying to create some sort of distributed system that would at startup split the file (if large enough) between different machines and each would serve the lines that match their chunk. Overall, I'd first question the constraints of the problem and the usage and then act on that. I would also like to spend time trying to figure out the concurrent connections issues and errors.

## If you were to critique your code, what would you have to say about it?

* Lacks tests.
* Some crude .unwraps() that should be handled.
* Split FileReader into it's own file.
* Needs a config/resource file to not have stuff * hardcoded.
* It's overall a bit raw.
* Corner cases better testing.
