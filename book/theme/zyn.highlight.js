(function() {
  const ZYN_DEFINITION = function(hljs) {
    // Pipe arguments: :arg1:arg2
    const PIPE_ARG = {
      className: 'params',
      begin: /:/, 
      end: /(?=[| \n\}])/,
      contains: [
        hljs.QUOTE_STRING_MODE,
        { className: 'number', begin: /\b\d+\b/ }
      ]
    };

    const PIPES = {
      className: 'subst',
      begin: /\|/, 
      end: /(?=\}\})/,
      contains: [
        {
          // Built-in and Custom Pipes
          className: 'title.function',
          begin: /[a-z][a-z0-9_]*/,
          keywords: {
            built_in: "upper lower snake camel pascal kebab screaming ident fmt"
          }
        },
        PIPE_ARG
      ]
    };

    return {
      name: 'zyn',
      aliases: ['zyn'],
      keywords: {
        keyword: "if else for match throw of",
        literal: "true false"
      },
      contains: [
        hljs.HASH_COMMENT_MODE,
        hljs.C_LINE_COMMENT_MODE,
        // 1. Directives: @if, @else, @for, @match, @throw
        {
          className: 'keyword',
          begin: /@/,
          end: /(if|else|for|match|throw)\b/
        },
        // 2. Custom Elements: @path::element or @element
        {
          begin: /@/,
          end: /(?=[(\s{])/,
          contains: [
            {
              className: 'title.function',
              begin: /[a-z][a-z0-9_]*(?:::[a-z][a-z0-9_]*)*/
            }
          ]
        },
        // 3. Interpolation: {{ expr | pipe }}
        {
          className: 'template-tag',
          begin: /\{\{/, 
          end: /\}\}/,
          contains: [
            {
              // The Rust Expression part
              subLanguage: 'rust',
              begin: /[^\s{][^|}]*/,
              endsBefore: /\||\}\}/,
              excludeEnd: true
            },
            PIPES
          ]
        },
        // 4. Handle Element Props: prop = value
        {
          begin: /[a-z][a-z0-9_]*\s*=/,
          returnBegin: true,
          contains: [
            { className: 'attr', begin: /[a-z][a-z0-9_]*/ },
            { begin: /=/, className: 'operator' }
          ]
        },
        // 5. Macro Entry: zyn! { ... }
        {
          className: 'built_in',
          begin: /zyn!\s*\{/,
          end: /\}/,
          contains: [{ self: true }, hljs.inherit(hljs.TITLE_MODE, { begin: /zyn/ })]
        }
      ]
    };
  };

  // mdBook Integration Logic
  const initZyn = () => {
    if (typeof hljs !== 'undefined') {
      hljs.registerLanguage('zyn', ZYN_DEFINITION);
      
      const highlightFn = hljs.highlightElement || hljs.highlightBlock;
      // We target BOTH .language-zyn and .language-rust if it looks like zyn
      document.querySelectorAll('code.language-zyn, code.language-rust').forEach((block) => {
        if (block.textContent.includes('@') || block.textContent.includes('{{')) {
          block.classList.add('language-zyn');
          highlightFn(block);
        }
      });
    }
  };

  if (document.readyState === 'loading') {
    window.addEventListener('load', initZyn);
  } else {
    initZyn();
  }
})();