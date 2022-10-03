"use strict";

let data;

function loadData(str) {
  data = str
    .split('\n')
    .replace(/\r/g,'')
    .filter(line => line.startsWith('PPU_EVENT'))
    .map(line => line.split(' ').slice(1))
    .map(line => ({
      type: line[0],
      args: Object.fromEntries(
        line.split(1).map(x => x.split('=').map(x => x.trim()))
      )
    }));
}

document.getElementById('file-upload').addEventListener('change', event => {
  const file = event.target.files[0];
  const reader = new FileReader();
  reader.addEventListener('load', loadData);
  reader.readAsText();
});
