#!/usr/bin/env python3
"""
Normalize common English console strings into Chinese inside src/zh-CN markdown files.
"""
import re
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[1] / 'src' / 'zh-CN'
if not root.exists():
  print(f"zh-CN directory not found: {root}", file=sys.stderr)
  sys.exit(2)

repls = [
  (r'"Hello, world!"', '"你好，世界！"'),
  (r'"Hello, {}!"', '"你好，{}！"'),
  (r'"Result: {}"', '"结果：{}"'),
  (r'"Error: {}"', '"错误：{}"'),
  (r'"Found at index: {}"', '"找到索引：{}"'),
  (r'"Not found"', '"未找到"'),
  (r'"The number is less than 10"', '"数字小于 10"'),
  (r'"The number is 10 or greater"', '"数字大于等于 10"'),
  (r'"Count is: {}"', '"计数：{}"'),
  (r'"The number is: {}"', '"数字是：{}"'),
  (r'"Something else!"', '"其他"'),
  (r'"Something else"', '"其他"'),
  (r'"On the y axis at {}"', '"在 y 轴，坐标 {}"'),
  (r'"On the x axis at {}"', '"在 x 轴，坐标 {}"'),
  (r'"The Quit variant has no data to destructure\."', '"Quit 变体没有可解构的数据。"'),
  (r'"Move in the x direction {} and in the y direction {}"', '"在 x 方向移动 {}，在 y 方向移动 {}"'),
  (r'"Text message: {}"', '"文本消息：{}"'),
  (r'"Change the color to red {}, green {}, and blue {}"', '"将颜色改为红 {}、绿 {}、蓝 {}"'),
  (r'"Some numbers: {}, {}, {}"', '"一些数字：{}, {}, {}"'),
  (r'"One through Five"', '"一到五"'),
  (r'"Less than five: {}"', '"小于五：{}"'),
  (r'"Hello, my name is {}!"', '"你好，我的名字是 {}！"'),
  (r'"Woof! My name is {}!"', '"汪！我的名字是 {}！"'),
  (r'"Name: {}, Age: {}"', '"名字：{}，年龄：{}"'),
  (r'"Unit struct created!"', '"单元结构已创建！"'),
  (r'"Heading North!"', '"朝北！"'),
  (r'"Heading South!"', '"朝南！"'),
  (r'"Heading East!"', '"朝东！"'),
  (r'"Heading West!"', '"朝西！"'),
  (r'"Quit message"', '"退出消息"'),
  (r'"Write message: {}"', '"写消息：{}"'),
  (r'"Change color to \({}, {}, {}\)"', '"将颜色改为 ({}, {}, {})"'),
  (r'"The larger value is: {}"', '"较大的值为：{}"'),
  (r'"Integer Point: \({}, {}\)"', '"整数点：({}, {})"'),
  (r'"Float Point: \({}, {}\)"', '"浮点点：({}, {})"'),
  (r'"Password: {}"', '"密码：{}"'),
  (r'"Salt: {}"', '"盐：{}"'),
  (r'"Hashed Password: {}"', '"哈希密码：{}"'),
  (r'"Hash verification successful!"', '"哈希验证成功！"'),
  (r'"Public Key A: {}"', '"公钥 A：{}"'),
  (r'"Public Key B: {}"', '"公钥 B：{}"'),
  (r'"Shared Secret A: {}"', '"共享密钥 A：{}"'),
  (r'"Shared Secret B: {}"', '"共享密钥 B：{}"'),
  (r'"Key exchange successful!"', '"密钥交换成功！"'),
  (r'"Original block: {:02x\?}"', '"原始块：{:02x?}"'),
  (r'"Encrypted block: {:02x\?}"', '"加密块：{:02x?}"'),
  (r'"Path found: \{:\?\}"', '"找到路径：{:?}"'),
  (r'"No path found\."', '"未找到路径。"'),
  (r'"Shortest distances between every pair of vertices:"', '"所有顶点对的最短距离："'),
  (r'"Edges in the constructed MST:"', '"生成的最小生成树的边："'),
  (r'"Using port: {}"', '"使用端口：{}"'),
  (r'"Config loaded: {} bytes"', '"配置已加载：{} 字节"'),
  (r'"Failed to load config: {}"', '"加载配置失败：{}"'),
  (r'"Please set CONFIG_PATH environment variable"', '"请设置 CONFIG_PATH 环境变量"'),
]

for p in root.rglob('*.md'):
  s = p.read_text(encoding='utf-8')
  new = s
  for pat, rep in repls:
    new = re.sub(pat, rep, new)
  if new != s:
    p.write_text(new, encoding='utf-8')

print('Normalization complete.')

        (r'"Error: {}"', '"错误：{}"'),
        (r'"Found at index: {}"', '"找到索引：{}"'),
        (r'"Not found"', '"未找到"'),
        (r'"The number is less than 10"', '"数字小于 10"'),
        (r'"The number is 10 or greater"', '"数字大于等于 10"'),
        (r'"Count is: {}"', '"计数：{}"'),
        (r'"The number is: {}"', '"数字是：{}"'),
        (r'"Something else!"', '"其他"'),
        (r'"Something else"', '"其他"'),
        (r'"On the y axis at {}"', '"在 y 轴，坐标 {}"'),
        (r'"On the x axis at {}"', '"在 x 轴，坐标 {}"'),
        (r'"The Quit variant has no data to destructure\."', '"Quit 变体没有可解构的数据。"'),
        (r'"Move in the x direction {} and in the y direction {}"', '"在 x 方向移动 {}，在 y 方向移动 {}"'),
        (r'"Text message: {}"', '"文本消息：{}"'),
        (r'"Change the color to red {}, green {}, and blue {}"', '"将颜色改为红 {}、绿 {}、蓝 {}"'),
        (r'"Some numbers: {}, {}, {}"', '"一些数字：{}, {}, {}"'),
        (r'"One through Five"', '"一到五"'),
        (r'"Less than five: {}"', '"小于五：{}"'),
        (r'"Hello, my name is {}!"', '"你好，我的名字是 {}！"'),
        (r'"Woof! My name is {}!"', '"汪！我的名字是 {}！"'),
        (r'"Name: {}, Age: {}"', '"名字：{}，年龄：{}"'),
        (r'"Unit struct created!"', '"单元结构已创建！"'),
        (r'"Heading North!"', '"朝北！"'),
        (r'"Heading South!"', '"朝南！"'),
        (r'"Heading East!"', '"朝东！"'),
        (r'"Heading West!"', '"朝西！"'),
        (r'"Quit message"', '"退出消息"'),
        (r'"Write message: {}"', '"写消息：{}"'),
        (r'"Change color to \({}, {}, {}\)"', '"将颜色改为 ({}, {}, {})"'),
        (r'"The larger value is: {}"', '"较大的值为：{}"'),
        (r'"Integer Point: \({}, {}\)"', '"整数点：({}, {})"'),
        (r'"Float Point: \({}, {}\)"', '"浮点点：({}, {})"'),
        (r'"Password: {}"', '"密码：{}"'),
        (r'"Salt: {}"', '"盐：{}"'),
        (r'"Hashed Password: {}"', '"哈希密码：{}"'),
        (r'"Hash verification successful!"', '"哈希验证成功！"'),
        (r'"Public Key A: {}"', '"公钥 A：{}"'),
        (r'"Public Key B: {}"', '"公钥 B：{}"'),
        (r'"Shared Secret A: {}"', '"共享密钥 A：{}"'),
        (r'"Shared Secret B: {}"', '"共享密钥 B：{}"'),
        (r'"Key exchange successful!"', '"密钥交换成功！"'),
        (r'"Original block: {:02x\?}"', '"原始块：{:02x?}"'),
        (r'"Encrypted block: {:02x\?}"', '"加密块：{:02x?}"'),
        (r'"Path found: \{:\?\}"', '"找到路径：{:?}"'),
        (r'"No path found\."', '"未找到路径。"'),
        (r'"Shortest distances between every pair of vertices:"', '"所有顶点对的最短距离："'),
        (r'"Edges in the constructed MST:"', '"生成的最小生成树的边："'),
        (r'"Using port: {}"', '"使用端口：{}"'),
        (r'"Config loaded: {} bytes"', '"配置已加载：{} 字节"'),
        (r'"Failed to load config: {}"', '"加载配置失败：{}"'),
        (r'"Please set CONFIG_PATH environment variable"', '"请设置 CONFIG_PATH 环境变量"'),
    ]

    from pathlib import Path
    import re

    root = Path("""%s""")
    for p in root.rglob('*.md'):
        s = p.read_text(encoding='utf-8')
        new = s
        for pat, rep in repls:
            new = re.sub(pat, rep, new)
        if new != s:
            p.write_text(new, encoding='utf-8')

    print('Normalization complete.')

    for p in ROOT.rglob('*.md'):
        s = p.read_text(encoding='utf-8')
        new = s
        for pat, rep in repls:
            new = re.sub(pat, rep, new)
        if new != s:
            p.write_text(new, encoding='utf-8')
    print('Done')
    PY
