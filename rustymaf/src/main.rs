/*
0   1       2       3   4       5
32, -4.69,  1.56,   32, 1.22,   8       
t   afc     afl     iac mafv    clol       
*/
use std::fs;

fn build_maftable() -> Vec<Vec<f32>> {
    let input = fs::read_to_string("maftable")
        .expect("Failed to read input.");
    let maftable = input.lines().collect::<Vec<&str>>();
    let mut cheatsheet = Vec::new();
    let mut index = 0;
    for i in &maftable {
        cheatsheet.push(Vec::new());
        cheatsheet[index].push(i.parse::<f32>().unwrap());
        if index == 0 {
            cheatsheet[index].push(0.0);
        } else{ 
            let prev = cheatsheet[index-1][2]; 
            cheatsheet[index].push(prev);
        }                        
        if index == (maftable.len() - 1 as usize) {
            cheatsheet[index].push(1000.0);
        }else{
            let next = maftable[index].parse::<f32>().unwrap()  +  ((maftable[index+1].parse::<f32>().unwrap() - maftable[index].parse::<f32>().unwrap())/2.0);
            cheatsheet[index].push(next);
        } 
        
        println!("{:?}", cheatsheet[index]);
        index += 1;
        }

    cheatsheet
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

    scale_maf(build_maftable(), smoothendata(data));
}

fn smoothendata(mut data: Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    let threshold = 0.2;

    let mut length = data.len();
    println!("Interpreting {} lines of data...", length);
    
    let j = (length - 1) as u32;

    for k in (1..j).rev() {
        println!("--");
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
        let corr = data[i][1] + data[i][2];
        data[i].push(corr);
        println!("{:?}", data[i]);
    }
    
    length = data.len();
    println!("Smoothened to {} lines of data.", length);

    data
}

fn scale_maf(maftable: Vec<Vec<f32>>, data : Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut final_maftable = Vec::new();
    for mut i in maftable {
        let mut sum = 0.0;
        let mut nums = 0.0;
        let lower = i[1];
        let upper = i[2];
        for j in &data {
            if j[6] >= lower && j[6] <= upper {
                sum += j[6];
                nums += 1.0;
            }
        }
        let average = sum / nums;
        i.push(average);
        
        final_maftable.push(i);
    }
    final_maftable}


/*
0   1       2       3   4       5       6
32, -4.69,  1.56,   32, 1.22,   8       
t   afc     afl     iac mafv    clol    cor%  
*/
