222\. 青蛙的约会

*    [题目](https://www.acwing.com/problem/content/description/224/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/224/1/)
*    [题解](https://www.acwing.com/problem/content/solution/224/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/224/)

  

两只青蛙在网上相识了，它们聊得很开心，于是觉得很有必要见一面。

它们很高兴地发现它们住在同一条纬度线上，于是它们约定各自朝西跳，直到碰面为止。

可是它们出发之前忘记了一件很重要的事情，既没有问清楚对方的特征，也没有约定见面的具体位置。

不过青蛙们都是很乐观的，它们觉得只要一直朝着某个方向跳下去，总能碰到对方的。

但是除非这两只青蛙在同一时间跳到同一点上，不然是永远都不可能碰面的。

为了帮助这两只乐观的青蛙，你被要求写一个程序来判断这两只青蛙是否能够碰面，会在什么时候碰面。

我们把这两只青蛙分别叫做青蛙 AA 和青蛙 BB，并且规定纬度线上东经 00 度处为原点，由东往西为正方向，单位长度 11 米，这样我们就得到了一条首尾相接的数轴。

设青蛙 AA 的出发点坐标是 xx，青蛙 BB 的出发点坐标是 yy。

青蛙 AA 一次能跳 mm 米，青蛙 BB 一次能跳 nn 米，两只青蛙跳一次所花费的时间相同。

纬度线总长 LL 米。

现在要你求出它们跳了几次以后才会碰面。

#### 输入格式

输入只包括一行 55 个整数 x，y，m，n，Lx，y，m，n，L。

#### 输出格式

输出碰面所需要的跳跃次数，如果永远不可能碰面则输出一行 `Impossible`。

#### 数据范围

x≠y<2000000000x≠y<2000000000,  
0<m,n<20000000000<m,n<2000000000,  
0<L<21000000000<L<2100000000

##### 输入样例：

    1 2 3 4 5
    

#### 输出样例：

    4
    

难度：困难

时/空限制：1s / 10MB

总通过数：5021

总尝试数：9675

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3773&show_algorithm_tags=0)[POJ1061](https://www.acwing.com/problem/search/1/?search_content=POJ1061&source_file_id=3773&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3773&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3773&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3773&show_algorithm_tags=1)[扩展欧几里得算法](https://www.acwing.com/problem/search/1/?search_content=%E6%89%A9%E5%B1%95%E6%AC%A7%E5%87%A0%E9%87%8C%E5%BE%97%E7%AE%97%E6%B3%95&source_file_id=3773&show_algorithm_tags=1)[同余方程](https://www.acwing.com/problem/search/1/?search_content=%E5%90%8C%E4%BD%99%E6%96%B9%E7%A8%8B&source_file_id=3773&show_algorithm_tags=1)