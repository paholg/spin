#include <Adafruit_NeoPixel.h>
#include <avr/power.h>

#define PIN 6
#define NLEDS 10

Adafruit_NeoPixel strip = Adafruit_NeoPixel(NLEDS, PIN, NEO_GRB + NEO_KHZ800);

void setup() {
  strip.begin();
  strip.show();
}

void loop() {
  for (int i=0; i<NLEDS; i++) {
    strip.setPixelColor(i, 1, 1, 1);
  }
  strip.show();
}
