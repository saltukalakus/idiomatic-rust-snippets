#!/usr/bin/env python3
"""
Improved translation script for Rust code comments and strings
Translates full sentences, not just individual words
"""

import os
import re
from pathlib import Path

PROJECT_ROOT = Path("/Users/saltuk/code/idiomatic-rust-snippets")
ZH_PATTERNS_DIR = PROJECT_ROOT / "src" / "zh-CN" / "patterns"

# Full phrase translations for comments
COMMENT_TRANSLATIONS = {
    # Pattern explanations
    "In Rust, the visitor pattern is often better replaced with enums and pattern matching": "在 Rust 中，访问者模式通常最好用枚举和模式匹配来替代",
    "This provides exhaustive checking and avoids complex trait hierarchies": "这提供了详尽检查并避免了复杂的 trait 层次结构",
    "Instead of a visitor trait, we use regular functions or methods": "我们使用常规函数或方法，而不是访问者 trait",
    "Pattern matching makes operations explicit and type-safe": "模式匹配使操作显式且类型安全",
    "If you absolutely need external operations (plugin-style), use trait objects": "如果确实需要外部操作（插件风格），使用 trait 对象",
    "But this is rarely needed - pattern matching is more idiomatic and performant": "但这很少需要——模式匹配更符合惯用法且性能更好",
    
    # Operations
    "Operation 1: Calculate a score": "操作 1：计算分数",
    "Operation 2: Generate a description": "操作 2：生成描述",
    "Operation 3: Transform the element": "操作 3：转换元素",
    "Transform the tree": "转换树结构",
    
    # General comments
    "Global state using OnceLock - idiomatic and safe": "使用 OnceLock 的全局状态 - 符合惯用法且安全",
    "Safe singleton using OnceLock - no unsafe code needed": "使用 OnceLock 的安全单例 - 无需 unsafe 代码",
    "Get the singleton instance": "获取单例实例",
    "Increment from multiple threads": "从多个线程递增",
    "All threads see the same value": "所有线程看到相同的值",
    "Thread-safe initialization without unsafe code": "无需 unsafe 代码的线程安全初始化",
    "Deferred initialization until first use": "延迟初始化直到首次使用",
    "Guaranteed single initialization even with concurrent access": "即使并发访问也保证单次初始化",
    "Zero runtime cost after first initialization": "首次初始化后零运行时成本",
    
    # Handler pattern
    "Set the next handler in the chain": "设置链中的下一个处理器",
    "Pass to next handler if exists": "如果存在则传递给下一个处理器",
    "Handle the request": "处理请求",
    "Build the chain": "构建链",
    "Process requests": "处理请求",
    
    # Strategy pattern
    "Define the strategy trait": "定义策略 trait",
    "Concrete strategies": "具体策略",
    "Context uses a strategy": "上下文使用策略",
    "Use different strategies": "使用不同的策略",
    
    # Factory patterns
    "Factory method pattern": "工厂方法模式",
    "Create products": "创建产品",
    "Concrete factory": "具体工厂",
    "Abstract factory": "抽象工厂",
    
    # Builder pattern
    "Builder pattern for complex object construction": "用于复杂对象构造的构建器模式",
    "Required fields": "必需字段",
    "Optional fields": "可选字段",
    "Build the final object": "构建最终对象",
    
    # Proxy pattern
    "Real subject": "真实主体",
    "Proxy controls access": "代理控制访问",
    "Check permissions": "检查权限",
    "Access denied": "访问被拒绝",
    "Access granted": "访问已授予",
    
    # Adapter pattern
    "Adaptee with incompatible interface": "具有不兼容接口的被适配者",
    "Adapter translates calls": "适配器转换调用",
    "Target interface": "目标接口",
    
    # Facade pattern
    "Complex subsystems": "复杂子系统",
    "Simplified facade": "简化的外观",
    "High-level operation": "高级操作",
    
    # Bridge pattern
    "Abstraction and implementation can vary independently": "抽象和实现可以独立变化",
    "Implementation interface": "实现接口",
    "Refined abstraction": "精化抽象",
    
    # Mediator pattern
    "Components communicate through mediator": "组件通过中介者通信",
    "Mediator coordinates interactions": "中介者协调交互",
    "Colleagues": "同事对象",
    
    # Message passing
    "Sender and receiver": "发送者和接收者",
    "Send message": "发送消息",
    "Receive message": "接收消息",
    "Channel closed": "通道已关闭",
    
    # RAII
    "Resource Acquisition Is Initialization": "资源获取即初始化",
    "Acquire resource": "获取资源",
    "Resource automatically released": "资源自动释放",
    "Cleanup happens here": "清理发生在这里",
    
    # Typestate
    "Typestate pattern": "类型状态模式",
    "States are types": "状态即类型",
    "Invalid state transitions prevented at compile time": "编译时防止无效状态转换",
    
    # Extension traits
    "Extension trait": "扩展 trait",
    "Add methods to existing types": "为现有类型添加方法",
    
    # Newtype
    "Newtype pattern": "新类型模式",
    "Wrapper type": "包装类型",
    "Strong typing": "强类型",
    
    # Common actions
    "Creating": "创建中",
    "Destroying": "销毁中",
    "Processing": "处理中",
    "Handling": "处理中",
    "Calling": "调用中",
    "Executing": "执行中",
    "Initializing": "初始化中",
    "Finalizing": "完成中",
}

# String literal translations
STRING_TRANSLATIONS = {
    # Output messages
    "Initial description": "初始描述",
    "Initial score": "初始分数",
    "After transformation": "转换后",
    "Description": "描述",
    "Score": "分数",
    "Number": "数字",
    "Text": "文本",
    "List": "列表",
    
    # Handler messages
    "handled request": "已处理请求",
    "passed the request": "已传递请求",
    "Request": "请求",
    "Response": "响应",
    
    # Status messages
    "Success": "成功",
    "Failed": "失败",
    "Error": "错误",
    "Result": "结果",
    "Value": "值",
    
    # Actions
    "Creating": "创建",
    "Destroying": "销毁",
    "Processing": "处理",
    "Executing": "执行",
    "Calling": "调用",
    
    # Access control
    "Access denied": "访问被拒绝",
    "Access granted": "访问已授予",
    "Permission denied": "权限被拒绝",
    
    # Configuration
    "Configuration": "配置",
    "Application": "应用程序",
    "System": "系统",
    "Service": "服务",
    
    # Greetings
    "Hello": "你好",
    "World": "世界",
    "Hello, world": "你好，世界",
    
    # Strategy names
    "Strategy A": "策略 A",
    "Strategy B": "策略 B",
    "Using strategy": "使用策略",
    
    # Products
    "Product A": "产品 A",
    "Product B": "产品 B",
    "Creating product": "创建产品",
}

def translate_comment(comment_text):
    """Translate a full comment line"""
    text = comment_text.strip()
    
    # Try exact match first
    for en, zh in COMMENT_TRANSLATIONS.items():
        if text == en or text.lower() == en.lower():
            return zh
    
    # Try partial matches for longer comments
    result = text
    for en, zh in sorted(COMMENT_TRANSLATIONS.items(), key=lambda x: -len(x[0])):
        if en in result:
            result = result.replace(en, zh)
    
    # If still mostly English, try word-by-word for remaining parts
    if any(c.isalpha() and ord(c) < 128 for c in result):
        # Still has English, apply word translations
        for en, zh in STRING_TRANSLATIONS.items():
            result = re.sub(r'\b' + re.escape(en) + r'\b', zh, result, flags=re.IGNORECASE)
    
    return result

def translate_string(text):
    """Translate a string literal"""
    # Try exact match first
    for en, zh in STRING_TRANSLATIONS.items():
        if text == en:
            return zh
    
    # Try partial matches
    result = text
    for en, zh in sorted(STRING_TRANSLATIONS.items(), key=lambda x: -len(x[0])):
        if en in result:
            result = result.replace(en, zh)
    
    return result

def retranslate_rust_file(file_path):
    """Re-translate comments and strings in a Rust file with better translations"""
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    new_lines = []
    for line in lines:
        # Handle line comments
        comment_match = re.match(r'^(\s*//\s*)(.+)$', line)
        if comment_match:
            indent = comment_match.group(1)
            comment_text = comment_match.group(2)
            translated = translate_comment(comment_text)
            new_lines.append(f"{indent}{translated}\n")
            continue
        
        # Handle string literals in the line
        def replace_string(match):
            quote = match.group(1)
            content = match.group(2)
            # Don't translate format placeholders
            if '{' in content or '{}' in content:
                # Split and translate non-placeholder parts
                parts = re.split(r'(\{[^}]*\})', content)
                translated_parts = []
                for part in parts:
                    if part.startswith('{'):
                        translated_parts.append(part)
                    else:
                        translated_parts.append(translate_string(part))
                translated = ''.join(translated_parts)
            else:
                translated = translate_string(content)
            return f'{quote}{translated}{quote}'
        
        # Replace double-quoted strings
        line = re.sub(r'(")([^"]*)(\")', replace_string, line)
        
        new_lines.append(line)
    
    # Write back
    with open(file_path, 'w', encoding='utf-8') as f:
        f.writelines(new_lines)

def main():
    print("Re-translating pattern code files with improved translations...")
    print("=" * 60)
    
    # Find all Rust files in zh-CN patterns
    rust_files = list(ZH_PATTERNS_DIR.rglob("*.rs"))
    
    print(f"Found {len(rust_files)} Rust files to re-translate\n")
    
    for rust_file in rust_files:
        rel_path = rust_file.relative_to(ZH_PATTERNS_DIR)
        print(f"Re-translating: {rel_path}")
        retranslate_rust_file(rust_file)
    
    print("\n" + "=" * 60)
    print(f"Done! Re-translated {len(rust_files)} files.")

if __name__ == "__main__":
    main()
