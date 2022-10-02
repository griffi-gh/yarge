import * as fs from 'fs/promises';

(await fs.writeFile('README.md', `
  <table>
    <tr>
      <th>Test</th>
      <th>Status</th>
    </tr>
    ${
      (await fs.readFile('README_TEMPLATE.md', 'utf-8'))
        .replace(
          '___TEST_TABLE___',
          (await fs.readFile('_test_result.json', 'ucs2'))
            .replace(/[^\n\x20-\x7E]/g, '')
            .split('\n')
            .map(str => JSON.parse(str))
            .filter(row => 
              (row.type === 'test') && (
                (row.event === "ok") || 
                (row.event === "failed")
              )
            ).map(row => `
              <tr>
                <td>${row.name}</td>
                <td>${(row.event === 'ok') ? '✔️' : '❌'}</td>
              </tr>
            `).join('')
        )
    }
  </table>
`));
