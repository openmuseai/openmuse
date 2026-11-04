#!/usr/bin/env node
// Render trusted release Markdown through the website's existing safe renderer.
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
const require = createRequire(new URL('../../../Muse-WebSite/package.json', import.meta.url));
const React = require('react');
const {renderToStaticMarkup} = require('react-dom/server');
const {default: Markdown} = await import(pathToFileURL(require.resolve('react-markdown')));
let text = '';
process.stdin.setEncoding('utf8');
for await (const chunk of process.stdin) text += chunk;
const heading = ({children}) => {
  const label = React.Children.toArray(children).join('');
  const id = label.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '');
  return React.createElement('h2', {id}, children);
};
process.stdout.write(renderToStaticMarkup(React.createElement(Markdown, {
  skipHtml: true,
  components: {
    h1: () => null,
    h2: heading,
    img: ({node, ...props}) => React.createElement('img', {...props, loading: 'lazy', decoding: 'async'}),
  },
}, text)));
