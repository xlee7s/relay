import type { ReactNode } from 'react';
import { renderInline } from './markdown';

/**
 * Keep the deployment guide tied to README.md while leaving the architecture
 * diagram out of the in-panel help page.
 */
export function extractInstallDocumentation(readme: string): string {
  const lines = readme.replace(/\r\n?/g, '\n').split('\n');
  const start = lines.findIndex((line) => /^##\s+(?:[^\p{L}\p{N}]+\s*)?安装\s*docker\s*$/iu.test(line.trim()));

  if (start < 0) return '';

  const result: string[] = [];
  let skippingArchitecture = false;

  for (const line of lines.slice(start)) {
    const heading = /^##\s+(.+?)\s*$/.exec(line.trim());
    if (heading) {
      const title = heading[1].replace(/^[^\p{L}\p{N}]+\s*/u, '');
      if (/^架构$/u.test(title)) {
        skippingArchitecture = true;
        continue;
      }
      skippingArchitecture = false;
    }
    if (!skippingArchitecture) result.push(line);
  }

  return result.join('\n').trim();
}

/** A small, safe README renderer. It produces React nodes only, so README
 * content is never injected as HTML. */
export function renderReadmeDocumentation(source: string): ReactNode {
  const lines = source.split('\n');
  const blocks: ReactNode[] = [];

  for (let index = 0; index < lines.length;) {
    const line = lines[index];
    const fence = /^```([^\s`]*)\s*$/.exec(line.trim());

    if (fence) {
      const code: string[] = [];
      index += 1;
      while (index < lines.length && !/^```\s*$/.test(lines[index].trim())) {
        code.push(lines[index]);
        index += 1;
      }
      if (index < lines.length) index += 1;
      blocks.push(
        <pre key={`code-${index}`} className="rp-doc-code" data-language={fence[1] || undefined}>
          <code>{code.join('\n')}</code>
        </pre>,
      );
      continue;
    }

    const heading = /^(#{2,3})\s+(.+)$/.exec(line.trim());
    if (heading) {
      const Heading = heading[1].length === 2 ? 'h2' : 'h3';
      blocks.push(<Heading key={`heading-${index}`}>{renderInline(heading[2], `heading-${index}-`)}</Heading>);
      index += 1;
      continue;
    }

    const quote = /^>\s?(.*)$/.exec(line);
    if (quote) {
      blocks.push(<blockquote key={`quote-${index}`}>{renderInline(quote[1], `quote-${index}-`)}</blockquote>);
      index += 1;
      continue;
    }

    if (/^---+\s*$/.test(line.trim())) {
      blocks.push(<hr key={`rule-${index}`} />);
      index += 1;
      continue;
    }

    if (line.trim()) {
      blocks.push(<p key={`paragraph-${index}`}>{renderInline(line, `paragraph-${index}-`)}</p>);
    }
    index += 1;
  }

  return <>{blocks}</>;
}
