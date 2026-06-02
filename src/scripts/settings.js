const { invoke } = window.__TAURI__.core;

const form = document.getElementById('settings-form');
const btnCancel = document.getElementById('btn-cancel');

async function loadConfig() {
  try {
    const config = await invoke('get_config');
    document.getElementById('work-duration').value = config.work_duration;
    document.getElementById('short-break').value = config.short_break_duration;
    document.getElementById('long-break').value = config.long_break_duration;
    document.getElementById('pomodoros-cycle').value = config.pomodoros_per_cycle;
    document.getElementById('animation-enabled').checked = config.animation_enabled;
  } catch (e) {
    console.error('load config error:', e);
  }
}

form.addEventListener('submit', async (e) => {
  e.preventDefault();
  try {
    const config = {
      work_duration: parseInt(document.getElementById('work-duration').value) || 25,
      short_break_duration: parseInt(document.getElementById('short-break').value) || 5,
      long_break_duration: parseInt(document.getElementById('long-break').value) || 15,
      pomodoros_per_cycle: parseInt(document.getElementById('pomodoros-cycle').value) || 4,
      animation_enabled: document.getElementById('animation-enabled').checked,
    };
    await invoke('update_config', { newConfig: config });
    window.close();
  } catch (e) {
    console.error('save config error:', e);
  }
});

btnCancel.addEventListener('click', () => {
  window.close();
});

loadConfig();
