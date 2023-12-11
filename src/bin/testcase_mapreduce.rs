use std::thread;


// This is the `main` thread
fn main() {
    let parallelism_level = num_cpus::get();
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    // "Map" phase
    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = split_into_equal_parts(data.replace('\n', ""), parallelism_level);
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segment" with a
    // "destructuring assignment"
    for (i, data_segment) in chunked_data.into_iter().enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();

            println!("processed segment {}, result={}", i, result);

            result
        }));
    }


    // "Reduce" phase
    // combine each thread's intermediate results into a single final sum.
    // we use the "turbofish" ::<> to provide sum() with a type hint.
    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
}

fn split_into_equal_parts(input: String, number_partitions: usize) -> Vec<String> {
    let length = input.len();
    let partition_size = length / number_partitions;
    let remaining_characters = length % number_partitions;

    (0..number_partitions)
        .scan((0, remaining_characters), |(start, remaining), _| {
            let window_size = partition_size + if *remaining > 0 { 1 } else { 0 };
            *remaining = remaining.saturating_sub(1);
            let result = input.chars().skip(*start).take(window_size).collect();
            *start += window_size;
            Some(result)
        })
        .collect()
}

