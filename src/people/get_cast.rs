use crate::people::people_structs::cast;

pub(crate) async fn get_cast_details(person_id: i32) -> cast {
    // https://api.themoviedb.org/3/person/2524?api_key=4e110fd06d91f1f01af5acd9fa42a82d&language=en-US

    let mut get_cast_url = String::from("https://api.themoviedb.org/3/person/");
    get_cast_url.push_str(&*person_id.to_string());
    get_cast_url.push_str("?api_key=4e110fd06d91f1f01af5acd9fa42a82d&language=en-US");

    let cast_object: cast = reqwest::Client::new()
        .get(get_cast_url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    cast_object
}
