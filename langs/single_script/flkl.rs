
fn main() {
    let x = lext(readFile("test.flkl"));
    let p = parses(x);
    exc(p.clone(), 0, "".to_string());
}
pub fn readFile(names: &str) -> String {
    //reads the file from names
    let s = "".to_string();
    let fnm = s+&names;
    let contents =  std::fs::read_to_string(fnm).unwrap();
    return contents;

    
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum TT {
    char(String),
}
#[derive(Clone)]
pub struct Coder {
    pub lex: Vec<TT>,
}
impl Coder {
    pub fn push(&mut self, tt: TT) {
        self.lex.push(tt);
    }
    pub fn new() -> Coder {
        Coder { lex: Vec::new() }
    }
    // finds the closesest TT after the current_line
    pub fn next(&self, chr: &str, current_line: usize) -> usize {
        let mut _retur = 0;
        //what the fuck did i make here? 
        let x = self.lex[current_line..self.lex.len()]
            .iter()
            .position(|x| *x == TT::char(chr.to_string()));
        let mut _pos = 0;
        match x {
            Some(x) => {
                _pos = x + current_line;
                _retur = _pos;
                //println!("test {}", pos);
            }
            None => {
                panic!("cannot find the stop for the if");
            }
        }
        if _retur != 0 {
            return _retur;
        }
        panic!("Next not found {},{}", chr, current_line)
    }
}
//this is the lexer that makes everything into a long vector of garbage that the parser then makes into a parsed vector the inter can read
pub fn lext(code: String) -> Vec<Coder> {
    let mut coders: Vec<Coder> = Vec::new();
    let code: Vec<&str> = code.split("n").collect();
    for code in code {
        let code: Vec<String> = code.chars().map(|x| x.to_string()).collect();
        let code: Vec<&str> = code.iter().map(|x| x.as_str()).collect();
        let mut holder = Coder::new();
        for char in code.into_iter() {
            match char {
                "\n" => {}
                a => {
                    holder.push(TT::char(a.to_string()));
                } //_=> holder.push(TT::Unknown)
            }
        }
        coders.push(holder);
    }
    return coders;
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    print,
    minus(i64),
    moveline(usize),
    clear,
    quit,
    load(String),
    If(String,usize),
}
#[derive(Clone)]
pub struct Parse {
    pub parsed_data: Vec<Command>,
}
impl Parse {
    pub fn new() -> Parse {
        Parse {
            parsed_data: Vec::new(),
        }
    }
    pub fn push(&mut self, ty: Command) {
        self.parsed_data.push(ty);
    }
}

fn parses(code: Vec<Coder>) -> Vec<Parse> {
    let mut pr: Vec<Parse> = Vec::new();
    for code in code {
        let mut prs = Parse::new();
        let mut mof = 0;
        for x in 0..code.lex.len() {
            let x = x + mof;
            if x >= code.lex.len() {
                break;
            }
            let ln = code.lex[x].clone();
            let mut s = "".to_string();

            match ln {
                TT::char(a) => s.push_str(a.as_str()),
            }
            match s.as_str().clone() {
                "a" => {}
                "f" => {
                    let op = code.next("|", x);
                    prs.push(Command::load(col([x + 1, op], code.clone())));
                    mof += op - x;
                }
                "l" => {
                    let op = code.next("a", x);
                    let line = col([x + 1, op], code.clone()).parse::<usize>().unwrap();
                    prs.push(Command::moveline(line));
                    mof += op - x;
                }
                "p" => prs.push(Command::print),
                "e" => {
                    let op1 = code.next("|", x);
                    let arg1 = col([x + 1, op1], code.clone());
                    let op = code.next("a", x);
                    let arg2 = col([op1+1, op], code.clone()).parse::<usize>().unwrap();
                    //panic!("{}",arg2);
                    prs.push(Command::If(arg1,arg2));
                    mof += op - x;
                }
                "m" => {
                    let op = code.next("a", x);
                    let min = col([x + 1, op], code.clone()).parse::<i64>().unwrap();
                    prs.push(Command::minus(min));
                    mof += op - x;
                }
                "q"=>prs.push(Command::quit),

                a => {
                    panic!("not implemented {},{},{}", a, mof, x)
                }
            }
        }
        pr.push(prs);
    }
    pr
}
fn col(x: [usize; 2], code: Coder) -> String {
    let mut str = "".to_string();
    for x in x[0]..x[1] {
        match code.lex[x].clone() {
            TT::char(a) => {
                str.push_str(a.as_str());
            }
        }
    }
    str
}
fn exc(ps: Vec<Parse>, i: usize, data: String) {
    let p = ps[i].clone();
    let mut data = data;
    for x in p.parsed_data.clone() {
        match x {
            Command::load(s) => {
                data = s;
            }
            Command::print => {
                println!("{}", data);
            }
            Command::moveline(a) => {
                exc(ps.clone(), a, data.clone());
                break;
            }
            Command::If(a,b) => {
                 
                if data == a {
                    exc(ps.clone(), b, data.clone());
                    break;
                }
                
            }
            Command::minus(s) => {
                if data.parse::<i64>().is_ok() {
                    let d = data.parse::<i64>().unwrap();
                    let d = d - s;
                    data = d.to_string();
                }
            }
            Command::quit =>{
                panic!("quit");
            }

            _ => {
                panic!("idiot");
            }
        }
    }
}