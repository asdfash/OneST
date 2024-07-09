use slicestring::Slice;

//modify values here
const YEARS: (i32, i32) =(1800,2024); //allowed range of years
const YEARLETTER: [char; 3] = ['R','S','T']; //allowed letters, arragend in order of represented centurary (acending)
const PQ:[&str; 40] = ["LP","LL","FC","PF","RF","MQ","MM","NB","CC","CS","MB","FM","GS","DO","DP","CP","NR","CM","CD","MD","HS","VH","CH","MH","CL","XL","CX","HX","RP","TU","TC","FB","FN","PA","PB","SS","MC","SM","GA","GB"];
//run through checks
pub fn verify(mut uen:String)->(String,bool){
    //clean whitespace, make all caps
    uen.retain(|c|!c.is_whitespace());

    //check if empty
    if uen.is_empty(){
        return("".to_string(),false)
    }
    uen = uen.to_uppercase();
    //check last character
    if !uen.chars().last().unwrap().is_alphabetic(){
        return ("last character is not alphabetic".to_string(),false)
    }

    //check number and format of characters
    if uen.len() == 9{

        // check string for all num (except last character)
        if !all_num(uen.slice(..8)){
            return("non numerical characters found in UEN, or wrong number of characters".to_string(),false)
        }
        return ("Type A verified".to_owned(),true)
    }
    else if uen.len()== 10 {
        //check last digits for numerical (type B,C)
        if !all_num(uen.slice(5..9)){
            return("trailing characters contain invalid characters".to_string(),false);
        }

        // check for year (type B)
        if all_num(uen.slice(..5)){
            let year:i32 = uen.slice(..4).parse().unwrap();
            if year<YEARS.0 || year > YEARS.1{   
                return("wrong year".to_string(),false)
            }
            return ("Type B verified".to_owned(),true)
        }
        // check alphanumeric cases for type C (refactored due to length)
        return typec(uen.slice(..5))
    }
    return ("wrong number of characters".to_string(),false)
}

//helper function to check that a string only contains numbers
fn all_num(str:String)->bool{
    let mut all_num = true;
    for c in str.chars(){
        if !c.is_numeric(){
            all_num = false;
        }
    }
    return all_num;
}

//type C cases
fn typec(str:String)->(String,bool){
    //
    let arr:Vec<char> = str.chars().collect();
    //check for valid year
    if !YEARLETTER.contains(&arr[0])|| !arr[1].is_numeric() || !arr[2].is_numeric(){
        return("Invalid year".to_string(),false)
    }
    //check if current century
    else if YEARLETTER[YEARLETTER.len()-1] == arr[0]{
        let year:i32 = str.slice(1..3).parse().unwrap();
        if year<YEARS.0%100 || year > YEARS.1%100{   
            return("Invalid year".to_string(),false)
        }
    }
    //check PQ
    let pq = &*str.slice(3..5);
    if !PQ.contains(&pq) {
        return("Invalid entity-type indicator".to_string(),false)
    }
    return("Type C verified".to_string(),true)
}