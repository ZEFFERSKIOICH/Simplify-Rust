/*
 * a)for multiple single variables of same type considering destructing the vector after vin()
   b)for multiple variables of different types replace vector with tuple and add similar generic types
     destructing the tuple after  calling vin()
*/
//#![allow(dead_code,unused_variables)]   uncomment this to remove such warnings 
use std::io;
//read single value in a line
#[inline(always)]
fn cin<T>()->T
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug     //ouput of Err type of FromStr is passed to Debug to unwrap()
    {
        let mut val=String::new();
        io::stdin().read_line(&mut val).unwrap();
        let val:T=val.trim().parse().unwrap();
        return val;
    }


//read entire line 
#[inline(always)]
fn vin<T>()->Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug     //ouput of Err type of FromStr is passed to Debug to unwrap()
    {
        let mut val=String::new();
        let mut v:Vec<T>=Vec::new();
        io::stdin().read_line(&mut val).unwrap();
        for words in val.split_whitespace(){
            v.push(words.trim().parse::<T>().unwrap());
        }
        return v;
    }


fn main(){
    // let n:usize=cin();
    // let mut v:Vec<i64>=vin();
}
