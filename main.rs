#![allow(non_snake_case)]
use std::i32;
use std::io;
fn number(_digits: u32) {            //for single digit numbers
    match _digits {
        1 => print!("One"),
        2 => print!("Two"),
        3 => print!("Three"),
        4 => print!("Four"),
        5 => print!("Five"),
        6 => print!("Six"),
        7 => print!("Seven"),
        8 => print!("Eight"),
        9 => print!(" Nine"),
        _ => print!("Nothing"),
    }
}



fn doublenumber(_digit_copy: u32, _digits: u32) {         //for double digit number
    match _digit_copy {
        1 => match _digits {
            10 => print!("Ten"),
            11 => print!("Eleven"),
            12 => print!("Twelve"),
            13 => print!("Thirteen"),
            14 => print!("Forteen"),
            15 => print!("Fifteen"),
            16 => print!("Sixteen "),
            17 => print!("Seventeen"),
            18 => print!("Eighteen"),
            19 => print!("Nineteen"),
            _ => print!("Nothing"),
        },
        2 => {
            print!(" Twenty");
            number(_digits);
        }
        3 => {
            print!(" Thirty");
            number(_digits);
        }
        4 => {
            print!(" Forty");
            number(_digits);
        }
        5 => {
            print!(" Fifty");
            number(_digits);
        }
        6 => {
            print!(" Sixty");
            number(_digits);
        }
        7 => {
            print!(" Seventy");
            number(_digits);
        }
        8 => {
            print!(" Eighty");
            number(_digits);
        }
        9 => {
            print!(" Ninety");
            number(_digits);
        }
        _ => println!("Nothing"),
    }
}




fn word(_int_amount: u32, _amount1: f64) {
    let _diff: f64 = _amount1 - f64::from(_int_amount);
     let _fn: f64 = _diff * f64::from(100); //finding float value
    let _int_fn = _fn.round(); //round function for float value
    let _decimalNo = _int_fn as u32;
    let mut _x = _int_amount;
    let mut _intlength = 0;
    while _x != 0 {
        _x /= 10;
        _intlength += 1;
    }
    let mut _x1 = _decimalNo;
    let mut _floatlength = 0;
    
    while _x1 != 0 {
        _x1 /= 10;
        _floatlength += 1;
    }
    let mut _intlen1 = _intlength;
    if _intlen1 > 9 {
        println!("Enter the number which will have 9 digits before decimal point")
    } else {
        let mut _amount_pow = u32::pow(10, _intlength - 1);
        while _intlength > 0 {
            let mut _p = _int_amount / _amount_pow; //Spilt the digit in the way they appear eg.123   1,2,3
            let mut _digits = _p % 10;
            _amount_pow = _amount_pow / 10;
            _intlength -= 1;
            let mut _digit_copy = _digits;
            if _intlength == 1 || _intlength == 4 || _intlength == 6 || _intlength == 8
            // for  2 digit numbers eg.10,20,30
            {
                if _digits == 1
                //for 10-19 values
                {
                    _p = _int_amount / _amount_pow;
                    _digits = _digits * 10 + _p % 10; //gives value 10,12,..,19
                    _amount_pow = _amount_pow / 10;
                    _intlength -= 1;
                } else {
                    _p = _int_amount / _amount_pow; //for no other than 10-19
                    _digits = _p % 10;
                    _amount_pow = _amount_pow / 10;
                    _intlength -= 1;
                    if _digits == 0 {
                        _intlength = 0; //so that loop will break here after getting 0
                    }
                }
            }
            if _digit_copy == 0 {
                _intlen1 = 0; //for 100,1000,10000
            }
            match _intlen1 {
                1 => {
                    print!(" ");
                    number(_digits);
                    _intlen1 -= 1
                }
                2 => {
                    doublenumber(_digit_copy, _digits);
                    _intlen1 -= 1;
                    _intlen1 -= 1;
                }
                3 => {
                    number(_digits);
                    print!(" Hundred");
                    _intlen1 -= 1;
                }
                4 => {
                    number(_digits);
                    print!(" Thousand ");
                    _intlen1 -= 1;
                }
                5 => {
                    doublenumber(_digit_copy, _digits);
                    print!(" Thousand");
                    _intlen1 -= 1;
                    _intlen1 -= 1;
                }
                6 => {
                    number(_digits);
                    print!(" Lakh");
                    _intlen1 -= 1;
                }
                7 => {
                    doublenumber(_digit_copy, _digits);
                    print!(" Lakh");
                    _intlen1 -= 1;
                    _intlen1 -= 1;
                }
                8 => {
                    number(_digits);
                    print!(" Crore");
                    _intlen1 -= 1;
                }
                9 => {
                    doublenumber(_digit_copy, _digits);
                    print!(" Crore");
                    _intlen1 -= 1;
                    _intlen1 -= 1;
                }
                _ => println!("Nothing"),
            }
        }
        print!(" ");
        if _decimalNo != 0 {
            let mut _decimalNo_pow = u32::pow(10, _floatlength - 1);
            print!("{}", _decimalNo);
            print!("/");
            print!("{}", _decimalNo_pow);
        }
    }
}

fn main() {
    println!(".............Currency_To_Text...............");
    // Request for entering number 1
   
    loop {
         let mut _loop_value = String::new();
        let mut _amount = String::new(); //user input for currecny amount
        println!("Enter the Currency amount:");
        io::stdin()
            .read_line(&mut _amount)
            .expect("Failed to read line"); //user input
        let _amount1: f64 = _amount
            .trim()
            .parse()
            .ok()
            .expect("Program only process numbers,Enter number"); //converting string to int
                                                                  //println!("{}", _amount1);

        let _int_amount = _amount1 as u32; //interger currency
         
         
        if _int_amount == 0 {
            println!("Zero")
        } else {
            print!("In words:");
            word(_int_amount, _amount1);
        }
        println!();
        println!("If you want to convert currency to text again enter 1:");
        io::stdin()
            .read_line(&mut _loop_value)
            .expect("Failed to read line"); //user input
             
        let mut _loopval1: i32 = _loop_value.trim().parse().ok().expect("Enter 1 to repeat"); //converting to int
        
        if _loopval1 != 1 {
            println!("Loop break ");
            break;
        }   
    }
}
