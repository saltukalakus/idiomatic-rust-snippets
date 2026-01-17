#!/usr/bin/env python3
"""
Duplicate pattern code samples for Chinese version and translate comments/strings
"""

import os
import re
import shutil
from pathlib import Path

PROJECT_ROOT = Path("/Users/saltuk/code/idiomatic-rust-snippets")
PATTERNS_DIR = PROJECT_ROOT / "src" / "patterns"
ZH_PATTERNS_DIR = PROJECT_ROOT / "src" / "zh-CN" / "patterns"

# Translation dictionary for common terms in comments and strings
TRANSLATIONS = {
    # Common programming terms
    "Error": "错误",
    "Success": "成功",
    "Failed": "失败",
    "Result": "结果",
    "Value": "值",
    "None": "无",
    "Some": "某值",
    "true": "真",
    "false": "假",
    
    # Pattern-specific terms
    "Creating": "创建",
    "Destroying": "销毁",
    "Processing": "处理",
    "Handler": "处理器",
    "Request": "请求",
    "Response": "响应",
    "Message": "消息",
    "Event": "事件",
    "State": "状态",
    "Strategy": "策略",
    "Context": "上下文",
    "Builder": "构建器",
    "Factory": "工厂",
    "Product": "产品",
    "Adapter": "适配器",
    "Bridge": "桥接",
    "Proxy": "代理",
    "Facade": "外观",
    "Mediator": "中介者",
    "Observer": "观察者",
    "Visitor": "访问者",
    "Chain": "链",
    
    # Action words
    "executing": "执行",
    "calling": "调用",
    "sending": "发送",
    "receiving": "接收",
    "handling": "处理",
    "processing": "处理",
    "creating": "创建",
    "destroying": "销毁",
    "initializing": "初始化",
    "finalizing": "完成",
    
    # Common phrases
    "Hello": "你好",
    "World": "世界",
    "Example": "示例",
    "Test": "测试",
    "Main": "主",
    "Configuration": "配置",
    "Application": "应用",
    "System": "系统",
    "Service": "服务",
    "Client": "客户端",
    "Server": "服务器",
}

def translate_string(text):
    """Translate a string using the translation dictionary"""
    result = text
    for en, zh in TRANSLATIONS.items():
        # Case-insensitive replacement
        result = re.sub(r'\b' + re.escape(en) + r'\b', zh, result, flags=re.IGNORECASE)
    return result

def translate_rust_file(source_path, dest_path):
    """Translate comments and strings in a Rust file"""
    with open(source_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Translate line comments (// ...)
    def translate_line_comment(match):
        comment = match.group(1)
        translated = translate_string(comment)
        return f"//{translated}"
    
    content = re.sub(r'//(.+)$', translate_line_comment, content, flags=re.MULTILINE)
    
    # Translate string literals
    def translate_string_literal(match):
        quote = match.group(1)
        string_content = match.group(2)
        # Don't translate format placeholders
        if '{' in string_content or '{}' in string_content:
            # Translate parts that are not format specifiers
            parts = re.split(r'(\{[^}]*\})', string_content)
            translated_parts = []
            for part in parts:
                if part.startswith('{'):
                    translated_parts.append(part)
                else:
                    translated_parts.append(translate_string(part))
            translated = ''.join(translated_parts)
        else:
            translated = translate_string(string_content)
        return f'{quote}{translated}{quote}'
    
    # Match both " and ' quoted strings
    content = re.sub(r'(")([^"]*)(\")', translate_string_literal, content)
    content = re.sub(r"(')([^']*)(\')", translate_string_literal, content)
    
    # Write translated content
    dest_path.parent.mkdir(parents=True, exist_ok=True)
    with open(dest_path, 'w', encoding='utf-8') as f:
        f.write(content)

def copy_and_translate_pattern(pattern_path):
    """Copy a pattern directory and translate its code"""
    # Get relative path from patterns dir
    rel_path = pattern_path.relative_to(PATTERNS_DIR)
    
    # Skip if not a pattern with code
    src_dir = pattern_path / "src"
    if not src_dir.exists():
        return
    
    print(f"Processing: {rel_path}")
    
    # Create destination directory
    dest_pattern_dir = ZH_PATTERNS_DIR / rel_path
    dest_src_dir = dest_pattern_dir / "src"
    dest_src_dir.mkdir(parents=True, exist_ok=True)
    
    # Copy and translate all Rust files
    for rust_file in src_dir.rglob("*.rs"):
        rel_file_path = rust_file.relative_to(src_dir)
        dest_file = dest_src_dir / rel_file_path
        
        print(f"  Translating: {rel_file_path}")
        translate_rust_file(rust_file, dest_file)
    
    # Copy Cargo.toml if it exists
    cargo_toml = pattern_path / "Cargo.toml"
    if cargo_toml.exists():
        dest_cargo = dest_pattern_dir / "Cargo.toml"
        shutil.copy2(cargo_toml, dest_cargo)
        print(f"  Copied: Cargo.toml")

def update_chinese_markdown_includes(pattern_path):
    """Update include directives in Chinese markdown to point to zh-CN code"""
    rel_path = pattern_path.relative_to(PATTERNS_DIR)
    
    # Find corresponding Chinese markdown file
    md_files = list(ZH_PATTERNS_DIR.glob(f"{rel_path}/*.md"))
    if not md_files:
        # Try direct file
        md_file = ZH_PATTERNS_DIR / f"{rel_path}.md"
        if md_file.exists():
            md_files = [md_file]
    
    for md_file in md_files:
        if not md_file.exists():
            continue
            
        with open(md_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Update include paths to point to zh-CN version
        # From: {{#include ../../../patterns/xxx/yyy/src/main.rs}}
        # To: {{#include yyy/src/main.rs}}
        
        pattern_name = rel_path.name
        category = rel_path.parent.name if rel_path.parent != Path('.') else ''
        
        if category:
            # Replace the long path with relative path
            old_pattern = f"../../../patterns/{category}/{pattern_name}/src/"
            new_pattern = f"{pattern_name}/src/"
            content = content.replace(old_pattern, new_pattern)
        
        with open(md_file, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f"  Updated: {md_file.relative_to(ZH_PATTERNS_DIR)}")

def main():
    print("Duplicating and translating pattern code samples...")
    print("=" * 60)
    
    # Find all pattern directories with code
    pattern_dirs = []
    for category in ['behavioral', 'creational', 'structural', 'rust-idioms']:
        category_dir = PATTERNS_DIR / category
        if category_dir.exists():
            for pattern_dir in category_dir.iterdir():
                if pattern_dir.is_dir() and (pattern_dir / "src").exists():
                    pattern_dirs.append(pattern_dir)
    
    print(f"Found {len(pattern_dirs)} patterns with code\n")
    
    # Process each pattern
    for pattern_dir in pattern_dirs:
        copy_and_translate_pattern(pattern_dir)
        update_chinese_markdown_includes(pattern_dir)
        print()
    
    print("=" * 60)
    print(f"Done! Processed {len(pattern_dirs)} patterns.")
    print("\nNote: Translations are basic. Manual review recommended for accuracy.")

if __name__ == "__main__":
    main()
