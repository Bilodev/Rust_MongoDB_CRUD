pub struct Customer {
    pub username:String,
    pub email:String,
    pub phone:u32,
}
#[warn(non_camel_case_types)]


pub fn readln( mut var:String ) -> String{

    std::io::stdin()
    .read_line(&mut var)
    .expect("Errore");

    return  var 
}