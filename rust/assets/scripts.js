const toggleSwitch = document.getElementById('myToggle');

toggleSwitch.addEventListener('change', function() {
  if (this.checked) {
    // Toggle is on
    const _x = fetch("/on").await;
    console.log('Toggle is on');
  } else {
    // Toggle is off
    const _x = fetch("/off").await;
    console.log('Toggle is off');
  }
});

const sliderBar = document.getElementById('myRange');

sliderBar.addEventListener('change', function () {
    console.log('Slider value: ', sliderBar.value);
});
