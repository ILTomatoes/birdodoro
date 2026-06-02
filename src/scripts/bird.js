const bird = document.getElementById('bird');

bird.addEventListener('animationend', () => {
  window.close();
});

document.addEventListener('click', () => {
  window.close();
});

document.addEventListener('keydown', () => {
  window.close();
});
