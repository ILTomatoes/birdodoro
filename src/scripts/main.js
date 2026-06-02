const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const display = document.getElementById('timer-display');
const ring = document.getElementById('ring-progress');
const phaseLabel = document.getElementById('phase-label');
const cycleInfo = document.getElementById('cycle-info');
const btnStart = document.getElementById('btn-start');
const btnPause = document.getElementById('btn-pause');
const btnStop = document.getElementById('btn-stop');
const btnSettings = document.getElementById('btn-settings');

const CIRCUMFERENCE = 2 * Math.PI * 90;

function formatTime(secs) {
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
}

function updateUI(state) {
  if (!state) return;
  display.textContent = formatTime(state.remaining_secs);

  const offset = CIRCUMFERENCE * (1 - state.progress);
  ring.style.strokeDashoffset = offset;

  const typeClass = state.session_type === 'Work' ? 'work'
    : state.session_type === 'ShortBreak' ? 'short-break'
    : 'long-break';

  ring.className = 'ring-progress ' + typeClass;
  phaseLabel.className = 'phase-label ' + typeClass;

  const phaseText = state.session_type === 'Work' ? '工作中'
    : state.session_type === 'ShortBreak' ? '短休息'
    : '长休息';
  phaseLabel.textContent = phaseText;

  const isRunning = state.status === 'Running';
  const isPaused = state.status === 'Paused';
  const isActive = isRunning || isPaused;

  btnStart.disabled = isActive;
  btnPause.disabled = !isRunning;
  btnStop.disabled = !isActive;
  btnPause.textContent = isPaused ? '继续' : '暂停';
}

async function startTimer() {
  try {
    const state = await invoke('start_timer', { sessionType: 'Work' });
    updateUI(state);
  } catch (e) {
    console.error('start_timer error:', e);
  }
}

async function pauseTimer() {
  try {
    const state = await invoke('pause_timer');
    if (state) updateUI(state);
  } catch (e) {
    console.error('pause_timer error:', e);
  }
}

async function resumeTimer() {
  try {
    const state = await invoke('resume_timer');
    if (state) updateUI(state);
  } catch (e) {
    console.error('resume_timer error:', e);
  }
}

async function stopTimer() {
  try {
    await invoke('stop_timer');
    display.textContent = '25:00';
    ring.style.strokeDashoffset = 0;
    phaseLabel.textContent = '空闲';
    phaseLabel.className = 'phase-label';
    btnStart.disabled = false;
    btnPause.disabled = true;
    btnStop.disabled = true;
    btnPause.textContent = '暂停';
    cycleInfo.textContent = '';
  } catch (e) {
    console.error('stop_timer error:', e);
  }
}

btnStart.addEventListener('click', startTimer);
btnPause.addEventListener('click', () => {
  if (btnPause.textContent === '继续') {
    resumeTimer();
  } else {
    pauseTimer();
  }
});
btnStop.addEventListener('click', stopTimer);

btnSettings.addEventListener('click', async () => {
  try {
    await invoke('open_settings');
  } catch (e) {
    console.error('open_settings error:', e);
  }
});

listen('timer:tick', (event) => {
  updateUI(event.payload);
});

listen('timer:complete', (event) => {
  updateUI(event.payload);
});

listen('cycle:updated', (event) => {
  const s = event.payload;
  cycleInfo.textContent = `🍅 ${s.completed_pomodoros} / ${s.cycle_count + 1} 组`;
});

listen('config:updated', async () => {
  try {
    const config = await invoke('get_config');
    display.textContent = formatTime(config.work_duration * 60);
  } catch (e) {
    console.error('config update error:', e);
  }
});
