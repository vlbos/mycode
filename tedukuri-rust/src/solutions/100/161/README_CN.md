161\. 电话列表

*    [题目](https://www.acwing.com/problem/content/description/163/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/163/1/)
*    [题解](https://www.acwing.com/problem/content/solution/163/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/163/)

  

给出一个电话列表，如果列表中存在其中一个号码是另一个号码的前缀这一情况，那么就称这个电话列表是不兼容的。

假设电话列表如下：

*   `Emergency 911`
*   `Alice 97 625 999`
*   `Bob 91 12 54 26`

在此例中，报警电话号码（`911`）为 Bob 电话号码（`91 12 54 26`）的前缀，所以该列表不兼容。

#### 输入格式

第一行输入整数 tt，表示测试用例数量。

对于每个测试用例，第一行输入整数 nn，表示电话号码数量。

接下来 nn 行，每行输入一个电话号码，号码内数字之间无空格，电话号码不超过 1010 位。

#### 输出格式

对于每个测试用例，如果电话列表兼容，则输出 `YES`。

否则，输出 `NO`。

#### 数据范围

1≤t≤401≤t≤40,  
1≤n≤100001≤n≤10000

#### 输入样例：

    2
    3
    911
    97625999
    91125426
    5
    113
    12340
    123440
    12345
    98346
    

#### 输出样例：

    NO
    YES
    

难度：简单

时/空限制：1s / 64MB

总通过数：3028

总尝试数：10680

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3712&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3712&show_algorithm_tags=0)

算法标签

[Trie](https://www.acwing.com/problem/search/1/?search_content=Trie&source_file_id=3712&show_algorithm_tags=1)[字典树](https://www.acwing.com/problem/search/1/?search_content=%E5%AD%97%E5%85%B8%E6%A0%91&source_file_id=3712&show_algorithm_tags=1)