pub fn translate_rust(code: String)
{
    //GUIDE:
    //>: data_ptr += 1;
    //<: data_ptr -= 1;
    //+: memory[data_ptr as usize] += 1;
    //-: memory[data_ptr as usize] -= 1;
    //.: print!(memory[data_ptr as usize] as char);
    //,: Take input?
    //[: while (memory[data_ptr as usize] > 0) {
    //]: }
}

pub fn interpret(code: String)
{
    let mut array: [u8; 65536] = [0; 65536];
    interpret_mapped(code, array);
}

pub fn interpret_mapped(code: String, mut array: [u8; 65536]) {
    let code_array: Vec<char> = code.chars().collect();

    let mut output = String::new();
    let mut data_ptr: u8 = 0;
    let mut code_ptr: usize = 0;
    let mut finished: bool = false;

    'interpret: while !finished {
        let c = code_array[code_ptr];

        if c == '>' { data_ptr+=1; }
        if c == '<' { data_ptr-=1; }
        if c == '+' { array[data_ptr as usize]+=1; }
        if c == '-' { array[data_ptr as usize]-=1; }
        if c == '.' { output.push(array[data_ptr as usize] as char); }
        if c == ',' { continue; } //TODO: Implement this
        if c == '[' {
            if array[data_ptr as usize] == 0 {
                //Search for the matching ]
                let mut found_ptr: usize = code_ptr;
                let mut depth: u8 = 0;
                'search: for i in code_ptr+1..code_array.len() {
                    let search_c: char = code_array[i];
                    if search_c == '[' {
                        depth+=1;
                    } else if search_c == ']' {
                        if depth == 0 {
                            found_ptr = i;
                            break 'search; //Found the right bracket
                        } else {
                            depth-=1;
                        }
                    }
                }

                if code_ptr == found_ptr {
                    //HELP IT BROKE
                    println!("Syntax error in loop at location {}", code_ptr);
                    finished = true;
                }

                code_ptr = found_ptr + 1;
            }
        }
        if c == ']' {
            if array[data_ptr as usize] != 0 {
                //Search for the matching ]
                let mut found_ptr: usize = code_ptr;
                let mut depth: u8 = 0;
                'search: for i in (0..code_ptr).rev() {
                    let search_c: char = code_array[i];
                    if search_c == ']' {
                        depth+=1;
                    } else if search_c == '[' {
                        if depth == 0 {
                            found_ptr = i;
                            break 'search; //Found the right bracket
                        } else {
                            depth-=1;
                        }
                    }
                }

                if code_ptr == found_ptr {
                    //HELP IT BROKE
                    println!("Syntax error in loop at location {}", code_ptr);
                    finished = true;
                }

                code_ptr = found_ptr;
            }
        }

        code_ptr+=1;
        if code_ptr >= code_array.len() {
            finished = true;
            println!("{}",output);
        }
    }
}
