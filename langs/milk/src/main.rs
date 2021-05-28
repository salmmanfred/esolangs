fn main() {
    let x = lext(readFile("test.milk"));
    let p = parses(x);
    exc(p.clone(), [0,p.parsed_data.len()], "".to_string());
}
pub fn readFile(names: &str) -> String {
    //reads the file from names
    let s = "".to_string();
    let fnm = s + &names;
    let contents = std::fs::read_to_string(fnm).unwrap();
    return contents;
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum TT {
    char(String),
    Space,
    End,
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
    pub fn next(&self, _tt: TT, chr: &str, current_line: usize) -> usize {
        let mut _retur = 0;
        let mut x: Option<usize>;
        match _tt {
            TT::char(_) => {
                x = self.lex[current_line..self.lex.len()]
                    .iter()
                    .position(|x| *x == TT::char(chr.to_string()));
            }

            _ => {
                x = self.lex[current_line..self.lex.len()]
                    .iter()
                    .position(|x| *x == _tt);
            }
        }

        let mut _pos = 0;
        match x {
            Some(x) => {
                _pos = x + current_line;
                _retur = _pos;
                //println!("test {}", pos);
            }
            None => {
                panic!("cannot find TokenType {:#?}",_tt);
            }
        }
        if _retur != 0 {
            return _retur;
        }
        panic!("Next not found {},{}", chr, current_line)
    }
    pub fn have(&self, _tt:TT,ind:usize)->bool{
        match _tt {
            TT::char(a) => {
                return self.lex.clone()[ind..self.lex.len()].into_iter().any(move|e| e == &TT::char(a.clone()));

            }
            _=>{
                return self.lex.clone()[ind..self.lex.len()].into_iter().any(|e| e == &_tt);

            }
        }
    }
}
//this is the lexer that makes everything into a long vector of garbage that the parser then makes into a parsed vector the inter can read
pub fn lext(code: String) -> Coder {
    let code: Vec<String> = code.chars().map(|x| x.to_string()).collect();
    let code: Vec<&str> = code.iter().map(|x| x.as_str()).collect();
    let mut holder = Coder::new();
    for char in code.into_iter() {
        match char {
            "\n" => {}
            " " => {
                holder.push(TT::Space);
            }
            "." =>{
                holder.push(TT::End);
            }
            a => {
                holder.push(TT::char(a.to_string()));
            }
        }
    }
    holder
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    printy(usize),
    print(String),

    minus(usize,i64),
    moveline(usize),
    newvar(String),
    clear,
    quit,
    load(String),
    milk,
    If(String, usize),
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

fn parses(code: Coder) -> Parse {
    let mut prs = Parse::new();
    let mut mof = 0;
    for x in 0..code.lex.len() {
        let x = x + mof;
        if x >= code.lex.len() {
            break;
        }
        let ln = code.lex[x].clone();
        let mut s = "".to_string();

        /*match ln {
            TT::char(a) => s.push_str(a.as_str()),
        }*/
        match ln.clone() {
            TT::char(a) => {
                let spc = code.next(TT::Space, "", x);
                match col([x, spc], code.clone()).as_str() {
                    "add" => {
                        // ! Add branch
                        let pos = code.next(TT::End, "", x);
                        match col([spc+1, pos], code.clone()).as_str() {
                            "milk" => {
                                prs.push(Command::milk);
                            }
                            a => {
                                panic!("adder What?! {}", a)
                            }
                        }
                        mof += pos - x;
                    }
                    "cheese" => {
                        // ! cheese branch
                        let pos = code.next(TT::char("".to_string()), "(", x);
                        match col([spc+1, pos], code.clone()).as_str() {
                            "minus" => {
                                let ind = col([pos,code.next(TT::char("".to_string()), ")", x)],code.clone()).parse::<usize>().unwrap();
                                let ch = col([ind,code.next(TT::End, ".", x)],code.clone()).parse::<i64>().unwrap();

                                prs.push(Command::minus(ind,ch));
                            }
                            a => {
                                panic!("adder What?! {}", a)
                            }
                        }
                        mof += pos - x;
                    }
                    "pour" => {
                        // ! pour branch
                        let mut pos = 0;
                        let end = code.next(TT::End,"x",spc+1);

                        if code.have(TT::Space, spc+1){
                            pos = code.next(TT::Space, "(", spc+1);
                        }
                        

                        if pos > end || pos <= spc+1{
                           pos = end;
                        }


                        match col([spc+1, pos], code.clone()).as_str() {
                            "yoghurt" => {
                                let ind = col([pos,code.next(TT::End, ")", x)],code.clone()).parse::<usize>().unwrap();

                                prs.push(Command::printy(ind));
                            }
                            a => {
                                prs.push(Command::print(a.to_string()));

                            }
                        }
                       // println!("{}",pos);
                        mof += end - x;
                    }
                    "get" => {
                        // ! get branch
                        match col([x, code.next(TT::Space, "", x)], code.clone()).as_str() {
                            "milk" => {
                                
                            }
                            a => {
                                panic!("What?! {}", a)
                            }
                        }
                    }
                    "check" => {
                        // ! check branch
                        match col([x, code.next(TT::Space, "", x)], code.clone()).as_str() {
                            "milk" => {
                                
                            }
                            a => {
                                panic!("What?! {}", a)
                            }
                        }
                    }
                    "spoil" => {
                        // ! spoil branch
                        match col([x, code.next(TT::Space, "", x)], code.clone()).as_str() {
                            "milk" => {
                                
                            }
                            a => {
                                panic!("What?! {}", a)
                            }
                        }
                    }
                    "yoghurt" => {
                        // ! Yoghurt branch
                        let pos = code.next(TT::End, "(", x);
                        match col([spc+1, pos], code.clone()).as_str() {
                           
                            a => {
                                //let ch = col([pos,code.next(TT::End, ".", x)],code.clone());

                                prs.push(Command::newvar(a.to_string()));
                            }
                        }
                        mof += pos - x;
                    }
                    a => {
                        panic!("What?! {}", a)
                    }
                }
            }

            d => {
                panic!("not implemented {:#?}",d)
            }
        }
    }
    prs
}
//collects a string from one point to another
fn col(x: [usize; 2], code: Coder) -> String {
    let mut str = "".to_string();
    for x in x[0]..x[1] {
        match code.lex[x].clone() {
            TT::char(a) => {
                str.push_str(a.as_str());
            }
            _ => {}
        }
    }
    str
}
//runs the parsed data
fn exc(ps: Parse,pos:[usize;2], data: String) {
    let p = ps.clone();
    drop(ps);
    let mut stack: Vec<String>= Vec::new();
    let mut data = data;
    for x in p.parsed_data[pos[0]..pos[1]].iter(){
        let x = x.to_owned();
        match x {
            Command::load(s) => {
                data = s;
            }
            Command::print(s) => {
                println!("{}", s);
            }
            Command::printy(s) => {
                println!("{}", stack[s]);
            }
            Command::milk =>{
                println!("mio")
            }
            Command::newvar(a) =>{
                stack.push(a);
            }
            /*Command::moveline(a) => {
                exc(ps.clone(), a, data.clone());
                break;
            }
            Command::If(a, b) => {
                if data == a {
                    exc(ps.clone(), b, data.clone());
                    break;
                }
            }*/
            Command::minus(s,d) => {
                if data.parse::<i64>().is_ok() {
                    let d = data.parse::<i64>().unwrap();
                    let d = d - d;
                    data = d.to_string();
                }
            }
            Command::quit => {
                panic!("quit");
            }

            _ => {
                panic!("idiot");
            }
        }
    }
}
