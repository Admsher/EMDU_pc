void setup() {
  Serial.begin(9600);  // Start the serial communication at 9600 baud rate
  randomSeed(analogRead(0));  // Seed the random number generator
}

void loop() {
  // Generate a random 15-bit number (value between 0 and 32767)
  uint16_t random_data = random(0, 32768); // Generate a random number between 0 and 32767 (15 bits)

  // Send the 15-bit data as two bytes
  sendPacket(random_data);

  // Print the random data to the Serial Monitor for debugging
  Serial.print("Sent 15-bit random data: ");
  Serial.println(random_data, BIN);  // Print data in binary format
  
  delay(1000);  // Send a new packet every 1 second
}

void sendPacket(uint16_t data) {
  // Split the 15-bit data into two bytes
  byte upper_byte = (data >> 8) & 0x7F;  // Extract upper 7 bits
  byte lower_byte = data & 0xFF;         // Extract lower 8 bits

  // Send the two bytes over the serial port
  Serial.write(upper_byte);
  Serial.write(lower_byte);
}
