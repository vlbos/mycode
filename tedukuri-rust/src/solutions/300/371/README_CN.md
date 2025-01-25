371\. 牧师约翰最忙碌的一天

*    [题目](https://www.acwing.com/problem/content/description/373/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/373/1/)
*    [题解](https://www.acwing.com/problem/content/solution/373/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/373/)

  

牧师约翰在 99 月 11 日这天非常的忙碌。

有 NN 对情侣在这天准备结婚，每对情侣都预先计划好了婚礼举办的时间，其中第 ii 对情侣的婚礼从时刻 SiSi 开始，到时刻 TiTi 结束。

婚礼有一个必须的仪式：站在牧师面前聆听上帝的祝福。

这个仪式要么在婚礼开始时举行，要么在结束时举行。

第 ii 对情侣需要 DiDi 分钟完成这个仪式，即必须选择 Si∼Si+DiSi∼Si+Di 或 Ti−Di∼TiTi−Di∼Ti 两个时间段之一。

牧师想知道他能否满足每场婚礼的要求，即给每对情侣安排Si∼Si+DiSi∼Si+Di 或 Ti−Di∼TiTi−Di∼Ti，使得这些仪式的时间段不重叠。

若能满足，还需要帮牧师求出任意一种具体方案。

注意，约翰不能同时主持两场婚礼，且 **所有婚礼的仪式均发生在 99 月 11 日当天**。

如果一场仪式的结束时间与另一场仪式的开始时间相同，则不算重叠。

例如：一场仪式安排在 08:00∼09:0008:00∼09:00，另一场仪式安排在 09:00∼10:0009:00∼10:00，则不认为两场仪式出现重叠。

#### 输入格式

第一行包含整数 NN。

接下来 NN 行，每行包含 Si,Ti,DiSi,Ti,Di，其中 SiSi 和 TiTi 是 hh:mmhh:mm 形式。

#### 输出格式

第一行输出能否满足，能则输出 `YES`，否则输出 `NO`。

接下来 NN 行，每行给出一个具体时间段安排。

#### 数据范围

1≤N≤10001≤N≤1000

#### 输入样例：

    2
    08:00 09:00 30
    08:15 09:00 20
    

#### 输出样例：

    YES
    08:00 08:30
    08:40 09:00
    

难度：中等

时/空限制：1s / 128MB

总通过数：1301

总尝试数：3132

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3922&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3922&show_algorithm_tags=0)[POJ3683](https://www.acwing.com/problem/search/1/?search_content=POJ3683&source_file_id=3922&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3922&show_algorithm_tags=1)[tarjan](https://www.acwing.com/problem/search/1/?search_content=tarjan&source_file_id=3922&show_algorithm_tags=1)[2-SAT](https://www.acwing.com/problem/search/1/?search_content=2-SAT&source_file_id=3922&show_algorithm_tags=1)