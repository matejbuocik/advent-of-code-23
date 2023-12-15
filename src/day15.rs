struct HashMap {
    fields: Vec<Vec<(String, u8)>>,
}

impl HashMap {
    fn new() -> Self {
        let fields = (0..256).map(|_| Vec::new()).collect();
        HashMap { fields }
    }

    fn insert(&mut self, str: &str) {
        let label = String::from_utf8(
            str.bytes()
                .take_while(|&b| b != b'=' && b != b'-')
                .collect(),
        )
        .unwrap();
        let bin = &mut self.fields[hash(&label)];

        if str.contains('-') {
            for i in 0..bin.len() {
                if bin[i].0 == label {
                    bin.remove(i);
                    break;
                }
            }
        } else {
            let number = str.bytes().last().unwrap() - b'0';
            let mut found = false;

            for val in bin.iter_mut() {
                if val.0 == label {
                    found = true;
                    val.1 = number;
                }
            }

            if !found {
                bin.push((label, number));
            }
        }
    }

    fn power(&self) -> usize {
        let mut power = 0;

        for (b, bin) in self.fields.iter().enumerate() {
            for (i, (_, num)) in bin.iter().enumerate() {
                power += (b + 1) * (i + 1) * (*num as usize);
            }
        }

        power
    }
}

pub fn p1() {
    let input = crate::read_file(15);
    let result: usize = input.split(',').map(hash).sum();
    println!("{}", result);
}

pub fn p2() {
    let input = crate::read_file(15);
    let mut hashmap = HashMap::new();
    input.split(',').for_each(|s| hashmap.insert(s));
    println!("{}", hashmap.power());
}

fn hash(str: &str) -> usize {
    str.bytes()
        .fold(0, |acc, b| ((acc + b as usize) * 17) & 0xff)
}
