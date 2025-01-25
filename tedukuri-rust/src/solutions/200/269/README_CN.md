269\. Fotile模拟赛L

*    [题目](https://www.acwing.com/problem/content/description/271/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/271/1/)
*    [题解](https://www.acwing.com/problem/content/solution/271/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/271/)

  

FOTILE 得到了一个长为 NN 的序列 AA，为了拯救地球，他希望知道某些区间内的最大的连续 XORXOR 和。

即对于一个询问，你需要求出 max(Ai xor Ai+1 xor Ai+2 … xor Aj)max(Ai xor Ai+1 xor Ai+2 … xor Aj)，其中 l≤i≤j≤rl≤i≤j≤r。

为了体现在线操作，对于一个询问 (x,y)(x,y)：

*   l\=min(((x+lastans)modN)+1,((y+lastans)modN)+1)l\=min(((x+lastans)modN)+1,((y+lastans)modN)+1)
*   r\=max(((x+lastans)modN)+1,((y+lastans)modN)+1)r\=max(((x+lastans)modN)+1,((y+lastans)modN)+1)

其中 lastanslastans 是上次询问的答案，一开始为 00。

#### 输入格式

第一行两个整数 NN 和 MM。

第二行有 NN 个正整数，其中第 ii 个数为 AiAi。

后 MM 行每行两个整数 x,yx,y 表示一对询问。

#### 输出格式

共 MM 行，每行输出一个正整数，第 ii 行的正整数表示第 ii 个询问的结果。

#### 数据范围

N\=12000，M\=6000，0<Ai<231，0≤x,y<231N\=12000，M\=6000，0<Ai<231，0≤x,y<231

#### 输入样例：

    3 3
    1 4 3
    0 1
    0 1
    4 3
    

#### 输出样例：

    5
    7
    7
    

难度：中等

时/空限制：1s / 64MB

总通过数：358

总尝试数：1785

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3820&show_algorithm_tags=0)

算法标签

[可持久化Trie](https://www.acwing.com/problem/search/1/?search_content=%E5%8F%AF%E6%8C%81%E4%B9%85%E5%8C%96Trie&source_file_id=3820&show_algorithm_tags=1)[分块](https://www.acwing.com/problem/search/1/?search_content=%E5%88%86%E5%9D%97&source_file_id=3820&show_algorithm_tags=1)