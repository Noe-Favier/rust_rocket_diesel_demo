#[macro_use] extern crate rocket;
extern crate tera;

pub mod models;
pub mod schema;

//imports ~ rocket + templates
use rocket_dyn_templates::{Template, context};
use rocket::response::content;

//diesel
use diesel::mysql::MysqlConnection as SQLConnection;
/*
    > en fonction de votre moteur de bdd vous auriez pu mettre : 
use diesel::pg::PgConnection as SQLConnection;
use diesel::sqlite::SqliteConnection as SQLConnection;
*/
use diesel::prelude::*;
use dotenvy::dotenv;

//autres
use std::env;


#[get("/")]
fn index() -> content::RawHtml<Template> { //on retourne du HTML, généré par un template
    use self::schema::liste::dsl::*; //permet l'utilisation d'alias 
    let mut c = establish_connection(); //fonction définie plus bas dans le code

    let results = liste.load::<models::Liste>(&mut c).expect("Impossible de charger la liste"); //on charge TOUTES les lignes de notre table `liste`

    content::RawHtml(Template::render(
        "index", //nom du template (index.tera)
        context! { list: results }, //on passe en argument à notre template le résultat de notre requête sql 
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index]) //on active notre route
    .attach(Template::fairing()) //on ajoute le templating au cycle de vie de rocket
}

pub fn establish_connection() -> SQLConnection {
    dotenv().ok(); //charge les variables présente dans le .env dans l'environnement
    let database_url = env::var("DATABASE_URL") //on tente de récuperer l'url de la BDD depuis l'environnement
        .expect("DATABASE_URL must be set"); //si elle n'existe pas on lève une erreur

        SQLConnection::establish(&database_url) //on tente d'établir une connexion avec la BDD
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)) //on retourne cette connexion (ou une erreur si connexion impossible)
}