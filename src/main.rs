use csv::Reader;
use rand::distributions::{Distribution, WeightedIndex};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn get_current_numbers(y: &str) -> String {
    let last_line = y.trim().split('\n').last().unwrap();
    let temp = last_line.split("left: ").last().unwrap();
    temp.split(')').next().unwrap().to_string()
}

struct Game24Task {
    _data: Vec<String>,
    value_cache: HashMap<String, f32>,
    steps: i32,
    _stops: Vec<String>,
}

// global request counter
static mut REQUEST_COUNTER: i32 = 0;

use generativelanguage_rs::common::api_client::{APIRequestClient, GenResponse};
use std::env;

const DEV_MODE: bool = false;

async fn gpt(prompt_text: String) -> Vec<String> {
    let mut api_key = String::new();

    let key = "GOOGLE_API_KEY";
    match env::var(key) {
        Ok(val) => {
            api_key = val;
        }
        Err(e) => {
            println!("couldn't interpret {}: {}", key, e);
        }
    }

    let client = APIRequestClient::new(&api_key);
    match client.send_request(&prompt_text).await {
        Ok(response_text) => {
            // parse response
            let response = match serde_json::from_str::<GenResponse>(&response_text) {
                Ok(response) => response,
                Err(err) => {
                    eprintln!("Error occurred response_text: {}", err);
                    GenResponse {
                        candidates: Vec::new(),
                    }
                }
            };

            unsafe {
                REQUEST_COUNTER += 1;
                println!("REQUEST_COUNTER: {}", REQUEST_COUNTER);
            }
            
            // print response
            for candidate in response.candidates.clone() {
                println!("\n{}\n", candidate.output);

                if DEV_MODE {
                    for safety_rating in candidate.safety_ratings {
                        println!(
                            "category: {}, probability: {}",
                            safety_rating.category, safety_rating.probability
                        );
                    }

                    println!("");
                }
            }

            let output = response.candidates[0].output.to_string();
            let output_lines: Vec<String> = output.split("\n").map(|s| s.to_string()).collect();

            return output_lines;
        }
        Err(err) => {
            eprintln!("Error occurred: {}", err);
            return Vec::new();
        }
    }
}

impl Game24Task {
    fn new(file: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new("24").join(file);
        let file = File::open(&path)?;
        let mut reader = Reader::from_reader(file);
        let mut data = Vec::new();

        for result in reader.records() {
            let record = result?;
            data.push(record.get(0).unwrap().to_string());
        }

        Ok(Game24Task {
            _data: data,
            value_cache: HashMap::new(),
            steps: 4,
            _stops: vec!["\n".to_string(); 4],
        })
    }

    fn propose_prompt_wrap(&self, x: &str, y: &str) -> String {
        let current_numbers = get_current_numbers(y);
        if current_numbers == "24" {
            format!("cot_prompt: {}\nSteps:\n{}", x, y) // Replace "cot_prompt: " with your actual prompt
        } else {
            format!(
                "Input: 2 5 9 10
Possible next steps:
2 + 5 = 7 (unused: 9 10) (new: 7) (left: 7 9 10)
2 * 5 = 10 (unused: 9 10) (new: 10) (left: 10 10)
10 / 2 = 5 (unused: 5 9) (new: 5) (left: 5 5 9)
10 - 2 = 8 (unused: 5 9) (new: 8) (left: 8 5 9)
9 - 5 = 4 (unused: 10 10) (new: 4) (left: 4 10 10)
Input: 6 10 12 14
Possible next steps:
12 - 6 = 6 (unused: 10 14) (new: 6) (left: 6 10 14)
10 + 6 = 16 (unused: 12 14) (new: 16) (left: 16 12 14)
14 - 10 = 4 (unused: 6 12) (new: 4) (left: 4 6 12)
14 + 6 = 20 (unused: 10 12) (new: 20) (left: 20 10 12)
Input: {}
Possible next steps:
",
                current_numbers
            ) // Replace "propose_prompt: " with your actual prompt
        }
    }

    fn value_prompt_wrap(&self, x: &str, y: &str) -> String {
        let last_line = y.trim().split('\n').last().unwrap();
        if !last_line.contains("left: ") {
            // last step
            let ans = last_line.replace("answer: ", "");
            format!("Use numbers and basic arithmetic operations (+ - * /) to obtain 24. Given an input and an answer, give a judgement (sure/impossible) if the answer is correct, i.e. it uses each input exactly once and no other numbers, and reach 24.
Input: 4 4 6 8
Answer: (4 + 8) * (6 - 4) = 24
Judge: 
sure
Input: 2 9 10 12
Answer: 2 * 12 * (10 - 9) = 24
Judge: 
sure
Input: 4 9 10 13
Answer: (13 - 9) * (10 - 4) = 24
Judge: 
sure
Input: 4 4 6 8
Answer: (4 + 8) * (6 - 4) + 1 = 25
Judge: 
impossible
Input: 2 9 10 12
Answer: 2 * (12 - 10) = 24
Judge: 
impossible
Input: 4 9 10 13
Answer: (13 - 4) * (10 - 9) = 24
Judge: 
impossible
Input: {}
Answer: {}
Judge:", x, ans)
        } else {
            let current_numbers = get_current_numbers(y);
            format!(
                "Evaluate if given numbers can reach 24 (sure/likely/impossible)
10 14
10 + 14 = 24
sure
11 12
11 + 12 = 23
12 - 11 = 1
11 * 12 = 132
11 / 12 = 0.91
impossible
4 4 10
4 + 4 + 10 = 8 + 10 = 18
4 * 10 - 4 = 40 - 4 = 36
(10 - 4) * 4 = 6 * 4 = 24
sure
4 9 11
9 + 11 + 4 = 20 + 4 = 24
sure
5 7 8
5 + 7 + 8 = 12 + 8 = 20
(8 - 5) * 7 = 3 * 7 = 21
I cannot obtain 24 now, but numbers are within a reasonable range
likely
5 6 6
5 + 6 + 6 = 17
(6 - 5) * 6 = 1 * 6 = 6
I cannot obtain 24 now, but numbers are within a reasonable range
likely
10 10 11
10 + 10 + 11 = 31
(11 - 10) * 10 = 10
10 10 10 are all too big
impossible
1 3 3
1 * 3 * 3 = 9
(1 + 3) * 3 = 12
1 3 3 are all too small
impossible
{}\n",
                current_numbers
            )
        }
    }

    fn value_outputs_unwrap(&self, _x: &str, y: &str, value_outputs: &[String]) -> f32 {
        if y.trim().split('\n').collect::<Vec<_>>().len() == 4
            && !y.to_lowercase().contains("answer")
        {
            return 0.0;
        }

        let value_names = value_outputs
            .iter()
            .map(|output| output.split('\n').last().unwrap())
            .collect::<Vec<_>>();

        let mut value_map = HashMap::new();
        value_map.insert("impossible", 0.001);
        value_map.insert("likely", 1.0);
        value_map.insert("sure", 20.0); // This is just an example. Replace these strings and values with the actual ones you use.

        let mut value = 0.0;
        for (name, &val) in value_map.iter() {
            value += val * (value_names.iter().filter(|&n| n == name).count() as f32);
        }
        value
    }

    async fn get_value(&mut self, x: &str, y: &str, cache_value: bool) -> f32 {
        let value_prompt = self.value_prompt_wrap(x, y);
        if cache_value {
            if let Some(value) = self.value_cache.get(&value_prompt) {
                return *value;
            }
        }

        let value_outputs = gpt(value_prompt.clone()).await;
        let value = self.value_outputs_unwrap(x, y, &value_outputs);

        if cache_value {
            self.value_cache.insert(value_prompt, value);
        }

        value
    }

    async fn get_values(&mut self, x: &str, ys: &[String], cache_value: bool) -> Vec<f32> {
        let mut values = Vec::new();
        let mut local_value_cache = HashMap::new();

        for y in ys {
            if let Some(value) = local_value_cache.get(y) {
                values.push(*value);
            } else {
                let value = self.get_value(x, y, cache_value).await;
                local_value_cache.insert(y.clone(), value);
                values.push(value);
            }
        }

        values
    }

    async fn get_proposals(&self, x: &str, y: &str) -> Vec<String> {
        let propose_prompt = self.propose_prompt_wrap(x, y);
        let proposals = gpt(propose_prompt.clone()).await;
        proposals.iter().map(|p| format!("{}", p)).collect()
    }

    fn select<'a>(
        &self,
        new_ys: &'a [String],
        values: &[f32],
        n_select_sample: usize,
        method: &str,
    ) -> Vec<&'a String> {
        match method {
            "greedy" => {
                let mut indices: Vec<usize> = (0..new_ys.len()).collect();
                indices.sort_unstable_by(|&a, &b| values[b].partial_cmp(&values[a]).unwrap());
                indices
                    .iter()
                    .map(|&i| &new_ys[i])
                    .take(n_select_sample)
                    .collect()
            }
            "sample" => {
                let total: f32 = values.iter().sum();
                let weights: Vec<f32> = values.iter().map(|&v| v / total).collect();
                let dist = WeightedIndex::new(&weights).unwrap();
                let mut rng = rand::thread_rng();
                (0..n_select_sample)
                    .map(|_| &new_ys[dist.sample(&mut rng)])
                    .collect()
            }
            _ => panic!("Unknown selection method: {}", method),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut task = Game24Task::new("24.csv").unwrap();

    let x = "Your task input here";

    let mut ys = vec!["4 9 10 13".to_string()];
    // ( 10 - 4 ) * ( 13 - 9 ) = 6 * 4 = 24

    println!("{:#?}", ys);

    let n_select_sample = 2;
    let method_generate = "sample";
    let method_evaluate = "value";
    let method_select = "greedy";

    for step in 0..task.steps {
        // generation
        let new_ys: Vec<String> = match method_generate {
            "sample" => {
                let mut new_ys = Vec::new();
                for y in ys {
                    let proposals = task.get_proposals(x, &y).await;
                    new_ys.extend(proposals);
                }
                new_ys
            }
            _ => panic!("Unknown generation method: {}", method_generate),
        };

        println!("\n======== step {} ========", step);

        println!("  Proposals:");

        // iterate over new_ys
        for (index, y) in new_ys.iter().enumerate() {
            println!("    {}: {}", index, y);
        }

        // evaluation
        let values = match method_evaluate {
            "value" => task.get_values(x, &new_ys, true).await,
            _ => panic!("Unknown evaluation method: {}", method_evaluate),
        };

        println!("  Values:");

        // iterate over values
        for (index, value) in values.iter().enumerate() {
            println!("    {}: {}", index, value);
        }

        // selection
        ys = task
            .select(&new_ys, &values, n_select_sample, method_select)
            .into_iter()
            .cloned()
            .collect();

        // iterate over ys
        println!("  Selected:");

        for (index, y) in ys.iter().enumerate() {
            println!("    {}: {}", index, y);
        }

        break;
    }

    println!("request counter: {}", unsafe { REQUEST_COUNTER });

    Ok(())
}
