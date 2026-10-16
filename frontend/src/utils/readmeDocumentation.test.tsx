import { describe, expect, it } from 'vitest';
import { extractInstallDocumentation } from './readmeDocumentation';

describe('extractInstallDocumentation', () => {
  it('keeps the install guide through the end and omits architecture', () => {
    const readme = `# Project
## 功能
intro
## 安装docker
install
## 🚀 快速开始
start
## 🏗️ 架构
secret diagram
## 🔄 更新
update
## 🗑️ 卸载
remove`;

    const result = extractInstallDocumentation(readme);

    expect(result).toContain('## 安装docker');
    expect(result).toContain('## 🚀 快速开始');
    expect(result).toContain('## 🔄 更新');
    expect(result).toContain('## 🗑️ 卸载');
    expect(result).not.toContain('架构');
    expect(result).not.toContain('secret diagram');
  });
});
