use std::io;
use std::f64;

//--------------------------Functions for Program------------------------------------------//
fn onetoten(x: u64) {

    print!("{}", ones[(x) as usize]);
}

fn twentytohundred(x:u64)-> u64{
    if(x<20){
        return(x);
    }

    print!("{}", tens[(x/10) as usize]);

    if x%10!=0{
        print!("-");
    }
    return x%10;
}

fn specialcase(x:u64){
    let mut rem=0;
    if(x<20)
        {
            onetoten(x);
        }
        else
        {
            rem=twentytohundred(x);
            onetoten(rem);
        }
}

fn hundredtothousand(x:u64)-> u64{
    if(x<100){
        return x-(x/100)*100;
    }

    print!("{} hundred ", ones[(x/100) as usize]);
    return x-(x/100)*100;
}

fn thousand_plus(x:u64,y:u32)->u64{//thousands y=5     million y=8   billion=11 etc..
    let base: u64=10;

    if x<base.pow(y) {
        let z:u64=x / (base.pow(y-2));
        specialcase(z);

        if y==5 && z!=0{
            print!(" thousand,");
        }
        if y==8 && z!=0{
            print!(" million, ");
        }
        if y==11 && z!=0{
            print!(" billion, ");
        }
        if y==14 && z!=0 {
            print!(" trillion, ");
        }
        if y==17 && z!=0{
            print!(" quadrillion, ");
        }

        return x - (x / (base.pow(y-2))) * (base.pow(y-2));
    }

    let zero_check=((x -(x / base.pow(y)) * base.pow(y))-x%base.pow(y-2))/base.pow(y-2);
    hundredtothousand(x/base.pow(y-2));
    specialcase(((x -(x / base.pow(y)) * base.pow(y))-x%base.pow(y-2))/base.pow(y-2));
    if y==5 {
        print!(" thousand ");
    }
    if y==8 {
        print!(" million ");
    }
    if y==11 {
        print!(" billion ");
    }
    if y==14 {
        print!(" trillion ");
    }
    if y==17 {
        print!(" quadrillion ");
    }
    if zero_check!=0{
        print!(",");
    }
    return x - (x / base.pow(y-2)) * base.pow(y-2);
}



fn quintillion(x:u64)-> u64{
    specialcase(x / 1000000000000000000);
    print!(" quintillion, ");
    return x - ((x / 1000000000000000000) * 1000000000000000000);
}

//----------------------------------End Functions----------------------------------------------//



const ones: &'static [&'static str] = &["", "one", "two", "three", "four","five", "six", "seven", "eight", "nine","ten", "eleven", "twelve", "thirteen", "fourteen","fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
const tens: &'static [&'static str] = &["","ten","twenty", "thirty", "forty", "fifty","sixty", "seventy", "eighty", "ninety"];


fn main() {
    println!("Please Enter an Integer (positve or negative of a max 64 bit integer): ");
    let mut inputstr = String::new();
    io::stdin()
        .read_line(&mut inputstr)
        .expect("Failed to read line.");

    let mut counter=0;
    let mut negative_counter=0;


    for c in inputstr.chars() {

        if c=='-' || c== '0' || c== '1' || c== '2' || c== '3' || c== '4' || c== '5' || c== '6' || c== '7' || c== '8' || c== '9'|| c== '\0' {
            counter=counter+1;
            if(c=='-'){
                negative_counter=negative_counter+1;
            }
        }
    }
    
    if negative_counter>1 {
        print!("Sorry Invalid Input");
        return;
    }
    if(counter!=inputstr.len()-1) {
        print!("Sorry Invalid Input");
        return;
    }


    let mut isnegative=0;
    let mut inputstring="";
    let s = &inputstr[0..1];
    let d=&inputstr[1..inputstr.len()];
    let input_one: u64 = d
        .trim()
        .parse()
        .expect("");

    if(s=="-" && input_one==0){
        println!("Cannot have Negative Zero");
        return;
    }
    if(s=="-"){
        print!("Negative ");
        isnegative=1;
        inputstring=&inputstr[1..inputstr.len()-1];    //Takes care of negative number case
    }
    else{
        inputstring=&inputstr;
    }


    let input: u64 = inputstring
        .trim()
        .parse()
        .expect("");



    if input==0{
        print!("zero");
    }

    if input>0 && input<=20 {
       onetoten(input);
    }

    if input>=20 && input<100{
        let one_out=twentytohundred(input);
        onetoten(one_out);
    }

    if input>=100 && input<1000{
        let hund_out=hundredtothousand(input);
        let one_out=twentytohundred(hund_out);
        onetoten(one_out);
    }
    if input>=1000 && input<1000000{
        let num:u32=5;
        let thous_out=thousand_plus(input,num);
        if thous_out!=0{
            print!(",");
        }
        let hund_out=hundredtothousand(thous_out);
        let one_out=twentytohundred(hund_out);
        onetoten(one_out);
    }


    if input>=1000000 && input<1000000000{
        let mill_out=thousand_plus(input,8);
        let thous_out=thousand_plus(mill_out,5);
        let hund_out=hundredtothousand(thous_out);
        let one_out=twentytohundred(hund_out);
        onetoten(one_out);
    }

    if input>=1000000000 && input<1000000000000{
        let bill_out= thousand_plus(input,11);
        let mill_out=thousand_plus(bill_out,8);
        let thous_out=thousand_plus(mill_out,5);
        let hund_out=hundredtothousand(thous_out);
        let one_out=twentytohundred(hund_out);
        onetoten(one_out);
    }

    if input>=1000000000000 && input<1000000000000000{
        let trill_out= thousand_plus(input,14);
        let bill_out= thousand_plus(trill_out,11);
        let mill_out=thousand_plus(bill_out,8);
        let thous_out=thousand_plus(mill_out,5);
        let hund_out=hundredtothousand(thous_out);
        let one_out=twentytohundred(hund_out);
        onetoten(one_out);
    }

    if input>=1000000000000000 && input<1000000000000000000{

        let quad_out= thousand_plus(input,17);
        let trill_out= thousand_plus(quad_out,14);
        let bill_out= thousand_plus(trill_out,11);
        let mill_out=thousand_plus(bill_out,8);
        let thous_out=thousand_plus(mill_out,5);
        let hund_out=hundredtothousand(thous_out);
        let one_out=twentytohundred(hund_out);
        onetoten(one_out);
    }
    if input>=1000000000000000000 && input<=9223372036854775807{

        let quin_out= quintillion(input);
        let quad_out= thousand_plus(quin_out,17);
        let trill_out= thousand_plus(quad_out,14);
        let bill_out= thousand_plus(trill_out,11);
        let mill_out=thousand_plus(bill_out,8);
        let thous_out=thousand_plus(mill_out,5);
        let hund_out=hundredtothousand(thous_out);
        let one_out=twentytohundred(hund_out);
        onetoten(one_out);
    }

    }
