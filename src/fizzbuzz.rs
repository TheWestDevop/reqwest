// pub fn while_loop() {
//     let mut count = 1;
//     let mut output = Vec::new();

//     while count <= 15 {
//         if count % 15 == 0 {
//             output.push(String::from("FizzBuzz"));
//         } else if count % 3 == 0 {
//             output.push(String::from("Fizz"))
//         } else if count % 5 == 0 {
//             output.push(String::from("Buzz"));
//         } else {
//             output.push(count.to_string());
//         }
//         count += 1;
//     }
//     print!("while loop : {:?}", output);
// }
// pub fn for_loop() {
//     let mut output = Vec::new();
//     for count in 1..=15 {
//         if count % 15 == 0 {
//             output.push(String::from("FizzBuzz"));
//         } else if count % 3 == 0 {
//             output.push(String::from("Fizz"))
//         } else if count % 5 == 0 {
//             output.push(String::from("Buzz"));
//         } else {
//             output.push(count.to_string());
//         }
//     }
//     print!("forloop : {:?}", output);
// }
// pub fn infinite_loop() {
//     let mut output = Vec::new();
//     let mut count = 1;
//     loop {
//         if count % 15 == 0 {
//             output.push(String::from("FizzBuzz"));
//         } else if count % 3 == 0 {
//             output.push(String::from("Fizz"))
//         } else if count % 5 == 0 {
//             output.push(String::from("Buzz"));
//         } else {
//             output.push(count.to_string());
//         }
//         if count == 15 {
//             break;
//         }
//         count += 1;
//     }
//     print!("infinit loop : {:?}", output);
// }
