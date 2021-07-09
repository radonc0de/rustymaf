/*
0   1       2       3   4       5
32, -4.69,  1.56,   32, 1.22,   8       
t   afc     afl     iac mafv    clol       
*/
use std::fs;

fn build_maftable() {
    let input = fs::read_to_string("maftable")
        .expect("Failed to read input.");
    let maftable = input.lines().collect::<Vec<&str>>();
    let mut cheatsheet = Vec::new();
    let mut index = 0;
    for i in maftable {
        cheatsheet.push(Vec::new());
        cheatsheet[index].push(i.parse::<f32>().unwrap());
        if index == 0 {
            cheatsheet[index].push(0);
        } else {
            cheatsheet[index].push(cheatsheet[index-1][2]);
        }                        
        if index == (maftable.len() - 1 as usize) {
            cheatsheet[index].push(1000);
        }else{
            cheatsheet[index].push(cheatsheet[index][0] +  ((cheatsheet[index + 1][0] - cheatsheet[index]) / 2));
        } 
        
        println!("{:?}", cheatsheet[index]):
        index += 1;
        }
}

fn main() {
    
    println!("Starting rustyMAF...");
    let input = fs::read_to_string("input")
        .expect("Failed to read input.");
    let mut csv = input.lines().collect::<Vec<&str>>();
    csv.remove(0);
    
    println!("CSV imported as string.");

    let mut data = Vec::new();
    let mut index = 0;
    for i in csv {
        data.push(Vec::new());
        let temp = i.split(",").collect::<Vec<&str>>();
        for j in temp {
            let num = j.parse::<f32>().unwrap();
            data[index].push(num);
        }
        index += 1;
    }

    println!("Reformatted data.");

    data = smoothendata(data);

    build_maftable();
}

fn smoothendata(mut data: Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    let threshold = 0.2;

    let mut length = data.len();
    println!("Interpreting {} lines of data...", length);
    
    let j = (length - 1) as u32;

    for k in (1..j).rev() {
        let i = k as usize;
        let mafv1 = data[i][4];
        let mafv0 = data[i-1][4];
        let t1 = data[i][0];
        let t0 = data[i-1][0];
        //println!("({} - {})/({} - {})", mafv1, mafv0, t1, t0);
        let delta = (((mafv1 - mafv0) / (t1 - t0))*1000.0).abs();
        if delta > threshold || data[i][5] >= 10.0 {
            //println!("dmafdt: {}", delta);
            data.remove(i);

        }
    }
    
    length = data.len();
    println!("Smoothened to {} lines of data.", length);

    for i in data {
        i.push(i[1]+i[2]);
    }

    data
}


/*
0   1       2       3   4       5       6
32, -4.69,  1.56,   32, 1.22,   8       
t   afc     afl     iac mafv    clol    cor%  
*/
