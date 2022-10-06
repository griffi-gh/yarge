<!-- THIS FILE IS GENERATED AUTOMATICALLY, ALL CHANGES WILL BE LOST -->
<!-- Generated from README_TEMPLATE.md -->

<!--TITLE-->
<h2 align="center">
  <img alt="logo" src=".assets/yarge.svg" border="0" width="78" height="78" align="left">
  <code>yarge</code> - <b>y</b>et <b>a</b>nother <b>r</b>ust <b>g</b>ameboy <b>e</b>mulator<br>
</h2>
<!--BADGES-->
<div align="center">
  <a href="https://github.com/griffi-gh/yarge/blob/master/LICENSE">
    <img alt="license" src="https://shields.io/github/license/griffi-gh/yarge  " border="0">
  </a>
  <a href="https://github.com/rust-secure-code/safety-dance/">
    <img alt="unsafe forbidden" src="https://img.shields.io/badge/unsafe-forbidden-success.svg" border="0">
  </a>
  <a href="https://github.com/griffi-gh/yarge/actions">
    <img alt="build" src="https://shields.io/github/workflow/status/griffi-gh/yarge/Build" border="0">
  </a>
  <a href="https://nightly.link/griffi-gh/yarge/workflows/build/master/release-win64.zip">
    <img alt="build win64" src="https://img.shields.io/badge/build-win64-blue" border="0">
  </a>
  <a href="https://nightly.link/griffi-gh/yarge/workflows/build/master/release-lin64.zip">
    <img alt="build win32" src="https://img.shields.io/badge/build-lin64-blue" border="0">
  </a>
</div>
<p>
  <i>TODO:</i> Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
</p>
<h1>Support</h1>
<h2>MBC Support</h2>
&nbsp;&#9745; ROM ONLY <br>
&nbsp;&#9745; MBC1 <br>
&nbsp;&#9745; MBC1+RAM <br>
&nbsp;&#9745; MBC1+RAM+BATTERY <br>
&nbsp;&#9744; MBC2 <br>
&nbsp;&#9744; MBC2+BATTERY <br>
&nbsp;&#9744; ROM+RAM 1 <br>
&nbsp;&#9744; ROM+RAM+BATTERY 1 <br>
&nbsp;&#9744; MMM01 <br>
&nbsp;&#9744; MMM01+RAM <br>
&nbsp;&#9744; MMM01+RAM+BATTERY <br>
&nbsp;&#9744; MBC3+TIMER+BATTERY <br>
&nbsp;&#9744; MBC3+TIMER+RAM+BATTERY 2 <br>
&nbsp;&#9744; MBC3 <br>
&nbsp;&#9744; MBC3+RAM 2 <br>
&nbsp;&#9744; MBC3+RAM+BATTERY 2 <br>
&nbsp;&#9744; MBC5 <br>
&nbsp;&#9744; MBC5+RAM <br>
&nbsp;&#9744; MBC5+RAM+BATTERY <br>
&nbsp;&#9744; MBC5+RUMBLE <br>
&nbsp;&#9744; MBC5+RUMBLE+RAM <br>
&nbsp;&#9744; MBC5+RUMBLE+RAM+BATTERY <br>
&nbsp;&#9744; MBC6 <br>
&nbsp;&#9744; MBC7+SENSOR+RUMBLE+RAM+BATTERY <br>
&nbsp;&#9744; POCKET CAMERA <br>
&nbsp;&#9744; BANDAI TAMA5 <br>
&nbsp;&#9744; HuC3 <br>
&nbsp;&#9744; HuC1+RAM+BATTERY <br>
<h1>Screenshots</h1>
<p><i>TODO</i></p>
<h1>Tests</h1>
<p><i>These tests are run automatically after each commit.</i></p>
<p>
   
<!-- GENERATED TABLE START -->
<table><tr><th>Test suite</th><th>Test name</th><th>Result</th></tr><tr><td><b>Acid</b></td><td><code>dmg_acid2</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/bits/mem_oam</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/bits/reg_f</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/bits/unused_hwio_GS</code></td><td align="center">❌</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/instr/daa</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/interrupts/ie_push</code></td><td align="center">❌</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/oam_dma/basic</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/oam_dma/reg_read</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/oam_dma/sources_GS</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/div_write</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/rapid_toggle</code></td><td align="center">❌</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tim00</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tim00_div_trigger</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tim01</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tim01_div_trigger</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tim10</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tim10_div_trigger</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tim11</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tim11_div_trigger</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tima_reload</code></td><td align="center">✔️</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tima_write_reloading</code></td><td align="center">❌</td></tr><tr><td><b>Mooneye</b></td><td><code>acceptance/timer/tma_write_reloading</code></td><td align="center">❌</td></tr></table>
<!-- GENERATED TABLE END -->
 
</p>
