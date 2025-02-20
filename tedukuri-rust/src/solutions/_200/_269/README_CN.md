269\. Fotile模拟赛L





 




  

FOTILE 得到了一个长为N的序列A，为了拯救地球，他希望知道某些区间内的最大的连续XOR和。

即对于一个询问，你需要求出 max(Ai xor Ai+1 xor Ai+2 … xor Aj)max(Ai xor Ai+1 xor Ai+2 … xor Aj)，其中 l≤i≤j≤rl≤i≤j≤r。

为了体现在线操作，对于一个询问 (x,y)(x,y)：

*   l\=min(((x+lastans)modN)+1,((y+lastans)modN)+1)l\=min(((x+lastans)modN)+1,((y+lastans)modN)+1)
*   r\=max(((x+lastans)modN)+1,((y+lastans)modN)+1)r\=max(((x+lastans)modN)+1,((y+lastans)modN)+1)

其中lastans是上次询问的答案，一开始为0。

#### 输入格式

第一行两个整数N和M。

第二行有N个正整数，其中第i个数为Ai。

后M行每行两个整数 x,yx,y 表示一对询问。

#### 输出格式

共M行，每行输出一个正整数，第i行的正整数表示第i个询问的结果。

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

习题/0x49 数据结构进阶 总结与练习/Fotile模拟赛L

算法标签

* 可持久化Trie 
* 分块 
