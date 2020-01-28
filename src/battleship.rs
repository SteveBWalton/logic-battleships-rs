// Module to define the 'Game' class.


// use std::fs::OpenOptions;
use std::io::prelude::*;


// Member variables for the 'Game' class.
#[derive(Clone)]
pub struct Game {
    // Properties specified by the command line.
    pub index: usize,
    pub indent: usize,
    pub start: f64,
    pub finish: f64,
    pub threads: usize,
    pub append: bool,
    pub largeSolver: bool,
    pub userMask: Vec<usize>,

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
    totalShips: usize,
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
        let userMask = Vec::new();
        Game{index: 0, grid: 0, indent: 0, start: 0.0, finish: 100.0, threads: 0, append: false, largeSolver: false, userMask: userMask, maxShip: 0, horizontal: horizontal, vertical: vertical, mask: mask, negativeMask: negativeMask, number: 1, count: 1, posibilities: posibilities, line: line, startTime: now, totalShips: 0}
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



    fn writeFile(&self, message: String, isAppend: bool) {
        print!("{}", message);
        // println!("isAppend = {}", isAppend);
        if !isAppend {
            match std::fs::remove_file("results.txt") {
                Err(_e) => {
                }

                Ok(_) => {
                }
            }
        }

        let mut option = std::fs::OpenOptions::new();
        if isAppend {
            // Append to existing file.
            option.append(true);
            option.create(true);
        }
        else {
            // Create new.
            // This errors if the file exists.
            // println!("Create new log file.");
            option.write(true);
            option.create_new(true);
        }

        match option.open("results.txt") {
            Err(e) => {
                println!("File open error.");
                println!("{}", e);
            }

            Ok(mut f) => {
                // match f.write_all(message.as_bytes()) {
                match write!(f, "{}", message) {
                    Err(_e) => {
                        println!("File write error.");
                    }

                    Ok(_) => {
                    }
                }
            }
        }
    }



    // Display the intial board.
    fn displayBoard(&self) {
        let mut board : String = format!("Logic Battleships Game Number {}\n", self.index);

        // println!("Logic Battleships Game Number {}", self.index);

        board += "\u{250F}";
        // print!("\u{250F}");
        for _y in 0..self.grid {
            // print!("\u{2501}");
            board += "\u{2501}";
        }
        // println!("\u{2513}");
        board += "\u{2513}\n";
        for y in 0..self.grid {
            board += "\u{2503}";
            for x in 0usize..self.grid {
                let mask = 2_usize.pow(x as u32);
                if self.mask[y] & mask == mask {
                    board += "\u{2589}";
                }
                else if self.negativeMask[y] & mask == mask {
                    board += "\u{00B7}";
                }
                else {
                    board += " ";
                }
            }
            board += "\u{2503}";
            board = format!("{}{}", board, self.horizontal[y]);
            board = format!("{}  There are {} possible lines.\n", board, self.posibilities[y].len());
        }
        board += "\u{2517}";
        for _y in 0..self.grid {
            board += "\u{2501}";
        }
        board += "\u{251B}\n";
        board += " ";
        for y in 0..self.grid {
            board = format!("{}{}", board,self.vertical[y]);
        }
        board += "\n";
        board = format!("{}Search space is {}\n", board, formatInt(self.number));
        self.writeFile(board, self.append);
    }



    // Display the current position.
    // This is usually a solution.
    fn displayPosition(& self) {
        let mut board : String = format!("Logic Battleships Game Number {}\n", self.index);

        let ships = self.getShips();

        board += "        \u{250F}";
        for _y in 0..self.grid {
            board += "\u{2501}";
        }
        board += "\u{2513}\n";
        for y in 0..self.grid {
            board += "        \u{2503}";
            for x in 0..self.grid {
                let mask = 2_usize.pow(x as u32);
                if self.line[y] & mask == mask {
                    board += "\u{2589}";
                }
                else {
                    board += "\u{00B7}";
                }
            }
            board += "\u{2503}";
            board = format!("{}{}", board, self.horizontal[y]);

            if y > 0 && y <= self.maxShip {
                board = format!("{}  {} x {} ship.", board, ships[y], y);
            }

            board += "\n";
        }
        board += "        \u{2517}";
        for _y in 0..self.grid {
            board += "\u{2501}";
        }
        board +="\u{251B}\n";
        board += "         ";
        for y in 0..self.grid {
            board = format!("{}{}", board, self.vertical[y]);
        }
        board += "\n";

        self.writeFile(board, true);
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
                            // let now = std::time::SystemTime::now();
                            // println!("\x1B[K{:?}", now);
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
                            println!("\x1B[{}C {} ", self.indent, formatTime(totalTime));
                            println!("\x1B[5A");
                        }
                        else {
                            println!("{:>7.3} ", percentage);
                            println!(" {} ", formatTime(estimatedTime));
                            println!(" {} ", formatTime(elapsedTime));
                            println!(" {} ", formatTime(totalTime));
                            println!("\x1B[5A");
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



    fn initialiseGame(&mut self) -> bool {
        self.totalShips = 0;
        let mut verticalShips = 0;
        for row in 0..self.grid {
            self.totalShips += self.horizontal[row];
            verticalShips += self.vertical[row];
        }
        if verticalShips != self.totalShips {
            println!("Horizontal Total {} != {} Vertical total.", self.totalShips, verticalShips);
            return false;
        }

        self.number = 1;
        for y in 0..self.grid {
            let lines = self.getPossibleLines(self.horizontal[y], self.mask[y], self.negativeMask[y]);
            self.number *= lines.len();
            self.posibilities.push(lines);
        }

        // Return success.
        return true;
    }


    // Find the solutions to the game.
    pub fn solve(&mut self) {
        if !self.initialiseGame()
        {
            return;
        }

        if self.threads == 0 {
            self.displayBoard();
        }
        else if self.threads == 1 {
            // Active solve this problem.
            if !self.append {
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

            if self.largeSolver {
                // Use the large solver.
                // This only works for maxShip 5 problems.
                if self.maxShip != 5 {
                    println!("The large solver only works for max ship 5 problems.");
                }
                else {
                    let numPositions = self.guessLargeShips(false, 0, 0);
                    self.guessLargeShips(true, 0, numPositions);
                }
            }
            else {
                // Launch threads to standard solve the problem.
                self.startThreads();
            }
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

        // Check for the correct number of ships.
        let ships = self.getShips();
        if ships[1] != 4 {
            return false;
        }
        if ships[2] != 3 {
            return false;
        }
        if ships[3] != 2 {
            return false;
        }
        if ships[4] != 1 {
            return false;
        }

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



    // Counts the number and size of ships on the line.
    fn countShipsOnLine(&self, line: usize, ships: &mut Vec<usize>) {
        // binaryLine = '{0:b}'.format(line)
        let mut current = 0;
        let mut mask = 1;
        for _pos in 0..self.grid {
            if line & mask == mask {
                current += 1;
            }
            else {
                if current > 1 {
                    ships[current] += 1;
                }
                current = 0;
            }
            mask *= 2;
        }
        if current > 1 {
            ships[current] += 1;
        }
    }



    /// Returns the number and size of the ships on the grid.
    fn getShips(&self) -> Vec<usize>  {
        let mut ships: Vec<usize> = Vec::new();
        for _shipSize in 0..self.maxShip+1 {
            ships.push(0);
        }
        for row in 0..self.grid {
            self.countShipsOnLine(self.line[row], &mut ships);
            self.countShipsOnLine(self.verticalLine(row), &mut ships);
        }
        let mut shipsOne = self.totalShips;
        for shipSize in 0..self.maxShip+1 {
            shipsOne -= shipSize * ships[shipSize];
        }
        ships[1] = shipsOne;

        return ships
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
            args.push("-k".to_string());

            if self.userMask.len() > 0 {
                args.push("-m".to_string());
                for x in 0..self.userMask.len() {
                    args.push(format!("{}", self.userMask[x]));
                }
            }

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
            let _result = thread.wait().unwrap();
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



    // Set the position to the position defined by the mask.
    fn applyMask(&mut self) {
        for x in 0..self.grid {
            self.line[x] = self.mask[x]
        }
    }

    // Loop through all the possible positions for the large ships.
    fn guessLargeShips(&mut self, isSolve: bool, numPositions: usize, totalPositions: usize) -> usize {

        // print('Loop through all the possible positions for the large ships. ')

        if !self.initialiseGame()
        {
            return 0;
        }

        let mut countPositions = numPositions;

        // Build the initial positions.
        self.applyMask();
        let ships = self.getShips();

        let mut shipSize = 0;
        if self.maxShip >= 4 {
            // print('There should be 1 size 4 ship.')
            // print('There are {} size 4 ships.'.format(ships[4]))
            if ships[4] < 1 {
                shipSize = 4;
            }
        }
        if self.maxShip >= 5 {
            // print('There should be 1 size 5 ship.')
            // print('There are {} size 5 ships.'.format(ships[5]))
            if ships[5] < 1 {
                shipSize = 5;
            }
        }

        if shipSize > 0 {
            // Try to fit a ship of size shipSize onto the grid.
            // Add the ship horizontally.
            for x in 0..self.grid - shipSize + 1 {
                for y in 0..self.grid {
                    self.applyMask();
                    for ship in x..x+shipSize {
                        self.line[y] |= 2_usize.pow(ship as u32);
                    }
                    if self.isPartialSolution(shipSize) {
                        if shipSize == 5 {
                            let mut newGame = self.clone(); // copy.deepcopy(self)
                            for i in 0..self.grid {
                                newGame.mask[i] = newGame.line[i];
                            }
                            countPositions = newGame.guessLargeShips(isSolve, countPositions, totalPositions);
                        }
                        else {
                            countPositions += 1;
                            if isSolve {
                                // self.write()
                                self.launch(countPositions, totalPositions)
                            }
                        }
                    }
                }
            }

            // Add the ship vertically.
            for x in 0..self.grid {
                for y in 0..self.grid - shipSize + 1 {
                    self.applyMask();
                    for ship in y..y+shipSize {
                        self.line[ship] |= 2_usize.pow(x as u32);
                    }
                    if self.isPartialSolution(shipSize) {
                        if shipSize == 5 {
                            let mut newGame = self.clone(); // copy.deepcopy(self)
                            for i in 0..self.grid {
                                newGame.mask[i] = newGame.line[i];
                            }
                            countPositions = newGame.guessLargeShips(isSolve, countPositions, totalPositions)
                        }
                        else {
                            countPositions += 1;
                            if isSolve {
                                // self.write()
                                self.launch(countPositions, totalPositions)
                            }
                        }
                    }
                }
            }
        }
        return countPositions;
    }


    // Returns true if the current position does not exceed the boundary conditions and could lead to a valid solultion.
    fn isPartialSolution(&self, shipSize: usize) -> bool {
        for row in 0..self.grid {
            // Check the number of blocks.
            if self.countSolids(self.line[row]) > self.horizontal[row] {
                // print('Too many blocks in row {}. ({}, {})'.format(row+1, countSolids(self.line[row]), self.horizontal[row]))
                return false;
            }

            // Check the length of the battle ships.
            if self.getLongestShip(self.line[row]) > self.maxShip {
                // print('Ship too long in row {}'.format(row+1))
                return false;
            }

            // Check the columns.
            let line = self.verticalLine(row);
            if self.countSolids(line) > self.vertical[row] {
                // print('Too many blocks in column {}'.format(row+1))
                return false;
            }

            // Check the length of the battle ships.
            if self.getLongestShip(line) > self.maxShip {
                // print('Ship too long in column {}'.format(row+1))
                return false;
            }
        }

        // Check the ships are not touching.
        for x in 0..self.grid {
            for y in 0..self.grid {
                if self.isShip(x as isize, y as isize) {
                    // Check the diagonals.
                    if self.isShip(x as isize-1, y as isize-1) {
                        return false;
                    }
                    if self.isShip(x as isize+1, y as isize-1) {
                        return false;
                    }
                    if self.isShip(x as isize-1, y as isize+1) {
                        return false;
                    }
                    if self.isShip(x as isize+1, y as isize+1) {
                        return false;
                    }
                }
            }
        }

        let ships = self.getShips();

        // Check the maximum size for a 4 problem this will not work.
        if shipSize == 5 {
            if ships[5] != 1 {
                // print(ships)
                return false;
            }
        }
        if shipSize == 4 {
            if ships[5] != 1 || ships[4] != 1 {
                // print(ships)
                return false;
            }
        }

        // Looks good!!
        return true;
    }


    // Launch a command line to solve the specified long solver position.
    fn launch(&self, numPositions: usize, totalPositions: usize) {
        if totalPositions > 0 {
            self.writeFile(format!("Long ships position {} of {}\n", numPositions, totalPositions), true);
        }
        if self.start as usize > numPositions {
            return;
        }

        // self.displayPosition();

        let mut args: Vec<String> = [].to_vec();
        args.push("-g".to_string());
        args.push(format!("{}", self.index));
        args.push("-t".to_string());
        args.push(format!("{}", self.threads));
        args.push("-k".to_string());
        args.push("-m".to_string());
        for x in 0..self.line.len() {
            args.push(format!("{}", self.line[x]));
        }
        if false {
            println!("{:?}", args);
        }

        // Execute this command and wait for it to finish.
        let mut command = std::process::Command::new("target/debug/battleships");
        command.args(args);
        let mut child = command.spawn().unwrap();
        let _result = child.wait();
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
