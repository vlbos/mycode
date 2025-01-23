132\. 小组队列

*    [题目](https://www.acwing.com/problem/content/description/134/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/134/1/)
*    [题解](https://www.acwing.com/problem/content/solution/134/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/134/)

  

有 nn 个小组要排成一个队列，每个小组中有若干人。

当一个人来到队列时，如果队列中已经有了自己小组的成员，他就直接插队排在自己小组成员的后面，否则就站在队伍的最后面。

请你编写一个程序，模拟这种小组队列。

#### 输入格式：

输入将包含一个或多个测试用例。

对于每个测试用例，第一行输入小组数量 tt。

接下来 tt 行，每行输入一个小组描述，第一个数表示这个小组的人数，接下来的数表示这个小组的人的编号。

编号是 00 到 999999999999 范围内的整数。

一个小组最多可包含 10001000 个人。

最后，命令列表如下。 有三种不同的命令：

1、`ENQUEUE x` \- 将编号是 xx 的人插入队列；

2、`DEQUEUE` \- 让整个队列的第一个人出队；

3、`STOP` \- 测试用例结束

每个命令占一行。

当输入用例 t\=0t\=0 时，代表停止输入。

需注意：测试用例最多可包含 200000200000（2020 万）个命令，因此小组队列的实现应该是高效的：

入队和出队都需要使用常数时间。

#### 输出样例

对于每个测试用例，首先输出一行 `Scenario #k`，其中 kk 是测试用例的编号。

然后，对于每个 `DEQUEUE` 命令，输出出队的人的编号，每个编号占一行。

在每个测试用例（包括最后一个测试用例）输出完成后，输出一个空行。

#### 数据范围

1≤t≤10001≤t≤1000

#### 输入样例：

    2
    3 101 102 103
    3 201 202 203
    ENQUEUE 101
    ENQUEUE 201
    ENQUEUE 102
    ENQUEUE 202
    ENQUEUE 103
    ENQUEUE 203
    DEQUEUE
    DEQUEUE
    DEQUEUE
    DEQUEUE
    DEQUEUE
    DEQUEUE
    STOP
    2
    5 259001 259002 259003 259004 259005
    6 260001 260002 260003 260004 260005 260006
    ENQUEUE 259001
    ENQUEUE 260001
    ENQUEUE 259002
    ENQUEUE 259003
    ENQUEUE 259004
    ENQUEUE 259005
    DEQUEUE
    DEQUEUE
    ENQUEUE 260002
    ENQUEUE 260003
    DEQUEUE
    DEQUEUE
    DEQUEUE
    DEQUEUE
    STOP
    0
    

输出样例：

    Scenario #1
    101
    102
    103
    201
    202
    203
    
    Scenario #2
    259001
    259002
    259003
    259004
    259005
    260001
    
    

难度：简单

时/空限制：2s / 64MB

总通过数：4251

总尝试数：9736

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3683&show_algorithm_tags=0)

算法标签

[队列](https://www.acwing.com/problem/search/1/?search_content=%E9%98%9F%E5%88%97&source_file_id=3683&show_algorithm_tags=1)