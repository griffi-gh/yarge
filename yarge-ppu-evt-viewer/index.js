"use strict";

function loadData(str) {
  return str
    .replace(/\r/g,'')
    .split('\n')
    .filter(line => line.startsWith('PPU_EVENT'))
    .map(line => line.split(' ').slice(1))
    .map(line => ({
      type: line[0],
      args: Object.fromEntries(
        line.slice(1).map(x => x.split('=').map(x => x.trim()))
      )
    }));
}

function drawPoints(data, state) {
  const canvas = document.getElementById('point-canvas');
  canvas.width  = parseInt(getComputedStyle(canvas).width);
  canvas.height = parseInt(getComputedStyle(canvas).height);
  const scale = canvas.width / (160 * 2);
  const ctx = canvas.getContext('2d');
  ctx.fillStyle = 'rgba(0,0,0,0)';
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  for (const point of data) {
    if (!state[point.type]) continue;
    switch (point.type) {
      case 'SPR_FETCH_START':
        ctx.fillStyle = 'rgba(0,0,255,0.0125)';
        ctx.fillRect(point.args.lx * scale, point.args.ly * scale, 8 * scale, 3);

        ctx.fillStyle = 'rgb(255,0,0)';
        ctx.fillRect(point.args.cycles * scale, point.args.ly * scale, 3, 3);
        break;
      case 'SPR_FETCH_END':
        ctx.fillStyle = 'rgb(0,255,0)';
        ctx.fillRect(point.args.cycles * scale, point.args.ly * scale, 3, 3);
        break;
      case 'PX_FETCH_LINE_END':
        ctx.fillStyle = 'rgb(255,255,0)';
        if (point.args.cycles > 289) {
          ctx.fillStyle = 'rgb(255,140,0)';
          // ctx.fillRect(point.args.cycles * scale - 3, point.args.ly * scale, 9, 3);
        }
        ctx.fillRect(point.args.cycles * scale, point.args.ly * scale, 3, 3);
        break;
    }
  }
  ctx.fillStyle = 'rgba(100,100,255,50)';
  ctx.fillRect(289 * scale,0,2,canvas.height);
}

function addToggles(data, cb) {
  const done = {};
  const types = [];
  for(const point of data) {
    if (done[point.type]) continue;
    done[point.type] = true;
    types.push(point.type);
  }

  const state = {};
  document.getElementById('toggl-inner').remove();
  const outer = document.createElement('div');
  outer.id = 'toggl-inner';
  for(const type of types) {
    state[type] = false;
    const div = document.createElement('div');
    const inp = document.createElement('input');
    inp.type = 'checkbox';
    inp.id = 'input-type-tg-' + type;
    inp.addEventListener('change', () => {
      state[type] = !state[type];
      cb(state);
    });
    div.appendChild(inp);
    const label = document.createElement('label');
    label.for = inp.id;
    label.textContent = type;
    div.appendChild(label);
    outer.appendChild(div);
  }
  document.getElementById('toggl').appendChild(outer);
  cb(state);
}

document.getElementById('file-upload').addEventListener('change', event => {
  const file = event.target.files[0];
  const reader = new FileReader();
  reader.addEventListener('load', () => {
    let data = loadData(reader.result);
    console.log('data loaded', data);
    addToggles(data, state => {
      drawPoints(data, state);
    });
    console.log('points drawn');
  });
  reader.readAsText(file);
});
