mod foo;

/// Serialise the Vec of DataPoints into a Vec of csv Strings (including a header)
fn write_to_csv(data: &[foo::DataPoint]) -> Vec<String> {
    let mut csv_vec = Vec::<u8>::new();
    {
        let mut csv_writer = csv::Writer::from_writer(&mut csv_vec);
        for dp in data {
            csv_writer.serialize(dp).unwrap();
        }

        csv_writer.flush().unwrap();
    }

    let tmp = String::from_utf8(csv_vec).unwrap();

    let vec_of_csv_strings = tmp.lines().map(|line| line.to_owned()).collect();
    vec_of_csv_strings
}

/// Serialise the Vec of DataPoints into a Vec of JSON Strings
fn write_to_json(data: &[foo::DataPoint]) -> Vec<String> {
    let mut json_vec = Vec::<String>::new();

    for dp in data {
        json_vec.push(serde_json::to_string(&dp).unwrap());
    }

    json_vec
}

/// Serialise the Vec of DataPoints into a Vec of TOML Strings
fn write_to_toml(data: &[foo::DataPoint]) -> Vec<String> {
    let mut toml_vec = Vec::<String>::new();

    for dp in data {
        toml_vec.push(toml::to_string(&dp).unwrap());
    }

    toml_vec
}

fn main() {
    let dp1 = foo::DataPointBuilder::new()
        .epoch(1617275437)
        .name("Alice")
        .details("Did a thing!")
        .value(8)
        .flag(true)
        .build();

    let dp2 = foo::DataPointBuilder::new()
        .epoch(1616543211)
        .name("Bob")
        .details("Took a nap!")
        .value(42)
        .flag(false)
        .build();

    let dp3 = foo::DataPointBuilder::new()
        .epoch(1601255511)
        .name("Charlie")
        .details("Went shopping!")
        .value(99)
        .flag(true)
        .build();

    let data = vec![dp1, dp2, dp3];
    println!("{:?}", &data);

    // CSV
    println!("-----------------------------");
    let csv_vec = write_to_csv(&data);
    for line in csv_vec {
        println!("{}", line);
    }

    // JSON
    println!("-----------------------------");
    let json_vec = write_to_json(&data);
    for line in json_vec {
        println!("{}", line);
    }

    // TOML
    println!("-----------------------------");
    let toml_vec = write_to_toml(&data);
    for line in toml_vec {
        println!("{}", line);
    }
}
