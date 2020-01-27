// Module to define the 'Game' class.


// Member variables for the 'Game' class.
pub struct Game {
    // Properties specified by the command line.
    pub index: usize,
    pub indent: usize,
    pub start: f64,
    pub finish: f64,
    pub threads: usize,
    pub append: bool,

    // Properties loaded from the game index.
    pub grid: usize,
    pub maxShip: usize,
    pub horizontal: Vec<usize>,
    pub vertical: Vec<usize>,
    pub mask: Vec<usize>,
    pub negativeMask: Vec<usize>,

    // Workspace for the solution.
    number: usize,
    count: usize,
    posibilities: Vec<Vec<usize>>,
    line: Vec<usize>,
    startTime: std::time::Instant,
}

// Methods for the 'Game' class.
impl Game {
    pub fn new() -> Game {
        let horizontal = Vec::new();
        let vertical = Vec::new();
        let mask = Vec::new();
        let negativeMask = Vec::new();
        let posibilities = Vec::new();
        let line = Vec::new();
        let now = std::time::Instant::now();
        Game{index: 0, grid: 0, indent: 0, start: 0.0, finish: 100.0, threads: 1, append: false, maxShip: 0, horizontal: horizontal, vertical: vertical, mask: mask, negativeMask: negativeMask, number: 1, count: 1, posibilities: posibilities, line: line, startTime: now}
    }



    // Initialise the arrays.
    pub fn initialise(&mut self) {
        // println!("initialise, self.grid = {}", self.grid);
        /*
        self.horizontal = vec![0; self.grid];
        self.vertical = vec![0; self.grid];
        self.mask = vec![0; self.grid];
        self.negativeMask = vec![0; self.grid];
        */
        for _i in 0..self.grid {
            self.horizontal.push(0);
            self.vertical.push(0);
            self.mask.push(0);
            self.negativeMask.push(0);
            self.line.push(0);
        }
    }



    // Make the calculations that displayBoard() makes.
    fn dontDisplayBoard(&mut self) {
        self.number = 1;
        for y in 0..self.grid {
            let lines = self.getPossibleLines(self.horizontal[y], self.mask[y], self.negativeMask[y]);
            self.number *= lines.len();
            self.posibilities.push(lines);
        }
    }



    // Display the intial board.
    fn displayBoard(&mut self) {
        println!("Logic Battleships Game Number {}", self.index);
        print!("\u{250F}");
        for _y in 0..self.grid {
            print!("\u{2501}");
        }
        println!("\u{2513}");
        self.number = 1;
        for y in 0..self.grid {
            print!("\u{2503}");
            for x in 0usize..self.grid {
                let mask = 2_usize.pow(x as u32);
                if self.mask[y] & mask == mask {
                    print!("\u{2588}");
                }
                else if self.negativeMask[y] & mask == mask {
                    print!("\u{00B7}");
                }
                else {
                    print!(" ");
                }
            }
            print!("\u{2503}");
            print!("{}", self.horizontal[y]);

            let lines = self.getPossibleLines(self.horizontal[y], self.mask[y], self.negativeMask[y]);
            println!("  There are {} possible lines.", lines.len());

            self.number *= lines.len();

            self.posibilities.push(lines);
        }
        print!("\u{2517}");
        for _y in 0..self.grid {
            print!("\u{2501}");
        }
        println!("\u{251B}");
        print!(" ");
        for y in 0..self.grid {
            print!("{}", self.vertical[y]);
        }
        println!("");
        println!("Search space is {}", formatInt(self.number));
    }


    // Display the current position.
    fn displayPosition(& self) {
        println!("Logic Battleships Game Number {}", self.index);
        print!("\u{250F}");
        for _y in 0..self.grid {
            print!("\u{2501}");
        }
        println!("\u{2513}");
        for y in 0..self.grid {
            print!("\u{2503}");
            for x in 0usize..self.grid {
                let mask = 2_usize.pow(x as u32);
                if self.line[y] & mask == mask {
                    print!("\u{2588}");
                }
                else {
                    print!(".");
                }
            }
            println!("\u{2503}");
        }
        print!("\u{2517}");
        for _y in 0..self.grid {
            print!("\u{2501}");
        }
        println!("\u{251B}");
    }



    // Returns the set of possible lines that have the specified number of solid positions.
    fn getPossibleLines(&self, numSolid :usize, mask :usize, negativeMask :usize) -> Vec<usize> {
        // print('GetPossibleLines({}, {}, {}, {}, {})'.format(nNumPositions, nNumSolid, nMaxShip, nMask, nNegativeMask))
        let mut listResult = Vec::new();
        let maximum = (2_usize.pow(self.grid as u32)) - 1;
        for pos in 0..maximum {
            if self.countSolids(pos) == numSolid {
                if pos & mask == mask {
                    if (!pos) & negativeMask == negativeMask {
                        if self.getLongestShip(pos) <= self.maxShip {
                            listResult.push(pos);
                        }
                    }
                }
            }
        }
        return listResult
    }



    // Returns the number of ship elements in the positon.
    fn countSolids(&self, position :usize) -> usize {
        let mut count :usize = 0;
        for pos in 0..self.grid {
            let mask = 2_usize.pow(pos as u32);
            if position & mask == mask {
                count += 1;
            }
        }
        return count;
    }



    // Returns the size of the longest ship in the position.
    fn getLongestShip(&self, position :usize) -> usize {
        let mut maximum :usize = 0;
        let mut current :usize = 0;
        for pos in 0..self.grid {
            let mask = 2_usize.pow(pos as u32);
            if position & mask == mask {
                current += 1;
                if current > maximum {
                    maximum = current;
                }
            }
            else {
                current = 0;
            }
        }
        return maximum;
    }



    fn getNumPossible(&self, level: usize) -> usize {
        let mut answer = 1;
        for index in level..self.grid {
            answer *= self.posibilities[index].len();
        }
        return answer;
    }



    fn search(&mut self, level: usize) {
        let percentage = 100.0 * self.count as f64 / self.number as f64;

        // If before start then do something.

        if percentage < self.start {
            let numSteps = self.getNumPossible(level);
            let stepPercentage = 100.0 * ( self.count + numSteps ) as f64 / self.number as f64;
            if stepPercentage < self.start {
                self.count += numSteps;
                return;
            }
            //else:
            //    pass
            //    #print('GO')
        }

        // if before end then do this.
        if percentage <= self.finish {
            // for possible in self.posibilities[level] {
            for index in 0..self.posibilities[level].len() {
                let possible = self.posibilities[level][index];
                self.line[level] = possible;
                if level == self.grid - 1 {
                    // Final level.
                    if percentage >= self.start { // and percentage <= self.finish_search:
                        if self.isValidSolution() {
                            let now = std::time::SystemTime::now();
                            println!("\x1B[K{:?}", now);
                            // println!("\033[K{}", datetime.datetime.now());
                            self.displayPosition();
                        }
                    }
                    self.count += 1;

                    // Display the progress on this thread.
                    if self.count % 10000000 == 0 {
                        let elapsedTime = self.startTime.elapsed().as_secs(); 3600.0; // time.time() - self.startTime
                        let completed = (percentage - self.start) / (self.finish - self.start);
                        let totalTime = (elapsedTime as f64 / completed) as u64;
                        let estimatedTime = 30 + ((1.0 - completed) * totalTime as f64) as u64;
                        if self.indent > 0 {
                            println!("\x1B[{}C{:>7.3} ", self.indent, percentage);
                            println!("\x1B[{}C {} ", self.indent, formatTime(estimatedTime));
                            println!("\x1B[{}C {} ", self.indent, formatTime(elapsedTime));
                            print!("\x1B[{}C {} \r\x1B[3A", self.indent, formatTime(totalTime));
                        }
                        else {
                            println!("{:>7.3} ", percentage);
                            println!(" {} ", formatTime(estimatedTime));
                            println!(" {} ", formatTime(elapsedTime));
                            print!(" {} \r\x1B[3A", formatTime(totalTime));
                        }
                    }
                }
                else {
                    // Search down to the next leve.
                    self.search(level+1);
                }
            }
        }
    }



    // Find the solutions to the game.
    pub fn solve(&mut self) {
        if self.threads == 1 {
            // Active solve this problem.
            if self.append {
                self.dontDisplayBoard();
            }
            else
            {
                self.displayBoard();
            }
            self.search(0);
            if self.indent > 0 {
                print!("\x1B[{}C ------ \r", self.indent);
            }
            else {
                print!(" ------ \r");
            }
        }
        else {
            self.displayBoard();

            // Launch threads to solve the problem.
            self.startThreads();
        }

    }



    // Returns true if the current position is valid solution to the problem.
    fn isValidSolution(&self) -> bool {
        for row in 0..self.grid {
            let line = self.verticalLine(row);

            // Check that the vertical lines match the conditions
            if self.countSolids(line) != self.vertical[row] {
                return false;
            }

            // Check the length of the battle ships.
            if self.getLongestShip(line) > self.maxShip {
                return false;
            }
        }

        // Check the ships are not touching.
        for x in 0..self.grid {
            for y in 0..self.grid {
                if self.isShip(x as isize, y as isize) {
                    // Check the diagonals.
                    if self.isShip(x as isize - 1, y as isize - 1) {
                        return false;
                    }
                    if self.isShip(x as isize + 1, y as isize - 1) {
                        return false;
                    }
                    if self.isShip(x as isize - 1, y as isize + 1) {
                        return false;
                    }
                    if self.isShip(x as isize + 1, y as isize + 1) {
                        return false;
                    }
                }
            }
        }

        // Optionally check for the correct number of ships.
        //if self.isCheckShips:
        //    ships = self.getShips()
        //    if ships[1] != 4:
        //        return False
        //    if ships[2] != 3:
        //        return False
        //    if ships[3] != 2:
        //        return False
        //    if ships[4] != 1:
        //        return False

        // Looks good!!
        return true;
    }



    // Calculates the score for the vertical line.
    fn verticalLine(&self, index: usize) -> usize {
        let mask = 2_usize.pow(index as u32);
        let mut line = 0;
        for row in 0..self.grid {
            if self.line[row] & mask == mask {
                line += 2_usize.pow(row as u32);
            }
        }
        return line
    }


    // Returns true if the specified position is ship in the current position.
    fn isShip(&self, x: isize, y: isize) -> bool {
        // Allow questions outside the grid.  The answer is false.
        if x < 0 || x >= self.grid as isize || y < 0 || y >= self.grid as isize {
            return false;
        }

        // Search on the board.
        let mask = 2_usize.pow(x as u32);
        return self.line[y as usize] & mask == mask;
    }



    /// Launch the specified number of threads to start the program.
    fn startThreads(&self) {
        let mut split = self.threads;
        if split > 20 {
            split = 20;
        }
        else if split < 2 {
            split = 2;
        }

        let amount = (self.finish - self.start) / split as f64;
        let mut start = self.start;
        let mut indent = 0;
        let mut threads: Vec<std::process::Child> = Vec::new(); // [].to_vec();
        // threads = []
        for i in 0..split {
            let mut args: Vec<String> = [].to_vec();
            args.push("-g".to_string());
            args.push(format!("{}", self.index));
            args.push("-s".to_string());
            args.push(format!("{}", start));
            args.push("-f".to_string());
            if i == split-1 {
                args.push("100".to_string());
            }
            else {
                args.push(format!("{}", start+amount));
            }
            args.push("-i".to_string());
            args.push(format!("{}", indent));
            args.push("-t".to_string());
            args.push("1".to_string());
            args.push("-a".to_string());
            if false {
                println!("{:?}", args);
            }

            let mut command = std::process::Command::new("target/debug/battleships");
            command.args(args);
            let child = command.spawn().unwrap();

            threads.push(child);

            // command = [__file__, '--game', '{}'.format(gameNumber) , '--start', '{}'.format(fStart), '--finish', '{}'.format(fStart + fAmount), '--indent', '{}'.format(indent), '--threads', '1']
            // if game.isTranspose:
            // command.append('-p')
            // if args.verbose:
            // command.append('-v')
            // if args.mask:
            // command.append('--mask')
            // for i in range(0, len(args.mask)):
            // command.append('{}'.format(args.mask[i]))
            // threads.append(subprocess.Popen(command))
            start += amount;
            indent += 7;
            /*
                if game.finishSearch >= 100:
                    if args.verbose:
                        print('Thread({}) --game={} --start={} --indent={} --threads={}'.format(nSplit-1, gameNumber, fStart, indent, 1))
                    command = [__file__, '--game', '{}'.format(gameNumber), '--start', '{}'.format(fStart), '--indent', '{}'.format(indent), '--threads', '1']
                    if game.isTranspose:
                        command.append('-p')
                    if args.verbose:
                        command.append('-v')
                    if args.mask:
                        command.append('--mask')
                        for i in range(0, len(args.mask)):
                            command.append('{}'.format(args.mask[i]))
                    threads.append(subprocess.Popen(command))
                else:
                    if args.verbose:
                        print('Thread({}) --game={} --start={} --finish={} --indent={} --threads={}'.format(nSplit-1, gameNumber, fStart, game.finishSearch, indent, 1))
                    command = [__file__, '--game', '{}'.format(gameNumber), '--start', '{}'.format(fStart), '--finish', '{}'.format(game.finishSearch), '--indent', '{}'.format(indent), '--threads', '1']
                    if game.isTranspose:
                        command.append('-p')
                    if args.verbose:
                        command.append('-v')
                    threads.append(subprocess.Popen(command))
                    */
        }
        for mut thread in threads {
            thread.wait();
        }

        /*
        while isAnyThreadRunning(threads):
            time.sleep(10)
        */
        println!();
        println!();
        println!();
        println!();
    }
}



fn formatInt(i: usize) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    return s;
}

fn formatTime(t: u64) -> String {
    let hours = t / 3600;
    let minutes = (t - hours * 3600) / 60;
    let s = format!("{:03}:{:02}", hours, minutes);
    return s;
}
