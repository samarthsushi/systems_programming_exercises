use std::{
    error::Error, 
    io::{
        self,
        Write, 
        BufReader, 
        BufRead, 
        Stdin
    }, 
    fs::File, 
    process
};

#[derive(Debug)]
pub enum ErrorType {
    TypeErr,
    RangeError,
    WriteErr(Box<dyn Error>),
    FileEmpty,
    CmdErr,
    ArgCountErr,
}

pub struct FileGod {
    file_path: String
}

impl FileGod {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn read_into_vec(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let file = File::open(&self.file_path).or_else(|_| {
            File::create(&self.file_path).and_then(|_| File::open(&self.file_path))
        })?;  
        let reader = BufReader::new(file);
        reader.lines().collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn write(&self, lines: &[String]) -> io::Result<()> {
        let mut file = File::create(&self.file_path)?;
        for line in lines {
            writeln!(file, "{line}")?;
        }
        Ok(())
    }
}

pub struct LineGod {
    lines: Vec<String>
}

impl LineGod {
    pub fn new(lines: Vec<String>) -> Self {
        Self{ lines }
    }

    pub fn insert(&mut self, index: Option<usize>, stdin: &Stdin) ->  Result<(), ErrorType> {
        let mut buf_vec = Vec::new();
        let idx = index.unwrap_or(0);
        if idx > self.lines.len() {
            return Err(ErrorType::RangeError);
        }

        self.input_mode(stdin, &mut buf_vec);
        self.lines.splice(idx..idx, buf_vec);
        Ok(())
    }

    pub fn append(&mut self, stdin: &Stdin) -> Result<(), ErrorType> {
        let mut buf_vec = Vec::new();
        self.input_mode(stdin, &mut buf_vec);
        self.lines.extend(buf_vec);
        Ok(())
    }

    pub fn fetch_lines(&self, range: Option<(usize, usize)>) -> Result<Option<String>, ErrorType> {
        if self.lines.is_empty() {
            return Err(ErrorType::FileEmpty);
        }

        let (start, end) = range.unwrap_or((0, self.lines.len() - 1));
        if start > end || end >= self.lines.len() {
            return Err(ErrorType::RangeError);
        }

        let result = self.lines[start..=end].join("\n");
        Ok(Some(result))
    }

    pub fn find(&self, needle: &str) -> Result<Option<String>, ErrorType> {
        let result = self
            .lines
            .iter()
            .filter(|line| line.contains(needle))
            .cloned()
            .collect::<Vec<String>>()
            .join("\n");

        Ok(Some(result))
    }

    fn input_mode(&self, stdin: &Stdin, buf_vec: &mut Vec<String>) {
        println!("::enteringinputmode");

        for line in stdin.lock().lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue, 
            };
            if line.trim() == "." {
                println!("::exitinginputmode");
                break;
            } else {
                buf_vec.push(line.trim_end().to_string());
            }
        }
    }

    pub fn save(&self, file_handler: &FileGod) -> Result<(), ErrorType> {
        file_handler.write(&self.lines).map_err(|e| ErrorType::WriteErr(Box::new(e)))?;
        Ok(())
    }
}

pub struct TextEditor {
    file_god: FileGod,
    line_god: LineGod
}

impl TextEditor {
    pub fn new(file_path: String) -> Result<Self, Box<dyn Error>> {
        let file_god = FileGod::new(file_path.clone());
        let lines = file_god.read_into_vec()?;
        println!("lines: {}", lines.len());
        let line_god = LineGod::new(lines);

        Ok(Self{
            file_god,
            line_god
        })
    }

    pub fn execute(&mut self, stdin: &Stdin) {
        loop {
            let input = match Self::process_input() {
                Ok(input) => input,
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(1);
                }
            };

            match self.command_handler(&input, stdin) {
                Ok(None) => {}
                Ok(Some(msg)) if msg == "kill" => break,
                Ok(Some(msg)) => println!("{}", msg),
                Err(e) => println!("{e:?}"),
            }
        }
    }

    pub fn process_input() -> Result<String, io::Error> {
        print!("? ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn command_handler(&mut self, input: &str, stdin: &Stdin) -> Result<Option<String>, ErrorType> {
        let mut args = input.split_whitespace();
        match args.next() {
            Some("i") => self.line_god.insert(args.next().and_then(|i| i.parse().ok()), stdin).map(|_| None),
            Some("a") => self.line_god.append(stdin).map(|_| None),
            Some("p") => {
                let n1 = args.next().ok_or(ErrorType::ArgCountErr)?.parse::<usize>().map_err(|_| ErrorType::TypeErr)?;
                let n2 = args.next().ok_or(ErrorType::ArgCountErr)?.parse::<usize>().map_err(|_| ErrorType::TypeErr)?;
                self.line_god.fetch_lines(Some((n1, n2)))
            }
            Some("f") => self.line_god.find(args.next().ok_or(ErrorType::ArgCountErr)?),
            Some("s") => self.line_god.save(&self.file_god).map(|_| None),
            Some("q") => Ok(Some("kill".to_string())),
            _ => Err(ErrorType::CmdErr),
        }
    }
}

impl Drop for TextEditor {
    fn drop(&mut self) {
        match self.line_god.save(&self.file_god) {
            Ok(_) => {},
            Err(_) => {}
        }
    }
}

