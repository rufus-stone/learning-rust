mod meal;

fn main() {
    let meal = meal::MealBuilder::new("ChickieNobs Bucket O'Nubbins")
        .starter("Scallops")
        .dessert("Key Lime Pie")
        .build();

    println!("{:?}", meal); // Meal { starter: Some("Scallops"), main: "ChickieNobs Bucket O\'Nubbins", dessert: Some("Key Lime Pie") }

    let meal = meal::Meal::default();

    println!("{:?}", meal); // Meal { starter: None, main: "Beef Short Rib", dessert: None }
}
