fn main() {
        //Generic vector, stores <t> value
        let t: Vec<i32> = Vec::new();
        //Specific vector, stores i32 values
        let mut i = vec![1, 2, 3, 4, 5];
        //New vector with mutable value, can be added to
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        //First method crashes the program if index is out of bounds
        let does_not_exist = &i[100];
        //Second method returns an None, which is safer
        let does_not_exist = i.get(100);

        //Iterating over a vector
        let iov = vec![100, 32, 57];
        for i in &iov {
        println!("{i}");
    }

        //Using an enum to store multiple types
            enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

        let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    }




