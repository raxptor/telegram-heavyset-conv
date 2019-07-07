use csv::ReaderBuilder;

pub fn name(input:&str) -> String {
    match input {
        "Squat" => "Knäböj",
        "Bench Press" => "Bänkpress",
        "Deadlift" => "Marklyft",
        _ => "Nåt annat"
    }.to_string()
}

pub fn convert(input:&str) -> Result<String, ()>
{
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(input.as_bytes());

    let mut last_date = String::new();
    let mut k = Vec::new();

    
    for result in rdr.records() {
        if let Ok(r) = result {
            if r.len() > 4 {                
                let date = (&r[0].to_string()[..10]).to_string();
                if last_date != date {
                    last_date = date;
                    println!("Date is {}", &r[0]);
                    k.clear();
                }
                k.push(r);
            }
        }
    }

    let mut out = Vec::new();
    let mut ex = String::new();
    for entry in k {
        if entry[2] != ex {
            ex = entry[2].to_string();
            if out.len() > 0 {
                out.push(String::new());
            }            
            out.push(format!("[B]{}[/B]", name(&entry[2])));
        }
        let w = entry[4].to_string().replace(".0", "");
        out.push(format!("{}x{} kg", entry[3].to_string(), w));
    }

    Ok(out.join("\n"))   
}