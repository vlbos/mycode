use glob::glob;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Data(Vec<(String, String)>);
use serde_json::to_writer_pretty;
use std::fs::{self, File};
use std::io::{self, BufRead};
const BASE_PATH: &str = "../../../../Downloads/mycodetest/tedukurigacode/配套光盘/";
const DATA_BAE_PATH: &str = "../../../../mygit/vlbos/mycode2023/tedukuri-rust/src/solutions/";
fn main() {
    let mut paths = [
        BASE_PATH,
        "例题/0x00 基本算法/0x01 位运算/",
        "a^b",
        "/*.in",
        "/*.out",
        DATA_BAE_PATH,
        "",
        "_89",
    ];
    //input_output_solving(&paths);
    (paths[2], paths[7]) = ("64位整数乘法", "_90");
    //input_output_solving(&paths);
    (paths[2], paths[7]) = ("最短Hamilton路径", "_91");
    //input_output_solving(&paths);
    (paths[1], paths[2], paths[7]) = (
        "例题/0x00 基本算法/0x02 递推与递归/",
        "递归实现指数型枚举",
        "_92",
    );
    //input_output_solving(&paths);
    (paths[2], paths[7]) = ("组合型枚举", "_93");
    //input_output_solving(&paths);
    (paths[2], paths[7]) = ("递归实现排列型枚举", "_94");
    //input_output_solving(&paths);
    (paths[2], paths[3], paths[4], paths[7]) =
        ("费解的开关", "/input/*.txt", "/output/*.txt", "_95");
    //input_output_solving(&paths);
    (paths[2], paths[3], paths[4], paths[7]) =
        ("Strange Towers of Hanoi", "/*.in", "/*.out", "_96");
    //input_output_solving(&paths);
    (paths[2], paths[7]) = ("Sumdiv", "_97");
    //input_output_solving(&paths);
    (paths[2], paths[3], paths[4], paths[7]) =
        ("Fractal Streets", "/data/*.in", "/data/*.out", "_98");
    //input_output_solving(&paths);

    (paths[1], paths[2], paths[3], paths[4], paths[7]) = (
        "例题/0x00 基本算法/0x03 前缀和与差分/",
        "激光炸弹",
        "/*.in",
        "/*.out",
        "_99",
    );
    //input_output_solving(&paths);

    (paths[2], paths[3], paths[4], paths[6], paths[7]) = (
        "IncDec Sequence",
        "/Input/*.in",
        "/Output/*.out",
        "_100/",
        "_100",
    );
    // input_output_solving(&paths);
    (paths[2], paths[3], paths[4], paths[7]) = ("Tallest Cow", "/*.in", "/*.out", "_101");
    // input_output_solving(&paths);

    (paths[1], paths[2], paths[7]) = ("例题/0x00 基本算法/0x04 二分/", "Best Cow Fence", "_102");
    // input_output_solving(&paths);

    (paths[1], paths[2], paths[7]) = ("例题/0x00 基本算法/0x05 排序/", "Cinema", "_103");
    // input_output_solving(&paths);

    (paths[2], paths[7]) = ("货仓选址", "_104");
    // input_output_solving(&paths);

    (paths[2], paths[3], paths[4], paths[7]) =
        ("七夕祭", "/tanabata/*.in", "/tanabata/*.out", "_105");
    // input_output_solving(&paths);

    (paths[2], paths[7]) = ("Running Median", "_106");
    // input_output_solving(&paths);

    let sub_paths = [
        "a^b",
        "64位整数乘法",
        "最短Hamilton路径",
        "递归实现指数型枚举",
        "组合型枚举",
        "递归实现排列型枚举",
        "费解的开关",
        "Strange Towers of Hanoi",
        "Sumdiv",
        "Fractal Streets",
        "激光炸弹",
        "IncDec Sequence",
        "Tallest Cow",
        "Best Cow Fence",
        "Cinema",
        "货仓选址",
        "七夕祭",
        "Running Median",
        "Ultra-QuickSort",
        "奇数码问题",
        "Genius ACM",
        "Sunscreen",
        "Stall Reservation",
        "Radar Installation",
        "Innovative Business",
        "国王游戏",
        "Color a Tree",
        "The Pilots Brothers' refrigerator",
        "占卜DIY",
        "Fractal",
        "Raid",
        "防线",
        "Corral the Cows",
        "糖果传递",
        "Soldiers",
        "Number Base Conversion",
        "Cow Acrobats",
        "To the Max",
        "Task",
        "Editor",
        "火车进出栈问题/火车进栈（递归枚举）",
        "火车进出栈问题/火车进出栈问题（计数）",
        "Largest Rectangle in a Histogram",
        "Team Queue",
        "蚯蚓",
        "双端队列 (选做)",
        "最大子序和",
        "邻值查找",
        "Snowflake Snow Snowflakes",
        "兔子与兔子",
        "Palindrome",
        "后缀数组",
        "Period",
        "前缀统计",
        "The XOR Largest Pair",
        "The xor-longest Path",
        "Supermarket",
        "Sequence",
        "数据备份",
        "合并果子",
        "荷马史诗",
        "括号画家",
        "表达式计算4",
        "City Game",
        "双栈排序",
        "Sliding Window",
        "内存分配",
        "Matrix",
        "Subway tree systems",
        "Necklace",
        "Milking Grid",
        "匹配统计",
        "Phone List",
        "Black Box",
        "生日礼物",
        "可达性统计",
        "小猫爬山",
        "Sudoku",
        "Sticks",
        "生日蛋糕",
        "Sudoku",
        "Addition Chains",
        "送礼物",
        "Bloxorz I",
        "矩阵距离",
        "Pushing Boxes",
        "电路维修",
        "Full Tank",
        "Nightmare II",
        "第K短路",
        "八数码",
        "Booksort",
        "The Rotation Game",
        "Square Destroyer",
        "靶形数独",
        "虫食算",
        "Mayan游戏",
        "The Buses",
        "Missile Defence System",
        "武士风度的牛",
        "乳草的入侵",
        "字串变换",
        "Weather Forecast",
        "Bloxorz II",
        "Power Hungry Cows",
        "Flood-it!",
        "骑士精神",
        "Prime Distance",
        "阶乘分解",
        "反素数",
        "余数之和",
    ];
    const fno: i32 = 130;
    for i in fno..=fno {
        let mut paths = [
            BASE_PATH,
            sub_path_solving(&i),
            sub_paths[i as usize - 89],
            "/*.in",
            "/*.out",
            DATA_BAE_PATH,
            if i / 100 == 0 {
                ""
            } else {
                &format!("_{}/", i / 100 * 100)
            },
            &format!("_{i}"),
        ];
        (paths[3], paths[4]) = file_extension_solving(i);

        input_output_solving(&paths);
    }
}

fn file_extension_solving(file_number: i32) -> (&'static str, &'static str) {
    match file_number {
        95 | 117|129 => ("/Input/*.txt", "/Output/*.txt"),
        98 | 112 => ("/data/*.in", "/data/*.out"),
        100 |128|130=> ("/Input/*.in", "/Output/*.out"),
        105 => ("/tanabata/*.in", "/tanabata/*.out"),
        114 => ("/Input/*.in", "/Output/*.ans"),
        115 => ("/Color/*.in", "/Color/*.out"),
        120 => ("/defender/*.in", "/defender/*.out"),
        123 => ("/SOLDIERS.IN*", "/SOLDIERS.OU*"),
        _ => ("/*.in", "/*.out"),
    }
}
fn sub_path_solving(file_number: &i32) -> &str {
    let fno = *file_number;
    match fno {
        _ if fno == fno.clamp(89, 91) => "例题/0x00 基本算法/0x01 位运算/",
        _ if fno == fno.clamp(92, 98) => "例题/0x00 基本算法/0x02 递推与递归/",
        _ if fno == fno.clamp(99, 101) => "例题/0x00 基本算法/0x03 前缀和与差分/",
        102 | 113 => "例题/0x00 基本算法/0x04 二分/",
        _ if fno == fno.clamp(103, 108) => "例题/0x00 基本算法/0x05 排序/",
        109 => "例题/0x00 基本算法/0x06 倍增/",
        _ if fno == fno.clamp(110, 115) => "例题/0x00 基本算法/0x07 贪心/",
        _ if fno == fno.clamp(116, 127) => "习题/0x08 基本算法 总结与练习/",
        _ if fno == fno.clamp(128, 131) => "例题/0x10 基本数据结构/0x11 栈/",
        _ if fno == fno.clamp(132, 135) => "例题/0x10 基本数据结构/0x12 队列/",
        136 => "例题/0x10 基本数据结构/0x13 链表与邻接表/",
        _ if fno == fno.clamp(137, 140) => "例题/0x10 基本数据结构/0x14 Hash/",
        141 => "例题/0x10 基本数据结构/0x15 字符串/",
        _ if fno == fno.clamp(142, 144) => "例题/0x10 基本数据结构/0x16 Trie/",
        _ if fno == fno.clamp(145, 149) => "例题/0x10 基本数据结构/0x17 二叉堆/",
        _ if fno == fno.clamp(150, 163) => "习题/0x18 基本数据结构 总结与练习/",
        164 => "例题/0x20 搜索/0x21 树与图的遍历/",
        _ if fno == fno.clamp(165, 166) => "例题/0x20 搜索/0x22 深度优先搜索/",
        _ if fno == fno.clamp(167, 169) => "例题/0x20 搜索/0x23 剪枝/",
        _ if fno == fno.clamp(170, 171) => "例题/0x20 搜索/0x24 迭代加深/",
        _ if fno == fno.clamp(172, 174) => "例题/0x20 搜索/0x25 广度优先搜索/",
        _ if fno == fno.clamp(175, 177) => "例题/0x20 搜索/0x26 广搜变形/",
        _ if fno == fno.clamp(178, 179) => "例题/0x20 搜索/0x27 A_star/",
        _ if fno == fno.clamp(180, 182) => "例题/0x20 搜索/0x28 ID_A_star/",
        _ if fno == fno.clamp(183, 195) => "习题/0x29 搜索 总结与练习/",
        _ if fno == fno.clamp(196, 197) => "例题/0x30 数学知识/0x31 质数/",
        _ if fno == fno.clamp(198, 200) => "例题/0x30 数学知识/0x32 约数/",
        _ => panic!("{}", fno),
    }
}

fn input_output_solving(paths: &[&str]) {
    let fno = paths[7][1..].parse::<i32>().unwrap();
    let input = match fno {
        89 | 92 | 93 | 97 | 113|129|130 => input_solving(&paths[..4]),
        90 | 103 | 104 | 107 | 116 | 117 | 118 | 127 | 128 => solving(&paths[..4], ","),
        95 => input_solving95(&paths[..4]),
        96 => input_solving96(&paths[..4]),
        91 | 98 | 122 | 123 | 124 => input_solving98(&paths[..4]),
        99 | 102 | 112 => input_solving99(&paths[..4]),
        101 | 105 => input_solving101(&paths[..4]),
        106 => input_solving106(&paths[..4]),
        108 => input_solving108(&paths[..4]),
        109 => input_solving109(&paths[..4]),
        110 => input_solving110(&paths[..4]),
        111 => solving111(&paths[..4], true),
        114 => input_solving114(&paths[..4]),
        115 => input_solving115(&paths[..4]),
        119 => input_solving119(&paths[..4]),
        120 => input_solving120(&paths[..4]),
        121 => solving121(&paths[..4], true),
        125 => solving125(&paths[..4], true),
        126 => input_solving126(&paths[..4]),
        _ => panic!("unexpected input={}", paths[7]),
    };
    let output = match fno {
        89 | 97 | 90 | 91 | 99 | 102 | 103 | 104 | 105 | 110 | 112 | 113 | 114 | 115 | 117
        | 122 | 123 | 126 | 127|130=> output_solving(&[paths[0], paths[1], paths[2], paths[4]]),
        92 => output_solving92(&[paths[0], paths[1], paths[2], paths[4]], &input),
        93 | 95 | 96 | 98 | 101 | 107 | 108 | 109 | 118 | 119 | 120 | 128|129 => {
            solving(&[paths[0], paths[1], paths[2], paths[4]], ",")
        }
        106 => output_solving106(&[paths[0], paths[1], paths[2], paths[4]]),
        116 => input_solving98(&[paths[0], paths[1], paths[2], paths[4]]),
        111 => solving111(&[paths[0], paths[1], paths[2], paths[4]], false),
        121 => solving121(&[paths[0], paths[1], paths[2], paths[4]], false),
        124 => output_solving124(&[paths[0], paths[1], paths[2], paths[4]]),
        125 => solving125(&[paths[0], paths[1], paths[2], paths[4]], false),
        _ => panic!("unexpected output={}", paths[7]),
    };
    input_output_writing(input, output, &paths[5..]);
}

fn output_solving92(paths: &[&str], input: &Vec<String>) -> Vec<String> {
    let mut output = vec![];
    for i in input {
        let n = i.parse::<i32>().unwrap();
        let (mut chosen, mut h) = (vec![], vec![]);
        calc(1, n, &mut chosen, &mut h);
        if !h.is_empty() {
            output.push(h.join(","));
        }
    }
    output
}

fn calc(x: i32, n: i32, chosen: &mut Vec<i32>, h: &mut Vec<String>) {
    if x == n + 1 {
        if !chosen.is_empty() {
            let v = chosen
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            h.push(v);
        }
        return;
    }
    calc(x + 1, n, chosen, h);
    chosen.push(x);
    calc(x + 1, n, chosen, h);
    chosen.pop();
}
fn input_output_writing(input: Vec<String>, output: Vec<String>, paths: &[&str]) {
    let data: Vec<_> = input.into_iter().zip(output).collect();
    let file_path = paths.concat() + "/data.json";
    println!("{file_path}");
    let writer = File::create(file_path).expect("{file_path}");
    to_writer_pretty(writer, &Data(data)).expect("{file_path}");
}
fn input_solving(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&paths.concat()).unwrap() {
        let message: String = fs::read_to_string(entry.as_ref().unwrap()).unwrap();
        println!("{},{}", entry.unwrap().display(), message);
        input.push(message.trim_end().to_owned());
    }
    assert!(!input.is_empty(), "{input:?},{paths:?}");
    input
}

fn solving(paths: &[&str], seperator: &str) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&paths.concat()).unwrap() {
        let message: String = fs::read_to_string(entry.as_ref().unwrap())
            .unwrap()
            .split("\n")
            .filter_map(|s| (!s.trim().is_empty()).then(|| s.trim()))
            .collect::<Vec<_>>()
            .join(seperator);
        println!("{},{}", entry.unwrap().display(), message);
        input.push(message.trim_end().to_owned());
    }
    assert!(!input.is_empty(), "{input:?},{paths:?}");
    input
}

fn input_solving95(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let n = lines.next().unwrap();
        let mut line = String::new();
        line.push_str(n.as_ref().unwrap());
        line.push(';');
        for _ in 0..n.unwrap().parse::<i32>().unwrap() {
            for _ in 0..5 {
                line.push_str(lines.next().unwrap().as_ref().unwrap());
                line.push(' ');
            }
            line.pop();
            line.push(',');
            let _ = lines.next();
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving98(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let n = lines.next().unwrap();
        let mut line = String::new();
        line.push_str(n.as_ref().unwrap());
        line.push(';');
        for _ in 0..n.unwrap().parse::<i32>().unwrap() {
            line.push_str(lines.next().unwrap().as_ref().unwrap());
            line.push(',');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving126(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let n = lines.next().unwrap();
        let mut line = String::new();
        line.push_str(n.as_ref().unwrap());
        line.push(';');
        for v in lines {
            line.push_str(v.as_ref().unwrap());
            line.push(' ');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving114(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let mut line = String::new();
        let n = lines.next().unwrap();
        line.push_str(n.as_ref().unwrap());
        line.push(';');
        let k = lines.next().unwrap();
        line.push_str(k.as_ref().unwrap());
        line.push(';');
        for _ in 0..n.unwrap().parse::<i32>().unwrap() {
            line.push_str(lines.next().unwrap().as_ref().unwrap());
            line.push(',');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn solving111(paths: &[&str], io_type: bool) -> Vec<String> {
    let (mut input, mut output) = (vec![], vec![]);
    let mut line = String::new();
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        loop {
            let Some(v) = lines.next() else { break };
            if v.as_ref().unwrap().starts_with("Input for test") {
                if !line.is_empty() {
                    line.pop();
                    output.push(line);
                    line = String::new();
                }
                let n = lines.next().unwrap();
                line.push_str(n.as_ref().unwrap());
                line.push(';');
                for _ in 0..n.unwrap().parse::<i32>().unwrap() {
                    line.push_str(lines.next().unwrap().as_ref().unwrap());
                    line.push(',');
                }
                line.pop();
                input.push(line);
                line = String::new();
            } else if !v.as_ref().unwrap().starts_with("Output for test") {
                line.push_str(v.as_ref().unwrap());
                line.push(',');
            }
        }
        println!("{},", entry.unwrap().display(),);
    }
    if !line.is_empty() {
        line.pop();
        output.push(line);
    }
    if io_type { input } else { output }
}

fn solving121(paths: &[&str], io_type: bool) -> Vec<String> {
    let (mut input, mut output) = (vec![], vec![]);
    let mut line = String::new();
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        loop {
            let Some(v) = lines.next() else { break };
            if v.as_ref().unwrap().starts_with("Input for test") {
                line = String::new();
                let n = lines.next().unwrap();
                line.push_str(n.as_ref().unwrap());
                line.push(';');
                for _ in 0..n
                    .unwrap()
                    .split_ascii_whitespace()
                    .next_back()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
                {
                    line.push_str(lines.next().unwrap().as_ref().unwrap());
                    line.push(',');
                }
                line.pop();
                input.push(line);
            } else if v.as_ref().unwrap().starts_with("Output for test") {
                line = String::new();
                line.push_str(lines.next().unwrap().as_ref().unwrap());
                output.push(line);
            }
        }
        println!("{},", entry.unwrap().display(),);
    }
    if io_type { input } else { output }
}

fn solving125(paths: &[&str], io_type: bool) -> Vec<String> {
    let (mut input, mut output) = (vec![], vec![]);
    let mut line = String::new();
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        loop {
            let Some(v) = lines.next() else { break };
            if v.as_ref().unwrap().starts_with("Input for test") {
                line = String::new();
                let n = lines.next().unwrap();
                line.push_str(n.as_ref().unwrap());
                line.push(';');
                for _ in 0..n.unwrap().parse::<i32>().unwrap() {
                    line.push_str(lines.next().unwrap().as_ref().unwrap());
                    line.push(',');
                }
                line.pop();
                input.push(line);
            } else if v.as_ref().unwrap().starts_with("Output for test") {
                output.push(lines.next().unwrap().unwrap());
            }
        }
        println!("{},", entry.unwrap().display(),);
    }
    if io_type { input } else { output }
}

fn input_solving108(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let mut line = String::new();
        loop {
            let Some(n) = lines.next() else { break };
            line.push_str(n.as_ref().unwrap());
            line.push(';');
            for _ in 0..n.unwrap().parse::<i32>().unwrap() * 2 {
                line.push_str(lines.next().unwrap().as_ref().unwrap());
                line.push(',');
            }
            line.pop();
            line.push(';');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving109(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let k = lines.next().unwrap();
        let mut line = String::new();
        line.push_str(k.as_ref().unwrap());
        line.push(';');
        for _ in 0..k.unwrap().parse::<i32>().unwrap() * 2 {
            line.push_str(lines.next().unwrap().as_ref().unwrap());
            line.push(',');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving99(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let v = lines.next().unwrap().unwrap();
        let mut line = String::new();
        line.push_str(&v);
        line.push(';');
        for _ in 0..v
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap()
        {
            line.push_str(lines.next().unwrap().as_ref().unwrap());
            line.push(',');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving115(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let v = lines.next().unwrap().unwrap();
        let mut line = String::new();
        line.push_str(&v);
        line.push(';');
        line.push_str(lines.next().unwrap().as_ref().unwrap());
        line.push(';');
        for _ in 0..v
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap()
            - 1
        {
            line.push_str(lines.next().unwrap().as_ref().unwrap());
            line.push(',');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving101(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let v = lines.next().unwrap().unwrap();
        let mut line = String::new();
        line.push_str(&v);
        line.push(';');
        for _ in 0..v
            .split_ascii_whitespace()
            .next_back()
            .unwrap()
            .parse::<i32>()
            .unwrap()
        {
            line.push_str(lines.next().unwrap().as_ref().unwrap());
            line.push(',');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving110(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let v = lines.next().unwrap().unwrap();
        let mut line = String::new();
        line.push_str(&v);
        line.push(';');
        for _ in 0..v
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap()
        {
            line.push_str(lines.next().unwrap().as_ref().unwrap());
            line.push(',');
        }
        line.pop();
        line.push(';');
        for _ in 0..v
            .split_ascii_whitespace()
            .next_back()
            .unwrap()
            .parse::<i32>()
            .unwrap()
        {
            line.push_str(lines.next().unwrap().as_ref().unwrap());
            line.push(',');
        }
        line.pop();

        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving106(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let p = lines.next().unwrap().unwrap();
        let mut line = String::new();
        line.push_str(&p);
        line.push(';');
        for _ in 0..p.parse::<i32>().unwrap() {
            let m = lines.next().unwrap().unwrap();
            line.push_str(&m);
            line.push(',');
            for _ in 0..(m
                .split_ascii_whitespace()
                .next_back()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                - 1)
                / 10
                + 1
            {
                line.push_str(lines.next().unwrap().as_ref().unwrap().trim_end());
                line.push(' ');
            }
            line.pop();
            line.push(';');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn output_solving106(paths: &[&str]) -> Vec<String> {
    let mut output = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let mut line = String::new();
        let m = lines.next().unwrap().unwrap();
        line.push_str(&m);
        line.push(',');
        for _ in 0..(m
            .split_ascii_whitespace()
            .next_back()
            .unwrap()
            .parse::<i32>()
            .unwrap()
            - 1)
            / 10
            + 1
        {
            line.push_str(lines.next().unwrap().as_ref().unwrap().trim_end());
            line.push(' ');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        output.push(line);
    }
    output
}

fn output_solving124(paths: &[&str]) -> Vec<String> {
    let mut output = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let mut line = String::new();
        loop {
            let Some(m) = lines.next() else { break };
            line.push_str(m.as_ref().unwrap());
            line.push(',');
            line.push_str(lines.next().unwrap().as_ref().unwrap().trim_end());
            let _ = lines.next();
            line.push(';');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        output.push(line);
    }
    output
}

fn input_solving119(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let t = lines.next().unwrap();
        let mut line = String::new();
        line.push_str(t.as_ref().unwrap());
        line.push(';');
        for _ in 0..t.as_ref().unwrap().parse::<i32>().unwrap() {
            let n = lines.next().unwrap();
            line.push_str(n.as_ref().unwrap());
            line.push(';');
            for _ in 0..n.as_ref().unwrap().parse::<i32>().unwrap() * 2 {
                line.push_str(lines.next().unwrap().as_ref().unwrap().trim_end());
                line.push(',');
            }
            line.pop();
            line.push(';');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving120(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let t = lines.next().unwrap();
        let mut line = String::new();
        line.push_str(t.as_ref().unwrap());
        line.push(';');
        for _ in 0..t.as_ref().unwrap().parse::<i32>().unwrap() {
            let n = lines.next().unwrap();
            line.push_str(n.as_ref().unwrap());
            line.push(';');
            for _ in 0..n.as_ref().unwrap().parse::<i32>().unwrap() {
                line.push_str(lines.next().unwrap().as_ref().unwrap().trim_end());
                line.push(',');
            }
            line.pop();
            line.push(';');
        }
        line.pop();
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
    input
}

fn input_solving96(_paths: &[&str]) -> Vec<String> {
    vec![String::new()]
}
fn output_solving(paths: &[&str]) -> Vec<String> {
    let mut output = vec![];
    for entry in glob(&paths.concat()).unwrap() {
        let message: String = fs::read_to_string(entry.as_ref().unwrap()).unwrap();
        println!("{},{}", entry.unwrap().display(), message);
        output.push(message.trim_end().to_owned());
    }
    assert!(!output.is_empty(), "output==={output:?},{paths:?}");
    output
}
