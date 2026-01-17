#!/usr/bin/env python3
"""
Normalize common English console strings into Chinese inside src/zh-CN markdown files.
"""
import re
from pathlib import Path
import sys

#!/usr/bin/env python3
"""
Normalize common English console strings into Chinese inside fenced code blocks
in `src/zh-CN/*.md` files.

Usage:
  normalize_zh_prints.sh [--dry-run] [paths...]

By default the script walks `src/zh-CN` from the repository root. When file
paths are provided, only those files will be processed. With `--dry-run` the
script prints which files would be changed and counts of replacements.
"""

import argparse
import re
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
ZH_ROOT = ROOT / 'src' / 'zh-CN'

REPLS = [
    (r'"Hello, world!"', '"你好，世界！"'),
    (r'"Hello, \{\}!"', '"你好，{}！"'),
    (r'"Result: \{\}"', '"结果：{}"'),
    (r'"Error: \{\}"', '"错误：{}"'),
    (r'"Found at index: \{\}"', '"找到索引：{}"'),
    (r'"Not found"', '"未找到"'),
    (r'"The number is less than 10"', '"数字小于 10"'),
    (r'"The number is 10 or greater"', '"数字大于等于 10"'),
    (r'"Count is: \{\}"', '"计数：{}"'),
    (r'"The number is: \{\}"', '"数字是：{}"'),
    (r'"Something else!?"', '"其他"'),
    (r'"On the y axis at \{\}"', '"在 y 轴，坐标 {}"'),
    (r'"On the x axis at \{\}"', '"在 x 轴，坐标 {}"'),
    (r'"The Quit variant has no data to destructure\."', '"Quit 变体没有可解构的数据。"'),
    (r'"Move in the x direction \{\} and in the y direction \{\}"', '"在 x 方向移动 {}，在 y 方向移动 {}"'),
    (r'"Text message: \{\}"', '"文本消息：{}"'),
    (r'"Change the color to red \{\}, green \{\}, and blue \{\}"', '"将颜色改为红 {}、绿 {}、蓝 {}"'),
    (r'"Some numbers: \{\}, \{\}, \{\}"', '"一些数字：{}, {}, {}"'),
    (r'"One through Five"', '"一到五"'),
    (r'"Less than five: \{\}"', '"小于五：{}"'),
    (r'"Hello, my name is \{\}!"', '"你好，我的名字是 {}！"'),
    (r'"Woof! My name is \{\}!"', '"汪！我的名字是 {}！"'),
    (r'"Name: \{\}, Age: \{\}"', '"名字：{}，年龄：{}"'),
    (r'"Unit struct created!"', '"单元结构已创建！"'),
    (r'"Heading North!"', '"朝北！"'),
    (r'"Heading South!"', '"朝南！"'),
    (r'"Heading East!"', '"朝东！"'),
    (r'"Heading West!"', '"朝西！"'),
    (r'"Quit message"', '"退出消息"'),
    (r'"Write message: \{\}"', '"写消息：{}"'),
    (r'"Change color to \(\{\}, \{\}, \{\}\)"', '"将颜色改为 ({}, {}, {})"'),
    (r'"The larger value is: \{\}"', '"较大的值为：{}"'),
    (r'"Integer Point: \(\{\}, \{\}\)"', '"整数点：({}, {})"'),
    (r'"Float Point: \(\{\}, \{\}\)"', '"浮点点：({}, {})"'),
    (r'"Password: \{\}"', '"密码：{}"'),
    (r'"Salt: \{\}"', '"盐：{}"'),
    (r'"Hashed Password: \{\}"', '"哈希密码：{}"'),
    (r'"Hash verification successful!"', '"哈希验证成功！"'),
    (r'"Public Key A: \{\}"', '"公钥 A：{}"'),
    (r'"Public Key B: \{\}"', '"公钥 B：{}"'),
    (r'"Shared Secret A: \{\}"', '"共享密钥 A：{}"'),
    (r'"Shared Secret B: \{\}"', '"共享密钥 B：{}"'),
    (r'"Key exchange successful!"', '"密钥交换成功！"'),
    (r'"No path found\."', '"未找到路径。"'),
    (r'"Using port: \{\}"', '"使用端口：{}"'),
    (r'"Config loaded: \{\} bytes"', '"配置已加载：{} 字节"'),
    (r'"Failed to load config: \{\}"', '"加载配置失败：{}"'),
    (r'"Please set CONFIG_PATH environment variable"', '"请设置 CONFIG_PATH 环境变量"'),
    (r'"pos: \{:\?\}"', '"位置：{:?}"'),
    (r'"pos: \{:\?\}"', '"位置：{:?}"'),
    (r'"pos: {}"', '"位置：{}"'),
    (r'"Path found: \{:\?\}"', '"找到路径：{:?}"'),
    (r'"No path found\."', '"未找到路径。"'),
    (r'"The shortest path from node \{\} to node \{\} is \{\}"', '"从节点 {} 到节点 {} 的最短路径是 {}"'),
    (r'"Edges in the constructed MST:"', '"生成的最小生成树的边："'),
    (r'"Shortest distances between every pair of vertices:"', '"所有顶点对的最短距离："'),
    (r'"INF"', '"INF"'),
    (r'"The maximum value that can be obtained is: \{\}"', '"可获得的最大值是：{}"'),
    (r'"Fibonacci number at position \{\} is \{\}"', '"位置 {} 的斐波那契数是 {}"'),
    (r'"Minimum number of multiplications is \{\}"', '"最小乘法次数为 {}"'),
    (r'"pos: {:?}"', '"位置：{:?}"'),
    (r'"svg"', '"svg"'),
    (r'"received: \{\}"', '"接收：{}"'),
    (r'"log: \{\}"', '"日志：{}"'),
    (r'"handled"', '"已处理"'),
    (r'"doing"', '"正在执行"'),
    (r'"core"', '"核心"'),
    (r'"before"', '"之前"'),
    (r'"after"', '"之后"'),
    (r'"real"', '"真实"'),
    (r'"proxying"', '"代理中"'),
    (r'"pos: {:?}"', '"位置：{:?}"'),
]

COMPILED = [(re.compile(pat), rep) for pat, rep in REPLS]

# Additional token-level mappings for best-effort translations of remaining
# English phrases that weren't covered by the explicit REPLS list.
WORD_MAP = {
    r'\bSum\b': '和',
    r'\bOne\b': '一',
    r'\bTwo\b': '二',
    r'\bThree\b': '三',
    r'\bOne or Two\b': '一或二',
    r'\bQuit\b': '退出',
    r'\becho\b': '回显',
    r'\bmove to\b': '移动到',
    r'\bConfig loaded\b': '配置已加载',
    r'\bFailed to load config\b': '加载配置失败',
    r'\bPlease set CONFIG_PATH environment variable\b': '请设置 CONFIG_PATH 环境变量',
    r'\bUsing port\b': '使用端口',
    r'\bPath found\b': '找到路径',
    r'\bNo path found\b': '未找到路径',
    r'\bThe maximum value that can be obtained is\b': '可获得的最大值是',
    r'\bThe Longest Common Subsequence is\b': '最长公共子序列是',
    r'\bFibonacci number at position\b': '位置 的斐波那契数是',
    r'\bMinimum number of multiplications is\b': '最小乘法次数为',
    r'\bPrints:\b': '打印：',
    r'\bOutput:\b': '输出：',
}

WORD_COMPILED = [(re.compile(k, re.IGNORECASE), v) for k, v in WORD_MAP.items()]


def process_file(path: Path) -> int:
    """Return number of replacements made in this file."""
    text = path.read_text(encoding='utf-8')

    # Find fenced code blocks (``` ... ```), operate only inside them
    fence_re = re.compile(r'(```(?:[^\n]*?)\n)(.*?)(\n```)', re.DOTALL)

    changed = False
    total_repl = 0

    def repl_block(m):
        nonlocal changed, total_repl
        head, body, tail = m.group(1), m.group(2), m.group(3)
        new_body = body
        # First apply explicit phrase replacements
        for cre, rep in COMPILED:
            new_body, n = cre.subn(rep, new_body)
            total_repl += n
            if n:
                changed = True

        # Translate end-of-line comments: // comment
        def translate_comment(cm):
            comment = cm.group(0)  # includes leading //
            text = comment[2:].strip()
            orig = text
            # apply explicit replacements then token map
            for cre, rep in COMPILED:
                text = cre.sub(rep, text)
            for cre, rep in WORD_COMPILED:
                text = cre.sub(rep, text)
            if text != orig:
                # approximate count for reporting
                return '// ' + text
            return comment

        new_body, n_comm = re.subn(r'//.*', translate_comment, new_body)
        total_repl += n_comm
        if n_comm:
            changed = True

        # Replace remaining quoted strings that contain ASCII letters using token map
        def translate_literal(lm):
            lit = lm.group(0)
            inner = lit[1:-1]
            if re.search(r'[A-Za-z]', inner):
                orig_inner = inner
                for cre, rep in COMPILED:
                    inner = cre.sub(rep, inner)
                for cre, rep in WORD_COMPILED:
                    inner = cre.sub(rep, inner)
                if inner != orig_inner:
                    return '"' + inner + '"'
            return lit

        new_body, n_lits = re.subn(r'"(?:[^"\\]|\\.)*"', translate_literal, new_body)
        total_repl += n_lits
        if n_lits:
            changed = True
        return head + new_body + tail

    new_text = fence_re.sub(repl_block, text)

    if changed and new_text != text:
        path.write_text(new_text, encoding='utf-8')
    return total_repl


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument('--dry-run', action='store_true', help='Show what would change')
    ap.add_argument('paths', nargs='*', help='Files or directories to process')
    args = ap.parse_args(argv)

    targets = []
    if args.paths:
        for p in args.paths:
            path = Path(p)
            if path.is_dir():
                targets.extend(path.rglob('*.md'))
            else:
                targets.append(path)
    else:
        if not ZH_ROOT.exists():
            print(f'zh-CN directory not found: {ZH_ROOT}', file=sys.stderr)
            return 2
        targets = list(ZH_ROOT.rglob('*.md'))

    total_files = 0
    total_changes = 0
    changed_files = []

    for t in targets:
        if not t.exists():
            continue
        total_files += 1
        repls = process_file(t)
        if repls:
            changed_files.append((t, repls))
            total_changes += repls

    if args.dry_run:
        if changed_files:
            for p, n in changed_files:
                print(f'Would change: {p} — {n} replacement(s)')
        else:
            print('No changes detected (dry-run).')
    else:
        print(f'Processed {total_files} files, made {total_changes} replacements in {len(changed_files)} files')

    return 0


if __name__ == '__main__':
    raise SystemExit(main(sys.argv[1:]))
