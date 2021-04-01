/// A meal must contain a main course, but starter and dessert are optional
#[derive(Debug)]
pub struct Meal {
    starter: Option<String>,
    main: String,
    dessert: Option<String>,
}

impl Meal {
    pub fn builder() -> MealBuilder {
        MealBuilder::default()
    }
}

impl Default for Meal {
    /// A default meal is just Beef Short Rib
    fn default() -> Self {
        Meal {
            starter: None,
            main: "Beef Short Rib".to_owned(),
            dessert: None,
        }
    }
}

#[derive(Default)]
pub struct MealBuilder {
    starter: Option<String>,
    main: String,
    dessert: Option<String>,
}

impl MealBuilder {
    /// A meal must have a main course, so this is a required parameter when making a MealBuilder
    pub fn new(main: &str) -> MealBuilder {
        MealBuilder {
            starter: None,
            main: main.to_owned(),
            dessert: None,
        }
    }

    /// Optionally specify a starter
    pub fn starter(mut self, starter: &str) -> MealBuilder {
        self.starter = Some(starter.to_owned());
        self
    }

    /// Optionally specify a dessert
    pub fn dessert(mut self, dessert: &str) -> MealBuilder {
        self.dessert = Some(dessert.to_owned());
        self
    }

    /// Build a new Meal
    pub fn build(self) -> Meal {
        Meal {
            starter: self.starter,
            main: self.main,
            dessert: self.dessert,
        }
    }
}
