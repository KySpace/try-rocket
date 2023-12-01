use rocket::response::content::RawHtml;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/echo/<num>")]
fn echo(num : u8) -> RawHtml<String> {
    let mut s = format!(r#"
    <!DOCTYPE html>
<!-- saved from url=(0026)https://kyspace.github.io/ -->
<html lang="en-US">
  <head>
  <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
  <title>kyspace.github.io</title>
  </head>
  <body>
    <div class="container-lg px-3 my-5 markdown-body">
      
      <h1>You asked for {}</h1>
      

      <p>Do you know that echo is an identity function?</p>


      
    </div> 
</body></html>
    "#, num.to_string());
    RawHtml(s)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
                .mount("/", routes![echo])
}