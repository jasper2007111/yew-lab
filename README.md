# yew-lab
Rust前端框架[Yew](https://github.com/yewstack/yew/tree/master)实验室，主要对一些小的功能的练习和测试。借此来研究Rust在前端领域的可能性。

以下是小的功能点测试项：
1. 图片的显示测试
2. Fetch网络拉取数据的测试
3. 计数器的测试

![Notepad](./组件截图.png)

组件测试，这个主要是对Element UI进行仿写，试图复刻一个Yew版的[Element UI](https://github.com/ElemeFE/element/tree/dev)。目前完成的组件如下：

1. Button 按钮，基本功能已经完成
2. Rate 评分，已经完成
3. ColorPicker 颜色选择器，核心功能已经完成，暂不支持自定义输入颜色，另外也不支持滑动，只支持点击选择功能。


运行

```sh
trunk serve
```

### 其它
`server`目录是测试用的服务端程序，使用Flask框架，也可以用自己的。

运行

```sh
python3 app.py
```
