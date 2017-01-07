use rocket::response::content::{HTML, JSON};
use serde_json;
use quotes;

#[get("/")]
pub fn index_html() -> HTML<String> {
    let quote = match quotes::get_random_quote() {
        Some(q) => q,
        None => {return HTML("
        The database could not be accessed.
        If you are the administrator of this instance,
        you must initialize the database.".into())}
    };
    let source_text = quote.get_source_as_text();
    HTML(format!("
    <html>
    <head>
    <title> Rocket Fortune </title>
    <style type='text/css'>
    body {{
        background-color: black;
        color: green;
        font-family: monospace;
    }}
    a:link {{
        color: green;
        text-decoration: none;
    }}
    a:visited {{
        color: green;
        text-decoration: none;
    }}
    a:active {{
        color: green;
        text-decoration: underline;
    }}
    a:hover {{
        color: green;
        text-decoration: underline;
    }}
    h1 h2 h3 {{
        text-align: right;
    }}
    blockquote {{
        padding: 4px;
        border-radius: 2px;
        background-color: #555;
        color: white;
    }}
    .quote-container {{
        padding: 10px;
        margin: 10px;
        border-radius: 2px;
        background-color: #333;
    }}
    .main-container {{
        margin-left: 25%;
        margin-right: 25%;
        margin-top: 20px;
        padding: 10px;
        border-radius: 2px;
        min-width: 200px;
        background-color: #111;
    }}
    </style>
    </head>
    <body>
    <div class='main-container'>
    <h1>Rocket Fortune</h1>
    <h3>A simple fortune application, built with Rocket.</h3>
    <div class='quote-container'>
    <blockquote>{}</blockquote>
    <br /> <strong>{}</strong> ({})
    </div>
    <br />
    <a href='http://rocket.rs'>Rocket</a> |
    <a href='http://silverwingedseraph.net'>My Blog</a> |
    <a href='http://twitter.com/leotindall'>My Twitter</a> |
    <a href='/json'> JSON version </a>
    </div>
    </body>
    </html>
    ", quote.quote, quote.author, source_text))
}

#[get("/json")]
pub fn json() -> JSON<String> {
    let quote = quotes::get_random_quote().unwrap();
    JSON(serde_json::to_string(&quote).unwrap())
}
