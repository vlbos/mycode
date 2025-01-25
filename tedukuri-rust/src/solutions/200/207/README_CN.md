207\. 球形空间产生器

*    [题目](https://www.acwing.com/problem/content/description/209/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/209/1/)
*    [题解](https://www.acwing.com/problem/content/solution/209/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/209/)

  

有一个球形空间产生器能够在 nn 维空间中产生一个坚硬的球体。

现在，你被困在了这个 nn 维球体中，你只知道球面上 n+1n+1 个点的坐标，你需要以最快的速度确定这个 nn 维球体的球心坐标，以便于摧毁这个球形空间产生器。

**注意：** 数据保证有唯一解。

#### 输入格式

第一行是一个整数 nn。

接下来的 n+1n+1 行，每行有 nn 个实数，表示球面上一点的 nn 维坐标。

每一个实数精确到小数点后 66 位，且其绝对值都不超过 2000020000。

#### 输出格式

有且只有一行，依次给出球心的 nn 维坐标（nn 个实数），两个实数之间用一个空格隔开。

每个实数精确到小数点后 33 位。

#### 数据范围

1≤n≤101≤n≤10

#### 输入样例：

    2
    0.0 0.0
    -1.0 1.0
    1.0 0.0
    

#### 输出样例：

    0.500 1.500