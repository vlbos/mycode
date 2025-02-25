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
        "Hankson的趣味题",
        "Visible Lattice Points",
        "The Luckiest Number",
        "同余方程",
        "Strange Way to Express Integers",
        "Fibonacci",
        "石头游戏",
        "球形空间产生器",
        "开关问题",
        "装备购买",
        "XOR",
        "计算系数",
        "Counting Swaps",
        "古代猪文",
        "Devu and Flowers",
        "Zap",
        "Rainbow的信号",
        "绿豆蛙的归宿",
        "扑克牌",
        "Cutting Game",
        "GCD",
        "Longge's Problem_x",
        "青蛙的约会",
        "Xiao 9(star)大战朱最学",
        "计算器",
        "Matrix Power Series",
        "233 Matrix",
        "Widget Factory",
        "XOR",
        "新NIM游戏",
        "排列计数",
        "Sky Code",
        "守卫者的挑战",
        "换教室",
        "Dropping Test",
        "魔法珠",
        "Georgia and Bob",
        "程序自动分析",
        "银河英雄传说",
        "Parity Game",
        "食物链",
        "楼兰图腾",
        "A Simple Problem with Integers",
        "A Simple Problem with Integers",
        "Lost Cows",
        "Can you answer on these queries III",
        "Interval GCD",
        "Atlantis",
        "Stars in Your Window",
        "蒲公英",
        "磁力块",
        "小Z的袜子",
        "Tree",
        "普通平衡树",
        "天使玩偶",
        "K-th Number",
        "最大异或和",
        "关押罪犯",
        "Rochambeau",
        "True Liars",
        "Buy Tickets",
        "Hotel",
        "Picture",
        "作诗",
        "Race",
        "营业额统计",
        "SuperMemo",
        "Mokia",
        "Meteors",
        "Fotile模拟赛L",
        "可持久化并查集加强版",
        "Mr. Young's Picture Permutations",
        "LCIS",
        "Making the Grade",
        "Mobile Service",
        "传纸条",
        "I-country",
        "Cookies",
        "数字组合",
        "自然数拆分Lunatic版",
        "Jury Compromise",
        "Coins",
        "石子合并",
        "Polygon",
        "金字塔",
        "没有上司的舞会",
        "选课",
        "Accumulation Degree",
        "Naptime",
        "环路运输",
        "Broken Robot",
        "Mondriaan's Dream",
        "炮兵阵地",
        "开车旅行",
        "Count The Repetitions",
        "Cleaning Shifts",
        "Cleaning Shifts",
        "The Battle of Chibi",
        "Fence",
        "Cut the Sequence",
        "任务安排1",
        "任务安排2",
        "任务安排3",
        "Cats Transport",
        "诗人小G",
        "An old Stone Game",
        "Gerald and Giant Chess",
        "Connected Graph",
        "How many of them",
        "A Decorative Fence",
        "启示录",
        "月之谜",
        "乌龟棋",
        "花店橱窗",
        "BUY LOW, BUY LOWER",
        "Trip",
        "SUBTRACT",
        "陨石的秘密",
        "划分大理石",
        "Folding",
        "能量项链",
        "棋盘分割",
        "Blocks",
        "Strategic game",
        "Bribing FIPA",
        "Computer",
        "XOR和路径",
        "Corn Fields",
        "Bugs Integrated, Inc",
        "Fence Obstacle Course",
        "Estimation",
        "干草堆",
        "股票交易",
        "Largest Submatrix",
        "K-Anonymous Sequence",
        "特别行动队",
        "Post Office",
        "扑克牌",
        "The Counting Problem",
        "Round Numbers",
        "Telephone Lines",
        "最优贸易",
        "道路与航线",
        "Sorting It All Out",
        "Sightseeing Trip",
        "Cow Relays",
        "走廊泼水节",
        "Picnic Planning",
        "最优比率生成树",
        "黑暗城堡",
        "巡逻",
        "树网的核",
        "闇の連鎖",
        "雨天的尾巴",
        "天天爱跑步",
        "异象石",
        "严格次小生成树",
        "疫情控制",
        "岛屿",
        "创世纪",
        "Freda的传呼机",
        "Sightseeing Cows",
        "Interval（西瓜种植）",
        "BLO",
        "Network",
        "Knights of the Round Table",
        "Watchcow",
        "Network of Schools",
        "PKU ACM Team's Excursion",
        "Katu Puzzle",
        "Priest John's Busiest Day",
        "棋盘覆盖",
        "車的放置",
        "导弹防御塔",
        "Ants",
        "Machine Schedule",
        "Muddy Fields",
        "骑士放置",
        "Vani和Cl2捉迷藏",
        "舞动的夜晚",
        "Cable TV Network",
        "K取方格数",
        "Sightseeing",
        "升降梯上",
        "GF和猫咪的玩具",
        "社交网络",
        "Arctic Network",
        "四叶草魔杖",
        "直径",
        "逃学的小孩",
        "聚会",
        "Rendezvous",
        "Cashier Employment",
        "最优高铁环",
        "Redundant Paths",
        "矿场搭建",
        "逃不掉的路",
        "Traffic Real Time Query System",
        "John's trip",
        "太鼓达人",
        "Going from u to v or from v to u？",
        "杀人游戏",
        "Planar",
        "Wedding",
        "Team Them Up!",
        "Place the Robots",
        "Steady Cow Assignment",
        "Going Home",
        "Air Raid",
        "Sorting Slides",
        "King's Quest",
        "Drainage Ditches",
        "",
        "Raising Modulo Numbers",
    ];
    const fno: i32 = 199;
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

fn input_output_solving(paths: &[&str]) {
    let fno = paths[7][1..].parse::<i32>().unwrap();
    let input = match fno {
        95 => input_solving95(&paths[..4]),
        96 => input_solving96(&paths[..4]),
        91 | 98 | 122..=124 => input_solving98(&paths[..4]),
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
        132 => input_solving132(&paths[..4]),
        145 => input_solving145(&paths[..4]),
        169 => solving(&paths[..4], ",", true),
        89 | 92 | 93 | 97 | 113 | 129 | 130 | 140 | 150 | 151 | 179 | 193 | 197..=199|203|211|213 
|218|220||221|222|279|284|308|311 |317 |319|339|400 => {
            input_solving(&paths[..4])
        }
        90| 103| 104| 107| 116..=118| 127| 128| 131| 133..=139| 141..=144| 146| 147| 148| 149| 152..=168| 170..=178| 180..=192| 194| 195| 196|200..=202|204|205|206|207|208|209|210|212|214|215 
|216|217|219|223|224|225|226|227|228|229|230|231|232|233|234|235|236|237|238|239
|240|241|242|243|244|245|246|247|248|249
|250|251|252|253|254|255|256|257|258|259|260|261|262|263|264|265|266|267|268|269
|270|271|272|273|274|275|276|277|278|280|281|282|283|285|286|287|288|289
|290|291|292|293|294|295|296|297|298|299|300|301|302|303|304|305|306|307|309
|310|312|313|314 |315|316|318 |320|321|322|323 |324|325|326|327|328 |329  
|330|331|332 |333|334|335|336 |337 |338|340 |341|342|343|344|345|346|347 |348 |349
 |350 |351 |352 |353 |354|355 |356 |357|358|359|360|361 |362 
|363 |364|365|366 |367 |368 |369|370 |371|372 |373|374|375|376|377|378|379
|380|381 |382 |383 |384 |385|386|387 |388 |389
|390 |391 |392 |393|394 |395|396 |397 |398 |399|401|402 |403|404 
|405 |406|407|408 |409|410 |411 |412|5579  => solving(&paths[..4], ",", false),
        1=>println!("todo"),
        _ => panic!("unexpected input={}", paths[7]),
    };
    let output = match fno {
        191 => input_solving96(&[paths[0], paths[1], paths[2], paths[4]]),
        92 => output_solving92(&[paths[0], paths[1], paths[2], paths[4]], &input),
        106 => output_solving106(&[paths[0], paths[1], paths[2], paths[4]]),
        116 => input_solving98(&[paths[0], paths[1], paths[2], paths[4]]),
        111 => solving111(&[paths[0], paths[1], paths[2], paths[4]], false),
        121 => solving121(&[paths[0], paths[1], paths[2], paths[4]], false),
        124 => output_solving124(&[paths[0], paths[1], paths[2], paths[4]]),
        125 => solving125(&[paths[0], paths[1], paths[2], paths[4]], false),
        132 => output_solving132(&[paths[0], paths[1], paths[2], paths[4]]),
        133 | 169 => solving(&[paths[0], paths[1], paths[2], paths[4]], ",", true),
        141 => output_solving141(&[paths[0], paths[1], paths[2], paths[4]]),
        174 => output_solving174(&[paths[0], paths[1], paths[2], paths[4]]),
        89..=91| 97| 99| 102..=105| 110| 112..=115| 117| 122| 123| 126| 127| 130| 134| 135| 137| 143| 144| 147| 148| 150..=152| 159| 163| 165| 168| 171| 179| 183| 184| 188..=190| 193| 198| 199|203|204|206|207|208|211|213|214 
|216|217|218|220|221|222|223|226|228|229|232|233|239|240|241
|250|257|262|264|265|272|273|274|275|278|279|282|284|285|286|288|289
|290|292|295|296|298|299|300|301|302|303|306|308|311|312|314|317|319 
|320|321|326 |327|329|331|332|335 |336 |339|340|341|344|345|347|349
 |350|351|352 |354 |356 |357 |358 |359|361|362 |368 
|370|372 |373|374|377|378|379 |382 |384|385|387|388
|390|394 |395|400 |402|407 |412    => output_solving(&[paths[0], paths[1], paths[2], paths[4]]),
93| 95| 96| 98| 101| 107..=109| 118..=120| 128| 129| 131| 136| 138..=140| 142| 145| 146| 149| 153..=158| 160| 162| 164| 166| 167| 170| 172| 173| 175..=178| 180..=182| 185..=187| 192| 194..=197|200..=202|205|208|212|215
|219|224|225|227|230|231|234|235|236|237|238|242|243|244|245|246|248|249
|251|252|253|254|255|256|258|259|260|261|263|266|267|268|269
|270|271|276|277|281|283|287|291|293|294|297|304|305|307 |309 
|310|313 |315 |316|318|322|323|324 |325|328 
|330 |333|334 |337 |338|342 |343| 346|348 |353|355 |360 
|363|365|366|367|369|371 |375|376
|380|381|383|386|389 |391|392 |393|396 |397|398 |399 |401|403 |404 
|405|408 |409|411 |5579  => solving(&[paths[0], paths[1], paths[2], paths[4]], ",", false),
         210|247|280|364|406|410  =>println!("Test Case # todo"),
        _ => panic!("unexpected output={}", paths[7]),
    };
    input_output_writing(input, output, &paths[5..]);
}

fn file_extension_solving(file_number: i32) -> (&'static str, &'static str) {
    match file_number {
        105 => ("/tanabata/*.in", "/tanabata/*.out"),
        115 => ("/Color/*.in", "/Color/*.out"),
        120 => ("/defender/*.in", "/defender/*.out"),
        123 => ("/SOLDIERS.IN*", "/SOLDIERS.OU*"),
        149 => ("/data/*.in", "/data/*.ans"),
        12 => ("/SUBTRACT.IN*", "/SUBTRACT.OU*"),
        31 => ("/data/fence.in*", "/data/fence.out*"),
        32 => ("/mystery/*.in", "/mystery/*.out"),
        33 => ("/castle/*.in", "/castle/*.out"),
        34 => ("/stone/*.in", "/stone/*.out"),
        35 => ("/rdz/*.in", "/rdz/*.out"),
        36 => ("/PARITY.I*", "/PARITY.O*"),
        37 => ("/trip.i*", "/trip.o*"),
        38 => ("/kth/tests/??", "/kth/tests/??.a"),
        39 => ("/teams.??", "/teams.??a"),
        13 => ("/tests/*.in", "/tests/*.out"),
        14 => ("/apo/*.in", "/apo/*.out"),
        23 => ("/fotile-L/*.in", "/fotile-L/*.out"),
        43 => ("/mokia/*.in", "/mokia/*.out"),
        53 => ("/meteors/*.in", "/meteors/*.out"),
        63 => ("/xor/*.in", "/xor/*.out"),
        73 => ("/yam/*.in", "/yam/*.out"),
        83 => ("/pyr/*.in", "/pyr/*.out"),
        93 => ("/disjoint-set/*.in", "/disjoint-set/*.out"),
        24 => ("/communicate/*.in", "/communicate/*.out"),
        23 => ("/gen/*.in", "/gen/*.out"),
        22 => ("/gin/*.in", "/gin/*.out"),
        26 => ("/clover/*.in", "/clover/*.out"),
        27 => ("/updown/*.in", "/updown/*.out"),
        28 => ("/taiko/*.in", "/taiko/*.out"),
        19 => ("/*.in", "/*.ans"),
        18 => ("/bugs.in*", "/bugs.out*"),
        42 => ("/Input/input*", "/Output/output*"),
        52 => ("/data_1832/*.in", "/data_1832/*.out"),
        170 => ("/data+spj/*.in", "/data+spj/*.out"),
        176 => ("/tank/tank.in*", "/tank/tank.out*"),
        186 => ("/input*.txt", "/output*.txt"),
        192 => ("/exa/*.in", "/exa/*.out"),
        98 | 112 => ("/data/*.in", "/data/*.out"),
        114 | 184 | 185 => ("/Input/*.in", "/Output/*.ans"),
        95 | 117 | 129 | 135 | 148 | 151 | 171 | 188 | 189 => ("/Input/*.txt", "/Output/*.txt"),
        100 | 128 | 130 | 139 | 144 | 152 | 153 | 162 | 165 | 177 | 179 | 180 | 181 | 183 | 194 => {
            ("/Input/*.in", "/Output/*.out")
        }
        _ => ("/*.in", "/*.out"),
    }
}
fn sub_path_solving(file_number: &i32) -> &str {
    let fno = *file_number;
    match fno {
        _ if fno == fno.clamp(89, 91) || fno == 5579 => "例题/0x00 基本算法/0x01 位运算/",
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
        _ if fno == fno.clamp(198, 201) => "例题/0x30 数学知识/0x32 约数/",
        _ if fno == fno.clamp(202, 204) => "例题/0x30 数学知识/0x33 同余/",
        _ if fno == fno.clamp(205, 206) => "例题/0x30 数学知识/0x34 矩阵乘法/",
        _ if fno == fno.clamp(207, 210) => "例题/0x30 数学知识/0x35 高斯消元与线性空间/",
        _ if fno == fno.clamp(211, 213) => "例题/0x30 数学知识/0x36 组合计数/",
        _ if fno == fno.clamp(214, 215) => "例题/0x30 数学知识/0x37 容斥原理与Möbius函数/",
        _ if fno == fno.clamp(216, 218) => "例题/0x30 数学知识/0x38 概率与数学期望/",
        219 => "例题/0x30 数学知识/0x3A 博弈论与SG函数/",
        _ if fno == fno.clamp(220, 236) => "习题/0x3B 数学知识 总结与练习/",
        _ if fno == fno.clamp(237, 240) => "例题/0x40 数据结构进阶/0x41 并查集/",
        _ if fno == fno.clamp(241, 244) => "例题/0x40 数据结构进阶/0x42 树状数组/",
        _ if fno == fno.clamp(245, 248) => "例题/0x40 数据结构进阶/0x43 线段树/",
        _ if fno == fno.clamp(249, 251) => "例题/0x40 数据结构进阶/0x44 分块/",
        252 => "例题/0x40 数据结构进阶/0x45 点分治/",
        253 => "例题/0x40 数据结构进阶/0x46 二叉查找树与平衡树初步/",
        254 => "例题/0x40 数据结构进阶/0x47 离线分治算法/",
        _ if fno == fno.clamp(255, 256) => "例题/0x40 数据结构进阶/0x48 可持久化数据结构/",
        _ if fno == fno.clamp(257, 270) => "习题/0x49 数据结构进阶 总结与练习/",
        _ if fno == fno.clamp(271, 277) => "例题/0x50 动态规划/0x51 线性DP/",
        _ if fno == fno.clamp(278, 281) => "例题/0x50 动态规划/0x52 背包/",
        _ if fno == fno.clamp(282, 284) => "例题/0x50 动态规划/0x53 区间DP/",
        _ if fno == fno.clamp(285, 287) => "例题/0x50 动态规划/0x54 树形DP/",
        _ if fno == fno.clamp(288, 290) => "例题/0x50 动态规划/0x55 环形与后效性处理/",
        _ if fno == fno.clamp(291, 292) => "例题/0x50 动态规划/0x56 状态压缩DP/",
        _ if fno == fno.clamp(293, 294) => "例题/0x50 动态规划/0x57 倍增优化DP/",
        _ if fno == fno.clamp(295, 297) => "例题/0x50 动态规划/0x58 数据结构优化DP/",
        _ if fno == fno.clamp(298, 299) => "例题/0x50 动态规划/0x59 单调队列优化DP/",
        _ if fno == fno.clamp(300, 299) => "例题/0x50 动态规划/0x5A 斜率优化/",
        _ if fno == fno.clamp(300, 299) => "例题/0x50 动态规划/0x5B 四边形不等式/",
        _ if fno == fno.clamp(306, 299) => "例题/0x50 动态规划/0x5C 计数类DP/",
        _ if fno == fno.clamp(310, 299) => "例题/0x50 动态规划/0x5D 数位统计DP/",
        _ if fno == fno.clamp(312, 299) => "习题/0x5E 动态规划 总结与练习/",
        _ if fno == fno.clamp(340, 299) => "例题/0x60 图论/0x61 最短路/",
        _ if fno == fno.clamp(346, 299) => "例题/0x60 图论/0x62 最小生成树/",
        _ if fno == fno.clamp(350, 299) => "例题/0x60 图论/0x63 树的直径与最近公共祖先/",
        _ if fno == fno.clamp(358, 299) => "例题/0x60 图论/0x64 基环树/",
        _ if fno == fno.clamp(361, 299) => "例题/0x60 图论/0x65 负环与差分约束/",
        _ if fno == fno.clamp(363, 299) => "例题/0x60 图论/0x66 Tarjan算法与无向图连通性/",
        _ if fno == fno.clamp(368, 299) => "例题/0x60 图论/0x67 Tarjan算法与有向图连通性/",
        _ if fno == fno.clamp(372, 299) => "例题/0x60 图论/0x68 二分图的匹配/",
        _ if fno == fno.clamp(376, 299) => "例题/0x60 图论/0x69 二分图的覆盖与独立集/",
        _ if fno == fno.clamp(380, 299) => "例题/0x60 图论/0x6A 网络流初步/",
        _ if fno == fno.clamp(383, 299) => "习题/0x6B 图论 总结与练习/",

        _ => panic!("{}", fno),
    }
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

fn solving(paths: &[&str], seperator: &str, with_blankline: bool) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&paths.concat()).unwrap() {
        let message: String = fs::read_to_string(entry.as_ref().unwrap())
            .unwrap()
            .split("\n")
            .filter_map(|s| (with_blankline || !s.trim().is_empty()).then(|| s.trim()))
            .collect::<Vec<_>>()
            .join(seperator);
        println!("{},{}", entry.unwrap().display(), message);
        input.push(message.trim_end().to_owned());
    }
    assert!(!input.is_empty(), "{input:?},{paths:?}");
    input
}

fn input_solving145(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];
    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let mut line = String::new();
        while let Some(n) = lines.next() {
            if n.as_ref().unwrap().trim().is_empty() {
                continue;
            }
            if !line.is_empty() {
                line.push(',');
            }
            if n.as_ref().unwrap().trim() == "0"
                || n.as_ref().unwrap().trim().parse::<i32>().is_err()
            {
                line.push_str(n.as_ref().unwrap());
                continue;
            }
            line.push_str(n.as_ref().unwrap().trim());
            line.push(' ');
            for _ in 0..n.unwrap().trim().parse::<i32>().unwrap() {
                line.push_str(lines.next().unwrap().as_ref().unwrap());
                line.push(' ');
            }
            line.pop();
        }
        println!("{},{}", entry.unwrap().display(), line);
        input.push(line);
    }
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

fn input_solving132(paths: &[&str]) -> Vec<String> {
    let mut input = vec![];

    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let mut line = String::new();
        loop {
            let Some(t) = lines.next() else { break };
            if t.as_ref().unwrap() == "0" {
                break;
            }
            if !line.is_empty() {
                line.push(';');
            }
            line.push_str(t.as_ref().unwrap());
            line.push(';');
            for _ in 0..t.unwrap().parse::<i32>().unwrap() {
                line.push_str(lines.next().unwrap().as_ref().unwrap());
                line.push(',');
            }
            line.pop();
            line.push(';');
            while let Some(v) = lines.next() {
                if v.as_ref().unwrap() == "STOP" {
                    break;
                }
                let mut v = v.unwrap().replace("ENQUEUE", "E");
                line.push_str(if &v == "DEQUEUE" { "D" } else { &v });
                line.push(',');
            }
            line.pop();
        }
        input.push(line);
        println!("{},", entry.unwrap().display(),);
    }
    input
}

fn output_solving132(paths: &[&str]) -> Vec<String> {
    let mut output = vec![];

    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let mut line = String::new();
        loop {
            let Some(t) = lines.next() else { break };
            if t.as_ref().unwrap().starts_with("Scenario") {
                if !line.is_empty() {
                    line.push(';');
                }
                while let Some(v) = lines.next() {
                    if v.as_ref().unwrap().is_empty() {
                        break;
                    }
                    line.push_str(v.as_ref().unwrap());
                    line.push(',');
                }
                line.pop();
            }
        }
        output.push(line);
        println!("{},", entry.unwrap().display(),);
    }
    output
}

fn output_solving141(paths: &[&str]) -> Vec<String> {
    let mut output = vec![];

    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let mut line = String::new();
        loop {
            let Some(t) = lines.next() else { break };
            if t.as_ref().unwrap().starts_with("Test case") {
                if !line.is_empty() {
                    line.push(';');
                }
                let len = line.len();
                while let Some(v) = lines.next() {
                    if v.as_ref().unwrap().trim().is_empty() {
                        if line.len() == len {
                            line.push(' ');
                        }
                        break;
                    }
                    line.push_str(v.as_ref().unwrap());
                    line.push(',');
                }
                line.pop();
            }
        }
        output.push(line);
        println!("{},", entry.unwrap().display(),);
    }
    output
}

fn output_solving174(paths: &[&str]) -> Vec<String> {
    let mut output = vec![];

    for entry in glob(&(paths.concat())).unwrap() {
        let file = File::open(entry.as_ref().unwrap()).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let mut line = String::new();
        loop {
            let Some(t) = lines.next() else { break };
            if t.as_ref().unwrap().starts_with("Maze") {
                line.push_str(lines.next().unwrap().as_ref().unwrap());
                line.push(',');
                let _ = lines.next();
            }
        }
        line.pop();
        output.push(line);
        println!("{},", entry.unwrap().display(),);
    }
    output
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
