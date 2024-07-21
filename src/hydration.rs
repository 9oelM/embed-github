use crate::LineRange;

pub(crate) fn generate_hydration_script(line_range: &LineRange) -> String {
    let starting_line = match line_range {
        LineRange::Single(line) => line,
        LineRange::Range(start, _) => start,
        LineRange::All => &1,
    };
    let ending_line = match line_range {
        LineRange::Single(line) => line,
        LineRange::Range(_, end) => end,
        LineRange::All => &u64::MAX,
    };

    format!(
        "
    function detectLineHeight(element) {{
      return element.offsetHeight;
  }}

    function getCodeblockFirstline() {{
      const pre = document.querySelector('pre');
      // get first span tag
      const firstSpan = pre.querySelector('span');
      return firstSpan;
    }}

    document.addEventListener('DOMContentLoaded', () => {{
      const firstLine = getCodeblockFirstline();
      const firstLineHeight = detectLineHeight(firstLine);
      const codeblock = document.querySelector('pre');
      const codeblockHeight = codeblock.scrollHeight;
      const lineCount = Math.floor(codeblockHeight / firstLineHeight);

      const headTag = document.querySelector('head');
      const styleTag = document.createElement('style');
      styleTag.innerHTML = `
        .line-number {{
          height: ${{firstLineHeight}}px;
          min-height: ${{firstLineHeight}}px;
          max-height: ${{firstLineHeight}}px;
          text-align: right;
          padding-right: 0.3em;
        }}
      `;

      headTag.appendChild(styleTag);

      const lineNumberBar = document.querySelector('.line-numbers');
      lineNumberBar.innerHTML = '';
      for (let i = 0; i < lineCount; i++) {{
        const line = document.createElement('p');
        line.classList.add('line-number');
        const lineNumber = i + {starting_line};
        line.innerText = lineNumber;
        lineNumberBar.appendChild(line);

        if (lineNumber === {ending_line}) {{
          break;
        }}
      }}
    }})
  "
    )
}
