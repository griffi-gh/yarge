"use strict";

const SUPPORTED_EVT_TYPES = new Set([
  'SPR_FETCH_START',
  'SPR_FETCH_END',
  'PX_FETCH_LINE_END',
  'LX_INC',
  'SPR_FETCHER_STATE_CHANGE'
]);

const appMain = document.getElementById('app-main');

function getMousePos(evt, base) {
  var rect = (base ?? evt.target).getBoundingClientRect();
  return {
    x: evt.clientX - rect.left,
    y: evt.clientY - rect.top
  };
}

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
        ctx.fillStyle = 'rgba(0,0,255,0.125)';
        ctx.fillRect(point.args.lx * scale, point.args.ly * scale, 8 * scale, 3);

        ctx.fillStyle = 'rgb(255,0,0)';
        ctx.fillRect(point.args.cycles * scale, point.args.ly * scale, 2, scale);
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
      case 'LX_INC':
        ctx.fillStyle = 'rgb(0,0,0,0.5)';
        ctx.fillRect(point.args.cycles * scale, point.args.ly * scale, 2, 2);
        break;
      case 'SPR_FETCHER_STATE_CHANGE':
        ctx.fillStyle = `rgb(${point.args.next * 64},64,64,${.5 + (point.args.next / 8)})`;
        ctx.fillRect(point.args.cycles * scale, point.args.ly * scale, 2, 2);
        break;
    }
  }
  ctx.fillStyle = 'rgb(100,100,255)';
  ctx.fillRect(289 * scale,0,2,canvas.height);
}

function addUI(data, cb) {
  const done = {};
  const types = [];
  for(const point of data) {
    if (done[point.type]) continue;
    done[point.type] = true;
    types.push(point.type);
  }

  let frames = 0;
  {
    let allowed = true;
    data.forEach(v => {
      if (allowed) {
        frames++;
        allowed = false;
      }
      if (v.type == "FRAME_END") {
        allowed = true;
      }
    });
  }

  const state = {};
  cb = cb ?? (statee => {
    drawPoints(data, statee);
  });
  document.getElementById('toggl-inner').remove();
  const outer = document.createElement('div');
  outer.id = 'toggl-inner';
  for(const type of types) {
    const lsKey = 'SAVE_CHECKBOX_' + type;
    state[type] = false;

    const div = document.createElement('div');

    const inp = document.createElement('input');
      inp.type = 'checkbox';
      inp.id = 'input-type-tg-' + type;
      inp.checked = (localStorage.getItem(lsKey) ?? 'false') === 'true';
      
      inp.addEventListener('change', () => {
        state[type] = inp.checked;
        localStorage.setItem(lsKey, state[type] ? 'true' : 'false');
        console.log(localStorage[lsKey]);
        cb(state);
      });
      state[type] = inp.checked;
      div.appendChild(inp);

    const label = document.createElement('label');
      label.setAttribute('for', inp.id);
      label.textContent = type;
      div.appendChild(label);
      if (!SUPPORTED_EVT_TYPES.has(type)) {
        label.textContent += '(Not supported)';
        inp.disabled = true;
        inp.checked = false;
        state[type] = false;
        localStorage.setItem(lsKey, 'false');
      }
      outer.appendChild(div);
  }
  document.getElementById('toggl').appendChild(outer);
  cb(state);

  const frmCntEl = document.getElementById('frm')
    frmCntEl.value = frames.toString();
    frmCntEl.classList.toggle('important', frames > 1);
    frmCntEl.classList.toggle('good', frames === 1);
  document.getElementById('pnt').value = data.length.toString();

  const tooltip = document.getElementById('canvas-tooltip');
  const canvas = document.getElementById('point-canvas');
  const scale = canvas.width / (160 * 2);
  document.getElementById('point-canvas').addEventListener('mousemove', event => {
    const pos = getMousePos(event);
    tooltip.style.transform = `translate(${pos.x + 20}px, ${pos.y + 20}px)`;
    document.getElementById('tt-lx').value = Math.floor(pos.x / scale).toString();
    //document.getElementById('tt-dots').value = (4 * (document.getElementById('tt-lx').value | 0)).toString();
    document.getElementById('tt-dots').value = (80 + (document.getElementById('tt-lx').value | 0)).toString();
    document.getElementById('tt-ly').value = Math.floor(pos.y / scale).toString();
  });

  const filt_inp = document.getElementById('filter-by-frame');
  const filt_btn = document.getElementById('filter-by-frame-btn');
  filt_btn.addEventListener('click', () => {
    const fval = filt_inp.value | 0;
    let iframe = 0;
    data = data.filter(v => {
      if (v.type == 'FRAME_END') {
        iframe++;
        return false;
      }
      if (iframe === fval) {
        return true;
      }
      return false;
    });
    addUI(data);
  });
}

const fup = document.getElementById('file-upload')
const fupCallback = () => {
  const file = fup.files[0];
  const reader = new FileReader();
  reader.addEventListener('load', () => {
    let data = loadData(reader.result);
    console.log('data loaded', data);
    addUI(data);
    console.log('points drawn');
    setTimeout(updOi, 0);
  });
  reader.readAsText(file);
};
fup.addEventListener('change', fupCallback);
if (fup.files.length) fupCallback();


//offset image
const oi = document.getElementById('offset-image');
oi.value = (localStorage.getItem('ref-offset') ?? 12).toString();
var updOi = () => {
  const canvas = document.getElementById('point-canvas');
  const scale = canvas.width / (160 * 2);
  appMain.style.backgroundPositionX = `${(oi.value|0)*scale}px`;
  localStorage.setItem('ref-offset', oi.value.toString());
  document.getElementById('offset-image-value').value = oi.value.toString();
};
updOi();
oi.addEventListener('change', updOi);
document.getElementById('offset-image-reset').addEventListener('click', () => {
  oi.value = '12';
  updOi();
});

const hi = document.getElementById('hide-refimg');
hi.checked = (localStorage.getItem('ref-hide') ?? 'false') === 'true';
const updHi = () => {
  appMain.style.backgroundImage = hi.checked ? 'unset': '';
  localStorage.setItem('ref-hide', hi.checked ? 'true' : 'false');
};
hi.addEventListener('change', updHi);
updHi();
