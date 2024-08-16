# Slimes World

> TODO，草稿，这个游戏大概率咕咕咕，看看能做多少叭

## 1. TODO Lists

* Everything

## 2. Features

* Preview
  * ![image-20240813225215613](./README/image-20240813225215613.png)
* 摄像机控制：WASDQE移动、鼠标右键旋转视角

## 3. 游戏设计

* 一个RTS、Single-Player、3D游戏
  * 以编程的方式控制单位
  * 模仿Screeps: World，但是一个简化版本
  * 体素风格
* 有存档功能，可以关闭后继续游玩

## 4. 游戏元素/实体

* 地图
  * 单个地图，地图是长方体，由map_width * map_height个方块组成，每个方块是一个单位
  * 每个方块可以是以下类型
    * 可移动的
      * dirt、grass、sand、bush
    * 障碍物
      * wall、rock、tree、water、building (home)
* 史莱姆
  * 因为我很喜欢史莱姆，所以每个单位都是史莱姆
  * 史莱姆需要进食，史莱姆什么都可以吃，不同食物有不同的效果
    * 草：增加生命值
      * Normal Mode
      * Realistic Mode：生命值随时间减少
    * 浆果：大幅增加生命值
    * 特殊史莱姆核心：获得新的模组
  * 史莱姆核心
    * 每个史莱姆都初始拥有一个史莱姆核心
    * 史莱姆可以通过杀死其他史莱姆，来增加自己的核心数量（或者繁殖）
  * 特殊史莱姆核心
    * 一些Boss或者特殊史莱姆会掉落特殊史莱姆核心
    * *特殊史莱姆核心* 可以用于让史莱姆获得新的模组，比如新的攻击方式、移动方式、生存方式等
    * *特殊史莱姆核心* 有几类模式，可以相互兼容
      * 第一类，每次update都会调用对应的核心方法，修改史莱姆的属性
      * 第二类，AI编程时主动触发核心方法，实现一些特殊的效果
* 建筑物
  * 家
    * 家可以容纳一定数量的史莱姆居住（人口上限）
  * 圣泉（Sacred Spring）
    * 圣泉会生产史莱姆核心
  * 墙
    * 障碍物
* 势力
  * 每个势力 = 一个AI

## 5. 代码相关 / 笔记

* 随缘，说不定某个时刻就重构了
* ECS架构
  * 关于Component中的方法：目前将极其简单的方法放在Component中，例如`new`等几乎不包含任何逻辑的方法

### 5.1 一个架构问题

* 我遇到了一个设计上的问题，原本想用 `Arc<RefCell<>>` 来解决这个问题，但上网查了下，好像这么做不一定很优雅。
  * 【注：AzurIce提醒下，发现 `Arc<RefCell<>>` 编译不过（原因是RefCell没有实现Sync trait），应该用 `Arc<Mutex<>>`，很有道理】
* 背景
  * 我游戏准备支持用户自己编写代码。我目前的设计是，提供一个GameController类，让用户根据GameController提供的API，来操控游戏中的单位和建筑物。代码编写完毕后，会在一个特定的system中进行调用（相当于把用户代码注入到某个特定的system中）
  * 现在，考虑游戏中的单位/建筑物数据。这些数据我存在Resources里，但是呢，用户自己编写代码的话，肯定也要访问这些单位/建筑物数据
* 问题
  * 如何让用户代码访问到这些数据？
* 方法一：引用
  * 问题：引用的生命周期问题。我对rust的生命周期还不是很熟悉，所以不知道怎么解决这个问题
* 方法二：`Arc<Mutex<>>`
  * 问题：这个方法好像不太优雅，而且我也不知道这个方法是否是最佳实践
* 方法三：通过函数参数传入
  * 在system处理用户代码时，将单位/建筑物数据通过函数参数直接传入用户API
  * 相当于一个自顶向下的调用，system执行时，将data传入GameController，GameController再执行用户code。当用户code执行完毕之后，函数调用结束，GameController就丧失了对data的所有权
  * 这个方法感觉特别对劲！用户只要把自己代码写在某个特定形式的函数里，就可以直接访问到游戏数据了！
  * 这种方法就有点类似于Bevy，我们在写System的时候，就可以自由添加函数参数，然后获取对应数据的控制权。system结束后，就重新释放控制权
  * 妙啊！我真牛逼！写rust让我感觉自己是天才！
  * 然后需要一个观察者模式，进行用户代码的注册和调用，就对劲了
* 最后
  * 方法三很对劲，但是这会使得用户代码的编写难度提高，使得更少人能玩这个游戏。我应该提供一个统一的接口，降低学习成本
  * 所以我决定使用 **方法一 改进版**
    * 我会在每次执行用户代码时，从零构造一个GameController，这样就解决了生命周期的问题。
    * 所以方法一改进版实际上是一种 方法一&方法三 的杂交
  * rust的引用真是太优雅了，具有启发性