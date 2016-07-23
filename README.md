# monty_hall 
A simple [Monty Hall](https://en.wikipedia.org/wiki/Monty_Hall_problem) experiment coded with [**Rust**](https://www.rust-lang.org/en-US/) to learn the language.
## Synopsis
Implementation of a simple scenario for experimenting Monty Hall problem.
Here is the flow;

1. Creates 2.000.000 rounds to get the results.
2. Each round has 3 doors which hides 1 car and 2 goats. For.ex (goat,goat,car). Positions are changing for each round.
4. Contestant selects a door (rand.random)
5. Host reveals one of the goats from remaining doors.
6. 1.000.000 rounds are without changing the contestant selection, and the other 1.000.000 rounds are with changing contestants selection with the remaing closed door.
7. Printing the total win counts for each.
## Installation
 1. Download and install **Rust** from [here](https://www.rust-lang.org/en-US/downloads.html)
 2. Download and install **Cargo** from [here](http://doc.crates.io/)
 3. Clone and run the project.
```bash 
git clone https://github.com/serayuzgur/monty_hall.git
cd monty_hall
cargo run
```

## License

The MIT License (MIT) Copyright (c) 2016 Seray Uzgur

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
