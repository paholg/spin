#include <Adafruit_NeoPixel.h>
#include <avr/power.h> // Comment out this line for non-AVR boards (Arduino Due, etc.)

#define PIN 6
#define NLEDS 10

#define GREEN 65280
#define BLUE 255
#define RED 16711680
#define PURPLE RED + BLUE
#define YELLOW RED + GREEN
#define TEAL BLUE + GREEN
#define WHITE 15777215

Adafruit_NeoPixel strip = Adafruit_NeoPixel(NLEDS, PIN, NEO_GRB + NEO_KHZ800);

// IMPORTANT: To reduce NeoPixel burnout risk, add 1000 uF capacitor across
// pixel power leads, add 300 - 500 Ohm resistor on first pixel's data input
// and minimize distance between Arduino and first pixel.


void setup() {

  randomSeed(analogRead(0));
  strip.begin();
  strip.show();
}

void loop() {

  //animate_pacman(RED);

  // random stuff
  for (uint8_t i=0; i<8; i++) rand_vary();

  //animate_pacman(GREEN);

  // petals / circles
  for (uint8_t i=1; i<6; i++) {
    for (uint8_t j=0; j<5; j++) rainbow_bounce(7/i, i);
  }

  //animate_pacman(PURPLE);

  // swastika mode
  for (uint8_t i=1; i<6; i++) {
    for (uint8_t j=0; j<5; j++) rainbow_double_tick(20/i, i);
  }
}

// ---------------------------------------------------------

void set_pixels(uint8_t first, uint8_t last, uint32_t color) {
  for (uint8_t i=first; i<last; i++) strip.setPixelColor(i, color);
}

void animate_pacman(uint32_t color) {
  uint8_t ndots = 3;
  for (uint8_t i=0; i<ndots; i++) {
    pac_frame0(ndots-i, color);
    delay(8*i);
    pac_frame1(ndots-i, color);
    delay(8*i);
    pac_frame2(ndots-i, color);
    delay(8*i);
    pac_frame3(ndots-i, color);
    delay(8*i);
  }
}

void pac_frame0(uint8_t ndots, uint32_t color) {
  for (uint8_t j=0; j<20; j++) {
    ghost(color);
    delay(10);

    pacman_closed();

    delay(6);
    for(uint8_t k=0; k<ndots; k++) {
      dot();
      delay(6);
    }
  }
}

void pac_frame1(uint8_t ndots, uint32_t color) {
  for (uint8_t j=0; j<20; j++) {
    delay(2);
    ghost(color);
    delay(10);

    pacman_mid();

    delay(4);
    for(uint8_t k=0; k<ndots; k++) {
      dot();
      delay(6);
    }
  }
}

void pac_frame2(uint8_t ndots, uint32_t color) {
  for (uint8_t j=0; j<20; j++) {
    delay(4);
    ghost(color);
    delay(10);

    pacman_open();

    delay(2);
    for(uint8_t k=0; k<ndots; k++) {
      dot();
      delay(6);
    }
  }
}

void pac_frame3(uint8_t ndots, uint32_t color) {
  for (uint8_t j=0; j<20; j++) {
    delay(6);
    ghost(color);
    delay(10);

    pacman_mid();

    delay(6);
    for(uint8_t k=0; k<ndots-1; k++) {
      dot();
      delay(6);
    }
  }
}

void ghost(uint32_t color) {
  uint8_t wait = 1;

  // 1
  set_pixels(3, 8, color);
  strip.show();
  delay(wait);

  // 2
  set_pixels(8, 9, color);
  set_pixels(3, 4, 0);
  strip.show();
  delay(wait);

  // 3
  set_pixels(9, 10, color);
  set_pixels(7, 8, WHITE);
  set_pixels(6, 7, BLUE);
  set_pixels(3, 4, color);
  strip.show();
  delay(wait);

  // 4
  set_pixels(6, 8, color);
  set_pixels(3, 4, 0);
  strip.show();
  delay(wait);

  // 5
  set_pixels(7, 8, WHITE);
  set_pixels(6, 7, BLUE);
  set_pixels(3, 4, color);
  strip.show();
  delay(wait);

  // 6
  set_pixels(9, 10, 0);
  set_pixels(6, 8, color);
  set_pixels(3, 4, 0);
  strip.show();
  delay(wait);

  // 7
  set_pixels(8, 9, 0);
  set_pixels(3, 4, color);
  strip.show();
  delay(wait);

  set_pixels(0, 10, 0);
  strip.show();
}

void pacman_open() {
  uint8_t wait = 1;

  // 1
  set_pixels(5, 8, YELLOW);
  strip.show();
  delay(wait);

  // 2
  set_pixels(8, 9, YELLOW);
  set_pixels(4, 5, YELLOW);
  strip.show();
  delay(wait);

  // 3
  set_pixels(9, 10, YELLOW);
  set_pixels(3, 4, YELLOW);
  strip.show();
  delay(wait);

  // 4
  set_pixels(6, 7, 0);
  strip.show();
  delay(wait);

  // 5
  set_pixels(7, 8, 0);
  set_pixels(5, 6, 0);
  strip.show();
  delay(wait);

  // 6
  set_pixels(9, 10, 0);
  set_pixels(3, 4, 0);
  strip.show();
  delay(wait);

  // 7
  set_pixels(0, 10, 0);
  strip.show();
  delay(wait);
}

void pacman_mid() {
  uint8_t wait = 1;

  // 1
  set_pixels(5, 8, YELLOW);
  strip.show();
  delay(wait);

  // 2
  set_pixels(8, 9, YELLOW);
  set_pixels(4, 5, YELLOW);
  strip.show();
  delay(wait);

  // 3
  set_pixels(9, 10, YELLOW);
  set_pixels(3, 4, YELLOW);
  strip.show();
  delay(wait);

  // 4
  strip.show();
  delay(wait);

  // 5
  set_pixels(6, 7, 0);
  strip.show();
  delay(wait);

  // 6
  set_pixels(9, 10, 0);
  set_pixels(3, 4, 0);
  strip.show();
  delay(wait);

  // 7
  set_pixels(8, 9, 0);
  set_pixels(4, 5, 0);
  strip.show();
  delay(wait);

  set_pixels(0, 10, 0);
  strip.show();
}

void pacman_closed() {
  uint8_t wait = 1;

  // 1
  set_pixels(5, 8, YELLOW);
  strip.show();
  delay(wait);

  // 2
  set_pixels(8, 9, YELLOW);
  set_pixels(4, 5, YELLOW);
  strip.show();
  delay(wait);

  // 3
  set_pixels(9, 10, YELLOW);
  set_pixels(3, 4, YELLOW);
  strip.show();
  delay(wait);

  // 4
  strip.show();
  delay(wait);

  // 5
  strip.show();
  delay(wait);

  // 6
  set_pixels(9, 10, 0);
  set_pixels(3, 4, 0);
  strip.show();
  delay(wait);

  // 7
  set_pixels(8, 9, 0);
  set_pixels(4, 5, 0);
  strip.show();
  delay(wait);

  set_pixels(0, 10, 0);
  strip.show();
}

void dot() {
  uint8_t wait = 1;

  // 1
  set_pixels(6, 8, WHITE);
  strip.show();
  delay(wait);

  // 2
  strip.show();
  delay(wait);

  set_pixels(0, 10, 0);
  strip.show();
}

void rainbow_bounce(uint8_t iters, uint8_t wait) {
  for (uint8_t i=0; i<iters; i++) bounce(strip.Color(255, 0, 0), wait);
  for (uint8_t i=0; i<iters; i++) bounce(strip.Color(255, 255, 0), wait);
  for (uint8_t i=0; i<iters; i++) bounce(strip.Color(0, 255, 0), wait);
  for (uint8_t i=0; i<iters; i++) bounce(strip.Color(0, 255, 255), wait);
  for (uint8_t i=0; i<iters; i++) bounce(strip.Color(0, 0, 255), wait);
  for (uint8_t i=0; i<iters; i++) bounce(strip.Color(255, 0, 255), wait);
}

void bounce(uint32_t color, uint16_t wait) {
  strip.setPixelColor(0, color);
  strip.show();
  delay(wait);

  for (uint16_t i=1; i < NLEDS; i++) {
    strip.setPixelColor(i-1, 0);
    strip.setPixelColor(i, color);
    strip.show();
    delay(wait);
  }
  for (uint16_t i=NLEDS-2; i > 0; i--) {
    strip.setPixelColor(i+1, 0);
    strip.setPixelColor(i, color);
    strip.show();
    delay(wait);
  }
  strip.setPixelColor(1, 0);

}

void double_tick_in(uint32_t color, uint16_t wait) {
  strip.setPixelColor(0, color);
  strip.setPixelColor(NLEDS-1, color);
  strip.show();
  delay(wait);

  for (uint8_t i=1; i < NLEDS/2; i++) {
    strip.setPixelColor(i-1, 0);
    strip.setPixelColor(NLEDS-i, 0);
    strip.setPixelColor(i, color);
    strip.setPixelColor(NLEDS-i-1, color);
    strip.show();
    delay(wait);
  }
  strip.setPixelColor(NLEDS/2-1, 0);
  strip.setPixelColor(NLEDS/2, 0);
}

void rainbow_double_tick(uint8_t iters, uint8_t wait) {
  for (uint8_t i=0; i<iters; i++) double_tick_in(strip.Color(255, 0, 0), wait);
  for (uint8_t i=0; i<iters; i++) double_tick_in(strip.Color(255, 255, 0), wait);
  for (uint8_t i=0; i<iters; i++) double_tick_in(strip.Color(0, 255, 0), wait);
  for (uint8_t i=0; i<iters; i++) double_tick_in(strip.Color(0, 255, 255), wait);
  for (uint8_t i=0; i<iters; i++) double_tick_in(strip.Color(0, 0, 255), wait);
  for (uint8_t i=0; i<iters; i++) double_tick_in(strip.Color(255, 0, 255), wait);
}

void double_bounce(uint32_t color, uint16_t wait) {
  strip.setPixelColor(0, color);
  strip.setPixelColor(NLEDS-1, color);
  delay(wait);

  for (uint8_t i=1; i < NLEDS; i++) {
    strip.setPixelColor(i-1, 0);
    strip.setPixelColor(NLEDS-i, 0);
    strip.setPixelColor(i, color);
    strip.setPixelColor(NLEDS-i-1, color);
    strip.show();
    delay(wait);
  }
}


void blinkAll() {
  for (uint16_t i=0; i<strip.numPixels(); i++) {
    strip.setPixelColor(i, 0);
  }
  strip.show();
  for (uint16_t i=0; i<strip.numPixels(); i++) {
    uint8_t r = random(256);
    uint8_t g = random(256);
    uint8_t b = random(256);
    strip.setPixelColor(i, strip.Color(r, g, b));
  }
  strip.show();
}

void blinkLast() {
  uint16_t i = strip.numPixels() - 1;
  strip.setPixelColor(i, strip.Color(0, 0, 0));
  strip.show();
  strip.setPixelColor(i, strip.Color(0, 0, 255));
  strip.show();
}

void randSet() {
  uint8_t r = random(2)*255;
  uint8_t g = random(2)*255;
  uint8_t b = random(2)*255;
  uint16_t i = random(strip.numPixels());
  uint16_t wait = 20;

  bool blank = random(2);

  if (blank) strip.setPixelColor(i, strip.Color(0, 0, 0));
  else strip.setPixelColor(i, strip.Color(r, g, b));
  strip.show();
  //delayMicroseconds(wait);
}

void rand_in_range(uint8_t low, uint8_t high) {
  uint16_t i = random(high - low) + low;
  bool blank = random(2);

  if (blank) strip.setPixelColor(i, 0);

  else {
    uint8_t r = random(2)*255;
    uint8_t g = random(2)*255;
    uint8_t b = random(2)*255;
    strip.setPixelColor(i, strip.Color(r, g, b));
  }
  strip.show();
}

void rand_vary() {
  const uint8_t n_iters = 100;
  // clear strip
  for (uint8_t i=0; i< NLEDS; i++) strip.setPixelColor(i, 0);
  strip.show();

  // fill from in to out
  for (uint8_t high = 1; high < NLEDS + 1; high++) {
    for (uint8_t j=0; j < n_iters; j++) {
      rand_in_range(0, high);
    }
  }

  // unfill from in to out
  for (uint8_t low=0; low < NLEDS; low++) {
    if (low > 0) strip.setPixelColor(low-1, 0);
    for (uint8_t j=0; j < n_iters; j++) {
      rand_in_range(low, NLEDS);
    }
  }

  // fill from out to in
  for (uint8_t low=NLEDS-1; low > 0; low--) {
    for (uint8_t j=0; j < n_iters; j++) {
      rand_in_range(low, NLEDS);
    }
  }

  // unfill from out to in
  for (uint8_t high = NLEDS; high > 0; high--) {
    if (high < NLEDS) strip.setPixelColor(high, 0);
    for (uint8_t j=0; j < n_iters; j++) {
      rand_in_range(0, high);
    }
  }
}

