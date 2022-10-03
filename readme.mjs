import * as fs from 'fs/promises';

const ordered = unordered => Object.keys(unordered).sort().reduce(
  (obj, key) => { 
    obj[key] = unordered[key]; 
    return obj;
  }, 
  {}
);

(await fs.writeFile('README.md', 
  "<!-- THIS FILE IS GENERATED AUTOMATICALLY, ALL CHANGES WILL BE LOST -->\n" + 
  "<!-- Generated from README_TEMPLATE.md -->\n\n" + 
  (await fs.readFile('README_TEMPLATE.md', 'utf-8'))
    .replace(
      '___TEST_TABLE___',
      (`
        <table>
          <tr>
            <th>Test suite</th>
            <th>Test name</th>
            <th>Result</th>
          </tr>
          ${
            (await fs.readFile('_test_result.json', 'utf-8'))
              .replace(/\r/g, '')
              .split('\n')
              .filter(str => str)
              .map(str => JSON.parse(str))
              .filter(row => 
                (row.type === 'test') && (
                  (row.event === "ok") || 
                  (row.event === "failed")
                )
              )
              .sort((a,b) => (a.name > b.name) ? 1 : ((b.name > a.name) ? -1 : 0))
              .map(row => `
                <tr>
                  <td><b>${ row.name.replace('tests::', '').split('___')[0].replace(/_/g, ' ') }</b></td>
                  <td>${ row.name.replace('tests::', '').split('___')[1] }</td>
                  <td align="center">${(row.event === 'ok') ? '✔️' : '❌'}</td>
                </tr>
              `).join('')
          }
        </table>
      `).replace(/\s{2,}/g, ' ')
    )
));
