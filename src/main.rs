use regex::Regex;
fn main() {
    println!("Hello, world!");

    // Regex  (\d+) \s? \+ \s? (\d+)
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_rest = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    //Traer Datos del Usuario

    println!("Por favor introduce tu expresion: ");

    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // multiplicaicon
    loop {
        // Aplicar Operaciones
        let caps = re_mul.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value * right_value;
        expression = expression.replace(cap_expression, &addition.to_string())
    }
    // division

    loop {
        // Aplicar Operaciones
        let caps = re_div.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value / right_value;
        expression = expression.replace(cap_expression, &addition.to_string())
    }

    // suma
    loop {
        // Aplicar Operaciones
        let caps = re_add.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value + right_value;
        expression = expression.replace(cap_expression, &addition.to_string())
    }
    // resta
    loop {
        // Aplicar Operaciones
        let caps = re_rest.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value - right_value;
        expression = expression.replace(cap_expression, &addition.to_string())
    }
    // Mostrar Resultado
    println!("Resultado : {}", expression);
}
