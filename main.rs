#![allow(non_snake_case)]
use std::i32;
use std::io;
//use std::str;


fn number(_digits: u32) -> String {
    //for single digit numbers

    let mut str = " ".to_string();
    match _digits {
        1 => str.push_str("one"),
        2 => str.push_str("Two"),
        3 => str.push_str("Three"),
        4 => str.push_str("Four"),
        5 => str.push_str("Five"),
        6 => str.push_str("Six"),
        7 => str.push_str("Seven"),
        8 => str.push_str("Eight"),
        9 => str.push_str(" Nine"),
        _ => str.push_str("Nothing"),
    }
    //println!("{}",str);
    return str;
}

fn doublenumber(_digit_copy: u32, _digits: u32) -> String {
    //for double digit number
    let mut str1 = " ".to_string();
    match _digit_copy {
        1 => match _digits {                                        //for number from 10-19
            10 => str1.push_str("Ten"),
            11 => str1.push_str("Eleven"),
            12 => str1.push_str("Twelve"),
            13 => str1.push_str("Thirteen"),
            14 => str1.push_str("Forteen"),
            15 => str1.push_str("Fifteen"),
            16 => str1.push_str("Sixteen "),
            17 => str1.push_str("Seventeen"),
            18 => str1.push_str("Eighteen"),
            19 => str1.push_str("Nineteen"),
            _ => str1.push_str("Nothing"),
        },
        2 => {                                                           //for number 20-90
            str1 = ["Twenty".to_string(), number(_digits)].join(" ");          
        }
        3 => {
            str1 = [" Thirty".to_string(), number(_digits)].join(" ");
        }
        4 => {
            str1 = [" Forty".to_string(), number(_digits)].join(" ");
        }
        5 => {
            str1 = [" Fifty".to_string(), number(_digits)].join(" ");
        }
        6 => {
            str1 = [" Sixty".to_string(), number(_digits)].join(" ");
        }
        7 => {
            str1 = [" Seventy".to_string(), number(_digits)].join(" ");
        }
        8 => {
            str1 = [" Eighty".to_string(), number(_digits)].join(" ");
        }
        9 => {
            str1 = [" Ninety".to_string(), number(_digits)].join(" ");
        }
        _ => println!("Nothing"),
    }
    return str1;                  //string return
}

fn word(_int_amount: u32, _amount1: f64) ->String
 {
    let mut str2 = " ".to_string();
    let _diff: f64 = _amount1 - f64::from(_int_amount);
    let _fn: f64 = _diff * f64::from(100);        //finding float value
    let _int_fn = _fn.round();                   //round function for float value
    let _decimalNo = _int_fn as u32;            //converting to u32
    let mut _x = _int_amount;
    let mut _intlength = 0;
    while _x != 0 {
        _x /= 10;                              //calculating the length of integer amount 
        _intlength += 1;
    }
    let mut _x1 = _decimalNo;
    let mut _floatlength = 0;
    while _x1 != 0 {
        _x1 /= 10;                             //calculating the length of float amount 
        _floatlength += 1;
    }
    let mut _intlen1 = _intlength;
    _intlen1 .to_string();
     /*if _intlen1 > 9 
            {
                     println!("Enter the number which will have 9 digits before decimal point");
            }*/
     //else {
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
            match _intlen1 {                                        //match case from single digit to 9 digit number
                1 => {
                    print!(" ");
                    str2 = [str2, number(_digits)].join(" ");         
                    _intlen1 -= 1
                }
                2 => {
                    str2 = [str2, doublenumber(_digit_copy, _digits)].join(" ");   
                    _intlen1 -= 1;
                    _intlen1 -= 1;
                }
                3 => {
                    str2 = [str2, number(_digits), "Hundred".to_string()].join(" ");
                    _intlen1 -= 1;
                }
                4 => {
                    str2 = [str2, number(_digits), "Thousand".to_string()].join(" ");
                    _intlen1 -= 1;
                }
                5 => {
                    str2 = [
                        str2,
                        doublenumber(_digit_copy, _digits),
                        "Thousand".to_string(),
                    ]
                    .join(" ");
                    _intlen1 -= 1;
                    _intlen1 -= 1;
                }
                6 => {
                    str2 = [str2, number(_digits), "Lakh".to_string()].join(" ");
                    _intlen1 -= 1;
                }
                7 => {
                    str2 = [str2, doublenumber(_digit_copy, _digits), "Lakh".to_string()].join(" ");
                    _intlen1 -= 1;
                    _intlen1 -= 1;
                }
                8 => {
                    str2 = [str2, number(_digits), "Crore".to_string()].join(" ");
                    _intlen1 -= 1;
                }
                9 => {
                    str2 = [
                        str2,
                        doublenumber(_digit_copy, _digits),
                        "Crore".to_string(),
                    ]
                    .join(" ");
                    _intlen1 -= 1;
                    _intlen1 -= 1;
                }
                _ => println!("Nothing"),
            }
        }
        //println!("{}", str2);
        //print!(" ");
        
        if _decimalNo != 0 {                                          //decimal Number addition to the string
            let mut _decimalNo_pow = u32::pow(10, _floatlength - 1);
            str2 = [str2,_decimalNo.to_string(),"/".to_string(), _decimalNo_pow.to_string()].join(" ");
        }
       return str2;                      //string return
    }  
//}

fn main() {
    println!(".............Currency_To_Text...............");
    // Request for entering number 1
    loop {
        let mut _loop_value = String::new();               //_loop_value
        let mut _amount = String::new();                  //user input for currecny amount
        println!("Enter the Currency amount:");           
        io::stdin()
            .read_line(&mut _amount)
            .expect("Failed to read line");               //user input
        let _floatamount: f64 = _amount
            .trim()
            .parse()
            .ok()
            .expect("Program only process numbers,Enter number"); //converting string to int
                                                                  //println!("{}", _amount1);

        let _int_amount = _floatamount as u32;               //interger currency
        if _int_amount == 0 {                           //for zero value
            println!("Zero")
        } else {
            print!("In words:");
            println!("{}" ,word(_int_amount, _floatamount));            //word function call
        }
        println!();
        println!("If you want to convert currency to text again enter 1:");
        io::stdin()
            .read_line(&mut _loop_value)
            .expect("Failed to read line");                          //user input
        let mut _loopval1: i32 = _loop_value.trim().parse().ok().expect("Enter 1 to repeat"); //converting to int
        if _loopval1 != 1 {
            println!("Loop break ");
            break;
        }
    }
}
